use crate::core::tree_space::TreeSpace;
use crate::core::*;

struct BeamHeap<P: Problem, TS: TreeSpace<P>> {
    /// (dual bound, parent index in previous beam, child id)
    /// 0-based max-heap
    heap: Vec<(P::Obj, usize, Option<TS::ChildId>)>,
}
impl<P: Problem, TS: TreeSpace<P>> BeamHeap<P, TS> {
    fn new(size: usize) -> Self {
        Self {
            heap: vec![(P::Obj::unfeas(), 0, None); size],
        }
    }
    fn add(&mut self, db: P::Obj, parent: usize, cid: TS::ChildId) {
        if db < self.heap[0].0 {
            self.heap[0] = (db, parent, Some(cid));
            let mut i = 0;
            while i * 2 + 1 < self.heap.len() {
                let mut j = i * 2 + 1;
                if j + 1 < self.heap.len() && self.heap[j + 1].0 > self.heap[j].0 {
                    j += 1;
                }
                if self.heap[i].0 >= self.heap[j].0 {
                    break;
                }
                self.heap.swap(i, j);
                i = j;
            }
        }
    }
    fn get(self) -> Vec<(P::Obj, usize, TS::ChildId)> {
        self.heap
            .into_iter()
            .filter_map(|(db, p, cid)| cid.map(|cid| (db, p, cid)))
            .collect()
    }
}

pub struct BeamSearch<P: Problem, TS: TreeSpace<P>> {
    beam_width: usize,
    _p: std::marker::PhantomData<P>,
    _ts: std::marker::PhantomData<TS>,
}
impl<P: Problem, TS: TreeSpace<P>> BeamSearch<P, TS> {
    pub fn new(beam_width: usize) -> Self {
        Self {
            beam_width,
            _p: std::marker::PhantomData,
            _ts: std::marker::PhantomData,
        }
    }
}
impl<P: Problem, TS: TreeSpace<P>> Solver<P> for BeamSearch<P, TS> {
    fn solve<T: stop_condition::Timer, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        mut stop: S,
    ) -> (Option<P::Solution>, SolverStats<T, P>) {
        let mut stats = SolverStats::new();
        let ts = TS::from(&p);
        let mut beam: Vec<TS::Node> = vec![ts.root()];
        let mut best_sol: Option<P::Solution> = None;
        let mut best_obj = P::Obj::unfeas();
        stats.iter();
        loop {
            if stop.stop(best_obj, P::Obj::unbounded()) {
                break;
            }
            let mut next_beam = BeamHeap::<P, TS>::new(self.beam_width);
            for (i, n) in beam.iter().enumerate() {
                for cid in ts.children(n) {
                    let db = ts.child_dual_bound(n, &cid, best_obj);
                    if db >= best_obj {
                        continue;
                    }
                    next_beam.add(db, i, cid);
                }
                if let Some(obj) = ts.primal_bound(n) {
                    if obj < best_obj {
                        best_obj = obj;
                        best_sol = ts.to_solution(n);
                        stats.add_primal_bound(best_obj);
                    }
                }
            }
            let next_beam = next_beam.get();
            if next_beam.is_empty() {
                break;
            }
            beam = next_beam
                .into_iter()
                .map(|(_, p, cid)| ts.child(&beam[p], &cid))
                .collect();
            stats.iter();
        }
        stats.finish();
        (best_sol, stats)
    }
}
