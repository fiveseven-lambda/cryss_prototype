use super::Expr;

use std::fmt::{self, Debug, Formatter};

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(var_id) => write!(f, "var_{var_id}"),
            Expr::Func(func_id, selected) => match selected.get() {
                Some(id) => write!(f, "func_{func_id}_{id}"),
                None => write!(f, "func_{func_id}"),
            },
            Expr::Const(ty) => write!(f, "const_of_type_{ty:?}"),
            Expr::Call(func, args) => {
                write!(
                    f,
                    "{func:?}({})",
                    args.iter()
                        .map(|arg| format!("{arg:?}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}
