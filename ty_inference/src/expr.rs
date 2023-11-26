use std::{cell::OnceCell, collections::HashMap};

use crate::ty;

mod debug_print;

pub enum Expr {
    Var(usize),
    Const(ty::Var),
    Func(usize, OnceCell<usize>),
    Call(Box<Expr>, Vec<Expr>),
}

impl Expr {
    pub fn ty<'expr>(
        &'expr self,
        vars: &mut HashMap<usize, ty::Var>,
        func_candidates: &[impl Fn() -> Vec<ty::Var>],
        equations: &mut Vec<(&'expr OnceCell<usize>, Vec<ty::Var>, ty::Var)>,
    ) -> ty::Var {
        match *self {
            Expr::Var(var_id) => vars.entry(var_id).or_insert_with(ty::Var::new).clone(),
            Expr::Const(ref ty) => ty.clone(),
            Expr::Func(func_id, ref ans) => {
                let func_ty = ty::Var::new();
                equations.push((ans, func_candidates[func_id](), func_ty.clone()));
                func_ty
            }
            Expr::Call(ref func, ref args) => {
                let func_ty = func.ty(vars, func_candidates, equations);
                let ret_ty = ty::Var::new();
                let mut ret_and_args_ty = vec![ret_ty.clone()];
                ret_and_args_ty.extend(
                    args.iter()
                        .map(|arg| arg.ty(vars, func_candidates, equations)),
                );
                func_ty.unify(
                    &ty::Var::ty(String::from("func"), ret_and_args_ty),
                    &mut ty::History::new(),
                );
                ret_ty
            }
        }
    }
}
