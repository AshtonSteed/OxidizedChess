use crate::engine;
use crate::engine::BitBoard;
use crate::moves;
use crate::moves::MoveStuff;
use crate::piececonstants;

pub fn refresh(
    // this monstrosity sets pinmasks and checkmasks, allowing for full move generation without needing to individually check move legality after the fact
    // should be significantly more efficient than generating and checking all naive moves
    // kingban must be initialized as enemy kings attack
    board: &mut engine::Board,
) {
    // sets checkmask to all 1s
    let mut checkmask = u64::MAX;

    let mut rook_pin = 0;
    let mut bishop_pin = 0;

    let raw_side = (board.side == Some(1)) as usize;
    let enemy_raw = (board.side == Some(0)) as usize;
    let side = 6 * raw_side;
    let enemy = 6 * enemy_raw; // acts opposite of side to move
    let kingboard = board.pieceboards[5 + side];
    let mut kingban =
        piececonstants::KING_ATTACKS[board.pieceboards[5 + enemy].trailing_zeros() as usize];
    //crate::print_bitboard(kingboard);
    let kingsq = kingboard.trailing_zeros() as usize;
    if kingsq == 64 {
        // maybe remove this and just require a king on board TODO:
        panic!("No King on board!");
    }
    let pawn_attacks = piececonstants::PAWN_ATTACKS[raw_side][kingsq] & board.pieceboards[enemy]; // updates checkmask to include a single psosible pawn attack
    if pawn_attacks != 0 {
        checkmask = pawn_attacks;
    }

    let knight_attacks = piececonstants::KNIGHT_ATTACKS[kingsq] & board.pieceboards[enemy + 1]; // updates checkmask to include a single knight attack
    if knight_attacks != 0 {
        checkmask = knight_attacks;
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
            atk_hv.pop_bit(square);
            check_by_slider(kingsq, square, &mut checkmask);
        }

        atk_hv = piececonstants::ROOK_RAW_ATTACKS[kingsq]
            & (board.pieceboards[4 + enemy] | board.pieceboards[3 + enemy]); // bitboard of all rooks/queens that xray king

        for _j in 0..atk_hv.count_ones() {
            // for each rook/queen, update the checkmask
            let square = atk_hv.trailing_zeros() as usize;
            atk_hv.pop_bit(square);
            pin_hv(kingsq, square, board.occupancies[2], &mut rook_pin);
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
            atk_hv.pop_bit(square);
            check_by_slider(kingsq, square, &mut checkmask);
        }

        atk_hv = piececonstants::BISHOP_RAW_ATTACKS[kingsq]
            & (board.pieceboards[4 + enemy] | board.pieceboards[2 + enemy]); // bitboard of all bishop/queens that xray king

        for _j in 0..atk_hv.count_ones() {
            // for each bishop/queen, update the pinmask
            // somewhat innefecient, dont care
            let square = atk_hv.trailing_zeros() as usize;
            atk_hv.pop_bit(square);
            pin_diag(
                kingsq,
                square,
                board.occupancies[2],
                &mut bishop_pin,
                &mut board.enpassant,
            );
        }
    }
    if board.enpassant != 0 {
        pin_ep(
            kingsq,
            board.occupancies[2],
            board.pieceboards,
            side,
            enemy,
            enemy_raw,
            &mut board.enpassant,
        );
    }
    let king_atk = piececonstants::KING_ATTACKS[kingsq] & !board.occupancies[raw_side] & !kingban;

    //piececonstants::KING_ATTACKS[board.pieceboards[enemy_raw + 5].trailing_zeros() as usize];
    if king_atk == 0 {
        board.movemasks = [kingban, checkmask, rook_pin, bishop_pin, 0];
    } else {
        let mut knights = board.pieceboards[enemy + 1];

        //print_bitboard(knights);
        for _i in 0..knights.count_ones() {
            let square = knights.trailing_zeros() as usize;
            knights.pop_bit(square);
            kingban |= piececonstants::KNIGHT_ATTACKS[square];
        }

        let mut pawns = board.pieceboards[enemy];
        for _j in 0..pawns.count_ones() {
            let square = pawns.trailing_zeros() as usize;
            pawns.pop_bit(square);
            kingban |= piececonstants::PAWN_ATTACKS[enemy_raw][square];
        }
        let kingless_occupancy = board.occupancies[2] & !kingboard;
        let mut bishops = board.pieceboards[enemy + 2] | board.pieceboards[enemy + 4];
        for _k in 0..bishops.count_ones() {
            let square = bishops.trailing_zeros() as usize;
            bishops.pop_bit(square);
            kingban |= piececonstants::get_bishop_attacks(square, kingless_occupancy);
        }
        let mut rooks = board.pieceboards[enemy + 3] | board.pieceboards[enemy + 4];
        for _i2 in 0..rooks.count_ones() {
            let square = rooks.trailing_zeros() as usize;
            rooks.pop_bit(square);
            kingban |= piececonstants::get_rook_attacks(square, kingless_occupancy);
        }
        board.movemasks = [
            kingban,
            checkmask,
            rook_pin,
            bishop_pin,
            king_atk & !kingban,
        ];
    }
    //print_bitboard(*checkmask);
    //print_bitboard(*rook_pin);
    //print
}

fn check_by_slider(kingsq: usize, square: usize, checkmask: &mut u64) {
    *checkmask &= piececonstants::RAY_BETWEEN[kingsq][square]; // adds to checkmask the raw between king and enemy, inlcuding the enemy. This is used to act as a set of spots where pieces can move, usually all set to 1
}

fn pin_hv(kingsq: usize, square: usize, occupancy: u64, pins: &mut u64) {
    // pins is  HV
    // function checks for pinned pieces, hopefully
    let pinmask = piececonstants::RAY_BETWEEN[kingsq][square];
    if (pinmask & occupancy).count_ones() == 2 {
        // 1 attacking piece + 1 pinning piece
        *pins |= pinmask;
    }
}

fn pin_diag(kingsq: usize, square: usize, occupancy: u64, pins: &mut u64, enpassant: &mut u64) {
    // pins is diags
    // function checks for pinned pieces, hopefully
    let pinmask = piececonstants::RAY_BETWEEN[kingsq][square];

    if (pinmask & (enpassant.clone() >> 8 | enpassant.clone() << 8)) != 0 {
        *enpassant = 0;
    }
    if (pinmask & occupancy).count_ones() == 2 {
        // 1 attacking piece + 1 pinning piece
        *pins |= pinmask;
    }
}

fn pin_ep(
    kingsq: usize,
    occupancy: u64,
    pieceboards: [u64; 12],
    side: usize,
    enemy: usize,
    enemy_raw: usize,
    enpassant: &mut u64,
) {
    // TODO:
    let ep_row: u64 = match side {
        0 => 4278190080,
        6 => 1095216660480,
        _ => 0,
    }; // bitboard of the colum where the ep target is

    //checks to see if there is a king, attacker, and attacking pawn in the EP row
    if (pieceboards[side + 5] & ep_row) != 0
        && ((pieceboards[enemy + 3] | pieceboards[enemy + 4]) & ep_row) != 0
        && (pieceboards[side] & ep_row) != 0
    {
        //only need to check pieces towards ep pair
        let ep_pawns = crate::piececonstants::PAWN_ATTACKS[enemy_raw]
            [enpassant.trailing_zeros() as usize]
            | pieceboards[side];

        // this operation feels suspect
        if ep_pawns == 2 {
            return;
        }
        let ep_occ = occupancy & !(ep_pawns | enpassant.clone() >> 8 | enpassant.clone() << 8);

        if piececonstants::get_rook_attacks(kingsq, ep_occ) & ep_row & pieceboards[enemy + 3] != 0 {
            *enpassant = 0;
        }
    }

    // checks for horizontal pin on enpassant attacking pawn, removing EP square if necessary
}

// assumes refresh in captures
// might convert to staged generation and staged sorting
pub fn generate_quiets(board: &mut engine::Board, movelist: &mut [u16], i: usize) -> usize {
    //let mut moves: Vec<moves::Move> = Vec::new();
    //let mut moveindex = generate_captures(board, movelist);
    let mut moveindex = i;
    let raw_side = (board.side == Some(1)) as usize;
    let raw_enemy = (board.side == Some(0)) as usize;
    let side = raw_side * 6;
    //let enemy = raw_enemy * 6;

    // quiet king
    let mut king_quiet = board.movemasks[4] & !board.occupancies[2];

    let moveable = !board.occupancies[raw_side] & board.movemasks[1];

    let kingsq = board.pieceboards[side + 5].trailing_zeros() as usize;

    //king quiet
    for _i in 0..king_quiet.count_ones() {
        let square = king_quiet.trailing_zeros() as usize;
        king_quiet.pop_bit(square);

        movelist[moveindex] = moves::Move::new(kingsq as u16, square as u16, 0);
        moveindex += 1;
    }

    // castling
    let shift = raw_side * 2;
    let (m1, m2, m3, m4): (u64, u64, u64, u64) = match raw_side {
        0 => (
            0x7000000000000000,
            0x6000000000000000,
            0x1C00000000000000,
            0xE00000000000000,
        ),
        1 => (0x70, 0x60, 0x1C, 0xE),
        _ => (0, 0, 0, 0),
    };
    //kingside
    let rights = board.castle >> shift;

    if rights & 1 == 1
        && m1 & board.movemasks[0] == 0 // checks for attacks along king path and check
        && m2 & board.occupancies[2] == 0
    //checks for obsructions
    {
        movelist[moveindex] = moves::Move::new(kingsq as u16, kingsq as u16 + 2, 2);
        moveindex += 1;
    }
    //queenside
    if rights & 2 == 2
        && m3 & board.movemasks[0] == 0 // checks for attacks along king path and check
        && m4 & board.occupancies[2] == 0
    //checks for obsructions
    {
        movelist[moveindex] = moves::Move::new(kingsq as u16, kingsq as u16 - 2, 3);
        moveindex += 1;
    }

    //knights
    let mut knights = board.pieceboards[side + 1] & !(board.movemasks[2] | board.movemasks[3]); // prunes pinned knights, they cannot move

    for _i in 0..knights.count_ones() {
        let knight = knights.trailing_zeros() as usize;
        knights.pop_bit(knight);
        let attacks = piececonstants::KNIGHT_ATTACKS[knight] & moveable; //prunes night moves to those that follow the checkmask and dont self capture

        let mut quiet = attacks & !board.occupancies[raw_enemy]; //divides night moves to those that dont capture

        for _j in 0..quiet.count_ones() {
            let attack = quiet.trailing_zeros() as usize;
            quiet.pop_bit(attack);

            movelist[moveindex] = moves::Move::new(knight as u16, attack as u16, 0);
            moveindex += 1;
        }
    }
    //rooks and queenHV
    let rooksq =
        (board.pieceboards[side + 4] | board.pieceboards[side + 3]) & !(board.movemasks[3]); // no hv when diagonally pinned
    let mut rooksqpin = rooksq & board.movemasks[2];
    let mut rooksqno = rooksq & !board.movemasks[2];

    //pinned rooks
    for _i in 0..rooksqpin.count_ones() {
        let rook = rooksqpin.trailing_zeros() as usize;
        rooksqpin.pop_bit(rook);
        let attacks = piececonstants::get_rook_attacks(rook, board.occupancies[2])
            & moveable
            & board.movemasks[2]; //prunes rook moves to those that follow the checkmask and dont self capture

        let mut quiet = attacks & !board.occupancies[raw_enemy]; //divides rook moves to those that dont capture
        for _j in 0..quiet.count_ones() {
            let attack = quiet.trailing_zeros() as usize;
            quiet.pop_bit(attack);

            movelist[moveindex] = moves::Move::new(rook as u16, attack as u16, 0);
            moveindex += 1;
        }
    }
    //unpinned rooks
    for _i in 0..rooksqno.count_ones() {
        let rook = rooksqno.trailing_zeros() as usize;
        rooksqno.pop_bit(rook);
        let attacks = piececonstants::get_rook_attacks(rook, board.occupancies[2]) & moveable; //prunes rook moves to those that follow the checkmask and dont self capture

        let mut quiet = attacks & !board.occupancies[raw_enemy]; //divides rook moves to those that dont capture
        for _j in 0..quiet.count_ones() {
            let attack = quiet.trailing_zeros() as usize;
            quiet.pop_bit(attack);
            movelist[moveindex] = moves::Move::new(rook as u16, attack as u16, 0);
            moveindex += 1;
        }
    }

    //bishops and queenDI
    let bishopq =
        (board.pieceboards[side + 4] | board.pieceboards[side + 2]) & !(board.movemasks[2]); // no DI when horizontally pinned
    let mut bishopqpin = bishopq & board.movemasks[3];
    let mut bishopqno = bishopq & !board.movemasks[3];

    //pinned bishops
    for _i in 0..bishopqpin.count_ones() {
        let bishop = bishopqpin.trailing_zeros() as usize;
        bishopqpin.pop_bit(bishop);
        let attacks = piececonstants::get_bishop_attacks(bishop, board.occupancies[2])
            & moveable
            & board.movemasks[3]; //prunes bishops moves to those that follow the checkmask and dont self capture

        let mut quiet = attacks & !board.occupancies[raw_enemy]; //divides bishops moves to those that dont capture
        for _j in 0..quiet.count_ones() {
            let attack = quiet.trailing_zeros() as usize;
            quiet.pop_bit(attack);
            movelist[moveindex] = moves::Move::new(bishop as u16, attack as u16, 0);
            moveindex += 1;
        }
    }
    //unpinned bishop
    for _i in 0..bishopqno.count_ones() {
        let bishop = bishopqno.trailing_zeros() as usize;
        bishopqno.pop_bit(bishop);
        let attacks = piececonstants::get_bishop_attacks(bishop, board.occupancies[2]) & moveable; //prunes bishop moves to those that follow the checkmask and dont self capture
        let mut quiet = attacks & !board.occupancies[raw_enemy]; //divides bishop moves to those that dont capture
        for _j in 0..quiet.count_ones() {
            let attack = quiet.trailing_zeros() as usize;
            quiet.pop_bit(attack);
            movelist[moveindex] = moves::Move::new(bishop as u16, attack as u16, 0);
            moveindex += 1;
        }
    }

    // IDEAS: Divide into a ton of pawn groups, I guess. Gonna make alot of messy, annoying code
    if raw_side == 0 {
        let pawns = board.pieceboards[0]; // white pawns
        let pawnsdiagonal = pawns & !board.movemasks[2]; // can go diagonal
        let pawnsforward = pawns & !board.movemasks[3]; // can walk forward
        let mut pinf = pawnsforward & board.movemasks[2]; // forward pawns that are pinned
        let mut nopinf = pawnsforward & !board.movemasks[2]; //unpinned forward pawns

        let pushrow = 0xFF000000000000u64;
        let promoterow = 0xFF00u64;

        // quiet normal pawn moves
        for _i in 0..nopinf.count_ones() {
            let pawn = nopinf.trailing_zeros() as u16;
            nopinf.pop_bit(pawn as usize);
            let pawnb = 1u64 << pawn;
            let attackb = pawnb >> 8;
            if attackb & board.occupancies[2] == 0 && attackb & board.movemasks[1] != 0 {
                if pawnb & promoterow != 0 {
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 8);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 9);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 10);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 11);
                    moveindex += 1;
                } else {
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 0);
                    moveindex += 1;
                }
            }
            // checks for pushable pawns by checking rank, path clearness, and then check mask
            if pawnb & pushrow != 0
                && ((attackb >> 8) | attackb) & board.occupancies[2] == 0
                && (attackb >> 8) & board.movemasks[1] != 0
            {
                movelist[moveindex] = moves::Move::new(pawn, pawn - 16, 1);
                moveindex += 1;
            };
        }
        // pinned normal moves
        for _j in 0..pinf.count_ones() {
            let pawn = pinf.trailing_zeros() as u16;
            pinf.pop_bit(pawn as usize);
            let pawnb = 1u64 << pawn;
            let attackb = pawnb >> 8;
            if attackb & board.occupancies[2] == 0
                && attackb & board.movemasks[1] != 0
                && attackb & board.movemasks[2] != 0
            {
                if pawnb & promoterow != 0 {
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 8);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 9);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 10);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 11);
                    moveindex += 1;
                } else {
                    movelist[moveindex] = moves::Move::new(pawn, pawn - 8, 0);
                    moveindex += 1;
                }
            }
            // checks for pushable pawns by checking rank, path clearness,  check mask, and pins
            if pawnb & pushrow != 0
                && ((attackb >> 8) | attackb) & board.occupancies[2] == 0
                && (attackb >> 8) & board.movemasks[1] != 0
                && (attackb >> 8) & board.movemasks[2] != 0
            {
                movelist[moveindex] = moves::Move::new(pawn, pawn - 16, 1);
                moveindex += 1;
            };
        }
    } else {
        //black pawns
        let pawns = board.pieceboards[6];
        let pawnsdiagonal = pawns & !board.movemasks[2]; // can go diagonal
        let pawnsforward = pawns & !board.movemasks[3]; // can walk forward
        let mut pinf = pawnsforward & board.movemasks[2]; // forward pawns that are pinned
        let mut nopinf = pawnsforward & !board.movemasks[2]; //unpinned forward pawns
        let mut pind = pawnsdiagonal & board.movemasks[3]; // diagonal pawns that are pinned
        let mut nopind = pawnsdiagonal & !board.movemasks[3]; // unpined diagonal pawns

        let pushrow = 0xFF00u64;
        let promoterow = 0xFF000000000000u64;

        // quiet normal pawn moves
        for _i in 0..nopinf.count_ones() {
            let pawn = nopinf.trailing_zeros() as u16;
            nopinf.pop_bit(pawn as usize);
            let pawnb = 1u64 << pawn;
            let attackb = pawnb << 8;
            if attackb & board.occupancies[2] == 0 && attackb & board.movemasks[1] != 0 {
                if pawnb & promoterow != 0 {
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 8);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 9);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 10);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 11);
                    moveindex += 1;
                } else {
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 0);
                    moveindex += 1;
                }
            }
            // checks for pushable pawns by checking rank, path clearness, and then check mask
            if pawnb & pushrow != 0
                && ((attackb << 8) | attackb) & board.occupancies[2] == 0
                && (attackb << 8) & board.movemasks[1] != 0
            {
                movelist[moveindex] = moves::Move::new(pawn, pawn + 16, 1);
                moveindex += 1;
            };
        }
        // pinned normal moves
        for _j in 0..pinf.count_ones() {
            let pawn = pinf.trailing_zeros() as u16;
            pinf.pop_bit(pawn as usize);
            let pawnb = 1u64 << pawn;
            let attackb = pawnb << 8;
            if attackb & board.occupancies[2] == 0
                && attackb & board.movemasks[1] & board.movemasks[2] != 0
            {
                if pawnb & promoterow != 0 {
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 8);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 9);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 10);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 11);
                    moveindex += 1;
                } else {
                    movelist[moveindex] = moves::Move::new(pawn, pawn + 8, 0);
                    moveindex += 1;
                }
            }
            // checks for pushable pawns by checking rank, path clearness,  check mask, and pins
            if pawnb & pushrow != 0
                && ((attackb << 8) | attackb) & board.occupancies[2] == 0
                && (attackb << 8) & board.movemasks[1] & board.movemasks[2] != 0
            {
                movelist[moveindex] = moves::Move::new(pawn, pawn + 16, 1);
                moveindex += 1;
            };
        }
    }

    moveindex
}

// only generates capture moves for quiescent searches
pub fn generate_captures(board: &mut engine::Board, movelist: &mut [u16]) -> usize {
    //let mut moves: Vec<moves::Move> = Vec::new();
    let mut moveindex = 0usize;
    let raw_side = (board.side == Some(1)) as usize;
    let raw_enemy = (board.side == Some(0)) as usize;
    let side = raw_side * 6;
    //let enemy = raw_enemy * 6;

    //king moves
    refresh(board);
    let mut king_captures = board.movemasks[4] & board.occupancies[raw_enemy];

    let moveable = !board.occupancies[raw_side] & board.movemasks[1];

    let kingsq = board.pieceboards[side + 5].trailing_zeros() as u16;
    //king captures
    for _i in 0..king_captures.count_ones() {
        let square = king_captures.trailing_zeros() as u16;
        king_captures.pop_bit(square as usize);
        movelist[moveindex] = moves::Move::new(kingsq, square, 4);
        moveindex += 1;
    }

    //knights
    let mut knights = board.pieceboards[side + 1] & !(board.movemasks[2] | board.movemasks[3]); // prunes pinned knights, they cannot move

    for _i in 0..knights.count_ones() {
        let knight = knights.trailing_zeros() as usize;
        knights.pop_bit(knight as usize);
        let attacks = piececonstants::KNIGHT_ATTACKS[knight] & moveable; //prunes night moves to those that follow the checkmask and dont self capture
        let mut captures = attacks & board.occupancies[raw_enemy]; //knight moves that capture
        for _k in 0..captures.count_ones() {
            let attack = captures.trailing_zeros() as usize;
            captures.pop_bit(attack);

            movelist[moveindex] = moves::Move::new(knight as u16, attack as u16, 4);
            moveindex += 1;
        }
    }
    //rooks and queenHV
    let rooksq =
        (board.pieceboards[side + 4] | board.pieceboards[side + 3]) & !(board.movemasks[3]); // no hv when diagonally pinned
    let mut rooksqpin = rooksq & board.movemasks[2];
    let mut rooksqno = rooksq & !board.movemasks[2];

    //pinned rooks
    for _i in 0..rooksqpin.count_ones() {
        let rook = rooksqpin.trailing_zeros() as usize;
        rooksqpin.pop_bit(rook);
        let attacks = piececonstants::get_rook_attacks(rook, board.occupancies[2])
            & moveable
            & board.movemasks[2]; //prunes rook moves to those that follow the checkmask and dont self capture
        let mut captures = attacks & board.occupancies[raw_enemy]; //rook moves that capture
        for _k in 0..captures.count_ones() {
            let attack = captures.trailing_zeros() as usize;
            captures.pop_bit(attack);

            movelist[moveindex] = moves::Move::new(rook as u16, attack as u16, 4);
            moveindex += 1;
        }
    }
    //unpinned rooks
    for _i in 0..rooksqno.count_ones() {
        let rook = rooksqno.trailing_zeros() as usize;
        rooksqno.pop_bit(rook);
        let attacks = piececonstants::get_rook_attacks(rook, board.occupancies[2]) & moveable; //prunes rook moves to those that follow the checkmask and dont self capture
        let mut captures = attacks & board.occupancies[raw_enemy]; //rook moves that capture
        for _k in 0..captures.count_ones() {
            let attack = captures.trailing_zeros() as usize;
            captures.pop_bit(attack);
            movelist[moveindex] = moves::Move::new(rook as u16, attack as u16, 4);
            moveindex += 1;
        }
    }

    //bishops and queenDI
    let bishopq =
        (board.pieceboards[side + 4] | board.pieceboards[side + 2]) & !(board.movemasks[2]); // no DI when horizontally pinned
    let mut bishopqpin = bishopq & board.movemasks[3];
    let mut bishopqno = bishopq & !board.movemasks[3];

    //pinned bishops
    for _i in 0..bishopqpin.count_ones() {
        let bishop = bishopqpin.trailing_zeros() as usize;
        bishopqpin.pop_bit(bishop);
        let attacks = piececonstants::get_bishop_attacks(bishop, board.occupancies[2])
            & moveable
            & board.movemasks[3]; //prunes bishops moves to those that follow the checkmask and dont self capture

        let mut captures = attacks & board.occupancies[raw_enemy]; //bishops moves that capture
        for _k in 0..captures.count_ones() {
            let attack = captures.trailing_zeros() as usize;
            captures.pop_bit(attack);
            movelist[moveindex] = moves::Move::new(bishop as u16, attack as u16, 4);
            moveindex += 1;
        }
    }

    //unpinned bishop
    for _i in 0..bishopqno.count_ones() {
        let bishop = bishopqno.trailing_zeros() as usize;
        bishopqno.pop_bit(bishop);
        let attacks = piececonstants::get_bishop_attacks(bishop, board.occupancies[2]) & moveable; //prunes bishop moves to those that follow the checkmask and dont self capture

        let mut captures = attacks & board.occupancies[raw_enemy]; //bishop moves that capture
        for _k in 0..captures.count_ones() {
            let attack = captures.trailing_zeros() as usize;
            captures.pop_bit(attack);
            movelist[moveindex] = moves::Move::new(bishop as u16, attack as u16, 4);
            moveindex += 1;
        }
    }

    // pawns (white) TODO: promotions need work
    // IDEAS: Divide into a ton of pawn groups, I guess. Gonna make alot of messy, annoying code
    if raw_side == 0 {
        let pawns = board.pieceboards[0]; // white pawns
        let pawnsdiagonal = pawns & !board.movemasks[2]; // can go diagonal
        let mut pind: u64 = pawnsdiagonal & board.movemasks[3]; // diagonal pawns that are pinned
        let mut nopind = pawnsdiagonal & !board.movemasks[3]; // unpined diagonal pawns
        let promoterow = 0xFF00u64;

        // Enpassant block
        if board.enpassant & moveable != 0 || (moveable != 0 && moveable == board.enpassant << 8) {
            let ensquare = board.enpassant.trailing_zeros();
            let mut nopinenpassant = piececonstants::PAWN_ATTACKS[1][ensquare as usize] & nopind;

            for _n in 0..nopinenpassant.count_ones() {
                let pawn = nopinenpassant.trailing_zeros() as u16;
                nopinenpassant.pop_bit(pawn as usize);
                movelist[moveindex] = moves::Move::new(pawn, ensquare as u16, 5);
                moveindex += 1;
            }
            if board.enpassant & board.movemasks[3] != 0 {
                let pinenpassant = piececonstants::PAWN_ATTACKS[1][ensquare as usize] & pind;
                if pinenpassant != 0 {
                    let pawn = pinenpassant.trailing_zeros() as u16;

                    movelist[moveindex] = moves::Move::new(pawn, ensquare as u16, 5);
                    moveindex += 1;
                }
            }
        }
        // quiet normal pawn moves

        //normal captures
        for _k in 0..nopind.count_ones() {
            let pawn = nopind.trailing_zeros() as u16;
            nopind.pop_bit(pawn as usize);
            let mut attacks = piececonstants::PAWN_ATTACKS[0][pawn as usize]
                & board.occupancies[1]
                & board.movemasks[1];
            for _l in 0..attacks.count_ones() {
                let attack = attacks.trailing_zeros() as u16;
                attacks.pop_bit(attack as usize);
                if promoterow.get_bit(pawn as usize) != 0 {
                    movelist[moveindex] = moves::Move::new(pawn, attack, 12);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 13);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 14);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 15);
                    moveindex += 1;
                } else {
                    movelist[moveindex] = moves::Move::new(pawn, attack, 4);
                    moveindex += 1;
                }
            }
        }
        //pinned captures
        for _m in 0..pind.count_ones() {
            let pawn = pind.trailing_zeros() as u16;
            pind.pop_bit(pawn as usize);
            let attacks = piececonstants::PAWN_ATTACKS[0][pawn as usize]
                & board.occupancies[1]
                & board.movemasks[1]
                & board.movemasks[3];

            // pretty sure theres no way for multiple pin attacks, TODO: check this
            if attacks != 0 {
                let attack = attacks.trailing_zeros() as u16;
                if promoterow.get_bit(pawn as usize) != 0 {
                    movelist[moveindex] = moves::Move::new(pawn, attack, 12);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 13);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 14);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 15);
                    moveindex += 1;
                } else {
                    movelist[moveindex] = moves::Move::new(pawn, attack, 4);
                    moveindex += 1;
                }
            }
        }
    } else {
        //black pawns
        let pawns = board.pieceboards[6];
        let pawnsdiagonal = pawns & !board.movemasks[2]; // can go diagonal
        let mut pind = pawnsdiagonal & board.movemasks[3]; // diagonal pawns that are pinned
        let mut nopind = pawnsdiagonal & !board.movemasks[3]; // unpined diagonal pawns

        let promoterow = 0xFF000000000000u64;

        // Enpassant block
        if board.enpassant & moveable != 0 || (moveable != 0 && moveable == board.enpassant >> 8) {
            let ensquare = board.enpassant.trailing_zeros();
            let mut nopinenpassant = piececonstants::PAWN_ATTACKS[0][ensquare as usize] & nopind;

            for _n in 0..nopinenpassant.count_ones() {
                let pawn = nopinenpassant.trailing_zeros() as u16;
                nopinenpassant.pop_bit(pawn as usize);
                movelist[moveindex] = moves::Move::new(pawn, ensquare as u16, 5);
                moveindex += 1;
            }
            if board.enpassant & board.movemasks[3] != 0 {
                let pinenpassant = piececonstants::PAWN_ATTACKS[0][ensquare as usize] & pind;
                if pinenpassant != 0 {
                    let pawn = pinenpassant.trailing_zeros() as u16;
                    movelist[moveindex] = moves::Move::new(pawn, ensquare as u16, 5);
                    moveindex += 1;
                }
            }
        }

        //normal captures
        for _k in 0..nopind.count_ones() {
            let pawn = nopind.trailing_zeros() as u16;
            nopind.pop_bit(pawn as usize);
            let mut attacks = piececonstants::PAWN_ATTACKS[1][pawn as usize]
                & board.occupancies[0]
                & board.movemasks[1];
            for _l in 0..attacks.count_ones() {
                let attack = attacks.trailing_zeros() as u16;
                attacks.pop_bit(attack as usize);
                if promoterow.get_bit(pawn as usize) != 0 {
                    movelist[moveindex] = moves::Move::new(pawn, attack, 12);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 13);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 14);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 15);
                    moveindex += 1;
                } else {
                    movelist[moveindex] = moves::Move::new(pawn, attack, 4);
                    moveindex += 1;
                }
            }
        }
        //pinned captures
        for _m in 0..pind.count_ones() {
            let pawn = pind.trailing_zeros() as u16;
            pind.pop_bit(pawn as usize);
            let attacks = piececonstants::PAWN_ATTACKS[1][pawn as usize]
                & board.occupancies[0]
                & board.movemasks[1]
                & board.movemasks[3];

            // pretty sure theres no way for multiple pin attacks, TODO: check this
            if attacks != 0 {
                let attack = attacks.trailing_zeros() as u16;
                if promoterow.get_bit(pawn as usize) != 0 {
                    movelist[moveindex] = moves::Move::new(pawn, attack, 12);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 13);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 14);
                    moveindex += 1;
                    movelist[moveindex] = moves::Move::new(pawn, attack, 15);
                    moveindex += 1;
                } else {
                    movelist[moveindex] = moves::Move::new(pawn, attack, 4);
                    moveindex += 1;
                }
            }
        }
    }

    moveindex
}

pub fn generate_moves(board: &mut engine::Board, movelist: &mut [u16]) -> usize {
    let c = generate_captures(board, movelist);
    return generate_quiets(board, movelist, c);
}
