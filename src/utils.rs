use crate::lexer::TokenInfo;

pub fn print_info(file: &str, source: &[u8], info: TokenInfo) {
    let mut line = 0;
    let mut line_offset = 0;
    let mut offset = 0;

    while offset < info.offset {
        if source[offset] == b'\n' {
            line += 1;
            line_offset = offset + 1;
        }
        offset += 1;
    }

    println!("File: \"{}\", line: {}", file, (line + 1));

    let mut i = line_offset;
    while source[i] != b'\r' && source[i] != b'\n' {
        print!("{}", source[i] as char);
        i += 1;
    }
    println!();
    
    for _ in line_offset..offset {
        print!(" ");
    }
    for _ in offset..(offset + info.length) {
        print!("^");
    }
    println!();
}
