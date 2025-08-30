use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Shl, Shr, Sub, SubAssign};

/// represents a sum of variables multiplied by some factor and a constant,
/// variables not present are assumed to be multiplied by 0
#[derive(Clone)]
pub struct Expression {
    /// the constant part of the sum
    constant: f64,
    /// variables ids with their multiplier, sorted by Id
    variables: Vec<(usize, f64)>,
}
impl Expression {
    fn cons(val: f64) -> Self {
        Self {
            constant: val,
            variables: vec![],
        }
    }
    fn from_var_id(id: usize) -> Self {
        Self {
            constant: 0f64,
            variables: vec![(id, 1f64)],
        }
    }
}
impl<T: Into<f64>> From<T> for Expression {
    fn from(t: T) -> Self {
        Self {
            constant: t.into(),
            variables: vec![],
        }
    }
}
impl<T: Into<f64> + Copy> MulAssign<T> for Expression {
    fn mul_assign(&mut self, rhs: T) {
        self.constant *= rhs.into();
        for var in self.variables.iter_mut() {
            var.1 *= rhs.into();
        }
    }
}
impl<T: Into<f64> + Copy> Mul<T> for Expression {
    type Output = Expression;
    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}
impl<T: Into<f64> + Copy> Mul<T> for &Expression {
    type Output = Expression;
    fn mul(self, rhs: T) -> Self::Output {
        self.clone() * rhs
    }
}
impl Mul<Expression> for f64 {
    type Output = Expression;
    fn mul(self, rhs: Expression) -> Expression {
        rhs * self
    }
}
impl Mul<&Expression> for f64 {
    type Output = Expression;
    fn mul(self, rhs: &Expression) -> Expression {
        rhs.clone() * self
    }
}
impl Mul<Expression> for i64 {
    type Output = Expression;
    fn mul(self, rhs: Expression) -> Expression {
        rhs * self as f64
    }
}
impl Mul<&Expression> for i64 {
    type Output = Expression;
    fn mul(self, rhs: &Expression) -> Expression {
        rhs.clone() * self as f64
    }
}

impl AddAssign<&Expression> for Expression {
    fn add_assign(&mut self, other: &Self) {
        self.constant += other.constant;
        let mut nv = Vec::<(usize, f64)>::new();
        let mut i0 = self.variables.iter().peekable();
        let mut i1 = other.variables.iter().peekable();
        while let (Some(v0), Some(v1)) = (i0.peek(), i1.peek()) {
            match v0.0.cmp(&v1.0) {
                Ordering::Less => {
                    nv.push(**v0);
                    i0.next();
                }
                Ordering::Equal => {
                    if v0.1 + v1.1 != 0.0 {
                        nv.push((v0.0, v0.1 + v1.1));
                    }
                    i0.next();
                    i1.next();
                }
                Ordering::Greater => {
                    nv.push(**v1);
                    i1.next();
                }
            }
        }
        for v0 in i0 {
            nv.push(*v0);
        }
        for v1 in i1 {
            nv.push(*v1);
        }
        self.variables = nv
    }
}
impl AddAssign for Expression {
    fn add_assign(&mut self, other: Self) {
        *self += &other;
    }
}
impl Add for Expression {
    type Output = Expression;
    fn add(mut self, rhs: Expression) -> Self::Output {
        self += rhs;
        self
    }
}
impl Add<&Expression> for Expression {
    type Output = Expression;
    fn add(self, rhs: &Expression) -> Self::Output {
        self + rhs.clone()
    }
}
impl Add for &Expression {
    type Output = Expression;
    fn add(self, rhs: &Expression) -> Self::Output {
        self.clone() + rhs.clone()
    }
}
impl Add<Expression> for &Expression {
    type Output = Expression;
    fn add(self, rhs: Expression) -> Self::Output {
        self.clone() + rhs
    }
}

impl Neg for Expression {
    type Output = Self;
    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}
impl Neg for &Expression {
    type Output = Expression;
    fn neg(self) -> Self::Output {
        -1.0 * self.clone()
    }
}
impl SubAssign<&Expression> for Expression {
    fn sub_assign(&mut self, other: &Expression) {
        *self += &-other.clone();
    }
}
impl SubAssign for Expression {
    fn sub_assign(&mut self, other: Expression) {
        *self -= &other
    }
}
impl Sub for Expression {
    type Output = Expression;
    fn sub(mut self, rhs: Expression) -> Self::Output {
        self -= rhs;
        self
    }
}
impl Sub<&Expression> for Expression {
    type Output = Expression;
    fn sub(self, rhs: &Expression) -> Self::Output {
        self - rhs.clone()
    }
}
impl Sub for &Expression {
    type Output = Expression;
    fn sub(self, rhs: &Expression) -> Self::Output {
        self.clone() - rhs.clone()
    }
}
impl Sub<Expression> for &Expression {
    type Output = Expression;
    fn sub(self, rhs: Expression) -> Self::Output {
        self.clone() - rhs
    }
}

/// represents <=
#[allow(clippy::suspicious_arithmetic_impl)]
impl Shl for Expression {
    type Output = Inequality;
    fn shl(self, rhs: Self) -> Self::Output {
        Inequality(self - rhs)
    }
}
/// represents <=
#[allow(clippy::suspicious_arithmetic_impl)]
impl Shl<&Expression> for Expression {
    type Output = Inequality;
    fn shl(self, rhs: &Expression) -> Self::Output {
        Inequality(self - rhs)
    }
}
/// represents <=
#[allow(clippy::suspicious_arithmetic_impl)]
impl Shl for &Expression {
    type Output = Inequality;
    fn shl(self, rhs: Self) -> Self::Output {
        Inequality(self - rhs)
    }
}
/// represents <=
#[allow(clippy::suspicious_arithmetic_impl)]
impl Shl<Expression> for &Expression {
    type Output = Inequality;
    fn shl(self, rhs: Expression) -> Self::Output {
        Inequality(self - rhs)
    }
}

/// represents >=
#[allow(clippy::suspicious_arithmetic_impl)]
impl Shr for Expression {
    type Output = Inequality;
    fn shr(self, rhs: Self) -> Self::Output {
        Inequality(rhs - self)
    }
}
/// represents >=
#[allow(clippy::suspicious_arithmetic_impl)]
impl Shr<&Expression> for Expression {
    type Output = Inequality;
    fn shr(self, rhs: &Expression) -> Self::Output {
        Inequality(rhs - self)
    }
}
/// represents >=
#[allow(clippy::suspicious_arithmetic_impl)]
impl Shr for &Expression {
    type Output = Inequality;
    fn shr(self, rhs: Self) -> Self::Output {
        Inequality(rhs - self)
    }
}
/// represents >=
#[allow(clippy::suspicious_arithmetic_impl)]
impl Shr<Expression> for &Expression {
    type Output = Inequality;
    fn shr(self, rhs: Expression) -> Self::Output {
        Inequality(rhs - self)
    }
}

/// corresponds to: sum of constant and all variables in the expression <= 0
#[derive(Clone)]
pub struct Inequality(Expression);

/// information on a single variable for the LP model
#[derive(Clone)]
struct VariableInfo {
    /// minimum value the variable can take
    lb: f64,
    /// maximum value the variable can take
    ub: f64,
    /// wether the variable should be restricted integers or not
    integer: bool,
    /// variable name
    name: String,
}
#[derive(Clone)]
pub struct ModelBuilder {
    variables: Vec<VariableInfo>,
    constraints: Vec<Inequality>,
}
impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            variables: vec![],
            constraints: vec![],
        }
    }
    pub fn add_var(&mut self, lb: f64, ub: f64, integer: bool, name: String) -> Expression {
        self.variables.push(VariableInfo {
            lb,
            ub,
            integer,
            name,
        });
        Expression::from_var_id(self.variables.len() - 1)
    }
    pub fn add_constraint(&mut self, constraint: Inequality) {
        self.constraints.push(constraint);
    }
    pub fn fmt_var(&self, id: usize, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if self.variables[id].name.is_empty() {
            write!(f, "var[{}]", id)
        } else {
            write!(f, "{}", self.variables[id].name)
        }
    }
    pub fn fmt_expr(&self, expr: &Expression, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for (id, mul) in expr.variables.iter() {
            write!(f, " ")?;
            if *mul > 0.0 {
                write!(f, "+")?;
            }
            if *mul < 0.0 {
                write!(f, "-")?;
            }
            if mul.abs() != 1.0 {
                write!(f, "{}·", mul.abs())?;
            }
            self.fmt_var(*id, f)?;
        }
        if expr.constant != 0.0 {
            write!(f, " ")?;
            if expr.constant > 0.0 {
                write!(f, "+")?;
            }
            write!(f, "{}", expr.constant)?;
        }
        Ok(())
    }
    pub fn fmt_ineq(&self, ineq: &Inequality, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        self.fmt_expr(&ineq.0, f)?;
        write!(f, " <= 0")
    }
}
impl Debug for ModelBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for ineq in self.constraints.iter() {
            self.fmt_ineq(ineq, f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage() {
        let mut mb = ModelBuilder::new();
        let mut x = Vec::<Expression>::new();
        for i in 0..10 {
            x.push(mb.add_var(0.0, 1.0, true, format!("x{}", i)));
        }
        mb.add_constraint(
            5.0*(-3*&x[0] + 2*&x[1]) + &x[4] - &x[5] + Expression::cons(2.0) << Expression::cons(3.0),
        );
        mb.add_constraint(x.iter().fold(Expression::cons(0.0), |acc,item| acc+item)>>Expression::cons(2.0));
        eprintln!("{:?}", mb);
    }
}
