use crate::{
    commands::command::BuiltinCommand,
    shell::{result::ShellResult, runtime::Runtime},
};

pub struct ClearCommand;

impl BuiltinCommand for ClearCommand {
    fn name(&self) -> &'static str {
        "clear"
    }

    fn run(&self, args: &[String], runtime: &mut Runtime) -> ShellResult {
        ShellResult::clear()
    }
}
