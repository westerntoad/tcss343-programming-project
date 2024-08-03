mod bitboard;
mod square;
mod move_gen;

use crate::square::*;
use crate::bitboard::*;
use crate::move_gen::lookup::*;
use crate::move_gen::pawn::*;
use crate::move_gen::sliding::*;

fn print_squares_bb() {
    for i in 0..64 {
        let sq = Square::new(i).unwrap();
        let name = format!("{}", format!("{}", sq).to_ascii_uppercase());
        let code = format!("{:#016x}", sq.bb().val());
        println!("pub const {}: Bitboard =               Bitboard({});", name, code);
    }
}

fn print_n_gen() {
    for i in 0..64 {
        let sq = Square::new(i).unwrap();
        let name = format!("{}", format!("{}", sq).to_ascii_uppercase());
        let code = format!("{:#016x}", n_gen_shift(sq.bb()).val());
        println!("        Bitboard::{name} => Bitboard::new({code}),");
    }
}

fn print_k_gen() {
    for i in 0..64 {
        let sq = Square::new(i).unwrap();
        let name = format!("{}", format!("{}", sq).to_ascii_uppercase());
        let code = format!("{:#016x}", k_gen_shift(sq.bb()).val());
        println!("        Bitboard::{name} => Bitboard::new({code}),");
    }
}

fn main() {
    //println!("{:?}\n\n\n{:?}", Square::E4.bb(), k_gen_lookup(Square::E4.bb()));
        let origin     = Square::A1.bb();
        let friend     = Square::F1.bb() | Square::C3.bb();
        let enemy      = Square::A5.bb() | Square::A6.bb() | Square::H1.bb() | Square::C6.bb();
    let output = r_gen_shift(origin, friend, enemy);


    println!("original:{:?}\nmoved:{:?}", origin, output);
}
