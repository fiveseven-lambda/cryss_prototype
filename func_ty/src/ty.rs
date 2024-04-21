use std::{cell::RefCell, rc::Rc};
mod fmt;

#[derive(Clone)]
pub struct Ty {
    inner: Rc<RefCell<Inner>>,
}

enum Inner {
    Determined(String, Vec<Ty>),
    Func(Vec<Ty>, Ty),
    Returns(Ty),
    Undetermined,
    SameAs(Ty),
}

pub fn new_determined(kind: String, args: Vec<Ty>) -> Ty {
    Ty {
        inner: Rc::new(RefCell::new(Inner::Determined(kind, args))),
    }
}
pub fn new_func(args: Vec<Ty>, ret: Ty) -> Ty {
    Ty {
        inner: Rc::new(RefCell::new(Inner::Func(args, ret))),
    }
}
pub fn new_returns(ret: Ty) -> Ty {
    Ty {
        inner: Rc::new(RefCell::new(Inner::Returns(ret))),
    }
}
pub fn new_undetermined() -> Ty {
    Ty {
        inner: Rc::new(RefCell::new(Inner::Undetermined)),
    }
}

impl Ty {
    pub fn is_valid(&self) -> bool {
        match &*self.inner.borrow() {
            Inner::Determined(_, args) => args.iter().all(Ty::is_valid),
            Inner::Func(args, ret) => ret.is_valid() && args.iter().all(Ty::is_valid),
            Inner::Returns(ret) => {
                matches!(*ret.inner.borrow(), Inner::Undetermined) || ret.is_valid()
            }
            Inner::Undetermined => false,
            Inner::SameAs(equiv) => equiv.is_valid(),
        }
    }
    pub fn unify(&self, other: &Ty) -> bool {
        assert!(self.is_valid());
        assert!(other.is_valid());
        let (is_equiv_self, self_args, self_ret) = self.equiv_or_returns();
        let (is_equiv_other, other_args, other_ret) = other.equiv_or_returns();
        match (is_equiv_self, is_equiv_other) {
            (true, true) if self_args.len() != other_args.len() => return false,
            (true, false) if self_args.len() < other_args.len() => return false,
            (false, true) if self_args.len() > other_args.len() => return false,
            _ => {}
        };
        match (&*self_ret.inner.borrow(), &*other_ret.inner.borrow()) {
            (
                Inner::Determined(self_kind, self_args),
                Inner::Determined(other_kind, other_args),
            ) => {
                if self_kind != other_kind {
                    return false;
                }
                if self_args.len() != other_args.len() {
                    return false;
                }
                for (self_arg, other_arg) in self_args.iter().zip(other_args) {
                    if !self_arg.unify(other_arg) {
                        return false;
                    }
                }
            }
            (Inner::Undetermined, _) => *self.inner.borrow_mut() = Inner::SameAs(other.clone()),
            _ => *other.inner.borrow_mut() = Inner::SameAs(self.clone()),
        }
        for (self_elems, other_elems) in self_args.iter().zip(&other_args) {
            if self_elems.len() != other_elems.len() {
                return false;
            }
            for (self_elem, other_elem) in self_elems.iter().zip(other_elems) {
                if !self_elem.unify(other_elem) {
                    return false;
                }
            }
        }
        true
    }
    pub fn equiv_or_returns(&self) -> (bool, Vec<Vec<Ty>>, Ty) {
        match &*self.inner.borrow() {
            Inner::Determined(_, _) | Inner::Undetermined => (true, vec![], self.clone()),
            Inner::Func(args, ret) => {
                let mut tmp = ret.equiv_or_returns();
                if tmp.0 {
                    tmp.1.push(args.clone());
                }
                tmp
            }
            Inner::Returns(ret) => {
                let mut tmp = ret.equiv_or_returns();
                tmp.0 = false;
                tmp
            }
            Inner::SameAs(equiv) => equiv.equiv_or_returns(),
        }
    }
}
