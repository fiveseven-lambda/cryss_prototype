use std::collections::HashMap;

use crate::{expr, ty};

impl expr::LiteralTy {
    fn translate(self) -> ty::Ty {
        ty::Ty::Const(
            self.0,
            self.1.into_iter().map(expr::LiteralTy::translate).collect(),
        )
    }
}
impl expr::Ty {
    fn translate(&self, vars: &mut HashMap<String, usize>, num_vars: &mut usize) -> ty::Ty {
        match self {
            expr::Ty::Var(name) => ty::Ty::Var(*vars.entry(name.to_owned()).or_insert_with(|| {
                let index = *num_vars;
                *num_vars += 1;
                index
            })),
            expr::Ty::Const(constructor, args) => ty::Ty::Const(
                constructor.to_owned(),
                args.into_iter()
                    .map(|arg| arg.translate(vars, num_vars))
                    .collect(),
            ),
            expr::Ty::Func(args, ret) => ty::Ty::Func(
                args.into_iter()
                    .map(|arg| arg.translate(vars, num_vars))
                    .collect(),
                Box::new(ret.translate(vars, num_vars)),
            ),
        }
    }
}
impl expr::Expr {
    pub fn get_vars(
        &mut self,
        funcs: &HashMap<String, Vec<expr::Ty>>,
        vars: &mut HashMap<String, usize>,
        num_vars: &mut usize,
    ) -> usize {
        if let expr::Expr::Call {
            num_tower,
            name,
            argss,
        } = self
        {
            if let Some(_) = funcs.get(name) {
                *num_tower = argss
                    .iter_mut()
                    .map(|args| -> usize {
                        args.iter_mut()
                            .map(|arg| arg.get_vars(funcs, vars, num_vars))
                            .sum()
                    })
                    .sum();
                *num_tower + 1
            } else {
                vars.insert(name.to_owned(), *num_vars);
                *num_vars += 1;
                if !argss.is_empty() {
                    panic!();
                }
                0
            }
        } else {
            0
        }
    }
    pub fn translate(
        self,
        funcs: &HashMap<String, Vec<expr::Ty>>,
        vars: &HashMap<String, usize>,
        num_vars: &mut usize,
        target: &mut Vec<ty::Tower>,
        ret_to: Option<(usize, usize, usize)>,
    ) -> ty::Arg {
        match self {
            expr::Expr::Literal(ty) => ty::Arg {
                ty: Some(ty.translate()),
                from: None,
            },
            expr::Expr::Call {
                num_tower,
                name,
                argss,
            } => {
                if let Some(candidates) = funcs.get(&name) {
                    let candidates = candidates
                        .iter()
                        .map(|candidate| candidate.translate(&mut HashMap::new(), num_vars))
                        .collect();
                    let index = target.len() + num_tower;
                    let calls = argss
                        .into_iter()
                        .enumerate()
                        .map(|(i, args)| ty::Call {
                            args: args
                                .into_iter()
                                .enumerate()
                                .map(|(j, arg)| {
                                    arg.translate(
                                        funcs,
                                        vars,
                                        num_vars,
                                        target,
                                        Some((index, i, j)),
                                    )
                                })
                                .collect(),
                        })
                        .collect();
                    target.push(ty::Tower {
                        candidates,
                        calls,
                        ret_to,
                    });
                    ty::Arg {
                        ty: None,
                        from: Some(index),
                    }
                } else if let Some(id) = vars.get(&name) {
                    ty::Arg {
                        ty: Some(ty::Ty::Const(String::from("ref"), vec![ty::Ty::Var(*id)])),
                        from: None,
                    }
                } else {
                    panic!();
                }
            }
        }
    }
}
