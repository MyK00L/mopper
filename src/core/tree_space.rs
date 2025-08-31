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
pub trait TreeSpaceIndirect<P: Problem> {
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

// Newtype wrappers to avoid conflicting trait implementations
pub struct TreeSpaceDirectW<T>(pub T);
pub struct TreeSpaceindirectW<T>(pub T);

impl<P: Problem, T: TreeSpaceDirect<P>> TreeSpaceBackend<P> for TreeSpaceDirectW<T> {
    type Node = T::Node;
    type ChildId = T::Node;
    fn root(&self) -> Self::Node {
        self.0.root()
    }
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId> {
        self.0.children(n)
    }
    fn child(&self, _n: &Self::Node, cid: Self::ChildId) -> Self::Node {
        cid
    }
    fn primal_bound(&self, n: &Self::Node) -> P::Obj {
        self.0.primal_bound(n)
    }
    fn dual_bound(&self, n: &Self::Node, primal: P::Obj) -> P::Obj {
        self.0.dual_bound(n, primal)
    }
    fn child_primal_bound(&self, _n: &Self::Node, cid: &Self::ChildId) -> P::Obj {
        self.0.primal_bound(cid)
    }
    fn child_dual_bound(&self, _n: &Self::Node, cid: &Self::ChildId, primal: P::Obj) -> P::Obj {
        self.0.dual_bound(cid, primal)
    }
}

impl<P: Problem, T: TreeSpaceIndirect<P>> TreeSpaceBackend<P> for TreeSpaceindirectW<T> {
    type Node = (T::Node, Option<P::Obj>, Option<P::Obj>); // node, primal bound, dual bound
    type ChildId = (T::ChildId, Option<P::Obj>, Option<P::Obj>); // child id, primal bound, dual bound
    fn root(&self) -> Self::Node {
        (
            self.0.root(),
            Some(<P as Problem>::Obj::unfeas()),
            Some(<P as Problem>::Obj::unbounded()),
        )
    }
    fn children(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId> {
        self.0.children(&n.0).map(move |cid| (cid, None, None))
    }
    fn child(&self, n: &Self::Node, cid: Self::ChildId) -> Self::Node {
        let primal = cid
            .1
            .or_else(|| Some(self.0.child_primal_bound(&n.0, &cid.0)));
        let dual = cid
            .2
            .or_else(|| Some(self.0.child_dual_bound(&n.0, &cid.0, primal.unwrap())));
        (self.0.child(&n.0, &cid.0), primal, dual)
    }
    fn primal_bound(&self, n: &Self::Node) -> <P as Problem>::Obj {
        n.1.unwrap()
    }
    fn dual_bound(&self, n: &Self::Node, _primal: <P as Problem>::Obj) -> <P as Problem>::Obj {
        n.2.unwrap()
    }
    fn child_dual_bound(
        &self,
        n: &Self::Node,
        cid: &Self::ChildId,
        primal: <P as Problem>::Obj,
    ) -> <P as Problem>::Obj {
        cid.2
            .unwrap_or_else(|| self.0.child_dual_bound(&n.0, &cid.0, primal))
    }
    fn child_primal_bound(&self, n: &Self::Node, cid: &Self::ChildId) -> <P as Problem>::Obj {
        cid.1
            .unwrap_or_else(|| self.0.child_primal_bound(&n.0, &cid.0))
    }
}
