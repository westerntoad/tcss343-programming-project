use rand::prelude::*;

use crate::bitboard::Bitboard;
use crate::square::Square;
use crate::gen::rook::{
    r_gen_shift_2_helper,
    MASK
};

pub static mut ROOK_MAGICS: [MagicEntry; Square::NUM] = [MagicEntry::EMPTY; Square::NUM];
pub static mut ROOK_MOVES: [[Bitboard; 1 << 12]; Square::NUM] = [[Bitboard::EMPTY; 1 << 12]; Square::NUM];

pub const ROOK_MAGICS_CONST: [MagicEntry; Square::NUM] = [
    MagicEntry { mask: Bitboard::new(0x000101010101017E), magic: 0x0780014001A08010, index_bits: 12 },
    MagicEntry { mask: Bitboard::new(0x000202020202027C), magic: 0x1140200010044000, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x000404040404047A), magic: 0x820010800A002240, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x0008080808080876), magic: 0x910020480C100100, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x001010101010106E), magic: 0x2980040080080036, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x002020202020205E), magic: 0x0100240021000208, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x004040404040403E), magic: 0x80800E0005800100, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x008080808080807E), magic: 0x020000C024009106, index_bits: 12 },
    MagicEntry { mask: Bitboard::new(0x0001010101017E00), magic: 0x18008004C0042680, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x0002020202027C00), magic: 0x0009004004810024, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0004040404047A00), magic: 0x20020020805200C8, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0008080808087600), magic: 0x0480808008001000, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0010101010106E00), magic: 0x0002000C48106200, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0020202020205E00), magic: 0x05418004000A0080, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0040404040403E00), magic: 0x1001002100040A00, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0080808080807E00), magic: 0x04A2000A20C08504, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x00010101017E0100), magic: 0x0002A08001804008, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x00020202027C0200), magic: 0x0880808040002004, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00040404047A0400), magic: 0x0A50420010820420, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0008080808760800), magic: 0x4404808008001004, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00101010106E1000), magic: 0x0002808068005400, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00202020205E2000), magic: 0x800308011040200C, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00404040403E4000), magic: 0x20000C0006080110, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00808080807E8000), magic: 0x00000A0001044084, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x000101017E010100), magic: 0x4000802280084004, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x000202027C020200), magic: 0x4800200040005000, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x000404047A040400), magic: 0x4400401300200100, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0008080876080800), magic: 0x0010003100082100, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x001010106E101000), magic: 0x2808020040040040, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x002020205E202000), magic: 0x1000220080040080, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x004040403E404000), magic: 0x00010F0400900208, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x008080807E808000), magic: 0x0380007200008409, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x0001017E01010100), magic: 0x000080C002800427, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x0002027C02020200), magic: 0x9048601001400444, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0004047A04040400), magic: 0x0000100080802000, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0008087608080800), magic: 0x8000820800801000, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0010106E10101000), magic: 0x0004080080802400, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0020205E20202000), magic: 0x5802000C00810080, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0040403E40404000), magic: 0x20514B0214000810, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0080807E80808000), magic: 0x06080400A2000041, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x00017E0101010100), magic: 0x208080A040008000, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x00027C0202020200), magic: 0x1110002001504000, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00047A0404040400), magic: 0x8020018850008020, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0008760808080800), magic: 0x0010012100110008, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00106E1010101000), magic: 0x0101014800510004, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00205E2020202000), magic: 0x0007000804010012, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00403E4040404000), magic: 0x00005850410C0012, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x00807E8080808000), magic: 0x00910010C089000E, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x007E010101010100), magic: 0x00B0400024800080, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x007C020202020200), magic: 0x0100401180200880, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x007A040404040400), magic: 0x0C00A00280100080, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x0076080808080800), magic: 0x2004500121022900, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x006E101010101000), magic: 0xC405440088008080, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x005E202020202000), magic: 0x0204840062008080, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x003E404040404000), magic: 0x0102211018220400, index_bits: 10 },
    MagicEntry { mask: Bitboard::new(0x007E808080808000), magic: 0x2080801051000880, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x7E01010101010100), magic: 0x1850230080004011, index_bits: 12 },
    MagicEntry { mask: Bitboard::new(0x7C02020202020200), magic: 0x4000400081150821, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x7A04040404040400), magic: 0x0010600088401101, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x7608080808080800), magic: 0x0008088421001001, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x6E10101010101000), magic: 0x0001000C70060801, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x5E20202020202000), magic: 0x0001000208CC0021, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x3E40404040404000), magic: 0x001008270210208C, index_bits: 11 },
    MagicEntry { mask: Bitboard::new(0x7E80808080808000), magic: 0x204C10A054008502, index_bits: 12 }
];

#[derive(Clone, Copy)]
pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: u64,
    pub index_bits: u8
}

struct TableFillError;

impl MagicEntry {
    
    const EMPTY: MagicEntry = Self {
        mask: Bitboard::EMPTY,
        magic: 0,
        index_bits: 12
    };

    pub fn get_index(&self, blockers: Bitboard) -> usize {
        let masked_blockers = blockers & self.mask;
        let hash = masked_blockers.val().wrapping_mul(self.magic);
        
        (hash >> (64 - self.index_bits)) as usize
    }
}

pub fn init_rook_magics() {
    for i in 0..Square::NUM {
        let bb = Square::new(i.try_into().unwrap()).unwrap().bb();
        //print!("Initializing magics for {}... ", Square::new(i as u8).unwrap());
        let index_bits = MASK[bb.index()].pop_count() as u8;
        let (magic_entry, mut vec) = find_rook_magic(bb, index_bits);

        unsafe {
            ROOK_MAGICS[i] = magic_entry;
            vec.set_len(1 << 12);
            let arr: [Bitboard; 1 << 12] = vec.as_slice().try_into().unwrap();
            ROOK_MOVES[i] = arr;
        }
        
        println!(
            "    MagicEntry {{ mask: Bitboard::new(0x{:016X}), magic: 0x{:016X}, index_bits: {} }}",
            magic_entry.mask.val(), magic_entry.magic, magic_entry.index_bits
        );
        //println!("Done!");
    }
}

pub fn init_rook_moves() {
    for i in 0..Square::NUM {
        let bb = Square::new(i.try_into().unwrap()).unwrap().bb();
        let mut table = vec![Bitboard::EMPTY; 1 << 12];
        let magic_entry = ROOK_MAGICS_CONST[i];
        let mask = magic_entry.mask;

        let mut blockers = Bitboard::EMPTY;
        loop {
            let moves = r_gen_shift_2_helper(bb, blockers);
            table[magic_entry.get_index(blockers)] = moves;

            blockers = Bitboard::new(blockers.val().wrapping_sub(mask.val()) & mask.val());
            if blockers.is_empty() {
                break;
            }
        }

        unsafe {
            ROOK_MOVES[i] = table.as_slice().try_into().unwrap()
        }
    }
}

pub fn find_rook_magic(orig: Bitboard, index_bits: u8) -> (MagicEntry, Vec<Bitboard>) {
    let mask = MASK[orig.index()];
    let mut rng = thread_rng();

    loop {
        let magic: u64 = rng.next_u64() & rng.next_u64() & rng.next_u64();
        let magic_entry = MagicEntry { mask, magic, index_bits };
        
        if let Ok(table) = try_rook_table(orig, &magic_entry) {
            return (magic_entry, table)
        }
    }
}

pub fn try_rook_table(orig: Bitboard, magic: &MagicEntry) -> Result<Vec<Bitboard>, TableFillError> {
    let mut table = vec![Bitboard::EMPTY; 1 << magic.index_bits];

    let mut blockers = Bitboard::EMPTY;
    loop {
        let moves = r_gen_shift_2_helper(orig, blockers);

        let table_entry = &mut table[magic.get_index(blockers)];
        if table_entry.is_empty() {
            *table_entry = moves;
        } else if *table_entry != moves {
            return Err(TableFillError);
        }
        
        // Carry-Rippler trick that enumerates all subsets of the mask, getting us all blockers.
        // https://www.chessprogramming.org/Traversing_Subsets_of_a_Set#All_Subsets_of_any_Set
        blockers = Bitboard::new(blockers.val().wrapping_sub(magic.mask.val()) & magic.mask.val());
        if blockers.is_empty() {
            break;
        }
    }
    Ok(table)
}
