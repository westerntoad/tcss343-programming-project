use crate::bitboard::Bitboard;

pub fn b_gen_shift(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::Square;
    
    
    #[test]
    fn test_b_gen_1() {
        let origin     = Square::E4.bb();
        let friend     = Bitboard::new(0);
        let enemy      = Bitboard::new(0);

        let output = b_gen_shift(origin, friend, enemy);
        let expected_output = Bitboard::new(0x8244280028448201);

        assert_eq!(output, expected_output);
    }
    
    #[test]
    fn test_b_gen_2() {
        let origin     = Square::B3.bb();
        let friend     = Square::D5.bb();
        let enemy      = Bitboard::new(0);

        let output = b_gen_shift(origin, friend, enemy);
        let expected_output = Bitboard::new(0x805000500000000);

        assert_eq!(output, expected_output);
    }
    
    #[test]
    fn test_b_gen_3() {
        let origin     = Square::F5.bb();
        let friend     = Square::E4.bb() | Square::H7.bb();
        let enemy      = Square::D3.bb() | Square::C8.bb() | Square::B5.bb();

        let output = b_gen_shift(origin, friend, enemy);
        let expected_output = Bitboard::new(0x00804000500804);

        assert_eq!(output, expected_output);
    }
}
