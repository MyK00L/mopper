use crate::core::*;

pub trait NeighbourSpace<P: Problem> {
    type Node: Clone + Debug;
    type NeighbourId: Clone + Debug;
    fn to_solution(&self, n: &Self::Node) -> P::Solution;
    fn to_node(&self, sol: &P::Solution) -> Self::Node;
    fn random_neighbour<R: rng::Rng>(&self, n: &Self::Node, rng: &mut R) -> Self::NeighbourId;
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
    fn from(p: &P) -> Self;
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
impl<P: Problem, NS1: NeighbourSpace<P>, NS2: NeighbourSpace<P>> ChainedNeighbourSpace<P, NS1, NS2>
where
    NS2::Node: From<NS1::Node>,
    NS1::Node: From<NS2::Node>,
{
    pub fn new(ns1: NS1, ns2: NS2) -> Self {
        Self {
            ns1,
            ns2,
            _p: std::marker::PhantomData,
        }
    }
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
    fn random_neighbour<R: rng::Rng>(&self, n: &Self::Node, rng: &mut R) -> Self::NeighbourId {
        let s1 = self.ns1.neighbour(n, self.ns1.random_neighbour(n, rng));
        let s2 = s1.into();
        self.ns2
            .neighbour(&s2, self.ns2.random_neighbour(&s2, rng))
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
    fn from(p: &P) -> Self {
        ChainedNeighbourSpace {
            ns1: NS1::from(p),
            ns2: NS2::from(p),
            _p: std::marker::PhantomData,
        }
    }
}

pub struct CombinedNeighbourSpace<P: Problem, NS1: NeighbourSpace<P>, NS2: NeighbourSpace<P>>
where
    NS1::Node: From<NS2::Node>,
    NS2::Node: From<NS1::Node>,
{
    ns1: NS1,
    ns2: NS2,
    _p: std::marker::PhantomData<P>,
}
impl<P: Problem, NS1: NeighbourSpace<P>, NS2: NeighbourSpace<P>> CombinedNeighbourSpace<P, NS1, NS2>
where
    NS1::Node: From<NS2::Node>,
    NS2::Node: From<NS1::Node>,
{
    pub fn new(ns1: NS1, ns2: NS2) -> Self {
        Self {
            ns1,
            ns2,
            _p: std::marker::PhantomData,
        }
    }
}
#[derive(Clone, Debug)]
pub enum CombinedNeighbourId<N1: Clone + Debug, N2: Clone + Debug> {
    Neigh1(N1),
    Neigh2(N2),
}
impl<P: Problem, NS1: NeighbourSpace<P>, NS2: NeighbourSpace<P>> NeighbourSpace<P>
    for CombinedNeighbourSpace<P, NS1, NS2>
where
    NS1::Node: From<NS2::Node>,
    NS2::Node: From<NS1::Node>,
{
    type Node = NS1::Node;
    type NeighbourId = CombinedNeighbourId<NS1::NeighbourId, NS2::NeighbourId>;
    fn to_solution(&self, n: &Self::Node) -> P::Solution {
        self.ns1.to_solution(n)
    }
    fn to_node(&self, sol: &P::Solution) -> Self::Node {
        self.ns1.to_node(sol)
    }
    fn random_neighbour<R: rng::Rng>(&self, n: &Self::Node, rng: &mut R) -> Self::NeighbourId {
        if rng.next01() < 0.5 {
            CombinedNeighbourId::Neigh1(self.ns1.random_neighbour(n, rng))
        } else {
            let s2: NS2::Node = n.clone().into();
            CombinedNeighbourId::Neigh2(self.ns2.random_neighbour(&s2, rng))
        }
    }
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId> {
        let n1 = self.ns1.neighbourhood(n).map(CombinedNeighbourId::Neigh1);
        let s2: NS2::Node = n.clone().into();
        let n2 = self
            .ns2
            .neighbourhood(&s2)
            .collect::<Vec<NS2::NeighbourId>>()
            .into_iter()
            .map(CombinedNeighbourId::Neigh2);
        n1.chain(n2)
    }
    fn neighbour(&self, n: &Self::Node, nid: Self::NeighbourId) -> Self::Node {
        match nid {
            CombinedNeighbourId::Neigh1(nid1) => self.ns1.neighbour(n, nid1),
            CombinedNeighbourId::Neigh2(nid2) => {
                let s2: NS2::Node = n.clone().into();
                self.ns2.neighbour(&s2, nid2).into()
            }
        }
    }
    fn eval(&self, n: &Self::Node) -> P::Obj {
        self.ns1.eval(n)
    }
    fn eval_neighbour(&self, n: &Self::Node, nid: &Self::NeighbourId) -> P::Obj {
        match nid {
            CombinedNeighbourId::Neigh1(nid1) => self.ns1.eval_neighbour(n, nid1),
            CombinedNeighbourId::Neigh2(nid2) => {
                let s2: NS2::Node = n.clone().into();
                self.ns2.eval_neighbour(&s2, nid2)
            }
        }
    }
    fn from(p: &P) -> Self {
        CombinedNeighbourSpace {
            ns1: NS1::from(p),
            ns2: NS2::from(p),
            _p: std::marker::PhantomData,
        }
    }
}
