mod ty;

fn main() {
    let x = ty::new_returns(ty::new_undetermined());
    let y = ty::new_determined(
        String::from("hoge"),
        vec![ty::new_returns(ty::new_undetermined())],
    );
    println!("{x}, {y}");
    x.unify(&ty::new_returns(y.clone()));
    println!("{x}, {y}");
}
