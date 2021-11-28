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

    board.parse_fen("8/1p6/1k1Q4/4r3/2P6/3p4/2p6/8 b KQkq e3 1 2");

    board.print_attacks();

    board.print_board();

    board.generate_moves();

    movegen::refresh(&board, &mut 0, &mut 0);

    let mut bb = 0u64;

    let mut ray_between: [[u64; 64]; 64] = [[0; 64]; 64];
    for kingsq in 12..13 as u64 {
        for attacksquare in 0..64 as u64 {
            let mut board: u64 = 0;
            let mut squares: Vec<usize> = Vec::new();
            let dif = (attacksquare as i32 - kingsq as i32).signum(); // negative if atk square is less than kingsquare
            let mut samefile;
            let mut samerank;
            let mut samediag;
            let mut sameadiag;
            if dif == 1 {
                samefile = (attacksquare - kingsq) & 7 == 0;
                samerank = (attacksquare >> 3) - (kingsq >> 3) == 0; // (attacksquare >> 3) - (kingsq >> 3) == 0;

                samediag = (attacksquare >> 3) - (kingsq >> 3) == (attacksquare - kingsq) & 7;
                sameadiag = (attacksquare >> 3) - (kingsq >> 3) + (attacksquare - kingsq) & 7 == 0;
            } else {
                samefile = (kingsq - attacksquare) & 7 == 0;
                samerank = (kingsq >> 3) - (attacksquare >> 3) == 0;
                samediag = (kingsq >> 3) - (attacksquare >> 3) == (kingsq - attacksquare) & 7;
                sameadiag = (kingsq >> 3) - (attacksquare >> 3) + (kingsq - attacksquare) & 7 == 0;
            }
            if dif == 1 {
                println!(
                    "{}, {}, {}",
                    attacksquare,
                    (attacksquare - kingsq) & 7,
                    (attacksquare >> 3) - (kingsq >> 3)
                );
            } else {
                println!(
                    "{}, {}, {}",
                    attacksquare,
                    (kingsq - attacksquare) & 7,
                    (kingsq >> 3) - (attacksquare >> 3)
                );
            }

            if samediag {
                set_bit!(bb, attacksquare)
            }

            let mut target = kingsq as i32;
            if dif == 0 {
                continue;
            }
            if samefile {
                while target != attacksquare as i32 {
                    target = target + 8 * dif;
                    squares.push(target as usize);
                }
            } else if samerank {
                while target != attacksquare as i32 {
                    target = target + 1 * dif;
                    squares.push(target as usize);
                }
            } else if samediag {
                while target != attacksquare as i32 {
                    target = target + 9 * dif;
                    squares.push(target as usize);
                    if target % 8 == 0 || target % 8 == 7 || target / 8 == 0 || target / 8 == 7 {
                        if target != attacksquare as i32 {
                            squares.clear();
                            break;
                        }
                    }
                    /*print_bitboard(board);
                    println!(
                        "{}. {}",
                        piececonstants::SQUARE_TO_COORDINATES[kingsq as usize],
                        piececonstants::SQUARE_TO_COORDINATES[attacksquare as usize]);*/
                }
            } else if sameadiag {
                while target != attacksquare as i32 {
                    target = target + 7 * dif;
                    squares.push(target as usize);
                    if target % 8 == 0 || target % 8 == 7 || target / 8 == 0 || target / 8 == 7 {
                        if target != attacksquare as i32 {
                            squares.clear();
                            break;
                        }
                    }
                    /*print_bitboard(board);
                    println!(
                        "{}. {}",
                        piececonstants::SQUARE_TO_COORDINATES[kingsq as usize],
                        piececonstants::SQUARE_TO_COORDINATES[attacksquare as usize]);*/
                }
            }
            for square in squares {
                set_bit!(board, square);
            }
            ray_between[kingsq as usize][attacksquare as usize] = board;
        }
    }
    for i in 0..64 {
        println!("{}", piececonstants::SQUARE_TO_COORDINATES[i]);
        print_bitboard(ray_between[12][i]);
    }
    /*for square in 0..64 {
        print_bitboard(ray_between[23][square]);
    }*/

    //print_bitboard(board.occupancies[0]);
    //let tic = Instant::now();

    /*for square in 0..64 {
        print_bitboard(get_rook_attacks(square, occupancy));
    }*/
    //let toc = Instant::now();

    //println!("{:#?}", SLIDER_STUFF_2);
}
