use std::collections::{HashMap, VecDeque};

use crate::ty::unify::Unify;

mod input;
mod ty;

fn main() {
    let (defs, exprs) = {
        let mut defs = HashMap::new();
        let mut exprs = Vec::new();
        use std::io::{BufRead, BufReader};
        for line in
            BufReader::new(std::fs::File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines()
        {
            let line = line.unwrap();
            let (_, input) = input::parse(&line).unwrap();
            match input {
                input::Input::Def(name, ty) => {
                    defs.insert(name, ty);
                }
                input::Input::Expr(expr) => exprs.push(expr),
            }
        }
        (defs, exprs)
    };
    /*
    for (i, (name, ty)) in defs.iter().enumerate() {
        println!("Def #{i}");
        println!("{name}: {ty}");
    }
    for (i, expr) in exprs.iter().enumerate() {
        println!("Expr #{i}");
        println!("{expr}");
    }
    */
    // 変数名とその型
    let mut vars = HashMap::new();
    // 関数や変数の使用
    let mut uses = Vec::new();

    for expr in exprs {
        expr.get_ty(&defs, &mut vars, &mut uses);
    }
    println!("-- Vars --");
    for (name, ty) in &vars {
        println!("{name}: {ty}");
    }
    for (i, func) in uses.iter().enumerate() {
        println!("-- Use #{i} --");
        func._debug_print();
    }
    println!();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    loop {
        use std::io::Write;
        write!(stdout, "Use #").unwrap();
        stdout.flush().unwrap();
        let target = {
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let input = input.trim();
            if input.is_empty() {
                break;
            }
            let idx: usize = input.parse().unwrap();
            &uses[idx]
        };
        let mut ty = target.ty.clone();
        for call in &target.calls {
            let args_ty: Vec<_> = call.args.iter().map(|_| ty::new_var()).collect();
            let mut ret_ty = ty::new_var();
            ty::new_func(args_ty.clone(), ret_ty.clone()).unify(&ty);
            let mut extra_args = VecDeque::new();
            for (call_arg, func_arg) in call.args.iter().zip(&args_ty) {
                match call_arg.ty.returns(func_arg) {
                    ty::unify::Requirements::Calls(args) => {
                        for (extra_arg, arg) in extra_args.iter().zip(&args) {
                            arg.unify(extra_arg);
                        }
                        if extra_args.len() < args.len() {
                            extra_args = args
                        }
                    }
                    ty::unify::Requirements::Unknown => {}
                    ty::unify::Requirements::Impossible => {}
                }
            }
            for args in extra_args {
                ret_ty = ty::new_func(args, ret_ty)
            }
            ty = ret_ty
        }
        target.ret_ty.unify(&ty);
        for (i, func) in uses.iter().enumerate() {
            println!("-- Use #{i} --");
            func._debug_print();
        }
        println!();
    }
}
