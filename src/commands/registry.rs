use std::collections::HashMap;

use crate::{
    commands::{
        clear::ClearCommand, command::BuiltinCommand, echo::EchoCommand, exit::ExitCommand,
    },
    shell::{result::ShellResult, runtime::Runtime},
};

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn BuiltinCommand>>,
}
impl CommandRegistry {
    pub fn default_builtins() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(EchoCommand));
        registry.register(Box::new(ClearCommand));
        registry.register(Box::new(ExitCommand));

        registry
    }
    pub fn register(&mut self, command: Box<dyn BuiltinCommand>) {
        let name = command.name().to_string();
        self.commands.insert(name, command);
    }
    pub fn run(&self, name: &str, args: &[String], runtime: &mut Runtime) -> ShellResult {
        match self.commands.get(name) {
            Some(command) => command.run(args, runtime),
            None => ShellResult::error(format!("{name}: command not found")),
        }
    }

    fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }
}
