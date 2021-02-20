use std::fs::{File, create_dir};
use std::fs;
use std::io::{Write, Read};
use clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Help,
    Invalid,
    Pick(&'a str),
    List,
    Add(&'a str, &'a str),
    Remove(&'a str),
}


fn help_command<'a>() -> Option<&'a str> {
    Some("Enter exit to quit.\nEnter 'list' to list all stored env\nEnter 'remove ENV_NAME' to remove specific environment\nEnter 'pick ENV_NAME' to retrieve specific environment file\nEnter 'add ENV_NAME ENV_STRING' to add new environment file\n\n")
}

fn invalid_command<'a>() -> Option<&'a str> {
    Some("invalid command, run 'help' to see a list of valid commands")
}

fn remove_command(env_name: &str) -> Option<&str> {
    let unable_to_remove = "Unable to remove file";
    if fs::remove_file(&format!("envs/{}", env_name)).is_err() {
        Some(unable_to_remove)
    } else {
        None
    }
}

fn list_command<'a>() -> Option<&'a str> {
    let path = "envs";
    let empty_dir = "No env's created yet";
    if let Ok(directory) = fs::read_dir(path) {
        let mut empty = true;
        println!("storagenv - Stored env's");
        for entry in directory {
            empty = false;
            println!("- {}", entry.unwrap().file_name().to_str().unwrap());
        }
        if empty {
            Some(empty_dir)
        } else {
            None
        }
    } else {
        create_dir(path).expect("Could not initialize directory to store envs");
        Some(empty_dir)
    }
}

fn add_command<'a>(env_name: &str, env_str: &str) -> Option<&'a str> {
    let unable_message = "Could not write env, try again later";

    if let Ok(mut file) = File::create(&format!("envs/{}", env_name)) {
        for line in env_str.split("\\n") {
            if file.write(format!("{}\n", line).as_bytes()).is_err() {
                return Some(unable_message)
            }
        }
    } else {
        return Some(unable_message)
    }
    None
}

fn pick_command(env_name: &str) -> Option<&str> {
    let invalid_file = "Invalid env name";
    let invalid_clipboard = "Could not set to clipboard";
    if let Ok(mut file) = File::open(format!("envs/{}", env_name)) {
        let mut content = String::new();
        if file.read_to_string(&mut content).is_err() {
            Some(invalid_file)
        } else {
            let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Could not initialize clipboard");
            if ctx.set_contents(content).is_err() {
                return Some(invalid_clipboard)
            }
            None
        }
    } else {
        Some(invalid_file)
    }
}

pub fn apply_command<'a>(command: &'a Command) -> Option<&'a str> {
    match command {
        Command::Help => help_command(),
        Command::List => list_command(),
        Command::Pick(name) => pick_command(name),
        Command::Add(name, env) => add_command(name, env),
        Command::Remove(name) => remove_command(name),
        Command::Invalid => invalid_command(),
    }
}

pub fn parse_command(stmt: &str) -> Option<Command> {
    let sub_stmts: Vec<&str> = stmt.split(' ').collect();
    let first_stmt = sub_stmts.first().expect("Empty statement");
    match *first_stmt {
        "help" => Some(Command::Help),
        "list" => Some(Command::List),
        "pick" => {
            if let Some(env_name) = sub_stmts.get(1) {
                Some(Command::Pick(env_name))
            } else {
                println!("Invalid pick command");
                None
            }
        },
        "remove" => {
            if let Some(env_name) = sub_stmts.get(1) {
                Some(Command::Remove(env_name))
            } else {
                println!("Invalid remove command");
                None
            }
        }
        "add" => {
            if let Some(env_name) = sub_stmts.get(1) {
                if let Some(env_str) = sub_stmts.get(2) {
                    Some(Command::Add(env_name, env_str))
                } else {
                    println!("Invalid ENV file");
                    None
                }
            } else {
                println!("Invalid add statement");
                None
            }
        }
        _ => Some(Command::Invalid),
    }
}




#[cfg(test)]
mod tests {
    use super::{parse_command, Command};

    #[test]
    fn it_parses_help_command_correctly() {
        let stmt = "help";
        let command = parse_command(stmt).unwrap();
        assert_eq!(command, Command::Help);
    }

    #[test]
    fn it_parses_list_command_correctly() {
        let stmt = "list";
        let command = parse_command(stmt).unwrap();
        assert_eq!(command, Command::List);
    }

    #[test]
    fn it_parses_invalid_command_correctly() {
        let stmt = "not_a_valid_statement";
        let command = parse_command(stmt).unwrap();
        assert_eq!(command, Command::Invalid);
    }

    #[test]
    fn it_parses_pick_command_correctly() {
        let stmt = "pick this.env";
        let command = parse_command(stmt).unwrap();
        assert_eq!(command, Command::Pick("this.env"));
    }

    #[test]
    fn it_parses_add_command_correctly() {
        let stmt = "add another.env ENV1=test\nENV2=another";
        let command = parse_command(stmt).unwrap();
        assert_eq!(command, Command::Add("another.env", "ENV1=test\nENV2=another"));
    }

    #[test]
    fn it_parses_remove_command_correctly() {
        let stmt = "remove another.env";
        let command = parse_command(stmt).unwrap();
        assert_eq!(command, Command::Remove("another.env"));
    }
}