use crate::core::*;

pub struct MicrocanonicalAnnealing<P: Problem, N: neighbour_space::NeighbourSpace<P>, R: rng::Rng> {
    initial_solution: P::Solution,
    initial_demon_energy: f64,
    rng: R,
    _n: std::marker::PhantomData<N>,
}
impl<P: Problem, N: neighbour_space::NeighbourSpace<P>, R: rng::Rng>
    MicrocanonicalAnnealing<P, N, R>
{
    pub fn new(initial_solution: P::Solution, initial_demon_energy: f64, rng: R) -> Self {
        debug_assert!(initial_demon_energy >= 0.0);
        Self {
            initial_solution,
            initial_demon_energy,
            rng,
            _n: std::marker::PhantomData,
        }
    }
}
impl<P: Problem, N: neighbour_space::NeighbourSpace<P>, R: rng::Rng> Solver<P>
    for MicrocanonicalAnnealing<P, N, R>
{
    fn solve<T: stop_condition::Timer, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        mut stop: S,
    ) -> (Option<P::Solution>, SolverStats<T, P>) {
        let mut stats = SolverStats::new();
        let neighbour_space = N::from(&p);
        let mut current_solution = neighbour_space.to_node(&self.initial_solution);
        let mut current_obj = neighbour_space.eval(&current_solution);
        let mut best_solution = current_solution.clone();
        let mut best_obj = current_obj;
        let mut demon_energy = self.initial_demon_energy;
        stats.iter();
        stats.add_primal_bound(best_obj);
        loop {
            if stop.stop(best_obj, P::Obj::unbounded()) {
                break;
            }
            let nid = neighbour_space.random_neighbour(&current_solution, &mut self.rng);
            let nobj = neighbour_space.eval_neighbour(&current_solution, &nid);
            let delta = nobj.into() - current_obj.into();
            if demon_energy >= delta {
                current_solution = neighbour_space.neighbour(current_solution, nid);
                current_obj = nobj;
                demon_energy -= delta;
                if current_obj < best_obj {
                    best_solution = current_solution.clone();
                    best_obj = current_obj;
                    stats.add_primal_bound(best_obj);
                }
            }
            stats.iter();
        }
        stats.finish();
        (Some(neighbour_space.to_solution(&best_solution)), stats)
    }
}
