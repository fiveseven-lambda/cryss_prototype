use std::collections::HashMap;

mod expr;
mod ty;
use ty::unify::Unify;

fn main() {
    let stdin = std::io::stdin();
    let mut vars = HashMap::new();
    loop {
        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 {
            break;
        }
        if input.is_empty() {
            break;
        }
        let (_, query) = expr::parse_query(&input).unwrap();
        match query {
            expr::Query::Unify(left, right) => {
                println!(
                    "{}",
                    left.into_ty(&mut vars).unify(&right.into_ty(&mut vars))
                );
            }
            expr::Query::Returns(left, right) => {
                match left.into_ty(&mut vars).returns(&right.into_ty(&mut vars)) {
                    ty::unify::Requirements::Calls(calls) => {
                        print!("possible, args: ");
                        for args in calls {
                            print!(
                                "({})",
                                args.iter()
                                    .map(ToString::to_string)
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
                        }
                        println!();
                    }
                    ty::unify::Requirements::Unknown => {
                        println!("possible, args: unknown");
                    }
                    ty::unify::Requirements::Impossible => {
                        println!("impossible");
                    }
                }
            }
            expr::Query::Print(expr) => {
                println!("{}", expr.into_ty(&mut vars));
            }
        }
    }
}
