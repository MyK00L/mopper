use crate::core::*;

// front end traits

/// Represents a search space that divides the problem into subproblems
pub trait TreeSpaceDirect<P: Problem> {
    /// A subproblem
    type Node: Clone + Debug;
    fn root(&self) -> Self::Node;
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::Node>;
    fn primal_bound(&self, n: &Self::Node) -> P::Obj;
    fn dual_bound(&self, n: &Self::Node, primal: P::Obj) -> P::Obj;
}

/// Represents a search space that divides the problem into subproblems
pub trait TreeSpaceIndirect<P: Problem>{
    /// A subproblem
    type Node: Clone + Debug;
    type ChildId: Clone + Debug;
    fn root(&self) -> Self::Node;
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId>;
    fn child(&self, n: &Self::Node, cid: &Self::ChildId) -> Self::Node;
    fn child_primal_bound(&self, n: &Self::Node, cid: &Self::ChildId) -> P::Obj;
    fn child_dual_bound(&self, n: &Self::Node, cid: &Self::ChildId, primal: P::Obj) -> P::Obj;
}

// back end traits
pub trait TreeSpaceBackend<P: Problem> {
    type Node: Clone + Debug;
    type ChildId: Clone + Debug;
    fn root(&self) -> Self::Node;
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId>;
    fn children_direct(&self, n: &Self::Node) -> impl Iterator<Item = Self::Node> {
        self.children(n).map(|cid| self.child(n, cid))
    }
    fn child(&self, n: &Self::Node, cid: Self::ChildId) -> Self::Node;
    fn primal_bound(&self, n: &Self::Node) -> P::Obj;
    fn dual_bound(&self, n: &Self::Node, primal: P::Obj) -> P::Obj;
    fn child_primal_bound(&self, n: &Self::Node, cid: &Self::ChildId) -> P::Obj;
    fn child_dual_bound(&self, n: &Self::Node, cid: &Self::ChildId, primal: P::Obj) -> P::Obj;
}
impl<P: Problem, T: TreeSpaceDirect<P>> TreeSpaceBackend<P> for T {
    type Node = T::Node;
    type ChildId = T::Node;
    fn root(&self) -> Self::Node {
        self.root()
    }
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId> {
        self.children(n)
    }
    fn child(&self, _n: &Self::Node, cid: Self::ChildId) -> Self::Node {
        cid
    }
    fn primal_bound(&self, n: &Self::Node) -> P::Obj {
        self.primal_bound(n)
    }
    fn dual_bound(&self, n: &Self::Node, primal: P::Obj) -> P::Obj {
        self.dual_bound(n, primal)
    }
    fn child_primal_bound(&self, _n: &Self::Node, cid: &Self::ChildId) -> P::Obj {
        self.primal_bound(cid)
    }
    fn child_dual_bound(&self, _n: &Self::Node, cid: &Self::ChildId, primal: P::Obj) -> P::Obj {
        self.dual_bound(cid, primal)
    }
}

impl<P: Problem, T: TreeSpaceIndirect<P>> TreeSpaceBackend<P> for T {
    type Node = T::Node;
    type ChildId = T::ChildId;
    fn root(&self) -> Self::Node {
        self.root()
    }
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId> {
        self.children(n)
    }
    fn child(&self, n: &Self::Node, cid: Self::ChildId) -> Self::Node {
        self.child(n, &cid)
    }
    fn primal_bound(&self, n: &Self::Node) -> P::Obj {
        self.dual_bound(n, P::Obj::unbounded())
    }
    fn dual_bound(&self, n: &Self::Node, primal: P::Obj) -> P::Obj {
        let mut best = P::Obj::unbounded();
        for cid in self.children(n) {
            let db = self.child_dual_bound(n, &cid, primal);
            if db < best {
                best = db;
            }
        }
        best
    }
    fn child_primal_bound(&self, n: &Self::Node, cid: &Self::ChildId) -> P::Obj {
        self.child_primal_bound(n, cid)
    }
    fn child_dual_bound(&self, n: &Self::Node, cid: &Self::ChildId, primal: P::Obj) -> P::Obj {
        self.child_dual_bound(n, cid, primal)
    }
}
