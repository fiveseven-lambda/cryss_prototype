use cranelift::{codegen::ir::immediates::Offset32, prelude::*};
use cranelift_module::{DataDescription, Linkage, Module};

use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
enum IRValue {
    Float(f64),
}
impl IRValue {
    fn translate(&self, builder: &mut FunctionBuilder) -> Value {
        match *self {
            IRValue::Float(value) => builder.ins().f64const(value),
        }
    }
}
#[derive(Debug)]
enum Sound {
    T,
    Const(IRValue),
    App(Func, Vec<Rc<Sound>>),
    Trim(f64, f64, Rc<Sound>),
}
impl Sound {
    fn translate(&self, builder: &mut FunctionBuilder, t: Value) -> Value {
        match self {
            Sound::T => t,
            Sound::Const(value) => value.translate(builder),
            Sound::App(func, args) => {
                let args = args
                    .iter()
                    .map(|sound| sound.translate(builder, t))
                    .collect();
                func.translate(builder, args)
            }
            Sound::Trim(_from, _to, _sound) => {
                todo!();
            }
        }
    }
}

#[derive(Debug)]
enum Func {
    Builtin(BuiltinFunc),
}
impl Func {
    fn translate(&self, builder: &mut FunctionBuilder, args: Vec<Value>) -> Value {
        match self {
            Func::Builtin(func) => func.translate(builder, args),
        }
    }
}
#[derive(Debug)]
enum BuiltinFunc {
    AddFloat,
    MulFloat,
}
impl BuiltinFunc {
    fn translate(&self, builder: &mut FunctionBuilder, args: Vec<Value>) -> Value {
        match self {
            BuiltinFunc::AddFloat => builder.ins().fadd(args[0], args[1]),
            BuiltinFunc::MulFloat => builder.ins().fmul(args[0], args[1]),
        }
    }
}

fn main() {
    let mut sounds = HashMap::new();
    sounds.insert("T", Rc::new(Sound::T));
    sounds.insert("2", Rc::new(Sound::Const(IRValue::Float(2.))));
    {
        let args = vec![sounds["T"].clone(), sounds["2"].clone()];
        sounds.insert(
            "X",
            Rc::new(Sound::App(Func::Builtin(BuiltinFunc::MulFloat), args)),
        );
    }
    {
        let arg = sounds["X"].clone();
        sounds.insert("Y", Rc::new(Sound::Trim(1., 2., arg)));
    }
    {
        let args = vec![sounds["X"].clone(), sounds["Y"].clone()];
        sounds.insert(
            "Z",
            Rc::new(Sound::App(Func::Builtin(BuiltinFunc::AddFloat), args)),
        );
    }
    for (name, sound) in &sounds {
        println!("{name}: {sound:?}");
    }

    let jit_builder =
        cranelift_jit::JITBuilder::new(cranelift_module::default_libcall_names()).unwrap();
    let mut module = cranelift_jit::JITModule::new(jit_builder);

    let mut ctx = module.make_context();
    ctx.func.signature = {
        let mut sig = module.make_signature();
        sig.returns.push(AbiParam::new(types::F64));
        sig
    };
    let mut fn_builder_ctx = FunctionBuilderContext::new();

    let t = module
        .declare_data("T", Linkage::Export, true, false)
        .unwrap();
    module
        .define_data(t, &{
            let mut desc = DataDescription::new();
            desc.define_zeroinit(8);
            desc
        })
        .unwrap();
    let t = module.declare_data_in_func(t, &mut ctx.func);

    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut fn_builder_ctx);
    let block = builder.create_block();

    builder.switch_to_block(block);
    let t = builder.ins().global_value(types::I64, t);
    let old_t = builder
        .ins()
        .load(types::F64, MemFlags::new(), t, Offset32::new(0));

    let ret = sounds["X"].translate(&mut builder, old_t);

    let tick = builder.ins().f64const(1. / 100.);
    let new_t = builder.ins().fadd(old_t, tick);
    builder
        .ins()
        .store(MemFlags::new(), new_t, t, Offset32::new(0));

    builder.ins().return_(&[ret]);

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
    let ptr = unsafe { std::mem::transmute::<_, fn() -> f64>(code) };
    for _ in 0..10 {
        println!("{}", ptr());
    }
}
