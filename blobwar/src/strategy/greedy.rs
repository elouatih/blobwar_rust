//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use std::fmt;
use std::borrow::Borrow;


/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        if state.movements().next().is_some() {
            let mut optimal_v: i8 = 127;
            let mut optimal_mv = state.movements().next();
            for mv in state.movements() {
                let w = state.play(mv.borrow()).skip_play().value();
                if w < optimal_v {
                    optimal_mv = Some(mv);
                    optimal_v = w;
                }
            }
            return optimal_mv;
        } else {
            return None;
        }
    }
}
