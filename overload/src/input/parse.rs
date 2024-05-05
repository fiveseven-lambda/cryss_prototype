use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};

use super::*;

/// 入力をパース．
pub fn parse(input: &str) -> IResult<&str, Input> {
    // 定義
    let parse_def = map(
        separated_pair(alphanumeric1, tag(":"), parse_ty),
        |(name, ty)| Input::Def(String::from(name), ty),
    );
    // 式
    let parse_expr = map(parse_expr, |expr| Input::Expr(expr));
    let parse = alt((parse_def, parse_expr));
    delimited(multispace0, parse, multispace0)(input)
}

/// 型をパース．
fn parse_ty(input: &str) -> IResult<&str, Ty> {
    // 型変数
    let parse_var = map(preceded(tag("$"), alphanumeric1), |name| {
        Ty::Var(String::from(name))
    });
    // 非関数型
    let parse_non_func = map(
        pair(
            alphanumeric1,
            opt(delimited(
                tag("["),
                separated_list0(tag(","), parse_ty),
                tag("]"),
            )),
        ),
        |(kind, args)| Ty::NonFunc {
            kind: String::from(kind),
            args: args.unwrap_or_else(Vec::new),
        },
    );
    // 関数型
    let parse_func = map(
        pair(
            delimited(tag("("), separated_list0(tag(","), parse_ty), tag(")")),
            parse_ty,
        ),
        |(args_ty, ret_ty)| Ty::Func {
            args: args_ty,
            ret: Box::new(ret_ty),
        },
    );
    // 非関数型か，型変数か，関数型
    let parse = alt((parse_non_func, parse_var, parse_func));
    delimited(multispace0, parse, multispace0)(input)
}

/// 式をパース．
fn parse_expr(input: &str) -> IResult<&str, Expr> {
    // 関数呼び出しの引数
    let parse_call = map(
        delimited(tag("("), separated_list0(tag(","), parse_expr), tag(")")),
        |args| Call { args },
    );
    // 変数の使用：変数名
    // 関数の使用：関数名に続いて，0回以上の関数呼び出し
    let parse = map(
        pair(alphanumeric1, many0(parse_call)),
        |(identifier, calls)| Expr {
            identifier: String::from(identifier),
            calls,
        },
    );
    delimited(multispace0, parse, multispace0)(input)
}
