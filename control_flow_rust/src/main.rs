fn main() {
    let source = vec![
        AStmt::If("a", ABlock::new(vec![]), ABlock::new(vec![])),
        AStmt::If(
            "b",
            ABlock::new(vec![AStmt::Expr("bX")]),
            ABlock::new(vec![]),
        ),
        AStmt::If(
            "c",
            ABlock::new(vec![AStmt::Expr("cX"), AStmt::Expr("cY")]),
            ABlock::new(vec![AStmt::Expr("cZ"), AStmt::Expr("cW")]),
        ),
        AStmt::While("d", ABlock::new(vec![])),
        AStmt::While("e", ABlock::new(vec![AStmt::Expr("eX")])),
        AStmt::While("f", ABlock::new(vec![AStmt::Expr("fX"), AStmt::Expr("fY")])),
        AStmt::While(
            "g",
            ABlock::new(vec![
                AStmt::If(
                    "ga",
                    ABlock::new(vec![AStmt::Expr("gaX"), AStmt::Expr("gaY")]),
                    ABlock::new(vec![]),
                ),
                AStmt::While(
                    "gb",
                    ABlock::new(vec![AStmt::Expr("gbX"), AStmt::Expr("gbY")]),
                ),
            ]),
        ),
    ];
    let mut builder = Builder::new();
    builder.add_stmts(source, None);
    for (i, stmt) in builder.result().iter().enumerate() {
        match stmt {
            IStmt::Expr(expr, next) => println!("{i}: {expr} -> {next:?}"),
            IStmt::Br(cond, next_true, next_false) => {
                println!("{i}: {cond} -> {next_true:?}, {next_false:?}")
            }
        }
    }
}

type Expr = &'static str;

enum AStmt {
    Expr(Expr),
    If(Expr, ABlock, ABlock),
    While(Expr, ABlock),
}
struct ABlock {
    stmts: Vec<AStmt>,
    size: usize,
}
impl ABlock {
    fn new(stmts: Vec<AStmt>) -> ABlock {
        let size = stmts
            .iter()
            .map(|stmt| match stmt {
                AStmt::Expr(_) => 1,
                AStmt::If(_, block_true, block_false) => 1 + block_true.size + block_false.size,
                AStmt::While(_, block) => 1 + block.size,
            })
            .sum();
        ABlock { stmts, size }
    }
}

enum IStmt {
    Expr(Expr, Option<usize>),
    Br(Expr, Option<usize>, Option<usize>),
}

struct Builder {
    stmts: Vec<IStmt>,
}
impl Builder {
    fn new() -> Builder {
        Builder { stmts: Vec::new() }
    }
    fn add_stmt(&mut self, stmt: IStmt) -> usize {
        let num = self.stmts.len();
        self.stmts.push(stmt);
        num
    }
    fn result(self) -> Vec<IStmt> {
        self.stmts
    }
    fn add_stmts(&mut self, mut stmts: Vec<AStmt>, end: Option<usize>) -> Option<usize> {
        if let Some(stmt) = stmts.pop() {
            let cur = match stmt {
                AStmt::Expr(expr) => self.add_stmt(IStmt::Expr(expr, end)),
                AStmt::If(cond, block_true, block_false) => {
                    let next_true = self.add_stmts(block_true.stmts, end);
                    let next_false = self.add_stmts(block_false.stmts, end);
                    self.add_stmt(IStmt::Br(cond, next_true, next_false))
                }
                AStmt::While(cond, block) => {
                    let cur = self.stmts.len() + block.size;
                    let next = self.add_stmts(block.stmts, Some(cur));
                    assert_eq!(cur, self.stmts.len());
                    self.add_stmt(IStmt::Br(cond, next, end))
                }
            };
            self.add_stmts(stmts, Some(cur))
        } else {
            end
        }
    }
}
