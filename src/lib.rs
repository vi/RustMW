mod wasm4;
use wasm4::*;

const SMILEY: [u8; 8] = [
    0b11000011,
    0b10000001,
    0b00100100,
    0b00100100,
    0b00000000,
    0b00100100,
    0b10011001,
    0b11000011,
];

struct State {
    x: u8,
    y: u8,
}

impl State {
    pub const fn new() -> State {
        State {
            x: 76,
            y: 76,
        }
    }

    pub fn tick(&mut self) {
        unsafe { *DRAW_COLORS = 2 }
        text("&format!(\"Qqq {}\", 34.5)", 10, 10);
    
        let gamepad = unsafe { *GAMEPAD1 };
        if gamepad & BUTTON_1 != 0 {
            unsafe { *DRAW_COLORS = 4 }
        }

        if gamepad & BUTTON_LEFT != 0 { self.x -= 1; }
        if gamepad & BUTTON_RIGHT != 0 { self.x += 1; }
        if gamepad & BUTTON_UP != 0 { self.y -= 1; }
        if gamepad & BUTTON_DOWN != 0 { self.y += 1; }
    
        blit(&SMILEY, self.x.into(), self.y.into(), 8, 8, BLIT_1BPP);
        text("Press X to blink", 16, 90);
    }
}

static mut STATE: State = State::new();

#[no_mangle]
fn update () {
    unsafe {
        STATE.tick();
    }
}
