mod frontend;
mod parser;
use frontend::Frontend;
use frontend::common::frontend::FrontendRunner;
fn main() -> std::io::Result<()> {
    let mut frontend = Frontend::new();
    frontend.run()
}
