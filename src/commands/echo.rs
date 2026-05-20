use crate::{
    commands::command::BuiltinCommand,
    shell::{result::ShellResult, runtime::Runtime},
};

pub struct EchoCommand;

impl BuiltinCommand for EchoCommand {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, args: &[String], runtime: &mut Runtime) -> ShellResult {
        ShellResult::ok(vec![args.join(" ")])
    }
}
