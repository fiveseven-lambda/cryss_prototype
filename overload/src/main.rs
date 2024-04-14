use std::collections::HashMap;

mod expr;
mod ty;

fn main() {
    let field = {
        let (funcs, mut exprs) = {
            let mut funcs = HashMap::new();
            let mut exprs = Vec::new();
            let mut state = false;
            use std::io::{BufRead, BufReader};
            for line in
                BufReader::new(std::fs::File::open(&std::env::args().nth(1).unwrap()).unwrap())
                    .lines()
            {
                let line = line.unwrap();
                if line.is_empty() {
                    state = !state;
                } else if !state {
                    let (_, expr::Func { name, candidates }) = expr::parse_func(&line).unwrap();
                    funcs.insert(name, candidates);
                } else {
                    let (_, expr) = expr::parse_expr(&line).unwrap();
                    exprs.push(expr);
                }
            }
            (funcs, exprs)
        };
        let mut vars = HashMap::new();
        let mut num_vars = 0;
        for expr in &mut exprs {
            expr.get_vars(&funcs, &mut vars, &mut num_vars);
        }
        let mut towers = Vec::new();
        for expr in exprs {
            expr.translate(&funcs, &vars, &mut num_vars, &mut towers, None);
        }
        ty::Field(towers)
    };
    let mut vars = HashMap::new();
    field._debug_print(&vars);
    println!();
    let stdin = std::io::stdin();
    loop {
        use std::io::Write;
        print!("Tower index: ");
        std::io::stdout().flush().unwrap();
        let tower_idx: usize = {
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let input = input.trim();
            if input.is_empty() {
                break;
            }
            input.parse().unwrap()
        };
        let candidate_idx = if field.0[tower_idx].candidates.len() == 1 {
            0
        } else {
            print!("Candidate index: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let input = input.trim();
            if input.is_empty() {
                break;
            }
            input.parse().unwrap()
        };
        field.determine(&mut vars, tower_idx, candidate_idx);
        println!();
    }
}
