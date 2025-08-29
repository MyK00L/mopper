use std::fmt::Debug;

/// Represents the trait for the type of the objective,
/// will always deal with minimization problems
/// unbounded < best <= value <= worst < unfeas
pub trait Objective: Clone + Copy + Debug + PartialOrd + Ord + Into<f64> {
    fn best() -> Self;
    fn worst() -> Self;
    fn unfeas() -> Self;
    fn unbounded() -> Self;
    fn is_feas(&self) -> bool;
    fn is_unbounded(&self) -> bool;
}

/// Represents a problem
pub trait Problem {
    type Solution: Clone + Debug;
    type Obj: Objective;
}

// TODO: fast timer, rng (splitmix64), random problem generation stuff

/// Represents a solver for a problem
pub trait Solver<P: Problem> {
    // TODO: also return a list of primal/dual bounds with their timestamps
    fn solve(p: P) -> P::Solution;
}

/// Represents a graph (not necessarily symmetric) in the solution space, with potentially unfeasible solutions too
/// this structure needs to contain the global state of the problem if there is any
pub trait NeighbourSpace<P: Problem>: From<P> {
    /// A potential solution
    type Node: Clone + Debug;
    /// A usually small structure to identify a solution,
    /// useful when calculating objective and feasibility before moving is faster.
    /// Could also be the same as Node if this is not necessary
    type NeighbourId: Clone + Debug;
    /// Converts a node to the original problem solution
    fn to_solution(&self, n: &Self::Node) -> P::Solution;
    /// Converts an original problem solution to its corresponding node
    fn to_node(&self, sol: &P::Solution) -> Self::Node;
    /// Returns the objective value of a node
    fn eval(&self, n: &Self::Node) -> P::Obj;
    /// Returns true if a node represents a feasible solution
    fn feas(&self, n: &Self::Node) -> bool;
    /// Returns the objective value of the neighbour of a node
    fn eval_neigh(&self, n: &Self::Node, nid: &Self::NeighbourId) -> P::Obj;
    /// Returns the objective value of the neighbour of a node
    fn feas_neigh(&self, n: &Self::Node, nid: &Self::NeighbourId) -> bool;
    /// Returns the id to a random neighbour
    fn random_neighbour(&self, n: &Self::Node) -> Option<Self::NeighbourId>;
    /// Returns an iterator to the whole neighbourhood, in arbitrary order
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId>;
    /// Transforms a node into one of its neighbours
    fn moveto(&self, n: &mut Self::Node, nid: &Self::NeighbourId);
    /// Returns an iterator to the feasible neighbours
    fn neighbourhood_feas(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId> {
        self.neighbourhood(n).filter(|nid| self.feas_neigh(n, nid))
    }
    /// Returns an iterator to the feasible neighbours from the one with best objective to worst
    fn neighbourhood_ord(
        &self,
        n: &Self::Node,
    ) -> impl Iterator<Item = (P::Obj, Self::NeighbourId)> {
        let mut neigh: Vec<(P::Obj, Self::NeighbourId)> = self
            .neighbourhood_feas(n)
            .map(|x| (self.eval_neigh(n, &x), x))
            .collect();
        neigh.sort_by_key(|x| x.0);
        neigh.into_iter()
    }
}

/// Represents a search space that divides the problem into subproblems
pub trait TreeSpace<P: Problem> {
    /// A subproblem
    type Node: Clone + Debug;
    type ChildId: Clone + Debug;
    fn primal_bound(&self, n: &Self::Node) -> P::Obj;
    fn dual_bound(&self, n: &Self::Node) -> P::Obj;
    fn child_primal_bound(&self, n: &Self::Node, cid: &Self::ChildId) -> P::Obj;
    fn child_dual_bound(&self, n: &Self::Node, cid: &Self::ChildId) -> P::Obj;
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId>;
    fn ordered_children(&self, n: &Self::Node) -> impl Iterator<Item = (P::Obj, Self::ChildId)> {
        let mut cld: Vec<(P::Obj, Self::ChildId)> = self
            .children(n)
            .map(|x| (self.child_dual_bound(n, &x), x))
            .collect();
        cld.sort_by_key(|x| x.0);
        cld.into_iter()
    }
}
