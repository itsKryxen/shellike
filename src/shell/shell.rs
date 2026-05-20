use crate::{
    commands::registry::CommandRegistry,
    shell::{result::ShellResult, runtime::Runtime},
};

pub struct Shell {
    runtime: Runtime,
    commands: CommandRegistry,
}
impl Shell {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new(),
            commands: CommandRegistry::default_builtins(),
        }
    }
    pub fn submit_line(&mut self, line: &str) -> ShellResult {
        let line = line.trim();

        if line.is_empty() {
            return ShellResult::ok(vec![]);
        }

        let parts: Vec<String> = line.split_whitespace().map(String::from).collect();

        let name = &parts[0];
        let args = &parts[1..];

        let result = self.commands.run(name, args, &mut self.runtime);
        self.runtime.last_status = result.status;
        result
    }
}
