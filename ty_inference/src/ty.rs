use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

mod debug_print;

pub struct Ty {
    pub kind: String,
    pub args: Vec<Var>,
}

#[derive(Clone)]
pub struct Var {
    inner: Rc<RefCell<Node>>,
}

enum Node {
    Determined(Ty),
    Undetermined { size: Cell<u32> },
    SameAs(Var),
}

impl Ty {
    fn contains(&self, target: &Var) -> bool {
        self.args.iter().any(|arg| arg.contains(target))
    }
}

impl Var {
    pub fn new() -> Var {
        Var {
            inner: Rc::new(RefCell::new(Node::Undetermined { size: Cell::new(1) })),
        }
    }
    pub fn ty(kind: String, args: Vec<Var>) -> Var {
        Var {
            inner: Rc::new(RefCell::new(Node::Determined(Ty { kind, args }))),
        }
    }
    fn contains(&self, target: &Var) -> bool {
        match *self.inner.borrow() {
            Node::Determined(ref ty) => ty.contains(target),
            Node::Undetermined { .. } => Rc::ptr_eq(&self.inner, &target.inner),
            Node::SameAs(ref parent) => parent.contains(target),
        }
    }
    pub fn unify(&self, other: &Var, history: &mut History) -> bool {
        let self_binding = self.inner.borrow();
        let other_binding = other.inner.borrow();
        match (&*self_binding, &*other_binding) {
            (Node::SameAs(self_parent), _) => {
                drop(other_binding);
                self_parent.unify(other, history)
            }
            (Node::Determined(self_ty), Node::Determined(other_ty)) => {
                self_ty.kind == other_ty.kind
                    && self_ty.args.len() == other_ty.args.len()
                    && self_ty
                        .args
                        .iter()
                        .zip(&other_ty.args)
                        .all(|(self_arg, other_arg)| self_arg.unify(other_arg, history))
            }
            (Node::Undetermined { size }, Node::Determined(ty)) => {
                if ty.contains(self) {
                    return false;
                }
                history.inner.push(Operation {
                    child: self.clone(),
                    old_child_size: size.get(),
                    old_parent_size: None,
                });
                drop(self_binding);
                *self.inner.borrow_mut() = Node::SameAs(other.clone());
                true
            }
            (Node::Undetermined { size: self_size }, Node::Undetermined { size: other_size })
                if self_size <= other_size =>
            {
                if Rc::ptr_eq(&self.inner, &other.inner) {
                    return false;
                }
                let new_size = self_size.get() + other_size.get();
                history.inner.push(Operation {
                    child: self.clone(),
                    old_child_size: other_size.get(),
                    old_parent_size: Some(self_size.get()),
                });
                other_size.set(new_size);
                drop(self_binding);
                *self.inner.borrow_mut() = Node::SameAs(other.clone());
                true
            }
            _ => {
                drop(self_binding);
                drop(other_binding);
                other.unify(self, history)
            }
        }
    }
}

pub struct History {
    inner: Vec<Operation>,
}

struct Operation {
    child: Var,
    old_child_size: u32,
    old_parent_size: Option<u32>,
}

impl History {
    pub fn new() -> History {
        History { inner: Vec::new() }
    }
    pub fn rollback(&mut self) {
        while let Some(Operation {
            child: Var { inner: child },
            old_child_size,
            old_parent_size,
        }) = self.inner.pop()
        {
            match *child.borrow() {
                Node::SameAs(Var { inner: ref parent }) => match old_parent_size {
                    Some(old_parent_size) => match *parent.borrow() {
                        Node::Undetermined {
                            size: ref parent_size,
                        } => parent_size.set(old_parent_size),
                        _ => panic!("rollback error"),
                    },
                    None => assert!(
                        matches!(*parent.borrow(), Node::Determined(_)),
                        "rollback error"
                    ),
                },
                _ => panic!("rollback error"),
            }
            *child.borrow_mut() = Node::Undetermined {
                size: Cell::new(old_child_size),
            };
        }
    }
}
