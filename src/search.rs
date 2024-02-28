use crate::{
    engine::{is_draw, Board},
    movegen,
    moves::{self, make_move, null_move, MoveStuff},
    piececonstants,
    uci::communicate,
};
use std::time::{Duration, Instant};

// TRANSPOSITION TABLE : [full hash, best move, depth, value]
// value uses integrated bounds, needs some fancy stuff in negamax but should work good
// this is annoying me, i need to sort out a way for it to perserve PV.
pub struct Transpositiontable(Vec<(u64, u16, usize, i32)>);

impl Transpositiontable {
    pub fn new() -> Transpositiontable {
        //crate::print_bitboard(piececonstants::TTABLESIZE as u64 - 1);
        Transpositiontable(vec![(0, 0, 1, 1); piececonstants::TTABLEMASK + 1])
    }
    #[inline(always)]
    fn read_value(&self, key: u64, depth: usize) -> Option<i32> {
        // TODO: hash problem :(
        //crate::print_bitboard(key);

        let index = key & (piececonstants::TTABLEMASK as u64);
        let entry = self.0[index as usize];
        if key == entry.0 && depth <= entry.2 {
            return Some(entry.3);
        } else {
            //println!("Dtable: {}, Dstate: {}", entry.2, depth);
            return None;
        }
    }
    #[inline(always)]
    pub fn read_move(&self, key: u64) -> Option<u16> {
        let index = (key & (piececonstants::TTABLEMASK as u64)) as usize;
        if self.0[index].0 == key && self.0[index].1.is_valid() {
            return Some(self.0[index].1);
        } else {
            return None;
        }
    }
    #[inline(always)]
    pub fn set_value(&mut self, key: u64, m: u16, depth: usize, score: i32) {
        let index = key & (piececonstants::TTABLEMASK as u64);
        let exactscore = score & 3 == 0;

        let entry = &mut self.0[index as usize];
        //current implementation is just stolen from stockfish on god
        if entry.1 == u16::MAX {
            // max move flags an irreplacable entry, used for repititions
            return;
        }

        if key != entry.0 || m != 0 {
            entry.1 = m
        }
        if exactscore || key != entry.0 || depth >= entry.2 {
            //key == entry.0 && depth >= entry.2) || (key != entry.0)
            // depth >= entry.2 && ((key == entry.0) || (key != entry.0 && entry.3 & 3 != 0))
            //(key == entry.0 && depth >= entry.2) || (key != entry.0 && entry.3 & 3 != 0)
            // || (key != entry.0 && entry.3 & 1 == 0) this should check to see if the node is a pv, in theory doesnt replace
            // replaces only if depth is greater than current depth. This helps perserve PV, ill see if theres a better way though
            //println!("collision: {}", entry.0 != key);
            // m as entry.1?
            entry.0 = key;
            entry.2 = depth;
            entry.3 = score;
        }
    }

    fn clear_move(&mut self, key: u64) {
        let index = key & (piececonstants::TTABLEMASK as u64);
        let entry = &mut self.0[index as usize];
        entry.1 = 0;
    }

    pub fn clear_entry(&mut self, key: u64) {
        let index = key & (piececonstants::TTABLEMASK as u64);
        let entry = &mut self.0[index as usize];
        *entry = (0, 0, 1, 1);
    }

    pub fn crude_full(&self) -> u32 {
        let mut nonzero = 0;

        for i in 0..1000 {
            if self.0[i] != (0, 0, 1, 1) {
                nonzero += 1;
            }
        }
        nonzero
    }
}
pub struct Repititiontable(Vec<bool>);
impl Repititiontable {
    pub fn new() -> Repititiontable {
        Repititiontable(vec![false; piececonstants::RTABLEMASK + 1])
    }
    #[inline(always)]
    pub fn set_value(&mut self, key: u64, value: bool) -> bool {
        let index = (key & (piececonstants::RTABLEMASK as u64)) as usize;
        let output = self.0[index].clone();
        self.0[index] = value;
        return output;
    }
}
#[derive(Clone)]
pub struct TreeLayer {
    pub board: Board,
    moves: Vec<u16>,
    scores: Vec<i32>,
    killers: Vec<u16>,
    exmove: u16,
    nullmoves: usize,
    PV: bool,
    in_check: bool,
}

fn now_and_next<T>(slc: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    // safe because a, b are in bounds and distinct
    unsafe {
        let ar = &mut *(slc.get_unchecked_mut(a) as *mut _);
        let br = &mut *(slc.get_unchecked_mut(b) as *mut _);
        (ar, br)
    }
}

pub fn search_position(
    board: &mut Board,
    maxdepth: usize,
    timelimit: Duration,
    ttable: &mut Transpositiontable,

    halfcount: usize,
) -> u16 {
    let mut treestack = vec![
        TreeLayer {
            board: Board::default(),
            moves: vec![0; 256],
            scores: vec![0i32; 256],
            killers: vec![0; 2],
            exmove: 0,
            nullmoves: 0,
            PV: false,
            in_check: false,
        };
        piececonstants::MAXPLY
    ];
    treestack[0].board = board.clone();
    treestack[0].PV = true;

    let mut m = 0;

    let mut historytable = vec![vec![vec![0usize; 64]; 64]; 2];

    let mut stopped = false;

    let start = Instant::now();
    let mut count = 0;
    for d in 1..=maxdepth {
        treestack[0].board = board.clone();

        let mut mate = None;

        let mut score = negamax(
            &mut count,
            &mut treestack,
            d,
            0,
            -100000000,
            100000000,
            ttable,
            halfcount,
            &mut historytable,
            &mut stopped,
            start,
            timelimit,
        );
        if stopped {
            break;
        }
        m = match ttable.read_move(board.key) {
            Some(i) => i,
            None => 0,
        };
        score = match ttable.read_value(board.key, d) {
            Some(i) => i,
            None => score,
        };

        let mut movestring = "".to_string();
        let mut boardcopy = board.clone();
        //for i in 0..d
        for i in 0..d {
            let m2 = &ttable.read_move(boardcopy.key);
            //println!("{:?}   {:?}", m2, score);
            match m2 {
                Some(j) => {
                    let _ = movegen::generate_moves(&mut boardcopy, &mut treestack[i].moves);
                    if treestack[i].moves.contains(&j) && j != &0 {
                        make_move(&mut boardcopy, j);
                        movestring += &j.to_uci();
                        movestring += " ";
                    }
                }
                None => {
                    break;
                    //println!("Not in Table!");
                }
            }
        }
        if ((score + 1) & !3) / 4 >= -piececonstants::MATESCORE - 200 {
            mate = Some(-((score + 1) & !3) / 4 - piececonstants::MATESCORE);
        } else if score <= piececonstants::MATESCORE + 200 {
            mate = Some(-((score + 1) & !3) / 4 + piececonstants::MATESCORE);
        }

        if mate != None {
            println!(
                "info score mate {} hashfull {} depth {} nodes {} time {} pv {} ",
                mate.unwrap(),
                ttable.crude_full(),
                d,
                &count,
                start.elapsed().as_millis(),
                movestring
            );
        } else {
            println!(
                "info score cp {} hashfull {} depth {} nodes {} time {} pv {} ",
                ((score + 1) & !3) / 4,
                ttable.crude_full(),
                d,
                &count,
                start.elapsed().as_millis(),
                movestring
            );
        }

        if stopped {
            break;
        }
    }
    println!("info nodes {}", count);

    /*let startsq = m.get_initial();
    let attacker = board.get_attacker(startsq as usize);
    if attacker == 0 || attacker == 6 || m.get_extra() != 0 {
        halfcount = 0;
    }*/

    //rtable.set_value(board.key, true);
    let mut boardcopy = board.clone();
    make_move(&mut boardcopy, &m);
    //rtable.set_value(boardcopy.key, true);

    //let m = ttable.read_move(board.key);

    //let m = ttable.read_move(board.key);

    m
}
//Positionstack: Vector of board states after search initiated
//Count: Number of function calls
//Movestack: Static movelist that is populated in generation for each ply
//scores: move scores hash
//Depth: Depth remainingF in search
//ply: opposite of depth, index of position and movestack
//alpha/beta: minmax bounds
// ttable/rtable: hash for transposition table and repitions
// killertable: Killer moves for move eval
// puneflags (null move count, PV search)
pub fn negamax(
    count: &mut usize,
    treestack: &mut [TreeLayer],
    depth: usize,
    ply: usize,
    alpha: i32,
    beta: i32,
    ttable: &mut Transpositiontable,
    halfcount: usize,
    historytable: &mut Vec<Vec<Vec<usize>>>,
    stopped: &mut bool,
    starttime: Instant,
    timelimit: Duration,
) -> i32 {
    *count += 1;
    //println!("Alpha: {} Beta: {}", alpha, beta);

    let key = treestack[0].board.key;
    if (*count & 4095) == 0 {
        communicate(stopped, starttime, timelimit);
    }
    if ply == piececonstants::MAXPLY - 1 {
        return 0;
    }

    if ttable.0[(key & (piececonstants::TTABLEMASK as u64)) as usize].0 != 0
        && ply != 0
        && treestack[0].nullmoves == 0
        && is_draw(treestack, ply, halfcount)
        && false
    {
        //println!("1");
        return piececonstants::draw_score(ply);
    }
    //println!("Full: {}", treestack.as_ptr() as usize);
    let (now, next) = treestack.split_at_mut(1);
    //println!("Start: {}", now.as_ptr() as usize);
    let position = &mut now[0];

    let cut = (beta + 1) & !3; //IBV tomfoole&mutry
    let mut alpha2 = (alpha + 1) & !3; // IBV tomfoolery again?
    let tscore = ttable.read_value(key, depth);
    match tscore {
        Some(score) => {
            if {
                match score & 3 {
                    0 => true,
                    1 => score >= cut,
                    3 => score <= alpha2,
                    _ => false,
                }
            } & !position.PV
            {
                return score;
            }
        }

        None => {}
    }
    //pruneflags.1 = false;

    if depth == 0 || *stopped {
        let score = quiescent(
            treestack,
            count,
            ply,
            0,
            alpha,
            beta,
            ttable,
            historytable,
            stopped,
            starttime,
            timelimit,
        );
        //println!("{}", score);

        //ttable.set_value(key, 0, 0, score);
        return score;
    }

    // Null Move Pruning
    let side = position.board.side.unwrap() as usize;
    position.in_check = position.board.is_king_attacked();
    // if pruning allowed in search (hasnt happened yet and not pV)& side has more than just pawns & depth is high enough for reduction & king is not in check
    if position.nullmoves < 2
        && depth >= 3
        && (position.board.occupancies[side]
            != (position.board.pieceboards[side * 6] | position.board.pieceboards[side * 6 + 5]))
        && !position.in_check
    {
        next[0].board = position.board.clone();
        next[0].nullmoves = position.nullmoves + 1;
        null_move(&mut next[0].board); // do nothing
        let score = -zerowindow(
            next,
            count,
            depth.saturating_sub(4),
            ply + 1,
            -alpha,
            ttable,
            halfcount + 1,
            historytable,
            stopped,
            starttime,
            timelimit,
        );

        //pruneflags.0 -= 1;

        if score >= cut {
            let lb = ((beta + 1) & !3) + 1;
            //ttable.set_value(key, 0, depth, lb);

            return lb;
        }
    }
    let mut d = depth;

    // sort moves
    // Extract move from Transposition Table
    let tmove = ttable.read_move(key);

    let mut movephase: usize = match tmove {
        Some(m) => {
            position.moves[0] = m;
            0
        }
        None => 1,
    };

    let mut maxh = 1;

    //movestack[ply][0..index].sort_unstable_by_key(|m| scores[4095 & *m as usize]);
    //println!("{:?}    {}", movestack[ply], index);

    let side = position.board.side.unwrap() as usize;

    //create a min heap for moves

    let mut best = 0;
    let mut index = 0;
    let mut totalmoves = 0;
    let mut i = 0;
    let mut j = 0;
    let mut badcaps = 0;
    let mut badindex = 0;

    loop {
        //println!("{} {}", alpha2, beta);
        // extract next move from heap

        let m = moves::pick_move(
            &mut position.board,
            &mut position.moves,
            &mut position.scores,
            &mut index,
            &mut totalmoves,
            &mut i,
            &mut movephase,
            &mut badcaps,
            &mut badindex,
            &position.killers,
            &historytable[side],
            true,
            tmove,
        );

        if m == 0 {
            break;
        }
        if m == position.exmove {
            continue;
        }
        if j == 0 {
            best = m;
        }
        j += 1;
        /* Singular Extensions:
        Need to mess with the criteria for this bullshit
         */
        // for tmove with no excluded move, sufficient depth, and lower bound ttable
        if m == tmove.unwrap_or(0)
            && position.exmove == 0
            && depth >= 4
            && tscore.unwrap_or(0) & 3 == 1
            && false
        {
            let singularbeta = tscore.unwrap() - 1 - 4 * d as i32;
            let singled = d / 2;
            position.exmove = m;
            //positionstack[ply + 1] = positionstack[ply].clone();
            let score = zerowindow(
                next,
                count,
                singled,
                ply,
                singularbeta,
                ttable,
                halfcount,
                historytable,
                stopped,
                starttime,
                timelimit,
            );
            position.exmove = 0;
            if score < singularbeta {
                d += 1;
            } else if singularbeta >= beta {
                return singularbeta;
            }
        }

        //println!("{:?}", &scores[0..index]);

        next[0].board = position.board.clone();

        let halfcount2 = match m & 61440 {
            0 => halfcount + 1,
            _ => 0,
        };
        //move resets halfmove clock, some kind of special move

        make_move(&mut next[0].board, &m);
        //next[0].board.print_board();
        //print_bitboard(positionstack[ply + 1].key);
        /*let op_checked = positionstack[ply + 1].is_king_attacked();
        // Check Extensions
        if op_checked {
            d += 1;
        } */
        let mut score;
        if j == 0 {
            score = -negamax(
                count,
                next,
                d - 1,
                ply + 1,
                -beta,
                -alpha2,
                ttable,
                halfcount2,
                historytable,
                stopped,
                starttime,
                timelimit,
            );
        } else {
            score = -zerowindow(
                next,
                count,
                d - 1,
                ply + 1,
                -alpha2,
                ttable,
                halfcount2,
                historytable,
                stopped,
                starttime,
                timelimit,
            );
            if score > alpha2 && score < cut {
                score = -negamax(
                    count,
                    next,
                    d - 1,
                    ply + 1,
                    -beta,
                    -alpha2,
                    ttable,
                    halfcount2,
                    historytable,
                    stopped,
                    starttime,
                    timelimit,
                );
            }
        }
        if *stopped {
            return 0;
        }
        //rtable.set_value(positionstack[ply + 1].key, false); // reset repition board
        //println!(
        //    "Score: {}   Cut: {}   Max: {}  Ply: {}",
        //    score, cut, alpha2, ply
        //);1562949

        if score > alpha2 {
            alpha2 = score;
            best = m;
            if score >= cut {
                // lower bound, cut node
                //let lb = ((score + 1) & !3) + 1; this is fail-soft for beta cutoff
                //println!("{} out of {}", i, index);

                alpha2 = beta;
                //ttable.set_value(key, m, depth, lb);
                if m & 16384 == 0 && !next[0].in_check {
                    // noncapture moves
                    position.killers[1] = position.killers[0]; // there could be some borrow bullshit happeneing here
                    position.killers[0] = m.clone();
                    historytable[side][m.get_initial() as usize][m.get_final() as usize] +=
                        depth * depth;
                }
                break;
            }
        }
        //not PV search anymore
    }

    if j == 0 {
        if position.in_check {
            let score = 4 * (piececonstants::MATESCORE + ply as i32);
            ttable.set_value(key, 0, depth, score);
            ttable.clear_move(key);
            return score;
        } else {
            //println!("2");
            let score = piececonstants::draw_score(ply);
            ttable.set_value(key, 0, depth, score);
            ttable.clear_move(key);
            return score;
        }
    } else if alpha2 == (alpha + 1) & !3 {
        // upper bound, open node
        alpha2 = ((alpha + 1) & !3) - 1;
    } else if alpha2 == beta {
        // Lower bound, Cut node
        alpha2 = ((beta + 1) & !3) + 1;
        //ttable.set_value(key, best, depth, alpha2);
    } else {
        alpha2 = (alpha2 + 1) & !3;
    }

    ttable.set_value(key, best, depth, alpha2);
    alpha2
}

#[inline]
pub fn quiescent(
    treestack: &mut [TreeLayer],
    count: &mut usize,

    ply: usize,
    qply: usize,
    alpha: i32,
    beta: i32,
    ttable: &mut Transpositiontable,

    historytable: &mut Vec<Vec<Vec<usize>>>,
    stopped: &mut bool,
    starttime: Instant,
    timelimit: Duration,
) -> i32 {
    *count += 1;
    //println!("Alpha: {} Beta: {}", alpha, beta);

    let key = treestack[0].board.key;
    if (*count & 4095) == 0 {
        communicate(stopped, starttime, timelimit);
    }
    /* match ttable.read_value(key, 0) {
        Some(score) => {
            if {
                match score & 3 {
                    0 => true,
                    1 => score >= beta,
                    3 => score <= alpha,
                    _ => false,
                }
            } {
                return score;
            }
        }

        None => {}
    }*/
    let (now, next) = treestack.split_at_mut(1);
    let position = &mut now[0];
    position.in_check = position.board.is_king_attacked();
    let eval = position.board.evaluate(ply);
    let cut = (beta + 1) & !3; //IBV tomfoolery
    let mut alpha2 = (alpha + 1) & !3;
    if eval >= cut && (position.in_check || qply <= 0) {
        return cut + 1;
    }
    let big_delta = 3600; // queen value, ignore promotion moves for now && !positionstack[ply].is_king_attacked()

    if (eval + big_delta < alpha2) || ply == piececonstants::MAXPLY - 1 {
        //&& !positionstack[ply].is_king_attacked() Consider not in check
        // If no move can improve alpha
        return alpha;
    }
    alpha2 = if eval > alpha2 { eval } else { alpha2 };

    // Extract move from Transposition Table
    let tmove = ttable.read_move(key);

    let mut movephase: usize = match tmove {
        Some(m) => {
            if m.get_extra() & 4 != 0 {
                position.moves[0] = m;
                0
            } else {
                1
            }
        }
        None => 1,
    };

    //movestack[ply][0..index].sort_unstable_by_key(|m| scores[4095 & *m as usize]);
    //println!("{:?}    {}", movestack[ply], index);

    let side = position.board.side.unwrap() as usize;

    //create a min heap for moves

    let mut index = 0;
    let mut i = 0;
    let mut j = 0;
    let mut totalmoves = 0;
    let mut badcaps = 0;
    let mut badindex = 0;
    //let mut bestmove = 0;

    loop {
        let m = moves::pick_move(
            &mut position.board,
            &mut position.moves,
            &mut position.scores,
            &mut index,
            &mut totalmoves,
            &mut i,
            &mut movephase,
            &mut badcaps,
            &mut badindex,
            &position.killers,
            &historytable[side],
            position.in_check && (qply <= 0),
            tmove,
        );
        //let m = 0;

        if m == 0 {
            break;
        }
        /*if i == 0 {
            bestmove = m;
        }*/
        j += 1;

        next[0].board = position.board.clone();

        make_move(&mut next[0].board, &m);

        //let key2 = board.key;

        let score = -quiescent(
            next,
            count,
            ply + 1,
            qply + 1,
            -beta,
            -alpha2,
            ttable,
            historytable,
            stopped,
            starttime,
            timelimit,
        );
        if score > alpha2 {
            //bestmove = m;
            if score >= cut {
                // lower bound, cut node
                alpha2 = beta;

                break;
            }

            alpha2 = score;
        }
    }
    if j == 0
        && ((treestack[0].in_check && qply <= 0) || !movegen::has_quiets(&mut treestack[0].board))
    {
        if treestack[0].in_check {
            let score = 4 * (piececonstants::MATESCORE + ply as i32);
            return score;
        } else {
            let score = piececonstants::draw_score(ply);
            return score;
        }
    } else if alpha2 == (alpha + 1) & !3 {
        // upper bound, open node
        alpha2 = ((alpha + 1) & !3) - 1;
    } else if alpha2 == beta {
        // Lower bound, Cut node
        alpha2 = ((beta + 1) & !3) + 1;
        //ttable.set_value(key, best, depth, alpha2);
    } else {
        alpha2 = (alpha2 + 1) & !3;
    }

    //ttable.set_value(key, bestmove, 0, alpha2);
    alpha2
}
//#[inline]
pub fn zerowindow(
    // TODO: some shit is fucked up here
    treestack: &mut [TreeLayer],
    count: &mut usize,

    depth: usize,
    ply: usize,
    beta: i32,
    ttable: &mut Transpositiontable,

    halfcount: usize,

    historytable: &mut Vec<Vec<Vec<usize>>>,

    stopped: &mut bool,
    starttime: Instant,
    timelimit: Duration,
) -> i32 {
    *count += 1;
    //println!("Alpha: {} Beta: {}", alpha, beta);
    if (*count & 4095) == 0 {
        communicate(stopped, starttime, timelimit);
    }

    let key = treestack[0].board.key;
    if ply == piececonstants::MAXPLY - 1 {
        return 0;
    } else if ttable.0[(key & (piececonstants::TTABLEMASK as u64)) as usize].0 != 0
        && treestack[0].nullmoves == 0
        && is_draw(treestack, ply, halfcount)
        && false
    // TODO
    {
        //println!("3");
        return piececonstants::draw_score(ply);
    }
    let tscore = ttable.read_value(key, depth);
    match tscore {
        Some(score) => {
            if treestack[0].exmove == 0 && {
                match score & 3 {
                    0 => true,
                    1 => score >= beta,
                    3 => score <= beta - 4,
                    _ => false,
                }
            } & !treestack[0].PV
            {
                return score;
            }
        }

        None => {}
    }

    if depth == 0 {
        let score = quiescent(
            treestack,
            count,
            ply,
            0,
            beta - 4,
            beta,
            ttable,
            historytable,
            stopped,
            starttime,
            timelimit,
        );
        //let score = positionstack[ply].evaluate();
        //(key, 0, 0, score + 1);
        return score;
    }
    let (now, next) = treestack.split_at_mut(1);
    let position = &mut now[0];

    let mut cut = (beta + 1) & !3; //IBV tomfoolery 199492
    let side = position.board.side.unwrap() as usize;
    position.in_check = position.board.is_king_attacked();

    // if pruning allowed in search (hasnt happened yet and not pV)& side has more than just pawns & depth is high enough for reduction & king is not in check
    if position.nullmoves < 2
        && position.exmove == 0
        && depth >= 3
        && (position.board.occupancies[side]
            != (position.board.pieceboards[side * 6] | position.board.pieceboards[side * 6 + 5]))
        && !position.in_check
    {
        next[0].board = position.board.clone();
        next[0].nullmoves = position.nullmoves + 1;
        null_move(&mut next[0].board); // do nothing
        let score = -zerowindow(
            next,
            count,
            depth.saturating_sub(4),
            ply + 1,
            4 - beta,
            ttable,
            halfcount,
            historytable,
            stopped,
            starttime,
            timelimit,
        );

        if score >= cut {
            //let lb = ((beta + 1) & !3) + 1;
            //ttable.set_value(key, 0, depth, lb);
            //println!("BRU {} {}", beta, score);
            return beta;
        }
    }
    let mut d = depth;
    //let index = generate_moves(&mut positionstack[ply], &mut movestack[ply]);
    // sort moves
    //println!("{:?}   {}", movestack[ply], index);
    // Extract move from Transposition Table
    let tmove = ttable.read_move(key);

    let mut movephase: usize = match tmove {
        Some(m) => {
            position.moves[0] = m;
            0
        }
        None => {
            // If no Tmove, check for Internal Iterative Reductions
            // goofy Internal Interative Reductions, helps with move ordering but also turns depth into a lie
            if depth >= piececonstants::INTERNALREDUCTION {
                d -= 1;
            }
            1
        }
    };

    let mut bestmove = 0;
    //let mut cut = ((beta + 1) & !3);

    //movestack[ply][0..index].sort_unstable_by_key(|m| scores[4095 & *m as usize]);
    //println!("{:?}    {}", movestack[ply], index);
    //let mut alpha2 = (alpha + 1) & !3; // IBV tomfoolery again?

    //create a min heap for moves

    let mut index = 0;
    let mut totalmoves = 0;
    let mut i = 0;
    let mut j = 0;
    let mut badcaps = 0;
    let mut badindex = 0;

    loop {
        let m = moves::pick_move(
            &mut position.board,
            &mut position.moves,
            &mut position.scores,
            &mut index,
            &mut totalmoves,
            &mut i,
            &mut movephase,
            &mut badcaps,
            &mut badindex,
            &position.killers,
            &historytable[side],
            true,
            tmove,
        );

        if m == 0 {
            break;
        }
        if m == position.exmove {
            continue;
        }
        j += 1;

        /* Singular Extensions:
        Need to mess with the criteria for this bullshit
         */
        // for tmove with no excluded move, suwfficient depth, and lower bound ttable
        if m == tmove.unwrap_or(0)
            && position.exmove == 0
            && depth >= 4
            && tscore.unwrap_or(0) & 3 == 1
            && false
        {
            let singularbeta = tscore.unwrap() - 1 - 4 * d as i32;
            let singled = d / 2;

            //positionstack[ply + 1] = positionstack[ply].clone();
            position.exmove = m;
            let score = zerowindow(
                next,
                count,
                singled,
                ply,
                singularbeta,
                ttable,
                halfcount + 1,
                historytable,
                stopped,
                starttime,
                timelimit,
            );
            position.exmove = 0;
            if score < singularbeta {
                d += 1;
            } else if singularbeta >= beta {
                return singularbeta;
            }
        }

        next[0].board = position.board.clone();
        let halfcount2 = match m & 61440 {
            0 => halfcount + 1,
            _ => 0,
        };

        //println!("{}", movestack[ply][i].to_uci());
        make_move(&mut next[0].board, &m);
        next[0].in_check = next[0].board.is_king_attacked();
        /*let op_checked = positionstack[ply + 1].is_king_attacked();
        // Check Extensions
        if op_checked {
            d += 1;
        } */
        let mut score;
        if d >= 2 && !position.in_check && m & 49152 == 0 && !next[0].in_check {
            let r = (((d as f64).log2() * (j as f64).log2() * piececonstants::LMRLEVEL).floor())
                .min(d as f64 - 0.)
                .max(1.) as usize;
            //println!("{}", r);
            //println!("{} {} {}", d, i, r);
            score = -zerowindow(
                next,
                count,
                d - r,
                ply + 1,
                4 - beta,
                ttable,
                halfcount + 1,
                historytable,
                stopped,
                starttime,
                timelimit,
            );
            if score > cut - 4 && score < cut {
                //CHECK THIS
                score = -zerowindow(
                    next,
                    count,
                    d - 1,
                    ply + 1,
                    4 - beta,
                    ttable,
                    halfcount + 1,
                    historytable,
                    stopped,
                    starttime,
                    timelimit,
                );
            }
        } else {
            score = -zerowindow(
                next,
                count,
                d - 1,
                ply + 1,
                4 - beta,
                ttable,
                halfcount + 1,
                historytable,
                stopped,
                starttime,
                timelimit,
            );
        }
        if *stopped {
            return 0;
        }

        //let key2 = board.key;

        //rtable.set_value(positionstack[ply + 1].key, false); // reset repition board
        //println!(
        //    "Score: {}   Cut: {}   Max: {}  Ply: {}",
        //    score, cut, alpha2, ply
        //);1562949
        if score >= cut {
            // lower bound, cut node
            cut = ((beta + 1) & !3) + 1;
            bestmove = m;

            if m & 16384 == 0 && !next[0].in_check {
                // noncapture moves
                position.killers[1] = position.killers[0]; // there could be some borrow bullshit happeneing here
                position.killers[0] = m.clone();
                historytable[position.board.side.unwrap() as usize][m.get_initial() as usize]
                    [m.get_final() as usize] += depth * depth;
            }
            //ttable.set_value(positionstack[ply].key, m, depth, lb);
            break;
        }
    }
    if j == 0 {
        if position.in_check {
            let score = 4 * (piececonstants::MATESCORE + ply as i32);
            ttable.set_value(key, 0, depth, score);
            ttable.clear_move(key);
            return score;
        } else {
            //println!("4");
            let score = piececonstants::draw_score(ply);
            ttable.set_value(key, 0, depth, score);
            ttable.clear_move(key);
            return score;
        }
    }
    if cut == (beta + 1) & !3 {
        cut -= 5;
    }
    ttable.set_value(key, bestmove, depth, cut);
    return cut;
}
