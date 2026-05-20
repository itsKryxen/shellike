use crate::shell::shell::Shell;

pub trait FrontendRunner {
    fn new() -> Self
    where
        Self: Sized;
    fn run(&mut self, shell: &mut Shell) -> std::io::Result<()>;
}
