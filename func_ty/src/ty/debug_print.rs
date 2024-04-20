use super::*;
use std::{
    fmt::{self, Display, Formatter},
    hash::{DefaultHasher, Hash, Hasher},
};

impl Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.args.is_empty() {
            write!(f, "const ")?;
        } else {
            for args in &self.args {
                write!(
                    f,
                    "({})",
                    args.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
        }
        write!(f, "{}", self.ret)
    }
}
impl Display for Ret {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        if !self.args.is_empty() {
            write!(
                f,
                "[{}]",
                self.args
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut hasher = DefaultHasher::new();
        Rc::as_ptr(&self.inner).hash(&mut hasher);
        write!(f, "{}", hasher.finish() % 1000)?;
        match &*self.inner.borrow() {
            Inner::Determined(inner) => write!(f, ":{inner}"),
            Inner::Undetermined => write!(f, "?"),
            Inner::SameAs(node) => write!(f, "={node}"),
        }
    }
}
