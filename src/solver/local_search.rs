use crate::core::*;

pub struct FirstImprovingRandomLocalSearch<
    P: Problem,
    N: neighbour_space::NeighbourSpace<P>,
    R: rng::Rng,
> {
    initial_solution: Option<P::Solution>,
    rng: R,
    _n: std::marker::PhantomData<N>,
}
impl<P: Problem, N: neighbour_space::NeighbourSpace<P>, R: rng::Rng>
    FirstImprovingRandomLocalSearch<P, N, R>
{
    pub fn new(initial_solution: P::Solution, rng: R) -> Self {
        Self {
            initial_solution: Some(initial_solution),
            rng,
            _n: std::marker::PhantomData,
        }
    }
}
impl<P: Problem, N: neighbour_space::NeighbourSpace<P>, R: rng::Rng> Solver<P>
    for FirstImprovingRandomLocalSearch<P, N, R>
{
    fn solve<T: stop_condition::Timer, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        mut stop: S,
    ) -> (Option<P::Solution>, SolverStats<T, P>) {
        let mut stats = SolverStats::new();
        let neighbour_space = N::from(&p);
        let mut solution = neighbour_space.to_node(self.initial_solution.take().unwrap());
        let mut obj = neighbour_space.eval(&solution);
        stats.add_primal_bound(obj);
        stats.iter();
        loop {
            if stop.stop(obj, P::Obj::unbounded()) {
                break;
            }
            let nid = neighbour_space.random_neighbour(&solution, &mut self.rng);
            let nobj = neighbour_space.eval_neighbour(&solution, &nid);
            if nobj < obj {
                solution = neighbour_space.neighbour(solution, nid);
                obj = nobj;
                stats.add_primal_bound(obj);
            }
            stats.iter();
        }
        stats.finish();
        (Some(neighbour_space.to_solution(solution)), stats)
    }
}

pub struct SteepestDescentLocalSearch<P: Problem, N: neighbour_space::NeighbourSpace<P>> {
    initial_solution: Option<P::Solution>,
    _n: std::marker::PhantomData<N>,
}
impl<P: Problem, N: neighbour_space::NeighbourSpace<P>> SteepestDescentLocalSearch<P, N> {
    pub fn new(initial_solution: P::Solution) -> Self {
        Self {
            initial_solution: Some(initial_solution),
            _n: std::marker::PhantomData,
        }
    }
}
impl<P: Problem, N: neighbour_space::NeighbourSpace<P>> Solver<P>
    for SteepestDescentLocalSearch<P, N>
{
    fn solve<T: stop_condition::Timer, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        mut stop: S,
    ) -> (Option<P::Solution>, SolverStats<T, P>) {
        let mut stats = SolverStats::new();
        let neighbour_space = N::from(&p);
        let mut solution = neighbour_space.to_node(self.initial_solution.take().unwrap());
        let mut obj = neighbour_space.eval(&solution);
        stats.add_primal_bound(obj);
        loop {
            if stop.stop(obj, P::Obj::unbounded()) {
                break;
            }
            stats.iter();
            let mut best_nid = None;
            let mut best_nobj = obj;
            for nid in neighbour_space.neighbourhood(&solution) {
                let nobj = neighbour_space.eval_neighbour(&solution, &nid);
                if nobj < best_nobj {
                    best_nobj = nobj;
                    best_nid = Some(nid);
                }
            }
            if let Some(nid) = best_nid {
                solution = neighbour_space.neighbour(solution, nid);
                obj = best_nobj;
                stats.add_primal_bound(obj);
            } else {
                break;
            }
        }
        stats.finish();
        (Some(neighbour_space.to_solution(solution)), stats)
    }
}
