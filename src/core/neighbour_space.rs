use crate::core::*;

pub trait NeighbourSpace<P: Problem> {
    type Node: Clone + Debug;
    type NeighbourId: Clone + Debug;
    fn to_solution(&self, n: &Self::Node) -> P::Solution;
    fn to_node(&self, sol: &P::Solution) -> Self::Node;
    fn random_neighbour(&self, n: &Self::Node) -> Self::NeighbourId;
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId>;
    fn neighbour(&self, n: &Self::Node, nid: Self::NeighbourId) -> Self::Node;
    fn eval(&self, n: &Self::Node) -> P::Obj;
    fn eval_neighbour(&self, n: &Self::Node, nid: &Self::NeighbourId) -> P::Obj;
    /// Returns an iterator to the feasible neighbours from the one with best objective to worst
    fn neighbourhood_ord(
        &self,
        n: &Self::Node,
    ) -> impl Iterator<Item = (P::Obj, Self::NeighbourId)> {
        let mut neigh: Vec<(P::Obj, Self::NeighbourId)> = self
            .neighbourhood(n)
            .map(|x| (self.eval_neighbour(n, &x), x))
            .collect();
        neigh.sort_by_key(|x| x.0);
        neigh.into_iter()
    }
}

pub struct ChainedNeighbourSpace<P: Problem, NS1: NeighbourSpace<P>, NS2: NeighbourSpace<P>>
where
    NS2::Node: From<NS1::Node>,
    NS1::Node: From<NS2::Node>,
{
    ns1: NS1,
    ns2: NS2,
    _p: std::marker::PhantomData<P>,
}
impl<P: Problem, NS1: NeighbourSpace<P>, NS2: NeighbourSpace<P>> NeighbourSpace<P>
    for ChainedNeighbourSpace<P, NS1, NS2>
where
    NS2::Node: From<NS1::Node>,
    NS1::Node: From<NS2::Node>,
{
    type Node = NS1::Node;
    type NeighbourId = NS1::Node;
    fn to_solution(&self, n: &Self::Node) -> P::Solution {
        self.ns1.to_solution(n)
    }
    fn to_node(&self, sol: &P::Solution) -> Self::Node {
        self.ns1.to_node(sol)
    }
    fn random_neighbour(&self, n: &Self::Node) -> Self::NeighbourId {
        let s1 = self.ns1.neighbour(n, self.ns1.random_neighbour(n));
        let s2 = s1.into();
        self.ns2
            .neighbour(&s2, self.ns2.random_neighbour(&s2))
            .into()
    }
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId> {
        let n1 = self.ns1.neighbourhood(n).map(move |nid1| {
            let s1 = self.ns1.neighbour(n, nid1);
            let s2: NS2::Node = s1.into();
            self.ns2
                .neighbourhood(&s2)
                .map({
                    let value = s2.clone();
                    move |nid2| {
                        let s3 = self.ns2.neighbour(&value, nid2);
                        s3.into()
                    }
                })
                .collect::<Vec<_>>()
        });
        n1.flatten()
    }
    fn neighbour(&self, _n: &Self::Node, nid: Self::NeighbourId) -> Self::Node {
        nid
    }
    fn eval(&self, n: &Self::Node) -> P::Obj {
        self.ns1.eval(n)
    }
    fn eval_neighbour(&self, _n: &Self::Node, nid: &Self::NeighbourId) -> P::Obj {
        self.ns1.eval(nid)
    }
}
