use crate::{MainState, wasm4::PALETTE};


pub struct MapViewer {
    
}

impl MapViewer {
    pub const fn new() -> MapViewer {
        MapViewer {

        }
    }

    pub fn tick(&mut self, gamepad: u8, prev_gamepad: u8) -> MainState {
        unsafe {
            *PALETTE = [0x101010, 0x808080, 0xFFFFFF, 0x8080FF];
        }
        MainState::Map
    }
}
