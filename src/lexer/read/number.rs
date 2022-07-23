use crate::lexer::{reader::Reader, ParsedToken, TokenInfo, Token};

pub fn read(reader: &mut Reader) -> Option<ParsedToken> {
    let offset = reader.offset();

    let mut is_real = false;

    loop {
        let c = match reader.peek() {
            Some(c) => c,
            None => break,
        };

        if c.is_ascii_digit() || c == b'.' {
            if c == b'.' {
                if is_real {
                    break;
                } else {
                    is_real = true;
                }
            }
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
        token: if is_real { Token::Real } else { Token::Integer },
        line: reader.line(),
    })
}
