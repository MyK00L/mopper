use crate::core::*;

/// Represents a search space that divides the problem into subproblems
pub trait TreeSpace<P: Problem> {
    /// A subproblem
    type Node: Clone + Debug;
    type ChildId: Clone + Debug;
    fn root(&self) -> Self::Node;
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId>;
    fn child(&self, n: &Self::Node, cid: &Self::ChildId) -> Self::Node;
    fn child_primal_bound(&self, n: &Self::Node, cid: &Self::ChildId) -> P::Obj;
    fn child_dual_bound(&self, n: &Self::Node, cid: &Self::ChildId, primal: P::Obj) -> P::Obj;
    fn primal_bound(&self, n: &Self::Node) -> P::Obj;
    fn dual_bound(&self, n: &Self::Node, primal: P::Obj) -> P::Obj;
    fn to_solution(&self, n: &Self::Node) -> Option<P::Solution>;
}
