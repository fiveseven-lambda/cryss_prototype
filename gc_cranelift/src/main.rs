use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    mem::ManuallyDrop,
    rc::Rc,
};

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;

#[derive(Debug)]
enum Tree {
    Leaf(i32),
    Node(Vec<Rc<Tree>>),
}
extern "C" fn new_leaf(value: i32) -> *const Tree {
    Rc::into_raw(Tree::Leaf(value).into())
}
unsafe extern "C" fn add_child(children: &mut Vec<Rc<Tree>>, tree: *const Tree) {
    children.push(Rc::from_raw(tree));
}
extern "C" fn new_node(children: &mut Vec<Rc<Tree>>) -> *const Tree {
    let children = std::mem::take(children);
    Rc::into_raw(Tree::Node(children).into())
}
unsafe extern "C" fn clone_tree(ptr: *const Tree) {
    Rc::increment_strong_count(ptr);
}
unsafe extern "C" fn delete_tree(ptr: *const Tree) {
    if !ptr.is_null() {
        Rc::from_raw(ptr);
    }
}
unsafe extern "C" fn print_tree(ptr: *const Tree) {
    let tree = ManuallyDrop::new(Rc::from_raw(ptr));
    println!("{}", tree2string(&tree));
}
fn tree2string(tree: &Rc<Tree>) -> String {
    let count = Rc::strong_count(tree);
    match **tree {
        Tree::Leaf(value) => {
            format!("({})\"{}\"", count, value)
        }
        Tree::Node(ref children) => {
            format!(
                "({})[{}]",
                count,
                children
                    .iter()
                    .map(tree2string)
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
}

enum Expr {
    Var(&'static str),
    Leaf(i32),
    Node(Vec<Expr>),
}
impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Var(name) => write!(f, "{name}"),
            Expr::Leaf(value) => write!(f, "{value}"),
            Expr::Node(children) => {
                write!(
                    f,
                    "[{}]",
                    children
                        .iter()
                        .map(|child| format!("{child:?}"))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
        }
    }
}
impl Expr {
    fn compile(
        &self,
        builder: &mut FunctionBuilder,
        variables: &mut HashMap<&str, Variable>,
        (new_leaf_sig, new_leaf_ptr): (codegen::ir::SigRef, Value),
        (clone_tree_sig, clone_tree_ptr): (codegen::ir::SigRef, Value),
        children: Value,
        (add_child_sig, add_child_ptr): (codegen::ir::SigRef, Value),
        (new_node_sig, new_node_ptr): (codegen::ir::SigRef, Value),
    ) -> Value {
        match *self {
            Expr::Leaf(value) => {
                let value = builder.ins().iconst(types::I32, Into::<i64>::into(value));
                let inst = builder
                    .ins()
                    .call_indirect(new_leaf_sig, new_leaf_ptr, &[value]);
                builder.inst_results(inst)[0]
            }
            Expr::Var(name) => {
                let variable = *variables.get(name).expect("undefined variable");
                let value = builder.use_var(variable);
                builder
                    .ins()
                    .call_indirect(clone_tree_sig, clone_tree_ptr, &[value]);
                value
            }
            Expr::Node(ref children_expr) => {
                for child_expr in children_expr {
                    let child = child_expr.compile(
                        builder,
                        variables,
                        (new_leaf_sig, new_leaf_ptr),
                        (clone_tree_sig, clone_tree_ptr),
                        children,
                        (add_child_sig, add_child_ptr),
                        (new_node_sig, new_node_ptr),
                    );
                    builder
                        .ins()
                        .call_indirect(add_child_sig, add_child_ptr, &[children, child]);
                }
                let inst = builder
                    .ins()
                    .call_indirect(new_node_sig, new_node_ptr, &[children]);
                builder.inst_results(inst)[0]
            }
        }
    }
}

fn main() {
    let program = vec![
        (Some("a"), Expr::Leaf(10)),
        (Some("b"), Expr::Leaf(20)),
        (
            Some("a"),
            Expr::Node(vec![
                Expr::Node(vec![Expr::Var("a"), Expr::Var("b")]),
                Expr::Var("b"),
            ]),
        ),
        (None, Expr::Var("a")),
        (None, Expr::Var("b")),
    ];
    for (var, expr) in &program {
        match var {
            Some(name) => println!("{name} <- {expr:?}"),
            None => println!("print {expr:?}"),
        }
    }

    let mut trees = Vec::new();

    let jit_builder = JITBuilder::new(cranelift_module::default_libcall_names()).unwrap();
    let mut module = JITModule::new(jit_builder);
    let ptr_ty = module.target_config().pointer_type();

    let mut ctx = module.make_context();
    ctx.func.signature = module.make_signature();
    let mut fn_builder_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fn_builder_ctx);

    let block = builder.create_block();
    builder.switch_to_block(block);

    let new_leaf_ptr = builder
        .ins()
        .iconst(ptr_ty, TryInto::<i64>::try_into(new_leaf as usize).unwrap());
    let new_leaf_sig = builder.import_signature({
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(types::I32));
        sig.returns.push(AbiParam::new(ptr_ty));
        sig
    });
    let add_child_ptr = builder.ins().iconst(
        ptr_ty,
        TryInto::<i64>::try_into(add_child as usize).unwrap(),
    );
    let add_child_sig = builder.import_signature({
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(ptr_ty));
        sig
    });
    let new_node_ptr = builder
        .ins()
        .iconst(ptr_ty, TryInto::<i64>::try_into(new_node as usize).unwrap());
    let new_node_sig = builder.import_signature({
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(ptr_ty));
        sig.returns.push(AbiParam::new(ptr_ty));
        sig
    });
    let clone_tree_ptr = builder.ins().iconst(
        ptr_ty,
        TryInto::<i64>::try_into(clone_tree as usize).unwrap(),
    );
    let clone_tree_sig = builder.import_signature({
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(ptr_ty));
        sig
    });
    let delete_tree_ptr = builder.ins().iconst(
        ptr_ty,
        TryInto::<i64>::try_into(delete_tree as usize).unwrap(),
    );
    let delete_tree_sig = clone_tree_sig;
    let print_tree_ptr = builder.ins().iconst(
        ptr_ty,
        TryInto::<i64>::try_into(print_tree as usize).unwrap(),
    );
    let print_tree_sig = delete_tree_sig;

    let children = builder
        .ins()
        .iconst(ptr_ty, &mut trees as *mut Vec<Rc<Tree>> as i64);

    let mut variables = HashMap::new();
    for (var, expr) in program {
        let value = expr.compile(
            &mut builder,
            &mut variables,
            (new_leaf_sig, new_leaf_ptr),
            (clone_tree_sig, clone_tree_ptr),
            children,
            (add_child_sig, add_child_ptr),
            (new_node_sig, new_node_ptr),
        );
        match var {
            Some(name) => {
                let index = variables.len();
                let variable = *variables.entry(name).or_insert_with(|| {
                    let variable = Variable::new(index);
                    builder.declare_var(variable, ptr_ty);
                    let null = builder
                        .ins()
                        .iconst(ptr_ty, std::ptr::null::<Tree>() as i64);
                    builder.def_var(variable, null);
                    variable
                });
                let old_value = builder.use_var(variable);
                builder
                    .ins()
                    .call_indirect(delete_tree_sig, delete_tree_ptr, &[old_value]);
                builder.def_var(variable, value);
            }
            None => {
                builder
                    .ins()
                    .call_indirect(print_tree_sig, print_tree_ptr, &[value]);
                builder
                    .ins()
                    .call_indirect(delete_tree_sig, delete_tree_ptr, &[value]);
            }
        }
    }
    for (_, variable) in variables {
        let value = builder.use_var(variable);
        builder
            .ins()
            .call_indirect(delete_tree_sig, delete_tree_ptr, &[value]);
    }

    builder.ins().return_(&[]);
    builder.seal_block(block);

    builder.finalize();
    println!("{}", ctx.func.display());

    let func = module
        .declare_anonymous_function(&ctx.func.signature)
        .unwrap();
    module.define_function(func, &mut ctx).unwrap();
    module.finalize_definitions().unwrap();
    let code = module.get_finalized_function(func);
    unsafe { std::mem::transmute::<_, unsafe fn() -> ()>(code)() }
}
