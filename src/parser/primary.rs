use crate::{lexer::{Lexer, ParsedToken, TokenInfo, Token}, vm::{Value, Opcode}};

use super::{ParserError, Parser, utils};

fn require(lexer: &mut Lexer) -> Result<ParsedToken, ParserError> {
    match lexer.next() {
        Some(token) => Ok(token),
        None => Err(ParserError {
            message: "Unexpected end of file.".to_string(),
            info: TokenInfo { offset: 0, length: 0 },
        }),
    }
}

fn from_utf8(slice: &[u8]) -> &str {
    std::str::from_utf8(slice).unwrap()
}

fn str_to_int(src: &str) -> Value {
    Value::Int(src.parse::<i64>().unwrap())
}

fn parse_integer(parser: &mut Parser, info: TokenInfo, line: usize) -> Result<(), ParserError> {
    let value = str_to_int(from_utf8(parser.lexer.get_slice(info)));
    let index = parser.builder.push_constant(value);
    parser.builder.push_opcode(Opcode::Const(index), line);
    Ok(())
}

pub fn parse(parser: &mut Parser) -> Result<(), ParserError> {
    let token = require(&mut parser.lexer)?;

    match token.token {
        Token::Integer => parse_integer(parser, token.info, token.line),
        Token::Unknown => utils::unknown(token.info),
        _ => utils::unexpected(token.info),
    }
}
