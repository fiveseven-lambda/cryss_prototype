use super::*;
use std::{
    fmt::{self, Display, Formatter},
    hash::{DefaultHasher, Hash, Hasher},
};

impl Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut hasher = DefaultHasher::new();
        Rc::as_ptr(&self.inner).hash(&mut hasher);
        write!(f, "{}", hasher.finish() % 1000)?;
        match &*self.inner.borrow() {
            Inner::Determined(kind, args) => {
                write!(f, ":{kind}")?;
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
            Inner::Func(args, ret) => {
                write!(
                    f,
                    ":({}){ret}",
                    args.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Inner::Returns(ret) => write!(f, "->{ret}"),
            Inner::Undetermined => write!(f, "?"),
            Inner::SameAs(self_equiv) => write!(f, "={self_equiv}"),
        }
    }
}
