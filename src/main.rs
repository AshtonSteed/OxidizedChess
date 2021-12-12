#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
mod engine;
mod movegen;
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

    board.parse_fen("8/2n5/8/1K1B11r1/8/8/8/8 w - - 0 1");

    board.print_attacks();

    board.print_board();

    board.generate_moves();

    movegen::refresh(&board, &mut 0, &mut 0, &mut 0, &mut 0);

    //print_bitboard(board.occupancies[0]);
    //let tic = Instant::now();

    /*for square in 0..64 {
        print_bitboard(get_rook_attacks(square, occupancy));
    }*/
    //let toc = Instant::now();

    //println!("{:#?}", SLIDER_STUFF_2);
}

//probably obsolete raw generator

/*let mut ray_between: [[u64; 64]; 64] = [[0; 64]; 64];
for kingsq in 0..64 as i32 {
    for attacksquare in 0..64 as i32 {
        let mut board: u64 = 0;
        let kingrank = kingsq / 8;
        let kingfile = kingsq % 8;
        let attackrank = attacksquare / 8;
        let attackfile = attacksquare % 8;
        let rankdif = kingrank - attackrank;
        let filedif = kingfile - attackfile;
        if rankdif == 0 && filedif == 0 {
            // case where atk square and king overlap, no fill needed
            continue;
        } else if filedif == 0 {
            let increment = rankdif.signum() * 8;
            let mut target = kingsq;
            while target != attacksquare {
                target -= increment;
                set_bit!(board, target);
            }
            target = kingsq;
            set_bit!(board, target);
            while target / 8 != 0 || target / 8 != 7 {
                set_bit!(board, target);
                target += increment;
            }
        } else if rankdif == 0 {
            let increment = filedif.signum();
            let mut target = kingsq;
            while target != attacksquare {
                target -= increment;
                set_bit!(board, target);
            }
            target = kingsq;
            set_bit!(board, target);
            while target % 8 != 0 || target % 8 != 7 {
                print_bitboard(board);
                println!("{}, {}", kingsq, attacksquare);
                target += increment;
                set_bit!(board, target);
            }
        } else if rankdif.abs() == filedif.abs() {
            let increment = rankdif.signum() * 8 + filedif.signum();
            let mut target = kingsq;
            while target != attacksquare {
                target -= increment;
                set_bit!(board, target);
            }
            target = kingsq;
            set_bit!(board, target);
            while target / 8 != 0 || target / 8 != 7 || target % 8 != 0 || target % 8 != 7 {
                target += increment;
                set_bit!(board, target);
            }
        }

        ray_between[kingsq as usize][attacksquare as usize] = board;
    }
}*/
