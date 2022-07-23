use crate::{lexer::Token, vm::Opcode};
use super::{Parser, ParserError, primary};

type BinaryParserFunction = fn(&mut Parser) -> Result<(), ParserError>;
type MapperParserFunction = fn(Token) -> Option<Opcode>;

fn parse_binary(parser: &mut Parser, next: BinaryParserFunction, mapper: MapperParserFunction) -> Result<(), ParserError> {
    (next)(parser)?;
    while let Some(token) = parser.lexer.peek() {
        match (mapper)(token.token.clone()) {
            Some(opcode) => {
                let line = parser.lexer.next().unwrap().line;
                (next)(parser)?;
                parser.builder.push_opcode(opcode, line);
            },
            None => break,
        }
    }
    Ok(())
}

fn factor_mapper(token: Token) -> Option<Opcode> {
    match token {
        Token::Asterisk => Some(Opcode::Multiply),
        Token::Slash => Some(Opcode::Divide),
        _ => None,
    }
}

fn term_mapper(token: Token) -> Option<Opcode> {
    match token {
        Token::Plus => Some(Opcode::Addict),
        Token::Minus => Some(Opcode::Subtract),
        _ => None,
    }
}

fn parse_factor(parser: &mut Parser) -> Result<(), ParserError> {
    parse_binary(parser, primary::parse, factor_mapper)
}

fn parse_term(parser: &mut Parser) -> Result<(), ParserError> {
    parse_binary(parser, parse_factor, term_mapper)
}

pub fn parse(parser: &mut Parser) -> Result<(), ParserError> {
    parse_term(parser)
}
