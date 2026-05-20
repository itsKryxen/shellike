use crate::{
    commands::command::BuiltinCommand,
    shell::{result::ShellResult, runtime::Runtime},
};

pub struct ExitCommand;

impl BuiltinCommand for ExitCommand {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn run(&self, args: &[String], runtime: &mut Runtime) -> ShellResult {
        ShellResult::exit()
    }
}
