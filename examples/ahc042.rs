use mopper::core::{tree_space::*, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Cell {
    Empty,
    Oni,
    Fuku,
}
#[derive(Copy, Clone, Debug)]
enum Dir {
    U(u8),
    D(u8),
    L(u8),
    R(u8),
}
#[derive(Copy, Clone, Debug)]
struct Move(u8, Dir);
const N: u8 = 20;
#[derive(Clone, Debug)]
struct Grid([[Cell; N as usize]; N as usize]);
impl Grid {
    fn apply_move(&mut self, m: Move) {
        match m.1 {
            Dir::U(x) => {
                for i in 0..(N - x) {
                    self.0[i as usize][m.0 as usize] = self.0[(i + x) as usize][m.0 as usize];
                }
                for i in (N - x)..N {
                    self.0[i as usize][m.0 as usize] = Cell::Empty;
                }
            }
            Dir::D(x) => {
                for i in (x..N).rev() {
                    self.0[i as usize][m.0 as usize] = self.0[(i - x) as usize][m.0 as usize];
                }
                for i in 0..x {
                    self.0[i as usize][m.0 as usize] = Cell::Empty;
                }
            }
            Dir::L(x) => {
                for i in 0..(N - x) {
                    self.0[m.0 as usize][i as usize] = self.0[m.0 as usize][(i + x) as usize];
                }
                for i in (N - x)..N {
                    self.0[m.0 as usize][i as usize] = Cell::Empty;
                }
            }
            Dir::R(x) => {
                for i in (x..N).rev() {
                    self.0[m.0 as usize][i as usize] = self.0[m.0 as usize][(i - x) as usize];
                }
                for i in 0..x {
                    self.0[m.0 as usize][i as usize] = Cell::Empty;
                }
            }
        }
    }
    fn can_move(&self, m: Move) -> bool {
        match m.1 {
            Dir::U(x) => {
                for i in 0..x {
                    if self.0[i as usize][m.0 as usize] == Cell::Fuku {
                        return false;
                    }
                }
            }
            Dir::D(x) => {
                for i in (N - x)..N {
                    if self.0[i as usize][m.0 as usize] == Cell::Fuku {
                        return false;
                    }
                }
            }
            Dir::L(x) => {
                for i in 0..x {
                    if self.0[m.0 as usize][i as usize] == Cell::Fuku {
                        return false;
                    }
                }
            }
            Dir::R(x) => {
                for i in (N - x)..N {
                    if self.0[m.0 as usize][i as usize] == Cell::Fuku {
                        return false;
                    }
                }
            }
        }
        true
    }
}
#[derive(Clone)]
struct Generator;
impl ProblemGenerator<Grid> for Generator {
    fn generate<R: rng::Rng>(&self, rng: &mut R) -> Grid {
        let mut g;
        let mut poss;
        loop {
            g = Grid([[Cell::Empty; N as usize]; N as usize]);
            let mut nchosen = 0;
            while nchosen < 80 {
                let xy = rng.next_u64() % 400;
                let x = (xy / 20) as usize;
                let y = (xy % 20) as usize;
                if g.0[x][y] == Cell::Empty {
                    g.0[x][y] = Cell::Oni;
                    nchosen += 1;
                }
            }
            poss = vec![];
            for i in 0..N {
                for j in 0..N {
                    if g.0[i as usize][j as usize] == Cell::Oni {
                        g.0[i as usize][j as usize] = Cell::Fuku;
                        poss.push((i, j));
                    }
                    if g.0[i as usize][j as usize] != Cell::Empty {
                        break;
                    }
                }
            }
            for i in 0..N {
                for j in (0..N).rev() {
                    if g.0[i as usize][j as usize] == Cell::Oni {
                        g.0[i as usize][j as usize] = Cell::Fuku;
                        poss.push((i, j));
                    }
                    if g.0[i as usize][j as usize] != Cell::Empty {
                        break;
                    }
                }
            }
            for j in 0..N {
                for i in 0..N {
                    if g.0[i as usize][j as usize] == Cell::Oni {
                        g.0[i as usize][j as usize] = Cell::Fuku;
                        poss.push((i, j));
                    }
                    if g.0[i as usize][j as usize] != Cell::Empty {
                        break;
                    }
                }
            }
            for j in 0..N {
                for i in (0..N).rev() {
                    if g.0[i as usize][j as usize] == Cell::Oni {
                        g.0[i as usize][j as usize] = Cell::Fuku;
                        poss.push((i, j));
                    }
                    if g.0[i as usize][j as usize] != Cell::Empty {
                        break;
                    }
                }
            }
            if poss.len() >= 40 {
                break;
            }
        }
        while poss.len() > 40 {
            let i = rng.next_u64() as usize % poss.len();
            g.0[poss[i].0 as usize][poss[i].1 as usize] = Cell::Oni;
            poss.swap_remove(i);
        }
        g
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Debug)]
struct Obj(i32);
impl From<Obj> for f64 {
    fn from(o: Obj) -> Self {
        o.0 as f64
    }
}
impl Objective for Obj {
    fn unfeas() -> Self {
        Obj(i32::MAX)
    }
    fn unbounded() -> Self {
        Obj(i32::MIN)
    }
    fn is_feas(&self) -> bool {
        self.0 != i32::MAX
    }
    fn is_bounded(&self) -> bool {
        self.0 != i32::MIN
    }
}
impl Problem for Grid {
    type Sol = Vec<Move>;
    type Obj = Obj;
    fn obj(&self, _sol: &Self::Sol) -> Self::Obj {
        unimplemented!();
    }
    fn is_feasible(&self, _sol: &Self::Sol) -> bool {
        true
    }
}
impl Reduction<Grid> for Grid {
    fn reduce_from(p: &Grid) -> Self {
        p.clone()
    }

    fn lift_solution_to(&self, sol: Self::Sol) -> <Grid as Problem>::Sol {
        sol
    }

    fn lift_obj_to(&self, obj: Self::Obj) -> <Grid as Problem>::Obj {
        obj
    }
}

#[derive(Clone, Debug)]
struct TreeNode {
    grid: Grid,
    turn: u16,
    moves: Vec<Move>,
    noni: u8,
}
#[derive(Clone)]
struct MyTree(Grid);
impl Tree<Grid> for MyTree {
    type Node = TreeNode;
    fn root(&self) -> Self::Node {
        TreeNode {
            grid: self.0.clone(),
            turn: 0,
            moves: vec![],
            noni: 40,
        }
    }
    /// True if the node is a leaf (no children)
    fn is_leaf(&self, n: &Self::Node) -> bool {
        n.noni == 0 || n.turn >= 300
    }
    /// Returns the objective value of the solution represented by this node, only if it is a leaf
    fn objective(&self, n: &Self::Node) -> Option<Obj> {
        if self.is_leaf(n) {
            Some(Obj(n.turn as i32 + n.noni as i32 * 400))
        } else {
            None
        }
    }
    /// Converts a leaf node to a solution
    fn to_solution(&self, n: &Self::Node) -> Option<Vec<Move>> {
        if self.is_leaf(n) {
            Some(n.moves.clone())
        } else {
            None
        }
    }
    /// Constructs a tree space from a problem instance
    fn from(p: &Grid) -> Self {
        MyTree(p.clone())
    }
}
impl TreeIndirect<Grid> for MyTree {
    type ChildId = Move;
    fn children_id(&self, n: &Self::Node) -> impl Iterator<Item = Self::ChildId> {
        let mut v = Vec::new();
        for i in 0..N {
            for dd in 0..4 {
                for x in 1..11u8 {
                    let d = match dd {
                        0 => Dir::D(x),
                        1 => Dir::L(x),
                        2 => Dir::R(x),
                        _ => Dir::U(x),
                    };
                    let m = Move(i, d);
                    if n.grid.can_move(m) {
                        v.push(m);
                    } else {
                        break;
                    }
                }
            }
        }
        v.into_iter()
    }
    fn child(&self, n: &Self::Node, cid: &Self::ChildId) -> Self::Node {
        let mut new_grid = n.grid.clone();
        new_grid.apply_move(*cid);
        let mut new_moves = n.moves.clone();
        new_moves.push(*cid);
        let new_noni = new_grid
            .0
            .iter()
            .flatten()
            .filter(|&&c| c == Cell::Oni)
            .count() as u8;
        let x = match cid.1 {
            Dir::L(a) => a,
            Dir::R(a) => a,
            Dir::U(a) => a,
            Dir::D(a) => a,
        };
        TreeNode {
            grid: new_grid,
            turn: n.turn + x as u16,
            moves: new_moves,
            noni: new_noni,
        }
    }
}
impl TreeGuided<Grid> for MyTree {
    type Guide = Obj;
    fn goodness(&self, n: &Self::Node) -> Self::Guide {
        let mut vals = [[0; 20]; 20];
        for i in 0..20 {
            let mut v = 1;
            for j in 0..20 {
                vals[i][j] = vals[i][j].min(v);
                v += match n.grid.0[i][j] {
                    Cell::Empty => 1,
                    Cell::Oni => 0,
                    Cell::Fuku => 2,
                }
            }
        }
        for i in 0..20 {
            let mut v = 1;
            for j in (0..20).rev() {
                vals[i][j] = vals[i][j].min(v);
                v += match n.grid.0[i][j] {
                    Cell::Empty => 1,
                    Cell::Oni => 0,
                    Cell::Fuku => 2,
                }
            }
        }
        for j in 0..20 {
            let mut v = 1;
            for i in 0..20 {
                vals[i][j] = vals[i][j].min(v);
                v += match n.grid.0[i][j] {
                    Cell::Empty => 1,
                    Cell::Oni => 0,
                    Cell::Fuku => 2,
                }
            }
        }
        for j in 0..20 {
            let mut v = 1;
            for i in (0..20).rev() {
                vals[i][j] = vals[i][j].min(v);
                v += match n.grid.0[i][j] {
                    Cell::Empty => 1,
                    Cell::Oni => 0,
                    Cell::Fuku => 2,
                }
            }
        }
        let mut ans = n.turn as i32;
        for i in 0..20 {
            for j in 0..20 {
                if n.grid.0[i][j] == Cell::Oni {
                    ans += 500 + vals[i][j] * 10;
                }
            }
        }
        Obj(ans)
    }
}
impl TreeIndirectGuided<Grid> for MyTree {
    fn child_goodness(&self, n: &Self::Node, cid: &Self::ChildId) -> Self::Guide {
        self.goodness(&self.child(n, cid)) // TODO: optimize(?)
    }
}

use mopper::solver::beam_search::BeamSearch;
use mopper::test_solvers;
fn main() {
    test_solvers!(
        Grid,
        Generator,
        std::time::Duration::from_secs(4),
        3,
        ["beam1", BeamSearch::<Grid, MyTree>::new(40), Grid]
    );
}
