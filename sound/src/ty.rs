#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Int,
    Float,
    Bool,
    Sound,
    Func,
}

#[derive(Clone)]
pub struct Ty {
    kind: Kind,
    args: Vec<Ty>,
}
impl Ty {
    pub fn int() -> Ty {
        Ty {
            kind: Kind::Int,
            args: vec![],
        }
    }
    pub fn float() -> Ty {
        Ty {
            kind: Kind::Float,
            args: vec![],
        }
    }
    pub fn bool() -> Ty {
        Ty {
            kind: Kind::Bool,
            args: vec![],
        }
    }
}

pub struct Func {
    pub args: Vec<Arg>,
    pub ret: Expr,
}

pub enum Expr {
    Var(usize),
    Range(usize, usize),
    App(Kind, Vec<Arg>),
}

pub enum Arg {
    Expr(Expr),
    Expand(Expr),
}

impl Func {
    pub fn eval(&self, tys: &[Ty]) -> Ty {
        let args = std::iter::once(self.ret.eval(tys))
            .chain(expand_args(&self.args, tys).into_iter())
            .collect::<Result<_, _>>()
            .unwrap();
        Ty {
            kind: Kind::Func,
            args,
        }
    }
}
fn expand_args(args: &[Arg], tys: &[Ty]) -> Vec<Result<Ty, Vec<Ty>>> {
    let mut ret = Vec::new();
    for arg in args {
        match arg {
            Arg::Expr(expr) => ret.push(expr.eval(tys)),
            Arg::Expand(expr) => ret.extend(expr.eval(tys).unwrap_err().into_iter().map(Ok)),
        }
    }
    ret
}
impl Expr {
    fn eval(&self, tys: &[Ty]) -> Result<Ty, Vec<Ty>> {
        match *self {
            Expr::Var(i) => Ok(tys[i].clone()),
            Expr::Range(from, to) => {
                let to = tys.len() - to;
                Err(tys[from..to].to_vec())
            }
            Expr::App(kind, ref args) => {
                let expanded = expand_args(args, tys);
                let mut lens = expanded
                    .iter()
                    .filter_map(|e| e.as_ref().map_err(|tys| tys.len()).err());
                match lens.next() {
                    Some(len) => {
                        assert!(lens.all(|n| n == len));
                        let mut iters: Vec<_> = expanded
                            .into_iter()
                            .map(|e| match e {
                                Ok(ty) => vec![ty; len].into_iter(),
                                Err(tys) => tys.into_iter(),
                            })
                            .collect();
                        Err((0..len)
                            .map(|_| Ty {
                                kind,
                                args: iters.iter_mut().map(|iter| iter.next().unwrap()).collect(),
                            })
                            .collect())
                    }
                    None => Ok(Ty {
                        kind,
                        args: expanded.into_iter().collect::<Result<_, _>>().unwrap(),
                    }),
                }
            }
        }
    }
}

use std::fmt;
impl fmt::Debug for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ty {
            kind: Kind::Func,
            args,
        } = self
        {
            write!(
                f,
                "fn({}) -> {:?}",
                args[1..]
                    .iter()
                    .map(|arg| format!("{arg:?}"))
                    .collect::<Vec<String>>()
                    .join(", "),
                args[0]
            )
        } else {
            write!(f, "{:?}", self.kind)?;
            if !self.args.is_empty() {
                write!(
                    f,
                    "[{}]",
                    self.args
                        .iter()
                        .map(|ty| format!("{ty:?}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
            }
            Ok(())
        }
    }
}
impl fmt::Debug for Func {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fn({}) -> {:?}",
            self.args
                .iter()
                .map(|ty| format!("{ty:?}"))
                .collect::<Vec<String>>()
                .join(", "),
            self.ret
        )
    }
}
impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Var(id) => write!(f, "#{id}",),
            Expr::Range(from, to) => write!(f, "#{from}:#-{to}",),
            Expr::App(Kind::Func, args) => {
                write!(
                    f,
                    "fn({}) -> {:?}",
                    args[1..]
                        .iter()
                        .map(|arg| format!("{arg:?}"))
                        .collect::<Vec<String>>()
                        .join(", "),
                    args[0]
                )
            }
            Expr::App(kind, args) => {
                write!(f, "{kind:?}")?;
                if !args.is_empty() {
                    write!(
                        f,
                        "[{}]",
                        args.iter()
                            .map(|arg| format!("{arg:?}"))
                            .collect::<Vec<String>>()
                            .join(", ")
                    )?
                }
                Ok(())
            }
        }
    }
}
impl fmt::Debug for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arg::Expr(expr) => write!(f, "{expr:?}"),
            Arg::Expand(expr) => {
                write!(f, "{expr:?}...")
            }
        }
    }
}
