use crate::piececonstants;
use crate::pieceinit;

lazy_static! { //allows me to use this stuff as statics, neat
    static ref SLIDER_ATTACKS: Vec<u64>= pieceinit::init_slider_attacks2().0;
}

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
    pub enpassant: usize, // enpassant possible square
    pub castle: u8, // castling rights, 4 bits, 0001 1  white king kingside, 0010 2 white king queenside, 0100 4 black king king side, 1000 8 black king queen side
}
impl Default for Board {
    fn default() -> Board {
        Board {
            pieceboards: [0; 12],                             // all boards to 0s
            occupancies: [0; 3],                              // all occupancies to 0s
            side: None,                                       // side no worky
            enpassant: piececonstants::Square::NOSQ as usize, // enpassant possible square, no_sq
            castle: 0,
        }
    }
}
impl Board {
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
        } else if get_bishop_attacks(square, self.occupancies[2])
            & (self.pieceboards[base + 2] | self.pieceboards[base + 4])
            != 0
        {
            return true;
        } else if get_rook_attacks(square, self.occupancies[2])
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
            piececonstants::SQUARE_TO_COORDINATES[self.enpassant]
        );
        println!("Castling      {:04b}", self.castle);
    }
    pub fn parse_fen(&mut self, fen: &str) {
        self.pieceboards = [0; 12]; // all boards to 0s
        self.occupancies = [0; 3]; // all occupancies to 0s
        self.side = None; // side no worky
        self.enpassant = piececonstants::Square::NOSQ as usize; // enpassant possible square, no_sq
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
            self.enpassant = rank * 8 + file;
        }

        //self.enpassant = piececonstants::Square::segments[3].unwrap()
    }
}
//                                      bit manipulations

pub fn get_bishop_attacks(square: usize, mut occupancy: u64) -> u64 {
    occupancy &= piececonstants::BISHOP_MASKS[square];
    occupancy = occupancy.wrapping_mul(piececonstants::BISHOPMAGICNUMBERS[square]);
    occupancy >>= 64 - piececonstants::BISHOPBITS[square];

    SLIDER_ATTACKS[piececonstants::BISHOP_POINTERS[square] + occupancy as usize]
}

pub fn get_rook_attacks(square: usize, mut occupancy: u64) -> u64 {
    occupancy &= piececonstants::ROOK_MASKS[square];
    occupancy = occupancy.wrapping_mul(piececonstants::ROOKMAGICNUMBERS[square]);
    occupancy >>= 64 - piececonstants::ROOKBITS[square];

    SLIDER_ATTACKS[piececonstants::ROOK_POINTERS[square] + occupancy as usize]
}

pub fn get_queen_attacks(square: usize, occupancy: u64) -> u64 {
    let mut bishop_occupancy = occupancy;

    let mut rook_occupancy = occupancy;

    bishop_occupancy &= piececonstants::BISHOP_MASKS[square];
    bishop_occupancy = bishop_occupancy.wrapping_mul(piececonstants::BISHOPMAGICNUMBERS[square]);
    bishop_occupancy >>= 64 - piececonstants::BISHOPBITS[square];

    rook_occupancy &= piececonstants::ROOK_MASKS[square];
    rook_occupancy = rook_occupancy.wrapping_mul(piececonstants::ROOKMAGICNUMBERS[square]);
    rook_occupancy >>= 64 - piececonstants::ROOKBITS[square];

    SLIDER_ATTACKS[piececonstants::BISHOP_POINTERS[square] + bishop_occupancy as usize]
        | SLIDER_ATTACKS[piececonstants::ROOK_POINTERS[square] + rook_occupancy as usize]
    // returns rook + bishop attacks from square
}
