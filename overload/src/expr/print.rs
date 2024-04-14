use super::*;

use std::fmt::{self, Display, Formatter};

impl Display for LiteralTy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        if !self.1.is_empty() {
            write!(
                f,
                "[{}]",
                self.1
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(ty) => write!(f, ":{}", ty),
            Expr::Call {
                num_tower,
                name,
                argss,
            } => {
                write!(f, "{num_tower}:{name}")?;
                for args in argss {
                    write!(
                        f,
                        "({})",
                        args.iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )?;
                }
                Ok(())
            }
        }
    }
}
impl Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Var(name) => write!(f, "${}", name),
            Ty::Const(constructor, args) => {
                write!(f, "{}", constructor)?;
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
            Ty::Func(args, ret) => {
                write!(
                    f,
                    "({}){}",
                    args.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret
                )
            }
        }
    }
}
