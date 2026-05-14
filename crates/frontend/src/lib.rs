use std::io::{self, Write};
pub fn run() {
    let mut shell = shell::Shell::new();

    'main_loop: loop {
        print!("shellike> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break 'main_loop,
            Ok(_) => {}
            Err(error) => {
                eprintln!("input error: {error}");
                continue;
            }
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match shell.submit_line(input) {
            shell::ShellAction::Continue => {}

            shell::ShellAction::Exit => {
                break 'main_loop;
            }

            shell::ShellAction::ClearScreen => {
                print!("\x1B[2J\x1B[H");
                io::stdout().flush().expect("failed to flush stdout");
            }

            shell::ShellAction::ParseError(error) => {
                eprintln!("error at parsing: {error:?}");
            }
        }
    }
}
