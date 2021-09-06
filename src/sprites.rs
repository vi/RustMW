use crate::utils::{sprite8x8,sprite16x16};
pub static _WHEEL: [u8; 8] = sprite8x8(
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

pub static WHEEL1: [u8; 32] = sprite16x16(
    "
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . X . . . X . . . . . . .  
    . . . . . X . . X . . . X . . .
    . . . . . . X X X X . X . . . .
    . . . . . X . . . . X . . . . .
    . . . X X X . . . . X . . . . .
    . . . . . X . . . . X X X . . .
    . . . . . X . . . . X . . . . .
    . . . . X . X X X X . . . . . .
    . . . . . . X . . . X . . . . .
    . . . . . X . . . . . X . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
",
);

pub static WHEEL2: [u8; 32] = sprite16x16(
    "
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . X . . . X . . . . .  
    . . . . . . X . . X . . . . . .
    . . . X . . . X X X . . . . . .
    . . . . X . X . . . X X . . . .
    . . . . . X . . . . . X X X . .
    . . . X X X . . . . . X . . . .
    . . . . . X . . . . X . . . . .
    . . . . X . X X X X . X . . . .
    . . . X . . . X . . . . X . . .
    . . . . . . . X . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
",
);

pub static SOLIDTILE: [u8; 8] = sprite8x8(
    "
    . X . X . X . X
    X . X . X . X .
    . X . X . X . X
    X . X . X . X .
    . X . X . X . X
    X . X . X . X .
    . X . X . X . X
    X . X . X . X .
",
);


pub static JUMPYTILE: [u8; 8] = sprite8x8(
    "
    . . X X . . X X
    X X . . X X . .
    . X . X . X . X
    X . X . X . X .
    . X . X . X . X
    X . X . X . X .
    . X . X . X . X
    X . X . X . X .
",
);

pub static LADDER1: [u8; 8] = sprite8x8(
    "
    . . . . . . . .
    X . . . . . . .
    . X . . . . . .
    X . X . . . . .
    . X . X . . . .
    X . X . X . . .
    . X . X . X . .
    X . X . X . X .
",
);
