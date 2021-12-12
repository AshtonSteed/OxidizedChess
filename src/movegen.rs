use crate::piececonstants;
use crate::pieceinit;
use crate::print_bitboard;

pub fn refresh(
    // this monstrosity sets pinmasks and checkmasks, allowing for full move generation without needing to individually check move legality after the fact
    board: &crate::engine::Board,
    kingban: &mut u64,
    checkmask: &mut u64,
    rook_pin: &mut u64,
    bishop_pin: &mut u64,
) {
    *checkmask = u64::MAX; // sets checkmask to all 1s
    *rook_pin = 0;
    *bishop_pin = 0;
    let raw_side = (board.side == Some(1)) as usize;
    let enemy_raw = (board.side == Some(0)) as usize;
    let side = 6 * raw_side;
    let enemy = 6 * enemy_raw; // acts opposite of side to move
    let kingboard = board.pieceboards[5 + side];
    //crate::print_bitboard(kingboard);
    let kingsq = kingboard.trailing_zeros() as usize;
    if kingsq == 64 {
        // maybe remove this and just require a king on board
        return;
    }
    let pawn_attacks = piececonstants::PAWN_ATTACKS[raw_side][kingsq] & board.pieceboards[enemy]; // updates checkmask to include a single psosible pawn attack
    if pawn_attacks != 0 {
        *checkmask = pawn_attacks;
    }

    let knight_attacks = piececonstants::KNIGHT_ATTACKS[kingsq] & board.pieceboards[enemy + 1]; // updates checkmask to include a single knight attack
    if knight_attacks != 0 {
        *checkmask = knight_attacks;
    }

    if piececonstants::ROOK_RAW_ATTACKS[kingsq] // checks if there is a rook or queen xraying the king
        & (board.pieceboards[4 + enemy] | board.pieceboards[3 + enemy])
        != 0
    {
        let mut atk_hv = piececonstants::get_rook_attacks(kingsq, board.occupancies[2])
            & (board.pieceboards[4 + enemy] | board.pieceboards[3 + enemy]); // bitboard of all rooks/queens that see the king

        for _i in 0..atk_hv.count_ones() {
            // for each rook/queen, update the checkmask
            let square = atk_hv.trailing_zeros() as usize;
            pop_bit!(atk_hv, square);
            check_by_slider(kingsq, square, checkmask);
        }

        atk_hv = piececonstants::ROOK_RAW_ATTACKS[kingsq]
            & (board.pieceboards[4 + enemy] | board.pieceboards[3 + enemy]); // bitboard of all rooks/queens that xray king

        for _j in 0..atk_hv.count_ones() {
            // for each rook/queen, update the checkmask
            let square = atk_hv.trailing_zeros() as usize;
            pop_bit!(atk_hv, square);
            pin_hv(kingsq, square, raw_side, board.occupancies, rook_pin);
        }
    }
    if piececonstants::BISHOP_RAW_ATTACKS[kingsq] // checks if there is a bishop or queen xraying the king
        & (board.pieceboards[4 + enemy] | board.pieceboards[2 + enemy])
        != 0
    {
        let mut atk_hv = piececonstants::get_bishop_attacks(kingsq, board.occupancies[2])
            & (board.pieceboards[4 + enemy] | board.pieceboards[2 + enemy]); // bitboard of all bishop/queens that see the king

        for _i in 0..atk_hv.count_ones() {
            // for each rook/queen, update the checkmask
            let square = atk_hv.trailing_zeros() as usize;
            pop_bit!(atk_hv, square);
            check_by_slider(kingsq, square, checkmask);
        }

        atk_hv = piececonstants::BISHOP_RAW_ATTACKS[kingsq]
            & (board.pieceboards[4 + enemy] | board.pieceboards[2 + enemy]); // bitboard of all bishop/queens that xray king

        for _j in 0..atk_hv.count_ones() {
            // for each rook/queen, update the pinmask
            // somewhat innefecient, dont care
            let square = atk_hv.trailing_zeros() as usize;
            pop_bit!(atk_hv, square);
            pin_diag(kingsq, square, raw_side, board.occupancies, bishop_pin);
        }
    }

    print_bitboard(*checkmask);
    print_bitboard(*rook_pin);
}

fn check_by_slider(kingsq: usize, square: usize, checkmask: &mut u64) {
    *checkmask &= piececonstants::RAY_BETWEEN[kingsq][square]; // adds to checkmask the raw between king and enemy, inlcuding the enemy. This is used to act as a set of spots where pieces can move, usually all set to 1
}

fn pin_hv(kingsq: usize, square: usize, side: usize, occupancies: [u64; 3], pins: &mut u64) {
    // pins is  HV
    // function checks for pinned pieces, hopefully
    let pinmask = piececonstants::RAY_BETWEEN[kingsq][square];
    if (pinmask & occupancies[2]).count_ones() == 2 {
        // 1 attacking piece + 1 pinning piece
        *pins |= pinmask & occupancies[side];
    }
}

fn pin_diag(kingsq: usize, square: usize, side: usize, occupancies: [u64; 3], pins: &mut u64) {
    // pins is diags
    // function checks for pinned pieces, hopefully
    let pinmask = piececonstants::RAY_BETWEEN[kingsq][square];
    if (pinmask & occupancies[2]).count_ones() == 2 {
        // 1 attacking piece + 1 pinning piece
        *pins |= pinmask & occupancies[side];
    }
}

// TODO:  need to figure out how the fuck to do enpassant
// needs something in the pin_diag, some really weird edge cases exist
// specifically, diagonal pins can be easily done in the Diag thing, but horizontal ones are odd
