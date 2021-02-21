use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::cli::commands::{parse_command, apply_command};

pub fn runner() {
    let mut repl = Editor::<()>::new();
    println!("storagenv - type 'help' to see a list of available commands");
    if repl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = repl.readline(">> ");

        match readline {
            Ok(line) => {
                repl.add_history_entry(line.as_str());
                if let Some(command) = parse_command(line.as_str()) {
                    if let Some(message) = apply_command(&command) {
                        println!("{}", message);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    repl.save_history("history.txt").unwrap();
}