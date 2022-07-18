/// Defines the size of the emulator frame buffer. By spec this is 64x32
pub const FB_SIZE: FramebufSpec = FramebufSpec { x: 64, y: 32 };
pub struct FramebufSpec { pub x: usize, pub y: usize, }

pub type FrameBuffer = [bool; FB_SIZE.y * FB_SIZE.x];


/// The internal emulator framebuffer.
#[derive(Debug)]
pub struct Frame {
    pub data: FrameBuffer,
    pub update: bool,
}

#[allow(clippy::new_without_default)]
impl Frame {
    pub fn new() -> Self {
        let data = [false; FB_SIZE.y * FB_SIZE.x];
        let update = false;
        Frame { data, update }
    }

    pub fn reset(&mut self) {
        self.data.fill(false);
        self.update = false;
    }
}

