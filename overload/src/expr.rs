mod parse;
pub use parse::{parse_expr, parse_func};
mod print;
mod translate;

pub struct Func {
    pub name: String,
    pub candidates: Vec<Ty>,
}
pub enum Expr {
    Literal(LiteralTy),
    Call {
        num_tower: usize,
        name: String,
        argss: Vec<Vec<Expr>>,
    },
}
pub struct LiteralTy(String, Vec<LiteralTy>);
pub enum Ty {
    Var(String),
    Const(String, Vec<Ty>),
    Func(Vec<Ty>, Box<Ty>),
}
