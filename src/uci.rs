use crate::engine::Board;
use crate::movegen::generate_moves;
use crate::moves::{make_move, MoveStuff};
use crate::piececonstants;
use crate::search::{search_position, Transpositiontable};

use std::io::{self, BufRead};
use std::time::{Duration, Instant};

// move parse from text input, can be human entered or from any UCI
pub fn parse_move(input: String, board: &mut Board) -> u16 {
    let mut moves = vec![0u16; 256];
    let _count = generate_moves(board, &mut moves);
    for m in moves {
        if m.to_uci() == input {
            return m;
        }
    }
    println!("Move not legal: {}", input);
    0
}
// startpos -> starting fen
// fen -> parse fen after
// make all moves following moves after either
pub fn parse_position(
    input: String,
    board: &mut Board,

    halfcount: &mut usize,
    ttable: &mut Transpositiontable,
) {
    *halfcount = 0;

    let split = input.split(' ');
    let segments: Vec<&str> = split.collect();
    let mut movei = 2;
    let fen = match segments[1] {
        "startpos" => "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        "fen" => {
            let mut f = segments[2].to_owned();
            for i in movei + 1..=7 {
                f += " ";
                f += segments[i];
            }
            movei += 6;
            f
        }
        _ => panic!("Position command for {} not found", segments[1]),
    };

    board.parse_fen(fen);
    *halfcount += 1;
    let mut past = vec![board.key];
    // First find how many moves are non repeatable while updating the board.

    if segments.len() > movei {
        //ttable.set_value(board.key, u16::MAX, usize::MAX, piececonstants::CONTEMPT);
        for i in &segments[movei + 1..] {
            let m = parse_move(i.to_string(), board);
            let temp = board.clone();
            make_move(board, &m);
            past.push(temp.key);

            if !temp.is_repeat(&board) {
                //println!("Move {} is not repeatable", m.to_uci());
                *halfcount = 0;
            }
            if ttable.read_move(board.key) == None {
                ttable.clear_entry(board.key);
            }

            *halfcount += 1;
        }
        let length = past.len();
        for i in 2..*halfcount {
            let pos = past[length - i];
            if pos != board.key {
                ttable.set_value(pos, u16::MAX, usize::MAX, piececonstants::CONTEMPT)
            }
        }
    }
}

// tells engine to calculate for move
// ex: go depth 3
//other go forms exist, will deal with them later
pub fn parse_go(
    input: String,
    board: &mut Board,
    ttable: &mut Transpositiontable,

    halfcount: &mut usize,
) {
    let split = input.split(' ');
    let segments: Vec<&str> = split.collect();
    let (depth, timelimit): (usize, Duration) = match segments[1] {
        "depth" => (segments[2].parse().unwrap(), Duration::MAX),
        "movetime" => (
            piececonstants::MAXPLY,
            Duration::from_millis(segments[2].parse().unwrap()),
        ),
        "wtime" => {
            let timelimit: u64 = {
                if board.side == Some(0) {
                    segments[2].parse().unwrap()
                } else {
                    segments[4].parse().unwrap()
                }
            };
            let movetime = Duration::from_millis(timelimit / 30); // estimates 25 moves left in game at any time

            (piececonstants::MAXPLY, movetime)
        }
        _ => (10, Duration::MAX), // placeholder for other moves, default to depth of 10
    };

    let m = search_position(board, depth, timelimit, ttable, halfcount.clone());
    println!("bestmove {}", m.to_uci());
    let mut temp = board.clone();
    make_move(&mut temp, &m);
}

pub fn communicate(stopped: &mut bool, starttime: Instant, timelimit: Duration) {
    if starttime.elapsed() >= timelimit {
        *stopped = true;
    }
}

//main uci and engine loop
pub fn uci_loop() {
    let mut lines = io::stdin().lock().lines();

    let mut board = Board {
        ..Default::default()
    };

    let mut ttable = Transpositiontable::new(); // transpotion table
                                                //let mut rtable = Repititiontable::new(); // repitiion table

    let mut halfclock: usize = 0;
    while let Some(line) = lines.next() {
        let input = line.unwrap().to_string();
        let split = input.split(' ');
        let segments: Vec<&str> = split.collect();
        match segments[0] {
            "uci" => {
                println!("id name OxidizedChess");
                println!("id author Ashton Steed");
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "ucinewgame" => {
                board = Board {
                    ..Default::default()
                };
                ttable = Transpositiontable::new();
                //rtable = Repititiontable::new();

                halfclock = 0;
                // TODO: add anything else that needs reset in new game
            }
            "position" => {
                parse_position(input, &mut board, &mut halfclock, &mut ttable);
                //history.insert(0, board.key.clone());
                //cahalfclock += 1;
            }
            "go" => {
                parse_go(input, &mut board, &mut ttable, &mut halfclock);
                // analyze board position
            }
            "quit" => break,

            _ => {} //panic!("Unkown command: {}", input),
        }
    }
    board.print_board();
}
