use std::fs::{File, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;

#[cfg(target_arch = "mips")]
type IoctlReq = libc::c_int;
#[cfg(not(target_arch = "mips"))]
type IoctlReq = libc::c_ulong;

const FBIOGET_VSCREENINFO: IoctlReq = 0x4600;

#[repr(C)]
#[derive(Default)]
struct FbVarScreeninfo {
    xres: u32,
    yres: u32,
    _padding: [u32; 38],
}

pub struct Framebuffer {
    file: File,
    width: u32,
    height: u32,
    buffer: Vec<u16>,
}

impl Framebuffer {
    pub fn new(device: Option<&str>) -> io::Result<Self> {
        let path = device.unwrap_or("/dev/fb0");
        let file = OpenOptions::new().read(true).write(true).open(path)?;

        let mut info = FbVarScreeninfo::default();
        unsafe {
            if libc::ioctl(file.as_raw_fd(), FBIOGET_VSCREENINFO, &mut info) < 0 {
                return Err(io::Error::last_os_error());
            }
        }

        let width = info.xres;
        let height = info.yres;
        let buffer = vec![0u16; (width * height) as usize];

        Ok(Self { file, width, height, buffer })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Blit render buffer with 90° CW rotation
    pub fn blit(&mut self, src: &[u16], src_w: u32, src_h: u32) {
        for src_y in 0..src_h {
            for src_x in 0..src_w {
                let fb_x = src_h - 1 - src_y;
                let fb_y = src_x;
                let src_idx = (src_y * src_w + src_x) as usize;
                let dst_idx = (fb_y * self.width + fb_x) as usize;
                if dst_idx < self.buffer.len() {
                    self.buffer[dst_idx] = src[src_idx];
                }
            }
        }
        self.flush();
    }

    fn flush(&mut self) {
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.buffer.as_ptr() as *const u8,
                self.buffer.len() * 2,
            )
        };
        let _ = self.file.seek(SeekFrom::Start(0));
        let _ = self.file.write_all(bytes);
    }
}
