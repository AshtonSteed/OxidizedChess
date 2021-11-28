use crate::piececonstants;
use crate::pieceinit;

pub fn refresh(board: &crate::engine::Board, kingban: &mut u64, checkmask: &mut u64) {
    let side = 6 * (board.side == Some(1)) as usize;
    let enemy = 6 * (board.side == Some(0)) as usize; // acts opposite of side to move
    let kingboard = board.pieceboards[5 + side];
    crate::print_bitboard(kingboard);
    let kingsq = kingboard.trailing_zeros() as usize;
    if kingsq == 64 {
        return;
    }

    let rook_pin = 0;
    let bishop_pin = 0;
    if piececonstants::ROOK_RAW_ATTACKS[kingsq] // checks if there is a square or queen xraying the king
        & (board.pieceboards[4 + enemy] | board.pieceboards[3 + enemy])
        != 0
    {
        let mut atk_hv = piececonstants::get_rook_attacks(kingsq, board.occupancies[2])
            & (board.pieceboards[4 + enemy] | board.pieceboards[3 + enemy]);

        crate::print_bitboard(atk_hv);

        for _i in 0..atk_hv.count_ones() {
            let square = atk_hv.trailing_zeros() as usize;
            pop_bit!(atk_hv, square);
            check_by_slider(kingsq, square, kingban, checkmask);
        }
    }
}

fn check_by_slider(kingsq: usize, square: usize, kingban: &mut u64, checkmask: &mut u64) {
    if checkmask == &mut 0xffffffffffffffff {
    } else {
        *checkmask = 0;
    }
}
