//! ファイルからの入力

mod fmt;
mod get_ty;
mod parse;
pub use parse::parse;

/// 入力
pub enum Input {
    /// 関数や定数の型を定義．
    Def(String, Ty),
    /// 式
    Expr(Expr),
}

/// 型
pub enum Ty {
    /// 型変数
    Var(String),
    /// 非関数型
    NonFunc { kind: String, args: Vec<Ty> },
    /// 関数型
    Func { args: Vec<Ty>, ret: Box<Ty> },
}

/// 式
pub struct Expr {
    /// 変数名または関数名
    identifier: String,
    /// 関数呼び出し
    calls: Vec<Call>,
}

/// 関数呼び出し
pub struct Call {
    /// 引数のリスト
    pub args: Vec<Expr>,
}
