use std::{cell::OnceCell, collections::HashMap};

mod expr;
mod ty;

fn main() {
    let program = vec![
        expr::Expr::Call(
            Box::new(expr::Expr::Func(0, OnceCell::new())),
            vec![
                expr::Expr::Var(0),
                expr::Expr::Call(
                    Box::new(expr::Expr::Var(1)),
                    vec![
                        expr::Expr::Const(ty::Var::ty(String::from("T"), vec![])),
                        expr::Expr::Call(
                            Box::new(expr::Expr::Func(1, OnceCell::new())),
                            vec![expr::Expr::Var(2), expr::Expr::Var(3)],
                        ),
                    ],
                ),
            ],
        ),
        expr::Expr::Call(
            Box::new(expr::Expr::Func(0, OnceCell::new())),
            vec![expr::Expr::Var(1), expr::Expr::Func(1, OnceCell::new())],
        ),
    ];

    println!("Program:");
    for expr in &program {
        println!("    {expr:?};");
    }
    println!("  where");

    let funcs = [
        || {
            let t = ty::Var::new();
            let void = ty::Var::ty(String::from("void"), vec![]);
            vec![ty::Var::ty("func".into(), vec![void, t.clone(), t.clone()])]
        },
        || {
            vec![
                ty::Var::ty(
                    "func".into(),
                    vec![
                        ty::Var::ty(String::from("S"), vec![]),
                        ty::Var::ty(String::from("T"), vec![]),
                        ty::Var::ty(String::from("R"), vec![]),
                    ],
                ),
                ty::Var::ty(
                    "func".into(),
                    vec![
                        ty::Var::ty(String::from("R"), vec![]),
                        ty::Var::ty(String::from("U"), vec![]),
                        ty::Var::ty(String::from("V"), vec![]),
                    ],
                ),
            ]
        },
    ];

    let mut vars = HashMap::new();
    let mut equations = Vec::new();

    for expr in &program {
        expr.ty(&mut vars, &funcs, &mut equations);
    }
    for (var_id, var_ty) in &vars {
        println!("    var_{var_id}: {var_ty:?}");
    }
    for (func_id, candidates) in funcs.iter().enumerate() {
        for (candidate_id, ty) in candidates().iter().enumerate() {
            println!("    func_{func_id}_{candidate_id}: {ty:?}");
        }
    }

    println!();
    println!("Overloads to resolve:");
    for (i, (_, candidates, var)) in equations.iter().enumerate() {
        println!("[{i}] {var:?}");
        for candidate in candidates {
            println!("  =? {candidate:?}");
        }
    }

    println!();
    for i in 0.. {
        let mut resolved = Vec::new();
        for (eq_idx, &(ans, ref candidates, ref var)) in equations.iter().enumerate() {
            if ans.get().is_some() {
                continue;
            }
            let mut ok = Vec::new();
            for (candidate_idx, candidate) in candidates.iter().enumerate() {
                let mut history = ty::History::new();
                if var.unify(candidate, &mut history) {
                    ok.push(candidate_idx);
                }
                history.rollback();
            }
            if let [i] = ok[..] {
                ans.set(i).unwrap();
                var.unify(&candidates[i], &mut ty::History::new());
                resolved.push(eq_idx);
            }
        }
        println!("Trial {i}: resolved {resolved:?}");
        if resolved.is_empty() {
            break;
        }
    }

    println!();
    println!("Result:");
    for expr in &program {
        println!("    {expr:?};");
    }
    println!("  where");
    for (var_id, var_ty) in &vars {
        println!("    var_{var_id}: {var_ty:?}");
    }
}
