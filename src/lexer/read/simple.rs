use crate::lexer::{reader::Reader, ParsedToken, Token, TokenInfo};


pub fn read_simple(reader: &mut Reader) -> Option<ParsedToken> {
    let offset = reader.offset();
    let c = reader.peek()?;

    let token = match c {
        b'+' => Token::Plus,
        b'*' => Token::Asterisk,
        _ => Token::Unknown,
    };

    if token != Token::Unknown {
        reader.skip();
    }

    Some(ParsedToken {
        info: TokenInfo {
            offset,
            length: 1,
        },
        token,
        line: reader.line(),
    })
}
