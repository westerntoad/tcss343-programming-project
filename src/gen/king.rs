use crate::bitboard::Bitboard;
use crate::square::Square;

const KING_ATTACKS: [Bitboard; Square::NUM] = [
    Bitboard::new(0x00000000000302), Bitboard::new(0x00000000000705),
    Bitboard::new(0x00000000000e0a), Bitboard::new(0x00000000001c14),
    Bitboard::new(0x00000000003828), Bitboard::new(0x00000000007050),
    Bitboard::new(0x0000000000e0a0), Bitboard::new(0x0000000000c040),
    Bitboard::new(0x00000000030203), Bitboard::new(0x00000000070507),
    Bitboard::new(0x000000000e0a0e), Bitboard::new(0x000000001c141c),
    Bitboard::new(0x00000000382838), Bitboard::new(0x00000000705070),
    Bitboard::new(0x00000000e0a0e0), Bitboard::new(0x00000000c040c0),
    Bitboard::new(0x00000003020300), Bitboard::new(0x00000007050700),
    Bitboard::new(0x0000000e0a0e00), Bitboard::new(0x0000001c141c00),
    Bitboard::new(0x00000038283800), Bitboard::new(0x00000070507000),
    Bitboard::new(0x000000e0a0e000), Bitboard::new(0x000000c040c000),
    Bitboard::new(0x00000302030000), Bitboard::new(0x00000705070000),
    Bitboard::new(0x00000e0a0e0000), Bitboard::new(0x00001c141c0000),
    Bitboard::new(0x00003828380000), Bitboard::new(0x00007050700000),
    Bitboard::new(0x0000e0a0e00000), Bitboard::new(0x0000c040c00000),
    Bitboard::new(0x00030203000000), Bitboard::new(0x00070507000000),
    Bitboard::new(0x000e0a0e000000), Bitboard::new(0x001c141c000000),
    Bitboard::new(0x00382838000000), Bitboard::new(0x00705070000000),
    Bitboard::new(0x00e0a0e0000000), Bitboard::new(0x00c040c0000000),
    Bitboard::new(0x03020300000000), Bitboard::new(0x07050700000000),
    Bitboard::new(0x0e0a0e00000000), Bitboard::new(0x1c141c00000000),
    Bitboard::new(0x38283800000000), Bitboard::new(0x70507000000000),
    Bitboard::new(0xe0a0e000000000), Bitboard::new(0xc040c000000000),
    Bitboard::new(0x302030000000000), Bitboard::new(0x705070000000000),
    Bitboard::new(0xe0a0e0000000000), Bitboard::new(0x1c141c0000000000),
    Bitboard::new(0x3828380000000000), Bitboard::new(0x7050700000000000),
    Bitboard::new(0xe0a0e00000000000), Bitboard::new(0xc040c00000000000),
    Bitboard::new(0x203000000000000), Bitboard::new(0x507000000000000),
    Bitboard::new(0xa0e000000000000), Bitboard::new(0x141c000000000000),
    Bitboard::new(0x2838000000000000), Bitboard::new(0x5070000000000000),
    Bitboard::new(0xa0e0000000000000), Bitboard::new(0x40c0000000000000),
];

#[inline(always)]
pub fn k_gen_shift(orig: Bitboard) -> Bitboard {
    let attacks = orig.nort_one() | orig.sout_one();

    attacks | attacks.west_one() | attacks.east_one() | orig.west_one() | orig.east_one()
}

#[inline(always)]
pub const fn k_gen_match(orig: Bitboard) -> Bitboard {
    match orig {
        Bitboard::A8 => Bitboard::new(0x00000000000302), Bitboard::B8 => Bitboard::new(0x00000000000705),
        Bitboard::C8 => Bitboard::new(0x00000000000e0a), Bitboard::D8 => Bitboard::new(0x00000000001c14),
        Bitboard::E8 => Bitboard::new(0x00000000003828), Bitboard::F8 => Bitboard::new(0x00000000007050),
        Bitboard::G8 => Bitboard::new(0x0000000000e0a0), Bitboard::H8 => Bitboard::new(0x0000000000c040),
        Bitboard::A7 => Bitboard::new(0x00000000030203), Bitboard::B7 => Bitboard::new(0x00000000070507),
        Bitboard::C7 => Bitboard::new(0x000000000e0a0e), Bitboard::D7 => Bitboard::new(0x000000001c141c),
        Bitboard::E7 => Bitboard::new(0x00000000382838), Bitboard::F7 => Bitboard::new(0x00000000705070),
        Bitboard::G7 => Bitboard::new(0x00000000e0a0e0), Bitboard::H7 => Bitboard::new(0x00000000c040c0),
        Bitboard::A6 => Bitboard::new(0x00000003020300), Bitboard::B6 => Bitboard::new(0x00000007050700),
        Bitboard::C6 => Bitboard::new(0x0000000e0a0e00), Bitboard::D6 => Bitboard::new(0x0000001c141c00),
        Bitboard::E6 => Bitboard::new(0x00000038283800), Bitboard::F6 => Bitboard::new(0x00000070507000),
        Bitboard::G6 => Bitboard::new(0x000000e0a0e000), Bitboard::H6 => Bitboard::new(0x000000c040c000),
        Bitboard::A5 => Bitboard::new(0x00000302030000), Bitboard::B5 => Bitboard::new(0x00000705070000),
        Bitboard::C5 => Bitboard::new(0x00000e0a0e0000), Bitboard::D5 => Bitboard::new(0x00001c141c0000),
        Bitboard::E5 => Bitboard::new(0x00003828380000), Bitboard::F5 => Bitboard::new(0x00007050700000),
        Bitboard::G5 => Bitboard::new(0x0000e0a0e00000), Bitboard::H5 => Bitboard::new(0x0000c040c00000),
        Bitboard::A4 => Bitboard::new(0x00030203000000), Bitboard::B4 => Bitboard::new(0x00070507000000),
        Bitboard::C4 => Bitboard::new(0x000e0a0e000000), Bitboard::D4 => Bitboard::new(0x001c141c000000),
        Bitboard::E4 => Bitboard::new(0x00382838000000), Bitboard::F4 => Bitboard::new(0x00705070000000),
        Bitboard::G4 => Bitboard::new(0x00e0a0e0000000), Bitboard::H4 => Bitboard::new(0x00c040c0000000),
        Bitboard::A3 => Bitboard::new(0x03020300000000), Bitboard::B3 => Bitboard::new(0x07050700000000),
        Bitboard::C3 => Bitboard::new(0x0e0a0e00000000), Bitboard::D3 => Bitboard::new(0x1c141c00000000),
        Bitboard::E3 => Bitboard::new(0x38283800000000), Bitboard::F3 => Bitboard::new(0x70507000000000),
        Bitboard::G3 => Bitboard::new(0xe0a0e000000000), Bitboard::H3 => Bitboard::new(0xc040c000000000),
        Bitboard::A2 => Bitboard::new(0x302030000000000), Bitboard::B2 => Bitboard::new(0x705070000000000),
        Bitboard::C2 => Bitboard::new(0xe0a0e0000000000), Bitboard::D2 => Bitboard::new(0x1c141c0000000000),
        Bitboard::E2 => Bitboard::new(0x3828380000000000), Bitboard::F2 => Bitboard::new(0x7050700000000000),
        Bitboard::G2 => Bitboard::new(0xe0a0e00000000000), Bitboard::H2 => Bitboard::new(0xc040c00000000000),
        Bitboard::A1 => Bitboard::new(0x203000000000000), Bitboard::B1 => Bitboard::new(0x507000000000000),
        Bitboard::C1 => Bitboard::new(0xa0e000000000000), Bitboard::D1 => Bitboard::new(0x141c000000000000),
        Bitboard::E1 => Bitboard::new(0x2838000000000000), Bitboard::F1 => Bitboard::new(0x5070000000000000),
        Bitboard::G1 => Bitboard::new(0xa0e0000000000000), Bitboard::H1 => Bitboard::new(0x40c0000000000000),
        _ => panic!()
    }
}

#[inline(always)]
pub const fn k_gen_lookup(orig: Bitboard) -> Bitboard {
    KING_ATTACKS[orig.index()]
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::Square;

    #[test]
    fn test_k_move_gen() {
        let output_shift = k_gen_shift(Square::E2.bb());
        let output_match = k_gen_match(Square::E2.bb());
        let expected_output = Bitboard::new(0x3828380000000000);


        assert_eq!(output_shift, expected_output);
        assert_eq!(output_match, expected_output);
    }

    #[test]
    fn test_k_move_gen_edge() {
        let output_shift = k_gen_shift(Square::A4.bb());
        let output_match = k_gen_match(Square::A4.bb());
        let expected_output = Bitboard::new(0x00030203000000);

        assert_eq!(output_shift, expected_output);
        assert_eq!(output_match, expected_output);
    }

    #[test]
    fn test_k_move_gen_corner() {
        let output_shift = k_gen_shift(Square::H8.bb());
        let output_match = k_gen_match(Square::H8.bb());
        let expected_output = Bitboard::new(0x0000000000c040);

        assert_eq!(output_shift, expected_output);
        assert_eq!(output_match, expected_output);
    }
}
