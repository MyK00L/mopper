#![allow(unused)]
use crate::{Problem, Reducer, Solver, SolverEvent};
use nalgebra::{Const, DMatrix, DVector, Dyn, Matrix, RowDVector, Vector};
use std::ops::{AddAssign, MulAssign, SubAssign};

type T = f64;

fn pivot(tab: &mut DMatrix<T>, col: usize, row: usize) {
    let scale = tab[(row, col)];
    tab.row_mut(row).unscale_mut(scale);
    for r in 0..tab.nrows() {
        if r != row {
            tab.set_row(r, &(tab.row(r) - tab.row(row) * tab[(r, col)]));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PivotChoice {
    Pivot(usize, usize),
    Optimal,
    Unlimited,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SimplexOutput {
    Optimal,
    Unlimited,
}
fn simplex<F: Fn(&DMatrix<T>) -> PivotChoice>(
    tab: &mut DMatrix<T>,
    choose_pivot: F,
) -> SimplexOutput {
    loop {
        match choose_pivot(tab) {
            PivotChoice::Pivot(col, row) => {
                pivot(tab, col, row);
            }
            PivotChoice::Optimal => {
                return SimplexOutput::Optimal;
            }
            PivotChoice::Unlimited => {
                return SimplexOutput::Unlimited;
            }
        }
    }
}
fn choose_pivot_col_min(tab: &DMatrix<T>) -> usize {
    tab.row(0)
        .iter()
        .enumerate()
        .skip(1)
        .min_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap())
        .unwrap()
        .0
}
fn choose_pivot_standard_limited<F: Fn(&DMatrix<T>) -> usize>(
    tab: &DMatrix<T>,
    choose_col: F,
    row_limit: usize,
) -> PivotChoice {
    let col = choose_col(tab);
    if tab[(0, col)] >= 0f64 {
        PivotChoice::Optimal
    } else {
        let row_opt = (1..row_limit)
            .filter_map(|ri| {
                if tab[(ri, col)] > 0f64 {
                    Some((ri, tab[(ri, col)] / tab[(ri, 0)]))
                } else {
                    None
                }
            })
            .max_by(|(_, v1), (_, v2)| v1.partial_cmp(v2).unwrap());
        row_opt
            .map(|(row, _)| PivotChoice::Pivot(col, row))
            .unwrap_or(PivotChoice::Unlimited)
    }
}
struct LPEqMin {
    a: DMatrix<T>,
    b: DVector<T>,
    c: DVector<T>,
}
impl LPEqMin {
    fn to_tableau(&self) -> DMatrix<T> {
        todo!();
    }
}

/// min c^T x:
/// x >= 0
/// al x <= bl
#[derive(Clone, Debug)]
struct LP {
    al: DMatrix<T>,
    bl: DVector<T>,
    c: DVector<T>,
}
impl LP {
    fn new(al: DMatrix<T>, bl: DVector<T>, c: DVector<T>) -> Self {
        debug_assert_eq!(al.nrows(), bl.nrows());
        if !al.is_empty() {
            debug_assert_eq!(al.ncols(), c.nrows());
        }
        Self { al, bl, c }
    }
}
impl Problem for LP {
    type X = DVector<T>;
    type Z = f64;
    fn obj(&self, x: &Self::X) -> Self::Z {
        self.c.dot(x)
    }
    fn feas(&self, x: &Self::X) -> bool {
        &self.al * x <= self.bl
    }
}
// TODO: handle degenerate problems
struct PrimalTwoPhaseSimplex {}
impl Solver<LP> for PrimalTwoPhaseSimplex {
    fn solve(&mut self, mut p: LP) -> impl Iterator<Item = SolverEvent<LP>> {
        let nvar = p.c.nrows();

        let (mi, &mb) =
            p.bl.iter()
                .enumerate()
                .min_by(|(i1, b1), (i2, b2)| b1.partial_cmp(b2).unwrap())
                .unwrap();
        let extra_slack = p.al.nrows();
        let extra_two_phase = /*p.ae.nrows() +*/ if mb < 0f64 { 1 } else { 0 };
        let extra = extra_slack + extra_two_phase;
        let total = p.al.ncols() + extra;
        let mut ei = p.al.ncols();
        p.al.resize_horizontally_mut(total, 0f64);
        //p.ae.resize_horizontally_mut(total, 0f64);
        let mut c1 = p.c.transpose();
        c1.resize_horizontally_mut(total, 0f64);

        let mut c2 = RowDVector::<f64>::zeros(total);
        let mut z0 = 0f64;

        if mb < 0f64 {
            p.al[(mi, ei + mi)] = 1f64; // slack variable
            p.al.row_mut(mi).neg_mut();
            p.bl.row_mut(mi).neg_mut();
            c2.sub_assign(p.al.row(mi));
            z0 += p.bl[mi];
            p.al[(mi, ei + extra_slack)] = 1f64;
        }
        let mic = p.al.row(mi).clone_owned();
        let bic = p.bl.row(mi).clone_owned();

        // add slack variables
        for i in 0..p.al.nrows() {
            if i != mi {
                p.al[(i, ei)] = 1f64; // slack variable
                if p.bl[i] < 0f64 {
                    // canonicalize for >= constraintns
                    if i != mi {
                        p.al.row_mut(i).add_assign(&mic);
                        p.bl.row_mut(i).add_assign(&bic);
                    }
                }
            }
            ei += 1;
        }

        // add two-phase extra variables
        if mb < 0f64 {
            ei += 1; // added earlier
        }
        /*
        for i in 0..p.ae.nrows() {
            if p.be[i] < 0f64 {
                p.be.row_mut(i).neg_mut();
                p.ae.row_mut(i).neg_mut();
            }
            c2.sub_assign(p.ae.row(i));
            z0 += p.be[i];

            p.ae[(i, ei)] = 1f64;

            ei += 1;
        }
        */

        #[allow(clippy::toplevel_ref_arg)]
        let mut tab = nalgebra::stack![
        nalgebra::matrix![-z0], c2;
        p.bl, p.al;
        //p.be, p.ae;
        nalgebra::matrix![0f64], c1
        ];

        let out = simplex(&mut tab, |tab: &DMatrix<T>| {
            choose_pivot_standard_limited(tab, choose_pivot_col_min, tab.nrows() - 1)
        });
        debug_assert!(out == SimplexOutput::Optimal);
        if tab[(0, 0)] != 0f64 {
            return [SolverEvent::<LP>::Infeasible].into_iter();
        }

        let new_width = tab.ncols() - extra_two_phase;
        let new_height = tab.nrows() - 1;
        tab.resize_horizontally_mut(new_width, 0f64);
        tab.swap_rows(0, new_height);
        tab.resize_vertically_mut(new_height, 0f64);
        let out = simplex(&mut tab, |tab: &DMatrix<T>| {
            choose_pivot_standard_limited(tab, choose_pivot_col_min, tab.nrows())
        });
        if out == SimplexOutput::Unlimited {
            return [SolverEvent::<LP>::Unlimited].into_iter();
        }

        let x: DVector<T> = DVector::<T>::from_iterator(
            nvar,
            (1..=nvar).map(|ci| {
                let mut n1 = false;
                let mut ans = 0f64;
                for ri in 0..tab.nrows() {
                    if tab[(ri, ci)] == 1f64 {
                        if n1 {
                            return 0f64;
                        }
                        n1 = true;
                        ans = tab[(ri, 0)];
                    } else if tab[(ri, ci)] != 0f64 {
                        return 0f64;
                    }
                }
                ans
            }),
        );
        let z = -tab[(0, 0)];

        [SolverEvent::<LP>::OptimalSolution(x, z)].into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_a() {
        use nalgebra::{dmatrix, dvector};
        // x1+x2 <= 2
        // x1 <= 1
        // x1+x2 >= 1/2
        // min -2x1-x2
        let al = dmatrix![1f64, 1f64; 1f64, 0f64; -1f64, -1f64];
        let bl = dvector![2f64, 1f64, -0.5f64];
        let c = dvector![-2f64, -1f64];

        let lp = LP::new(al, bl, c);
        let mut solver = PrimalTwoPhaseSimplex {};
        let out = solver.solve_to_end(lp);

        assert_eq!(out.best_solution, Some((dvector![1f64, 1f64], -3f64)));
    }
}
