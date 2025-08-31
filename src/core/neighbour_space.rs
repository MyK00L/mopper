use crate::core::*;

pub trait NeighbourSpaceDirect<P: Problem> {
    type Node: Clone + Debug;
    fn to_solution(&self, n: &Self::Node) -> P::Solution;
    fn to_node(&self, sol: &P::Solution) -> Self::Node;
    fn eval(&self, n: &Self::Node) -> P::Obj;
    fn random_neighbour(&self, n: &Self::Node) -> Self::Node;
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::Node>;
}

pub trait NeghbourSpaceIndirect<P: Problem> {
    type Node: Clone + Debug;
    type NeighbourId: Clone + Debug;
    fn to_solution(&self, n: &Self::Node) -> P::Solution;
    fn to_node(&self, sol: &P::Solution) -> Self::Node;
    fn random_neighbour(&self, n: &Self::Node) -> Self::NeighbourId;
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId>;
    fn neighbour(&self, n: &Self::Node, nid: &Self::NeighbourId) -> Self::Node;
    fn eval(&self, n: &Self::Node) -> P::Obj;
    fn eval_neighbour(&self, n: &Self::Node, nid: &Self::NeighbourId) -> P::Obj;
}

/// Represents a graph (not necessarily symmetric) in the solution space, with potentially unfeasible solutions too
/// this structure needs to contain the global state of the problem if there is any
pub trait NeighbourSpaceBackend<P: Problem> {
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
    /// Returns the objective value of the neighbour of a node
    fn eval_neigh(&self, n: &Self::Node, nid: &Self::NeighbourId) -> P::Obj;
    /// Returns the id to a random neighbour
    fn random_neighbour(&self, n: &Self::Node) -> Self::NeighbourId;
    /// Returns an iterator to the whole neighbourhood, in arbitrary order
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId>;
    /// Transforms a node into one of its neighbours
    fn moveto(&self, n: &mut Self::Node, nid: &Self::NeighbourId);
    /// Returns an iterator to the feasible neighbours from the one with best objective to worst
    fn neighbourhood_ord(
        &self,
        n: &Self::Node,
    ) -> impl Iterator<Item = (P::Obj, Self::NeighbourId)> {
        let mut neigh: Vec<(P::Obj, Self::NeighbourId)> = self
            .neighbourhood(n)
            .map(|x| (self.eval_neigh(n, &x), x))
            .collect();
        neigh.sort_by_key(|x| x.0);
        neigh.into_iter()
    }
}

// Newtype wrapper to avoid conflicting trait implementations
pub struct NeighbourSpaceDirectW<T>(pub T);
pub struct NeighbourSpaceIndirectW<T>(pub T);

impl<P: Problem, T: NeighbourSpaceDirect<P>> NeighbourSpaceBackend<P> for NeighbourSpaceDirectW<T> {
    type Node = T::Node;
    type NeighbourId = T::Node;
    fn to_solution(&self, n: &Self::Node) -> P::Solution {
        self.0.to_solution(n)
    }
    fn to_node(&self, sol: &P::Solution) -> Self::Node {
        self.0.to_node(sol)
    }
    fn eval(&self, n: &Self::Node) -> P::Obj {
        self.0.eval(n)
    }
    fn eval_neigh(&self, _n: &Self::Node, nid: &Self::NeighbourId) -> P::Obj {
        self.0.eval(nid)
    }
    fn random_neighbour(&self, n: &Self::Node) -> Self::NeighbourId {
        self.0.random_neighbour(n)
    }
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId> {
        self.0.neighbourhood(n)
    }
    fn moveto(&self, n: &mut Self::Node, nid: &Self::NeighbourId) {
        *n = nid.clone();
    }
}

impl<P: Problem, T: NeghbourSpaceIndirect<P>> NeighbourSpaceBackend<P>
    for NeighbourSpaceIndirectW<T>
{
    type Node = T::Node;
    type NeighbourId = T::NeighbourId;
    fn to_solution(&self, n: &Self::Node) -> P::Solution {
        self.0.to_solution(n)
    }
    fn to_node(&self, sol: &P::Solution) -> Self::Node {
        self.0.to_node(sol)
    }
    fn eval(&self, n: &Self::Node) -> P::Obj {
        self.0.eval(n)
    }
    fn eval_neigh(&self, n: &Self::Node, nid: &Self::NeighbourId) -> P::Obj {
        self.0.eval_neighbour(n, nid)
    }
    fn random_neighbour(&self, n: &Self::Node) -> Self::NeighbourId {
        self.0.random_neighbour(n)
    }
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId> {
        self.0.neighbourhood(n)
    }
    fn moveto(&self, n: &mut Self::Node, nid: &Self::NeighbourId) {
        *n = self.0.neighbour(n, nid);
    }
}
