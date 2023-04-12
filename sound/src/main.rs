mod ty;

use enum_as_inner::EnumAsInner;

#[derive(Clone, Debug, EnumAsInner)]
enum Value {
    Float(f64),
    Func(Func),
    Sound(Sound),
}

#[derive(Clone, Debug)]
enum Func {
    Sin,
    Add,
    App,
    Const,
}
impl Func {
    fn call(&self, args: Vec<Value>) -> Value {
        let mut args = args.into_iter();
        let mut arg = || args.next().unwrap();
        match self {
            Func::Sin => {
                let theta = arg().into_float().unwrap();
                Value::Float(theta.sin())
            }
            Func::Add => {
                let first = arg().into_float().unwrap();
                let second = arg().into_float().unwrap();
                Value::Float(first + second)
            }
            Func::App => {
                let func = arg().into_func().unwrap();
                let sound = args.map(|value| value.into_sound().unwrap()).collect();
                Value::Sound(Sound::App(func, sound))
            }
            Func::Const => Value::Sound(Sound::Const(Box::new(arg()))),
        }
    }
}

#[derive(Clone, Debug)]
enum Sound {
    T,
    Const(Box<Value>),
    App(Func, Vec<Sound>),
}
impl Sound {
    fn sample(&self, rate: f64, n: usize) -> Vec<Value> {
        match self {
            Sound::Const(value) => vec![*value.clone(); n],
            Sound::T => (0..n).map(|t| Value::Float((t as f64) / rate)).collect(),
            Sound::App(func, args) => {
                let mut args: Vec<_> = args
                    .iter()
                    .map(|sound| sound.sample(rate, n).into_iter())
                    .collect();
                (0..n)
                    .map(|_| func.call(args.iter_mut().map(|iter| iter.next().unwrap()).collect()))
                    .collect()
            }
        }
    }
}

enum Expr {
    Imm(Value),
    Local(usize),
    Call(Box<Expr>, Vec<Expr>),
}
impl Expr {
    fn eval(&self, locals: &[Value]) -> Value {
        match *self {
            Expr::Imm(ref value) => value.clone(),
            Expr::Local(pos) => locals[pos].clone(),
            Expr::Call(ref func, ref args) => {
                let func = func.eval(locals).into_func().unwrap();
                let args = args.into_iter().map(|arg| arg.eval(locals)).collect();
                func.call(args)
            }
        }
    }
}

fn main() {
    println!(
        "{:?}",
        Expr::Call(
            Box::new(Expr::Imm(Value::Func(Func::Sin))),
            vec![Expr::Local(0)],
        )
        .eval(&[Value::Float(1.0)])
    );
    println!(
        "{:?}",
        Expr::Call(
            Box::new(Expr::Imm(Value::Func(Func::App))),
            vec![Expr::Imm(Value::Func(Func::Sin)), Expr::Local(0)],
        )
        .eval(&[Value::Sound(Sound::T)])
    );
    println!(
        "{:?}",
        Expr::Call(
            Box::new(Expr::Imm(Value::Func(Func::App))),
            vec![
                Expr::Imm(Value::Func(Func::Add)),
                Expr::Local(0),
                Expr::Call(
                    Box::new(Expr::Imm(Value::Func(Func::Const))),
                    vec![Expr::Local(1)]
                )
            ],
        )
        .eval(&[Value::Sound(Sound::T), Value::Float(1.0)])
        .into_sound()
        .unwrap()
        .sample(2., 10)
    );
    println!(
        "{:?}",
        Sound::App(
            Func::App,
            vec![
                Sound::Const(Box::new(Value::Func(Func::Add))),
                Sound::Const(Box::new(Value::Sound(Sound::T))),
                Sound::App(Func::Const, vec![Sound::T]),
            ]
        )
        .sample(1., 5)
        .into_iter()
        .map(|sound| sound.into_sound().unwrap().sample(1., 5))
        .collect::<Vec<_>>()
    );
}
