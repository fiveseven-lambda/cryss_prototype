//! 入力から型を得る

use std::collections::HashMap;

use super::*;
use crate::ty;

impl Ty {
    fn translate(&self, vars: &mut HashMap<String, ty::Ty>) -> ty::Ty {
        match self {
            Ty::Var(name) => vars
                .entry(name.to_owned())
                .or_insert_with(ty::new_var)
                .clone(),
            Ty::NonFunc { kind, args } => ty::new_non_func(
                kind.to_owned(),
                args.iter().map(|ty| ty.translate(vars)).collect(),
            ),
            Ty::Func { args, ret } => ty::new_func(
                args.iter().map(|ty| ty.translate(vars)).collect(),
                ret.translate(vars),
            ),
        }
    }
}
impl Expr {
    /// * `defs` - 関数や定数の定義．
    /// * `vars` - 変数の型を格納する．
    /// * `target` - 結果を格納する．
    pub fn get_ty(
        self,
        defs: &HashMap<String, Ty>,
        vars: &mut HashMap<String, ty::Ty>,
        target: &mut Vec<ty::Use>,
    ) -> ty::Ty {
        if let Some(ty) = defs.get(&self.identifier) {
            let ty = ty.translate(&mut HashMap::new());
            let calls = self
                .calls
                .into_iter()
                .map(|call| ty::Call {
                    args: call
                        .args
                        .into_iter()
                        .map(|arg| ty::Arg {
                            ty: arg.get_ty(defs, vars, target),
                        })
                        .collect(),
                })
                .collect();
            let ret_ty = ty::new_var();
            target.push(ty::Use {
                ty,
                calls,
                ret_ty: ret_ty.clone(),
            });
            ret_ty
        } else {
            let ty = vars
                .entry(self.identifier)
                .or_insert_with(ty::new_var)
                .clone();
            assert!(
                self.calls.is_empty(),
                "変数はそのままだと関数として呼べないので，derefをかませる必要がある"
            );
            ty::new_non_func(String::from("ref"), vec![ty])
        }
    }
}
