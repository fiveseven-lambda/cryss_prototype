use std::rc::Rc;

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;

#[derive(Debug)]
enum Tree {
    Leaf(i32),
    Node(Rc<Tree>, Rc<Tree>),
}
extern "C" fn new_leaf(value: i32) -> *const Tree {
    let ptr = Rc::into_raw(Tree::Leaf(value).into());
    ptr
}
unsafe extern "C" fn new_node(left: *const Tree, right: *const Tree) -> *const Tree {
    Rc::increment_strong_count(left);
    Rc::increment_strong_count(right);
    let ptr = Rc::into_raw(Tree::Node(Rc::from_raw(left), Rc::from_raw(right)).into());
    ptr
}
unsafe extern "C" fn delete_tree(ptr: *const Tree) {
    Rc::from_raw(ptr);
}
unsafe extern "C" fn print_tree(ptr: *const Tree) {
    let tree = Rc::from_raw(ptr);
    println!("{}", tree2string(&tree));
    std::mem::forget(tree);
}
fn tree2string(tree: &Rc<Tree>) -> String {
    let count = Rc::strong_count(&tree);
    match **tree {
        Tree::Leaf(value) => {
            format!("({})\"{}\"", count, value)
        }
        Tree::Node(ref left, ref right) => {
            format!("({})[{},{}]", count, tree2string(left), tree2string(right))
        }
    }
}

fn main() {
    let jit_builder = JITBuilder::new(cranelift_module::default_libcall_names()).unwrap();
    let mut module = JITModule::new(jit_builder);
    let ptr_ty = module.target_config().pointer_type();

    let mut ctx = module.make_context();
    ctx.func.signature = module.make_signature();
    let mut fn_builder_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fn_builder_ctx);

    let block = builder.create_block();
    builder.append_block_params_for_function_params(block);
    builder.switch_to_block(block);

    let new_leaf_ptr = builder.ins().iconst(ptr_ty, new_leaf as i64);
    let new_leaf_sig = builder.import_signature({
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(types::I32));
        sig.returns.push(AbiParam::new(ptr_ty));
        sig
    });
    let new_node_ptr = builder.ins().iconst(ptr_ty, new_node as i64);
    let new_node_sig = builder.import_signature({
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(ptr_ty));
        sig.params.push(AbiParam::new(ptr_ty));
        sig.returns.push(AbiParam::new(ptr_ty));
        sig
    });
    let delete_tree_ptr = builder.ins().iconst(ptr_ty, delete_tree as i64);
    let delete_tree_sig = builder.import_signature({
        let mut sig = module.make_signature();
        sig.params.push(AbiParam::new(ptr_ty));
        sig
    });
    let print_tree_ptr = builder.ins().iconst(ptr_ty, print_tree as i64);
    let print_tree_sig = delete_tree_sig;

    let var0 = Variable::new(0);
    builder.declare_var(var0, ptr_ty);
    let one = builder.ins().iconst(types::I32, 1);
    let leaf_one_inst = builder
        .ins()
        .call_indirect(new_leaf_sig, new_leaf_ptr, &[one]);
    let leaf_one = builder.inst_results(leaf_one_inst)[0];
    builder.def_var(var0, leaf_one);

    let var1 = Variable::new(1);
    builder.declare_var(var1, ptr_ty);
    let node_one_one_inst =
        builder
            .ins()
            .call_indirect(new_node_sig, new_node_ptr, &[leaf_one, leaf_one]);
    let node_one_one = builder.inst_results(node_one_one_inst)[0];
    builder
        .ins()
        .call_indirect(print_tree_sig, print_tree_ptr, &[node_one_one]);
    builder
        .ins()
        .call_indirect(delete_tree_sig, delete_tree_ptr, &[leaf_one]);
    builder
        .ins()
        .call_indirect(print_tree_sig, print_tree_ptr, &[node_one_one]);
    builder
        .ins()
        .call_indirect(delete_tree_sig, delete_tree_ptr, &[node_one_one]);

    builder.ins().return_(&[]);
    builder.seal_block(block);

    builder.finalize();
    println!("{}", ctx.func.display());

    let func = module
        .declare_function("", cranelift_module::Linkage::Local, &ctx.func.signature)
        .unwrap();
    module.define_function(func, &mut ctx).unwrap();
    module.finalize_definitions().unwrap();
    let code = module.get_finalized_function(func);
    let ptr = unsafe { std::mem::transmute::<_, unsafe fn() -> ()>(code) };
    unsafe { ptr() };
}
