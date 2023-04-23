use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Bool,
    Int,
    Float,
    Ref,
    Sound,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ty {
    pub kind: Kind,
    pub args: Vec<Ty>,
}
impl Debug for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.kind)?;
        if !self.args.is_empty() {
            write!(f, "{:?}", self.args)?;
        }
        Ok(())
    }
}
impl Ty {
    pub fn pause(&self) -> (usize, &Ty) {
        if self.kind == Kind::Sound {
            let (n, ty) = self.args[0].pause();
            (n + 1, ty)
        } else {
            (0, self)
        }
    }
}

#[macro_export]
macro_rules! ty {
    ($kind:ident) => { ty!($kind,) };
    ($kind:ident, $($args:expr),*) => {
        ty::Ty {
            kind: ty::Kind::$kind,
            args: vec![$($args),*]
        }
    }
}

pub enum Expr {
    Var(usize),
    App { kind: Kind, args: Vec<Expr> },
}
impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Expr::Var(id) => write!(f, "#{id}"),
            Expr::App { kind, ref args } => {
                write!(f, "{kind:?}")?;
                if !args.is_empty() {
                    write!(f, "{args:?}")?;
                }
                Ok(())
            }
        }
    }
}
impl Expr {
    pub fn subst(&self, vars: &im::Vector<Option<Ty>>) -> Ty {
        match *self {
            Expr::Var(id) => vars[id].clone().unwrap(),
            Expr::App { kind, ref args } => Ty {
                kind,
                args: args.iter().map(|expr| expr.subst(vars)).collect(),
            },
        }
    }
    pub fn identify(&self, dest: &Ty, vars: &mut im::Vector<Option<Ty>>) -> bool {
        match *self {
            Expr::Var(id) => match &vars[id] {
                Some(ty) => ty == dest,
                None => {
                    vars[id] = Some(dest.clone());
                    true
                }
            },
            Expr::App { kind, ref args } => {
                kind == dest.kind
                    && args.len() == dest.args.len()
                    && args
                        .iter()
                        .zip(&dest.args)
                        .all(|(expr, ty)| expr.identify(ty, vars))
            }
        }
    }
    pub fn pause(&self) -> (usize, &Expr) {
        if let Expr::App {
            kind: Kind::Sound,
            args,
        } = self
        {
            let (n, expr) = args[0].pause();
            (n + 1, expr)
        } else {
            (0, self)
        }
    }
}

#[macro_export]
macro_rules! expr {
    ($id:literal) => { ty::Expr::Var($id) };
    ($kind:ident) => { expr!($kind,) };
    ($kind:ident, $($args:expr),*) => {
        ty::Expr::App {
            kind: ty::Kind::$kind,
            args: vec![$($args),*]
        }
    }
}

#[derive(Debug)]
pub struct Converter {
    pub num_vars: usize,
    pub from: Expr,
    pub to: Expr,
}
