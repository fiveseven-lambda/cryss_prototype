//! `Display` を impl

use super::*;

use std::fmt::{self, Display, Formatter};

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier)?;
        for call in &self.calls {
            write!(
                f,
                "({})",
                call.args
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
impl Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Var(name) => write!(f, "${}", name),
            Ty::NonFunc { kind, args } => {
                write!(f, "{}", kind)?;
                if !args.is_empty() {
                    write!(
                        f,
                        "[{}]",
                        args.iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )?;
                }
                Ok(())
            }
            Ty::Func {
                args: args_ty,
                ret: ret_ty,
            } => {
                write!(
                    f,
                    "({}){}",
                    args_ty
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret_ty
                )
            }
        }
    }
}
