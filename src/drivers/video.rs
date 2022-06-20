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

//TODO: Make this a config/CLI option
const SCALE: u32 = 8;

pub struct VideoDriver {
    canvas: Canvas<Window>
}

impl VideoDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, Box<dyn Error>> {
        //let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(
                "Chip-8", 
                FB_SIZE.x as u32 * SCALE, 
                FB_SIZE.y as u32 * SCALE,
            )
            .opengl()
            .build()?;
        let mut canvas = window.into_canvas()
            .index(find_sdl_gl_driver().ok_or("No opengl driver")?)
            .build()?;

        // Set screen to all black
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas.present();
        Ok( VideoDriver{ canvas } )
    }

    pub fn draw(&mut self, fb: &FrameBuffer) -> Result<(), Box<dyn Error>> {

        for (x, row) in fb.iter().enumerate() {
            for (y, pixel) in row.iter().enumerate() {

                let window_x = x as u32 * SCALE;
                let window_y = y as u32 * SCALE;

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
                        (FB_SIZE.x as u32) * SCALE,
                        (FB_SIZE.y as u32) * SCALE,
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
            return Some(index as u32);
        }
    }
    None
}

