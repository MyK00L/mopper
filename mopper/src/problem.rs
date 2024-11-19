use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::marker::PhantomData;

/// Type for the objective function of a problem
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum Obj<Z: Clone + Copy + PartialOrd + Debug> {
    Unlimited,
    Some(Z),
    Infeasible,
}

/// obj: X → Z
/// feas: X → 2
/// min obj(x) st feas(x) = 1
pub trait Problem: Clone {
    type X: Clone;
    type Z: PartialEq + Clone + Copy + PartialOrd + Debug;
    fn obj(&self, x: &Self::X) -> Self::Z;
    fn feas(&self, x: &Self::X) -> bool;
}
pub enum SolverEvent<P: Problem> {
    /// feas(x) = 1, obj(x) = z
    Solution(P::X, P::Z),
    /// should only be sent if there's no solution corresponding to the bound
    PrimalBound(Obj<P::Z>),
    DualBound(Obj<P::Z>),
}

#[derive(Clone, Debug)]
pub struct SolverOutput<P: Problem> {
    pub best_solution: Option<(P::X, P::Z)>,
    pub primal_bound: Obj<P::Z>,
    pub dual_bound: Obj<P::Z>,
}
impl<P: Problem> Default for SolverOutput<P> {
    fn default() -> Self {
        Self {
            best_solution: None,
            primal_bound: Obj::Infeasible,
            dual_bound: Obj::Unlimited,
        }
    }
}
pub trait Solver<P: Problem> {
    fn solve(&mut self, p: P) -> impl Iterator<Item = SolverEvent<P>>;
    fn solve_to_end(&mut self, p: P) -> SolverOutput<P> {
        let mut ans = SolverOutput::<P>::default();
        for se in self.solve(p) {
            match se {
                SolverEvent::Solution(x, z) => {
                    let b = ans.best_solution.as_ref().is_none_or(|(_xb, zb)| z < *zb);
                    if b {
                        ans.best_solution.replace((x, z));
                    }

                    if Obj::Some(z) <= ans.primal_bound {
                        ans.primal_bound = Obj::Some(z);
                    }
                }
                SolverEvent::PrimalBound(z) => {
                    if z <= ans.primal_bound {
                        ans.primal_bound = z;
                    }
                }
                SolverEvent::DualBound(z) => {
                    if z >= ans.dual_bound {
                        ans.dual_bound = z;
                    }
                }
            }
            if ans.primal_bound <= ans.dual_bound {
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

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_obj_ord() {
        let a = Obj::<f64>::Unlimited;
        let b = Obj::<f64>::Some(1f64);
        let c = Obj::<f64>::Some(2f64);
        let d = Obj::<f64>::Infeasible;
        assert!(a < b);
        assert!(b < c);
        assert!(c < d);
        assert!(a <= a);
        assert!(b <= b);
        assert!(c <= c);
        assert!(d <= d);
    }
}
