use std::fs::{File, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

// Linux framebuffer ioctl constants
// Use Ioctl type which varies by platform (c_ulong on x86_64, c_int on MIPS)
#[cfg(target_arch = "mips")]
type IoctlRequest = libc::c_int;
#[cfg(not(target_arch = "mips"))]
type IoctlRequest = libc::c_ulong;

const FBIOGET_VSCREENINFO: IoctlRequest = 0x4600;
const FBIOGET_FSCREENINFO: IoctlRequest = 0x4602;

/// Variable screen info from fbdev
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct FbVarScreeninfo {
    pub xres: u32,
    pub yres: u32,
    pub xres_virtual: u32,
    pub yres_virtual: u32,
    pub xoffset: u32,
    pub yoffset: u32,
    pub bits_per_pixel: u32,
    pub grayscale: u32,
    pub red: FbBitfield,
    pub green: FbBitfield,
    pub blue: FbBitfield,
    pub transp: FbBitfield,
    pub nonstd: u32,
    pub activate: u32,
    pub height: u32,
    pub width: u32,
    pub accel_flags: u32,
    // Timing - not used but needed for struct size
    pub pixclock: u32,
    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub sync: u32,
    pub vmode: u32,
    pub rotate: u32,
    pub colorspace: u32,
    pub reserved: [u32; 4],
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct FbBitfield {
    pub offset: u32,
    pub length: u32,
    pub msb_right: u32,
}

/// Fixed screen info from fbdev
#[repr(C)]
#[derive(Default, Debug, Clone)]
pub struct FbFixScreeninfo {
    pub id: [u8; 16],
    pub smem_start: libc::c_ulong,
    pub smem_len: u32,
    pub fb_type: u32,
    pub type_aux: u32,
    pub visual: u32,
    pub xpanstep: u16,
    pub ypanstep: u16,
    pub ywrapstep: u16,
    pub line_length: u32,
    pub mmio_start: libc::c_ulong,
    pub mmio_len: u32,
    pub accel: u32,
    pub capabilities: u16,
    pub reserved: [u16; 2],
}

/// RGB565 pixel type
pub type Rgb565 = u16;

/// Convert RGB888 to RGB565
#[inline(always)]
pub fn rgb_to_565(r: u8, g: u8, b: u8) -> Rgb565 {
    ((r as u16 >> 3) << 11) | ((g as u16 >> 2) << 5) | (b as u16 >> 3)
}

/// Convert ARGB8888 to RGB565
#[inline(always)]
pub fn argb_to_565(pixel: u32) -> Rgb565 {
    let r = ((pixel >> 16) & 0xFF) as u8;
    let g = ((pixel >> 8) & 0xFF) as u8;
    let b = (pixel & 0xFF) as u8;
    rgb_to_565(r, g, b)
}

/// Linux framebuffer display
pub struct Framebuffer {
    file: File,
    pub var_info: FbVarScreeninfo,
    pub fix_info: FbFixScreeninfo,
    buffer: Vec<u8>,
}

impl Framebuffer {
    /// Open framebuffer device (default: /dev/fb0)
    pub fn new(device: Option<&str>) -> io::Result<Self> {
        let path = device.unwrap_or("/dev/fb0");
        let file = OpenOptions::new().read(true).write(true).open(path)?;

        let mut var_info = FbVarScreeninfo::default();
        let mut fix_info = FbFixScreeninfo::default();

        // Get screen info via ioctl
        unsafe {
            if libc::ioctl(file.as_raw_fd(), FBIOGET_VSCREENINFO, &mut var_info) < 0 {
                return Err(io::Error::last_os_error());
            }
            if libc::ioctl(file.as_raw_fd(), FBIOGET_FSCREENINFO, &mut fix_info) < 0 {
                return Err(io::Error::last_os_error());
            }
        }

        let buffer_size = (var_info.yres * fix_info.line_length) as usize;
        let buffer = vec![0u8; buffer_size];

        Ok(Self {
            file,
            var_info,
            fix_info,
            buffer,
        })
    }

    /// Get framebuffer width in pixels
    pub fn width(&self) -> u32 {
        self.var_info.xres
    }

    /// Get framebuffer height in pixels
    pub fn height(&self) -> u32 {
        self.var_info.yres
    }

    /// Get bits per pixel
    pub fn bpp(&self) -> u32 {
        self.var_info.bits_per_pixel
    }

    /// Check if display is 16-bit RGB565
    pub fn is_rgb565(&self) -> bool {
        self.var_info.bits_per_pixel == 16
    }

    /// Get line length in bytes
    pub fn line_length(&self) -> u32 {
        self.fix_info.line_length
    }

    /// Clear buffer to a color (RGB565)
    pub fn clear(&mut self, color: Rgb565) {
        if self.is_rgb565() {
            let bytes = color.to_ne_bytes();
            for chunk in self.buffer.chunks_exact_mut(2) {
                chunk[0] = bytes[0];
                chunk[1] = bytes[1];
            }
        }
    }

    /// Set a pixel in the buffer (RGB565, no bounds checking for speed)
    #[inline(always)]
    pub fn set_pixel_unchecked(&mut self, x: u32, y: u32, color: Rgb565) {
        let offset = (y * self.fix_info.line_length + x * 2) as usize;
        let bytes = color.to_ne_bytes();
        self.buffer[offset] = bytes[0];
        self.buffer[offset + 1] = bytes[1];
    }

    /// Set a pixel in the buffer (RGB565, with bounds checking)
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb565) {
        if x < self.var_info.xres && y < self.var_info.yres {
            self.set_pixel_unchecked(x, y, color);
        }
    }

    /// Get raw buffer for direct manipulation
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer
    }

    /// Get buffer as RGB565 slice (assumes 16-bit mode)
    pub fn buffer_rgb565_mut(&mut self) -> &mut [Rgb565] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.buffer.as_mut_ptr() as *mut Rgb565,
                self.buffer.len() / 2,
            )
        }
    }

    /// Flush buffer to display
    pub fn flush(&mut self) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(&self.buffer)?;
        Ok(())
    }

    /// Flush with vsync (slower but no tearing)
    pub fn flush_vsync(&mut self) -> io::Result<()> {
        self.flush()?;
        self.file.sync_all()?;
        Ok(())
    }
}

/// Render buffer at internal resolution, then scale to framebuffer
pub struct RenderBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Rgb565>,
}

impl RenderBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height) as usize],
        }
    }

    /// Create with default display resolution
    pub fn default_resolution() -> Self {
        Self::new(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    }

    /// Clear to color
    pub fn clear(&mut self, color: Rgb565) {
        self.pixels.fill(color);
    }

    /// Set pixel
    #[inline(always)]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgb565) {
        if x < self.width && y < self.height {
            self.pixels[(y * self.width + x) as usize] = color;
        }
    }

    /// Get pixel
    #[inline(always)]
    pub fn get_pixel(&self, x: u32, y: u32) -> Rgb565 {
        self.pixels[(y * self.width + x) as usize]
    }

    /// Blit to framebuffer with scaling and optional rotation
    /// rotation: 0 = none, 1 = 90° CW, 2 = 180°, 3 = 90° CCW
    pub fn blit_scaled(&self, fb: &mut Framebuffer, rotation: u8) {
        let fb_w = fb.width();
        let fb_h = fb.height();
        let fb_buf = fb.buffer_rgb565_mut();

        match rotation {
            0 => {
                // No rotation - scale directly
                for fb_y in 0..fb_h {
                    let src_y = (fb_y * self.height / fb_h) as usize;
                    for fb_x in 0..fb_w {
                        let src_x = (fb_x * self.width / fb_w) as usize;
                        let src_idx = src_y * self.width as usize + src_x;
                        let dst_idx = (fb_y * fb_w + fb_x) as usize;
                        fb_buf[dst_idx] = self.pixels[src_idx];
                    }
                }
            }
            3 => {
                // 90° CCW - like the DOOM port
                // FB(x,y) <- Src(y, width-1-x)
                for fb_y in 0..fb_h {
                    let src_x = ((fb_h - 1 - fb_y) * self.width / fb_h) as usize;
                    for fb_x in 0..fb_w {
                        let src_y = (fb_x * self.height / fb_w) as usize;
                        let src_idx = src_y * self.width as usize + src_x;
                        let dst_idx = (fb_y * fb_w + fb_x) as usize;
                        fb_buf[dst_idx] = self.pixels[src_idx];
                    }
                }
            }
            _ => {
                // TODO: implement other rotations if needed
            }
        }
    }
}
