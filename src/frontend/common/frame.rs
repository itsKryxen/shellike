#[derive(Debug, Clone)]
pub enum FrontendEffect {
    None,
    ClearScreen,
    Exit,
}
#[derive(Debug, Clone)]
pub struct FrontendFrame {
    pub prompt: String,
    pub input: String,
    pub output: Vec<String>,
    pub effect: FrontendEffect,
}
impl FrontendFrame {
    pub fn input(prompt: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            input: input.into(),
            output: vec![],
            effect: FrontendEffect::None,
        }
    }

    pub fn output(lines: Vec<String>) -> Self {
        Self {
            prompt: "shellike> ".into(),
            input: String::new(),
            output: lines,
            effect: FrontendEffect::None,
        }
    }

    pub fn clear() -> Self {
        Self {
            prompt: "shellike> ".into(),
            input: String::new(),
            output: vec![],
            effect: FrontendEffect::ClearScreen,
        }
    }

    pub fn exit() -> Self {
        Self {
            prompt: "shellike> ".into(),
            input: String::new(),
            output: vec![],
            effect: FrontendEffect::Exit,
        }
    }
}
