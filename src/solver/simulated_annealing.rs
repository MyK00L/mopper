use crate::core::*;

pub trait CoolingSchedule {
    fn temperature(&mut self, obj: f64) -> f64;
}

pub struct SimulatedAnnealing<
    P: crate::core::Problem,
    N: crate::core::neighbour_space::NeighbourSpace<P>,
    R: crate::core::rng::Rng,
    CS: CoolingSchedule,
> {
    initial_solution: P::Solution,
    rng: R,
    cooling_schedule: CS,
    _n: std::marker::PhantomData<N>,
}
impl<
        P: crate::core::Problem,
        N: crate::core::neighbour_space::NeighbourSpace<P>,
        R: crate::core::rng::Rng,
        CS: CoolingSchedule,
    > SimulatedAnnealing<P, N, R, CS>
{
    pub fn new(initial_solution: P::Solution, rng: R, cooling_schedule: CS) -> Self {
        Self {
            initial_solution,
            rng,
            cooling_schedule,
            _n: std::marker::PhantomData,
        }
    }
}
impl<
        P: crate::core::Problem,
        N: crate::core::neighbour_space::NeighbourSpace<P>,
        R: crate::core::rng::Rng,
        CS: CoolingSchedule,
    > crate::core::Solver<P> for SimulatedAnnealing<P, N, R, CS>
{
    fn solve<
        T: crate::core::stop_condition::Timer,
        S: crate::core::stop_condition::StopCondition<P::Obj>,
    >(
        &mut self,
        p: P,
        timer: T,
        mut stop: S,
    ) -> (Option<P::Solution>, Vec<crate::core::SolverEvent<P>>) {
        let start_time = timer.time();
        let neighbour_space = N::from(&p);
        let mut events = Vec::new();
        let mut current_solution = neighbour_space.to_node(&self.initial_solution);
        let mut current_obj = neighbour_space.eval(&current_solution);
        let mut best_solution = current_solution.clone();
        let mut best_obj = current_obj;
        events.push(crate::core::SolverEvent {
            time: timer.time() - start_time,
            primal_bound: best_obj,
            dual_bound: P::Obj::unbounded(),
        });
        loop {
            if stop.stop(best_obj, P::Obj::unbounded()) {
                break;
            }
            let temp = self.cooling_schedule.temperature(current_obj.into());
            let nid = neighbour_space.random_neighbour(&current_solution, &mut self.rng);
            let nobj = neighbour_space.eval_neighbour(&current_solution, &nid);
            let delta = nobj.into() - current_obj.into();
            if delta < 0.0 || self.rng.next01() < (-delta / temp).exp() {
                current_solution = neighbour_space.neighbour(&current_solution, nid);
                current_obj = nobj;
                if current_obj < best_obj {
                    best_solution = current_solution.clone();
                    best_obj = current_obj;
                    events.push(crate::core::SolverEvent {
                        time: timer.time() - start_time,
                        primal_bound: best_obj,
                        dual_bound: P::Obj::unbounded(),
                    });
                }
            }
        }
        (Some(neighbour_space.to_solution(&best_solution)), events)
    }
}
