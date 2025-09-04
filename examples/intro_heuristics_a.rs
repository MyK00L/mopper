// from https://atcoder.jp/contests/intro-heuristics/tasks/intro_heuristics_a

use mopper::core::neighbour_space::*;
use mopper::core::tree_space::*;
use mopper::core::*;

/// number of days
const D: usize = 365;
/// number of contest types
const C: usize = 26;

#[derive(Debug, Clone)]
struct MyProblemInput {
    c: [u8; C],
    s: [[u32; C]; D],
}
#[derive(Debug, Clone)]
struct MyProblemOutput {
    /// forall i (0..C).contains(t[i])
    t: [u8; D],
}
#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug)]
struct MyObj(std::cmp::Reverse<i32>);
impl From<i32> for MyObj {
    fn from(value: i32) -> Self {
        Self(std::cmp::Reverse(value))
    }
}
impl Objective for MyObj {
    fn unfeas() -> Self {
        i32::MIN.into()
    }
    fn unbounded() -> Self {
        i32::MAX.into()
    }
    fn is_feas(&self) -> bool {
        *self != Self::unfeas()
    }
    fn is_bounded(&self) -> bool {
        *self != Self::unbounded()
    }
}
impl From<MyObj> for f64 {
    fn from(value: MyObj) -> Self {
        (-(value.0 .0 as f64) - 1000000.0) * 50.0
    }
}
impl Problem for MyProblemInput {
    type Sol = MyProblemOutput;
    type Obj = MyObj;
    fn obj(&self, sol: &Self::Sol) -> Self::Obj {
        let mut last_d = [0u16; C];
        let mut ans = 0i32;
        for i in 0..D {
            ans += self.s[i][sol.t[i] as usize] as i32;
            last_d[sol.t[i] as usize] = (1 + i) as u16;
            for t in 0..C {
                ans -= (((1 + i) as i32 - last_d[t] as i32) * self.c[t] as i32) as i32;
            }
        }
        ans.into()
    }
    fn is_feasible(&self, sol: &Self::Sol) -> bool {
        for i in 0..D {
            if sol.t[i] >= C as u8 {
                return false;
            }
        }
        true
    }
}
impl Reduction<MyProblemInput> for MyProblemInput {
    fn reduce_from(p: &MyProblemInput) -> Self {
        p.clone()
    }
    fn lift_solution_to(&self, sol: Self::Sol) -> <MyProblemInput as Problem>::Sol {
        sol
    }
    fn lift_obj_to(&self, obj: Self::Obj) -> <MyProblemInput as Problem>::Obj {
        obj
    }
}

#[derive(Clone)]
struct MyGenerator;
impl ProblemGenerator<MyProblemInput> for MyGenerator {
    fn generate<R: rng::Rng>(&self, rng: &mut R) -> MyProblemInput {
        let mut c = [0u8; C];
        let mut s = [[0u32; C]; D];

        for i in 0..C {
            c[i] = (rng.next_u64() % 101) as u8;
        }
        for i in 0..D {
            for j in 0..C {
                s[i][j] = (rng.next_u64() % 20001) as u32;
            }
        }

        MyProblemInput { c, s }
    }
}

#[derive(Clone)]
struct MyTree {
    prob: MyProblemInput,
    csum: i32,
}
#[derive(Clone, Debug)]
struct MyTreeNode {
    t: [u8; D],
    ti: u16,
    obj: i32,
    last_d: [u16; C],
    dec_vel: i32,
}
impl Tree<MyProblemInput> for MyTree {
    type Node = MyTreeNode;
    fn root(&self) -> Self::Node {
        MyTreeNode {
            t: [255; D],
            ti: 0,
            obj: 0,
            last_d: [0; C],
            dec_vel: 0,
        }
    }
    fn objective(&self, n: &Self::Node) -> Option<<MyProblemInput as Problem>::Obj> {
        if n.ti == D as u16 {
            Some(n.obj.into())
        } else {
            None
        }
    }
    fn to_solution(&self, n: &Self::Node) -> Option<<MyProblemInput as Problem>::Sol> {
        if n.ti == D as u16 {
            Some(MyProblemOutput { t: n.t })
        } else {
            None
        }
    }
    fn from(p: &MyProblemInput) -> Self {
        let prob = p.clone();
        let csum = prob.c.iter().map(|x| *x as i32).sum();
        Self { prob, csum }
    }
}
impl TreeIndirect<MyProblemInput> for MyTree {
    type ChildId = u8;
    fn children_id(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId> {
        if n.ti == D as u16 {
            0u8..0
        } else {
            0u8..C as u8
        }
    }
    fn child(&self, n: &Self::Node, cid: &Self::ChildId) -> Self::Node {
        let mut nn = n.clone();
        nn.dec_vel += self.csum
            - (nn.ti as i32 + 1 - nn.last_d[*cid as usize] as i32)
                * (self.prob.c[*cid as usize] as i32);
        nn.obj += self.prob.s[nn.ti as usize][*cid as usize] as i32 - nn.dec_vel;
        nn.t[nn.ti as usize] = *cid;
        nn.ti += 1;
        nn.last_d[*cid as usize] = nn.ti;
        nn
    }
}
impl TreeGuided<MyProblemInput> for MyTree {
    type Guide = MyObj;
    fn goodness(&self, n: &Self::Node) -> Self::Guide {
        n.obj.into()
    }
}
impl TreeIndirectGuided<MyProblemInput> for MyTree {
    fn child_goodness(&self, n: &Self::Node, cid: &Self::ChildId) -> Self::Guide {
        let nndec_vel = n.dec_vel + self.csum
            - (n.ti as i32 + 1 - n.last_d[*cid as usize] as i32)
                * (self.prob.c[*cid as usize] as i32);
        let nnobj = n.obj + self.prob.s[n.ti as usize][*cid as usize] as i32 - nndec_vel;
        nnobj.into()
    }
}

#[derive(Debug, Clone)]
struct MyNProblem {
    c: [u8; C],
    s: [[u32; C]; D],
}
#[derive(Debug, Clone)]
struct MySol {
    t: [u8; D],
    score: i32,
}
impl Problem for MyNProblem {
    type Sol = MySol;
    type Obj = MyObj;
    fn obj(&self, sol: &Self::Sol) -> Self::Obj {
        todo!()
    }
    fn is_feasible(&self, sol: &Self::Sol) -> bool {
        todo!()
    }
}
#[derive(Clone)]
struct MyNeighbourSpace {
    c: [u8; C],
}
impl MyNeighbourSpace {
    fn sc(&self, i: isize, j: isize, x: u8) -> i32 {
        let d = j - i;
        (d * (d - 1)) as i32 * self.c[x as usize] as i32 / 2
    }
}
impl NeighbourhoodIndirectRandom<MyNProblem> for MyNeighbourSpace {
    type NeighbourId = (u16, u8);
    fn random_neighbour_id<R: rng::Rng>(
        &self,
        _p: &MyNProblem,
        node: &<MyNProblem as Problem>::Sol,
        rng: &mut R,
    ) -> Self::NeighbourId {
        let i = rng.next_u64() as usize % D;
        let mut x = (rng.next_u64() % (C as u64 - 1)) as u8;
        if x >= node.t[i] {
            x += 1;
        }
        (i as u16, x)
    }
    fn random_neighbour(
        &self,
        p: &MyNProblem,
        mut node: <MyNProblem as Problem>::Sol,
        nid: Self::NeighbourId,
    ) -> <MyNProblem as Problem>::Sol {
        /*
        int pi=-1;
        int ni=D;
        for(int j=i-1; j>=0; --j) if(ans[j]==ans[i]) {pi=j; break;}
        for(int j=i+1; j<D; ++j) if(ans[j]==ans[i]) {ni=j; break;}
        score+=sc(i,ni,ans[i]);
        score+=sc(pi,i,ans[i]);
        score-=sc(pi,ni,ans[i]);
        score-=s[i][ans[i]];
        ans[i]=x;
        pi=-1;
        ni=D;
        for(int j=i-1; j>=0; --j) if(ans[j]==ans[i]) {pi=j; break;}
        for(int j=i+1; j<D; ++j) if(ans[j]==ans[i]) {ni=j; break;}
        score-=sc(i,ni,ans[i]);
        score-=sc(pi,i,ans[i]);
        score+=sc(pi,ni,ans[i]);
        score+=s[i][ans[i]];
         */
        let i = nid.0 as usize;
        let x = nid.1;
        let pi: isize = (0isize..i as isize)
            .rev()
            .find(|x| node.t[*x as usize] == node.t[i])
            .unwrap_or(-1);
        let ni = ((i + 1)..D)
            .find(|x| node.t[*x as usize] == node.t[i])
            .unwrap_or(D);
        node.score += self.sc(i as isize, ni as isize, node.t[i]);
        node.score += self.sc(pi as isize, i as isize, node.t[i]);
        node.score -= self.sc(pi as isize, ni as isize, node.t[i]);
        node.score -= p.s[i][node.t[i] as usize] as i32;

        node.t[i] = x;
        let pi: isize = (0isize..i as isize)
            .rev()
            .find(|x| node.t[*x as usize] == node.t[i])
            .unwrap_or(-1);
        let ni = ((i + 1)..D)
            .find(|x| node.t[*x as usize] == node.t[i])
            .unwrap_or(D);
        node.score -= self.sc(i as isize, ni as isize, node.t[i]);
        node.score -= self.sc(pi as isize, i as isize, node.t[i]);
        node.score += self.sc(pi as isize, ni as isize, node.t[i]);
        node.score += p.s[i][node.t[i] as usize] as i32;

        node
    }
    fn neighbour_obj(
        &self,
        p: &MyNProblem,
        node: &<MyNProblem as Problem>::Sol,
        nid: &Self::NeighbourId,
    ) -> <MyNProblem as Problem>::Obj {
        // not easy to do unless you actually move node, should be using a rollbackable thing, but alas
        // library still needs some way to deal with that
        todo!()
    }
}

use mopper::solver::beam_search::BeamSearch;
use mopper::test_solvers;
fn main() {
    test_solvers!(
        MyProblemInput,
        MyGenerator,
        std::time::Duration::from_secs(2),
        8,
        [
            "beam0",
            BeamSearch::<MyProblemInput, MyTree>::new(3500),
            MyProblemInput
        ]
    );
}
