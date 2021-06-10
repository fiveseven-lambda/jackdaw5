use crate::pos::Pos;

#[derive(Debug)]
pub struct Token {
    pub name: TokenName,
    pub lexeme: String,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum TokenName {
    Identifier { dollar: bool },
    Number{ scientific: bool },
    String,
    Plus,             // +
    Minus,            // -
    Asterisk,         // *
    Slash,            // /
    Circumflex,       // ^
    Equal,            // =
    DoubleEqual,      // ==
    Exclamation,      // !
    ExclamationEqual, // !=
    Less,             // <
    DoubleLess,       // <<
    Greater,          // >
    DoubleGreater,    // >>
    Ampersand,        // &
    DoubleAmpersand,  // &&
    Bar,              // |
    DoubleBar,        // ||
    Colon,            // :
    Semicolon,        // ;
    Comma,            // ,
    Dot,              // .
    OpeningParen,     // (
    ClosingParen,     // )
    OpeningBracket,   // [
    ClosingBracket,   // ]
    OpeningBrace,     // {
    ClosingBrace,     // }
}
