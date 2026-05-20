use crate::shell::{result::ShellResult, runtime::Runtime};

pub trait BuiltinCommand {
    fn name(&self) -> &'static str;
    fn run(&self, args: &[String], runtime: &mut Runtime) -> ShellResult;
}
