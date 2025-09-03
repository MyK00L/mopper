use crate::core::neighbour_space::*;
use crate::core::*;

pub struct MicrocanonicalAnnealing<P: Problem, N: NeighbourhoodIndirectRandom<P>, R: rng::Rng> {
    initial_solution: Option<(P::Sol, P::Obj)>,
    initial_demon_energy: f64,
    rng: R,
    ns: N,
}
impl<P: Problem, N: NeighbourhoodIndirectRandom<P>, R: rng::Rng> MicrocanonicalAnnealing<P, N, R> {
    pub fn new(
        ns: N,
        initial_solution: (P::Sol, P::Obj),
        initial_demon_energy: f64,
        rng: R,
    ) -> Self {
        debug_assert!(initial_demon_energy >= 0.0);
        Self {
            initial_solution: Some(initial_solution),
            initial_demon_energy,
            rng,
            ns,
        }
    }
}
impl<P: Problem, N: NeighbourhoodIndirectRandom<P>, R: rng::Rng> Solver<P>
    for MicrocanonicalAnnealing<P, N, R>
{
    fn solve<SK: SolutionKeeper<P>, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        sk: &mut SK,
        mut stop: S,
    ) {
        let (mut current_solution, mut current_obj) = self.initial_solution.take().unwrap();
        sk.add_solution(&current_solution, current_obj);
        let mut demon_energy = self.initial_demon_energy;
        loop {
            if stop.stop(sk.best_obj(), P::Obj::unbounded()) {
                break;
            }
            sk.iter();
            let nid = self
                .ns
                .random_neighbour_id(&p, &current_solution, &mut self.rng);
            let nobj = self.ns.neighbour_obj(&p, &current_solution, &nid);
            let delta = nobj.into() - current_obj.into();
            if demon_energy >= delta {
                current_solution = self.ns.random_neighbour(&p, current_solution, nid);
                current_obj = nobj;
                demon_energy -= delta;
                sk.add_solution(&current_solution, current_obj);
            }
        }
    }
}
