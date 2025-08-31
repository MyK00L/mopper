use crate::core::*;

pub trait NeghbourSpace<P: Problem> {
    type Node: Clone + Debug;
    type NeighbourId: Clone + Debug;
    fn to_solution(&self, n: &Self::Node) -> P::Solution;
    fn to_node(&self, sol: &P::Solution) -> Self::Node;
    fn random_neighbour(&self, n: &Self::Node) -> Self::NeighbourId;
    fn neighbourhood(&self, n: &Self::Node) -> impl Iterator<Item = Self::NeighbourId>;
    fn neighbour(&self, n: &Self::Node, nid: &Self::NeighbourId) -> Self::Node;
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
