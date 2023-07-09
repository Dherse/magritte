//! C expression parsing

use std::{borrow::Cow, fmt::Display};

use serde::{Deserialize, Serialize};

/// A parsed Vulkan C-like expression
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr<'a> {
    /// A literal string
    String(String),

    /// Resolve a variable
    Variable(Cow<'a, str>),

    /// Resolve a constant
    Constant(Cow<'a, str>),

    /// Constant integer value
    ConstantInt(i64),

    /// Constant float value
    ConstantFloat(f32),

    /// Variable resolving (i.e `a->b`)
    Resolve(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Divide operation
    Divide(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Multiply operation
    Multiply(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Add operation
    Add(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Subtract operatin
    Subtract(Box<Expr<'a>>, Box<Expr<'a>>),

    /// Bitwise not-ing
    BitwiseNot(Box<Expr<'a>>),

    /// Negative of a value
    Neg(Box<Expr<'a>>),
}

impl<'a> Display for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::String(s) => write!(f, "\"{s}\""),
            Expr::Variable(var) => write!(f, "{var}"),
            Expr::Constant(cst) => write!(f, "{cst}"),
            Expr::ConstantInt(cst) => write!(f, "{cst}"),
            Expr::ConstantFloat(cst) => write!(f, "{cst}"),
            Expr::Resolve(a, b) => write!(f, "{a}->{b}"),
            Expr::Divide(a, b) => write!(f, "{a} / {b}"),
            Expr::Multiply(a, b) => write!(f, "{a} * {b}"),
            Expr::Add(a, b) => write!(f, "{a} + {b}"),
            Expr::Subtract(a, b) => write!(f, "{a} - {b}"),
            Expr::BitwiseNot(a) => write!(f, "!{a}"),
            Expr::Neg(a) => write!(f, "-{a}"),
        }
    }
}

impl<'a> Expr<'a> {
    /// Whether the expression contains a specific variable
    pub fn contains_variable(&self, var: &str) -> bool {
        match self {
            Expr::Variable(a) => a == var,
            Expr::Resolve(a, b)
            | Expr::Divide(a, b)
            | Expr::Multiply(a, b)
            | Expr::Add(a, b)
            | Expr::Subtract(a, b) => a.contains_variable(var) || b.contains_variable(var),
            Expr::BitwiseNot(a) => a.contains_variable(var),
            _ => false,
        }
    }

    /// Computes the value, panics otherwise
    pub fn compute(&self) -> i64 {
        match self {
            Expr::ConstantInt(a) => *a,
            Expr::Divide(a, b) => a.compute() / b.compute(),
            Expr::Multiply(a, b) => a.compute() * b.compute(),
            Expr::Add(a, b) => a.compute() + b.compute(),
            Expr::Subtract(a, b) => a.compute() - b.compute(),
            Expr::BitwiseNot(a) => !a.compute(),

            _ => unreachable!("Expression cannot be computed: {:?}", self),
        }
    }

    /// List the **variables** (not constants) used in this expression
    pub fn variables(&self) -> Vec<Cow<'a, str>> {
        let mut out = Vec::default();

        self.variables_internal(&mut out);

        out
    }

    fn variables_internal(&self, out: &mut Vec<Cow<'a, str>>) {
        match self {
            Expr::Variable(var) => out.push(var.clone()),
            Expr::Resolve(var, _) => var.variables_internal(out),
            Expr::Divide(a, b) | Expr::Multiply(a, b) | Expr::Add(a, b) | Expr::Subtract(a, b) => {
                a.variables_internal(out);
                b.variables_internal(out);
            },
            Expr::BitwiseNot(var) | Expr::Neg(var) => var.variables_internal(out),
            _ => (),
        }
    }

    /// List the **constants** (not variables) used in this expression
    pub fn constants(&self) -> Vec<Cow<'a, str>> {
        let mut out = Vec::default();

        self.constants_internal(&mut out);

        out
    }

    fn constants_internal(&self, out: &mut Vec<Cow<'a, str>>) {
        match self {
            Expr::Constant(var) => out.push(var.clone()),
            Expr::Resolve(var, _) => var.constants_internal(out),
            Expr::Divide(a, b) | Expr::Multiply(a, b) | Expr::Add(a, b) | Expr::Subtract(a, b) => {
                a.constants_internal(out);
                b.constants_internal(out);
            },
            Expr::BitwiseNot(var) | Expr::Neg(var) => var.constants_internal(out),
            _ => (),
        }
    }

    pub fn as_static(&self) -> Expr<'static> {
        match self {
            Expr::String(a) => Expr::String(a.clone()),
            Expr::Variable(a) => Expr::Variable(Cow::Owned(a.to_string())),
            Expr::Constant(a) => Expr::Constant(Cow::Owned(a.to_string())),
            Expr::ConstantInt(a) => Expr::ConstantInt(*a),
            Expr::ConstantFloat(a) => Expr::ConstantFloat(*a),
            Expr::Resolve(a, b) => Expr::Resolve(Box::new(a.as_static()), Box::new(b.as_static())),
            Expr::Divide(a, b) => Expr::Divide(Box::new(a.as_static()), Box::new(b.as_static())),
            Expr::Multiply(a, b) => Expr::Multiply(Box::new(a.as_static()), Box::new(b.as_static())),
            Expr::Add(a, b) => Expr::Add(Box::new(a.as_static()), Box::new(b.as_static())),
            Expr::Subtract(a, b) => Expr::Subtract(Box::new(a.as_static()), Box::new(b.as_static())),
            Expr::BitwiseNot(a) => Expr::BitwiseNot(Box::new(a.as_static())),
            Expr::Neg(a) => Expr::Neg(Box::new(a.as_static())),
        }
    }
}
