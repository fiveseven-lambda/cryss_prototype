#![allow(dead_code)]

pub enum Kind {
    Float,
    Sound,
}

pub struct Ty {
    kind: Kind,
    args: Vec<Ty>,
}

impl Ty {
    pub fn float() -> Ty {
        Ty {
            kind: Kind::Float,
            args: vec![],
        }
    }
}

pub struct Func {
    pub args: Vec<Ty>,
    pub ret: Ty,
}
