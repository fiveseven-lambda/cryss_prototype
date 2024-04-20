use crate::ty::Unify;

mod ty;

fn main() {
    let x = ty::new_ty(
        vec![vec![ty::new_ty(vec![], ty::new_undetermined())]],
        ty::new_ret(String::from("abc"), vec![]),
    );
    let y = ty::new_ty(
        vec![vec![ty::new_ty(vec![], ty::new_undetermined())]],
        ty::new_ret(String::from("abc"), vec![]),
    );
    let z = ty::new_ty(
        vec![vec![ty::new_ty(
            vec![],
            ty::new_ret(String::from("def"), vec![]),
        )]],
        ty::new_ret(String::from("abc"), vec![]),
    );
    println!("{x}\n{y}");
    x.unify(&y);
    y.unify(&z);
    println!("{x}\n{y}");
}
