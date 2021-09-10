
#[macro_export]
macro_rules! tile_types_mapping {
    (convert $chr:ident) => {
        stringify!($chr).as_bytes()[0]
    };
    (convert $chr:literal) => {
        $chr
    };
    ($(($item:ident $($chr:tt)*))*) => {
        [
            $(
                $(
                    crate::MappingBetweenCharAndTileType {
                        chr: tile_types_mapping!(convert $chr),
                        tt: crate::tiles::TileTypeEnum::$item(crate::tiles::$item),
                    }
                ),*
            ),*
        ]
    };
}

#[macro_export]
macro_rules! tile_type {
    ($item:ident) => {crate::tiles::TileTypeEnum::$item(crate::tiles::$item)}
}


#[macro_export]
macro_rules! unique_items_mapping {
    (c2i $item:ident $chr:ident ) => {
        crate::MappingBetweenCharAndItem {
            chr: stringify!($chr).as_bytes()[0],
            item: crate::UniqueItem::$item,
            priority: false,
        }
    };
    (c2i $item:ident ! $chr:ident ) => {
        crate::MappingBetweenCharAndItem {
            chr: stringify!($chr).as_bytes()[0],
            item: crate::UniqueItem::$item,
            priority: true,
        }
    };
    (c2i  $item:ident $chr:literal) => {
        crate::MappingBetweenCharAndItem {
            chr: $chr,
            item: crate::UniqueItem::$item,
            priority: false,
        }
    };
    (c2i $item:ident ! $chr:literal) => {
        crate::MappingBetweenCharAndItem {
            chr: $chr,
            item: crate::UniqueItem::$item,
            priority: true,
        }
    };

    (convert $chr:ident) => {
        stringify!($chr).as_bytes()[0]
    };
    (convert $chr:literal) => {
        $chr
    };
    ($( (  $($x:tt)* ) )*) => {
        [
            $(
                unique_items_mapping!(c2i $($x)*)
            ),*
        ]
    };
}


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

use crate::{Area, AreaSource, CharDescription, Level, LowlevelCellType, MAX_UNIQUE_ITEM_POSITIONS, MappingBetweenCharAndItem, MappingBetweenCharAndTileType, RoomBlock, RoomMetadata, TilePos, UniqueItem, UniqueItemPosition, UniqueItemPositionLowlevel, UniqueItemPositions, level, tiles::{TileTypeEnum, tile_type_enum_eq}}; 

const fn lookup_char<const N:usize>(c: u8, char_lookup:[CharDescription; N]) -> CharDescription {
    let mut j = 0;
    while j < char_lookup.len() {
        if char_lookup[j].chr == c {
            return char_lookup[j];
        }
        j+=1;
    }
    loop{
        b"Special character type not found"[999];
    }
}

const fn lookup_tt<const N:usize>(c: u8, lookup:[MappingBetweenCharAndTileType; N]) -> TileTypeEnum {
    let mut j = 0;
    while j < lookup.len() {
        if lookup[j].chr == c {
            return lookup[j].tt;
        }
        j+=1;
    }
    loop{
        b"Tile type not found for this charater"[999];
    }
}

pub const fn makearea<const C:usize, const T:usize, const I:usize>(src: AreaSource<C,T,I>) -> (RoomBlock, [Option<UniqueItemPositionLowlevel>; 32], [RoomMetadata; 32]) {
    let mut buf = [[0u32; 16]; 32];
    let mut special_positions = [None; 32];
    let mut special_position_index = 0;

    let mut lineidx = 0;
    let mut cellidx = 0;

    // stores whether we are currently parsing text between the `|` markers
    let mut within_active_area = false;

    let s = src.cells;

    let mut meta = [RoomMetadata {
        block_type_sp: Some(src.empty_tile_style),
        block_type_x: Some(src.solid_tile_style),
        block_type_a: None,
        block_type_b: None,
    }; 32];

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
                
                let room_x = cellidx / 16;
                let room_y = lineidx / 8;
                let roomidx = (room_y * 8 + room_x) as usize;

                use LowlevelCellType::*;
                let (upper,  lower) = match chr {
                    b' ' => (Empty, Empty),
                    b'`' => (Solid, Empty),
                    b',' => (Empty, Solid),
                    b'X' => (Solid, Solid),
                    _ => {
                        let info = lookup_char(chr, src.char_lookup);
                        
                        match (info.upper, info.lower) {
                            (CustomA, CustomB) | (CustomB, CustomA) => {
                                b"This combination of low-level cell types is not allowed"[999];
                            }
                            _ => (),
                        }

                        match info.upper {
                            Empty => (),
                            Solid => (),
                            Special => (),
                            CustomA => {
                                let tt = lookup_tt(chr, src.tile_lookup);
                                match  meta[roomidx].block_type_a {
                                    None => meta[roomidx].block_type_a = Some(tt),
                                    Some(x) if tile_type_enum_eq(x, tt) => (),
                                    _ => {
                                        b"Room overloaded with special tile types for type A"[999];
                                    }
                                }
                            }
                            CustomB => {
                                let tt = lookup_tt(chr, src.tile_lookup);
                                match  meta[roomidx].block_type_b {
                                    None => meta[roomidx].block_type_b = Some(tt),
                                    Some(x) if tile_type_enum_eq(x, tt) => (),
                                    _ => {
                                        b"Room overloaded with special tile types for type B"[999];
                                    }
                                }
                            }
                        }

                        match info.lower {
                            Empty => (),
                            Solid => (),
                            Special => (),
                            CustomA => {
                                let tt = lookup_tt(chr, src.tile_lookup);
                                match  meta[roomidx].block_type_a {
                                    None => meta[roomidx].block_type_a = Some(tt),
                                    Some(x) if tile_type_enum_eq(x, tt) => (),
                                    _ => {
                                        b"Room overloaded with special tile types for type A"[999];
                                    }
                                }
                            }
                            CustomB => {
                                let tt = lookup_tt(chr, src.tile_lookup);
                                match  meta[roomidx].block_type_b {
                                    None => meta[roomidx].block_type_b = Some(tt),
                                    Some(x) if tile_type_enum_eq(x, tt) => (),
                                    _ => {
                                        b"Room overloaded with special tile types for type B"[999];
                                    }
                                }
                            }
                        }
                       
                        (info.upper, info.lower)
                    }
                };
                
                let within_room_x = cellidx % 16;
                let within_room_y = lineidx % 8;

                if matches!(upper, Special) {
                    special_positions[special_position_index] = Some(UniqueItemPositionLowlevel {
                        chr,
                        pos: (cellidx, 2*lineidx),
                        priority: false,
                    });
                    special_position_index+=1;
                }
                if matches!(lower, Special) {
                    special_positions[special_position_index] = Some(UniqueItemPositionLowlevel {
                        chr,
                        pos: (cellidx, 2*lineidx+1),
                        priority: false,
                    });
                    special_position_index+=1;
                }

                buf[roomidx][(2*within_room_y+0) as usize] |= (upper.ll_code() as u32) << (within_room_x*2);
                buf[roomidx][(2*within_room_y+1) as usize] |= (lower.ll_code() as u32) << (within_room_x*2);


                cellidx+=1;
            }
            _ => (),
        }
        i += 1;
    }  
    if lineidx != 32 {
        b"There must by exactly 32 lines in each area"[999];
    }

    (buf, special_positions, meta)
}


impl Area {
    pub const fn build<const C: usize, const T: usize, const I:usize>(src: AreaSource<C,T,I>) -> (Area, UniqueItemPositions) {  
        let item_lookup = src.item_lookup;   
        let (rooms, specials_ll, meta) = makearea(src);

        let mut specials = [None; MAX_UNIQUE_ITEM_POSITIONS];

        let mut i = 0;
        let mut j: usize = 0;
        while i < specials_ll.len() {
            if let Some(spcll) = specials_ll[i] {
                let chr = spcll.chr;
                let mut k = 0;
                let mut found = false;

                while k < item_lookup.len() {
                    let MappingBetweenCharAndItem { chr: m_chr, item, priority } = item_lookup[k];
                    if m_chr == chr {
                        found = true;

                        specials[j] = Some(UniqueItemPosition{item, pos:spcll.pos, priority});
                        j+=1;

                        break;
                    }
                    k += 1;
                }

                if !found {
                    b"Encountered unique item character that is not mapped to UniquItem"[999];
                }
            }
            i+=1;
        }

        (Area {
            rooms,
            meta,
        }, specials)
    }
}

impl Level {
    pub const fn new() -> Level {
        let mut unique_items = [(UniqueItem::PlayerStart, (0,0)); UniqueItem::VARIANT_COUNT];
        let mut prioritized = [false; UniqueItem::VARIANT_COUNT];

        let mut i = 0;

        let mut j = 0;
        let specials = level::AREA1.1;
        while j < specials.len() {
            if let Some(UniqueItemPosition { item, pos, priority}) = specials[j] {
                let mut insert_at_the_end = true;

                let mut k = 0;
                while k < i {
                    if unique_items[k].0 as u8== item as u8 {
                        insert_at_the_end = false;
                        match (priority, prioritized[k]) {
                            (false, false) => {b"Duplicate position for an unique item"[999];}
                            (false, true) => (), // silently ignore non-priority position when priority one is already set
                            (true, false) => {
                                unique_items[k].1 = pos;
                                prioritized[k] = true;
                            }
                            (true, true) => {b"Duplicate priority position for an unique item"[999];}
                        }
                    }
                    k+=1;
                }
                
                if insert_at_the_end {
                    unique_items[i].0 = item;
                    unique_items[i].1 = pos;
                    prioritized[i] = priority;
                    i+=1;
                }
            }
            j += 1;
        }
        if i != UniqueItem::VARIANT_COUNT {
            b"There is a missing unique item on the level"[999];
        }

        Level {
            the_area: level::AREA1.0,
            unique_items,
        }
    }

    pub const fn unique_item_pos(&self, item: UniqueItem) -> TilePos {
        let mut i=0;
        while i < self.unique_items.len() {
            if item as u8 == self.unique_items[i].0 as u8 {
                return self.unique_items[i].1;
            }
            i+=1;
        }
        #[allow(unconditional_panic)]
        b"Internal error: Level::new should have caught missing item position"[999];
        (0,0)
    }
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
            let mut buf = crate::utils::UfmtBuf::<30>::new();
            let _ = ::ufmt::uwrite!(
                buf,
                $fmt,
                $($args)*
            );
            crate::wasm4::trace(buf.as_str());
        }
    }
}

pub const fn ll_char_descriptions<const N: usize>(specifier: &'static [u8]) -> [CharDescription; N] {
    let mut v = [CharDescription{chr: b'?', upper: LowlevelCellType::Empty, lower: LowlevelCellType::Empty}; N];
    let mut i = 0;
    let mut j = 0;
    
    enum S {
        Idle,
        Upper,
        Lower,
    }
    let mut s = S::Idle;
    while i < specifier.len() {
        s = match s{
            S::Idle =>  match specifier[i] {
                b' ' | b'\n' | b'\t' => S::Idle,
                x => {
                    v[j].chr = x;
                    S::Upper
                }
            },
            S::Upper | S::Lower => {
                let t = match specifier[i] {
                    b'.' => LowlevelCellType::Empty,
                    b'X' => LowlevelCellType::Solid,
                    b'A' => LowlevelCellType::CustomA,
                    b'B' => LowlevelCellType::CustomB,
                    b'!' => LowlevelCellType::Special,
                    _ => {
                        b"Invalid low-level cell type letter"[999];
                        LowlevelCellType::Empty
                    }
                };
                match s {
                    S::Idle => S::Idle, // actually unreachable!(),
                    S::Upper => {
                        v[j].upper = t;
                        S::Lower
                    }
                    S::Lower => {
                        v[j].lower = t;
                        j+=1;
                        S::Idle
                    }
                }
            }
        };
        i+=1;
    }
    if j != v.len() {
        b"Mismatch between declared length of low-level cell types and actual number of letter triplets"[999];
    }
    v
}
