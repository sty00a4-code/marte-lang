pub type Register = u8;
pub type Address = u16;
pub type LiteralInt = i16;
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ByteCode {
    NOP,
    Jump {
        addr: Address,
    },
    JumpIf {
        cond: Register,
        addr: Address,
    },
    JumpIfNot {
        cond: Register,
        addr: Address,
    },
    JumpIfSome {
        cond: Register,
        addr: Address,
    },
    JumpIfNone {
        cond: Register,
        addr: Address,
    },
    Int {
        dst: Register,
        value: LiteralInt,
    },
    Bool {
        dst: Register,
        value: bool,
    },
    String {
        dst: Register,
        addr: Address,
    },
    Binary {
        op: BinaryOperation,
        dst: Register,
        left: Register,
        right: Register,
    },
    Unary {
        op: UnaryOperator,
        dst: Register,
        right: Register,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryOperation {
    And,
    Or,
    EQ,
    NE,
    LT,
    GT,
    LE,
    GE,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnaryOperator {
    Neg,
    Not,
}