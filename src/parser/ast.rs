pub enum Ast {
    Command(CommandAst),
}

pub struct CommandAst {
    pub name: String,
    pub args: Vec<String>,
}
