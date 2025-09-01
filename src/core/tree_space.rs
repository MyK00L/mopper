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
    fn primal_bound(&self, n: &Self::Node) -> Option<P::Obj>;
    fn dual_bound(&self, n: &Self::Node, primal: P::Obj) -> P::Obj;
    fn to_solution(&self, n: &Self::Node) -> Option<P::Solution>;
    /// Returns an iterator over children ordered by their dual bounds (best first)
    fn children_ord(&self, n: &Self::Node) -> impl Iterator<Item = (P::Obj, Self::ChildId)> {
        let mut ch: Vec<(P::Obj, Self::ChildId)> = self
            .children(n)
            .map(|cid| {
                let db = self.child_dual_bound(
                    n,
                    &cid,
                    self.primal_bound(n).unwrap_or_else(P::Obj::unfeas),
                );
                (db, cid)
            })
            .collect();
        ch.sort_by(|(db1, _), (db2, _)| db1.cmp(db2));
        ch.into_iter()
    }
    fn from(p: &P) -> Self;
}
