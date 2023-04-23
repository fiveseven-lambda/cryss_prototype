#[derive(Clone, Copy, Debug)]
pub enum Func {
    Deref,
    BoolToInt,
    IntToFloat,
    Const,
    App,
}

#[derive(Clone)]
pub enum Expr {
    Atom(&'static str),
    Func(Func),
    Call(Box<Expr>, Vec<Expr>),
}

use std::fmt::{self, Debug, Formatter};
impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Expr::Atom(str) => write!(f, "{str}"),
            Expr::Func(func) => write!(f, "{func:?}"),
            Expr::Call(ref func, ref args) => {
                write!(
                    f,
                    "{func:?}({})",
                    args.iter()
                        .map(|arg| format!("{arg:?}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}
