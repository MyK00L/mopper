use crate::core::*;

pub trait NeighbourhoodDirect<P: Problem>: Clone {
    fn neighbourhood(&self, p: &P, node: P::Sol) -> impl Iterator<Item = P::Sol>;
}
pub trait NeighbourhoodIndirect<P: Problem>: Clone {
    type NeighbourId: Clone + Debug;
    fn neighbourhood_id(&self, p: &P, node: &P::Sol) -> impl Iterator<Item = Self::NeighbourId>;
    fn neighbour_obj(&self, p: &P, node: &P::Sol, nid: &Self::NeighbourId) -> P::Obj;
    fn neighbour(&self, p: &P, node: P::Sol, nid: Self::NeighbourId) -> P::Sol;
}
pub trait NeighbourhoodDirectRandom<P: Problem>: Clone {
    fn random_neighbour<R: rng::Rng>(&self, p: &P, node: &P::Sol, rng: &mut R) -> P::Sol;
}
pub trait NeighbourhoodIndirectRandom<P: Problem>: Clone {
    type NeighbourId: Clone + Debug;
    fn random_neighbour_id<R: rng::Rng>(
        &self,
        p: &P,
        node: &P::Sol,
        rng: &mut R,
    ) -> Self::NeighbourId;
    fn random_neighbour(&self, p: &P, node: P::Sol, nid: Self::NeighbourId) -> P::Sol;
    fn neighbour_obj(&self, p: &P, node: &P::Sol, nid: &Self::NeighbourId) -> P::Obj;
}

// TODO: trait implementation macros
