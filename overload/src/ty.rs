mod _debug_print;
mod unify;

pub struct Field(pub Vec<Tower>);
pub struct Tower {
    pub candidates: Vec<Ty>,
    pub calls: Vec<Call>,
    pub ret_to: Option<(usize, usize, usize)>,
}
pub enum Ty {
    Var(usize),
    Const(String, Vec<Ty>),
    Func(Vec<Ty>, Box<Ty>),
}
pub enum Info {
    Equal(Ty),
    Ret(Ty),
}
pub struct Call {
    pub args: Vec<Arg>,
}
pub struct Arg {
    pub ty: Option<Ty>,
    pub from: Option<usize>,
}
