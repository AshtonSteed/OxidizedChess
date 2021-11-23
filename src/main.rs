#[macro_use]
extern crate lazy_static;
#[macro_use]
mod engine;
mod piececonstants;
mod pieceinit;
//                                          enums and constants
/*const NOTAFILE: u64 = 18374403900871474942; // masks giving 1s for all files but the edge files
const NOTHFILE: u64 = 9187201950435737471; //probably not needed in this file
const NOTHGFILE: u64 = 4557430888798830399;
const NOTABFILE: u64 = 18229723555195321596;*/
//use .count_ones to count bits
//use .trailing_zeros to find least significant bit index

pub fn print_bitboard(bitboard: u64) -> () {
    //prints a bitboard
    println!();
    for rank in 0..8 {
        for file in 0..8 {
            // init board square, turn file and rank into square
            let square = rank * 8 + file;
            //print!("{}", square);
            if file == 0 {
                print!("{}  ", 8 - rank);
            }
            //println!("{}", bitboard & 1u64 << square);
            print!("{} ", get_bit!(bitboard, square));
        }
        //print new line to seperate ranks
        println!();
    }
    println!("   a b c d e f g h");

    println!("Bitboard Value: {}", bitboard)
}

//                                              attacks

//                                              main driver

//use std::time::{Duration, Instant};

fn main() {
    let mut board = engine::Board {
        ..Default::default()
    };

    board.parse_fen("8/1p6/1k1q4/4r3/2P6/3p4/2p6/8 w KQkq e3 1 2");

    board.print_attacks();

    board.print_board();

    board.generate_moves();

    //print_bitboard(board.occupancies[0]);
    //let tic = Instant::now();

    /*for square in 0..64 {
        print_bitboard(get_rook_attacks(square, occupancy));
    }*/
    //let toc = Instant::now();

    //println!("{:#?}", SLIDER_STUFF_2);
}
