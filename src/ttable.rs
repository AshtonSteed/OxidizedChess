use crate::moves::MoveStuff;

#[derive(Clone)]
pub struct TableEntry {
    pub key: u64,
    bestmove: u16,
    depth: usize,
    score: i32,
}
impl TableEntry {
    pub fn new() -> TableEntry {
        //crate::print_bitboard(piececonstants::TTABLESIZE as u64 - 1);
        TableEntry {
            key: 0,
            bestmove: 0,
            depth: 1,
            score: 1,
        }
    }
    #[inline(always)]
    pub fn read_value(&self, key: u64, depth: usize) -> Option<i32> {
        // TODO: hash problem :(
        //crate::print_bitboard(key);

        if key == self.key && depth <= self.depth {
            return Some(self.score);
        } else {
            //println!("Dtable: {}, Dstate: {}", entry.2, depth);
            return None;
        }
    }
    #[inline(always)]
    pub fn read_move(&self, key: u64) -> Option<u16> {
        // This needs fixed
        if self.key == key && self.bestmove.is_valid() {
            return Some(self.bestmove);
        } else {
            return None;
        }
    }
    #[inline(always)]
    pub fn set_value(&mut self, key: u64, m: u16, depth: usize, score: i32) {
        let exactscore = score & 3 == 0;

        //current implementation is just stolen from stockfish on god
        if self.bestmove == u16::MAX {
            // max move flags an irreplacable entry, used for repititions
            return;
        }

        if key != self.key || m != 0 {
            self.bestmove = m
        }
        if exactscore || key != self.key || depth >= self.depth {
            //key == entry.0 && depth >= entry.2) || (key != entry.0)
            // depth >= entry.2 && ((key == entry.0) || (key != entry.0 && entry.3 & 3 != 0))
            //(key == entry.0 && depth >= entry.2) || (key != entry.0 && entry.3 & 3 != 0)
            // || (key != entry.0 && entry.3 & 1 == 0) this should check to see if the node is a pv, in theory doesnt replace
            // replaces only if depth is greater than current depth. This helps perserve PV, ill see if theres a better way though
            //println!("collision: {}", entry.0 != key);
            // m as entry.1?
            self.key = key;
            self.depth = depth;
            self.score = score;
        }
    }

    pub fn clear_move(&mut self) {
        self.bestmove = 0;
    }

    pub fn clear_entry(&mut self) {
        self.key = 0;
        self.bestmove = 0;
        self.depth = 1;
        self.score = 1;
    }
}

pub fn hash(key: u64) -> usize {
    key as usize & (crate::piececonstants::TTABLEMASK as usize)
}

// value uses integrated bounds, needs some fancy stuff in negamax but should work good
// this is annoying me, i need to sort out a way for it to perserve PV.
pub fn crude_full(ttable: &Vec<TableEntry>) -> u32 {
    let mut nonzero = 0;

    for i in 0..1000 {
        if ttable[i].key != 0 {
            nonzero += 1;
        }
    }
    nonzero
}
