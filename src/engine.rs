use std::mem::swap;

use crate::{
    movegen,
    moves::{self, MoveStuff},
    piececonstants::{self, draw_score},
};

#[derive(Copy, Clone)]
pub struct Board {
    pub pieceboards: [u64; 12], // [P, N, B, R, Q, K, p, n, b, r, q, k]
    pub occupancies: [u64; 3],  // [white occupancies, black occupancies, total occupancies]
    pub movemasks: [u64; 5],    // [attacks, checkmask, rookpin, bishoppin, kingraw]
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
            movemasks: [0; 5],    // all masks to 0s
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

        let attacked = (piececonstants::PAWN_ATTACKS[!side as usize][square]
            & self.pieceboards[base])
            | (piececonstants::KNIGHT_ATTACKS[square] & self.pieceboards[base + 1])
            | (piececonstants::KING_ATTACKS[square] & self.pieceboards[base + 5])
            | (piececonstants::get_bishop_attacks(square, self.occupancies[2])
                & (self.pieceboards[base + 2] | self.pieceboards[base + 4]))
            | (piececonstants::get_rook_attacks(square, self.occupancies[2])
                & (self.pieceboards[base + 3] | self.pieceboards[base + 4]));
        return attacked != 0;
    }
    pub fn is_repeat(&self, next: &Board) -> bool {
        let mut differences = 0;
        for i in 0..12 {
            if self.pieceboards[i] != next.pieceboards[i] {
                if i == 0 || i == 6 {
                    return false;
                }
                differences += 1;
            }
        }
        return differences < 2;
    }

    pub fn is_king_attacked(&self) -> bool {
        let side: bool = self.side.unwrap() == 0; // true for white, false for black
        let selfside = !side;
        let base = side as usize * 6; // enemy base score
        let square = self.pieceboards[selfside as usize * 6 + 5].trailing_zeros() as usize;

        assert!(square != 64, "No King On Board! {}", {
            self.print_board();
            "Board:"
        });
        //self.print_board();

        let attacked = (piececonstants::PAWN_ATTACKS[!side as usize][square]
            & self.pieceboards[base])
            | (piececonstants::KNIGHT_ATTACKS[square] & self.pieceboards[base + 1])
            | (piececonstants::get_bishop_attacks(square, self.occupancies[2])
                & (self.pieceboards[base + 2] | self.pieceboards[base + 4]))
            | (piececonstants::get_rook_attacks(square, self.occupancies[2])
                & (self.pieceboards[base + 3] | self.pieceboards[base + 4]));
        return attacked != 0;
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
        self.print_board();

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

    #[inline(always)]
    pub fn piece_on(&self, square: usize) -> usize {
        // assumes that piece is of current side
        let squareboard = 1u64 << square;

        for i in 0..6 {
            if squareboard & (self.pieceboards[i + 6] | self.pieceboards[i]) != 0 {
                return i;
            }
        }
        // need this to sate the compilier, shouldnt ever happen
        panic!("No Piece found for square {}", square);
    }
    pub fn print_attacks(&self) {
        let mut attackboard: u64 = 0;
        for rank in 0..8 {
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.is_square_attacked(square) {
                    attackboard.set_bit(square);
                }
            }
        }
        attackboard.print_bitboard()
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
                    if self.pieceboards[array].get_bit(square) == 1 {
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
                        self.pieceboards[piece].set_bit(square);
                        self.occupancies[2].set_bit(square);
                        self.key ^= piececonstants::PIECEKEYS[piece][square as usize];
                        if piece < 6 {
                            self.occupancies[0].set_bit(square);
                        } else {
                            self.occupancies[1].set_bit(square);
                        }
                    }
                    square += 1;
                }
                true => square += c as usize - '0' as usize,
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
                'K' => self.castle += piececonstants::Castling::Wk as u8,
                'Q' => self.castle += piececonstants::Castling::Wq as u8,
                'k' => self.castle += piececonstants::Castling::Bk as u8,
                'q' => self.castle += piececonstants::Castling::Bq as u8,
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
    #[inline(always)]
    pub fn evaluate(&mut self, ply: usize) -> i32 {
        let side = (self.side == Some(0)) as i32 * 2 - 1;

        // check for insufficient material draw
        // KvK, KMvK, KBvKB(same color)
        if self.occupancies[2].count_ones() <= 4 {
            let kings = self.pieceboards[5] | self.pieceboards[11];
            let bn = self.pieceboards[1]
                | self.pieceboards[2]
                | self.pieceboards[7]
                | self.pieceboards[8];

            // Draws if just kings, if one side has one minor piece, or if there are 2 minor pieces on one side. This is not true, but works for now.
            if self.occupancies[2] == kings | bn
            /*|| {
                self.occupancies[0].count_ones() == 2
                    && (self.occupancies[2]
                        == kings | self.pieceboards[2] | self.pieceboards[8])
            }*/
            {
                return draw_score(ply);
            }
        }

        //let mut score = 0;
        let mut midgame = 0;
        let mut endgame = 0;
        let mut phase = 0;
        let mut mobility = 0;
        let mut virtual_mobility = 0;

        // Use of helper boards to find isolated, passed, and double pawns

        let mut wiso = 0u64;
        let mut wspan = 0u64;

        let mut biso = 0u64;
        let mut bspan = 0u64;

        // Most pieces
        for i in 0..6 {
            let mut wpieces = self.pieceboards[i];
            for _j in 0..wpieces.count_ones() {
                let square = wpieces.trailing_zeros() as usize;
                wpieces.pop_bit(square);
                phase += piececonstants::PHASEWEIGHT[i];
                midgame += piececonstants::MIDGAMETABLE[i][square];
                endgame += piececonstants::ENDGAMETABLE[i][square];
                match i {
                    0 => {
                        wiso |= piececonstants::CLOSEFILES[square & 7];
                        wspan |= piececonstants::PAWNSPANS[0][square];
                        mobility += (piececonstants::PAWN_ATTACKS[0][square] & self.occupancies[1])
                            .count_ones() as i32;
                    }
                    1 => {
                        mobility += (piececonstants::KNIGHT_ATTACKS[square] & !self.occupancies[0])
                            .count_ones() as i32;
                    }
                    2 => {
                        mobility +=
                            (piececonstants::get_bishop_attacks(square, self.occupancies[2])
                                & !self.occupancies[0])
                                .count_ones() as i32;
                    }
                    3 => {
                        mobility += (piececonstants::get_rook_attacks(square, self.occupancies[2])
                            & !self.occupancies[0])
                            .count_ones() as i32;
                    }
                    4 => {
                        mobility += (piececonstants::get_queen_attacks(square, self.occupancies[2])
                            & !self.occupancies[0])
                            .count_ones() as i32;
                    }
                    5 => {
                        mobility += (piececonstants::KING_ATTACKS[square] & !self.occupancies[0])
                            .count_ones() as i32;
                        virtual_mobility +=
                            (piececonstants::get_queen_attacks(square, self.occupancies[2])
                                & !self.occupancies[0])
                                .count_ones() as i32;
                    }
                    _ => {}
                }
            }
            let mut bpieces = self.pieceboards[i + 6];
            for _j in 0..bpieces.count_ones() {
                let square = bpieces.trailing_zeros() as usize;

                bpieces.pop_bit(square);
                phase += piececonstants::PHASEWEIGHT[i];
                midgame -= piececonstants::MIDGAMETABLE[i][square ^ 56];
                endgame -= piececonstants::ENDGAMETABLE[i][square ^ 56];

                match i {
                    0 => {
                        biso |= piececonstants::CLOSEFILES[square & 7];
                        bspan |= piececonstants::PAWNSPANS[1][square];
                        mobility -= (piececonstants::PAWN_ATTACKS[1][square] & self.occupancies[0])
                            .count_ones() as i32;
                    }
                    1 => {
                        mobility -= (piececonstants::KNIGHT_ATTACKS[square] & !self.occupancies[1])
                            .count_ones() as i32;
                    }
                    2 => {
                        mobility -=
                            (piececonstants::get_bishop_attacks(square, self.occupancies[2])
                                & !self.occupancies[1])
                                .count_ones() as i32;
                    }
                    3 => {
                        mobility -= (piececonstants::get_rook_attacks(square, self.occupancies[2])
                            & !self.occupancies[1])
                            .count_ones() as i32;
                    }
                    4 => {
                        mobility -= (piececonstants::get_queen_attacks(square, self.occupancies[2])
                            & !self.occupancies[1])
                            .count_ones() as i32;
                    }
                    5 => {
                        mobility -= (piececonstants::KING_ATTACKS[square] & !self.occupancies[1])
                            .count_ones() as i32;
                        virtual_mobility -=
                            (piececonstants::get_queen_attacks(square, self.occupancies[2])
                                & !self.occupancies[1])
                                .count_ones() as i32;
                    }
                    _ => {}
                }
            }
        }
        // Pawn structure summations, W is positive7
        let doublepawns = ((wspan & self.pieceboards[0]).count_ones() as i32
            - (self.pieceboards[6] & bspan).count_ones() as i32);
        let passedpawns = (!(bspan | bspan.left() | bspan.right()) & self.pieceboards[0])
            .count_ones() as i32
            - (!(wspan | wspan.left() | wspan.right()) & self.pieceboards[6]).count_ones() as i32;
        let isopawns = (self.pieceboards[0] & !wiso).count_ones() as i32
            - (self.pieceboards[6] & !biso).count_ones() as i32;

        //println!("{} {} {}", doublepawns, passedpawns, isopawns);
        midgame += passedpawns * piececonstants::PAWN_STRUCTURE_VALUES[0][0]
            + isopawns * piececonstants::PAWN_STRUCTURE_VALUES[0][1]
            + doublepawns * piececonstants::PAWN_STRUCTURE_VALUES[0][2];
        endgame += passedpawns * piececonstants::PAWN_STRUCTURE_VALUES[1][0]
            + isopawns * piececonstants::PAWN_STRUCTURE_VALUES[1][1]
            + doublepawns * piececonstants::PAWN_STRUCTURE_VALUES[1][2];

        //                            Mobility

        // finish pawn forward move mobility
        mobility += ((self.pieceboards[0] << 8) & !self.occupancies[2]).count_ones() as i32
            - ((self.pieceboards[6] >> 8) & !self.occupancies[2]).count_ones() as i32;

        midgame += mobility / piececonstants::MOBILITY_SCALE;
        endgame += (mobility + virtual_mobility) / piececonstants::MOBILITY_SCALE;

        //println!("{} {}", mobility, virtual_mobility);

        let factor = 1.0_f64.min(0.0_f64.max(
            (phase as f64 - piececonstants::ENDGAME)
                / (piececonstants::MIDGAME - piececonstants::ENDGAME),
        ));

        //let factor = 1.0 / (1.0 + (-0.002 * phase as f64 + 5.6).exp());

        return (factor * midgame as f64 + (1. - factor) * endgame as f64) as i32 * 4 * side;
    }

    // Statically evaluates a move and decide if it is better than a given threshold. This should allow for cheap seperation of bad moves.
    // True means that the capture is good according to the threshold (capture is better than the minimum of the threshold)
    pub fn see(&self, m: &moves::Move, threshold: i32) -> bool {
        if m.get_extra() != 4 {
            // For non-normal captures, return if it is a good capture initially
            return 0 >= threshold;
        }

        let from = m.get_initial() as usize;
        let to = m.get_final() as usize;

        let mut swap = piececonstants::PIECEWEIGHT[self.piece_on(to)] - threshold;

        if swap < 0 {
            // If material gained is strictly bad, return a bad capture
            return false;
        }

        swap = piececonstants::PIECEWEIGHT[self.piece_on(from)] - swap;

        if swap <= 0 {
            // If the capture is still bad with the threshold, assume it is bad
            return true;
        }

        let mut side = self.side.unwrap_or_default() as usize;

        // gather all pieces that might lead to discovered xrays
        //let notknights = !(self.pieceboards[1] | self.pieceboards[7]);

        let mut occupancy = self.occupancies[2] ^ (1 << from) ^ (1 << to);

        let mut res = 1;

        let mut stmattackers;

        let mut bb;
        let mut bigside;

        // Get a bitboard of all pieces currently attacking the attacked square
        let mut attadef = (piececonstants::PAWN_ATTACKS[0][to] & self.pieceboards[6])
            | (piececonstants::PAWN_ATTACKS[1][to] & self.pieceboards[0])
            | (piececonstants::KNIGHT_ATTACKS[to] & (self.pieceboards[1] | self.pieceboards[7]))
            | (piececonstants::get_bishop_attacks(to, occupancy)
                & (self.pieceboards[2]
                    | self.pieceboards[4]
                    | self.pieceboards[8]
                    | self.pieceboards[10]))
            | (piececonstants::get_rook_attacks(to, occupancy)
                & (self.pieceboards[3]
                    | self.pieceboards[4]
                    | self.pieceboards[9]
                    | self.pieceboards[10]))
            | (piececonstants::KING_ATTACKS[to] & (self.pieceboards[6] | self.pieceboards[11]));

        'outer: loop {
            side = side ^ 1;

            bigside = side * 6;

            attadef &= occupancy;

            stmattackers = attadef & self.occupancies[side];
            if stmattackers == 0 {
                // attacking side has no more attackers
                break;
            }

            res ^= 1;

            // Find next least valuable attacker
            for i in 0..6 {
                bb = stmattackers & (self.pieceboards[bigside + i]);
                if i == 6 {
                    if attadef & !self.occupancies[side] != 0 {
                        return (res ^ 1) == 1;
                    } else {
                        return res == 1;
                    }
                } else if bb != 0 {
                    swap = piececonstants::PIECEWEIGHT[i] - swap;
                    if swap < res {
                        break 'outer;
                    }

                    occupancy.pop_bit(bb.trailing_zeros() as usize);

                    // Check for discovered attacks
                    match i {
                        0 | 2 => {
                            attadef |= piececonstants::get_bishop_attacks(to, occupancy)
                                & (self.pieceboards[bigside + 2] | self.pieceboards[bigside + 4])
                        }
                        3 => {
                            attadef |= piececonstants::get_rook_attacks(to, occupancy)
                                & (self.pieceboards[bigside + 3] | self.pieceboards[bigside + 4])
                        }
                        4 => {
                            attadef |= (piececonstants::get_bishop_attacks(to, occupancy)
                                & (self.pieceboards[bigside + 2] | self.pieceboards[bigside + 4]))
                                | piececonstants::get_rook_attacks(to, occupancy)
                                    & (self.pieceboards[bigside + 3]
                                        | self.pieceboards[bigside + 4])
                        }
                        _ => {}
                    }
                    break;
                }
            }
        }
        //println!("{}", swap);

        res == 1
    }
    pub fn hash(&self) -> usize {
        return self.key as usize & (piececonstants::TTABLEMASK as usize);
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
        moves::make_move(&mut boardcopy, &movestack[*ply][m]);
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
        moves::make_move(&mut boardcopy, &movestack[*ply][m]);
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
pub trait BitBoard {
    fn left(&self) -> u64;
    fn right(&self) -> u64;
    fn get_bit(&self, index: usize) -> usize;
    fn print_bitboard(&self);
    fn set_bit(&mut self, square: usize);
    fn pop_bit(&mut self, square: usize);
}

impl BitBoard for u64 {
    #[inline(always)]
    fn left(&self) -> u64 {
        return (self >> 1) & 9187201950435737471;
    }
    #[inline(always)]
    fn right(&self) -> u64 {
        return (self << 1) & 18374403900871474942;
    }
    fn print_bitboard(&self) -> () {
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
                print!("{} ", self.get_bit(square));
            }
            //print new line to seperate ranks
            println!();
        }
        println!("   a b c d e f g h");

        println!("Bitboard Value: {}", self)
    }
    #[inline(always)]
    fn get_bit(&self, square: usize) -> usize {
        if (*self & 1u64 << square) != 0 {
            1
        } else {
            0
        } // if statement checks if the and operator
          // between bitboard and square is non zero, checks and returns square bit value
    }
    #[inline(always)]
    fn set_bit(&mut self, square: usize) {
        *self |= 1u64 << square;
    }
    #[inline(always)]
    fn pop_bit(&mut self, square: usize) {
        *self &= !(1u64 << square);
    }
}

pub fn is_draw(positionstack: &[Board], ply: usize, halfcount: usize) -> bool {
    // 50 move rule
    if halfcount >= 50 {
        true;
    }

    // Check for double repitition in search
    let depth = ply.min(halfcount);
    let now = positionstack[ply].key;
    //let mut n = 0;
    for i in ply - depth..ply {
        if positionstack[i].key == now {
            return true;
        }
    }
    // else, no repitiion
    return false;
}
