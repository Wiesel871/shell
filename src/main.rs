use std::io::{self, Write};

enum WhatDo {
    Exit,
    Fork { name: String, args: Vec<String> },
    Parallel { parts: Vec<WhatDo> },
}

fn main() {
    print!("> ");
    io::stdout().flush().expect("flushE");
    let mut line = String::new();
    while let Ok(len) = io::stdin().read_line(&mut line) {
        if len == 0 || line.trim() == "exit"  {
            break;
        }
        print!("> ");
        io::stdout().flush().expect("flushE");
        line = String::new();
    }
}
