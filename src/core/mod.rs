pub mod neighbour_space;
pub mod rng;
pub mod stop_condition;
pub mod tree_space;

use std::fmt::Debug;

pub use crate::core::rng::*;
pub use crate::core::stop_condition::*;

/// Represents the trait for the type of the objective,
/// will always deal with minimization problems
/// unbounded < best <= value <= worst < unfeas
pub trait Objective: Clone + Copy + Debug + PartialOrd + Ord + Into<f64> {
    /// returns a value that represents an unfeasible objective
    fn unfeas() -> Self;
    /// returns a value that represents an unbounded objective
    fn unbounded() -> Self;
    /// returns true if feasible
    fn is_feas(&self) -> bool;
    /// returns true if bounded
    fn is_bounded(&self) -> bool;
}

/// Represents a problem
pub trait Problem: Clone + Debug {
    type Solution: Clone + Debug;
    type Obj: Objective;
    fn objective(&self, sol: &Self::Solution) -> Self::Obj;
    fn is_feasible(&self, sol: &Self::Solution) -> bool;
}

/// Represents a generator for a problem with a certain distribution
pub trait ProblemGenerator<P: Problem> {
    fn generate<R: rng::Rng>(rng: &mut R) -> P;
}

pub struct SolverEvent<P: Problem> {
    pub time: std::time::Duration,
    pub primal_bound: P::Obj,
    pub dual_bound: P::Obj,
}
/// Represents a solver for a problem
pub trait Solver<P: Problem> {
    // TODO: also return a list of primal/dual bounds with their timestamps, add stopping
    // conditions
    fn solve<T: Timer, S: StopCondition<P::Obj>>(
        &mut self,
        p: P,
        timer: T,
        stop: S,
    ) -> (Option<P::Solution>, Vec<SolverEvent<P>>);
}
