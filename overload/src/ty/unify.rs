use super::*;
use std::collections::HashMap;

impl Field {
    pub fn determine(
        &self,
        vars: &mut HashMap<usize, Info>,
        tower_idx: usize,
        candidate_idx: usize,
    ) {
        let mut ty = &self.0[tower_idx].candidates[candidate_idx];
        println!("<{tower_idx}>: {}", ty._debug_print(vars));
        for call in &self.0[tower_idx].calls {
            match ty {
                Ty::Func(args, ret) => {
                    for (arg, call_arg) in args.iter().zip(&call.args) {
                        if let Some(ty) = &call_arg.ty {
                            println!(
                                "Unify {} and {}",
                                arg._debug_print(vars),
                                ty._debug_print(vars)
                            );
                        }
                        if let Some(from) = &call_arg.from {
                            println!("Propagate ..{} to <{from}>", arg._debug_print(vars));
                        }
                    }
                    ty = ret;
                }
                Ty::Var(_) => break,
                Ty::Const(_, _) => break,
            }
        }
    }
}
