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
    pub fn cons(val: f64) -> Self {
        Self {
            constant: val,
            variables: vec![],
        }
    }
}
impl From<VariableId> for Expression {
    fn from(vid: VariableId) -> Self {
        Self {
            constant: 0f64,
            variables: vec![(vid.0, 1f64)],
        }
    }
}
impl From<&Expression> for Expression {
    fn from(e: &Expression) -> Self {
        e.clone()
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
impl<T: Into<Expression>> AddAssign<T> for Expression {
    fn add_assign(&mut self, othert: T) {
        let other = othert.into();
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
impl<T: Into<Expression>> Add<T> for Expression {
    type Output = Expression;
    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs.into();
        self
    }
}
impl<T: Into<Expression>> Add<T> for &Expression {
    type Output = Expression;
    fn add(self, rhs: T) -> Self::Output {
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
impl<T: Into<Expression>> SubAssign<T> for Expression {
    fn sub_assign(&mut self, other: T) {
        *self += &-other.into();
    }
}
impl<T: Into<Expression>> Sub<T> for Expression {
    type Output = Expression;
    fn sub(mut self, rhs: T) -> Self::Output {
        self -= rhs;
        self
    }
}
impl<T: Into<Expression>> Sub<T> for &Expression {
    type Output = Expression;
    fn sub(self, rhs: T) -> Self::Output {
        self.clone() - rhs
    }
}
/// represents <=
#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: Into<Expression>> Shl<T> for Expression {
    type Output = Inequality;
    fn shl(self, rhs: T) -> Self::Output {
        Inequality(self - rhs)
    }
}
/// represents <=
#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: Into<Expression>> Shl<T> for &Expression {
    type Output = Inequality;
    fn shl(self, rhs: T) -> Self::Output {
        Inequality(self - rhs)
    }
}
/// represents >=
#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: Into<Expression>> Shr<T> for Expression {
    type Output = Inequality;
    fn shr(self, rhs: T) -> Self::Output {
        Inequality(rhs.into() - self)
    }
}
/// represents >=
#[allow(clippy::suspicious_arithmetic_impl)]
impl<T: Into<Expression>> Shr<T> for &Expression {
    type Output = Inequality;
    fn shr(self, rhs: T) -> Self::Output {
        Inequality(rhs.into() - self)
    }
}

impl<T: Into<f64> + Copy> Mul<T> for VariableId {
    type Output = Expression;
    fn mul(self, rhs: T) -> Self::Output {
        Into::<Expression>::into(self) * rhs
    }
}
impl Mul<VariableId> for f64 {
    type Output = Expression;
    fn mul(self, rhs: VariableId) -> Expression {
        Into::<Expression>::into(rhs) * self
    }
}
impl Mul<VariableId> for i64 {
    type Output = Expression;
    fn mul(self, rhs: VariableId) -> Expression {
        self * Into::<Expression>::into(rhs)
    }
}
impl<T: Into<Expression>> Add<T> for VariableId {
    type Output = Expression;
    fn add(self, rhs: T) -> Self::Output {
        Into::<Expression>::into(self) + rhs
    }
}
impl<T: Into<Expression>> Sub<T> for VariableId {
    type Output = Expression;
    fn sub(self, rhs: T) -> Self::Output {
        Into::<Expression>::into(self) - rhs
    }
}
impl Neg for VariableId {
    type Output = Expression;
    fn neg(self) -> Self::Output {
        -Into::<Expression>::into(self)
    }
}

/// corresponds to: sum of constant and all variables in the expression <= 0
#[derive(Clone)]
pub struct Inequality(Expression);

/// information on a single variable for the LP model
#[derive(Clone)]
pub struct VariableInfo {
    /// minimum value the variable can take
    pub lb: f64,
    /// maximum value the variable can take
    pub ub: f64,
    /// wether the variable should be restricted integers or not
    pub integer: bool,
    /// variable name
    pub name: String,
}
#[derive(Clone)]
pub struct ModelBuilder {
    pub variables: Vec<VariableInfo>,
    pub constraints: Vec<Inequality>,
    next_variable_id: usize,
    next_constraint_id: usize,
}
impl Default for ModelBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VariableId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConstraintId(usize);
impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            variables: vec![],
            constraints: vec![],
            next_variable_id: 0,
            next_constraint_id: 0,
        }
    }
    pub fn add_var(&mut self, lb: f64, ub: f64, integer: bool, name: String) -> VariableId {
        self.variables.push(VariableInfo {
            lb,
            ub,
            integer,
            name,
        });
        let ret = VariableId(self.next_variable_id);
        self.next_variable_id += 1;
        ret
    }
    pub fn add_constraint(&mut self, constraint: Inequality) -> ConstraintId {
        self.constraints.push(constraint);
        let ret = ConstraintId(self.next_constraint_id);
        self.next_constraint_id += 1;
        ret
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
        let mut x = Vec::<VariableId>::new();
        for i in 0..10 {
            x.push(mb.add_var(0.0, 1.0, true, format!("x{}", i)));
        }
        mb.add_constraint((5.1 * (-3 * x[0] + 2 * x[1]) + x[4] - x[5] + 2) << 3);
        mb.add_constraint(
            x.into_iter()
                .fold(Expression::cons(0.0), |acc, item| acc + item)
                >> 2,
        );
        eprintln!("{:?}", mb);
    }
}
