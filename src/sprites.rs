use crate::utils::{sprite8x8,sprite16x16};
pub static WHEEL_S: [u8; 8] = sprite8x8(
    b"
    |. X . . X . . .|
    |. . X X X X . X|
    |. X . . . . X .|
    |X X . . . . X .|
    |. X . . . . X X|
    |. X . . . . X .|
    |X . X X X X . .|
    |. . . X . . X .|
",
);

pub static WHEEL1: [u8; 32] = sprite16x16(
    b"
    |. . . . . . . . . . . . . . . .|
    |. . . . . . . . . . . . . . . .|
    |. . . . . . . . . . . . . . . .|
    |. . . . X . . . X . . . . . . .|  
    |. . . . . X . . X . . . X . . .|
    |. . . . . . X X X X . X . . . .|
    |. . . . . X . . . . X . . . . .|
    |. . . X X X . . . . X . . . . .|
    |. . . . . X . . . . X X X . . .|
    |. . . . . X . . . . X . . . . .|
    |. . . . X . X X X X . . . . . .|
    |. . . . . . X . . . X . . . . .|
    |. . . . . X . . . . . X . . . .|
    |. . . . . . . . . . . . . . . .|
    |. . . . . . . . . . . . . . . .|
    |. . . . . . . . . . . . . . . .|
",
);

pub static WHEEL2: [u8; 32] = sprite16x16(
    b"
    |. . . . . . . . . . . . . . . .|
    |. . . . . . . . . . . . . . . .|
    |. . . . . . . . . . . . . . . .|
    |. . . . . . X . . . X . . . . .|  
    |. . . . . . X . . X . . . . . .|
    |. . . X . . . X X X . . . . . .|
    |. . . . X . X . . . X X . . . .|
    |. . . . . X . . . . . X X X . .|
    |. . . X X X . . . . . X . . . .|
    |. . . . . X . . . . X . . . . .|
    |. . . . X . X X X X . X . . . .|
    |. . . X . . . X . . . . X . . .|
    |. . . . . . . X . . . . . . . .|
    |. . . . . . . . . . . . . . . .|
    |. . . . . . . . . . . . . . . .|
    |. . . . . . . . . . . . . . . .|
",
);

pub static SOLIDTILE: [u8; 8] = sprite8x8(
    b"
    |. X . X . X . X|
    |X . X . X . X .|
    |. X . X . X . X|
    |X . X . X . X .|
    |. X . X . X . X|
    |X . X . X . X .|
    |. X . X . X . X|
    |X . X . X . X .|
",
);


pub static JUMPYTILE: [u8; 8] = sprite8x8(
    b"
    |. . X X . . X X|
    |X X . . X X . .|
    |. X . X . X . X|
    |X . X . X . X .|
    |. X . X . X . X|
    |X . X . X . X .|
    |. X . X . X . X|
    |X . X . X . X .|
",
);

pub static LADDER1: [u8; 8] = sprite8x8(
    b"
    |. . . . . . . .|
    |X . . . . . . .|
    |. X . . . . . .|
    |X . X . . . . .|
    |. X . X . . . .|
    |X . X . X . . .|
    |. X . X . X . .|
    |X . X . X . X .|
",
);
 



pub static INFOBOX1: [u8; 8] = sprite8x8(
    b"
    |. . .   X . . .|
    |. X           X|
    |. X   X X     X|
    |. X     X     X|
    |. X   X X X   X|
    |. X           X|
    |.   X X X X X .|
    |. . . . . . . .|
",
);


pub static INFOBOX2: [u8; 8] = sprite8x8(
    b"
    |. . .   X . . .|
    |.              |
    |.     X X      |
    |.       X      |
    |.     X X X    |
    |.              |
    |.             .|
    |. . . . . . . .|
",
);



pub static STAR1: [u8; 8] = sprite8x8(
    b"
    |. . . X   . . .|
    |.     X X      |
    |X X X X X X X X|
    |. X X X X X X  |
    |.   X X X X    |
    |. X X   X X X  |
    |. X X     X X .|
    |X . . . . . . X|
",
);


pub static STAR2: [u8; 8] = sprite8x8(
    b"
    |. . X     . .  |
    |.   X X   X X  |
    |    X X X X X  |
    |. X X X X X    |
    |X X X X X X X  |
    |.   X X X X X X|
    |.   X X   X   .|
    |    X . . . .  |
",
);

pub static CRATE: [u8; 8] = sprite8x8(
    b"
    |. . X X X X X X|
    |. X         X X|
    |X X X X X X   X|
    |X         X   X|
    |X         X   X|
    |X         X   X|
    |X         X X .|
    |X X X X X X .  |
",
);
