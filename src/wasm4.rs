//
// WASM-4: https://wasm4.org/docs

// Use `wee_alloc` as the global allocator.
extern crate wee_alloc;
#[global_allocator] static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │ Platform Constants                                                        │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘

pub const SCREEN_SIZE: u32 = 160;

// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │ Memory Addresses                                                          │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘

pub static mut PALETTE: *mut [u32; 4] = 0x04 as *mut [u32; 4];
pub const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;
pub const GAMEPAD1: *const u8 = 0x16 as *const u8;
pub const GAMEPAD2: *const u8 = 0x17 as *const u8;
pub const GAMEPAD3: *const u8 = 0x18 as *const u8;
pub const GAMEPAD4: *const u8 = 0x19 as *const u8;
pub const MOUSE_X: *const i16 = 0x1a as *const i16;
pub const MOUSE_Y: *const i16 = 0x1c as *const i16;
pub const MOUSE_BUTTONS: *const u8 = 0x1e as *const u8;
pub static mut FRAMEBUFFER: *mut [u8; 6400] = 0xa0 as *mut [u8; 6400];

pub const BUTTON_1: u8 = 1;
pub const BUTTON_2: u8 = 2;
pub const BUTTON_LEFT: u8 = 16;
pub const BUTTON_RIGHT: u8 = 32;
pub const BUTTON_UP: u8 = 64;
pub const BUTTON_DOWN: u8 = 128;

// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │ Drawing Functions                                                         │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘

/// Copies pixels to the framebuffer.
pub fn blit (sprite: &[u8], x: i32, y: i32, width: u32, height: u32, flags: u32) {
    unsafe { extern_blit(sprite.as_ptr(), x, y, width, height, flags) }
}
extern {
    #[link_name = "blit"]
    fn extern_blit (sprite: *const u8, x: i32, y: i32, width: u32, height: u32, flags: u32);
}

/// Copies a subregion within a larger sprite atlas to the framebuffer.
pub fn blit_sub (sprite: &[u8], x: i32, y: i32, width: u32, height: u32, src_x: u32, src_y: u32, stride: u32, flags: u32) {
    unsafe { extern_blit_sub(sprite.as_ptr(), x, y, width, height, src_x, src_y, stride, flags) }
}
extern {
    #[link_name = "blitSub"]
    fn extern_blit_sub (sprite: *const u8, x: i32, y: i32, width: u32, height: u32, src_x: u32, src_y: u32, stride: u32, flags: u32);
}

pub const BLIT_2BPP: u32 = 1;
pub const BLIT_1BPP: u32 = 0;
pub const BLIT_FLIP_X: u32 = 2;
pub const BLIT_FLIP_Y: u32 = 4;
pub const BLIT_ROTATE: u32 = 8;

/// Draws a line between two points.
pub fn line (x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe { extern_line(x1, y1, x2, y2) }
}
extern {
    #[link_name = "line"]
    fn extern_line (x1: i32, y1: i32, x2: i32, y2: i32);
}

/// Draws an oval (or circle).
pub fn oval (x: i32, y: i32, width: u32, height: u32) {
    unsafe { extern_oval(x, y, width, height) }
}
extern {
    #[link_name = "oval"]
    fn extern_oval (x: i32, y: i32, width: u32, height: u32);
}

/// Draws a rectangle.
pub fn rect (x: i32, y: i32, width: u32, height: u32) {
    unsafe { extern_rect(x, y, width, height) }
}
extern {
    #[link_name = "rect"]
    fn extern_rect (x: i32, y: i32, width: u32, height: u32);
}

/// Draws text using the built-in system font.
pub fn text (text: &str, x: i32, y: i32) {
    unsafe { extern_text(text.as_ptr(), text.len(), x, y) }
}
extern {
    #[link_name = "textUtf8"]
    fn extern_text (text: *const u8, length: usize, x: i32, y: i32);
}

// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │ Sound Functions                                                           │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘

/// Plays a sound tone.
pub fn tone (frequency: u32, volume: u32, duration: u32, flags: u32) {
    unsafe { extern_tone(frequency, volume, duration, flags) }
}
extern {
    #[link_name = "tone"]
    fn extern_tone (frequency: u32, volume: u32, duration: u32, flags: u32);
}

pub const TONE_PULSE1: u32 = 0;
pub const TONE_PULSE2: u32 = 1;
pub const TONE_TRIANGLE: u32 = 2;
pub const TONE_NOISE: u32 = 3;
pub const TONE_MODE1: u32 = 0;
pub const TONE_MODE2: u32 = 4;
pub const TONE_MODE3: u32 = 8;
pub const TONE_MODE4: u32 = 12;

// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │ Storage Functions                                                         │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘

extern {
    /// Reads up to `size` bytes from persistent storage into the pointer `dest`.
    pub fn diskr (dest: *mut u8, size: u32) -> u32;

    /// Writes up to `size` bytes from the pointer `src` into persistent storage.
    pub fn diskw (src: *const u8, size: u32) -> u32;
}

// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │ Other Functions                                                           │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘

extern {
    /// Copies `size` bytes from `srcPtr` into `destPtr`.
    #[link_name = "memcpy"]
    pub fn memcpy (dest: *mut u8, src: *const u8, size: usize) -> usize;

    /// Fills memory at `destPtr` with `size` bytes of the fixed value `value`.
    #[link_name = "memset"]
    pub fn memset (dest: *mut u8, byte: u8, size: usize) -> usize;
}

/// Prints a message to the debug console.
pub fn trace (text: &str) {
    unsafe { extern_trace(text.as_ptr(), text.len()) }
}
extern {
    #[link_name = "traceUtf8"]
    fn extern_trace (trace: *const u8, length: usize);
}
