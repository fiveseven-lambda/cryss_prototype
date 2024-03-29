use std::collections::VecDeque;
use std::iter;

mod ir;
mod ty;

fn main() {
    let converters = vec![
        (
            ir::Func::Deref,
            ty::Converter {
                num_vars: 1,
                from: expr!(Ref, expr!(0)),
                to: expr!(0),
            },
        ),
        (
            ir::Func::BoolToInt,
            ty::Converter {
                num_vars: 0,
                from: expr!(Bool),
                to: expr!(Int),
            },
        ),
        (
            ir::Func::IntToFloat,
            ty::Converter {
                num_vars: 0,
                from: expr!(Int),
                to: expr!(Float),
            },
        ),
    ];
    let mut vars: im::Vector<_> = (0..0).map(|_| None).collect();
    let result = vec![
        (ir::Expr::Atom("x"), ty!(Sound, ty!(Sound, ty!(Int))), expr!(Float)),
        (ir::Expr::Atom("y"), ty!(Sound, ty!(Int)), expr!(Float)),
    ]
    .iter()
    .map(|(expr, given, expected)| convert(expr, given, expected, &mut vars, &converters))
    .collect::<Option<Vec<_>>>()
    .map(|args| app(ir::Expr::Atom("f"), args));
    dbg!(&result);
}

fn convert(
    expr: &ir::Expr,
    ty: &ty::Ty,
    dest: &ty::Expr,
    vars: &mut im::Vector<Option<ty::Ty>>,
    converters: &[(ir::Func, ty::Converter)],
) -> Option<(usize, usize, ir::Expr)> {
    let (ty_depth, ty_inner) = ty.pause();
    let (dest_depth, dest_inner) = dest.pause();
    let mut queue = VecDeque::from([(ty_inner.clone(), expr.clone())]);
    while let Some((ty, expr)) = queue.pop_front() {
        let mut vars_cloned = vars.clone();
        if dest_inner.identify(&ty, &mut vars_cloned) {
            *vars = vars_cloned;
            return Some((ty_depth, dest_depth, expr));
        }
        for &(converter, ref converter_ty) in converters {
            let mut converter_vars = (0..converter_ty.num_vars).map(|_| None).collect();
            if converter_ty.from.identify(&ty, &mut converter_vars) {
                let next_ty = converter_ty.to.subst(&converter_vars);
                let next_expr = app(ir::Expr::Func(converter), vec![(ty_depth, 0, expr.clone())]);
                queue.push_back((next_ty, next_expr));
            }
        }
    }
    None
}

fn app(func: ir::Expr, args: Vec<(usize, usize, ir::Expr)>) -> ir::Expr {
    if args.iter().any(|(given, expected, _)| given > expected) {
        app(
            ir::Expr::Func(ir::Func::App),
            iter::once((0, 0, func))
                .chain(
                    args.into_iter()
                        .map(|(given, expected, expr)| (given, expected + 1, expr)),
                )
                .collect(),
        )
    } else {
        ir::Expr::Call(
            func.into(),
            args.into_iter()
                .map(|(given, expected, expr)| {
                    (given..expected).fold(expr, |expr, i| {
                        app(ir::Expr::Func(ir::Func::Const), vec![(i, 0, expr)])
                    })
                })
                .collect(),
        )
    }
}
