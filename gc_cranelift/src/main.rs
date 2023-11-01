use cranelift::prelude::*;
use cranelift_module::Module;

use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

enum Tree {
    Leaf(i32),
    Node(*const Tree, *const Tree),
}
unsafe extern "C" fn print_unsafe(ptr: *const Tree) {
    println!("{}", to_string_unsafe(ptr))
}
unsafe fn to_string_unsafe(ptr: *const Tree) -> String {
    match *ptr {
        Tree::Leaf(value) => value.to_string(),
        Tree::Node(left, right) => {
            format!("({},{})", to_string_unsafe(left), to_string_unsafe(right))
        }
    }
}
extern "C" fn leaf(value: i32) -> *const Tree {
    Box::into_raw(Tree::Leaf(value).into())
}
extern "C" fn node(left: *const Tree, right: *const Tree) -> *const Tree {
    Box::into_raw(Tree::Node(left, right).into())
}

enum Expr {
    Var(String),
    Leaf(i32),
    Node(Box<Expr>, Box<Expr>),
}
impl Expr {
    fn compile(
        self,
        module: &impl Module,
        builder: &mut FunctionBuilder,
        vars: &HashMap<&str, Value>,
    ) -> Value {
        match self {
            Expr::Leaf(value) => {
                let value = builder.ins().iconst(types::I32, value as i64);
                let sig = builder.import_signature({
                    let mut sig = module.make_signature();
                    sig.params.push(AbiParam::new(types::I32));
                    sig.returns.push(AbiParam::new(types::I64));
                    sig
                });
                let leaf_ptr = builder.ins().iconst(types::I64, leaf as i64);
                let inst = builder.ins().call_indirect(sig, leaf_ptr, &[value]);
                builder.inst_results(inst)[0]
            }
            Expr::Node(left, right) => {
                let left = left.compile(module, builder, vars);
                let right = right.compile(module, builder, vars);
                let sig = builder.import_signature({
                    let mut sig = module.make_signature();
                    sig.params.push(AbiParam::new(types::I64));
                    sig.params.push(AbiParam::new(types::I64));
                    sig.returns.push(AbiParam::new(types::I64));
                    sig
                });
                let node_ptr = builder.ins().iconst(types::I64, node as i64);
                let inst = builder.ins().call_indirect(sig, node_ptr, &[left, right]);
                builder.inst_results(inst)[0]
            }
            Expr::Var(name) => vars[&name[..]],
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Var(name) => write!(f, "{name}"),
            Expr::Leaf(value) => write!(f, "{value}"),
            Expr::Node(left, right) => {
                write!(f, "({left:?}, {right:?})")
            }
        }
    }
}

macro_rules! expr {
    ($name:literal) => {
        Expr::Var(String::from($name))
    };
    (($value:literal)) => {
        Expr::Leaf($value)
    };
    (($left:tt, $right:tt)) => {
        Expr::Node(Box::new(expr!($left)), Box::new(expr!($right)))
    };
}

fn main() {
    let program = vec![
        (Some("a"), expr!((10))),
        (Some("b"), expr!(("a", "a"))),
        (Some("c"), expr!(("a", "b"))),
        (None, expr!("c")),
    ];

    for (var, expr) in &program {
        println!("{var:?} <- {expr:?}");
    }

    let jit_builder =
        cranelift_jit::JITBuilder::new(cranelift_module::default_libcall_names()).unwrap();
    let mut module = cranelift_jit::JITModule::new(jit_builder);
    let mut ctx = module.make_context();
    ctx.func.signature = module.make_signature();
    let mut fn_builder_ctx = FunctionBuilderContext::new();

    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fn_builder_ctx);
    let block = builder.create_block();
    builder.switch_to_block(block);

    let mut vars = HashMap::new();
    for (name, expr) in program {
        let value = expr.compile(&module, &mut builder, &vars);
        match name {
            Some(name) => {
                vars.insert(name, value);
            }
            None => {
                let sig = builder.import_signature({
                    let mut sig = module.make_signature();
                    sig.params.push(AbiParam::new(types::I64));
                    sig
                });
                let ptr = builder.ins().iconst(types::I64, print_unsafe as i64);
                builder.ins().call_indirect(sig, ptr, &[value]);
            }
        };
    }

    builder.ins().return_(&[]);
    builder.seal_block(block);
    builder.finalize();
    println!("{}", ctx.func.display());

    let func = module
        .declare_function(
            "func",
            cranelift_module::Linkage::Local,
            &ctx.func.signature,
        )
        .unwrap();
    module.define_function(func, &mut ctx).unwrap();
    module.finalize_definitions().unwrap();
    let code = module.get_finalized_function(func);
    let ptr = unsafe { std::mem::transmute::<_, unsafe fn() -> *const Tree>(code) };
    unsafe { ptr() };
}
