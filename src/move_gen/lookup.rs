use crate::bitboard::Bitboard;

pub fn n_gen_shift(orig: Bitboard) -> Bitboard {
    let mut horizontal = orig.east_one().east_one();
    horizontal |= orig.west_one().west_one();
    horizontal = horizontal.nort_one() | horizontal.sout_one();

    let mut vertical = orig.nort_one().nort_one();
    vertical |= orig.sout_one().sout_one();
    vertical = vertical.east_one() | vertical.west_one();

    horizontal | vertical
}

pub const fn n_gen_lookup(orig: Bitboard) -> Bitboard {
    match orig {
        Bitboard::A8 => Bitboard::new(0x00000000020400),
        Bitboard::B8 => Bitboard::new(0x00000000050800),
        Bitboard::C8 => Bitboard::new(0x000000000a1100),
        Bitboard::D8 => Bitboard::new(0x00000000142200),
        Bitboard::E8 => Bitboard::new(0x00000000284400),
        Bitboard::F8 => Bitboard::new(0x00000000508800),
        Bitboard::G8 => Bitboard::new(0x00000000a01000),
        Bitboard::H8 => Bitboard::new(0x00000000402000),
        Bitboard::A7 => Bitboard::new(0x00000002040004),
        Bitboard::B7 => Bitboard::new(0x00000005080008),
        Bitboard::C7 => Bitboard::new(0x0000000a110011),
        Bitboard::D7 => Bitboard::new(0x00000014220022),
        Bitboard::E7 => Bitboard::new(0x00000028440044),
        Bitboard::F7 => Bitboard::new(0x00000050880088),
        Bitboard::G7 => Bitboard::new(0x000000a0100010),
        Bitboard::H7 => Bitboard::new(0x00000040200020),
        Bitboard::A6 => Bitboard::new(0x00000204000402),
        Bitboard::B6 => Bitboard::new(0x00000508000805),
        Bitboard::C6 => Bitboard::new(0x00000a1100110a),
        Bitboard::D6 => Bitboard::new(0x00001422002214),
        Bitboard::E6 => Bitboard::new(0x00002844004428),
        Bitboard::F6 => Bitboard::new(0x00005088008850),
        Bitboard::G6 => Bitboard::new(0x0000a0100010a0),
        Bitboard::H6 => Bitboard::new(0x00004020002040),
        Bitboard::A5 => Bitboard::new(0x00020400040200),
        Bitboard::B5 => Bitboard::new(0x00050800080500),
        Bitboard::C5 => Bitboard::new(0x000a1100110a00),
        Bitboard::D5 => Bitboard::new(0x00142200221400),
        Bitboard::E5 => Bitboard::new(0x00284400442800),
        Bitboard::F5 => Bitboard::new(0x00508800885000),
        Bitboard::G5 => Bitboard::new(0x00a0100010a000),
        Bitboard::H5 => Bitboard::new(0x00402000204000),
        Bitboard::A4 => Bitboard::new(0x02040004020000),
        Bitboard::B4 => Bitboard::new(0x05080008050000),
        Bitboard::C4 => Bitboard::new(0x0a1100110a0000),
        Bitboard::D4 => Bitboard::new(0x14220022140000),
        Bitboard::E4 => Bitboard::new(0x28440044280000),
        Bitboard::F4 => Bitboard::new(0x50880088500000),
        Bitboard::G4 => Bitboard::new(0xa0100010a00000),
        Bitboard::H4 => Bitboard::new(0x40200020400000),
        Bitboard::A3 => Bitboard::new(0x204000402000000),
        Bitboard::B3 => Bitboard::new(0x508000805000000),
        Bitboard::C3 => Bitboard::new(0xa1100110a000000),
        Bitboard::D3 => Bitboard::new(0x1422002214000000),
        Bitboard::E3 => Bitboard::new(0x2844004428000000),
        Bitboard::F3 => Bitboard::new(0x5088008850000000),
        Bitboard::G3 => Bitboard::new(0xa0100010a0000000),
        Bitboard::H3 => Bitboard::new(0x4020002040000000),
        Bitboard::A2 => Bitboard::new(0x400040200000000),
        Bitboard::B2 => Bitboard::new(0x800080500000000),
        Bitboard::C2 => Bitboard::new(0x1100110a00000000),
        Bitboard::D2 => Bitboard::new(0x2200221400000000),
        Bitboard::E2 => Bitboard::new(0x4400442800000000),
        Bitboard::F2 => Bitboard::new(0x8800885000000000),
        Bitboard::G2 => Bitboard::new(0x100010a000000000),
        Bitboard::H2 => Bitboard::new(0x2000204000000000),
        Bitboard::A1 => Bitboard::new(0x04020000000000),
        Bitboard::B1 => Bitboard::new(0x08050000000000),
        Bitboard::C1 => Bitboard::new(0x110a0000000000),
        Bitboard::D1 => Bitboard::new(0x22140000000000),
        Bitboard::E1 => Bitboard::new(0x44280000000000),
        Bitboard::F1 => Bitboard::new(0x88500000000000),
        Bitboard::G1 => Bitboard::new(0x10a00000000000),
        Bitboard::H1 => Bitboard::new(0x20400000000000),
        _ => panic!()
    }
}

/*
pub fn k_move_gen(orig: Bitboard, friend: Bitboard) -> Bitboard {
    let attacks = orig.nort_one() | orig.sout_one();

    (attacks | attacks.west_one() | attacks.east_one() | orig.west_one() | orig.east_one()) & !friend
}
*/


#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::Square;

    #[test]
    fn test_n_gen_1() {
        let output = n_gen_shift(Square::E4.bb());
        let expected_output = Bitboard::new(0x28440044280000);


        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_gen_edge_close() {
        let output = n_gen_shift(Square::A4.bb());
        let expected_output = Bitboard::new(0x02040004020000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_gen_edge_far() {
        let output = n_gen_shift(Square::G5.bb());
        let expected_output = Bitboard::new(0x00a0100010a000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_gen_corner() {
        let output = n_gen_shift(Square::A1.bb());
        let expected_output = Bitboard::new(0x04020000000000);

        assert_eq!(output, expected_output);
    }
    
    /*
    #[test]
    fn test_k_move_gen() {
        let output = k_move_gen(Square::E2.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x3828380000000000);


        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_move_gen_edge() {
        let output = k_move_gen(Square::A4.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x00030203000000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_move_gen_corner() {
        let output = k_move_gen(Square::H8.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x0000000000c040);

        assert_eq!(output, expected_output);
    }
    */
}
