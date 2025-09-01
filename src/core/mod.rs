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

pub struct SolverEvent<T: Timer, P: Problem> {
    pub time: T::Instant,
    pub it: u64,
    pub primal_bound: P::Obj,
    pub dual_bound: P::Obj,
}
pub struct SolverStats<T: Timer, P: Problem> {
    pub its: u64,
    pub events: Vec<SolverEvent<T, P>>,
    pub start_time: T::Instant,
    pub end_time: Option<T::Instant>,
    pub timer: T,
}
impl<T: Timer, P: Problem> Default for SolverStats<T, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Timer, P: Problem> SolverStats<T, P> {
    pub fn duration(&self) -> Option<std::time::Duration> {
        self.end_time.map(|x| x - self.start_time)
    }
    pub fn new() -> Self {
        let timer = T::default();
        Self {
            its: 0,
            events: Vec::new(),
            start_time: timer.time(),
            end_time: None,
            timer,
        }
    }
    pub fn add_primal_bound(&mut self, pb: P::Obj) {
        self.events.push(SolverEvent {
            time: self.timer.time(),
            it: self.its,
            primal_bound: pb,
            dual_bound: self
                .events
                .last()
                .map_or(P::Obj::unbounded(), |x| x.dual_bound),
        });
    }
    pub fn add_dual_bound(&mut self, db: P::Obj) {
        self.events.push(SolverEvent {
            time: self.timer.time(),
            it: self.its,
            primal_bound: self
                .events
                .last()
                .map_or(P::Obj::unbounded(), |x| x.primal_bound),
            dual_bound: db,
        });
    }
    pub fn add_bounds(&mut self, pb: P::Obj, db: P::Obj) {
        self.events.push(SolverEvent {
            time: self.timer.time(),
            it: self.its,
            primal_bound: pb,
            dual_bound: db,
        });
    }
    pub fn iter(&mut self) {
        self.its += 1;
    }
    pub fn finish(&mut self) {
        self.events.push(SolverEvent {
            time: self.timer.time(),
            it: self.its,
            primal_bound: self
                .events
                .last()
                .map_or(P::Obj::unbounded(), |x| x.primal_bound),
            dual_bound: self
                .events
                .last()
                .map_or(P::Obj::unbounded(), |x| x.dual_bound),
        });
        self.end_time = Some(self.timer.time());
    }
    pub fn total_time(&self) -> Option<std::time::Duration> {
        self.end_time.map(|et| et - self.start_time)
    }
}

/// Represents a solver for a problem
pub trait Solver<P: Problem> {
    fn solve<T: Timer, S: StopCondition<P::Obj>>(
        &mut self,
        p: P,
        stop: S,
    ) -> (Option<P::Solution>, SolverStats<T, P>);
}
