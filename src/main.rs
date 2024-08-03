mod bitboard;
mod square;
mod move_gen;

use crate::square::*;
use crate::move_gen::lookup::*;

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

fn main() {
    println!("{:?}\n\n\n{:?}", Square::E4.bb(), n_gen_lookup(Square::E4.bb()));
}
