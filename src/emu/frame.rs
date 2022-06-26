/// Defines the size of the emulator frame buffer. By spec this is 64x32
pub const FB_SIZE: FramebufSpec = FramebufSpec { x: 64, y: 32 };
pub struct FramebufSpec { pub x: usize, pub y: usize, }

pub type FrameBuffer = [[bool; FB_SIZE.y]; FB_SIZE.x];


/// The internal emulator framebuffer.
pub struct Frame {
    pub buf: FrameBuffer,
    pub update: bool,
}

#[allow(clippy::new_without_default)]
impl Frame {
    pub fn new() -> Self {
        let buf = [[false; FB_SIZE.y]; FB_SIZE.x];
        let update = false;
        Frame { buf, update }
    }
}

