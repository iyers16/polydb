use std::io::{self, Write};
use polydb::code_file::CodeFile;


fn main() {

    let code : CodeFile = CodeFile::new().expect("Failed to parse code");

    println!("{}", code);
    
    loop {
        let user_input = get_cli_input();
        let _command = match user_input.as_str() {
            "exit" | "quit" | "q" => break,
            _ => user_input,
        };
    }
}

fn get_cli_input() -> String {
    print!("polydb> ");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    return String::from(user_input.trim());
}