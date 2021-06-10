use crate::pos::Pos;

// None は空の式を表す
#[derive(Debug)]
pub struct Expression(Option<PosNode>);

#[derive(Debug)]
pub struct PosNode {
    pos: Pos,
    node: Node,
}

#[derive(Debug)]
pub enum Node {
    Identifier(String, bool),
    Number(f64),
    String(String), // ←これはエスケープ処理後！
    Member(Box<Expression>, String),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
    Invocation(Box<Expression>, Vec<Expression>),
    Group(Box<Expression>),
}

#[derive(Debug)]
pub enum UnaryOperator {
    Nop,
    Minus,
    Reciprocal,
    Not,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Less,
    Greater,
    LeftShift,
    RightShift,
    Equal,
    NotEqual,
    And,
    Or,
}

impl Expression {
    pub fn new(pos: Pos, node: Node) -> Expression {
        Expression(Some(PosNode { pos: pos, node: node }))
    }
    pub fn empty() -> Expression {
        Expression(None)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
    pub fn pos(&self) -> Option<Pos> {
        self.0.as_ref().map(|PosNode { pos, .. }| pos.clone())
    }
    pub fn try_into_identifier(self) -> Option<(String, bool)> {
        match self.0 {
            Some(PosNode {
                node: Node::Identifier(identifier, dollar),
                ..
            }) => Some((identifier, dollar)),
            _ => None,
        }
    }
}
