use crate::bitboard::Bitboard;
use crate::square::Square;
use crate::magic::{
    BISHOP_MAGICS,
    BISHOP_MAGICS_CONST,
    BISHOP_MOVES
};

pub const MASK: [Bitboard; Square::NUM] = [
    Bitboard::new(0x40201008040200), Bitboard::new(0x00402010080400),
    Bitboard::new(0x00004020100a00), Bitboard::new(0x00000040221400),
    Bitboard::new(0x00000002442800), Bitboard::new(0x00000204085000),
    Bitboard::new(0x00020408102000), Bitboard::new(0x02040810204000),
    Bitboard::new(0x20100804020000), Bitboard::new(0x40201008040000),
    Bitboard::new(0x004020100a0000), Bitboard::new(0x00004022140000),
    Bitboard::new(0x00000244280000), Bitboard::new(0x00020408500000),
    Bitboard::new(0x02040810200000), Bitboard::new(0x04081020400000),
    Bitboard::new(0x10080402000200), Bitboard::new(0x20100804000400),
    Bitboard::new(0x4020100a000a00), Bitboard::new(0x00402214001400),
    Bitboard::new(0x00024428002800), Bitboard::new(0x02040850005000),
    Bitboard::new(0x04081020002000), Bitboard::new(0x08102040004000),
    Bitboard::new(0x08040200020400), Bitboard::new(0x10080400040800),
    Bitboard::new(0x20100a000a1000), Bitboard::new(0x40221400142200),
    Bitboard::new(0x02442800284400), Bitboard::new(0x04085000500800),
    Bitboard::new(0x08102000201000), Bitboard::new(0x10204000402000),
    Bitboard::new(0x04020002040800), Bitboard::new(0x08040004081000),
    Bitboard::new(0x100a000a102000), Bitboard::new(0x22140014224000),
    Bitboard::new(0x44280028440200), Bitboard::new(0x08500050080400),
    Bitboard::new(0x10200020100800), Bitboard::new(0x20400040201000),
    Bitboard::new(0x02000204081000), Bitboard::new(0x04000408102000),
    Bitboard::new(0x0a000a10204000), Bitboard::new(0x14001422400000),
    Bitboard::new(0x28002844020000), Bitboard::new(0x50005008040200),
    Bitboard::new(0x20002010080400), Bitboard::new(0x40004020100800),
    Bitboard::new(0x00020408102000), Bitboard::new(0x00040810204000),
    Bitboard::new(0x000a1020400000), Bitboard::new(0x00142240000000),
    Bitboard::new(0x00284402000000), Bitboard::new(0x00500804020000),
    Bitboard::new(0x00201008040200), Bitboard::new(0x00402010080400),
    Bitboard::new(0x02040810204000), Bitboard::new(0x04081020400000),
    Bitboard::new(0x0a102040000000), Bitboard::new(0x14224000000000),
    Bitboard::new(0x28440200000000), Bitboard::new(0x50080402000000),
    Bitboard::new(0x20100804020000), Bitboard::new(0x40201008040200)
];

#[inline(always)]
pub fn b_gen_shift_1(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let mut no_ea_idx = orig.nort_one().east_one();
    while !no_ea_idx.is_empty() && (no_ea_idx & friend).is_empty() {
        attacks |= no_ea_idx;
        if !(no_ea_idx & enemy).is_empty() {
            break;
        }
        no_ea_idx = no_ea_idx.nort_one().east_one();
    }

    let mut so_ea_idx = orig.sout_one().east_one();
    while !so_ea_idx.is_empty() && (so_ea_idx & friend).is_empty() {
        attacks |= so_ea_idx;
        if !(so_ea_idx & enemy).is_empty() {
            break;
        }
        so_ea_idx = so_ea_idx.sout_one().east_one();
    }

    let mut so_we_idx = orig.sout_one().west_one();
    while !so_we_idx.is_empty() && (so_we_idx & friend).is_empty() {
        attacks |= so_we_idx;
        if !(so_we_idx & enemy).is_empty() {
            break;
        }
        so_we_idx = so_we_idx.sout_one().west_one();
    }

    let mut no_we_idx = orig.nort_one().west_one();
    while !no_we_idx.is_empty() && (no_we_idx & friend).is_empty() {
        attacks |= no_we_idx;
        if !(no_we_idx & enemy).is_empty() {
            break;
        }
        no_we_idx = no_we_idx.nort_one().west_one();
    }

    attacks
}

#[inline(always)]
pub fn b_gen_shift_2_helper(orig: Bitboard, blockers: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let mut idx = orig.nort_one().east_one();
    while !idx.is_empty() {
        attacks |= idx;
        if !(idx & blockers).is_empty() {
            break;
        }
        idx = idx.nort_one().east_one();
    }

    idx = orig.sout_one().east_one();
    while !idx.is_empty() {
        attacks |= idx;
        if !(idx & blockers).is_empty() {
            break;
        }
        idx = idx.sout_one().east_one();
    }

    idx = orig.sout_one().west_one();
    while !idx.is_empty() {
        attacks |= idx;
        if !(idx & blockers).is_empty() {
            break;
        }
        idx = idx.sout_one().west_one();
    }

    idx = orig.nort_one().west_one();
    while !idx.is_empty() {
        attacks |= idx;
        if !(idx & blockers).is_empty() {
            break;
        }
        idx = idx.nort_one().west_one();
    }

    attacks
}

#[inline(always)]
pub fn b_gen_shift_2(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    b_gen_shift_2_helper(orig, friend | enemy) & !friend
}

#[inline(always)]
pub fn b_gen_magic(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    unsafe {
        let blockers = friend | enemy;
        let magic = &BISHOP_MAGICS_CONST[orig.index()];
        let moves = BISHOP_MOVES[orig.index()];

        moves[magic.get_index(blockers)] & !friend
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::Square;
    use crate::magic::init_bishop_moves;
    
    #[test]
    fn test_b_gen_1() {
        init_bishop_moves();

        let origin     = Square::E4.bb();
        let friend     = Bitboard::new(0);
        let enemy      = Bitboard::new(0);

        let output_shift_1 = b_gen_shift_1(origin, friend, enemy);
        let output_shift_2 = b_gen_shift_2(origin, friend, enemy);
        let output_magic = b_gen_magic(origin, friend, enemy);
        let expected_output = Bitboard::new(0x8244280028448201);

        assert_eq!(output_shift_1, expected_output);
        assert_eq!(output_shift_2, expected_output);
        assert_eq!(output_magic, expected_output);
    }
    
    #[test]
    fn test_b_gen_2() {
        init_bishop_moves();
        
        let origin     = Square::B3.bb();
        let friend     = Square::D5.bb();
        let enemy      = Bitboard::new(0);

        let output_shift_1 = b_gen_shift_1(origin, friend, enemy);
        let output_shift_2 = b_gen_shift_2(origin, friend, enemy);
        let output_magic = b_gen_magic(origin, friend, enemy);
        let expected_output = Bitboard::new(0x805000500000000);

        assert_eq!(output_shift_1, expected_output);
        assert_eq!(output_shift_2, expected_output);
        assert_eq!(output_magic, expected_output);
    }
    
    #[test]
    fn test_b_gen_3() {
        init_bishop_moves();

        let origin     = Square::F5.bb();
        let friend     = Square::E4.bb() | Square::H7.bb();
        let enemy      = Square::D3.bb() | Square::C8.bb() | Square::B5.bb();

        let output_shift_1 = b_gen_shift_1(origin, friend, enemy);
        let output_shift_2 = b_gen_shift_2(origin, friend, enemy);
        let output_magic = b_gen_magic(origin, friend, enemy);
        let expected_output = Bitboard::new(0x00804000500804);

        assert_eq!(output_shift_1, expected_output);
        assert_eq!(output_shift_2, expected_output);
        assert_eq!(output_magic, expected_output);
    }
}
