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

const SCALE: u32 = 6;
const X_RES: u32 = 64 * SCALE;
const Y_RES: u32 = 32 * SCALE;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr $(,)?) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub struct VideoDriver {
    canvas: Canvas<Window>,
}

impl VideoDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, Box<dyn Error>> {

        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("Chip-8", X_RES, Y_RES)
            .opengl()
            .build()?;

        //window.set_bordered(false);

        let mut canvas = window.into_canvas()
            .index(find_sdl_gl_driver().ok_or("No opengl driver")?)
            .build()?;

        log::info!("SDL video subsystem initialized");

        canvas.set_draw_color(Color::RGB(145, 145, 135));
        canvas.fill_rect(rect!(0, 0, X_RES, Y_RES))?;

        canvas.present();

        Ok( VideoDriver{ canvas } )
    }

    /// Draws the entire application screen
    pub fn draw(&mut self, fb: &FrameBuffer) -> Result<(), Box<dyn Error>> {
        self.canvas.set_draw_color(Color::RGB(200,190,150));
        self.canvas.fill_rect(rect!(0, 0, X_RES, Y_RES))?;
        self.draw_screen(fb)?;
        // TODO: Add more components to this
        self.canvas.present();
        Ok(())
    }

    /// Update the screen subframe to correspond to the framebuffer
    fn draw_screen(&mut self, framebuf: &FrameBuffer) -> Result<(), Box<dyn Error>> {
        self.canvas.set_draw_color(Color::RGB(255,0,0));

        for (y, row) in framebuf.chunks_exact(FB_SIZE.x).enumerate() {
            for (x, pixel) in row.iter().enumerate() {

                let window_x = x as u32 * SCALE;
                let window_y = y as u32 * SCALE;

                let color = if *pixel {
                    Color::RGB(32, 42, 52)
                } else {
                    Color::RGB(145, 145, 135)
                };

                self.canvas.set_draw_color(color);
                self.canvas.fill_rect(rect!(window_x, window_y, SCALE, SCALE))?;
            }
        }


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

