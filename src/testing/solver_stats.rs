use crate::core::*;

pub struct SolverEvent<T: Timer, P: Problem> {
    pub time: T::Instant,
    pub it: u64,
    pub primal_bound: P::Obj,
    pub dual_bound: P::Obj,
}
pub struct SolverStats<T: Timer, P: Problem, SK: SolutionKeeper<P>> {
    pub its: u64,
    pub events: Vec<SolverEvent<T, P>>,
    pub start_time: T::Instant,
    pub last_time: T::Instant,
    pub timer: T,
    pub underlying: SK,
    pub problem: P,
}
impl<T: Timer, P: Problem, SK: SolutionKeeper<P>> SolutionKeeper<P> for SolverStats<T, P, SK> {
    fn add_solution(&mut self, sol: &P::Sol, obj: P::Obj) {
        self.events.push(SolverEvent {
            time: self.timer.time(),
            it: self.its,
            primal_bound: self.current_primal().min(obj),
            dual_bound: self.current_dual(),
        });
        self.underlying.add_solution(sol, obj);
    }
    fn add_solution_fn<F: FnOnce() -> P::Sol>(&mut self, f: F, obj: P::Obj) {
        let sol = f();
        self.add_solution(&sol, obj);
    }
    fn add_dual_bound(&mut self, db: P::Obj) {
        self.events.push(SolverEvent {
            time: self.timer.time(),
            it: self.its,
            primal_bound: self.current_primal(),
            dual_bound: self.current_dual().max(db),
        });
        self.underlying.add_dual_bound(db);
    }
    fn best_solution(&self) -> Option<(P::Sol, P::Obj)> {
        self.underlying.best_solution()
    }
    fn iter(&mut self) {
        self.its += 1;
        self.last_time = self.timer.time();
        self.underlying.iter();
    }
}
impl<T: Timer, P: Problem, SK: SolutionKeeper<P>> SolverStats<T, P, SK> {
    fn current_primal(&self) -> P::Obj {
        self.events
            .last()
            .map_or(P::Obj::unfeas(), |e| e.primal_bound)
    }
    fn current_dual(&self) -> P::Obj {
        self.events
            .last()
            .map_or(P::Obj::unbounded(), |e| e.dual_bound)
    }
    pub fn new(underlying: SK, problem: P) -> Self {
        let timer = T::default();
        Self {
            its: 0,
            events: Vec::new(),
            start_time: timer.time(),
            last_time: timer.time(),
            timer,
            underlying,
            problem,
        }
    }
    pub fn finish(&mut self) {
        self.events.push(SolverEvent {
            time: self.timer.time(),
            it: self.its,
            primal_bound: self.current_primal(),
            dual_bound: self.current_dual(),
        });
    }
}
