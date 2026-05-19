use crate::frontend::common::{frame::FrontendFrame, key::FrontendKey};

pub trait FrontendRunner {
    fn new() -> Self
    where
        Self: Sized;
    fn run(&mut self) -> std::io::Result<()>;
    fn submit_data(&mut self, input: &str) -> FrontendFrame;
    fn handle_key(&mut self, key: FrontendKey) -> FrontendFrame;
}
