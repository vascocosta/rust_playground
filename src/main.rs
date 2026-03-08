enum Token {
    Number(i64),
    Plus,
    Star,
}

enum Expr {
    Number(i64),
}

struct BinaryOp {
    left: Expr,
    op: Operator,
    right: Expr,
}

enum Operator {
    Add,
    Mul,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

fn main() {}
