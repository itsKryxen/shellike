pub enum ShellEffects {
    None,
    ClearScreen,
    Exit,
}
pub struct ShellResult {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub status: i32,
    pub effect: ShellEffects,
}

impl ShellResult {
    pub fn ok(lines: Vec<String>) -> Self {
        Self {
            stdout: lines,
            stderr: vec![],
            status: 0,
            effect: ShellEffects::None,
        }
    }
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            stdout: vec![],
            stderr: vec![message.into()],
            status: 1,
            effect: ShellEffects::None,
        }
    }
    pub fn exit() -> Self {
        Self {
            stdout: vec![],
            stderr: vec![],
            status: 0,
            effect: ShellEffects::Exit,
        }
    }
    pub fn clear() -> Self {
        Self {
            stdout: vec![],
            stderr: vec![],
            status: 0,
            effect: ShellEffects::ClearScreen,
        }
    }
}
