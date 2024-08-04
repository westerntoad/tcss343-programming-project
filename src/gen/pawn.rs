use crate::bitboard::Bitboard;

#[inline(always)]
pub fn p_gen_shift(
    orig: Bitboard,
    is_white: bool,
    friend: Bitboard,
    enemy: Bitboard,
    en_passant: Bitboard
) -> Bitboard {
    let forward = match is_white {
        true  => orig.nort_one(),
        false => orig.sout_one()
    };

    let blocks = friend | enemy;
    let attacks = (forward.east_one() | forward.west_one()) & (enemy | en_passant);
    let movement = if is_white && orig.rank() == 6 {
        let step = forward & !blocks;
        step | step.nort_one() & !blocks
    } else if !is_white && orig.rank() == 1 {
        let step = forward & !blocks;
        step | step.sout_one() & !blocks
    } else {
        forward & !blocks
    };

    attacks | movement
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::Square;

    #[test]
    fn test_p_gen_start_white() {
        let origin     = Square::E2.bb();
        let is_white   = true;
        let friend     = Bitboard::new(0);
        let enemy      = Bitboard::new(0);
        let en_passant = Bitboard::new(0);

        let output = p_gen_shift(origin, is_white, friend, enemy, en_passant);
        let expected_output = Bitboard::new(0x00101000000000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_p_gen_start_black() {
        let origin     = Square::H7.bb();
        let is_white   = false;
        let friend     = Bitboard::new(0);
        let enemy      = Bitboard::new(0);
        let en_passant = Bitboard::new(0);

        let output = p_gen_shift(origin, is_white, friend, enemy, en_passant);
        let expected_output = Bitboard::new(0x00000080800000);

        assert_eq!(output, expected_output);
    }


    #[test]
    fn test_p_gen_middle() {
        let origin     = Square::C3.bb();
        let is_white   = true;
        let friend     = Bitboard::new(0);
        let enemy      = Bitboard::new(0);
        let en_passant = Bitboard::new(0);

        let output = p_gen_shift(origin, is_white, friend, enemy, en_passant);
        let expected_output = Bitboard::new(0x00000400000000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_p_gen_end() {
        let origin     = Square::C2.bb();
        let is_white   = false;
        let friend     = Bitboard::new(0);
        let enemy      = Bitboard::new(0);
        let en_passant = Bitboard::new(0);

        let output = p_gen_shift(origin, is_white, friend, enemy, en_passant);
        let expected_output = Bitboard::new(0x400000000000000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_p_gen_block() {
        let origin     = Square::D6.bb();
        let is_white   = false;
        let friend     = Square::D5.bb();
        let enemy      = Bitboard::new(0);
        let en_passant = Bitboard::new(0);

        let output = p_gen_shift(origin, is_white, friend, enemy, en_passant);
        let expected_output = Bitboard::EMPTY;

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_p_gen_start_block() {
        let origin     = Square::A2.bb();
        let is_white   = true;
        let friend     = Bitboard::new(0);
        let enemy      = Square::A3.bb();
        let en_passant = Bitboard::new(0);

        let output = p_gen_shift(origin, is_white, friend, enemy, en_passant);
        let expected_output = Bitboard::EMPTY;

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_p_gen_enemy() {
        let origin     = Square::E4.bb();
        let is_white   = false;
        let friend     = Bitboard::new(0);
        let enemy      = Square::D5.bb() | Square::D3.bb();
        let en_passant = Bitboard::new(0);

        let output = p_gen_shift(origin, is_white, friend, enemy, en_passant);
        let expected_output = Square::D3.bb() | Square::E3.bb();

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_p_gen_block_and_enemy() {
        let origin     = Square::C4.bb();
        let is_white   = true;
        let friend     = Square::C5.bb();
        let enemy      = Square::B5.bb();
        let en_passant = Bitboard::new(0);

        let output = p_gen_shift(origin, is_white, friend, enemy, en_passant);
        let expected_output = Square::B5.bb();

        assert_eq!(output, expected_output);
    }
}
