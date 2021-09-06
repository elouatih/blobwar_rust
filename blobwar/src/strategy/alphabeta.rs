//! Alpha - Beta algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;
use std::borrow::Borrow;
use std::cmp::min;
use std::cmp::max;
/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl AlphaBeta {
    fn alpha_beta_player(&mut self, state :&Configuration, mut alpha: i8, beta: i8, player :bool) -> i8{
        if self.0 == 0{
            return (state.blobs[player as usize].len() - state.blobs[!player as usize].len()) as i8
        }
        self.0 -= 1;
        for mv in state.movements(){
            let conf: Configuration = state.play(mv.borrow());
            alpha = max(alpha, self.alpha_beta_against(conf.borrow(), alpha, beta, !player));
            if alpha >= beta {
                return beta
            }
        }
        return alpha
    }

    fn alpha_beta_against(&mut self, state :&Configuration, alpha: i8, mut beta: i8, player :bool) -> i8{
        if self.0 == 0{
            return (state.blobs[player as usize].len() - state.blobs[!player as usize].len()) as i8
        }
        self.0 -= 1;
        for mv in state.movements(){
            let conf: Configuration = state.play(mv.borrow());
            beta = min(alpha, self.alpha_beta_player(conf.borrow(), alpha, beta, !player));
            if alpha >= beta {
                return alpha
            }
        }
        return beta
    }
}

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let mut best:i8 = -65;
        let mut candidate_moves = Vec::new();
        let current:bool = state.current_player;
        for mv in state.movements() {
            let conf: Configuration = state.play(mv.borrow());
            let value = self.alpha_beta_player(conf.borrow(), -65, 65, current);
            if value == best {
                candidate_moves.push(mv);
            } else if value > best {
                candidate_moves.clear();
                candidate_moves.push(mv);
                best = value;
            }
        }
        if candidate_moves.is_empty() {
            None
        } else {
            Some(candidate_moves[0])
        }
    }
}
