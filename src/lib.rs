mod wasm4;
use wasm4::*;


pub mod utils;

use utils::{sprite8x8, draw_colours, UfmtBuf};

const WHEEL: [u8; 8] = sprite8x8(
    "
    . X . . X . . .
    . . X X X X . X
    . X . . . . X .
    X X . . . . X .
    . X . . . . X X
    . X . . . . X .
    X . X X X X . .
    . . . X . . X .
",
);

const SPRITES: [[u8; 8]; 1] = [WHEEL];



struct Player {
    x: u8,
    y: u8,
    anim_timer: std::num::Wrapping<u8>,
}

impl Player {
    const fn new() -> Player {
        Player {
            x: 70,
            y: 70,
            anim_timer: std::num::Wrapping(0),
        }
    }
    fn control(&mut self, _prev: u8, cur: u8) {
        let mut do_move = false;
        if cur & BUTTON_LEFT != 0 {
            self.x -= 1;
            do_move = true;
        }
        if cur & BUTTON_RIGHT != 0 {
            self.x += 1;
            do_move = true;
        }
        if cur & BUTTON_UP != 0 {
            self.y -= 1;
            do_move = true;
        }
        if cur & BUTTON_DOWN != 0 {
            self.y += 1;
            do_move = true;
        }
        if do_move {
            self.anim_timer += std::num::Wrapping(1);   
        }
    }
    fn draw(&self, _global_frame: u8) {
        draw_colours(3,0,0,0);
        let bf = if self.anim_timer.0 & 0x1F < 16 {
            0
        } else {
            BLIT_FLIP_X
        };
        blit(&WHEEL, self.x.into(), self.y.into(), 8, 8, BLIT_1BPP | bf);
    }
}

struct TextBox {
    c: u8,
}

impl TextBox {
    const fn new() -> TextBox {
        TextBox {
            c: 2,
        }
    }

    fn control(&mut self, prev: u8, cur: u8) {
        if (cur & !prev) & BUTTON_1 != 0 {
            self.c += 1;
            if self.c > 4 {
                self.c = 2;
            }
        }
    }

    fn draw(&self, _global_frame: u8) {
        draw_colours(self.c,0,0,0);
        let mut buf = UfmtBuf::<11>::new();
        let _ = ufmt::uwrite!(buf, "{}", 33);
        text(buf.as_str(), 10, 10);
    }
}

struct State {
    prevpad: u8,
    frame: u8,

    player: Player,
    textbox: TextBox,
}

impl State {
    pub const fn new() -> State {
        State {
            prevpad: 0,
            frame: 0,
            player: Player::new(),
            textbox: TextBox::new(),
        }
    }

    pub fn tick(&mut self) {
        unsafe {
            *PALETTE = [0, 0x808080, 0xFFFFFF, 0x8080FF];
        }

        let gamepad = unsafe { *GAMEPAD1 };
        self.player.control(self.prevpad, gamepad);
        self.textbox.control(self.prevpad, gamepad);

        
        self.player.draw(self.frame);
        self.textbox.draw(self.frame);

        self.prevpad = gamepad;
        self.frame = self.frame.wrapping_add(1);
    }
}

static mut STATE: State = State::new();

#[no_mangle]
fn update() {
    unsafe {
        STATE.tick();
    }
}
