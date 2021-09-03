mod wasm4;
use wasm4::*;

const fn sprite8x8(x: &'static str) -> [u8; 8] {
    let mut buf = [0u8; 8];
    let x = x.as_bytes();
    let mut byteidx = 0;
    let mut bitidx = 0;
    let mut i = 0;
    while i < x.len() {
        let chr = x[i];
        match chr {
            b'X' => {
                bitidx += 1;
            }
            b'.' => {
                buf[byteidx] |= 1 << (7-bitidx);
                bitidx += 1;
            }
            _ => (),
        }
        if bitidx >= 8 {
            bitidx = 0;
            byteidx+=1;
        }
        i += 1;
    }
    buf
}

const WHEEL1: [u8; 8] = sprite8x8("
    . X . . X . . .
    . . X X X X . X
    . X . . . . X .
    X X . . . . X .
    . X . . . . X X
    . X . . . . X .
    X . X X X X . .
    . . . X . . X .
");



struct State {
    x: u8,
    y: u8,
    c: u8,
    prevpad: u8,
    frame: u8,
}

impl State {
    pub const fn new() -> State {
        State {
            x: 76,
            y: 76,
            c: 2,
            prevpad: 0,
            frame: 0,
        }
    }

    pub fn tick(&mut self) {
        unsafe {
            *DRAW_COLORS = 2;
            *PALETTE = [0,0xFF, 0xFF00, 0xFF0000];
        }
        text("&format!(\"Qqq {}\", 34.5)", 10, 10);
    
        let gamepad = unsafe { *GAMEPAD1 };
        if (gamepad & !self.prevpad) & BUTTON_1 != 0 {
            self.c += 1;
            if self.c > 4 {
                self.c = 2;
            }
        }
        unsafe { *DRAW_COLORS = self.c as u16; }

        if gamepad & BUTTON_LEFT != 0 { self.x -= 1; }
        if gamepad & BUTTON_RIGHT != 0 { self.x += 1; }
        if gamepad & BUTTON_UP != 0 { self.y -= 1; }
        if gamepad & BUTTON_DOWN != 0 { self.y += 1; }
    
        let bf = if self.frame & 0x1F < 16 { 0 } else { BLIT_FLIP_X };
        blit(&WHEEL1, self.x.into(), self.y.into(), 8, 8, BLIT_1BPP | bf);
        text("Press X to blink", 16, 90);

        self.prevpad = gamepad;
        self.frame = self.frame.wrapping_add(1);
    }
}

static mut STATE: State = State::new();

#[no_mangle]
fn update () {
    unsafe {
        STATE.tick();
    }
}
