use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

/// 型
struct Ty {
    /// 型名
    kind: String,
    /// 型引数
    args: Vec<Rc<RefCell<Var>>>,
}

/// 型変数
enum Var {
    /// 中身が判明している．
    Determined(Ty),
    /// 判明していない．
    Undetermined {
        /// 「これと同じ」と言っている（Var::SameAs を通して直接的または間接的に指している）型変数の数（自身も含む）．
        size: Cell<u32>,
    },
    /// 他の型変数と同じであることが分かっている．
    SameAs(Rc<RefCell<Var>>),
}

/** `ty` の中に型変数 `var` が含まれているか判定する．
 * 計算量：O(`ty` の子孫の数)
 *
 * `ty` が型変数じゃん → ごめんなさい，多分 `ty: &Ty` とかに書き換えることができます (todo)
 */
fn contains(ty: &Rc<RefCell<Var>>, var: &Rc<RefCell<Var>>) -> bool {
    match *ty.borrow() {
        Var::Determined(ref ty) => ty.args.iter().any(|arg| contains(arg, var)),
        Var::Undetermined { .. } => Rc::ptr_eq(ty, var),
        Var::SameAs(ref parent) => contains(parent, var),
    }
}

/** 単一化を行う．
 * 計算量：O(log(型変数の個数))
 *
 * `left` と `right` が同じ型にできれば `true`，できなければ `false` を返す．
 */
fn unify(left: &Rc<RefCell<Var>>, right: &Rc<RefCell<Var>>, history: &mut impl History) -> bool {
    let left_binding = left.borrow();
    let right_binding = right.borrow();
    match (&*left_binding, &*right_binding) {
        (Var::SameAs(left_parent), _) => {
            drop(right_binding);
            unify(left_parent, right, history)
        }
        (_, Var::SameAs(right_parent)) => {
            drop(left_binding);
            unify(left, right_parent, history)
        }
        (Var::Determined(left_ty), Var::Determined(right_ty)) => {
            left_ty.kind == right_ty.kind
                && left_ty.args.len() == right_ty.args.len()
                && left_ty
                    .args
                    .iter()
                    .zip(&right_ty.args)
                    .all(|(left_arg, right_arg)| unify(left_arg, right_arg, history))
        }
        (Var::Undetermined { size }, Var::Determined(_)) => {
            if contains(right, left) {
                // left と Ty[(left を含む)] を unify しようとした．
                return false;
            }
            // left: Undetermined から SameAs
            // right: Determined のまま
            history.add(Operation {
                child: left.clone(),
                old_child_size: size.get(),
                old_parent_size: None,
            });
            drop(left_binding);
            *left.borrow_mut() = Var::SameAs(right.clone());
            true
        }
        (Var::Determined(_), Var::Undetermined { size }) => {
            if contains(left, right) {
                // right と Ty[(right を含む)] を unify しようとした．
                return false;
            }
            // right: Undetermined から SameAs
            // left: Determined のまま
            history.add(Operation {
                child: right.clone(),
                old_child_size: size.get(),
                old_parent_size: None,
            });
            drop(right_binding);
            *right.borrow_mut() = Var::SameAs(left.clone());
            true
        }
        (Var::Undetermined { size: left_size }, Var::Undetermined { size: right_size }) => {
            if Rc::ptr_eq(left, right) {
                return false;
            }
            let new_size = left_size.get() + right_size.get();
            if left_size > right_size {
                // right: Undetermined から SameAs
                // left: Undetermined のまま size 変更
                history.add(Operation {
                    child: right.clone(),
                    old_child_size: right_size.get(),
                    old_parent_size: Some(left_size.get()),
                });
                drop(right_binding);
                left_size.set(new_size);
                *right.borrow_mut() = Var::SameAs(left.clone());
            } else {
                // left: Undetermined から SameAs
                // right: Undetermined のまま size 変更
                history.add(Operation {
                    child: left.clone(),
                    old_child_size: left_size.get(),
                    old_parent_size: Some(right_size.get()),
                });
                drop(left_binding);
                right_size.set(new_size);
                *left.borrow_mut() = Var::SameAs(right.clone());
            }
            true
        }
    }
}

/// デバッグ出力用
macro_rules! _debug {
    ($($var:expr),*) => {
        $( eprintln!("{}: {}", stringify!($var), _to_string(&$var));)*
    };
}
/// デバッグ出力用
fn _to_string(var: &Rc<RefCell<Var>>) -> String {
    let ret = match *var.borrow() {
        Var::Determined(Ty { ref kind, ref args }) => {
            let mut ret = kind.to_owned();
            if !args.is_empty() {
                ret += &format!(
                    "[{}]",
                    args.iter().map(_to_string).collect::<Vec<_>>().join(", ")
                );
            }
            ret
        }
        Var::Undetermined { ref size } => format!("? (size {})", size.get()),
        Var::SameAs(ref parent) => _to_string(parent),
    };
    format!("{:p} {ret}", var.as_ptr())
}

/**
 * 型変数に対する操作．以下の 2 通り：
 * - child, parent がともに Undetermined のとき， child ← SameAs(parent) する．このとき parent の size を増やす．
 * - child: Undetermined，parent: Determined のとき， child ← SameAs(parent) する．
 *
 * この struct Operation は，操作を undo するために十分な情報を持つ．
 * 1 回の undo は O(1) で行える．
 */
struct Operation {
    child: Rc<RefCell<Var>>,
    old_child_size: u32,
    old_parent_size: Option<u32>,
}
/**
 * 行った操作を記録し，復元する．
*/
trait History {
    /// 記録
    fn add(&mut self, _: Operation);
    /// 記録された全ての操作を逆順に復元する．計算量 O(操作数)
    fn rollback(&mut self);
}
impl History for () {
    fn add(&mut self, _: Operation) {}
    fn rollback(&mut self) {}
}
impl History for Vec<Operation> {
    fn add(&mut self, op: Operation) {
        self.push(op);
    }
    fn rollback(&mut self) {
        while let Some(Operation {
            child,
            old_child_size,
            old_parent_size,
        }) = self.pop()
        {
            match *child.borrow() {
                Var::SameAs(ref parent) => match old_parent_size {
                    Some(old_parent_size) => match *parent.borrow() {
                        Var::Undetermined {
                            size: ref parent_size,
                        } => parent_size.set(old_parent_size),
                        _ => panic!("rollback error"),
                    },
                    None => assert!(
                        matches!(*parent.borrow(), Var::Determined(_)),
                        "rollback error"
                    ),
                },
                _ => panic!("rollback error"),
            }
            *child.borrow_mut() = Var::Undetermined {
                size: Cell::new(old_child_size),
            };
        }
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Var::Undetermined { size: Cell::new(1) }));
    let b = Rc::new(RefCell::new(Var::Undetermined { size: Cell::new(1) }));
    let c = Rc::new(RefCell::new(Var::Undetermined { size: Cell::new(1) }));
    let d = Rc::new(RefCell::new(Var::Undetermined { size: Cell::new(1) }));
    let e = Rc::new(RefCell::new(Var::Undetermined { size: Cell::new(1) }));
    let f = Rc::new(RefCell::new(Var::Undetermined { size: Cell::new(1) }));
    let hoge_e = Rc::new(RefCell::new(Var::Determined(Ty {
        kind: String::from("hoge"),
        args: vec![e.clone()],
    })));
    let hoge_f = Rc::new(RefCell::new(Var::Determined(Ty {
        kind: String::from("hoge"),
        args: vec![f.clone()],
    })));

    dbg!(unify(&a, &b, &mut ()));
    dbg!(unify(&c, &d, &mut ()));
    let mut history = Vec::new();
    dbg!(unify(&a, &hoge_e, &mut history));
    dbg!(unify(&c, &hoge_f, &mut history));
    _debug!(a, b, c, d);
    dbg!(unify(&b, &c, &mut history));
    _debug!(a, b, c, d);
    history.rollback();
    _debug!(a, b, c, d);
}
