use crate::{
    engine::{is_draw, Board},
    movegen,
    moves::{self, make_move, null_move, MoveStuff},
    piececonstants,
    ttable::{crude_full, hash, TableEntry},
    uci::communicate,
};
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
    time::{Duration, Instant},
};

pub fn search_position(
    board: &mut Board,
    maxdepth: usize,
    timelimit: Duration,
    ttable: &mut Vec<TableEntry>,
    halfcount: usize,
) -> u16 {
    let mut positionstack = vec![board.clone(); piececonstants::MAXPLY];
    let mut m = 0;

    let mut killertable: [[u16; 2]; 64] = [[0; 2]; piececonstants::MAXPLY];
    let mut historytable: [[[usize; 64]; 64]; 2] = [[[0usize; 64]; 64]; 2];

    let mut movestack: [[u16; 256]; 64] = [[0; 256]; piececonstants::MAXPLY];
    let mut movescores: [[i32; 256]; 64] = [[0i32; 256]; piececonstants::MAXPLY];
    let mut stopped = false;
    let mut pruneflags = (0, true);
    let start = Instant::now();
    let mut count = 0;
    for d in 1..=maxdepth {
        positionstack[0] = board.clone();

        let mut mate = None;

        let mut score = negamax(
            &mut positionstack,
            &mut count,
            &mut movestack,
            &mut movescores,
            d,
            0,
            -100000000,
            100000000,
            ttable,
            halfcount,
            &mut killertable,
            &mut historytable,
            &mut pruneflags,
            0,
            &mut stopped,
            start,
            timelimit,
        );
        if stopped {
            break;
        }
        m = match ttable[board.hash()].read_move(board.key) {
            Some(i) => i,
            None => 0,
        };
        score = match ttable[board.hash()].read_value(board.key, d) {
            Some(i) => i,
            None => score,
        };

        let mut movestring = "".to_string();
        let mut boardcopy = board.clone();
        //for i in 0..d
        for i in 0..d {
            let m2 = &ttable[boardcopy.hash()].read_move(boardcopy.key);
            //println!("{:?}   {:?}", m2, score);
            match m2 {
                Some(j) => {
                    let _ = movegen::generate_moves(&mut boardcopy, &mut movestack[i]);
                    if movestack[i].contains(&j) && j != &0 {
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
                crude_full(ttable),
                d,
                &count,
                start.elapsed().as_millis(),
                movestring
            );
        } else {
            println!(
                "info score cp {} hashfull {} depth {} nodes {} time {} pv {} ",
                ((score + 1) & !3) / 4,
                crude_full(ttable),
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

    //let m = ttable[board.hash()].read_move(board.key);

    //let m = ttable[board.hash()].read_move(board.key);

    m
}
/*
struct Search<'a> {
    positionstack: Vec<Board>,
    movestack: [[u16; 256]; 64],
    scores: [[i32; 256]; 64],
    killertable: [[u16; 2]; 64],
    historytable: [[[usize; 64]; 64]; 2],
    count: usize,
    nullmoves: usize,
    root: bool,
    ttable: RefMut<'a, Vec<TableEntry>>,
    stopped: bool,
    starttime: Instant,
    timelimit: Duration,
}
impl<'a> Search<'a> {
    fn new(
        board: &'a mut Board,
        timelimit: Duration,
        ttable: &'a Rc<RefCell<Vec<TableEntry>>>,
        historytable: &'a mut [[[usize; 64]; 64]; 2],
        killertable: &'a mut [[u16; 2]; 64],
        halfcount: usize,
    ) -> Search<'a> {
        let mut positionstack = vec![board.clone(); piececonstants::MAXPLY];
        let mut m = 0;

        let mut movestack: [[u16; 256]; 64] = [[0; 256]; piececonstants::MAXPLY];
        let mut movescores: [[i32; 256]; 64] = [[0i32; 256]; piececonstants::MAXPLY];

        let starttime = Instant::now();
        return Search {
            positionstack,
            movestack,
            killertable,
            historytable,
            timelimit,
            starttime,
            ttable: ttable.borrow_mut(),
            scores: movescores,
            stopped: false,
            root: true,
            count: 0,
            nullmoves: 0,
        };
    }
    fn communicate(&mut self) {
        if self.starttime.elapsed() >= self.timelimit {
            self.stopped = true;
        }
    }
    fn start_search(&mut self, board: &'a mut Board, maxdepth: usize, halfcount: usize) -> u16 {
        let mut m = 0;
        for d in 1..=maxdepth {
            self.positionstack[0] = board.clone();
            let mut mate = None;

            let mut score = self.negamax(-100000000, 100000000, d, 0, halfcount, 0);
            if self.stopped {
                break;
            }
            m = match self.ttable[board.hash()].read_move(board.key) {
                Some(i) => i,
                None => 0,
            };
            score = match self.ttable[board.hash()].read_value(board.key, d) {
                Some(i) => i,
                None => score,
            };

            let mut movestring = "".to_string();
            let mut boardcopy = board.clone();
            //for i in 0..d
            for i in 0..d {
                let m2 = &self.ttable[boardcopy.hash()].read_move(boardcopy.key);
                //println!("{:?}   {:?}", m2, score);
                match m2 {
                    Some(j) => {
                        let _ = movegen::generate_moves(&mut boardcopy, &mut self.movestack[i]);
                        if self.movestack[i].contains(&j) && j != &0 {
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
                    crude_full(&self.ttable),
                    d,
                    &self.count,
                    self.starttime.elapsed().as_millis(),
                    movestring
                );
            } else {
                println!(
                    "info score cp {} hashfull {} depth {} nodes {} time {} pv {} ",
                    ((score + 1) & !3) / 4,
                    crude_full(&self.ttable),
                    d,
                    &self.count,
                    self.starttime.elapsed().as_millis(),
                    movestring
                );
            }

            if self.stopped {
                break;
            }
        }
        println!("info nodes {}", self.count);

        let mut boardcopy = board.clone();
        make_move(&mut boardcopy, &m);

        m
    }

    fn negamax(
        &mut self,
        alpha: i32,
        beta: i32,
        depth: usize,
        ply: usize,
        halfcount: usize,
        exmove: u16,
    ) -> i32 {
        self.count += 1;
        if (self.count & 4095) == 0 {
            self.communicate();
        } else if ply == piececonstants::MAXPLY - 1 {
            return 0;
        }
        let key = self.positionstack[ply].key;
        let tprobe = &self.ttable[hash(key)];
        if tprobe.key != 0
            && ply != 0
            && self.nullmoves == 0
            && is_draw(&self.positionstack, ply, halfcount)
        {
            //println!("1");
            return piececonstants::draw_score(ply);
        }
        let cut = (beta + 1) & !3; //IBV tomfoolery
        let mut alpha2 = (alpha + 1) & !3; // IBV tomfoolery again?
        let tscore = tprobe.read_value(key, depth);
        let tmove = tprobe.read_move(key);
        match tscore {
            Some(score) => {
                if {
                    match score & 3 {
                        0 => true,
                        1 => score >= cut,
                        3 => score <= alpha2,
                        _ => false,
                    }
                } & !self.root
                {
                    return score;
                }
            }

            None => {}
        }
        self.root = false;

        if depth == 0 || self.stopped {
            let score = 0; //self.q()
                           //println!("{}", score);

            //ttable.set_value(key, 0, 0, score);
            return score;
        }

        // Null Move Pruning
        let side = self.positionstack[ply].side.unwrap() as usize;
        let kingattacked = self.positionstack[ply].is_king_attacked();
        // if pruning allowed in search (hasnt happened yet and not pV)& side has more than just pawns & depth is high enough for reduction & king is not in check
        if self.nullmoves < 2
            && depth >= 3
            && (self.positionstack[ply].occupancies[side]
                != (self.positionstack[ply].pieceboards[side * 6]
                    | self.positionstack[ply].pieceboards[side * 6 + 5]))
            && !kingattacked
        {
            self.positionstack[ply + 1] = self.positionstack[ply].clone();
            self.nullmoves += 1;
            null_move(&mut self.positionstack[ply + 1]); // do nothing
            let score = 0; //-self.zerowindow()

            self.nullmoves -= 1;

            if score >= cut {
                let lb = ((beta + 1) & !3) + 1;
                //ttable.set_value(key, 0, depth, lb);

                return lb;
            }
        }
        let mut d = depth;

        // sort moves
        // Set move from Transposition Table

        let mut movephase: usize = match tmove {
            Some(m) => {
                self.movestack[ply][0] = m;
                0
            }
            None => 1,
        };

        let side = self.positionstack[ply].side.unwrap() as usize;

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
                &mut self.positionstack[ply],
                &mut self.movestack[ply],
                &mut self.scores[ply],
                &mut index,
                &mut totalmoves,
                &mut i,
                &mut movephase,
                &mut badcaps,
                &mut badindex,
                &self.killertable[ply],
                &self.historytable[side],
                true,
                tmove,
            );

            if m == 0 {
                break;
            }
            if m == exmove {
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
            if m == tmove.unwrap_or(0) && exmove == 0 && depth >= 4 && tscore.unwrap_or(0) & 3 == 1
            {
                let singularbeta = tscore.unwrap() - 1 - 4 * d as i32;
                let singled = d / 2;
                let singleex = m;
                //positionstack[ply + 1] = positionstack[ply].clone();
                let score = 0; //zerowindow();
                if score < singularbeta {
                    d += 1;
                } else if singularbeta >= beta {
                    return singularbeta;
                }
            }

            //println!("{:?}", &scores[0..index]);

            self.positionstack[ply + 1] = self.positionstack[ply];
            let halfcount2 = match m & 61440 {
                0 => halfcount + 1,
                _ => 0,
            };
            //move resets halfmove clock, some kind of special move

            make_move(&mut self.positionstack[ply + 1], &m);
            //print_bitboard(positionstack[ply + 1].key);
            /*let op_checked = positionstack[ply + 1].is_king_attacked();
            // Check Extensions
            if op_checked {
                d += 1;
            } */
            let mut score;
            if j == 0 {
                score = -self.negamax(-beta, -alpha2, d - 1, ply + 1, halfcount2, exmove);
            } else {
                score = 0; //-zerowindow(d - 1, ply + 1, -alpha2, z);
                if score > alpha2 && score < cut {
                    score = -self.negamax(-beta, -alpha2, d - 1, ply + 1, halfcount2, exmove);
                }
            }
            if self.stopped {
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
                    if m & 16384 == 0 && !self.positionstack[ply + 1].is_king_attacked() {
                        // noncapture moves
                        self.killertable[ply][1] = self.killertable[ply][0]; // there could be some borrow bullshit happeneing here
                        self.killertable[ply][0] = m.clone();
                        self.historytable[self.positionstack[ply].side.unwrap() as usize]
                            [m.get_initial() as usize][m.get_final() as usize] += depth * depth;
                    }
                    break;
                }
            }
            //not PV search anymore
        }
        let tset = &mut self.ttable[hash(key)];
        if j == 0 {
            if kingattacked {
                let score = 4 * (piececonstants::MATESCORE + ply as i32);
                tset.set_value(key, 0, depth, score);
                tset.clear_move();
                return score;
            } else {
                //println!("2");
                let score = piececonstants::draw_score(ply);
                tset.set_value(key, 0, depth, score);
                tset.clear_move();
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

        tset.set_value(key, best, depth, alpha2);
        alpha2
    }
}
*///Positionstack: Vector of board states after search initiated
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
    positionstack: &mut Vec<Board>,
    count: &mut usize,
    movestack: &mut [[u16; 256]; 64],
    scores: &mut [[i32; 256]; 64],
    depth: usize,
    ply: usize,
    alpha: i32,
    beta: i32,
    ttable: &mut Vec<TableEntry>,
    halfcount: usize,
    killertable: &mut [[u16; 2]; 64],
    historytable: &mut [[[usize; 64]; 64]; 2],
    pruneflags: &mut (u8, bool),
    exmove: u16,
    stopped: &mut bool,
    starttime: Instant,
    timelimit: Duration,
) -> i32 {
    *count += 1;
    //println!("Alpha: {} Beta: {}", alpha, beta);
    let key = positionstack[ply].key;
    if (*count & 4095) == 0 {
        communicate(stopped, starttime, timelimit);
    }
    if ply == piececonstants::MAXPLY - 1 {
        return 0;
    }
    let tprobe = &ttable[hash(key)];
    if tprobe.key != 0 && ply != 0 && pruneflags.0 == 0 && is_draw(positionstack, ply, halfcount) {
        //println!("1");
        return piececonstants::draw_score(ply);
    }
    let cut = (beta + 1) & !3; //IBV tomfoolery
    let mut alpha2 = (alpha + 1) & !3; // IBV tomfoolery again?
    let tscore = tprobe.read_value(key, depth);
    let tmove = tprobe.read_move(key);
    match tscore {
        Some(score) => {
            if {
                match score & 3 {
                    0 => true,
                    1 => score >= cut,
                    3 => score <= alpha2,
                    _ => false,
                }
            } & !pruneflags.1
            {
                return score;
            }
        }

        None => {}
    }
    pruneflags.1 = false;

    if depth == 0 || *stopped {
        let score = quiescent(
            positionstack,
            count,
            movestack,
            scores,
            ply,
            0,
            alpha,
            beta,
            ttable,
            killertable,
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
    let side = positionstack[ply].side.unwrap() as usize;
    let kingattacked = positionstack[ply].is_king_attacked();
    // if pruning allowed in search (hasnt happened yet and not pV)& side has more than just pawns & depth is high enough for reduction & king is not in check
    if pruneflags.0 < 2
        && depth >= 3
        && (positionstack[ply].occupancies[side]
            != (positionstack[ply].pieceboards[side * 6]
                | positionstack[ply].pieceboards[side * 6 + 5]))
        && !kingattacked
    {
        positionstack[ply + 1] = positionstack[ply];
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
            halfcount + 1,
            killertable,
            historytable,
            pruneflags,
            0,
            stopped,
            starttime,
            timelimit,
        );

        pruneflags.0 -= 1;

        if score >= cut {
            let lb = ((beta + 1) & !3) + 1;
            //ttable.set_value(key, 0, depth, lb);

            return lb;
        }
    }
    let mut d = depth;

    // sort moves
    // Set move from Transposition Table

    let mut movephase: usize = match tmove {
        Some(m) => {
            movestack[ply][0] = m;
            0
        }
        None => 1,
    };

    let mut maxh = 1;

    //movestack[ply][0..index].sort_unstable_by_key(|m| scores[4095 & *m as usize]);
    //println!("{:?}    {}", movestack[ply], index);

    let side = positionstack[ply].side.unwrap() as usize;

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
            &mut positionstack[ply],
            &mut movestack[ply],
            &mut scores[ply],
            &mut index,
            &mut totalmoves,
            &mut i,
            &mut movephase,
            &mut badcaps,
            &mut badindex,
            &killertable[ply],
            &historytable[side],
            true,
            tmove,
        );

        if m == 0 {
            break;
        }
        if m == exmove {
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
        if m == tmove.unwrap_or(0) && exmove == 0 && depth >= 4 && tscore.unwrap_or(0) & 3 == 1 {
            let singularbeta = tscore.unwrap() - 1 - 4 * d as i32;
            let singled = d / 2;
            let singleex = m;
            //positionstack[ply + 1] = positionstack[ply].clone();
            let score = zerowindow(
                positionstack,
                count,
                movestack,
                scores,
                singled,
                ply,
                singularbeta,
                ttable,
                halfcount,
                killertable,
                historytable,
                pruneflags,
                singleex,
                stopped,
                starttime,
                timelimit,
            );
            if score < singularbeta {
                d += 1;
            } else if singularbeta >= beta {
                return singularbeta;
            }
        }

        //println!("{:?}", &scores[0..index]);

        positionstack[ply + 1] = positionstack[ply];
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
        if j == 0 {
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
                halfcount2,
                killertable,
                historytable,
                pruneflags,
                0,
                stopped,
                starttime,
                timelimit,
            );
        } else {
            score = -zerowindow(
                positionstack,
                count,
                movestack,
                scores,
                d - 1,
                ply + 1,
                -alpha2,
                ttable,
                halfcount2,
                killertable,
                historytable,
                pruneflags,
                0,
                stopped,
                starttime,
                timelimit,
            );
            if score > alpha2 && score < cut {
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
                    halfcount2,
                    killertable,
                    historytable,
                    pruneflags,
                    0,
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
                if m & 16384 == 0 && !positionstack[ply + 1].is_king_attacked() {
                    // noncapture moves
                    killertable[ply][1] = killertable[ply][0]; // there could be some borrow bullshit happeneing here
                    killertable[ply][0] = m;
                    historytable[positionstack[ply].side.unwrap() as usize]
                        [m.get_initial() as usize][m.get_final() as usize] += depth * depth;
                }
                break;
            }
        }
        //not PV search anymore
    }
    let tset = &mut ttable[hash(key)];
    if j == 0 {
        if kingattacked {
            let score = 4 * (piececonstants::MATESCORE + ply as i32);
            tset.set_value(key, 0, depth, score);
            tset.clear_move();
            return score;
        } else {
            //println!("2");
            let score = piececonstants::draw_score(ply);
            tset.set_value(key, 0, depth, score);
            tset.clear_move();
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

    tset.set_value(key, best, depth, alpha2);
    alpha2
}

#[inline]
pub fn quiescent(
    positionstack: &mut Vec<Board>,
    count: &mut usize,
    movestack: &mut [[u16; 256]; 64],
    scores: &mut [[i32; 256]; 64],
    ply: usize,
    qply: usize,
    alpha: i32,
    beta: i32,
    ttable: &mut Vec<TableEntry>,
    killertable: &mut [[u16; 2]; 64],
    historytable: &mut [[[usize; 64]; 64]; 2],
    stopped: &mut bool,
    starttime: Instant,
    timelimit: Duration,
) -> i32 {
    *count += 1;
    //println!("Alpha: {} Beta: {}", alpha, beta);
    let key = positionstack[ply].key;
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
    let in_check = positionstack[ply].is_king_attacked();
    let eval = positionstack[ply].evaluate(ply);
    let cut = (beta + 1) & !3; //IBV tomfoolery
    let mut alpha2 = (alpha + 1) & !3;
    if eval >= cut && (!in_check || qply <= 0) {
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
    let tmove = ttable[hash(key)].read_move(key);

    let mut movephase: usize = match tmove {
        Some(m) => {
            if m.get_extra() & 4 != 0 {
                movestack[ply][0] = m;
                0
            } else {
                1
            }
        }
        None => 1,
    };

    //movestack[ply][0..index].sort_unstable_by_key(|m| scores[4095 & *m as usize]);
    //println!("{:?}    {}", movestack[ply], index);

    let side = positionstack[ply].side.unwrap() as usize;

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
            &mut positionstack[ply],
            &mut movestack[ply],
            &mut scores[ply],
            &mut index,
            &mut totalmoves,
            &mut i,
            &mut movephase,
            &mut badcaps,
            &mut badindex,
            &killertable[ply],
            &historytable[side],
            in_check && (qply <= 0),
            tmove,
        );

        if m == 0 {
            break;
        }
        /*if i == 0 {
            bestmove = m;
        }*/
        j += 1;

        positionstack[ply + 1] = positionstack[ply];

        make_move(&mut positionstack[ply + 1], &m);

        //let key2 = board.key;

        let score = -quiescent(
            positionstack,
            count,
            movestack,
            scores,
            ply + 1,
            qply + 1,
            -beta,
            -alpha2,
            ttable,
            killertable,
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
    if j == 0 && ((in_check && qply <= 0) || !movegen::has_quiets(&mut positionstack[ply])) {
        if in_check {
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
    positionstack: &mut Vec<Board>,
    count: &mut usize,
    movestack: &mut [[u16; 256]; 64],
    scores: &mut [[i32; 256]; 64],
    depth: usize,
    ply: usize,
    beta: i32,
    ttable: &mut Vec<TableEntry>,

    halfcount: usize,
    killertable: &mut [[u16; 2]; 64],
    historytable: &mut [[[usize; 64]; 64]; 2],
    pruneflags: &mut (u8, bool),
    exmove: u16,
    stopped: &mut bool,
    starttime: Instant,
    timelimit: Duration,
) -> i32 {
    *count += 1;
    //println!("Alpha: {} Beta: {}", alpha, beta);
    if (*count & 4095) == 0 {
        communicate(stopped, starttime, timelimit);
    }

    let key = positionstack[ply].key;
    if ply == piececonstants::MAXPLY - 1 {
        return 0;
    }
    let tprobe = &ttable[hash(key)];

    if tprobe.key != 0 && pruneflags.0 == 0 && is_draw(positionstack, ply, halfcount) {
        //println!("3");
        return piececonstants::draw_score(ply);
    }
    let tscore = tprobe.read_value(key, depth);
    let tmove = tprobe.read_move(key);
    match tscore {
        Some(score) => {
            if exmove == 0 && {
                match score & 3 {
                    0 => true,
                    1 => score >= beta,
                    3 => score <= beta - 4,
                    _ => false,
                }
            } & !pruneflags.1
            {
                return score;
            }
        }

        None => {}
    }

    if depth == 0 {
        let score = quiescent(
            positionstack,
            count,
            movestack,
            scores,
            ply,
            0,
            beta - 4,
            beta,
            ttable,
            killertable,
            historytable,
            stopped,
            starttime,
            timelimit,
        );
        //let score = positionstack[ply].evaluate();
        //(key, 0, 0, score + 1);
        return score;
    }
    let mut cut = (beta + 1) & !3; //IBV tomfoolery 199492
    let side = positionstack[ply].side.unwrap() as usize;
    let kingattacked = positionstack[ply].is_king_attacked();

    // if pruning allowed in search (hasnt happened yet and not pV)& side has more than just pawns & depth is high enough for reduction & king is not in check
    if pruneflags.0 < 2
        && exmove == 0
        && depth >= 3
        && (positionstack[ply].occupancies[side]
            != (positionstack[ply].pieceboards[side * 6]
                | positionstack[ply].pieceboards[side * 6 + 5]))
        && !kingattacked
    {
        positionstack[ply + 1] = positionstack[ply];
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
            halfcount,
            killertable,
            historytable,
            pruneflags,
            0,
            stopped,
            starttime,
            timelimit,
        );
        pruneflags.0 -= 1;
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
    let mut movephase: usize = match tmove {
        Some(m) => {
            movestack[ply][0] = m;
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
            &mut positionstack[ply],
            &mut movestack[ply],
            &mut scores[ply],
            &mut index,
            &mut totalmoves,
            &mut i,
            &mut movephase,
            &mut badcaps,
            &mut badindex,
            &killertable[ply],
            &historytable[side],
            true,
            tmove,
        );

        if m == 0 {
            break;
        }
        if m == exmove {
            continue;
        }
        j += 1;

        /* Singular Extensions:
        Need to mess with the criteria for this bullshit
         */
        // for tmove with no excluded move, suwfficient depth, and lower bound ttable
        if m == tmove.unwrap_or(0) && exmove == 0 && depth >= 4 && tscore.unwrap_or(0) & 3 == 1 {
            let singularbeta = tscore.unwrap() - 1 - 4 * d as i32;
            let singled = d / 2;
            let singleex = m;
            //positionstack[ply + 1] = positionstack[ply].clone();
            let score = zerowindow(
                positionstack,
                count,
                movestack,
                scores,
                singled,
                ply,
                singularbeta,
                ttable,
                halfcount + 1,
                killertable,
                historytable,
                pruneflags,
                singleex,
                stopped,
                starttime,
                timelimit,
            );
            if score < singularbeta {
                d += 1;
            } else if singularbeta >= beta {
                return singularbeta;
            }
        }

        positionstack[ply + 1] = positionstack[ply];
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
        if d >= 2 && !kingattacked && m & 49152 == 0 && !positionstack[ply + 1].is_king_attacked() {
            let r = (((d as f64).log2() * (j as f64).log2() * piececonstants::LMRLEVEL).floor())
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
                halfcount2,
                killertable,
                historytable,
                pruneflags,
                0,
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
                    halfcount2,
                    killertable,
                    historytable,
                    pruneflags,
                    0,
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
                halfcount2,
                killertable,
                historytable,
                pruneflags,
                0,
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

            if m & 16384 == 0 && !positionstack[ply + 1].is_king_attacked() {
                // noncapture moves
                killertable[ply][1] = killertable[ply][0]; // there could be some borrow bullshit happeneing here
                killertable[ply][0] = m;
                historytable[positionstack[ply].side.unwrap() as usize]
                    [m.get_initial() as usize][m.get_final() as usize] += depth * depth;
            }
            //ttable.set_value(positionstack[ply].key, m, depth, lb);
            break;
        }
    }
    let tset = &mut ttable[hash(key)];
    if j == 0 {
        if kingattacked {
            let score = 4 * (piececonstants::MATESCORE + ply as i32);
            tset.set_value(key, 0, depth, score);
            tset.clear_move();
            return score;
        } else {
            //println!("4");
            let score = piececonstants::draw_score(ply);
            tset.set_value(key, 0, depth, score);
            tset.clear_move();
            return score;
        }
    }
    if cut == (beta + 1) & !3 {
        cut -= 5;
    }
    tset.set_value(key, bestmove, depth, cut);
    return cut;
}
