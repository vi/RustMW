
pub const fn sprite8x8(x: &'static str) -> [u8; 8] {
    let mut buf = [0u8; 8];
    let s = x.as_bytes();
    let mut byteidx = 0;
    let mut bitidx = 0;
    let mut i = 0;
    while i < s.len() {
        let chr = s[i];
        match chr {
            b'X' => {
                bitidx += 1;
            }
            b'.' => {
                buf[byteidx] |= 1 << (7 - bitidx);
                bitidx += 1;
            }
            _ => (),
        }
        if bitidx >= 8 {
            bitidx = 0;
            byteidx += 1;
        }
        i += 1;
    }
    buf
}

pub const fn sprite16x16(x: &'static str) -> [u8; 32] {
    let mut buf = [0u8; 32];
    let s = x.as_bytes();
    let mut byteidx = 0;
    let mut bitidx = 0;
    let mut i = 0;
    while i < s.len() {
        let chr = s[i];
        match chr {
            b'X' => {
                bitidx += 1;
            }
            b'.' => {
                buf[byteidx] |= 1 << (7 - bitidx);
                bitidx += 1;
            }
            _ => (),
        }
        if bitidx >= 8 {
            bitidx = 0;
            byteidx += 1;
        }
        i += 1;
    }
    buf
}

pub const fn room16x16(s: &'static [u8]) -> [u32; 16] {
    let mut buf = [0u32; 16];

    let mut lineidx = 0;
    let mut cellidx = 0;
    let mut within_room_area = false;

    let mut i = 0;
    while i < s.len() {
        let chr = s[i];
        match chr {
            b'|' => {
                within_room_area = !within_room_area;

                if within_room_area {
                    // began the line
                    if lineidx >= 8 {
                        b"There must by exactly 8 lines in each room"[999];
                    }
                } else {
                    // finished the line
                    if cellidx != 16 {
                        b"Each line of the room but by eactly 16 characters long"[999];
                    }
                    cellidx = 0;
                    lineidx += 1;
                }
            }
            _ if within_room_area => {
                if cellidx >= 16 {
                    b"Each line of the room but by eactly 16 characters long"[999];
                }
                match chr {
                    b' ' => {
                        buf[2*lineidx+0] |= 0b00 << (cellidx*2);
                        buf[2*lineidx+1] |= 0b00 << (cellidx*2);
                    }
                    b'`' => {
                        buf[2*lineidx+0] |= 0b01 << (cellidx*2);
                        buf[2*lineidx+1] |= 0b00 << (cellidx*2);
                    }
                    b',' => {
                        buf[2*lineidx+0] |= 0b00 << (cellidx*2);
                        buf[2*lineidx+1] |= 0b01 << (cellidx*2);
                    }
                    b'X' => {
                        buf[2*lineidx+0] |= 0b01 << (cellidx*2);
                        buf[2*lineidx+1] |= 0b01 << (cellidx*2);
                    }
                    _ => {
                        b"Undefined character encountered within the room area"[999];
                    }
                }
                cellidx+=1;
            }
            _ => (),
        }
        i += 1;
    }  
    if lineidx != 8 {
        b"There must by exactly 8 lines in each room"[999];
    }

    buf
}

use crate::{CharDescription, RoomBlock, SpecialPosition, LowlevelCellType}; 

const fn lookup_char<const N:usize>(c: u8, char_lookup:[CharDescription; N]) -> CharDescription {
    let mut j = 0;
    while j < char_lookup.len() {
        j+=1;
        if char_lookup[j].chr == c {
            return char_lookup[j];
        }
    }
    loop{
        b"Special character type not found"[999];
    }
}

pub const fn makearea<const N:usize>(s: &'static [u8], char_lookup:[CharDescription; N]) -> (RoomBlock, [Option<SpecialPosition>; 32]) {
    let mut buf = [[0u32; 16]; 32];
    let mut special_positions = [None; 32];
    let mut special_position_index = 0;

    let mut lineidx = 0;
    let mut cellidx = 0;

    // stores whether we are currently parsing text between the `|` markers
    let mut within_active_area = false;


    let mut i = 0;
    while i < s.len() {
        let chr = s[i];
        match chr {
            b'|' => {
                within_active_area = !within_active_area;

                if within_active_area {
                    // began the line
                    if lineidx >= 32 {
                        b"There must by exactly 32 lines in each area"[999];
                    }
                } else {
                    // finished the line
                    if cellidx != 128 {
                        b"Each line of the area must be exactly 128 characters long"[999];
                    }
                    cellidx = 0;
                    lineidx += 1;
                }
            }
            _ if within_active_area => {
                if cellidx >= 128 {
                    b"Each line of the area must be exactly 128 characters long"[999];
                }
                
                use LowlevelCellType::*;
                let (upper,  lower) = match chr {
                    b' ' => (Empty, Empty),
                    b'`' => (Solid, Empty),
                    b',' => (Empty, Solid),
                    b'X' => (Solid, Solid),
                    _ => {
                        let info = lookup_char(chr, char_lookup);
                        (info.upper, info.lower)
                    }
                };
                
                let room_x = cellidx / 16;
                let room_y = lineidx / 8;
                let within_room_x = cellidx % 16;
                let within_room_y = lineidx % 8;

                if matches!(upper, Special) {
                    special_positions[special_position_index] = Some(SpecialPosition {
                        chr,
                        x: cellidx,
                        y: 2*lineidx,
                    });
                    special_position_index+=1;
                }
                if matches!(lower, Special) {
                    special_positions[special_position_index] = Some(SpecialPosition {
                        chr,
                        x: cellidx,
                        y: 2*lineidx+1,
                    });
                    special_position_index+=1;
                }

                buf[(room_y*8+room_x) as usize][(2*within_room_y+0) as usize] |= (upper.ll_code() as u32) << (within_room_x*2);
                buf[(room_y*8+room_x) as usize][(2*within_room_y+1) as usize] |= (lower.ll_code() as u32) << (within_room_x*2);


                cellidx+=1;
            }
            _ => (),
        }
        i += 1;
    }  
    if lineidx != 32 {
        b"There must by exactly 32 lines in each area"[999];
    }

    (buf, special_positions)
}



#[inline]
pub fn draw_colours(c0: u8, c1: u8, c2: u8, c3: u8) {
    unsafe {
        *crate::wasm4::DRAW_COLORS = 
            (((c0 & 0xF) as u16) << 0)
            |
            (((c1 & 0xF) as u16) << 4)
            |
            (((c2 & 0xF) as u16) << 8)
            |
            (((c3 & 0xF) as u16) << 12);
    }
}

pub struct UfmtBuf<const N: usize> {
    cursor: u16,
    buf: [u8; N],
}
impl<const N:usize> UfmtBuf<N> {
    pub fn new() -> Self {
        Self {
            cursor: 0,
            buf: [0u8; N],
        }
    }
    pub fn as_str(&self) -> &str {
        //std::hint::black_box( [0u8; 33]);
        unsafe{std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.buf.as_ptr(), self.cursor as usize))}
    }
}
impl<const N:usize> ufmt::uWrite for UfmtBuf<N> {
    type Error = ();
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        if self.cursor as usize + s.len() > N {
            return Ok(())
        }

        //self.buf[(self.cursor as usize)..(self.cursor as usize + s.as_bytes().len())].copy_from_slice(s.as_bytes());

        // safety: wasm4's safety standards are lax, so just using it to avoid panic handling code.
        unsafe {
            std::ptr::copy_nonoverlapping(s.as_bytes().as_ptr(), self.buf.as_mut_ptr().offset(self.cursor as isize), s.as_bytes().len());
        }

        self.cursor += s.as_bytes().len() as u16;
        Ok(())
    }
}

#[macro_export]
macro_rules! traceln {
    ($fmt:literal, $($args:tt)*) => {
        {
            let mut buf = UfmtBuf::<30>::new();
            let _ = ::ufmt::uwrite!(
                buf,
                $fmt,
                $($args)*
            );
            trace(buf.as_str());
        }
    }
}
