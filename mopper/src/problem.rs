use std::marker::PhantomData;

/// obj: X → Z
/// feas: X → 2
/// min obj(x) st feas(x) = 1
pub trait Problem: Clone {
    type X: Clone;
    type Z: Clone + Copy + PartialOrd;
    fn obj(&self, x: &Self::X) -> Self::Z;
    fn feas(&self, x: &Self::X) -> bool;
}
pub enum SolverEvent<P: Problem> {
    /// feas(x) = 1, obj(x) = z
    OptimalSolution(P::X, P::Z),
    Solution(P::X, P::Z),
    /// should only be sent if there's no solution corresponding to the bound
    PrimalBound(P::Z),
    DualBound(P::Z),
    Infeasible,
    Unlimited,
}
#[derive(Clone, Debug)]
pub struct SolverOutput<P: Problem> {
    pub best_solution: Option<(P::X, P::Z)>,
    pub primal_bound: Option<P::Z>,
    pub dual_bound: Option<P::Z>,
    pub is_feasible: Option<bool>,
    pub is_limited: Option<bool>,
}
impl<P: Problem> Default for SolverOutput<P> {
    fn default() -> Self {
        Self {
            best_solution: None,
            primal_bound: None,
            dual_bound: None,
            is_feasible: None,
            is_limited: None,
        }
    }
}
pub trait Solver<P: Problem> {
    fn solve(&mut self, p: P) -> impl Iterator<Item = SolverEvent<P>>;
    fn solve_to_end(&mut self, p: P) -> SolverOutput<P> {
        let mut ans = SolverOutput::<P>::default();
        for se in self.solve(p) {
            match se {
                SolverEvent::OptimalSolution(x, z) => {
                    ans.best_solution.replace((x, z));
                    ans.primal_bound.replace(z);
                    ans.dual_bound.replace(z);
                    ans.is_feasible = Some(true);
                    ans.is_limited = Some(true);
                }
                SolverEvent::Solution(x, z) => {
                    let b = ans.best_solution.as_ref().is_none_or(|(_xb, zb)| z < *zb);
                    if b {
                        ans.best_solution.replace((x, z));
                    }
                    ans.is_feasible = Some(true);

                    let b = ans.primal_bound.is_none_or(|zb| z < zb);
                    if b {
                        ans.primal_bound.replace(z);
                    }
                }
                SolverEvent::PrimalBound(z) => {
                    let b = ans.primal_bound.is_none_or(|zb| z < zb);
                    if b {
                        ans.primal_bound.replace(z);
                    }
                    ans.is_feasible = Some(true);
                }
                SolverEvent::DualBound(z) => {
                    let b = ans.primal_bound.is_none_or(|zb| z < zb);
                    if b {
                        ans.dual_bound.replace(z);
                    }
                    ans.is_limited = Some(true);
                }
                SolverEvent::Infeasible => {
                    ans.is_feasible = Some(false);
                }
                SolverEvent::Unlimited => {
                    ans.is_limited = Some(false);
                }
            }
            if ans.is_feasible == Some(false)
                || ans.is_limited == Some(false)
                || ans
                    .primal_bound
                    .is_some_and(|pb| ans.dual_bound.is_some_and(|db| pb >= db))
            {
                break;
            }
        }
        ans
    }
}

pub trait Reducer<P1: Problem, P2: Problem> {
    fn reduce(p: P1) -> P2;
    fn retrieve(se: SolverEvent<P2>) -> Option<SolverEvent<P1>>;
}
pub struct ReductionSolver<P1: Problem, P2: Problem, R: Reducer<P1, P2>, S: Solver<P2>> {
    s: S,
    _r: PhantomData<R>,
    _p1: PhantomData<P1>,
    _p2: PhantomData<P2>,
}
impl<P1: Problem, P2: Problem, R: Reducer<P1, P2>, S: Solver<P2>> ReductionSolver<P1, P2, R, S> {
    pub fn new(s: S) -> Self {
        Self {
            s,
            _r: PhantomData,
            _p1: PhantomData,
            _p2: PhantomData,
        }
    }
}
impl<P1: Problem, P2: Problem, R: Reducer<P1, P2>, S: Solver<P2>> Solver<P1>
    for ReductionSolver<P1, P2, R, S>
{
    fn solve(&mut self, p1: P1) -> impl Iterator<Item = SolverEvent<P1>> {
        let p2 = R::reduce(p1);
        self.s.solve(p2).filter_map(R::retrieve)
    }
}
