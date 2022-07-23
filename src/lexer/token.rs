#[derive(PartialEq, Clone)]
pub enum Token {
    Integer,
    Plus,                      // +
    Asterisk,                  // *
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
