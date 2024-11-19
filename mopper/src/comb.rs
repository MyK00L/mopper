use crate::*;

pub trait GroundSetX {
    /// returns true iff item i is in the set
    fn contains_item(&self, i: usize) -> bool;
    /// adds item if it was not in the set and removes it otherwise
    /// returns true if the item was added
    fn switch_item(&mut self, i: usize) -> bool;
    /// returns an iterator over the items in the set
    fn iter(&self) -> impl Iterator<Item = usize>;
    /// returns the number of elements in the ground set
    fn ground_set_size(&self) -> usize;
    /// returns the empty set
    fn empty() -> Self;
}
pub trait GroundSetProblem: Problem
where
    Self::X: GroundSetX,
{
}

pub struct GroundSetProblemEnumerationSolver {}
impl<P: GroundSetProblem> Solver<P> for GroundSetProblemEnumerationSolver
where
    P::X: GroundSetX,
{
    fn solve(&mut self, p: P) -> impl Iterator<Item = SolverEvent<P>> {
        gen move {
            let mut set = P::X::empty();
            let n = set.ground_set_size();
            let mut final_bound = Obj::Infeasible;
            for i in 1u64..(1u64 << n) {
                let lgc: u64 = (i - 1) ^ ((i - 1) >> 1);
                let gc: u64 = i ^ (i >> 1);
                let d: usize = (gc ^ lgc).trailing_zeros() as usize;
                set.switch_item(d);
                if p.feas(&set) {
                    let z = p.obj(&set);
                    if Obj::Some(z) <= final_bound {
                        final_bound = Obj::Some(z);
                        yield SolverEvent::<P>::Solution(set.clone(), z);
                    }
                }
            }
            yield SolverEvent::<P>::PrimalBound(final_bound);
            yield SolverEvent::<P>::DualBound(final_bound);
        }
    }
}
