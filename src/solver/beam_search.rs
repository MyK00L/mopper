use crate::core::tree_space::*;
use crate::core::*;

struct BeamHeap<P: Problem, TS: TreeIndirectGuided<P>> {
    /// (dual bound, parent index in previous beam, child id)
    /// 0-based max-heap
    heap: Vec<(TS::Guide, usize, Option<TS::ChildId>)>,
}
impl<P: Problem, TS: TreeIndirectGuided<P>> BeamHeap<P, TS> {
    fn new(size: usize) -> Self {
        Self {
            heap: vec![(TS::Guide::unfeas(), 0, None); size],
        }
    }
    fn add(&mut self, db: TS::Guide, parent: usize, cid: TS::ChildId) {
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
    fn get(self) -> Vec<(TS::Guide, usize, TS::ChildId)> {
        self.heap
            .into_iter()
            .filter_map(|(db, p, cid)| cid.map(|cid| (db, p, cid)))
            .collect()
    }
}

pub struct BeamSearch<P: Problem, TS: TreeIndirectGuided<P>> {
    beam_width: usize,
    _p: std::marker::PhantomData<P>,
    _ts: std::marker::PhantomData<TS>,
}
impl<P: Problem, TS: TreeIndirectGuided<P>> BeamSearch<P, TS> {
    pub fn new(beam_width: usize) -> Self {
        Self {
            beam_width,
            _p: std::marker::PhantomData,
            _ts: std::marker::PhantomData,
        }
    }
}
impl<P: Problem, TS: TreeIndirectGuided<P>> Solver<P> for BeamSearch<P, TS> {
    fn solve<SK: SolutionKeeper<P>, S: stop_condition::StopCondition<P::Obj>>(
        &mut self,
        p: P,
        sk: &mut SK,
        mut stop: S,
    ) {
        let ts = TS::from(&p);
        let mut beam: Vec<TS::Node> = vec![ts.root()];
        loop {
            if stop.stop(sk.best_obj(), P::Obj::unbounded()) {
                break;
            }
            sk.iter();
            let mut next_beam = BeamHeap::<P, TS>::new(self.beam_width);
            for (i, n) in beam.iter().enumerate() {
                for cid in ts.children_id(n) {
                    let goodness: <TS as TreeGuided<P>>::Guide = ts.child_goodness(n, &cid);
                    next_beam.add(goodness, i, cid);
                }
                if let Some(obj) = ts.objective(n) {
                    sk.add_solution(&ts.to_solution(n).unwrap(), obj);
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
        }
    }
}
