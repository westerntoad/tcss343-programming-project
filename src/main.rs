mod square;
mod bitboard;
mod gen;
mod magic;

use crate::square::*;
use crate::bitboard::*;
use crate::magic::*;
use crate::gen::{
    knight::*,
    king::*,
    pawn::*,
    bishop::*,
    rook::*,
    queen::*
};

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

fn print_rook_masks() {
    for i in 0..64 {
        let sq = Square::new(i).unwrap();
        let attacks = {
            let orig = sq.bb();
            let mut attacks = Bitboard::EMPTY;

            let mut idx = orig.nort_one();
            while !idx.nort_one().is_empty() {
                attacks |= idx;
                idx = idx.nort_one();
            }
            
            idx = orig.east_one();
            while !idx.east_one().is_empty() {
                attacks |= idx;
                idx = idx.east_one();
            }

            idx = orig.sout_one();
            while !idx.sout_one().is_empty() {
                attacks |= idx;
                idx = idx.sout_one();
            }

            idx = orig.west_one();
            while !idx.west_one().is_empty() {
                attacks |= idx;
                idx = idx.west_one();
            }
    
            attacks
        };
        //let attacks = r_gen_shift_2_helper(sq.bb(), Bitboard::EMPTY);
        let code = format!("{:#016x}", attacks.val());
        println!("    Bitboard::new({code}),");
    }
}


pub fn get_all_rook_magics() -> (String, String){
    let mut magic_str = String::new();
    let mut move_str = String::new();

    magic_str.push_str("pub const ROOK_MAGICS: &[MagicEntry; Square::NUM] = &[\n");
    move_str.push_str("pub const ROOK_MOVES: &[&[Bitboard]; Square::NUM] = &[\n");
    
    for i in 0..Square::NUM {
        let bb = Square::new(i.try_into().unwrap()).unwrap().bb();
        let index_bits = MASK[bb.index()].pop_count() as u8;
        let (magic_entry, vec) = find_rook_magic(bb, index_bits);

        magic_str.push_str(&format!(
            "    MagicEntry {{ mask: Bitboard::new(0x{:016X}), magic: 0x{:016X}, index_bits: {} }}",
            magic_entry.mask.val(), magic_entry.magic, magic_entry.index_bits
        ));
        magic_str.push_str(match i {
            63 => "\n",
            _  => ",\n"
        });
        
        move_str.push_str("    &[\n");
        let mut iter = vec.iter().peekable();
        while let Some(bb_i) = iter.next() {
            let code = format!("{:#016x}", n_gen_shift(*bb_i).val());
            move_str.push_str(&format!("        Bitboard::new({})", code));
            move_str.push_str(if iter.peek().is_none()
                { "\n" } else { ",\n" });
        }
        move_str.push_str("    ]");
        
        move_str.push_str(match i {
            63 => "\n",
            _  => ",\n"
        });
    }

    magic_str.push_str("];");
    move_str.push_str("];");

    (magic_str, move_str)
}



fn main() {
    //init_rook_magics();
    init_rook_moves();
    /* 
    let origin = Bitboard::E4;
    let index_bits = MASK[origin.index()].pop_count() as u8;
    let (magic_entry, vec) = find_rook_magic(origin, index_bits);
    let blockers = Bitboard::E8;
    //println!("{:#?}", MASK[origin.index()]);
    //println!("{:#?}", r_gen_shift_2_helper(origin, blockers));

    let output = vec[magic_entry.get_index(blockers)];
    println!("{:#?}", output);
    */

    //let (magic_str, move_str) = get_all_rook_magics();
    //println!("{}", magic_str);
    //for _ in 0..50 {
        //println!();
    //}
    //println!("{}", move_str);

    /*unsafe {
        init_rook_magics();
        for entry in MAGICS {
            println!("{:?}", entry.mask);
        }
    }*/

    let bb = Bitboard::E4;
    let blockers = Bitboard::A1 | Bitboard::E6;
    println!("{:#?}", r_gen_magic(bb, blockers, Bitboard::EMPTY));
}

