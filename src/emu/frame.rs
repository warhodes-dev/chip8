/// Defines the size of the emulator frame buffer. By spec this is 64x32
pub const FB_SIZE: FramebufSpec = FramebufSpec { x: 64, y: 32 };
pub struct FramebufSpec { pub x: usize, pub y: usize, }

/// The internal emulator framebuffer type.
pub type FrameBuf = [[bool; FB_SIZE.x]; FB_SIZE.y];

