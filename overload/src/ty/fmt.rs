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
            TyInner::NonFunc(ret) => write!(f, "={ret}"),
            TyInner::Func {
                args: args_ty,
                ret: ret_ty,
            } => write!(
                f,
                "=({}){}",
                args_ty
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", "),
                ret_ty
            ),
            TyInner::Returns(ret) => write!(f, "=..{ret}"),
            TyInner::SameAs(ty) => write!(f, "={ty}"),
        }
    }
}
impl Display for NonFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut hasher = DefaultHasher::new();
        Rc::as_ptr(&self.inner).hash(&mut hasher);
        write!(f, "{}", hasher.finish() % 1000)?;
        match &*self.inner.borrow() {
            NonFuncInner::Determined { kind, args } => {
                write!(f, "={}", kind)?;
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
            NonFuncInner::Undetermined => write!(f, "?"),
            NonFuncInner::SameAs(ret) => write!(f, "={ret}"),
        }
    }
}
