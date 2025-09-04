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
    fn reduce_from(p: &P) -> Self;
    fn lift_solution_to(&self, sol: Self::Sol) -> P::Sol;
    fn lift_obj_to(&self, obj: Self::Obj) -> P::Obj;
}

/// Represents a generator for a problem with a certain distribution
pub trait ProblemGenerator<P: Problem>: Clone {
    fn generate<R: rng::Rng>(&self, rng: &mut R) -> P;
}

/// Used both to pass solutions to a solver
/// and to retrieve solutions from it
pub trait SolutionKeeper<P: Problem> {
    /// this or `add_solution_fn` should be called every time a new solution is found, even if worse
    fn add_solution(&mut self, sol: &P::Sol, obj: P::Obj);
    /// this or `add_solution` should be called every time a new solution is found, even if worse
    /// useful when the solution is expensive to compute and should be computed only if needed
    fn add_solution_fn<F: FnOnce() -> P::Sol>(&mut self, f: F, obj: P::Obj);
    /// should be called every time a new global dual bound is found, even if worse
    fn add_dual_bound(&mut self, db: P::Obj);
    /// should be called at each iteration of the solver
    fn iter(&mut self);
    /// returns the best solution found so far, if any
    fn best_solution(&self) -> Option<(P::Sol, P::Obj)>;
    fn best_obj(&self) -> P::Obj {
        self.best_solution()
            .as_ref()
            .map_or(P::Obj::unfeas(), |x| x.1)
    }
}

// TODO: reduction between problem -> automatic conversion between solution keepers

pub struct SimpleSolutionKeeper<P: Problem> {
    pub best_sol: Option<(P::Sol, P::Obj)>,
    pub dual_bound: P::Obj,
}
impl<P: Problem> SolutionKeeper<P> for SimpleSolutionKeeper<P> {
    fn add_solution(&mut self, sol: &P::Sol, obj: P::Obj) {
        if obj < self.best_obj() {
            self.best_sol = Some((sol.clone(), obj));
        }
    }
    fn add_solution_fn<F: FnOnce() -> P::Sol>(&mut self, f: F, obj: P::Obj) {
        if obj < self.best_obj() {
            self.best_sol = Some((f(), obj));
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

/// Represents a solver for a problem
pub trait Solver<P: Problem>: Clone {
    fn solve<SK: SolutionKeeper<P>, S: StopCondition<P::Obj>>(
        &mut self,
        p: P,
        sk: &mut SK,
        stop: S,
    );
}
