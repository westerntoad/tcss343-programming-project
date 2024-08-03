use std::{ 
    fmt,
    ops::BitAnd,
    ops::BitAndAssign,
    ops::BitOr,
    ops::BitOrAssign,
    ops::BitXor,
    ops::Not
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(u64);

#[allow(dead_code)]
impl Bitboard {
    pub const fn new(val: u64) -> Bitboard {
        Self(val)
    }

    pub fn val(&self) -> u64 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        &self.0 == &0
    }

    pub fn nort_one(&self) -> Bitboard {
        Self(&self.0 >> 8)
    }
    
    pub fn east_one(&self) -> Bitboard {
        Self(&self.0 << 1) & !Self::A_FILE
    }
    
    pub fn sout_one(&self) -> Bitboard {
        Self(&self.0 << 8)
    }

    pub fn west_one(&self) -> Bitboard {
        Self(&self.0 >> 1) & !Self::H_FILE
    }
}

// BIT OPERATIONS :
impl BitAnd for Bitboard {
    type Output = Self;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str("\n");
        let bytes = self.0.to_be_bytes();

        for i in 0..64 {
            let bit = (bytes[7 - (i / 8)] >> (i % 8)) % 2;

            output.push_str(&bit.to_string());

            output.push(' ');
            if i % 8 == 7 {
                output.push('\n');
            }
        }

        output.push_str(&format!("\nHex:\n{:#016x}", self.0));
        write!(f, "{}", output)
    }
}
//impl fmt::Debug for Bitboard {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        let mut output = String::new();
//        
//        for i in 1..=8 {
//            let val = (self.0 >> 64 - 8 * i) % 0x100;
//            output.push_str(format!("{:08b}\n", val).as_str());
//        }
//
//        write!(f, "{}", output)
//    }
//}

#[allow(dead_code)]
impl Bitboard {
    pub const EMPTY: Bitboard =             Bitboard(0);
    pub const UNIVERSE: Bitboard =          Bitboard(u64::MAX);

    pub const A_FILE: Bitboard =            Bitboard(0x01_01_01_01_01_01_01_01);
    pub const H_FILE: Bitboard =            Bitboard(0x80_80_80_80_80_80_80_80);

    pub const STARTING_WHITE: Bitboard =    Bitboard(0x00_00_00_00_00_00_ff_ff);
    pub const STARTING_BLACK: Bitboard =    Bitboard(0xff_ff_00_00_00_00_00_00);
    pub const STARTING_PAWNS: Bitboard =    Bitboard(0x00_ff_00_00_00_00_ff_00);
    pub const STARTING_KNIGHTS: Bitboard =  Bitboard(0x42_00_00_00_00_00_00_42);
    pub const STARTING_BISHOPS: Bitboard =  Bitboard(0x24_00_00_00_00_00_00_24);
    pub const STARTING_ROOKS: Bitboard =    Bitboard(0x81_00_00_00_00_00_00_81);
    pub const STARTING_QUEENS: Bitboard =   Bitboard(0x08_00_00_00_00_00_00_08);
    pub const STARTING_KINGS: Bitboard =    Bitboard(0x10_00_00_00_00_00_00_10);

    pub const A8: Bitboard =               Bitboard(0x00000000000001);
    pub const B8: Bitboard =               Bitboard(0x00000000000002);
    pub const C8: Bitboard =               Bitboard(0x00000000000004);
    pub const D8: Bitboard =               Bitboard(0x00000000000008);
    pub const E8: Bitboard =               Bitboard(0x00000000000010);
    pub const F8: Bitboard =               Bitboard(0x00000000000020);
    pub const G8: Bitboard =               Bitboard(0x00000000000040);
    pub const H8: Bitboard =               Bitboard(0x00000000000080);
    pub const A7: Bitboard =               Bitboard(0x00000000000100);
    pub const B7: Bitboard =               Bitboard(0x00000000000200);
    pub const C7: Bitboard =               Bitboard(0x00000000000400);
    pub const D7: Bitboard =               Bitboard(0x00000000000800);
    pub const E7: Bitboard =               Bitboard(0x00000000001000);
    pub const F7: Bitboard =               Bitboard(0x00000000002000);
    pub const G7: Bitboard =               Bitboard(0x00000000004000);
    pub const H7: Bitboard =               Bitboard(0x00000000008000);
    pub const A6: Bitboard =               Bitboard(0x00000000010000);
    pub const B6: Bitboard =               Bitboard(0x00000000020000);
    pub const C6: Bitboard =               Bitboard(0x00000000040000);
    pub const D6: Bitboard =               Bitboard(0x00000000080000);
    pub const E6: Bitboard =               Bitboard(0x00000000100000);
    pub const F6: Bitboard =               Bitboard(0x00000000200000);
    pub const G6: Bitboard =               Bitboard(0x00000000400000);
    pub const H6: Bitboard =               Bitboard(0x00000000800000);
    pub const A5: Bitboard =               Bitboard(0x00000001000000);
    pub const B5: Bitboard =               Bitboard(0x00000002000000);
    pub const C5: Bitboard =               Bitboard(0x00000004000000);
    pub const D5: Bitboard =               Bitboard(0x00000008000000);
    pub const E5: Bitboard =               Bitboard(0x00000010000000);
    pub const F5: Bitboard =               Bitboard(0x00000020000000);
    pub const G5: Bitboard =               Bitboard(0x00000040000000);
    pub const H5: Bitboard =               Bitboard(0x00000080000000);
    pub const A4: Bitboard =               Bitboard(0x00000100000000);
    pub const B4: Bitboard =               Bitboard(0x00000200000000);
    pub const C4: Bitboard =               Bitboard(0x00000400000000);
    pub const D4: Bitboard =               Bitboard(0x00000800000000);
    pub const E4: Bitboard =               Bitboard(0x00001000000000);
    pub const F4: Bitboard =               Bitboard(0x00002000000000);
    pub const G4: Bitboard =               Bitboard(0x00004000000000);
    pub const H4: Bitboard =               Bitboard(0x00008000000000);
    pub const A3: Bitboard =               Bitboard(0x00010000000000);
    pub const B3: Bitboard =               Bitboard(0x00020000000000);
    pub const C3: Bitboard =               Bitboard(0x00040000000000);
    pub const D3: Bitboard =               Bitboard(0x00080000000000);
    pub const E3: Bitboard =               Bitboard(0x00100000000000);
    pub const F3: Bitboard =               Bitboard(0x00200000000000);
    pub const G3: Bitboard =               Bitboard(0x00400000000000);
    pub const H3: Bitboard =               Bitboard(0x00800000000000);
    pub const A2: Bitboard =               Bitboard(0x01000000000000);
    pub const B2: Bitboard =               Bitboard(0x02000000000000);
    pub const C2: Bitboard =               Bitboard(0x04000000000000);
    pub const D2: Bitboard =               Bitboard(0x08000000000000);
    pub const E2: Bitboard =               Bitboard(0x10000000000000);
    pub const F2: Bitboard =               Bitboard(0x20000000000000);
    pub const G2: Bitboard =               Bitboard(0x40000000000000);
    pub const H2: Bitboard =               Bitboard(0x80000000000000);
    pub const A1: Bitboard =               Bitboard(0x100000000000000);
    pub const B1: Bitboard =               Bitboard(0x200000000000000);
    pub const C1: Bitboard =               Bitboard(0x400000000000000);
    pub const D1: Bitboard =               Bitboard(0x800000000000000);
    pub const E1: Bitboard =               Bitboard(0x1000000000000000);
    pub const F1: Bitboard =               Bitboard(0x2000000000000000);
    pub const G1: Bitboard =               Bitboard(0x4000000000000000);
    pub const H1: Bitboard =               Bitboard(0x8000000000000000);
}


// TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::*;

    #[test]
    fn test_directional_circle() {
        let mut bb: Bitboard = Square::D4.bb();

        bb = bb.nort_one();
        assert_eq!(bb, Square::D5.bb());
        bb = bb.east_one();
        assert_eq!(bb, Square::E5.bb());
        bb = bb.sout_one();
        assert_eq!(bb, Square::E4.bb());
        bb = bb.west_one();
        assert_eq!(bb, Square::D4.bb());
    }

    #[test]
    fn test_directional_overflow() {
        assert_eq!(Square::B8.bb().nort_one(), Bitboard::EMPTY);
        assert_eq!(Square::H5.bb().east_one(), Bitboard::EMPTY);
        assert_eq!(Square::D1.bb().sout_one(), Bitboard::EMPTY);
        assert_eq!(Square::A2.bb().west_one(), Bitboard::EMPTY);
    }
}
