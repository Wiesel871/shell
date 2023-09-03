use std::io::{stdin, stdout, stderr, Write, BufReader, BufRead};

enum WhatDo {
    Exit,

    Fork { name: String, args: Vec<String> },
    Parallel { parts: Vec<WhatDo> },
}

fn main() -> std::io::Result<()> {
    let should_print = |b| {if b {print!("> "); stdout().flush().expect("flushE");}};
    let basic_commands = vec!["echo", "exit", "pwd", "cd"];
    let (mut input_file, out): (Box<dyn BufRead>, bool) = match std::env::args().nth(1) {
        Some(input) => (Box::new(BufReader::new(std::fs::File::open(input).unwrap())), 
                        false),
        None => (Box::new(BufReader::new(stdin())),
                true)
    };
    should_print(out);
    let mut line = String::new();
    while let Ok(len) = input_file.read_line(&mut line) {
        if len == 0 {
            break;
        }

        should_print(out);
        line = String::new();
    };
    Ok(())
}
