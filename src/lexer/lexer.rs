use super::{reader::Reader, ParsedToken};
use super::{read::*, TokenInfo};

fn read(reader: &mut Reader) -> Option<ParsedToken> {
    skip_whitespaces(reader)?;

    if reader.peek()?.is_ascii_digit() {
        return read_number(reader);
    }

    read_simple(reader)
}

pub struct Lexer<'a> {
    reader: Reader<'a>,
    token: Option<ParsedToken>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        let mut reader = Reader::new(source);
        Self {
            token: read(&mut reader),
            reader,
        }
    }

    pub fn peek(&self) -> Option<&ParsedToken> {
        self.token.as_ref()
    }

    pub fn next(&mut self) -> Option<ParsedToken> {
        std::mem::replace(&mut self.token, read(&mut self.reader))
    }

    pub fn get_slice(&self, info: TokenInfo) -> &[u8] {
        self.reader.get_slice(info.offset..(info.offset + info.length))
    }
}
