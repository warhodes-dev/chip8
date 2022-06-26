use std::error::Error;
use sdl2::{
    video::Window,
    render::Canvas,
    rect::Rect,
    pixels::Color,
};
use crate::emu::frame::{
    FrameBuffer,
    FB_SIZE,
};

pub struct VideoDriver {
    canvas: Canvas<Window>,
    scale: u32,
}

impl VideoDriver {
    pub fn new(sdl_context: &sdl2::Sdl, scale: u32) -> Result<Self, Box<dyn Error>> {

        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Chip-8", FB_SIZE.x as u32 * scale, FB_SIZE.y as u32 * scale)
            .opengl()
            .build()?;

        let mut canvas = window.into_canvas()
            .index(find_sdl_gl_driver().ok_or("No opengl driver")?)
            .build()?;

        log::info!("SDL video subsystem initialized");

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();

        Ok( VideoDriver{ canvas, scale } )
    }

    /// Update the sdl window to correspond to the framebuffer
    pub fn draw(&mut self, framebuf: &FrameBuffer) -> Result<(), Box<dyn Error>> {

        for (x, row) in framebuf.iter().enumerate() {
            for (y, pixel) in row.iter().enumerate() {

                let window_x = x as u32 * self.scale;
                let window_y = y as u32 * self.scale;

                let color = if *pixel {
                    Color::RGB(255, 255, 255)
                } else {
                    Color::RGB(0, 0, 0)
                };

                self.canvas.set_draw_color(color);
                self.canvas.fill_rect(
                    Rect::new(
                        window_x as i32,
                        window_y as i32,
                        (FB_SIZE.x as u32) * self.scale,
                        (FB_SIZE.y as u32) * self.scale,
                    )
                )?;
            }
        }

        self.canvas.present();

        Ok(())
    }
}

/* SDL Helpers */
fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            log::info!("opengl driver identified");
            return Some(index as u32);
        }
    }
    None
}

