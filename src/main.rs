pub const DISPLAY_WIDTH: u32 = 320;
pub const DISPLAY_HEIGHT: u32 = 200;

mod display;

#[cfg(target_os = "linux")]
use display::{rgb_to_565, Framebuffer, RenderBuffer};

fn main() {
    println!("OpenPager starting...");
    println!("Internal resolution: {}x{}", DISPLAY_WIDTH, DISPLAY_HEIGHT);

    #[cfg(target_os = "linux")]
    {
        match run_display() {
            Ok(()) => println!("Display test complete"),
            Err(e) => eprintln!("Display error: {}", e),
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        println!("No display backend available on this platform");
    }
}

#[cfg(target_os = "linux")]
fn run_display() -> std::io::Result<()> {
    // Open framebuffer
    let mut fb = Framebuffer::new(None)?;
    println!(
        "Framebuffer: {}x{} @ {}bpp",
        fb.width(),
        fb.height(),
        fb.bpp()
    );

    // Create render buffer at internal resolution
    let mut render = RenderBuffer::default_resolution();

    // Draw test pattern
    let red = rgb_to_565(255, 0, 0);
    let green = rgb_to_565(0, 255, 0);
    let blue = rgb_to_565(0, 0, 255);
    let white = rgb_to_565(255, 255, 255);

    render.clear(0);

    // Draw colored rectangles
    for y in 0..100 {
        for x in 0..160 {
            render.set_pixel(x, y, red);
        }
        for x in 160..320 {
            render.set_pixel(x, y, green);
        }
    }
    for y in 100..200 {
        for x in 0..160 {
            render.set_pixel(x, y, blue);
        }
        for x in 160..320 {
            render.set_pixel(x, y, white);
        }
    }

    // Blit to framebuffer (90° CCW rotation like DOOM port)
    render.blit_scaled(&mut fb, 3);
    fb.flush()?;

    println!("Test pattern displayed");
    Ok(())
}
