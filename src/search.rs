use crate::{
    engine::Board,
    movegen::{generate_captures, generate_moves},
    moves::{self, make_move, null_move, MoveStuff},
    piececonstants,
    uci::communicate,
};
use std::{
    io::repeat,
    time::{Duration, Instant},
};

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
        if self.0[index].0 == key {
            return Some(self.0[index].1);
        } else {
            return None;
        }
    }
    #[inline(always)]
    fn set_value(&mut self, key: u64, m: u16, depth: usize, score: i32) {
        let index = key & (piececonstants::TTABLEMASK as u64);
        let exactscore = score & 3 == 0;

        let mut entry = self.0[index as usize];
        //current implementation is just stolen from stockfish on god

        if key != entry.0 || depth >= entry.2 {
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
            self.0[index as usize] = (key, entry.1, depth, score);
        }
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

pub fn search_position(
    board: &mut Board,
    maxdepth: usize,
    timelimit: Duration,
    ttable: &mut Transpositiontable,
    history: &mut Vec<u64>,
    halfcount: usize,
) -> u16 {
    let mut positionstack = vec![board.clone(); piececonstants::MAXPLY];
    let mut m = 0;

    let mut killertable = vec![vec![0; 2]; piececonstants::MAXPLY];
    let mut historytable = vec![vec![vec![0usize; 64]; 64]; 2];

    let mut movestack = vec![vec![0; 256]; piececonstants::MAXPLY];
    let mut movescores = vec![vec![0i32; 256]; piececonstants::MAXPLY];
    let mut stopped = false;
    let mut pruneflags = (0, true);
    let start = Instant::now();

    for d in 1..=maxdepth {
        positionstack[0] = board.clone();

        let mut count = 0;
        let mut mate = None;
        let mut score = negamax(
            &mut positionstack,
            &mut count,
            &mut movestack,
            &mut movescores,
            d,
            0,
            -2147483600,
            2147483600,
            ttable,
            history,
            halfcount,
            &mut killertable,
            &mut historytable,
            &mut pruneflags,
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
        for i in 0..d {
            let m2 = &ttable.read_move(boardcopy.key);
            //println!("{:?}   {:?}", m2, score);
            match m2 {
                Some(j) => {
                    let _ = generate_moves(&mut boardcopy, &mut movestack[i]);
                    if movestack[i].contains(&j) && j != &0 {
                        make_move(&mut boardcopy, j);
                        movestring += &j.to_uci();
                        movestring += " ";
                    };
                }
                None => {}
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
//Depth: Depth remaining in search
//ply: opposite of depth, index of position and movestack
//alpha/beta: minmax bounds
// ttable/rtable: hash for transposition table and repitions
// killertable: Killer moves for move eval
// puneflags (null move count, PV search)
pub fn negamax(
    positionstack: &mut Vec<Board>,
    count: &mut usize,
    movestack: &mut Vec<Vec<u16>>,
    scores: &mut Vec<Vec<i32>>,
    depth: usize,
    ply: usize,
    alpha: i32,
    beta: i32,
    ttable: &mut Transpositiontable,
    history: &mut Vec<u64>,
    halfcount: usize,
    killertable: &mut Vec<Vec<u16>>,
    historytable: &mut Vec<Vec<Vec<usize>>>,
    pruneflags: &mut (u8, bool),
    stopped: &mut bool,
    starttime: Instant,
    timelimit: Duration,
) -> i32 {
    if *stopped {
        return alpha;
    }
    *count += 1;
    //println!("Alpha: {} Beta: {}", alpha, beta);
    let key = positionstack[ply].key;
    if (*count & 4095) == 0 {
        communicate(stopped, starttime, timelimit);
    }
    if ply == piececonstants::MAXPLY - 1 {
        return 0;
    } else if ttable.0[(key & (piececonstants::TTABLEMASK as u64)) as usize].0 != 0
        && pruneflags.0 == 0
    {
        if halfcount <= ply || history.len() == 0 {
            for i in (2..halfcount).step_by(2) {
                if key == positionstack[ply - i].key {
                    //positionstack[ply].print_board();
                    //println!("{}", depth);
                    //ttable.set_value(key, 0, 100, piececonstants::CONTEMPT);
                    return piececonstants::CONTEMPT;
                }
            }
        } else {
            let plycount = halfcount - ply;

            for i in (2..ply).step_by(2) {
                if key == positionstack[ply - i].key {
                    //positionstack[ply].print_board();
                    //ttable.set_value(key, 0, 100, piececonstants::CONTEMPT);

                    return piececonstants::CONTEMPT;
                }
            }
            let mut repcount = 0;
            // set starting value based on ply and side to move?
            let start = 1 - (ply & 1);
            for j in (start..plycount).step_by(2) {
                if key == history[j] {
                    //positionstack[ply].print_board();
                    //println!("{} {}", key, history[j]);
                    //ttable.set_value(key, 0, 100, piececonstants::CONTEMPT);
                    repcount += 1;
                    if repcount == 2 {
                        return piececonstants::CONTEMPT;
                    }
                }
            }
        }
        // i really need to figure this out better
        // handles repitions, considers single repitions as zeros
        /*

        for i in (2..ply).step_by(2) {
            println!("ply: {}, i: {}, Ply - i{}", ply, i, ply - i);
            if key == positionstack[ply - i].key {
                positionstack[ply].print_board();
                ttable.set_value(key, 0, depth, piececonstants::CONTEMPT);
                return piececonstants::CONTEMPT;
            } else {
                continue;
            }
        } */

        //return 0;
    }
    match ttable.read_value(key, depth) {
        Some(score) => return score,
        None => {}
    }
    if depth == 0 {
        let score = quiescent(
            positionstack,
            count,
            movestack,
            scores,
            ply,
            alpha,
            beta,
            ttable,
            history,
            killertable,
            historytable,
            stopped,
            starttime,
            timelimit,
        );
        //let score = positionstack[ply].evaluate();
        if *stopped {
            return alpha;
        }
        ttable.set_value(key, 0, 0, score);
        return score;
    }
    let cut = (beta + 1) & !3; //IBV tomfoolery 199492

    // Null Move Pruning
    let side = positionstack[ply].side.unwrap() as usize;
    let kingattacked = positionstack[ply].is_king_attacked();
    // if pruning allowed in search (hasnt happened yet and not pV)& side has more than just pawns & depth is high enough for reduction & king is not in check
    if !pruneflags.1
        && pruneflags.0 < 2
        && depth >= 3
        && (positionstack[ply].occupancies[side]
            != (positionstack[ply].pieceboards[side * 6]
                & positionstack[ply].pieceboards[side * 6 + 5]))
        && !kingattacked
    {
        positionstack[ply + 1] = positionstack[ply].clone();
        pruneflags.0 += 1;
        null_move(&mut positionstack[ply + 1]); // do nothing
        let score = -zerowindow(
            positionstack,
            count,
            movestack,
            scores,
            depth.saturating_sub(4),
            ply + 1,
            -alpha,
            ttable,
            history,
            halfcount + 1,
            killertable,
            historytable,
            pruneflags,
            stopped,
            starttime,
            timelimit,
        );
        pruneflags.0 -= 1;

        if score >= cut {
            let lb = ((beta + 1) & !3) + 1;
            ttable.set_value(key, 0, depth, lb);
            return lb;
        }
    }
    let mut d = depth;
    let index = generate_moves(&mut positionstack[ply], &mut movestack[ply]);
    // sort moves
    //println!("{:?}   {}", movestack[ply], index);
    // maps first 12 bits to an index in score table
    let tmove = ttable.read_move(positionstack[ply].key);
    // goofy Internal Interative Reductions, helps with move ordering but also turns depth into a lie
    if tmove == None && depth >= piececonstants::INTERNALREDUCTION && !pruneflags.1 {
        d -= 1;
    }
    let mut maxh = 1;
    for m in &movestack[ply][0..index] {
        let from = m.get_initial();
        let to = m.get_final();
        maxh = maxh.max(
            historytable[positionstack[ply].side.unwrap() as usize][from as usize][to as usize],
        );
    }
    for i in 0..index {
        let m = &movestack[ply][i];
        scores[ply][i] = moves::score_move(
            m,
            &positionstack[ply],
            tmove,
            &killertable[ply],
            &historytable[positionstack[ply].side.unwrap() as usize],
            maxh,
        )
    }

    //movestack[ply][0..index].sort_unstable_by_key(|m| scores[4095 & *m as usize]);
    //println!("{:?}    {}", movestack[ply], index);
    let mut alpha2 = (alpha + 1) & !3; // IBV tomfoolery again?

    //create a min heap for moves
    moves::build_min_heap(
        &mut movestack[ply][..index],
        &mut scores[ply][..index],
        index,
    );
    let mut best = movestack[ply][0];

    for i in 0..index {
        // extract next move from heap
        moves::next_move(
            &mut movestack[ply][..index],
            &mut scores[ply][..index],
            index,
            i,
        );
        let m = &movestack[ply][index - i - 1];
        /*if depth == 1 {
            println!("{:?}", &scores[ply][0..index + 1]);
            println!("{:?}", &movestack[ply][0..index + 1]);
        }*/

        //println!("{:?}", &scores[0..index]);

        positionstack[ply + 1] = positionstack[ply].clone();
        let halfcount2 = match m & 61440 {
            0 => halfcount + 1,
            _ => 0,
        };
        //move resets halfmove clock, some kind of special move

        make_move(&mut positionstack[ply + 1], &m);
        //print_bitboard(positionstack[ply + 1].key);
        /*let op_checked = positionstack[ply + 1].is_king_attacked();
        // Check Extensions
        if op_checked {
            d += 1;
        } */
        let mut score;
        if i == 0 {
            score = -negamax(
                positionstack,
                count,
                movestack,
                scores,
                d - 1,
                ply + 1,
                -beta,
                -alpha2,
                ttable,
                history,
                halfcount2,
                killertable,
                historytable,
                pruneflags,
                stopped,
                starttime,
                timelimit,
            );
        } else {
            // LATE MOVE REDUCTIONS
            // 4th or later move, not PV, depth greater than or equal to minimum, not in check, not capture or promotion, move doesnt give check,
            //TODO: update formula for reduction to get better results, also consider history heuristic
            if i > piececonstants::LMRCOUNT
                && !pruneflags.1
                && d >= 2
                && !kingattacked
                && m & 49152 == 0
                && !positionstack[ply + 1].is_king_attacked()
            {
                let r = (((d as f64).log2() * (i as f64).log2() * piececonstants::LMRLEVEL).floor())
                    .min(d as f64 - 0.)
                    .max(1.) as usize;
                //println!("{}", r);
                score = -zerowindow(
                    positionstack,
                    count,
                    movestack,
                    scores,
                    d - r,
                    ply + 1,
                    -alpha2,
                    ttable,
                    history,
                    halfcount2,
                    killertable,
                    historytable,
                    pruneflags,
                    stopped,
                    starttime,
                    timelimit,
                );
            } else {
                score = alpha2 + 4;
            }
            if score > alpha2 {
                score = -zerowindow(
                    positionstack,
                    count,
                    movestack,
                    scores,
                    d - 1,
                    ply + 1,
                    -alpha2,
                    ttable,
                    history,
                    halfcount2,
                    killertable,
                    historytable,
                    pruneflags,
                    stopped,
                    starttime,
                    timelimit,
                );
                if score > alpha2 && score < cut {
                    //println!("{} out of {}", i, index);
                    //println!("awooooga");
                    if *stopped {
                        //rtable.set_value(positionstack[ply + 1].key, false); // reset repition board
                        return alpha;
                    }
                    score = -negamax(
                        positionstack,
                        count,
                        movestack,
                        scores,
                        d - 1,
                        ply + 1,
                        -beta,
                        -alpha2,
                        ttable,
                        history,
                        halfcount2,
                        killertable,
                        historytable,
                        pruneflags,
                        stopped,
                        starttime,
                        timelimit,
                    );
                }
            }
        }
        if *stopped {
            //rtable.set_value(positionstack[ply + 1].key, false); // reset repition board
            return alpha;
        }
        //rtable.set_value(positionstack[ply + 1].key, false); // reset repition board
        //println!(
        //    "Score: {}   Cut: {}   Max: {}  Ply: {}",
        //    score, cut, alpha2, ply
        //);1562949
        if score >= cut {
            // lower bound, cut node
            //let lb = ((score + 1) & !3) + 1; this is fail-soft for beta cutoff
            //println!("{} out of {}", i, index);
            let m = movestack[ply][index - i - 1];
            let lb = ((beta + 1) & !3) + 1;
            ttable.set_value(key, m, depth, lb);
            if m & 16384 == 0 && !positionstack[ply + 1].is_king_attacked() {
                // noncapture moves
                killertable[ply][1] = killertable[ply][0]; // there could be some borrow bullshit happeneing here
                killertable[ply][0] = m.clone();
                historytable[positionstack[ply].side.unwrap() as usize]
                    [m.get_initial() as usize][m.get_final() as usize] += depth * depth;
            }

            return lb;
        } else if score > alpha2 {
            alpha2 = score;
            best = movestack[ply][index - i - 1];
        }
        pruneflags.1 = false; //not PV search anymore

        if *stopped {
            return alpha;
        }
    }

    if index == 0 {
        if positionstack[ply].movemasks[1] != u64::MAX {
            let score = 4 * (piececonstants::MATESCORE + ply as i32);
            ttable.set_value(key, 0, depth, score);
            return score;
        } else {
            ttable.set_value(key, 0, depth, piececonstants::CONTEMPT);
            return piececonstants::CONTEMPT;
        }
    } else if alpha2 == alpha {
        // upper bound, open node
        let upper = ((alpha + 1) & !3) - 1;
        ttable.set_value(key, best, depth, upper);
        upper
    } else {
        // PV node
        alpha2 = (alpha2 + 1) & !3;
        ttable.set_value(key, best, depth, alpha2);
        alpha2
    }
}

//#[inline]
pub fn quiescent(
    positionstack: &mut Vec<Board>,
    count: &mut usize,
    movestack: &mut Vec<Vec<u16>>,
    scores: &mut Vec<Vec<i32>>,
    ply: usize,
    alpha: i32,
    beta: i32,
    ttable: &Transpositiontable,
    history: &mut Vec<u64>,

    killertable: &mut Vec<Vec<u16>>,
    historytable: &mut Vec<Vec<Vec<usize>>>,
    stopped: &mut bool,
    starttime: Instant,
    timelimit: Duration,
) -> i32 {
    *count += 1;
    //println!("Alpha: {} Beta: {}", alpha, beta);
    //let key = positionstack[ply].key;
    if (*count & 4095) == 0 {
        communicate(stopped, starttime, timelimit);
    }
    let eval = positionstack[ply].evaluate();
    let cut = (beta + 1) & !3; //IBV tomfoolery
    if eval >= cut {
        return cut + 1;
    }
    let big_delta = 900 * 4; // queen value, ignore promotion moves for now

    if eval < alpha.saturating_sub(big_delta) || ply == piececonstants::MAXPLY - 1 {
        return alpha;
    }
    let mut alpha2 = match eval > alpha {
        true => eval,
        false => alpha,
    };

    let index = generate_captures(&mut positionstack[ply], &mut movestack[ply]);
    // sort moves
    // IID here?
    //println!("{:?}   {}", movestack[ply], index);
    let tmove = ttable.read_move(positionstack[ply].key);
    let maxh = 0;

    for i in 0..index {
        let m = &movestack[ply][i];
        scores[ply][i] = moves::score_move(
            m,
            &positionstack[ply],
            tmove,
            &killertable[ply],
            &historytable[positionstack[ply].side.unwrap() as usize],
            maxh,
        )
    }

    //movestack[ply][0..index].sort_unstable_by_key(|m| scores[4095 & *m as usize]);
    //println!("{:?}    {}", movestack[ply], index);
    moves::build_min_heap(
        &mut movestack[ply][..index],
        &mut scores[ply][..index],
        index,
    );

    for i in 0..index {
        positionstack[ply + 1] = positionstack[ply].clone();
        //println!("{}", movestack[ply][i].to_uci());
        moves::next_move(
            &mut movestack[ply][..index],
            &mut scores[ply][..index],
            index,
            i,
        );

        make_move(&mut positionstack[ply + 1], &movestack[ply][index - i - 1]);

        //let key2 = board.key;

        let score = -quiescent(
            positionstack,
            count,
            movestack,
            scores,
            ply + 1,
            -beta,
            -alpha2,
            ttable,
            history,
            killertable,
            historytable,
            stopped,
            starttime,
            timelimit,
        );

        if score >= cut {
            // lower bound, cut node
            //let lb = ((score + 1) & !3) + 1; this is fail-soft for beta cutoff

            return beta;
        } else if score > alpha2 {
            alpha2 = score;
        }
        if *stopped {
            return alpha;
        }
    }
    return alpha2;
}
//#[inline]
pub fn zerowindow(
    // TODO: some shit is fucked up here
    positionstack: &mut Vec<Board>,
    count: &mut usize,
    movestack: &mut Vec<Vec<u16>>,
    scores: &mut Vec<Vec<i32>>,
    depth: usize,
    ply: usize,
    beta: i32,
    ttable: &mut Transpositiontable,
    history: &mut Vec<u64>,
    halfcount: usize,
    killertable: &mut Vec<Vec<u16>>,
    historytable: &mut Vec<Vec<Vec<usize>>>,
    pruneflags: &mut (u8, bool),
    stopped: &mut bool,
    starttime: Instant,
    timelimit: Duration,
) -> i32 {
    *count += 1;
    //println!("Alpha: {} Beta: {}", alpha, beta);
    if (*count & 4095) == 0 {
        communicate(stopped, starttime, timelimit);
    }
    if *stopped {
        return beta - 4;
    }
    let key = positionstack[ply].key;
    if ply == piececonstants::MAXPLY - 1 {
        return 0;
    } else if ttable.0[(key & (piececonstants::TTABLEMASK as u64)) as usize].0 != 0
        && pruneflags.0 == 0
    {
        if halfcount <= ply || history.len() == 0 {
            for i in (2..halfcount).step_by(2) {
                if key == positionstack[ply - i].key {
                    //positionstack[ply].print_board();
                    //println!("{}", depth + i);
                    //ttable.set_value(key, 0, 100, piececonstants::CONTEMPT);
                    return piececonstants::CONTEMPT;
                }
            }
        } else {
            let plycount = halfcount - ply;
            for i in (2..ply).step_by(2) {
                if key == positionstack[ply - i].key {
                    //positionstack[ply].print_board();
                    //println!("{}", depth);
                    //ttable.set_value(key, 0, 100, piececonstants::CONTEMPT);
                    return piececonstants::CONTEMPT;
                }
            }
            let mut repcount = 0;
            // set starting value based on ply and side to move?
            //println!("{}", ply);
            let start = 2 - (ply & 1);
            for j in (start..plycount).step_by(2) {
                if key == history[j] {
                    //positionstack[ply].print_board();
                    //println!("{}", depth);
                    //ttable.set_value(key, 0, 100, piececonstants::CONTEMPT);
                    repcount += 1;
                    if repcount == 2 {
                        return piececonstants::CONTEMPT;
                    }
                    return piececonstants::CONTEMPT;
                }
            }
        }
    }
    match ttable.read_value(key, depth) {
        Some(score) => return score,
        None => {}
    }
    if ply == piececonstants::MAXPLY - 1 {
        return 0;
    } else if depth == 0 {
        let score = quiescent(
            positionstack,
            count,
            movestack,
            scores,
            ply,
            beta - 4,
            beta,
            ttable,
            history,
            killertable,
            historytable,
            stopped,
            starttime,
            timelimit,
        );
        //let score = positionstack[ply].evaluate();
        //ttable.set_value(key, 0, 0, ((score + 1) & !3) + 1);
        return score;
    }
    let cut = (beta + 1) & !3; //IBV tomfoolery 199492
    let side = positionstack[ply].side.unwrap() as usize;
    let kingattacked = positionstack[ply].is_king_attacked();

    // if pruning allowed in search (hasnt happened yet and not pV)& side has more than just pawns & depth is high enough for reduction & king is not in check
    if !pruneflags.1
        && pruneflags.0 < 2
        && depth >= 3
        && (positionstack[ply].occupancies[side]
            != (positionstack[ply].pieceboards[side * 6]
                & positionstack[ply].pieceboards[side * 6 + 5]))
        && !kingattacked
    {
        positionstack[ply + 1] = positionstack[ply].clone();
        pruneflags.0 += 1;
        null_move(&mut positionstack[ply + 1]); // do nothing
        let score = -zerowindow(
            positionstack,
            count,
            movestack,
            scores,
            depth.saturating_sub(4),
            ply + 1,
            4 - beta,
            ttable,
            history,
            halfcount,
            killertable,
            historytable,
            pruneflags,
            stopped,
            starttime,
            timelimit,
        );
        pruneflags.0 -= 1;
        if score >= cut {
            //let lb = ((beta + 1) & !3) + 1;
            //ttable.set_value(key, 0, depth, lb);
            return beta;
        }
    }
    let mut d = depth;
    let index = generate_moves(&mut positionstack[ply], &mut movestack[ply]);
    // sort moves
    //println!("{:?}   {}", movestack[ply], index);
    let tmove = ttable.read_move(positionstack[ply].key);
    if tmove == None && depth >= piececonstants::INTERNALREDUCTION && !pruneflags.1 {
        d -= 1;
    }
    let mut maxh = 1;
    for m in &movestack[ply][0..index] {
        let from = m.get_initial();
        let to = m.get_final();
        maxh = maxh.max(
            historytable[positionstack[ply].side.unwrap() as usize][from as usize][to as usize],
        );
    }
    for i in 0..index {
        let m = &movestack[ply][i];
        scores[ply][i] = moves::score_move(
            m,
            &positionstack[ply],
            tmove,
            &killertable[ply],
            &historytable[positionstack[ply].side.unwrap() as usize],
            maxh,
        )
    }

    //movestack[ply][0..index].sort_unstable_by_key(|m| scores[4095 & *m as usize]);
    //println!("{:?}    {}", movestack[ply], index);

    moves::build_min_heap(
        &mut movestack[ply][..index],
        &mut scores[ply][..index],
        index,
    );

    for i in 0..index {
        moves::next_move(
            &mut movestack[ply][..index],
            &mut scores[ply][..index],
            index,
            i,
        );
        let m = &movestack[ply][index - i - 1];
        positionstack[ply + 1] = positionstack[ply].clone();
        let halfcount2 = match m & 61440 {
            0 => halfcount + 1,
            _ => 0,
        };

        //println!("{}", movestack[ply][i].to_uci());
        make_move(&mut positionstack[ply + 1], &m);
        /*let op_checked = positionstack[ply + 1].is_king_attacked();
        // Check Extensions
        if op_checked {
            d += 1;
        } */
        let mut score;
        if i > piececonstants::LMRCOUNT
            && !pruneflags.1
            && d >= 2
            && !kingattacked
            && m & 49152 == 0
            && !positionstack[ply + 1].is_king_attacked()
        {
            let r = (((d as f64).log2() * (i as f64).log2() * piececonstants::LMRLEVEL).floor())
                .min(d as f64 - 0.)
                .max(1.) as usize;
            //println!("{}", r);
            //println!("{} {} {}", d, i, r);
            score = -zerowindow(
                positionstack,
                count,
                movestack,
                scores,
                d - r,
                ply + 1,
                4 - beta,
                ttable,
                history,
                halfcount2,
                killertable,
                historytable,
                pruneflags,
                stopped,
                starttime,
                timelimit,
            );
            if score > cut - 4 && score < cut {
                //CHECK THIS
                score = -zerowindow(
                    positionstack,
                    count,
                    movestack,
                    scores,
                    d - 1,
                    ply + 1,
                    4 - beta,
                    ttable,
                    history,
                    halfcount2,
                    killertable,
                    historytable,
                    pruneflags,
                    stopped,
                    starttime,
                    timelimit,
                );
            }
        } else {
            score = -zerowindow(
                positionstack,
                count,
                movestack,
                scores,
                d - 1,
                ply + 1,
                4 - beta,
                ttable,
                history,
                halfcount2,
                killertable,
                historytable,
                pruneflags,
                stopped,
                starttime,
                timelimit,
            );
        }
        if *stopped {
            //rtable.set_value(positionstack[ply + 1].key, false); // reset repition board
            return beta - 4;
        }
        //let key2 = board.key;

        //rtable.set_value(positionstack[ply + 1].key, false); // reset repition board
        //println!(
        //    "Score: {}   Cut: {}   Max: {}  Ply: {}",
        //    score, cut, alpha2, ply
        //);1562949
        if score >= cut {
            // lower bound, cut node
            //let lb = ((score + 1) & !3) + 1; this is fail-soft for beta cutoff
            let m = movestack[ply][index - i - 1];
            if m & 16384 == 0 && !positionstack[ply + 1].is_king_attacked() {
                // noncapture moves
                killertable[ply][1] = killertable[ply][0]; // there could be some borrow bullshit happeneing here
                killertable[ply][0] = m.clone();
                historytable[positionstack[ply].side.unwrap() as usize]
                    [m.get_initial() as usize][m.get_final() as usize] += depth * depth;
            }
            return beta;
        }
        if *stopped {
            return beta - 4;
        }
    }
    if index == 0 {
        if positionstack[ply].movemasks[1] != u64::MAX {
            let score = 4 * (piececonstants::MATESCORE + ply as i32);
            ttable.set_value(key, 0, depth, score);
            return score;
        } else {
            ttable.set_value(key, 0, depth, piececonstants::CONTEMPT);
            return piececonstants::CONTEMPT;
        }
    }
    return beta - 4;
}
