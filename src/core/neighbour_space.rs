use crate::core::*;

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


