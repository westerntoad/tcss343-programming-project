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

pub fn r_gen_shift(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
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
        if !(nort_idx & enemy).is_empty() {
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

pub fn q_gen_shift(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    b_gen_shift(orig, enemy, friend) | r_gen_shift(orig, enemy, friend)
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
    
    #[test]
    fn test_r_gen_1() {
        let origin     = Square::C3.bb();
        let friend     = Square::C7.bb();
        let enemy      = Bitboard::EMPTY;

        let output = r_gen_shift(origin, friend, enemy);
        let expected_output = Bitboard::new(0x404fb0404040000);

        assert_eq!(output, expected_output);
    }
    
    #[test]
    fn test_r_gen_2() {
        let origin     = Square::A1.bb();
        let friend     = Square::F1.bb() | Square::C3.bb();
        let enemy      = Square::A5.bb() | Square::A6.bb() | Square::H1.bb() | Square::C6.bb();

        let output = r_gen_shift(origin, friend, enemy);
        let expected_output = Bitboard::new(0x201010101000000);

        assert_eq!(output, expected_output);
    }
}
