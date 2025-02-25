use std::{fmt::Debug, ops::Range};

#[derive(Clone)]
pub struct Located<T: Debug + Clone> {
    pub value: T,
    pub loc: Range<usize>,
}
impl<T: Debug + Clone> Located<T> {
    pub fn new(value: T, loc: Range<usize>) -> Self {
        Self { value, loc }
    }
}
impl<T: Debug + Clone> Debug for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct Chunk(pub Vec<Located<Statement>>);

#[derive(Debug, Clone)]
pub enum Statement {
    Block(Vec<Located<Self>>),
    Let {
        ident: Located<String>,
        expr: Located<Expression>,
    },
    Assign {
        op: AssignOperator,
        path: Located<Path>,
        expr: Located<Expression>,
    },
    Call {
        path: Located<Path>,
        args: Vec<Located<Expression>>,
    },
    If {
        cond: Located<Expression>,
        case: Box<Located<Self>>,
        else_case: Option<Box<Located<Self>>>,
    },
    IfSome {
        ident: Located<String>,
        expr: Located<Expression>,
        case: Box<Located<Self>>,
        else_case: Option<Box<Located<Self>>>,
    },
    While {
        cond: Located<Expression>,
        body: Box<Located<Self>>,
    },
    WhileSome {
        ident: Located<String>,
        expr: Located<Expression>,
        body: Box<Located<Self>>,
    },
    For {
        ident: Located<String>,
        iter: Located<Expression>,
        body: Box<Located<Self>>,
    },
    Break,
    Continue,
    Return(Located<Expression>),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssignOperator {
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Exponent,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Atom(Atom),
    Binary {
        op: BinaryOperator,
        left: Box<Located<Self>>,
        right: Box<Located<Self>>,
    },
    Unary {
        op: UnaryOperator,
        right: Box<Located<Self>>,
    },
    Field {
        head: Box<Located<Self>>,
        field: Located<String>,
    },
    Index {
        head: Box<Located<Self>>,
        index: Box<Located<Self>>,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperator {
    And,
    Or,
    EqualEqual,
    ExclamationEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Exponent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOperator {
    Minus,
    Not,
}
#[derive(Debug, Clone)]
pub enum Atom {
    Null,
    Ident(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
    Expression(Box<Located<Expression>>),
    Tuple(Vec<Located<Expression>>),
    Vector(Vec<Located<Expression>>),
    Object(Vec<(Located<String>, Located<Expression>)>),
    Fn {
        params: Vec<Located<String>>,
        body: Box<Located<Statement>>,
    }
}
#[derive(Debug, Clone)]
pub enum Path {
    Ident(String),
    Field {
        head: Box<Located<Self>>,
        field: Located<String>,
    },
    Index {
        head: Box<Located<Self>>,
        index: Box<Located<Expression>>,
    },
}