use crate::lexer::reader::Reader;

fn is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t' || c == b'\r' || c == b'\n'
}

pub fn skip_whitespaces(reader: &mut Reader) -> Option<()> {
    loop {
        let c = reader.peek()?;
        if is_whitespace(c) {
            if c == b'\n' {
                reader.next_line();
            }
            reader.skip();
        } else {
            break;
        }
    }
    Some(())
}
