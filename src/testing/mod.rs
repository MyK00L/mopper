#![allow(dead_code)]

use crate::core::rng::*;
use crate::core::stop_condition::*;
use crate::core::*;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct DataPoint {
    time: std::time::Duration,
    it: u64,
    pb: f64,
    db: f64,
}

const UNFEAS: f64 = f64::INFINITY;
const UNBOUNDED: f64 = f64::NEG_INFINITY;

#[derive(Debug, Clone)]
pub struct SingleTestData {
    data_points: Vec<DataPoint>,
}
impl SingleTestData {
    pub fn lb(&self) -> f64 {
        self.data_points.last().map_or(UNFEAS, |dp| dp.pb)
    }
    pub fn ub(&self) -> f64 {
        self.data_points.last().map_or(UNBOUNDED, |dp| dp.db)
    }
    pub fn nits(&self) -> u64 {
        self.data_points.last().map_or(0, |dp| dp.it)
    }
    pub fn time(&self) -> std::time::Duration {
        self.data_points
            .last()
            .map_or(std::time::Duration::ZERO, |dp| dp.time)
    }
    fn rescale(&mut self, factor: f64) {
        for dp in self.data_points.iter_mut() {
            dp.pb *= factor;
            dp.db *= factor;
        }
    }
    /// Build from `SolverStats`, converting the objective values back to the original problem's
    fn from<OP: Problem, T: Timer, P: Problem + Reduction<OP>, SK: SolutionKeeper<P>>(
        stats: SolverStats<T, P, SK>,
    ) -> Self {
        Self {
            data_points: stats
                .events
                .iter()
                .scan(
                    DataPoint {
                        time: Duration::ZERO,
                        it: 0,
                        pb: UNFEAS,
                        db: UNBOUNDED,
                    },
                    |st, item| {
                        // pb,db,time,it should already be ordered
                        st.pb = item
                            .primal_bound
                            .as_ref()
                            .map_or(st.pb, |(_sol, obj)| stats.problem.lift_obj_to(*obj).into());
                        st.db = item
                            .dual_bound
                            .map(|obj| stats.problem.lift_obj_to(obj).into())
                            .unwrap_or(st.db);
                        st.time = item.time - stats.start_time;
                        st.it = item.it;
                        Some(*st)
                    },
                )
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AggregatedTestData {
    pub name: String,
    pub runs: Vec<SingleTestData>,
    pub avg_lb: f64,
    pub avg_ub: f64,
    pub avg_nits: f64,
    pub avg_time: Duration,
}
impl AggregatedTestData {
    fn new(name: String, runs: Vec<SingleTestData>) -> Self {
        let (mut avg_lb, mut avg_ub, mut avg_nits, mut avg_time) =
            (0.0f64, 0.0f64, 0.0f64, Duration::ZERO);
        for run in runs.iter() {
            avg_lb += run.lb();
            avg_ub += run.ub();
            avg_nits += run.nits() as f64;
            avg_time += run.time();
        }
        let nruns = runs.len() as f64;
        avg_lb /= nruns;
        avg_ub /= nruns;
        avg_nits /= nruns;
        avg_time /= nruns as u32;
        Self {
            name,
            runs,
            avg_lb,
            avg_ub,
            avg_nits,
            avg_time,
        }
    }
}

fn test_single<
    OP: Problem,
    P: Problem + Reduction<OP>,
    S: Solver<P>,
    SC: StopCondition<P::Obj>,
    T: Timer,
>(
    mut solver: S,
    problem: &OP,
    stop_condition: SC,
) -> SingleTestData {
    let reduced = P::reduce_from(problem);
    let mut stats: SolverStats<T, P, SimpleSolutionKeeper<P>> =
        SolverStats::new(SimpleSolutionKeeper::default(), reduced.clone());
    solver.solve(reduced, &mut stats, stop_condition);
    SingleTestData::from(stats)
}
fn test_solver<
    OP: Problem,
    P: Problem + Reduction<OP>,
    S: Solver<P>,
    SC: StopCondition<P::Obj>,
    T: Timer,
    G: ProblemGenerator<OP>,
    R: Rng,
>(
    name: &str,
    solver: S,
    stop_condition: SC,
    generator: G,
    seeds: &[u64],
) -> AggregatedTestData {
    let mut runs = Vec::new();
    for &seed in seeds {
        let mut rng = R::from_u64(seed);
        let prob = generator.generate(&mut rng);
        let run = test_single::<OP, P, S, SC, T>(solver.clone(), &prob, stop_condition.clone());
        runs.push(run);
    }
    AggregatedTestData::new(name.to_string(), runs)
}
pub fn test_solver_default<
    OP: Problem,
    P: Problem + Reduction<OP>,
    S: Solver<P>,
    G: ProblemGenerator<OP>,
>(
    name: &str,
    solver: S,
    generator: G,
    time: std::time::Duration,
    number: u64,
) -> AggregatedTestData {
    let seeds = (0..number).collect::<Vec<u64>>();
    test_solver::<OP, P, S, TimeStop<DefaultTimer>, DefaultTimer, G, Splitmix64>(
        name,
        solver,
        TimeStop::new(DefaultTimer::default(), time),
        generator,
        &seeds,
    )
}

#[macro_export]
macro_rules! test_solvers {
    ($op:ty, $gener:expr, $time:expr, $number:expr, [ $($name:expr, $solver:expr, $p:ty);+ ]) => {
        {
            let mut results = Vec::new();
            $(
                let res = mopper::testing::test_solver_default::<$op, $p, _, _>($name, $solver.clone(), $gener.clone(), $time, $number);
                eprintln!("{}\tobj:{}\tit:{}\ttime:{}", res.name, res.avg_lb, res.avg_nits, res.avg_time.as_millis());
                results.push(res);
            )*
            results
        }
    };
}
