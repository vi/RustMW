
pub const fn sprite8x8(x: &'static str) -> [u8; 8] {
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
    cursor: u8,
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
        unsafe{std::str::from_utf8_unchecked(&self.buf[0..(self.cursor as usize)])}
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

        self.cursor += s.as_bytes().len() as u8;
        Ok(())
    }
}
