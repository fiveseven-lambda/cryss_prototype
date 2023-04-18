use std::collections::{HashMap, VecDeque};
use std::fmt::{self, Debug, Formatter};

fn main() {
    let converters = vec![
        Converter {
            name: "i2f",
            num_vars: 0,
            from: Expr::App("int", vec![]),
            to: Expr::App("float", vec![]),
        },
        Converter {
            name: "deref",
            num_vars: 1,
            from: Expr::App("ref", vec![Expr::Var(0)]),
            to: Expr::Var(0),
        },
    ];
    let mut queue = VecDeque::new();
    queue.push_back(Ty {
        kind: "ref",
        args: vec![Ty {
            kind: "int",
            args: vec![],
        }],
    });
    let mut prev = HashMap::new();
    while let Some(from) = queue.pop_front() {
        for converter in &converters {
            if let Some(to) = converter.app(&from) {
                if !prev.contains_key(&to) {
                    prev.insert(to.clone(), (converter.name, from.clone()));
                    queue.push_back(to);
                }
            }
        }
    }
    println!("{:?}", prev);
}

type Kind = &'static str;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Ty {
    kind: Kind,
    args: Vec<Ty>,
}
impl Debug for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        if !self.args.is_empty() {
            write!(f, "{:?}", self.args)?;
        }
        Ok(())
    }
}

enum Expr {
    Var(usize),
    App(Kind, Vec<Expr>),
}
impl Expr {
    fn subst(&self, vars: &[Option<Ty>]) -> Ty {
        match *self {
            Expr::Var(id) => vars[id].clone().unwrap(),
            Expr::App(kind, ref args) => Ty {
                kind,
                args: args.iter().map(|expr| expr.subst(vars)).collect(),
            },
        }
    }
    fn identify(&self, goal: &Ty, vars: &mut Vec<Option<Ty>>) -> bool {
        match *self {
            Expr::Var(id) => match &vars[id] {
                Some(ty) => ty == goal,
                None => {
                    vars[id] = Some(goal.clone());
                    true
                }
            },
            Expr::App(kind, ref args) => {
                kind == goal.kind
                    && args.len() == goal.args.len()
                    && args
                        .iter()
                        .zip(&goal.args)
                        .all(|(expr, ty)| expr.identify(ty, vars))
            }
        }
    }
}

struct Converter {
    name: &'static str,
    num_vars: usize,
    from: Expr,
    to: Expr,
}

impl Converter {
    fn app(&self, from: &Ty) -> Option<Ty> {
        let mut vars = vec![None; self.num_vars];
        self.from
            .identify(from, &mut vars)
            .then(|| self.to.subst(&vars))
    }
}
