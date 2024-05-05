use std::{cell::RefCell, rc::Rc};

mod fmt;
pub mod unify;

pub fn new_var() -> Ty {
    Ty {
        inner: Rc::new(RefCell::new(TyInner::Returns(NonFunc {
            inner: Rc::new(RefCell::new(NonFuncInner::Undetermined)),
        }))),
    }
}
pub fn new_func(args_ty: Vec<Ty>, ret_ty: Ty) -> Ty {
    Ty {
        inner: Rc::new(RefCell::new(TyInner::Func { args_ty, ret_ty })),
    }
}
pub fn new_non_func(kind: String, args: Vec<Ty>) -> Ty {
    Ty {
        inner: Rc::new(RefCell::new(TyInner::NonFunc(NonFunc {
            inner: Rc::new(RefCell::new(NonFuncInner::Determined { kind, args })),
        }))),
    }
}

#[derive(Clone)]
struct NonFunc {
    inner: Rc<RefCell<NonFuncInner>>,
}
enum NonFuncInner {
    Undetermined,
    Determined { kind: String, args: Vec<Ty> },
    SameAs(NonFunc),
}
#[derive(Clone)]
pub struct Ty {
    inner: Rc<RefCell<TyInner>>,
}
enum TyInner {
    NonFunc(NonFunc),
    Func { args_ty: Vec<Ty>, ret_ty: Ty },
    Returns(NonFunc),
    SameAs(Ty),
}
