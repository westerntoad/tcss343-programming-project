use crate::bitboard::Bitboard;
use crate::gen::{
    bishop::*,
    rook::*
};

pub fn q_gen_shift_1(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    b_gen_shift_1(orig, enemy, friend) | r_gen_shift_2(orig, enemy, friend)
}

pub fn q_gen_shift_2(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    b_gen_shift_2(orig, enemy, friend) | r_gen_shift_2(orig, enemy, friend)
}

pub fn q_gen_magic(orig: Bitboard, friend: Bitboard, enemy: Bitboard) -> Bitboard {
    b_gen_magic(orig, enemy, friend) | r_gen_magic(orig, enemy, friend)
}
