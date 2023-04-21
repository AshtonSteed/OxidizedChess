use crate::{
    movegen::{self, generate_captures, generate_moves},
    moves::{self, make_move, null_move, MoveStuff},
    piececonstants, print_bitboard,
    uci::communicate,
};
use std::{
    cmp::{max, min},
    time::{Duration, Instant},
};

//          Bit macros
macro_rules! get_bit {
    //returns either 1 or 0, depending on if the square is a active bit
    ($bb:expr, $square:expr) => {
        if ($bb & 1u64 << $square) != 0 {
            1
        } else {
            0
        } // if statement checks if the and operator
          // between bitboard and square is non zero, checks and returns square bit value
    };
}
macro_rules! set_bit {
    //sets a bit on a board to a 1
    ($bb:expr, $square:expr) => {
        $bb |= 1u64 << $square // takes the or between the bitboard the the shifted square number
    };
}
macro_rules! pop_bit {
    //sets a bit on a board to a 0
    ($bb:expr, $square:expr) => {
        $bb &= !(1u64 << $square) // takes the nand between the bitboard and the shifted square
    };
}
#[derive(Copy, Clone)]
pub struct Board {
    pub pieceboards: [u64; 12], // [P, N, B, R, Q, K, p, n, b, r, q, k]
    pub occupancies: [u64; 3],  // [white occupancies, black occupancies, total occupancies]
    pub movemasks: [u64; 4],    // [attacks, checkmask, rookpin, bishoppin]
    pub side: Option<u8>, // side to move, None for invalid, 0 for white, 1 for black (Color enum)
    pub enpassant: u64,   // enpassant possible square (as a bitboard)
    pub castle: u8, // castling rights, 4 bits, 0001 1  white king kingside, 0010 2 white king queenside, 0100 4 black king king side, 1000 8 black king queen side
    pub key: u64,   //zorbrist hash key of position
}
impl Default for Board {
    fn default() -> Board {
        Board {
            pieceboards: [0; 12], // all boards to 0s
            occupancies: [0; 3],  // all occupancies to 0s
            movemasks: [0; 4],    // all masks to 0s
            side: None,           // side no worky
            enpassant: 0,         // enpassant possible square, no_sq
            castle: 0,
            key: 0,
        }
    }
}
impl Board {
    pub fn is_square_attacked(&self, square: usize) -> bool {
        // returns true if square is attacked by current side
        // only used
        let side = self.side.unwrap() != 0;
        let base = side as usize * 6;
        if piececonstants::PAWN_ATTACKS[!side as usize][square] & self.pieceboards[base] != 0 {
            return true;
        } else if piececonstants::KNIGHT_ATTACKS[square] & self.pieceboards[base + 1] != 0 {
            return true;
        } else if piececonstants::KING_ATTACKS[square] & self.pieceboards[base + 5] != 0 {
            return true;
        } else if piececonstants::get_bishop_attacks(square, self.occupancies[2])
            & (self.pieceboards[base + 2] | self.pieceboards[base + 4])
            != 0
        {
            return true;
        } else if piececonstants::get_rook_attacks(square, self.occupancies[2])
            & (self.pieceboards[base + 3] | self.pieceboards[base + 4])
            != 0
        {
            return true;
        } else {
            return false;
        }
    }
    pub fn is_king_attacked(&self) -> bool {
        let side = self.side.unwrap() == 1; // false for white, true for black
        let base = !side as usize * 6; // enemy base score
        let square = self.pieceboards[side as usize * 6 + 5].trailing_zeros() as usize;
        //self.print_board();

        return piececonstants::PAWN_ATTACKS[side as usize][square] & self.pieceboards[base] != 0
            || piececonstants::KNIGHT_ATTACKS[square] & self.pieceboards[base + 1] != 0
            || piececonstants::get_bishop_attacks(square, self.occupancies[2])
                & (self.pieceboards[base + 2] | self.pieceboards[base + 4])
                != 0
            || piececonstants::get_rook_attacks(square, self.occupancies[2])
                & (self.pieceboards[base + 3] | self.pieceboards[base + 4])
                != 0;
    }
    #[inline(always)]
    pub fn get_attacker(&self, square: usize) -> usize {
        // assumes that piece is of current side
        let squareboard = 1u64 << square;
        let raw_side = (self.side == Some(1)) as usize * 6;
        for i in 0..6 {
            if squareboard & self.pieceboards[i + raw_side] != 0 {
                return i + raw_side;
            }
        }
        // need this to sate the compilier, shouldnt ever happen
        panic!("No Attacker found for square {}", square);
    }
    #[inline(always)]
    pub fn get_target(&self, square: usize) -> Option<usize> {
        // assumes that piece is of current side
        let squareboard = 1u64 << square;
        let enemy_side = (self.side == Some(0)) as usize * 6;
        if squareboard & self.occupancies[2] == 0 {
            None
        }
        // target square is empty
        else {
            for i in 0..6 {
                if squareboard & self.pieceboards[i + enemy_side] != 0 {
                    return Some(i + enemy_side);
                }
            }

            panic!("Target square {} is not empty or enemy", square);
        }
    }
    pub fn print_attacks(&self) {
        let mut attackboard: u64 = 0;
        for rank in 0..8 {
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.is_square_attacked(square) {
                    set_bit!(attackboard, square);
                }
            }
        }
        crate::print_bitboard(attackboard);
    }
    pub fn print_board(&self) -> () {
        for rank in 0..8 {
            for file in 0..8 {
                if file == 0 {
                    print!("{}  ", 8 - rank);
                }
                // init board square, turn file and rank into square
                let square = rank * 8 + file;

                let mut piece: Option<usize> = None;
                for array in 0..12 {
                    if get_bit!(self.pieceboards[array], square) == 1 {
                        piece = Some(array);
                    }
                }
                match piece {
                    None => print!(". "),
                    Some(number) => print!("{} ", piececonstants::UNICODE_PIECES[number]),
                }
            }
            //print new line to seperate ranks
            println!();
        }
        println!("   a b c d e f g h");
        match self.side {
            Some(0) => println!("Side:        White"),
            Some(1) => println!("Side:        Black"),
            _ => println!("Side:         None"),
        }
        println!(
            "Enpassant:      {}",
            piececonstants::SQUARE_TO_COORDINATES[self.enpassant.trailing_zeros() as usize]
        );
        println!("Castling      {:04b}", self.castle);
        println!("Hash: {:#04X}", self.key);
    }
    pub fn parse_fen(&mut self, fen: String) {
        self.pieceboards = [0; 12]; // all boards to 0s
        self.occupancies = [0; 3]; // all occupancies to 0s
        self.side = None; // side no worky
        self.enpassant = 0; // enpassant possible square, no_sq
        self.castle = 0;
        self.key = 0;

        let mut square = 0;

        let split = fen.split(' ');
        let segments: Vec<&str> = split.collect();

        for c in segments[0].chars() {
            match c.is_numeric() {
                false => {
                    let piece = match c {
                        'P' => 0,
                        'N' => 1,
                        'B' => 2,
                        'R' => 3,
                        'Q' => 4,
                        'K' => 5,
                        'p' => 6,
                        'n' => 7,
                        'b' => 8,
                        'r' => 9,
                        'q' => 10,
                        'k' => 11,
                        '/' => continue,
                        _ => 12,
                    };
                    if piece != 12 {
                        set_bit!(self.pieceboards[piece], square);
                        set_bit!(self.occupancies[2], square);
                        self.key ^= piececonstants::PIECEKEYS[piece][square as usize];
                        if piece < 6 {
                            set_bit!(self.occupancies[0], square);
                        } else {
                            set_bit!(self.occupancies[1], square);
                        }
                    }
                    square += 1;
                }
                true => square += c as u32 - '0' as u32,
            };
        }
        //w KQkq - 0 1

        for c in segments[1].chars() {
            self.side = match c {
                'w' => Some(0),
                'b' => {
                    self.key ^= piececonstants::SIDEKEY;
                    Some(1)
                }
                _ => break,
            };
        }

        for c in segments[2].chars() {
            match c {
                'K' => self.castle += piececonstants::Castling::wk as u8,
                'Q' => self.castle += piececonstants::Castling::wq as u8,
                'k' => self.castle += piececonstants::Castling::bk as u8,
                'q' => self.castle += piececonstants::Castling::bq as u8,
                '-' => (),
                _ => break,
            }
        }
        self.key ^= piececonstants::CASTLEKEYS[self.castle as usize];

        if segments[3] != "-" {
            let chars: Vec<char> = segments[3].chars().collect();
            let file = chars[0] as usize - 'a' as usize;
            let rank = 8 - (chars[1] as usize - '0' as usize);
            self.key ^= piececonstants::EPKEY[file];
            self.enpassant = 1u64 << (rank * 8 + file);
        }

        //self.enpassant = piececonstants::Square::segments[3].unwrap()
    }
    pub fn evaluate(&mut self) -> i32 {
        let side = (self.side == Some(0)) as i32 * 2 - 1;

        //let mut score = 0;
        let mut midgame = 0;
        let mut endgame = 0;
        let mut phase = 0;
        for i in 0..6 {
            let mut wpieces = self.pieceboards[i];
            for _j in 0..wpieces.count_ones() {
                let square = wpieces.trailing_zeros() as usize;
                pop_bit!(wpieces, square);
                phase += piececonstants::PHASEWEIGHT[i];
                midgame += piececonstants::MIDGAMETABLE[i][square];
                endgame += piececonstants::ENDGAMETABLE[i][square];
            }
            let mut bpieces = self.pieceboards[i + 6];
            for _j in 0..bpieces.count_ones() {
                let square = bpieces.trailing_zeros() as usize;

                pop_bit!(bpieces, square);
                phase += piececonstants::PHASEWEIGHT[i];
                midgame -= piececonstants::MIDGAMETABLE[i][square ^ 56];
                endgame -= piececonstants::ENDGAMETABLE[i][square ^ 56];
            }
        }
        let factor = min(
            1,
            max(
                0,
                (phase - piececonstants::ENDGAME)
                    / (piececonstants::MIDGAME - piececonstants::ENDGAME),
            ),
        );
        return (factor * midgame + (1 - factor) * endgame) * 4 * side;
    }
}

pub fn perft_test(
    positionstack: &mut Vec<Board>,
    movestack: &mut Vec<Vec<u16>>,
    ply: &mut usize,
    depth: usize,
) -> u64 {
    if depth == 1 {
        return 1;
    }
    let mut count = 0;
    let index = movegen::generate_moves(&mut positionstack[*ply], &mut movestack[*ply]);

    for m in 0..index {
        let mut boardcopy = positionstack[*ply].clone();
        moves::make_move(&mut boardcopy, movestack[*ply][m]);
        *ply += 1;
        positionstack[*ply] = boardcopy;
        count += perft_test(positionstack, movestack, ply, depth - 1);
        *ply -= 1;
    }

    count
}

pub fn perft_div(
    positionstack: &mut Vec<Board>,
    movestack: &mut Vec<Vec<u16>>,
    ply: &mut usize,
    depth: usize,
) -> Vec<(u16, u64)> {
    let index = movegen::generate_moves(&mut positionstack[*ply], &mut movestack[*ply]);
    let mut count = vec![(0u16, 0u64); index];
    let mut i = 0;
    for m in 0..index {
        let mut boardcopy = positionstack[*ply].clone();
        moves::make_move(&mut boardcopy, movestack[*ply][m]);
        *ply += 1;
        positionstack[*ply] = boardcopy;
        count[i] = (
            movestack[*ply][m],
            perft_test(positionstack, movestack, ply, depth - 1),
        );
        *ply -= 1;
        i += 1;
    }
    count
}
