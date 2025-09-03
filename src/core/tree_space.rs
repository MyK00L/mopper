use crate::core::*;

pub trait Tree<P: Problem> {
    type Node: Clone + Debug;
    /// Returns the root node of the search space
    fn root(&self) -> Self::Node;
    /// True if the node is a leaf (no children)
    fn is_leaf(&self, n: &Self::Node) -> bool;
    /// Returns the objective value of the solution represented by this node, only if it is a leaf
    fn objective(&self, n: &Self::Node) -> Option<P::Obj>;
    /// Converts a leaf node to a solution
    fn to_solution(&self, n: &Self::Node) -> Option<P::Sol>;
    /// Constructs a tree space from a problem instance
    fn from(p: &P) -> Self;
}
pub trait TreeDirect<P: Problem>: Tree<P> {
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::Node>;
}
pub trait TreeBounded<P: Problem>: Tree<P> {
    fn primal_bound(&self, n: &Self::Node) -> P::Obj;
    fn dual_bound(&self, n: &Self::Node, primal: P::Obj) -> P::Obj;
}
pub trait TreeGuided<P: Problem>: Tree<P> {
    type Guide: crate::core::Objective; // = P::Obj; // associated type defaults are unstable
    fn goodness(&self, n: &Self::Node) -> Self::Guide;
}
pub trait TreeIndirect<P: Problem>: Tree<P> {
    type ChildId: Clone + Debug;
    fn children_id(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId>;
    fn child(&self, n: &Self::Node, cid: &Self::ChildId) -> Self::Node;
}
pub trait TreeIndirectBounded<P: Problem>: TreeIndirect<P> {
    fn child_primal_bound(&self, n: &Self::Node, cid: &Self::ChildId) -> P::Obj;
    fn child_dual_bound(&self, n: &Self::Node, cid: &Self::ChildId, primal: P::Obj) -> P::Obj;
}
pub trait TreeIndirectGuided<P: Problem>: TreeIndirect<P> + TreeGuided<P> {
    fn child_goodness(&self, n: &Self::Node, cid: &Self::ChildId) -> Self::Guide;
}
pub trait TreeDirectRandom<P: Problem>: Tree<P> {
    fn random_child<R: rng::Rng>(&self, n: &Self::Node, rng: &mut R) -> Option<Self::Node>;
    fn random_child_consuming<R: rng::Rng>(&self, n: Self::Node, rng: &mut R)
        -> Option<Self::Node>;
}
pub trait TreeRollback<P: Problem>: Tree<P> {
    type RollbackInfo: Clone + Debug;
    fn rollback(&self, n: Self::Node, info: Self::RollbackInfo) -> Self::Node;
}
pub trait TreeRollbackDirectRandom<P: Problem>: TreeRollback<P> {
    fn random_child_rollback<R: rng::Rng>(
        &self,
        n: Self::Node,
        rng: &mut R,
    ) -> Option<(Self::Node, Self::RollbackInfo)>;
}
pub trait TreeRollbackIndirect<P: Problem>: TreeRollback<P> {
    type ChildIdR: Clone + Debug;
    /// Returns an iterator over the children of a node along with the rollback info to get back to the parent
    fn children_id_rollback(
        &self,
        n: &Self::Node,
    ) -> impl Iterator<Item = (Self::ChildIdR, Self::RollbackInfo)>;
    /// Returns the child node given its id
    fn child_r(&self, n: Self::Node, cid: &Self::ChildIdR) -> Self::Node;
}

// TODO: trait implementation macros
