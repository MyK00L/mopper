use crate::core::neighbour_space::*;
use crate::core::*;

#[derive(Clone)]
pub struct FirstImprovingRandomLocalSearch<
    P: Problem,
    N: NeighbourhoodIndirectRandom<P>,
    R: rng::Rng,
> {
    initial_solution: Option<(P::Sol, P::Obj)>,
    rng: R,
    ns: N,
}
impl<P: Problem, N: NeighbourhoodIndirectRandom<P>, R: rng::Rng>
    FirstImprovingRandomLocalSearch<P, N, R>
{
    pub fn new(ns: N, initial_solution: (P::Sol, P::Obj), rng: R) -> Self {
        Self {
            initial_solution: Some(initial_solution),
            rng,
            ns,
        }
    }
}
impl<P: Problem, N: NeighbourhoodIndirectRandom<P>, R: rng::Rng> Solver<P>
    for FirstImprovingRandomLocalSearch<P, N, R>
{
    fn solve<SK: SolutionKeeper<P>, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        sk: &mut SK,
        mut stop: S,
    ) {
        let (mut solution, mut obj) = self.initial_solution.take().unwrap();
        loop {
            if stop.stop(sk.best_obj(), P::Obj::unbounded()) {
                break;
            }
            sk.iter();
            let nid = self.ns.random_neighbour_id(&p, &solution, &mut self.rng);
            let nobj = self.ns.neighbour_obj(&p, &solution, &nid);
            if nobj < obj {
                solution = self.ns.random_neighbour(&p, solution, nid);
                obj = nobj;
                sk.add_solution(&solution, obj);
            }
        }
    }
}

#[derive(Clone)]
pub struct SteepestDescentLocalSearch<P: Problem, N: NeighbourhoodIndirect<P>> {
    initial_solution: Option<(P::Sol, P::Obj)>,
    ns: N,
}
impl<P: Problem, N: NeighbourhoodIndirect<P>> SteepestDescentLocalSearch<P, N> {
    pub fn new(ns: N, initial_solution: (P::Sol, P::Obj)) -> Self {
        Self {
            initial_solution: Some(initial_solution),
            ns,
        }
    }
}
impl<P: Problem, N: NeighbourhoodIndirect<P>> Solver<P> for SteepestDescentLocalSearch<P, N> {
    fn solve<SK: SolutionKeeper<P>, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        sk: &mut SK,
        mut stop: S,
    ) {
        let (mut solution, mut obj) = self.initial_solution.take().unwrap();
        sk.add_solution(&solution, obj);
        loop {
            if stop.stop(obj, P::Obj::unbounded()) {
                break;
            }
            sk.iter();
            let mut best_nid = None;
            let mut best_nobj = obj;
            for nid in self.ns.neighbourhood_id(&p, &solution) {
                let nobj = self.ns.neighbour_obj(&p, &solution, &nid);
                if nobj < best_nobj {
                    best_nobj = nobj;
                    best_nid = Some(nid);
                }
            }
            if let Some(nid) = best_nid {
                solution = self.ns.neighbour(&p, solution, nid);
                obj = best_nobj;
                sk.add_solution(&solution, obj);
            } else {
                break;
            }
        }
    }
}
