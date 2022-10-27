use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
enum Value {
    Null,
    Int(i32),
    Bool(bool),
    Addr(Addr),
    Func(Rc<Func>),
}

#[derive(Debug)]
enum Func {
    Assign,
    Deref,
    IAdd,
    Def(Vec<Stmt>, usize),
}

impl Func {
    fn invoke(&self, context: &mut Context, mut args: Vec<Value>) -> Value {
        match self {
            Func::Assign => {
                let mut args = args.into_iter();
                let addr = match args.next().unwrap() {
                    Value::Addr(addr) => addr,
                    _ => panic!(),
                };
                let value = args.next().unwrap();
                context.subst(addr, &value);
                value
            }
            Func::Deref => {
                let mut args = args.into_iter();
                let addr = match args.next().unwrap() {
                    Value::Addr(addr) => addr,
                    _ => panic!(),
                };
                context.deref(addr).unwrap().clone()
            }
            Func::IAdd => {
                let mut args = args.into_iter();
                let left = match args.next().unwrap() {
                    Value::Int(left) => left,
                    _ => panic!(),
                };
                let right = match args.next().unwrap() {
                    Value::Int(right) => right,
                    _ => panic!(),
                };
                Value::Int(left + right)
            }
            Func::Def(stmts, num_local) => {
                args.resize(*num_local, Value::Null);
                let n = context.num_stack;
                context.stack.insert(n, args);
                context.num_stack = n + 1;
                let mut index = 0;
                let ret = loop {
                    let value = match &stmts[index] {
                        Stmt::Jmp(to) => {
                            index = *to;
                            None
                        }
                        Stmt::Expr(expr, to) => {
                            index = *to;
                            Some(expr.eval(context))
                        }
                        Stmt::Br(expr, to_true, to_false) => {
                            let cond = match expr.eval(context) {
                                Value::Bool(cond) => cond,
                                _ => panic!(),
                            };
                            index = if cond { *to_true } else { *to_false };
                            None
                        }
                    };
                    if index == stmts.len() {
                        break value.unwrap();
                    }
                };
                context.stack.remove(&n);
                ret
            }
        }
    }
}

struct Context {
    global: Vec<Value>,
    stack: HashMap<usize, Vec<Value>>,
    num_stack: usize,
}

#[derive(Clone, Debug)]
enum Addr {
    Global(usize),
    Stack(usize, usize),
}

impl Context {
    fn new() -> Context {
        Context {
            global: Vec::new(),
            stack: HashMap::new(),
            num_stack: 0,
        }
    }
    fn deref(&self, addr: Addr) -> Option<&Value> {
        match addr {
            Addr::Global(n) => self.global.get(n),
            Addr::Stack(m, n) => self.stack.get(&m).map(|s| s.get(n)).flatten(),
        }
    }
    fn subst(&mut self, addr: Addr, value: &Value) {
        match addr {
            Addr::Global(n) => {
                self.global[n] = value.clone();
            }
            Addr::Stack(m, n) => {
                self.stack.get_mut(&m).unwrap()[n] = value.clone();
            }
        }
    }
}

#[derive(Debug)]
enum Expr {
    Imm(Value),
    Call(Rc<Func>, Vec<Expr>),
}
impl Expr {
    fn eval(&self, context: &mut Context) -> Value {
        match self {
            Expr::Imm(value) => value.clone(),
            Expr::Call(func, args) => {
                let args = args.into_iter().map(|expr| expr.eval(context)).collect();
                func.invoke(context, args)
            }
        }
    }
}

#[derive(Debug)]
enum Stmt {
    Jmp(usize),
    Expr(Expr, usize),
    Br(Expr, usize, usize),
}

fn main() {
    let mut context = Context::new();
    context.global.push(Value::Int(10));
    let iadd = Rc::new(Func::IAdd);
    let deref = Rc::new(Func::Deref);
    dbg!(Expr::Call(
        iadd,
        vec![
            Expr::Call(deref, vec![Expr::Imm(Value::Addr(Addr::Global(0)))]),
            Expr::Imm(Value::Int(15))
        ]
    )
    .eval(&mut context));
}
