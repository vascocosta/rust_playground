#[derive(Clone, Copy)]
enum Token {
    Number(i64),
    Plus,
    Star,
}

#[derive(Debug)]
enum Expr {
    Number(i64),
    BinaryOp {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peak(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<Token> {
        let token = self.peak().map(|t| t.clone());
        self.pos += 1;
        token
    }

    fn parse_primary(&mut self) -> Expr {
        match self.consume() {
            Some(Token::Number(n)) => Expr::Number(n),
            _ => panic!("expected a number"),
        }
    }

    fn token_to_operator(token: &Token) -> Option<Operator> {
        match token {
            Token::Plus => Some(Operator::Add),
            Token::Star => Some(Operator::Mul),
            _ => None,
        }
    }

    fn parse_exp(&mut self, min_prec: u8) -> Expr {
        let mut left = self.parse_primary();

        loop {
            let op = match self.peak().and_then(Self::token_to_operator) {
                Some(op) => op,
                None => break,
            };

            let prec = precedence(&op);
            if prec < min_prec {
                break;
            }

            self.consume();

            let right = self.parse_exp(prec + 1);
            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }

        left
    }
}

struct Tokenizer<'a> {
    source: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn tokenize(&self) -> Vec<Token> {
        self.source
            .split_ascii_whitespace()
            .map(|s| match s {
                "+" => Token::Plus,
                "*" => Token::Star,
                _ => Token::Number(s.parse::<i64>().expect("ivalid token")),
            })
            .collect()
    }
}

fn precedence(op: &Operator) -> u8 {
    match op {
        Operator::Add => 10,
        Operator::Mul => 20,
    }
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::BinaryOp { left, op, right } => {
            let l = eval(left);
            let r = eval(right);

            match op {
                Operator::Add => l + r,
                Operator::Mul => l * r,
            }
        }
    }
}

fn main() {
    let tokenizer = Tokenizer {
        source: "2 + 2 * 3 + 7",
    };

    let tokens = tokenizer.tokenize();

    let mut parser = Parser { tokens, pos: 0 };

    let ast = parser.parse_exp(0);

    let result = eval(&ast);

    println!("Result = {}", result);
}
