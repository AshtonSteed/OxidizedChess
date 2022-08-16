use crate::piececonstants;
use crate::pieceinit;

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
pub struct Board {
    pub pieceboards: [u64; 12], // [P, N, B, R, Q, K, p, n, b, r, q, k]
    pub occupancies: [u64; 3],  // [white occupancies, black occupancies, total occupancies]
    pub side: Option<u8>, // side to move, None for invalid, 0 for white, 1 for black (Color enum)
    pub enpassant: u64,   // enpassant possible square (as a bitboard)
    pub castle: u8, // castling rights, 4 bits, 0001 1  white king kingside, 0010 2 white king queenside, 0100 4 black king king side, 1000 8 black king queen side
}
impl Default for Board {
    fn default() -> Board {
        Board {
            pieceboards: [0; 12], // all boards to 0s
            occupancies: [0; 3],  // all occupancies to 0s
            side: None,           // side no worky
            enpassant: 0,         // enpassant possible square, no_sq
            castle: 0,
        }
    }
}
impl Board {
    pub fn generate_moves(&self) {
        let base = self.side.unwrap() as usize * 6;
        let mut bitboard: u64 = 0;
        let mut moves: Vec<Move> = Vec::new();
        for piece in base..base + 6 {
            bitboard = self.pieceboards[piece];
            match piece {
                0 => self.white_q_pawn_moves(&mut moves),
                6 => self.black_q_pawn_moves(&mut moves),
                _ => continue,
            }
            if piece == 6 {}
        }
    }
    fn white_q_pawn_moves(&self, moves: &mut Vec<Move>) {
        const rank4: u64 = 0x000000FF00000000;
        let mut pawn_targets = self.pieceboards[0] >> 8 & !self.occupancies[2];
        let mut double_push = pawn_targets >> 8 & rank4 & !self.occupancies[2];
        while pawn_targets != 0 {
            let target_square = pawn_targets.trailing_zeros(); //gets a bit from the board, removes it
            pop_bit!(pawn_targets, target_square);
            let source_square = target_square + 8;
            if target_square >= piececonstants::Square::A8 as u32
                && target_square <= piececonstants::Square::H8 as u32
            {
                //add 4 promotion moves
                moves.push(Move(write_move(
                    //knight promotion
                    source_square,
                    target_square,
                    false,
                    true,
                    false,
                    false,
                )));
                moves.push(Move(write_move(
                    // bishop promotion
                    source_square,
                    target_square,
                    false,
                    true,
                    false,
                    true,
                )));
                moves.push(Move(write_move(
                    //rook promotion
                    source_square,
                    target_square,
                    false,
                    true,
                    true,
                    false,
                )));
                moves.push(Move(write_move(
                    //queen promotion
                    source_square,
                    target_square,
                    false,
                    true,
                    true,
                    true,
                )));
            } else {
                // add one square quiet move
                moves.push(Move(write_move(
                    source_square,
                    target_square,
                    false,
                    false,
                    false,
                    false,
                ))); // normal 1 forward pawn move
            }
        }

        while double_push != 0 {
            let target_square = double_push.trailing_zeros(); //gets a bit from the board, removes it
            pop_bit!(double_push, target_square);
            let source_square = target_square + 16;
            moves.push(Move(write_move(
                source_square,
                target_square,
                false,
                false,
                false,
                true,
            )))
        }
    }

    fn black_q_pawn_moves(&self, moves: &mut Vec<Move>) {
        // black pawn quiet moves
        const rank5: u64 = 0x00000000FF000000;
        let mut pawn_targets = self.pieceboards[6] << 8 & !self.occupancies[2];
        let mut double_push = pawn_targets << 8 & rank5 & !self.occupancies[2];
        while pawn_targets != 0 {
            let target_square = pawn_targets.trailing_zeros(); //gets a bit from the board, removes it
            pop_bit!(pawn_targets, target_square);
            let source_square = target_square - 8;
            if target_square >= piececonstants::Square::A1 as u32
                && target_square <= piececonstants::Square::H1 as u32
            {
                //add 4 promotion moves
                moves.push(Move(write_move(
                    //knight promotion
                    source_square,
                    target_square,
                    false,
                    true,
                    false,
                    false,
                )));
                moves.push(Move(write_move(
                    // bishop promotion
                    source_square,
                    target_square,
                    false,
                    true,
                    false,
                    true,
                )));
                moves.push(Move(write_move(
                    //rook promotion
                    source_square,
                    target_square,
                    false,
                    true,
                    true,
                    false,
                )));
                moves.push(Move(write_move(
                    //queen promotion
                    source_square,
                    target_square,
                    false,
                    true,
                    true,
                    true,
                )));
            } else {
                // add one square quiet move
                moves.push(Move(write_move(
                    source_square,
                    target_square,
                    false,
                    false,
                    false,
                    false,
                ))); // normal 1 forward pawn move
            }
        }
        while double_push != 0 {
            let target_square = double_push.trailing_zeros(); //gets a bit from the board, removes it
            pop_bit!(double_push, target_square);
            let source_square = target_square - 16;
            moves.push(Move(write_move(
                source_square,
                target_square,
                false,
                false,
                false,
                true,
            )))
        }
    }

    pub fn is_square_attacked(&self, square: usize) -> bool {
        // returns true if square is attacked by current side
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
    }
    pub fn parse_fen(&mut self, fen: &str) {
        self.pieceboards = [0; 12]; // all boards to 0s
        self.occupancies = [0; 3]; // all occupancies to 0s
        self.side = None; // side no worky
        self.enpassant = 0; // enpassant possible square, no_sq
        self.castle = 0;

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
                'b' => Some(1),
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

        if segments[3] != "-" {
            let chars: Vec<char> = segments[3].chars().collect();
            let file = chars[0] as usize - 'a' as usize;
            let rank = 8 - (chars[1] as usize - '0' as usize);
            self.enpassant = 0;
            set_bit!(self.enpassant, rank * 8 + file);
        }

        //self.enpassant = piececonstants::Square::segments[3].unwrap()
    }
}

// 0000 0000 0011 1111 initial square
// 0000 1111 1100 0000 final square
// 1111 0000 0000 0000 special flags, [promotion, capture, special1, special2]
// 0000 quiet move
// 0001 double pawn push
// 0010 kingside castle
// 0011 queenside castle
// 0100 capture moves
// 0101 enpassant capture
// 1000 knight promotion
// 1001 bishop promotion
// 1010 rook promotion
// 1011 queen promotion
// 1100 - 1111 same as promotions but captures
pub struct Move(u16);
pub fn write_move(
    initial_square: u32,
    target_square: u32,
    capture: bool,
    promotion: bool,
    special1: bool,
    special2: bool,
) -> u16 {
    println!(
        "{} to {}",
        piececonstants::SQUARE_TO_COORDINATES[initial_square as usize],
        piececonstants::SQUARE_TO_COORDINATES[target_square as usize]
    );
    0
}
