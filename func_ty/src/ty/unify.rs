use std::collections::VecDeque;

use super::*;

pub trait Unify {
    fn unify(&self, other: &Self) -> bool;
}
impl<T: Unify> Unify for Vec<T> {
    fn unify(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .zip(other)
                .all(|(self_elem, other_elem)| self_elem.unify(other_elem))
    }
}

impl Unify for NonFunc {
    fn unify(&self, other: &NonFunc) -> bool {
        let self_binding = self.inner.borrow();
        let other_binding = other.inner.borrow();
        match (&*self_binding, &*other_binding) {
            (
                NonFuncInner::Determined {
                    kind: self_kind,
                    args: self_args,
                },
                NonFuncInner::Determined {
                    kind: other_kind,
                    args: other_args,
                },
            ) => self_kind == other_kind && self_args.unify(other_args),
            (NonFuncInner::Undetermined, _) => {
                drop(self_binding);
                *self.inner.borrow_mut() = NonFuncInner::SameAs(other.clone());
                true
            }
            (_, NonFuncInner::Undetermined) => {
                drop(other_binding);
                *other.inner.borrow_mut() = NonFuncInner::SameAs(self.clone());
                true
            }
            (NonFuncInner::SameAs(self_equiv), _) => {
                drop(other_binding);
                self_equiv.unify(other)
            }
            (_, NonFuncInner::SameAs(other_equiv)) => {
                drop(self_binding);
                self.unify(other_equiv)
            }
        }
    }
}

impl Unify for Ty {
    fn unify(&self, other: &Ty) -> bool {
        let self_binding = self.inner.borrow();
        let other_binding = other.inner.borrow();
        match (&*self_binding, &*other_binding) {
            (TyInner::Returns(self_ret), _) => {
                if other.unify_ret(self_ret) {
                    drop(self_binding);
                    *self.inner.borrow_mut() = TyInner::SameAs(other.clone());
                    true
                } else {
                    false
                }
            }
            (_, TyInner::Returns(other_ret)) => {
                if self.unify_ret(other_ret) {
                    drop(other_binding);
                    *other.inner.borrow_mut() = TyInner::SameAs(self.clone());
                    true
                } else {
                    false
                }
            }
            (
                TyInner::Func {
                    args_ty: self_args_ty,
                    ret_ty: self_ret_ty,
                },
                TyInner::Func {
                    args_ty: other_args_ty,
                    ret_ty: other_ret_ty,
                },
            ) => self_ret_ty.unify(other_ret_ty) && self_args_ty.unify(other_args_ty),
            (TyInner::NonFunc(self_equiv), TyInner::NonFunc(other_equiv)) => {
                self_equiv.unify(other_equiv)
            }
            (TyInner::Func { .. }, TyInner::NonFunc(_)) => false,
            (TyInner::NonFunc(_), TyInner::Func { .. }) => false,
            (TyInner::SameAs(self_equiv), _) => {
                drop(other_binding);
                self_equiv.unify(other)
            }
            (_, TyInner::SameAs(other_equiv)) => {
                drop(self_binding);
                self.unify(other_equiv)
            }
        }
    }
}

impl Ty {
    fn unify_ret(&self, other: &NonFunc) -> bool {
        let self_binding = self.inner.borrow();
        match &*self_binding {
            TyInner::Returns(self_ret) => self_ret.unify(other),
            TyInner::NonFunc(self_equiv) => self_equiv.unify(other),
            TyInner::Func { args_ty: _, ret_ty } => ret_ty.unify_ret(other),
            TyInner::SameAs(self_equiv) => self_equiv.unify_ret(other),
        }
    }
}

pub enum Requirements {
    Calls(VecDeque<Vec<Ty>>),
    Unknown,
    Impossible,
}

impl Ty {
    pub fn returns(&self, other: &Ty) -> Requirements {
        let self_binding = self.inner.borrow();
        let other_binding = other.inner.borrow();
        match (&*self_binding, &*other_binding) {
            (TyInner::NonFunc(self_equiv), TyInner::NonFunc(other_equiv)) => {
                if self_equiv.unify(other_equiv) {
                    Requirements::Calls(VecDeque::new())
                } else {
                    Requirements::Impossible
                }
            }
            (
                TyInner::Func {
                    args_ty: self_args_ty,
                    ret_ty: self_ret_ty,
                },
                TyInner::NonFunc(_),
            ) => {
                let mut tmp = self_ret_ty.returns(other);
                if let Requirements::Calls(calls) = &mut tmp {
                    calls.push_front(self_args_ty.clone());
                }
                tmp
            }
            (TyInner::Returns(self_ret), _) => {
                if other.unify_ret(self_ret) {
                    Requirements::Unknown
                } else {
                    Requirements::Impossible
                }
            }
            (
                TyInner::Func {
                    args_ty: self_args_ty,
                    ret_ty: self_ret_ty,
                },
                TyInner::Func {
                    args_ty: other_args_ty,
                    ret_ty: other_ret_ty,
                },
            ) => {
                let mut tmp = self_ret_ty.returns(other_ret_ty);
                if let Requirements::Calls(calls) = &mut tmp {
                    calls.push_front(self_args_ty.clone());
                    let args_ty = calls.pop_back().unwrap();
                    if args_ty.len() != other_args_ty.len() {
                        return Requirements::Impossible;
                    }
                    for (arg_ty, other_arg_ty) in args_ty.iter().zip(other_args_ty) {
                        if !arg_ty.unify(other_arg_ty) {
                            return Requirements::Impossible;
                        }
                    }
                }
                tmp
            }
            (TyInner::NonFunc(self_equiv), TyInner::Returns(other_ret)) => {
                if self_equiv.unify(other_ret) {
                    drop(other_binding);
                    *other.inner.borrow_mut() = TyInner::SameAs(self.clone());
                    Requirements::Calls(VecDeque::new())
                } else {
                    return Requirements::Impossible;
                }
            }
            (TyInner::Func { args_ty: _, ret_ty }, TyInner::Returns(other_ret)) => {
                if ret_ty.unify_ret(other_ret) {
                    Requirements::Unknown
                } else {
                    return Requirements::Impossible;
                }
            }
            (TyInner::NonFunc(_), TyInner::Func { .. }) => Requirements::Impossible,
            (TyInner::SameAs(self_equiv), _) => {
                drop(other_binding);
                self_equiv.returns(other)
            }
            (_, TyInner::SameAs(other_equiv)) => {
                drop(self_binding);
                self.returns(other_equiv)
            }
        }
    }
}
