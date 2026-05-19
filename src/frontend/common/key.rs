#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontendKey {
    Char(char),

    Enter,
    Backspace,
    Delete,
    Tab,
    Escape,

    Left,
    Right,
    Up,
    Down,

    Home,
    End,

    CtrlC,
    CtrlD,
}
