pub mod neighbour_space;
pub mod rng;
pub mod stop_condition;
pub mod tree_space;

use std::fmt::Debug;

pub use crate::core::rng::*;
pub use crate::core::stop_condition::*;

/// Represents the trait for the type of the objective,
/// will always deal with minimization problems
/// unbounded < any other value < unfeas
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
    /// Solution space
    type Sol: Clone + Debug;
    type Obj: Objective;
    /// function to minimize
    fn obj(&self, sol: &Self::Sol) -> Self::Obj;
    /// should return false iff f returns unfeas
    fn is_feasible(&self, sol: &Self::Sol) -> bool;
}

pub trait Reduction<P: Problem>: Problem {
    fn reduce_from(&self, p: &P) -> Self;
    fn lift_solution_to(&self, sol: Self::Sol) -> P::Sol;
}

/// Represents a generator for a problem with a certain distribution
pub trait ProblemGenerator<P: Problem> {
    fn generate<R: rng::Rng>(rng: &mut R) -> P;
}

/// Used both to pass solutions to a solver
/// and to retrieve solutions from it
pub trait SolutionKeeper<P: Problem> {
    /// should be called every time a new solution is found, even if worse
    fn add_solution<F: FnOnce() -> P::Sol>(&mut self, sol_fn: F, obj: P::Obj);
    /// should be called every time a new global dual bound is found, even if worse
    fn add_dual_bound(&mut self, db: P::Obj);
    /// should be called at each iteration of the solver
    fn iter(&mut self);
    /// returns the best solution found so far, if any
    fn best_solution(&self) -> Option<(P::Sol, P::Obj)>;
}

pub struct SimpleSolutionKeeper<P: Problem> {
    pub best_sol: Option<(P::Sol, P::Obj)>,
    pub dual_bound: P::Obj,
}
impl<P: Problem> SolutionKeeper<P> for SimpleSolutionKeeper<P> {
    fn add_solution<F: FnOnce() -> P::Sol>(&mut self, sol_fn: F, obj: P::Obj) {
        if obj < self.best_sol.as_ref().map_or(P::Obj::unbounded(), |x| x.1) {
            self.best_sol = Some((sol_fn(), obj));
        }
    }
    fn add_dual_bound(&mut self, db: P::Obj) {
        if db > self.dual_bound {
            self.dual_bound = db;
        }
    }
    fn best_solution(&self) -> Option<(P::Sol, P::Obj)> {
        self.best_sol.clone()
    }
    fn iter(&mut self) {}
}
impl<P: Problem> Default for SimpleSolutionKeeper<P> {
    fn default() -> Self {
        Self {
            best_sol: None,
            dual_bound: P::Obj::unbounded(),
        }
    }
}

pub struct SolverEvent<T: Timer, P: Problem> {
    pub time: T::Instant,
    pub it: u64,
    pub primal_bound: Option<(P::Sol, P::Obj)>,
    pub dual_bound: Option<P::Obj>,
}
pub struct SolverStats<T: Timer, P: Problem, SK: SolutionKeeper<P>> {
    pub its: u64,
    pub events: Vec<SolverEvent<T, P>>,
    pub start_time: T::Instant,
    pub last_time: T::Instant,
    pub timer: T,
    pub underlying: SK,
}
impl<T: Timer, P: Problem, SK: SolutionKeeper<P>> SolutionKeeper<P> for SolverStats<T, P, SK> {
    fn add_solution<F: FnOnce() -> P::Sol>(&mut self, sol_fn: F, obj: P::Obj) {
        let sol = sol_fn();
        self.events.push(SolverEvent {
            time: self.timer.time(),
            it: self.its,
            primal_bound: Some((sol.clone(), obj)),
            dual_bound: None,
        });
        self.underlying.add_solution(|| sol, obj);
    }
    fn add_dual_bound(&mut self, db: P::Obj) {
        self.events.push(SolverEvent {
            time: self.timer.time(),
            it: self.its,
            primal_bound: None,
            dual_bound: Some(db),
        });
        self.underlying.add_dual_bound(db);
    }
    fn best_solution(&self) -> Option<(P::Sol, P::Obj)> {
        self.underlying.best_solution()
    }
    fn iter(&mut self) {
        self.its += 1;
        self.last_time = self.timer.time();
        self.underlying.iter();
    }
}
impl<T: Timer, P: Problem, SK: SolutionKeeper<P>> SolverStats<T, P, SK> {
    pub fn new(underlying: SK) -> Self {
        let timer = T::default();
        Self {
            its: 0,
            events: Vec::new(),
            start_time: timer.time(),
            last_time: timer.time(),
            timer,
            underlying,
        }
    }
}

/// Represents a solver for a problem
pub trait Solver<P: Problem, SK: SolutionKeeper<P>> {
    fn solve<T: Timer, S: StopCondition<P::Obj>>(&mut self, p: P, sk: &mut SK, stop: S);
}
