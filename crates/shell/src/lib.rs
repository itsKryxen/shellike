#[derive(Debug)]
pub enum ShellAction {
    Continue,
    Exit,
    ClearScreen,
    ParseError(parser::ParseError),
}

#[derive(Debug, Default)]
pub struct Shell;

impl Shell {
    pub fn new() -> Self {
        Self
    }

    pub fn submit_line(&mut self, input: &str) -> ShellAction {
        let command = match parser::parse(input) {
            Ok(command) => command,
            Err(error) => return ShellAction::ParseError(error),
        };

        println!("{command:?}");

        match command.program.as_str() {
            "exit" => ShellAction::Exit,
            "clear" => ShellAction::ClearScreen,
            _ => ShellAction::Continue,
        }
    }
}
