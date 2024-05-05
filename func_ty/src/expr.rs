use crate::ty;
use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};

pub enum Query {
    Print(Expr),
    Unify(Expr, Expr),
    Returns(Expr, Expr),
}
pub fn parse_query(input: &str) -> IResult<&str, Query> {
    let parse_unify = map(
        separated_pair(parse_expr, tag("="), parse_expr),
        |(left, right)| Query::Unify(left, right),
    );
    let parse_returns = map(
        separated_pair(parse_expr, tag("returns"), parse_expr),
        |(left, right)| Query::Returns(left, right),
    );
    let parse_print = map(parse_expr, |expr| Query::Print(expr));
    alt((parse_unify, parse_returns, parse_print))(input)
}

pub enum Expr {
    Var(String),
    NonFunc(String, Vec<Expr>),
    Func {
        args_ty: Vec<Expr>,
        ret_ty: Box<Expr>,
    },
}

impl Expr {
    pub fn into_ty(self, vars: &mut HashMap<String, ty::Ty>) -> ty::Ty {
        match self {
            Expr::Var(name) => vars.entry(name).or_insert_with(ty::new_var).clone(),
            Expr::NonFunc(kind, args) => ty::new_non_func(
                kind,
                args.into_iter().map(|expr| expr.into_ty(vars)).collect(),
            ),
            Expr::Func { args_ty, ret_ty } => ty::new_func(
                args_ty.into_iter().map(|expr| expr.into_ty(vars)).collect(),
                ret_ty.into_ty(vars),
            ),
        }
    }
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let parse_non_func = map(
        pair(
            alpha1,
            opt(delimited(
                tag("["),
                separated_list0(tag(","), parse_expr),
                tag("]"),
            )),
        ),
        |(kind, args)| Expr::NonFunc(String::from(kind), args.unwrap_or_else(Vec::new)),
    );
    let parse_var = map(preceded(tag("$"), alpha1), |name| {
        Expr::Var(String::from(name))
    });
    let parse_func = map(
        pair(
            delimited(tag("("), separated_list0(tag(","), parse_expr), tag(")")),
            parse_expr,
        ),
        |(args_ty, ret_ty)| Expr::Func {
            args_ty,
            ret_ty: Box::new(ret_ty),
        },
    );
    let parse = alt((parse_non_func, parse_var, parse_func));
    delimited(multispace0, parse, multispace0)(input)
}
