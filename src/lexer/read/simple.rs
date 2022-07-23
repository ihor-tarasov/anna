use crate::lexer::{reader::Reader, ParsedToken, Token, TokenInfo};

pub fn read(reader: &mut Reader) -> Option<ParsedToken> {
    let offset = reader.offset();
    let c = reader.peek()?;
    reader.skip();

    if let Some(c2) = reader.peek() {
        let token = match (c, c2) {
            (b'>', b'>') => Some(Token::GreaterGreater),
            (b'<', b'<') => Some(Token::LessLess),
            _ => None,
        };

        if let Some(token) = token {
            reader.skip();

            return Some(ParsedToken {
                info: TokenInfo {
                    offset,
                    length: 2,
                },
                token,
                line: reader.line(),
            })
        }
    }

    let token = match c {
        b'+' => Token::Plus,
        b'*' => Token::Asterisk,
        b'-' => Token::Minus,
        b'/' => Token::Slash,
        b'|' => Token::VerticalBar,
        b'&' => Token::Ampersand,
        b'%' => Token::Percent,
        b'^' => Token::Circumflex,
        _ => Token::Unknown,
    };

    Some(ParsedToken {
        info: TokenInfo {
            offset,
            length: 1,
        },
        token,
        line: reader.line(),
    })
}
