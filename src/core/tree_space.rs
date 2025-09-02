use crate::core::*;

/// Represents a search space that divides the problem into subproblems
pub trait TreeSpace<P: Problem> {
    /// A subproblem
    type Node: Clone + Debug;
    /// An identifier for a child of a node, could be the same as `Node`
    type ChildId: Clone + Debug;
    /// Returns the root node of the search space
    fn root(&self) -> Self::Node;
    /// Returns an iterator over the children of a node
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId>;
    /// Converts a node and a child id to the corresponding child node
    fn child(&self, n: &Self::Node, cid: &Self::ChildId) -> Self::Node;
    /// Returns a random child of a node, or `None` if the node is a leaf
    fn random_child<R: rng::Rng>(&self, n: Self::Node, rng: &mut R) -> Option<Self::Node>;
    /// Returns the primal bound for a child, or `P::Obj::unfeas()` if none is known
    fn child_primal_bound(&self, n: &Self::Node, cid: &Self::ChildId) -> P::Obj;
    /// Returns the dual bound for a child, can cut short if it is worse than `primal`
    fn child_dual_bound(&self, n: &Self::Node, cid: &Self::ChildId, primal: P::Obj) -> P::Obj;
    /// Returns the primal bound of a node, or `P::Obj::unfeas()` if none is known
    fn primal_bound(&self, n: &Self::Node) -> P::Obj;
    /// Returns the dual bound of a node, can cut short if it is worse than `primal`
    fn dual_bound(&self, n: &Self::Node, primal: P::Obj) -> P::Obj;
    /// True if the node is a leaf (no children)
    fn is_leaf(&self, n: &Self::Node) -> bool;
    /// Returns the objective value of the solution represented by this node, only if it is a leaf
    fn objective(&self, n: &Self::Node) -> Option<P::Obj>;
    /// Converts a leaf node to a solution
    fn to_solution(&self, n: &Self::Node) -> Option<P::Solution>;
    /// Returns an iterator over children ordered by their dual bounds (best first)
    fn children_ord(&self, n: &Self::Node) -> impl Iterator<Item = (P::Obj, Self::ChildId)> {
        let mut ch: Vec<(P::Obj, Self::ChildId)> = self
            .children(n)
            .map(|cid| {
                let db = self.child_dual_bound(n, &cid, P::Obj::unfeas());
                (db, cid)
            })
            .collect();
        ch.sort_by(|(db1, _), (db2, _)| db1.cmp(db2));
        ch.into_iter()
    }
    /// Constructs a tree space from a problem instance
    fn from(p: &P) -> Self;
}
