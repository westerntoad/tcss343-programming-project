use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use move_gen_analysis::{
    bitboard::Bitboard,
    square::Square
};
use move_gen_analysis::magic::{
    init_rook_moves,
    init_bishop_moves
};
use move_gen_analysis::gen::{
    knight::{
        n_gen_shift,
        n_gen_match,
        n_gen_lookup
    },
    king::{
        k_gen_shift,
        k_gen_match,
        k_gen_lookup
    },
    pawn::p_gen_shift,
    rook::{
        r_gen_shift_1,
        r_gen_shift_2,
        r_gen_magic
    },
    bishop::{
        b_gen_shift_1,
        b_gen_shift_2,
        b_gen_magic
    },
    queen::{
        q_gen_shift_1,
        q_gen_shift_2,
        q_gen_magic
    }

};


pub fn bench_knights(c: &mut Criterion) {
    let mut group = c.benchmark_group("Knight");

    let mut bb = Bitboard::A1;
    let mut category = "Corner";
    group.bench_with_input(BenchmarkId::new("Calculation", category), &bb,
        |b, i| b.iter(|| n_gen_shift(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Match", category), &bb,
        |b, i| b.iter(|| n_gen_match(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Array", category), &bb,
        |b, i| b.iter(|| n_gen_lookup(*i)));

    bb = Bitboard::H4;
    category = "Edge";
    group.bench_with_input(BenchmarkId::new("Calculation", category), &bb,
        |b, i| b.iter(|| n_gen_shift(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Match", category), &bb,
        |b, i| b.iter(|| n_gen_match(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Array", category), &bb,
        |b, i| b.iter(|| n_gen_lookup(*i)));

    bb = Bitboard::D5;
    category = "Center";
    group.bench_with_input(BenchmarkId::new("Calculation", category), &bb,
        |b, i| b.iter(|| n_gen_shift(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Match", category), &bb,
        |b, i| b.iter(|| n_gen_match(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Array", category), &bb,
        |b, i| b.iter(|| n_gen_lookup(*i)));
    
    group.finish()
}

pub fn bench_kings(c: &mut Criterion) {
    let mut group = c.benchmark_group("King");

    let mut bb = Bitboard::A1;
    let mut category = "Corner";
    group.bench_with_input(BenchmarkId::new("Calculation", category), &bb,
        |b, i| b.iter(|| k_gen_shift(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Match", category), &bb,
        |b, i| b.iter(|| k_gen_match(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Array", category), &bb,
        |b, i| b.iter(|| k_gen_lookup(*i)));

    bb = Bitboard::H4;
    category = "Edge";
    group.bench_with_input(BenchmarkId::new("Calculation", category), &bb,
        |b, i| b.iter(|| k_gen_shift(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Match", category), &bb,
        |b, i| b.iter(|| k_gen_match(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Array", category), &bb,
        |b, i| b.iter(|| k_gen_lookup(*i)));

    bb = Bitboard::D5;
    category = "Center";
    group.bench_with_input(BenchmarkId::new("Calculation", category), &bb,
        |b, i| b.iter(|| k_gen_shift(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Match", category), &bb,
        |b, i| b.iter(|| k_gen_match(*i)));
    group.bench_with_input(BenchmarkId::new("Lookup-Array", category), &bb,
        |b, i| b.iter(|| k_gen_lookup(*i)));
    
    group.finish()
}

pub fn bench_pawns(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pawn");
    
    let mut orig = Bitboard::E2;
    let mut is_white = true;
    let mut friend = Bitboard::EMPTY;
    let mut enemy = Bitboard::EMPTY;
    let mut en_passant = Bitboard::EMPTY;
    let mut input = (orig, is_white, friend, enemy, en_passant);
    let mut category = "Start";
    group.bench_with_input(BenchmarkId::new("Shift", category), &input,
        |b, (o, w, f, e, ep)| b.iter(|| p_gen_shift(*o, *w, *f, *e, *ep)));
    
    orig = Bitboard::D5;
    is_white = true;
    friend = Bitboard::EMPTY;
    enemy = Bitboard::EMPTY;
    en_passant = Bitboard::EMPTY;
    input = (orig, is_white, friend, enemy, en_passant);
    category = "Center";
    group.bench_with_input(BenchmarkId::new("Shift", category), &input,
        |b, (o, w, f, e, ep)| b.iter(|| p_gen_shift(*o, *w, *f, *e, *ep)));

    orig = Bitboard::B3;
    is_white = true;
    friend = Bitboard::B4;
    enemy = Bitboard::EMPTY;
    en_passant = Bitboard::EMPTY;
    input = (orig, is_white, friend, enemy, en_passant);
    category = "Blocked";
    group.bench_with_input(BenchmarkId::new("Shift", category), &input,
        |b, (o, w, f, e, ep)| b.iter(|| p_gen_shift(*o, *w, *f, *e, *ep)));

    orig = Bitboard::G6;
    is_white = true;
    friend = Bitboard::EMPTY;
    enemy = Bitboard::F7 | Bitboard::H7;
    en_passant = Bitboard::EMPTY;
    input = (orig, is_white, friend, enemy, en_passant);
    category = "Attacks";
    group.bench_with_input(BenchmarkId::new("Shift", category), &input,
        |b, (o, w, f, e, ep)| b.iter(|| p_gen_shift(*o, *w, *f, *e, *ep)));

    orig = Bitboard::H5;
    is_white = true;
    friend = Bitboard::EMPTY;
    enemy = Bitboard::EMPTY;
    en_passant = Bitboard::G6;
    input = (orig, is_white, friend, enemy, en_passant);
    category = "EnPassant";
    group.bench_with_input(BenchmarkId::new("Shift", category), &input,
        |b, (o, w, f, e, ep)| b.iter(|| p_gen_shift(*o, *w, *f, *e, *ep)));

    group.finish()
}

pub fn bench_rooks(c: &mut Criterion) {
    init_rook_moves();
    let mut group = c.benchmark_group("Rook");
    
    let mut orig = Bitboard::H8;
    let mut friend = Bitboard::EMPTY;
    let mut enemy = Bitboard::EMPTY;
    let mut input = (orig, friend, enemy);
    let mut category = "Corner";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_magic(*o, *f, *e)));
    
    orig = Square::A1.bb();
    friend = Square::F1.bb() | Square::C3.bb();
    enemy = Square::A5.bb() | Square::A6.bb() | Square::H1.bb() | Square::C6.bb();
    input = (orig, friend, enemy);
    category = "CornerPopulated";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_magic(*o, *f, *e)));

    orig = Bitboard::H3;
    friend = Bitboard::EMPTY;
    enemy = Bitboard::EMPTY;
    input = (orig, friend, enemy);
    category = "Edge";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_magic(*o, *f, *e)));
    
    orig = Bitboard::H3;
    friend = Bitboard::H1 | Bitboard::H6;
    enemy = Bitboard::E3 | Bitboard::A1 | Bitboard::B3;
    input = (orig, friend, enemy);
    category = "EdgePopulated";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_magic(*o, *f, *e)));

    orig = Bitboard::E5;
    friend = Bitboard::EMPTY;
    enemy = Bitboard::EMPTY;
    input = (orig, friend, enemy);
    category = "Center";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_magic(*o, *f, *e)));
     
    orig = Bitboard::D5;
    friend = Bitboard::D4 | Bitboard::H7 | Bitboard::G5 | Bitboard::C4;
    enemy = Bitboard::D8 | Bitboard::A5 | Bitboard::C3;
    input = (orig, friend, enemy);
    category = "CenterPopulated";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| r_gen_magic(*o, *f, *e)));
    

    group.finish()
}

pub fn bench_bishops(c: &mut Criterion) {
    init_bishop_moves();
    let mut group = c.benchmark_group("Bishop");
    
    let mut orig = Bitboard::H8;
    let mut friend = Bitboard::EMPTY;
    let mut enemy = Bitboard::EMPTY;
    let mut input = (orig, friend, enemy);
    let mut category = "Corner";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_magic(*o, *f, *e)));
    
    orig = Square::A1.bb();
    friend = Square::F6.bb() | Square::C4.bb();
    enemy = Square::A5.bb() | Square::A6.bb() | Square::H1.bb() | Square::C6.bb();
    input = (orig, friend, enemy);
    category = "CornerPopulated";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_magic(*o, *f, *e)));

    orig = Bitboard::H4;
    friend = Bitboard::EMPTY;
    enemy = Bitboard::EMPTY;
    input = (orig, friend, enemy);
    category = "Edge";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_magic(*o, *f, *e)));
    
    orig = Bitboard::H4;
    friend = Bitboard::H1 | Bitboard::F6;
    enemy = Bitboard::E1 | Bitboard::A1 | Bitboard::B3;
    input = (orig, friend, enemy);
    category = "EdgePopulated";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_magic(*o, *f, *e)));

    orig = Bitboard::E5;
    friend = Bitboard::EMPTY;
    enemy = Bitboard::EMPTY;
    input = (orig, friend, enemy);
    category = "Center";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_magic(*o, *f, *e)));
     

    orig = Square::F5.bb();
    friend = Square::E4.bb() | Square::H7.bb() | Square::G6.bb();
    enemy = Square::D3.bb() | Square::C8.bb() | Square::B5.bb() | Square::C2.bb();
    input = (orig, friend, enemy);
    category = "CenterPopulated";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| b_gen_magic(*o, *f, *e)));
    

    group.finish()
}

pub fn bench_queens(c: &mut Criterion) {
    init_rook_moves();
    init_bishop_moves();
    let mut group = c.benchmark_group("Queen");
    
    let mut orig = Bitboard::H8;
    let mut friend = Bitboard::EMPTY;
    let mut enemy = Bitboard::EMPTY;
    let mut input = (orig, friend, enemy);
    let mut category = "Corner";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_magic(*o, *f, *e)));
    
    orig = Square::A1.bb();
    friend = Square::F6.bb() | Square::C4.bb() | Square::F1.bb() | Square::C3.bb();
    enemy = Square::A5.bb() | Square::A6.bb() | Square::H1.bb() | Square::C6.bb();
    input = (orig, friend, enemy);
    category = "CornerPopulated";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_magic(*o, *f, *e)));

    orig = Bitboard::H4;
    friend = Bitboard::EMPTY;
    enemy = Bitboard::EMPTY;
    input = (orig, friend, enemy);
    category = "Edge";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_magic(*o, *f, *e)));
    
    orig = Bitboard::H4;
    friend = Bitboard::H1 | Bitboard::F6 | Bitboard::H6;
    enemy = Bitboard::E1 | Bitboard::A1 | Bitboard::B3 | Bitboard::E3;
    input = (orig, friend, enemy);
    category = "EdgePopulated";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_magic(*o, *f, *e)));

    orig = Bitboard::E5;
    friend = Bitboard::EMPTY;
    enemy = Bitboard::EMPTY;
    input = (orig, friend, enemy);
    category = "Center";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_magic(*o, *f, *e)));
     

    orig = Square::F5.bb();
    friend = Square::E4.bb() | Square::H7.bb() | Square::G6.bb() | Bitboard::D4 | Bitboard::G5 | Bitboard::C4;
    enemy = Square::D3.bb() | Square::C8.bb() | Square::B5.bb() | Square::C2.bb() | Bitboard::D8 | Bitboard::A5 | Bitboard::C3;
    input = (orig, friend, enemy);
    category = "CenterPopulated";
    group.bench_with_input(BenchmarkId::new("Shift-DoubleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_1(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("Shift-SingleBlock", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_shift_2(*o, *f, *e)));
    group.bench_with_input(BenchmarkId::new("MagicLookup", category), &input,
        |b, (o, f, e)| b.iter(|| q_gen_magic(*o, *f, *e)));
    

    group.finish()
}






criterion_group!(
    benches,
    bench_knights//,
    //bench_kings,
    //bench_pawns,
    //bench_rooks,
    //bench_bishops,
    //bench_queens
);
criterion_main!(benches); 

