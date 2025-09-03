use crate::core::*;
use neighbour_space::*;

pub trait CoolingSchedule {
    fn temperature(&mut self, obj: f64) -> f64;
}

pub struct SimulatedAnnealing<
    P: Problem,
    N: NeighbourhoodIndirectRandom<P>,
    R: rng::Rng,
    CS: CoolingSchedule,
> {
    initial_solution: Option<(P::Sol, P::Obj)>,
    rng: R,
    cooling_schedule: CS,
    ns: N,
}
impl<P: Problem, N: NeighbourhoodIndirectRandom<P>, R: rng::Rng, CS: CoolingSchedule>
    SimulatedAnnealing<P, N, R, CS>
{
    pub fn new(ns: N, initial_solution: (P::Sol, P::Obj), rng: R, cooling_schedule: CS) -> Self {
        Self {
            initial_solution: Some(initial_solution),
            rng,
            cooling_schedule,
            ns,
        }
    }
}
impl<P: Problem, N: NeighbourhoodIndirectRandom<P>, R: rng::Rng, CS: CoolingSchedule> Solver<P>
    for SimulatedAnnealing<P, N, R, CS>
{
    fn solve<SK: SolutionKeeper<P>, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        sk: &mut SK,
        mut stop: S,
    ) {
        let (mut current_solution, mut current_obj) = self.initial_solution.take().unwrap();
        loop {
            if stop.stop(sk.best_obj(), P::Obj::unbounded()) {
                break;
            }
            sk.iter();
            let temp = self.cooling_schedule.temperature(current_obj.into());
            if temp <= 0.0 {
                break;
            }
            let nid = self
                .ns
                .random_neighbour_id(&p, &current_solution, &mut self.rng);
            let nobj = self.ns.neighbour_obj(&p, &current_solution, &nid);
            let delta = nobj.into() - current_obj.into();
            if delta < 0.0 || self.rng.next01() < (-delta / temp).exp() {
                current_solution = self.ns.random_neighbour(&p, current_solution, nid);
                current_obj = nobj;
                sk.add_solution(&current_solution, current_obj);
            }
        }
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
