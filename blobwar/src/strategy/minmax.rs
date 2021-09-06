//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use std::borrow::Borrow;
use std::cmp::min;
use std::cmp::max;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl MinMax {
    fn min_max(&mut self, state: &Configuration, player :bool) -> i8 {
        if self.0 == 0 {//|| state.game_over() {
            return (state.blobs[player as usize].len() - state.blobs[!player as usize].len()) as i8
        }
        self.0 -= 1;
        if state.current_player == player {
            let mut max_eval = -65;
            for mov in state.movements() {
                let conf: Configuration = state.play(mov.borrow());
                max_eval = max(max_eval, self.min_max(conf.borrow(), player));
            }
            max_eval
        } else {
            let mut min_eval = 65;
            for mov in state.movements() {
                let conf: Configuration = state.play(mov.borrow());
                min_eval = min(min_eval, self.min_max(conf.borrow(), player));
            }
            min_eval
        }
    }
}

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut best:i8 = -65;
        let mut same_values_moves = Vec::new();
        let player :bool = state.current_player;
        for mv in state.movements(){
            let conf:Configuration = state.play(mv.borrow());
            let v = self.min_max(conf.borrow(), player);
            if v == best {
                same_values_moves.push(mv);
            } else if v > best {
                same_values_moves.clear();
                same_values_moves.push(mv);
                best = v;
            }
        }
        if same_values_moves.is_empty() {
            None
        } else {
            Some(same_values_moves[0])
        }
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}