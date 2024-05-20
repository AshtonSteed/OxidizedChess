#![allow(dead_code)]
#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;
use std::time::Instant;

use rustfft::{num_complex::Complex, FftPlanner};

#[macro_use]
mod engine;

use engine::{BitBoard, Board};
use movegen::{generate_captures, generate_moves};
use rand::Rng;
use uci::uci_loop;

use crate::moves::MoveStuff;

mod fourier;
mod movegen;
mod moves;
mod piececonstants;
mod pieceinit;
mod search;
mod ttable;
mod uci;
//                                          enums and constants
/*const NOTAFILE: u64 = 18374403900871474942; // masks giving 1s for all files but the edge files
const NOTHFILE: u64 = 9187201950435737471; //probably not needed in this file
const NOTHGFILE: u64 = 4557430888798830399;
const NOTABFILE: u64 = 18229723555195321596;*/
//use .count_ones to count bits
//use .trailing_zeros to find least significant bit index

//                                              attacks

//                                              main driver

//use std::time::{Duration, Instant};

//cargo rustc --bin OxidizedChess --release -- -Z emit-stack-sizes

fn main() {
    // TODO: figure out how to refine values, consider aspiration windows, reconsider draw stuff again
    // add incremental time controls
    // Might be an issue where a pawn can attack and capture a king

    uci_loop();
    /*
    let mut board = Board::default();

    board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    board.print_board();

    board.pieceboards[5].print_bitboard();

    let mut result = vec![
        vec![
            Complex {
                re: 0.0f32,
                im: 0.0f32
            };
            144
        ];
        2
    ];
    let mut result2 = result[0].clone();
    let mut scratch = result[0].clone();

    let mut filter = vec![vec![1.; 3]; 3];
    filter[1][1] = 3.;
    //filter[0][0] = 3.;

    fourier::lazy_bitboard_fft(&mut result[0], &mut scratch, &board.pieceboards[5]);
    fourier::lazy_bitboard_fft(&mut result[1], &mut scratch, &board.pieceboards[9]);
    fourier::filter_fft(&mut result2, &mut scratch, &filter);
    fourier::signal_conv(&mut result[0], &result2);
    fourier::signal_conv(&mut result[1], &result2);
    fourier::collapse_conv(&mut result2, &mut result);
    fourier::ifft(&mut result2, &mut scratch);

    fourier::print_ifft(&result2);
    let now = Instant::now();
    for i in 0..12 {
        fourier::lazy_bitboard_fft(&mut result[0], &mut scratch, &board.pieceboards[i]);
    }
    let elapsed_time = now.elapsed();
    println!(
        "Computing FFT's by lookup took {} micros.",
        elapsed_time.as_micros()
    );

    let now = Instant::now();
    for i in 0..12 {
        fourier::bitboard_fft(&mut result[0], &mut scratch, &board.pieceboards[i]);
    }
    let elapsed_time = now.elapsed();
    println!(
        "Computing FFT's normally took {} micros.",
        elapsed_time.as_micros()
    ); */

    /*
    let now = Instant::now();

    let mut result = vec![
        vec![
            Complex {
                re: 0.0f32,
                im: 0.0f32
            };
            144
        ];
        64 * 64
    ];
    let mut scratch = result[0].clone();
    for i in 0..64 {
        for j in 0..64 {
            let bitobard = (1 << i) | (1 << j);
            fourier::bitboard_fft(&mut result[i], &mut scratch, &bitobard);
        }
    }
    let elapsed_time = now.elapsed();
    println!(
        "Computing FFT's took {} milliseconds.",
        elapsed_time.as_millis()
    );*/
    /*let mut board = Board::default();
    let mut moves = vec![0; 256];
    board.parse_fen("k2r4/8/8/8/8/3K4/8/8 b - - 0 1".to_string());
    let i = generate_captures(&mut board, &mut moves);
    println!("{}", i);
    // NO clue what the threshold is
    // Lower threshold allows for worse moves to be considered good, honestly probably good

    board.print_board();

    println!("{}", board.evaluate(0));*/

    //board.movemasks[0].print_bitboard();
    //board.movemasks[1].print_bitboard();
    //board.movemasks[2].print_bitboard();
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
