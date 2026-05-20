mod commands;
mod frontend;
mod parser;
mod shell;
mod vfs;
use frontend::Frontend;
use frontend::common::frontend::FrontendRunner;
use shell::shell::Shell;

fn main() -> std::io::Result<()> {
    let mut shell = Shell::new();
    let mut frontend = Frontend::new();
    frontend.run(&mut shell)
}
