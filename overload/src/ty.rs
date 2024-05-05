use std::{cell::RefCell, rc::Rc};
pub mod _debug_print;
pub mod fmt;
pub mod unify;

pub fn new_var() -> Ty {
    Ty {
        inner: Rc::new(RefCell::new(TyInner::Returns(NonFunc {
            inner: Rc::new(RefCell::new(NonFuncInner::Undetermined)),
        }))),
    }
}
pub fn new_func(args: Vec<Ty>, ret: Ty) -> Ty {
    Ty {
        inner: Rc::new(RefCell::new(TyInner::Func { args, ret })),
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
    Func { args: Vec<Ty>, ret: Ty },
    Returns(NonFunc),
    SameAs(Ty),
}

pub struct Use {
    pub ty: Ty,
    pub calls: Vec<Call>,
    pub ret_ty: Ty,
}
pub struct Call {
    pub args: Vec<Arg>,
}
pub struct Arg {
    pub ty: Ty,
}
