use std::fs::File;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

#[repr(C)]
struct InputEvent {
    _tv_sec: u32,
    _tv_usec: u32,
    type_: u16,
    code: u16,
    value: i32,
}

const INPUT_EVENT_SIZE: usize = 16;
const EV_KEY: u16 = 0x01;

// Pager GPIO button key codes
const KEY_UP: u16 = 103;
const KEY_DOWN: u16 = 108;
const KEY_LEFT: u16 = 105;
const KEY_RIGHT: u16 = 106;
const BTN_FORWARD: u16 = 305;
const BTN_BACK: u16 = 304;
const KEY_POWER: u16 = 116;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Back,
    Power,
}

pub struct Input {
    file: File,
    buf: [u8; INPUT_EVENT_SIZE],
}

impl Input {
    pub fn new(device: Option<&str>) -> io::Result<Self> {
        let path = device.unwrap_or("/dev/input/event0");
        let file = File::open(path)?;

        unsafe {
            let fd = file.as_raw_fd();
            let flags = libc::fcntl(fd, libc::F_GETFL);
            libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
        }

        Ok(Self {
            file,
            buf: [0u8; INPUT_EVENT_SIZE],
        })
    }

    pub fn poll(&mut self) -> Option<Button> {
        if let Ok(INPUT_EVENT_SIZE) = self.file.read(&mut self.buf) {
            let event: InputEvent = unsafe { std::ptr::read(self.buf.as_ptr() as *const _) };
            if event.type_ == EV_KEY && event.value == 1 {
                return self.map_key(event.code);
            }
        }
        None
    }

    pub fn wait(&mut self) -> io::Result<Button> {
        loop {
            if let Some(btn) = self.poll() {
                return Ok(btn);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn map_key(&self, code: u16) -> Option<Button> {
        match code {
            KEY_UP => Some(Button::Up),
            KEY_DOWN => Some(Button::Down),
            KEY_LEFT => Some(Button::Left),
            KEY_RIGHT => Some(Button::Right),
            BTN_FORWARD => Some(Button::Forward),
            BTN_BACK => Some(Button::Back),
            KEY_POWER => Some(Button::Power),
            _ => None,
        }
    }
}
