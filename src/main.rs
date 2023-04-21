#![allow(dead_code)]

use movegen::generate_moves;
use piececonstants::RAY_BETWEEN;

#[macro_use]
extern crate lazy_static;
#[macro_use]
mod engine;
use moves::{make_move, score_move, MoveStuff};
use rand::Rng;
use uci::uci_loop;

use crate::engine::Board;
mod movegen;
mod moves;
mod piececonstants;
mod pieceinit;
mod search;
mod uci;
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
    // TODO: movescoring remake (maybe), Time control stuff,  figure out how to test ELO and refine values
    // also add a system to keep track of gamestate over time and check for repititions better
    // futility pruning could be useful

    uci_loop();
}

/*  let king_attack = movegen::refresh(&mut board);
a
println!("King Moves");
print_bitboard(king_attack);
println!("Attacked Squares");
print_bitboard(board.movemasks[0]);

println!("Checking Mask");
print_bitboard(board.movemasks[1]);

println!("Horizontal Pins");
print_bitboard(board.movemasks[2]);

println!("Diagonal Pins");
print_bitboard(board.movemasks[3]);

println!("Enpassant Target");
print_bitboard(board.enpassant);

let moves = movegen::generate_moves(&mut board);
for a in &moves {
    a.print()
}
println!("Moves: {}", moves.len());
board.print_board();
moves::make_move(&mut board, moves[0]);
board.print_board(); */
