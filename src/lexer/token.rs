#[derive(PartialEq, Clone)]
pub enum Token {
    Integer,
    Real,
    Plus,                      // +
    Minus,                     // -
    Asterisk,                  // *
    Slash,                     // /
    Ampersand,                 // &
    VerticalBar,               // |
    Percent,                   // %
    Circumflex,                // ^
    LessLess,                  // <<
    GreaterGreater,            // >>
    Unknown,
}

#[derive(Clone)]
pub struct TokenInfo {
    pub offset: usize,
    pub length: usize,
}

pub struct ParsedToken {
    pub info: TokenInfo,
    pub token: Token,
    pub line: usize,
}
