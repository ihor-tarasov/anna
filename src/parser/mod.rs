use crate::{
    lexer::{Lexer, TokenInfo, Token},
    vm::{Function, FunctionBuilder, Opcode},
};

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub builder: FunctionBuilder,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            lexer: Lexer::new(source),
            builder: FunctionBuilder::new(),
        }
    }
}

pub struct ParserError {
    pub message: String,
    pub info: TokenInfo,
}

pub type ParserResult = Result<Function, ParserError>;

mod binary;
mod primary;
mod utils;

pub fn parse(mut parser: Parser) -> ParserResult {
    binary::parse(&mut parser)?;
    
    if let Some(token) = parser.lexer.peek() {
        if token.token == Token::Unknown {
            utils::unknown(token.info.clone())?;
        } else {
            utils::unexpected(token.info.clone())?;
        }
    } else {
        parser.builder.push_opcode(Opcode::End, 0);
    }

    Ok(parser.builder.build())
}
