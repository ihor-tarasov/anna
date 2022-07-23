use crate::lexer::TokenInfo;
use super::ParserError;

pub fn unexpected(info: TokenInfo) -> Result<(), ParserError> {
    Err(ParserError {
        message: "Unexpected token.".to_string(),
        info,
    })
}

pub fn unknown(info: TokenInfo) -> Result<(), ParserError> {
    Err(ParserError {
        message: "Unknown character.".to_string(),
        info,
    })
}
