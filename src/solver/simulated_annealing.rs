use crate::core::*;

pub trait CoolingSchedule {
    fn temperature(&mut self, obj: f64) -> f64;
}

pub struct SimulatedAnnealing<
    P: Problem,
    N: neighbour_space::NeighbourSpace<P>,
    R: rng::Rng,
    CS: CoolingSchedule,
> {
    initial_solution: Option<P::Solution>,
    rng: R,
    cooling_schedule: CS,
    _n: std::marker::PhantomData<N>,
}
impl<P: Problem, N: neighbour_space::NeighbourSpace<P>, R: rng::Rng, CS: CoolingSchedule>
    SimulatedAnnealing<P, N, R, CS>
{
    pub fn new(initial_solution: P::Solution, rng: R, cooling_schedule: CS) -> Self {
        Self {
            initial_solution: Some(initial_solution),
            rng,
            cooling_schedule,
            _n: std::marker::PhantomData,
        }
    }
}
impl<P: Problem, N: neighbour_space::NeighbourSpace<P>, R: rng::Rng, CS: CoolingSchedule> Solver<P>
    for SimulatedAnnealing<P, N, R, CS>
{
    fn solve<T: stop_condition::Timer, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        mut stop: S,
    ) -> (Option<P::Solution>, SolverStats<T, P>) {
        let mut stats = SolverStats::new();
        let neighbour_space = N::from(&p);
        let mut current_solution = neighbour_space.to_node(self.initial_solution.take().unwrap());
        let mut current_obj = neighbour_space.eval(&current_solution);
        let mut best_solution = current_solution.clone();
        let mut best_obj = current_obj;
        stats.add_primal_bound(best_obj);
        loop {
            if stop.stop(best_obj, P::Obj::unbounded()) {
                break;
            }
            stats.iter();
            let temp = self.cooling_schedule.temperature(current_obj.into());
            if temp <= 0.0 {
                break;
            }
            let nid = neighbour_space.random_neighbour(&current_solution, &mut self.rng);
            let nobj = neighbour_space.eval_neighbour(&current_solution, &nid);
            let delta = nobj.into() - current_obj.into();
            if delta < 0.0 || self.rng.next01() < (-delta / temp).exp() {
                current_solution = neighbour_space.neighbour(current_solution, nid);
                current_obj = nobj;
                if current_obj < best_obj {
                    best_solution = current_solution.clone();
                    best_obj = current_obj;
                    stats.add_primal_bound(best_obj);
                }
            }
        }
        (Some(neighbour_space.to_solution(best_solution)), stats)
    }
}

/// arithmetic if a=1
/// geometric if b=0
/// if |a|<1 converges to b/(1-a)
pub struct ArithmeticGeometricCooling {
    temp: f64,
    a: f64,
    b: f64,
}
impl ArithmeticGeometricCooling {
    pub fn new(initial_temp: f64, a: f64, b: f64) -> Self {
        debug_assert!(initial_temp > 0.0);
        debug_assert!((0.0..=1.0).contains(&a));
        Self {
            temp: initial_temp,
            a,
            b,
        }
    }
}
impl CoolingSchedule for ArithmeticGeometricCooling {
    fn temperature(&mut self, _obj: f64) -> f64 {
        let t = self.temp;
        self.temp = self.a * self.temp + self.b;
        t
    }
}

// TODO: more cooling schedules, including adaptive ones
