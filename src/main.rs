use std::io::Write;

use anna::{
    parser::ParserResult,
    vm::{Function, State},
};

fn run_program(program: Function) {
    match anna::vm::run(State::new(program)) {
        Ok(value) => {
            println!("{:?}", value);
        }
        Err(error) => {
            println!("Runtime error: {}", error.message);
        }
    }
}

fn parse_code(source: &[u8]) -> ParserResult {
    let parser = anna::parser::Parser::new(source);
    anna::parser::parse(parser)
}

fn parse_and_run(source: &[u8]) -> bool {
    match parse_code(source) {
        Ok(program) => {
            run_program(program);
            true
        }
        Err(error) => {
            if error.info.length == 0 {
                false
            } else {
                anna::utils::print_info("stdin", source, error.info.clone());
                println!("Parser error: {}", error.message);
                true
            }
        }
    }
}

fn read_line() -> String {
    let mut line = String::new();

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    line
}

fn repl_iteration() {
    let mut code = String::new();
    print!("-> ");

    loop {
        code.push_str(read_line().as_str());

        if !parse_and_run(code.as_bytes()) {
            print!("-| ");
        } else {
            break;
        }
    }
}

fn main() {
    loop {
        repl_iteration();
    }
}
