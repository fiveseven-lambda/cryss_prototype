use super::{Node, Ty, Var};

use std::fmt::{self, Debug, Formatter};

impl Debug for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if &self.kind == "func" {
            let mut iter = self.args.iter();
            write!(
                f,
                "func({args}) -> {ret:?}",
                ret = iter.next().unwrap(),
                args = iter
                    .map(|arg| format!("{arg:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            write!(f, "{}", self.kind)?;
            if !self.args.is_empty() {
                write!(
                    f,
                    "[{}]",
                    self.args
                        .iter()
                        .map(|arg| format!("{arg:?}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
            Ok(())
        }
    }
}

impl Debug for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner.borrow())
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Determined(ty) => write!(f, "{ty:?}"),
            Node::SameAs(var) => write!(f, "{var:?}"),
            Node::Undetermined { .. } => write!(f, "{self:p}"),
        }
    }
}
