use lavendeux_parser::{ParserState, Token};
use std::collections::VecDeque;
use std::env;
use std::io::{stdin, stdout, Write};

/// Get the next command from the user
fn next_command() -> String {
    let mut input = String::new();
    print!("> ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    return input.trim().to_string();
}

fn main() {
    let mut state: ParserState = ParserState::new();

    // Load extensions
    let results = state.extensions.load_all("example_extensions");
    for result in results {
        if let Err(err) = result {
            println!("could not load an extension: {}", err);
        }
    }

    // Preload command stack from arguments
    let mut stack: VecDeque<String> = env::args().skip(1).collect();
    if stack.is_empty() {
        println!("Ready! Type expressions below!");
    } else {
        stack.push_back("exit".to_string());
    }

    loop {
        // Make sure we have a command ready
        if stack.is_empty() {
            stack.push_back(next_command());
        }
        let cmd = stack.pop_front().unwrap();

        if cmd.len() == 0 {
            continue;
        } else if ["exit", "quit"].contains(&cmd.as_str()) {
            break;
        } else {
            // Process the command
            match Token::new(&cmd, &mut state) {
                Ok(result) => println!("{}", result.text()),
                Err(e) => eprintln!("{}: {}", cmd, e),
            }
        }
    }
}
