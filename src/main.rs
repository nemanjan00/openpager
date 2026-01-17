pub const DISPLAY_WIDTH: u32 = 320;
pub const DISPLAY_HEIGHT: u32 = 200;

mod display;

use display::{rgb_to_565, Display, RenderBuffer};

fn main() {
    println!("OpenPager starting...");
    println!("Internal resolution: {}x{}", DISPLAY_WIDTH, DISPLAY_HEIGHT);

    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}

#[cfg(not(target_arch = "mips"))]
fn run() -> std::io::Result<()> {
    // Windowed mode for development
    let mut display = Display::default_resolution(2)?;
    let mut render = RenderBuffer::default_resolution();

    println!("Window: {}x{}", display.width(), display.height());

    // Draw test pattern
    draw_test_pattern(&mut render);
    render.blit(&mut display);
    display.update()?;

    // Keep window open
    while display.is_open() {
        display.update()?;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    Ok(())
}

#[cfg(target_arch = "mips")]
fn run() -> std::io::Result<()> {
    // Framebuffer mode for embedded
    let mut display = Display::new(None)?;
    let mut render = RenderBuffer::default_resolution();

    println!(
        "Framebuffer: {}x{} @ {}bpp",
        display.width(),
        display.height(),
        display.bpp()
    );

    // Draw test pattern
    draw_test_pattern(&mut render);
    render.blit_scaled(&mut display, 3); // 90° CCW rotation

    println!("Test pattern displayed");
    Ok(())
}

fn draw_test_pattern(render: &mut RenderBuffer) {
    let red = rgb_to_565(255, 0, 0);
    let green = rgb_to_565(0, 255, 0);
    let blue = rgb_to_565(0, 0, 255);
    let white = rgb_to_565(255, 255, 255);

    render.clear(0);

    // Draw colored quadrants
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
}
