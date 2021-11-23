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

fn print_bitboard(bitboard: u64) -> () {
    //prints a bitboard
    println!();
    for rank in 0..8 {
        for file in 0..8 {
            // init board square, turn file and rank into square
            let square = rank * 8 + file;
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
    print_bitboard(piececonstants::PAWN_ATTACKS[0][piececonstants::Square::C5 as usize]);

    println!("{}", piececonstants::UNICODE_PIECES[2]);

    let mut board = engine::Board {
        ..Default::default()
    };
    set_bit!(board.pieceboards[2], 23);
    set_bit!(board.pieceboards[7], 54);
    board.side = Some(0);
    //board.castle = 13;

    board.parse_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq e3 1 2");

    board.print_board();

    //print_bitboard(board.occupancies[0]);
    //let tic = Instant::now();

    /*for square in 0..64 {
        print_bitboard(get_rook_attacks(square, occupancy));
    }*/
    //let toc = Instant::now();

    //println!("{:#?}", SLIDER_STUFF_2);
}
