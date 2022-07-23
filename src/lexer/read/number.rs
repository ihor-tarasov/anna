use crate::lexer::{reader::Reader, ParsedToken, TokenInfo, Token};

pub fn read(reader: &mut Reader) -> Option<ParsedToken> {
    let offset = reader.offset();

    loop {
        let c = match reader.peek() {
            Some(c) => c,
            None => break,
        };

        if c.is_ascii_digit() {
            reader.skip();
        } else {
            break;
        }
    }

    assert_ne!(offset, reader.offset());

    Some(ParsedToken {
        info: TokenInfo {
            offset,
            length: reader.offset() - offset,
        },
        token: Token::Integer,
        line: reader.line(),
    })
}
