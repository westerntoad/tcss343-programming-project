use crate::bitboard::Bitboard;
use crate::square::Square;
use crate::magic::{
    ROOK_MAGICS,
    ROOK_MAGICS_CONST,
    ROOK_MOVES
};

pub const MASK: [Bitboard; Square::NUM] = [
    Bitboard::new(0x0101010101017e), Bitboard::new(0x0202020202027c),
    Bitboard::new(0x0404040404047a), Bitboard::new(0x08080808080876),
    Bitboard::new(0x1010101010106e), Bitboard::new(0x2020202020205e),
    Bitboard::new(0x4040404040403e), Bitboard::new(0x8080808080807e),
    Bitboard::new(0x01010101017e00), Bitboard::new(0x02020202027c00),
    Bitboard::new(0x04040404047a00), Bitboard::new(0x08080808087600),
    Bitboard::new(0x10101010106e00), Bitboard::new(0x20202020205e00),
    Bitboard::new(0x40404040403e00), Bitboard::new(0x80808080807e00),
    Bitboard::new(0x010101017e0100), Bitboard::new(0x020202027c0200),
    Bitboard::new(0x040404047a0400), Bitboard::new(0x08080808760800),
    Bitboard::new(0x101010106e1000), Bitboard::new(0x202020205e2000),
    Bitboard::new(0x404040403e4000), Bitboard::new(0x808080807e8000),
    Bitboard::new(0x0101017e010100), Bitboard::new(0x0202027c020200),
    Bitboard::new(0x0404047a040400), Bitboard::new(0x08080876080800),
    Bitboard::new(0x1010106e101000), Bitboard::new(0x2020205e202000),
    Bitboard::new(0x4040403e404000), Bitboard::new(0x8080807e808000),
    Bitboard::new(0x01017e01010100), Bitboard::new(0x02027c02020200),
    Bitboard::new(0x04047a04040400), Bitboard::new(0x08087608080800),
    Bitboard::new(0x10106e10101000), Bitboard::new(0x20205e20202000),
    Bitboard::new(0x40403e40404000), Bitboard::new(0x80807e80808000),
    Bitboard::new(0x017e0101010100), Bitboard::new(0x027c0202020200),
    Bitboard::new(0x047a0404040400), Bitboard::new(0x08760808080800),
    Bitboard::new(0x106e1010101000), Bitboard::new(0x205e2020202000),
    Bitboard::new(0x403e4040404000), Bitboard::new(0x807e8080808000),
    Bitboard::new(0x7e010101010100), Bitboard::new(0x7c020202020200),
    Bitboard::new(0x7a040404040400), Bitboard::new(0x76080808080800),
    Bitboard::new(0x6e101010101000), Bitboard::new(0x5e202020202000),
    Bitboard::new(0x3e404040404000), Bitboard::new(0x7e808080808000),
    Bitboard::new(0x7e01010101010100), Bitboard::new(0x7c02020202020200),
    Bitboard::new(0x7a04040404040400), Bitboard::new(0x7608080808080800),
    Bitboard::new(0x6e10101010101000), Bitboard::new(0x5e20202020202000),
    Bitboard::new(0x3e40404040404000), Bitboard::new(0x7e80808080808000)
];

#[inline(always)]
pub fn r_gen_shift_1(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let mut nort_idx = orig.nort_one();
    while !nort_idx.is_empty() && (nort_idx & friend).is_empty() {
        attacks |= nort_idx;
        if !(nort_idx & enemy).is_empty() {
            break;
        }
        nort_idx = nort_idx.nort_one();
    }
    
    let mut east_idx = orig.east_one();
    while !east_idx.is_empty() && (east_idx & friend).is_empty() {
        attacks |= east_idx;
        if !(east_idx & enemy).is_empty() {
            break;
        }
        east_idx = east_idx.east_one();
    }

    let mut sout_idx = orig.sout_one();
    while !sout_idx.is_empty() && (sout_idx & friend).is_empty() {
        attacks |= sout_idx;
        if !(sout_idx & enemy).is_empty() {
            break;
        }
        sout_idx = sout_idx.sout_one();
    }

    let mut west_idx = orig.west_one();
    while !west_idx.is_empty() && (west_idx & friend).is_empty() {
        attacks |= west_idx;
        if !(west_idx & enemy).is_empty() {
            break;
        }
        west_idx = west_idx.west_one();
    }

    attacks
}

#[inline(always)]
pub fn r_gen_shift_2_helper(orig: Bitboard, blockers: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let mut nort_idx = orig.nort_one();
    while !nort_idx.is_empty() {
        attacks |= nort_idx;
        if !(nort_idx & blockers).is_empty() {
            break;
        }
        nort_idx = nort_idx.nort_one();
    }
    
    let mut east_idx = orig;
    while !east_idx.is_empty() {
        east_idx = east_idx.east_one();
        attacks |= east_idx;
        if !(east_idx & blockers).is_empty() {
            break;
        }
    }

    let mut sout_idx = orig.sout_one();
    while !sout_idx.is_empty() {
        attacks |= sout_idx;
        if !(sout_idx & blockers).is_empty() {
            break;
        }
        sout_idx = sout_idx.sout_one();
    }

    let mut west_idx = orig.west_one();
    while !west_idx.is_empty() {
        attacks |= west_idx;
        if !(west_idx & blockers).is_empty() {
            break;
        }
        west_idx = west_idx.west_one();
    }

    attacks
}

#[inline(always)]
pub fn r_gen_shift_2(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    r_gen_shift_2_helper(orig, friend | enemy) & !friend
}

#[inline(always)]
pub fn r_gen_magic(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    unsafe {
        let blockers = friend | enemy;
        let magic = &ROOK_MAGICS_CONST[orig.index()];
        let moves = ROOK_MOVES[orig.index()];

        moves[magic.get_index(blockers)] & !friend
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::Square;
    use crate::magic::init_rook_moves;
    
    
    #[test]
    fn test_r_gen_1() {
        init_rook_moves();

        let origin     = Square::C3.bb();
        let friend     = Square::C7.bb();
        let enemy      = Bitboard::EMPTY;

        let output_shift_1 = r_gen_shift_1(origin, friend, enemy);
        let output_shift_2 = r_gen_shift_2(origin, friend, enemy);
        let output_magic = r_gen_magic(origin, friend, enemy);
        let expected_output = Bitboard::new(0x404fb0404040000);

        assert_eq!(output_shift_1, expected_output);
        assert_eq!(output_shift_2, expected_output);
        assert_eq!(output_magic, expected_output);
    }
    
    #[test]
    fn test_r_gen_2() {
        init_rook_moves();

        let origin     = Square::A1.bb();
        let friend     = Square::F1.bb() | Square::C3.bb();
        let enemy      = Square::A5.bb() | Square::A6.bb() | Square::H1.bb() | Square::C6.bb();

        let output_shift_1 = r_gen_shift_1(origin, friend, enemy);
        let output_shift_2 = r_gen_shift_2(origin, friend, enemy);
        let output_magic = r_gen_magic(origin, friend, enemy);
        let expected_output = Bitboard::new(0x1e01010101000000);

        assert_eq!(output_shift_1, expected_output);
        assert_eq!(output_shift_2, expected_output);
        assert_eq!(output_magic, expected_output);
    }
}
