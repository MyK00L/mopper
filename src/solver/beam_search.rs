use crate::core::tree_space::*;
use crate::core::*;
use crate::utils::set::{AlwaysEmptySet, Set};

#[derive(Clone)]
pub struct BeamSearch<P: Problem, TS: TreeIndirectGuided<P>, S: Set<TS::Node> = AlwaysEmptySet> {
    beam_width: usize,
    _p: std::marker::PhantomData<P>,
    _ts: std::marker::PhantomData<TS>,
    _s: std::marker::PhantomData<S>,
}
impl<P: Problem, TS: TreeIndirectGuided<P>, S: Set<TS::Node>> BeamSearch<P, TS, S> {
    pub fn new(beam_width: usize) -> Self {
        Self {
            beam_width,
            _p: std::marker::PhantomData,
            _ts: std::marker::PhantomData,
            _s: std::marker::PhantomData,
        }
    }
}
impl<P: Problem, TS: TreeIndirectGuided<P>, S: Set<TS::Node>> Solver<P> for BeamSearch<P, TS, S> {
    fn solve<SK: SolutionKeeper<P>, SC: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        sk: &mut SK,
        mut stop: SC,
    ) {
        let ts = TS::from(&p);
        let mut set = S::default();
        let mut beam: Vec<TS::Node> = vec![ts.root()];
        loop {
            if stop.stop(sk.best_obj(), P::Obj::unbounded()) || beam.is_empty() {
                break;
            }
            sk.iter();
            // (goodness, parent index in previous beam, child id)
            let mut next_beam = Vec::<(TS::Guide, usize, TS::ChildId)>::new();
            for (i, n) in beam.iter().enumerate() {
                for cid in ts.children_id(n) {
                    let goodness: <TS as TreeGuided<P>>::Guide = ts.child_goodness(n, &cid);
                    next_beam.push((goodness, i, cid));
                }
                if let Some(obj) = ts.objective(n) {
                    sk.add_solution_fn(|| ts.to_solution(n).unwrap(), obj);
                }
            }
            next_beam.sort_by_key(|(g, _, _)| *g);
            beam = next_beam
                .into_iter()
                .filter_map(|(_, p, cid)| {
                    let child = ts.child(&beam[p], &cid);
                    if !set.insert(&child) {
                        Some(child)
                    } else {
                        None
                    }
                })
                .take(self.beam_width)
                .collect();
        }
    }
}
