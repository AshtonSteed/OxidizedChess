// 0000 0000 0011 1111 initial square (0x3F)
// 0000 1111 1100 0000 final square (0xFC0)
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

//pub struct Move(u16);
use crate::{engine, piececonstants};

pub use u16 as Move;

pub trait MoveStuff {
    fn print(&self, full: bool);
    fn new(
        initial_square: u16,
        target_square: u16,
        capture: u16,
        promotion: u16,
        special1: u16,
        special2: u16,
    ) -> Move;
    fn get_initial(&self) -> u16;
    fn get_final(&self) -> u16;
    fn get_extra(&self) -> u16;
    fn to_uci(&self) -> String;
}

impl MoveStuff for Move {
    #[inline(always)]
    fn new(
        initial_square: u16,
        target_square: u16,
        capture: u16,
        promotion: u16,
        special1: u16,
        special2: u16,
    ) -> Move // special list: 00 n, 01 b, 10 r, 11 q
    {
        return initial_square
            | (target_square << 6)
            | (special2 << 12)
            | (special1 << 13)
            | (capture << 14)
            | (promotion << 15);
    }
    #[inline(always)]
    fn get_initial(&self) -> u16 {
        self & 0x3F
    }
    #[inline(always)]
    fn get_final(&self) -> u16 {
        (self & 0xFC0) >> 6
    }
    #[inline(always)]
    fn get_extra(&self) -> u16 {
        (self & 0xF000) >> 12
    }

    fn print(&self, full: bool) {
        let initial_square = self.get_initial();
        let final_square = self.get_final();
        let capture = (self & 0x4000) >> 14;
        let promotion = (self & 0x8000) >> 15;
        let special1 = (self & 0x2000) >> 13;
        let special2 = (self & 0x1000) >> 12;

        println!(
            "{}{}",
            piececonstants::SQUARE_TO_COORDINATES[initial_square as usize],
            piececonstants::SQUARE_TO_COORDINATES[final_square as usize]
        );
        if full {
            println!(
                "Is captue: {}    Is Promotion {}    Special1 and 2: {}{}",
                capture, promotion, special1, special2
            );
            println!("{:#018b}", self);
        }
    }
    fn to_uci(&self) -> String {
        let initial_square = piececonstants::SQUARE_TO_COORDINATES[self.get_initial() as usize];
        let final_square = piececonstants::SQUARE_TO_COORDINATES[self.get_final() as usize];
        if self & 0x8000 != 0 {
            return (initial_square.to_owned()
                + final_square
                + piececonstants::ASCII_PIECES[((3 & self.get_extra()) + 7) as usize])
                .to_string();
        } else {
            return (initial_square.to_owned() + final_square).to_string();
        }
    }
}

pub fn make_move(board: &mut crate::engine::Board, movement: &Move) {
    let initial_square = movement.get_initial() as usize;
    let final_square = movement.get_final() as usize;
    let extra = movement.get_extra();
    let capture = extra & 4 != 0;
    let promotion = extra & 8 != 0;
    //STRUCTURE :
    // CAPTURE -> Normal Captures, Enpassant, Promotion
    // QUIET -> Normal Moves, Push, Promotion, Castle

    // first, stuff that happens for every single move
    let raw_side = (board.side == Some(1)) as usize;
    let enemy_raw = (board.side == Some(0)) as usize;

    let attacker = board.get_attacker(initial_square);
    //let targetoption = board.get_target(final_square);
    let notattackboard = !(1u64 << initial_square);
    let endboard = 1u64 << final_square;

    //removes attacker from position and moves to next position
    board.pieceboards[attacker] &= notattackboard;
    board.occupancies[raw_side] &= notattackboard;

    board.pieceboards[attacker] |= endboard;
    board.occupancies[raw_side] |= endboard;
    board.key ^= piececonstants::PIECEKEYS[attacker][initial_square]
        ^ piececonstants::PIECEKEYS[attacker][final_square];
    // clears EP
    if board.enpassant != 0 {
        board.key ^= piececonstants::EPKEY[(board.enpassant.trailing_zeros()) as usize % 8];
    }

    board.enpassant = 0;

    //updates castling rights
    board.key ^= piececonstants::CASTLEKEYS[board.castle as usize];
    board.castle &= piececonstants::CASTLING_RIGHTS[initial_square];
    board.castle &= piececonstants::CASTLING_RIGHTS[final_square];
    board.key ^= piececonstants::CASTLEKEYS[board.castle as usize];
    // if move is capture, remove from pieceboards, and handle EP
    if capture {
        match board.get_target(final_square) {
            Some(target) => {
                board.pieceboards[target] &= !endboard;
                board.occupancies[enemy_raw] &= !endboard;
                board.key ^= piececonstants::PIECEKEYS[target][final_square]
            }
            // could handle enpassant here?
            None => {
                let ep_target_square = final_square + 8 - raw_side * 16;
                let ep_board = !(1u64 << ep_target_square);

                //clears EP taken pawn
                board.pieceboards[enemy_raw * 6] &= ep_board;
                board.occupancies[enemy_raw] &= ep_board;
                board.key ^= piececonstants::PIECEKEYS[enemy_raw * 6][ep_target_square];
            }
        }
    } else {
        // ONE OF THE MATCH STATEMENTS OF ALL TIME?????
        // takes care of castles and push
        match extra {
            0b0001 => {
                board.enpassant = 1u64 << (final_square + 8 - 16 * raw_side);
                board.key ^= piececonstants::EPKEY[final_square % 8];
            } //push
            0b0010 => {
                let rook = raw_side * 6 + 3;
                let notrookstart = !(1u64 << (initial_square + 3));
                let rookend = 1u64 << (initial_square + 1);
                board.pieceboards[rook] &= notrookstart;
                board.occupancies[raw_side] &= notrookstart;

                board.pieceboards[rook] |= rookend;
                board.occupancies[raw_side] |= rookend;
                board.key ^= piececonstants::PIECEKEYS[rook][initial_square + 3]
                    ^ piececonstants::PIECEKEYS[rook][initial_square + 1];
            } //castle kingside
            0b0011 => {
                let rook = raw_side * 6 + 3;
                let notrookstart = !(1u64 << (initial_square - 4));
                let rookend = 1u64 << (initial_square - 1);
                board.pieceboards[rook] &= notrookstart;
                board.occupancies[raw_side] &= notrookstart;

                board.pieceboards[rook] |= rookend;
                board.occupancies[raw_side] |= rookend;
                board.key ^= piececonstants::PIECEKEYS[rook][initial_square - 4]
                    ^ piececonstants::PIECEKEYS[rook][initial_square - 1];
            } //castle queenside

            _ => {} //panic!("Invalid Move Decoding"),
        };
    }
    // handles promotions after captures n stuff
    if promotion {
        let piece = ((3 & extra) + 1) as usize + raw_side * 6;

        board.pieceboards[attacker] &= !endboard; // remove promoting pawn

        board.pieceboards[piece] |= endboard; // adds promoted piece

        board.key ^= piececonstants::PIECEKEYS[attacker][final_square]
            ^ piececonstants::PIECEKEYS[piece][final_square];
    }

    // changes player up
    board.side = match board.side {
        Some(0) => Some(1),
        Some(1) => Some(0),
        _ => None,
    };
    board.key ^= piececonstants::SIDEKEY;
    // sets total occupancies
    board.occupancies[2] = board.occupancies[1] | board.occupancies[0];
}

pub fn null_move(board: &mut engine::Board) {
    // flips side to move
    board.side = match board.side {
        Some(0) => Some(1),
        Some(1) => Some(0),
        _ => None,
    };
    board.key ^= piececonstants::SIDEKEY;
    // clears EP
    if board.enpassant != 0 {
        board.key ^= piececonstants::EPKEY[(board.enpassant.trailing_zeros()) as usize % 8];
    }

    board.enpassant = 0;
}
// returns -score of move, lower the better for sorting purposes
// we need to make this way better for PVS
#[inline(always)]
pub fn score_move(
    m: &u16,
    board: &engine::Board,
    tmove: Option<u16>,
    killers: &Vec<u16>,
    history: &Vec<Vec<usize>>,
    maxh: usize,
) -> i32 {
    // PV / ttable move
    if Some(*m) == tmove {
        //println!("PV");
        return 1000;
    }

    let initsq = m.get_initial() as usize;
    let attacker = board.get_attacker(initsq);
    let finalsq = m.get_final() as usize;
    let extra = m.get_extra();

    // captures
    if extra & 4 != 0 {
        match board.get_target(finalsq) {
            Some(target) => return piececonstants::MVV_LVA[attacker][target],
            // enpassant, pawn takes pawn but a bit more interesting
            None => return 120,
        }
    } else {
        if *m == killers[0] || *m == killers[1] {
            return 100;
        }

        //promotion and then castling
        if extra & 8 != 0 {
            return piececonstants::PIECEWEIGHT[extra as usize & 3 + 1] / 10;
        } else if extra & 2 != 0 {
            return 95;
        }

        let hval = 1 + 90 * history[initsq][finalsq] / maxh;

        return hval as i32;
    }
    //println!("Move: {} Score: {}", m.to_uci(), score); 3712990
}
#[inline]
fn shift_down(moves: &mut [u16], scores: &mut [i32], start: usize, end: usize) {
    let mut root = start;

    loop {
        let mut child = root * 2 + 1;
        if child > end {
            break;
        }
        if child + 1 <= end && &scores[child] < &scores[child + 1] {
            child += 1;
        }
        if &scores[root] < &scores[child] {
            scores.swap(root, child);
            moves.swap(root, child);
            root = child
        } else {
            break;
        }
    }
}
#[inline]
pub fn build_min_heap(moves: &mut [u16], scores: &mut [i32], size: usize) {
    // Build a min heap.
    for i in (0..size / 2).rev() {
        shift_down(moves, scores, i, size - 1);
    }
}

#[inline]
pub fn next_move(moves: &mut [u16], scores: &mut [i32], size: usize, i: usize) {
    let end = size - i - 1;

    // Extract elements one by one
    if end == 0 {
        return;
    }

    scores.swap(0, end); // Swap
    moves.swap(0, end);

    shift_down(moves, scores, 0, end - 1);
    /*
    let mut max = 0;
    let mut k = 0;
    for i in 0..(size - i) {
        if scores[i] > max {
            k = i;
            max = scores[i];
        }
    }
    scores.swap(k, end);
    moves.swap(k, end);*/
}
