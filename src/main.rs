use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Token {
    Number(i64),
    Ident(String),
    Plus,
    Star,
    Greater,
    Smaller,
    Equals,
    If,
    While,
    LBrace,
    RBrace,
    LParen,
    RParen,
}

#[derive(Debug)]
enum Expr {
    Number(i64),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
enum Stmt {
    Assign {
        name: String,
        value: Expr,
    },
    Expr(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Greater,
    Smaller,
}

#[derive(Debug, Clone)]
enum Value {
    Number(i64),
    Bool(bool),
}

struct Program {
    statements: Vec<Stmt>,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn consume(&mut self) -> Option<Token> {
        let t = self.peek().cloned();
        self.pos += 1;
        t
    }

    fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();

        while self.peek().is_some() {
            statements.push(self.parse_stmt());
        }

        Program { statements }
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::Ident(_)) => {
                if matches!(self.peek_next(), Some(Token::Equals)) {
                    self.parse_assign()
                } else {
                    Stmt::Expr(self.parse_expr(0))
                }
            }
            _ => Stmt::Expr(self.parse_expr(0)),
        }
    }

    fn parse_assign(&mut self) -> Stmt {
        let name = match self.consume() {
            Some(Token::Ident(n)) => n,
            _ => panic!("expected identifier"),
        };

        self.consume(); // Discard Token::Equals

        let value = self.parse_expr(0);

        Stmt::Assign { name, value }
    }

    fn parse_if(&mut self) -> Stmt {
        self.consume(); // Discard Token::If

        let condition = self.parse_expr(0);

        // Discard Token::LBrace
        match self.consume() {
            Some(Token::LBrace) => {}
            _ => panic!("expected {{"),
        }

        let mut then_branch = Vec::new();

        while !matches!(self.peek(), Some(Token::RBrace)) {
            then_branch.push(self.parse_stmt());
        }

        self.consume(); // Discard Token::RBrace

        Stmt::If {
            condition,
            then_branch,
        }
    }

    fn parse_while(&mut self) -> Stmt {
        self.consume(); // while

        let condition = self.parse_expr(0);

        match self.consume() {
            Some(Token::LBrace) => {}
            _ => panic!("expected {{"),
        }

        let mut body = Vec::new();

        while !matches!(self.peek(), Some(Token::RBrace)) {
            body.push(self.parse_stmt());
        }

        self.consume(); // }

        Stmt::While { condition, body }
    }

    fn parse_expr(&mut self, min_prec: u8) -> Expr {
        let mut left = self.parse_primary();

        loop {
            let op = match self.peek().and_then(Self::token_to_operator) {
                Some(op) => op,
                None => break,
            };

            let prec = precedence(&op);
            if prec < min_prec {
                break;
            }

            self.consume();

            let right = self.parse_expr(prec + 1);

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }

        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.consume() {
            Some(Token::Number(n)) => Expr::Number(n),
            Some(Token::Ident(name)) => Expr::Variable(name),
            Some(Token::LParen) => {
                let expr = self.parse_expr(0);

                match self.consume() {
                    Some(Token::RParen) => expr,
                    _ => panic!("expected ')'"),
                }
            }

            other => panic!("unexpected token: {:?}", other),
        }
    }

    fn token_to_operator(token: &Token) -> Option<Operator> {
        match token {
            Token::Plus => Some(Operator::Add),
            Token::Star => Some(Operator::Mul),
            Token::Greater => Some(Operator::Greater),
            Token::Smaller => Some(Operator::Smaller),
            _ => None,
        }
    }
}

fn precedence(op: &Operator) -> u8 {
    match op {
        Operator::Greater => 5,
        Operator::Smaller => 5,
        Operator::Add => 10,
        Operator::Mul => 20,
    }
}

struct Env {
    vars: HashMap<String, Value>,
}

impl Env {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}

fn eval_expr(expr: &Expr, env: &Env) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(*n),
        Expr::Variable(name) => env.vars.get(name).expect("undefined variable").clone(),
        Expr::Binary { left, op, right } => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);

            match (l, r, op) {
                (Value::Number(a), Value::Number(b), Operator::Add) => Value::Number(a + b),
                (Value::Number(a), Value::Number(b), Operator::Mul) => Value::Number(a * b),
                (Value::Number(a), Value::Number(b), Operator::Greater) => Value::Bool(a > b),
                (Value::Number(a), Value::Number(b), Operator::Smaller) => Value::Bool(a < b),
                _ => panic!("type error"),
            }
        }
    }
}

fn exec_stmt(stmt: &Stmt, env: &mut Env) -> Option<Value> {
    match stmt {
        Stmt::Assign { name, value } => {
            let v = eval_expr(value, env);
            env.vars.insert(name.clone(), v);
            None
        }

        Stmt::Expr(expr) => Some(eval_expr(expr, env)),

        Stmt::If {
            condition,
            then_branch,
        } => {
            let cond = eval_expr(condition, env);

            if let Value::Bool(true) = cond {
                for stmt in then_branch {
                    exec_stmt(stmt, env);
                }
            }

            None
        }

        Stmt::While { condition, body } => {
            loop {
                let cond = eval_expr(condition, env);

                match cond {
                    Value::Bool(true) => {
                        for stmt in body {
                            exec_stmt(stmt, env);
                        }
                    }

                    Value::Bool(false) => break,

                    _ => panic!("while condition must be bool"),
                }
            }

            None
        }
    }
}

fn run(program: &Program) -> Option<Value> {
    let mut env = Env::new();
    let mut last = None;

    for stmt in &program.statements {
        if let Some(v) = exec_stmt(stmt, &mut env) {
            last = Some(v);
        }
    }

    last
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new<'a>(source: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.pos += 1;
        ch
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.peek() {
            match c {
                // ignore whitespace
                c if c.is_whitespace() => {
                    self.advance();
                }

                // numbers
                c if c.is_ascii_digit() => {
                    tokens.push(self.lex_number());
                }

                // identifiers / keywords
                c if c.is_ascii_alphabetic() => {
                    tokens.push(self.lex_identifier());
                }

                '+' => {
                    self.advance();
                    tokens.push(Token::Plus);
                }

                '*' => {
                    self.advance();
                    tokens.push(Token::Star);
                }

                '>' => {
                    self.advance();
                    tokens.push(Token::Greater);
                }

                '<' => {
                    self.advance();
                    tokens.push(Token::Smaller);
                }

                '=' => {
                    self.advance();
                    tokens.push(Token::Equals);
                }

                '{' => {
                    self.advance();
                    tokens.push(Token::LBrace);
                }

                '}' => {
                    self.advance();
                    tokens.push(Token::RBrace);
                }

                '(' => {
                    self.advance();
                    tokens.push(Token::LParen);
                }

                ')' => {
                    self.advance();
                    tokens.push(Token::RParen);
                }

                _ => panic!("Unexpected character: {}", c),
            }
        }

        tokens
    }

    fn lex_number(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let num: String = self.chars[start..self.pos].iter().collect();

        Token::Number(num.parse().unwrap())
    }

    fn lex_identifier(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident: String = self.chars[start..self.pos].iter().collect();

        match ident.as_str() {
            "if" => Token::If,
            "while" => Token::While,
            _ => Token::Ident(ident),
        }
    }
}

fn main() {
    let source = "
        x = 0

        while x < 5 {
            x = x + 1
        }

        x
    ";

    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize();

    let mut parser = Parser { tokens, pos: 0 };

    let program = parser.parse_program();

    println!("AST:\n{:#?}", program.statements);

    let result = run(&program);

    println!("\nProgram result: {:?}", result);
}
