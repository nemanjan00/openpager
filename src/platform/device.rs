use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;

use super::{Action, RenderBuffer};

const FBIOGET_VSCREENINFO: libc::c_int = 0x4600;
const INPUT_EVENT_SIZE: usize = 16;
const EV_KEY: u16 = 0x01;
const KEY_UP: u16 = 103;
const KEY_DOWN: u16 = 108;
const BTN_FORWARD: u16 = 305;
const BTN_BACK: u16 = 304;
const KEY_POWER: u16 = 116;

#[repr(C)]
struct FbVarScreeninfo {
    xres: u32,
    yres: u32,
    _padding: [u32; 38],
}

impl Default for FbVarScreeninfo {
    fn default() -> Self {
        Self {
            xres: 0,
            yres: 0,
            _padding: [0; 38],
        }
    }
}

#[repr(C)]
struct InputEvent {
    _tv_sec: u32,
    _tv_usec: u32,
    type_: u16,
    code: u16,
    value: i32,
}

pub struct DevicePlatform {
    fb_file: File,
    fb_width: u32,
    fb_buffer: Vec<u16>,
    input_file: File,
    input_buf: [u8; INPUT_EVENT_SIZE],
}

impl DevicePlatform {
    pub fn new() -> io::Result<Self> {
        // Open framebuffer
        let fb_file = OpenOptions::new().read(true).write(true).open("/dev/fb0")?;
        let mut info = FbVarScreeninfo::default();
        unsafe {
            if libc::ioctl(fb_file.as_raw_fd(), FBIOGET_VSCREENINFO, &mut info) < 0 {
                return Err(io::Error::last_os_error());
            }
        }
        let fb_width = info.xres;
        let fb_height = info.yres;
        let fb_buffer = vec![0u16; (fb_width * fb_height) as usize];

        // Open input
        let input_file = File::open("/dev/input/event0")?;
        unsafe {
            let fd = input_file.as_raw_fd();
            let flags = libc::fcntl(fd, libc::F_GETFL);
            libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
        }

        Ok(Self {
            fb_file,
            fb_width,
            fb_buffer,
            input_file,
            input_buf: [0u8; INPUT_EVENT_SIZE],
        })
    }

    pub fn is_open(&self) -> bool {
        true
    }

    pub fn poll(&mut self) -> Option<Action> {
        // Drain events until we find an actionable one or buffer is empty
        while let Ok(INPUT_EVENT_SIZE) = self.input_file.read(&mut self.input_buf) {
            let event: InputEvent =
                unsafe { std::ptr::read(self.input_buf.as_ptr() as *const _) };
            if event.type_ == EV_KEY && event.value == 1 {
                let action = match event.code {
                    KEY_UP => Some(Action::Up),
                    KEY_DOWN => Some(Action::Down),
                    BTN_FORWARD => Some(Action::Select),
                    BTN_BACK | KEY_POWER => Some(Action::Back),
                    _ => None,
                };
                if action.is_some() {
                    return action;
                }
            }
        }
        None
    }

    pub fn draw(&mut self, render: &RenderBuffer) {
        let src = render.pixels_raw();
        let src_w = render.width;
        let src_h = render.height;

        // 90° CW rotation + RGB888 to RGB565 conversion
        for src_y in 0..src_h {
            for src_x in 0..src_w {
                let fb_x = src_h - 1 - src_y;
                let fb_y = src_x;
                let src_idx = (src_y * src_w + src_x) as usize;
                let dst_idx = (fb_y * self.fb_width + fb_x) as usize;
                if dst_idx < self.fb_buffer.len() {
                    let rgb = src[src_idx];
                    let r = ((rgb >> 16) & 0xFF) as u16;
                    let g = ((rgb >> 8) & 0xFF) as u16;
                    let b = (rgb & 0xFF) as u16;
                    self.fb_buffer[dst_idx] = ((r >> 3) << 11) | ((g >> 2) << 5) | (b >> 3);
                }
            }
        }

        // Flush to framebuffer
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.fb_buffer.as_ptr() as *const u8,
                self.fb_buffer.len() * 2,
            )
        };
        let _ = self.fb_file.seek(SeekFrom::Start(0));
        let _ = self.fb_file.write_all(bytes);
    }

    pub fn wait(&self) {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
