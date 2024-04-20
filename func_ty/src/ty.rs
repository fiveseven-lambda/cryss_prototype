use std::{cell::RefCell, rc::Rc};
mod debug_print;

pub struct Ty {
    args: Vec<Vec<Node<Ty>>>,
    ret: Node<Ret>,
}
pub fn new_ty(args: Vec<Vec<Node<Ty>>>, ret: Node<Ret>) -> Node<Ty> {
    Node {
        inner: Rc::new(RefCell::new(Inner::Determined(Ty { args, ret }))),
    }
}
pub fn new_ret(kind: String, args: Vec<Node<Ty>>) -> Node<Ret> {
    Node {
        inner: Rc::new(RefCell::new(Inner::Determined(Ret { kind, args }))),
    }
}
pub fn new_undetermined<T>() -> Node<T> {
    Node {
        inner: Rc::new(RefCell::new(Inner::Undetermined)),
    }
}
pub struct Ret {
    kind: String,
    args: Vec<Node<Ty>>,
}

pub struct Node<T> {
    inner: Rc<RefCell<Inner<T>>>,
}
impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node {
            inner: self.inner.clone(),
        }
    }
}
enum Inner<T> {
    Determined(T),
    Undetermined,
    SameAs(Node<T>),
}
pub trait Unify {
    fn unify(&self, other: &Self) -> bool;
}
impl Unify for Ty {
    fn unify(&self, other: &Self) -> bool {
        self.ret.unify(&other.ret) && self.args.unify(&other.args)
    }
}
impl Unify for Ret {
    fn unify(&self, other: &Self) -> bool {
        self.kind == other.kind && self.args.unify(&other.args)
    }
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
impl<T: Unify> Unify for Node<T> {
    fn unify(&self, other: &Self) -> bool {
        let self_binding = self.inner.borrow();
        let other_binding = other.inner.borrow();
        match (&*self_binding, &*other_binding) {
            (Inner::Determined(self_inner), Inner::Determined(other_inner)) => {
                self_inner.unify(other_inner)
            }
            (Inner::Undetermined, _) => {
                drop(self_binding);
                *self.inner.borrow_mut() = Inner::SameAs(other.clone());
                true
            }
            (_, Inner::Undetermined) => {
                drop(other_binding);
                *other.inner.borrow_mut() = Inner::SameAs(self.clone());
                true
            }
            (Inner::SameAs(self_equiv), _) => {
                drop(other_binding);
                self_equiv.unify(other)
            }
            (_, Inner::SameAs(other_equiv)) => {
                drop(self_binding);
                self.unify(other_equiv)
            }
        }
    }
}
