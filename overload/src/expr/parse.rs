use super::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0},
    combinator::{map, opt},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};

fn parse_literal_ty(input: &str) -> IResult<&str, LiteralTy> {
    let parser = map(
        pair(
            alpha1,
            opt(delimited(
                tag("["),
                separated_list0(tag(","), parse_literal_ty),
                tag("]"),
            )),
        ),
        |(constructor, args)| LiteralTy(constructor.to_owned(), args.unwrap_or_else(Vec::new)),
    );
    delimited(multispace0, parser, multispace0)(input)
}
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let parser = alt((
        map(preceded(tag(":"), parse_literal_ty), |ty| Expr::Literal(ty)),
        map(
            pair(
                alpha1,
                many0(delimited(
                    tag("("),
                    separated_list0(tag(","), parse_expr),
                    tag(")"),
                )),
            ),
            |(name, argss)| Expr::Call {
                num_tower: 0,
                name: name.to_string(),
                argss,
            },
        ),
    ));
    delimited(multispace0, parser, multispace0)(input)
}
fn parse_ty(input: &str) -> IResult<&str, Ty> {
    let parser = alt((
        map(
            pair(
                alpha1,
                opt(delimited(
                    tag("["),
                    separated_list0(tag(","), parse_ty),
                    tag("]"),
                )),
            ),
            |(constructor, args)| Ty::Const(constructor.to_owned(), args.unwrap_or_else(Vec::new)),
        ),
        map(preceded(tag("$"), alpha1), |name| {
            Ty::Var(String::from(name))
        }),
        map(
            pair(
                delimited(tag("("), separated_list0(tag(","), parse_ty), tag(")")),
                parse_ty,
            ),
            |(args, ret)| Ty::Func(args, Box::new(ret)),
        ),
    ));
    delimited(multispace0, parser, multispace0)(input)
}
pub fn parse_func(input: &str) -> IResult<&str, Func> {
    let parser = map(
        separated_pair(alpha1, tag(":"), separated_list1(tag(","), parse_ty)),
        |(name, candidates)| Func {
            name: String::from(name),
            candidates,
        },
    );
    delimited(multispace0, parser, multispace0)(input)
}
