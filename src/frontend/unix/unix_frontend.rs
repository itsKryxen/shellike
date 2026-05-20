use crate::frontend::common::key::FrontendKey;
use crate::frontend::unix::termios::RawMode;
use crate::shell::result::ShellResult;
use crate::shell::shell::Shell;
use crate::{frontend::common::frontend::FrontendRunner, shell::result::ShellEffects};
use std::io::{self, Read, Write};

pub struct UnixFrontend {
    input: String,
    cursor: usize,
}
impl UnixFrontend {
    fn handle_edit_key(&mut self, key: FrontendKey) {
        match key {
            FrontendKey::Char(ch) => {
                self.input.insert(self.cursor, ch);
                self.cursor += ch.len_utf8();
            }

            FrontendKey::Backspace => {
                if self.cursor > 0 {
                    let prev = previous_char_boundary(&self.input, self.cursor);
                    self.input.replace_range(prev..self.cursor, "");
                    self.cursor = prev;
                }
            }

            FrontendKey::Delete => {
                if self.cursor < self.input.len() {
                    let next = next_char_boundary(&self.input, self.cursor);
                    self.input.replace_range(self.cursor..next, "");
                }
            }

            FrontendKey::Left => {
                if self.cursor > 0 {
                    self.cursor = previous_char_boundary(&self.input, self.cursor);
                }
            }

            FrontendKey::Right => {
                if self.cursor < self.input.len() {
                    self.cursor = next_char_boundary(&self.input, self.cursor);
                }
            }

            FrontendKey::Home => {
                self.cursor = 0;
            }

            FrontendKey::End => {
                self.cursor = self.input.len();
            }

            FrontendKey::CtrlC => {
                self.input.clear();
                self.cursor = 0;
            }

            _ => {}
        }
    }
    fn render_shell_result(&mut self, result: ShellResult) -> io::Result<bool> {
        for line in result.stdout {
            println!("\r{line}");
        }

        for line in result.stderr {
            eprintln!("\r{line}");
        }

        match result.effect {
            ShellEffects::None => Ok(false),

            ShellEffects::ClearScreen => {
                print!("\x1B[2J\x1B[H");
                io::stdout().flush()?;
                Ok(false)
            }

            ShellEffects::Exit => {
                println!();
                Ok(true)
            }
        }
    }
    fn draw_input(&self) -> io::Result<()> {
        let prompt = "shellike> ";

        print!("\r\x1B[2K");
        print!("{prompt}{}", self.input);

        let input_chars = self.input.chars().count();
        let cursor_chars = self.input[..self.cursor].chars().count();
        let move_left = input_chars.saturating_sub(cursor_chars);

        if move_left > 0 {
            print!("\x1B[{move_left}D");
        }

        io::stdout().flush()
    }
}
impl FrontendRunner for UnixFrontend {
    fn new() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
        }
    }

    fn run(&mut self, shell: &mut Shell) -> io::Result<()> {
        let _raw = RawMode::enable()?;
        let mut stdin = io::stdin().lock();

        self.draw_input()?;

        loop {
            let key = read_key(&mut stdin)?;

            match key {
                FrontendKey::Enter => {
                    println!();

                    let line = std::mem::take(&mut self.input);
                    self.cursor = 0;

                    let result = shell.submit_line(line.trim());

                    let should_exit = self.render_shell_result(result)?;

                    if should_exit {
                        break;
                    }

                    self.draw_input()?;
                }

                FrontendKey::CtrlD if self.input.is_empty() => {
                    println!();
                    break;
                }

                key => {
                    self.handle_edit_key(key);
                    self.draw_input()?;
                }
            }
        }

        Ok(())
    }
}

fn read_key(stdin: &mut impl Read) -> io::Result<FrontendKey> {
    let mut byte = [0u8; 1];
    stdin.read_exact(&mut byte)?;

    match byte[0] {
        b'\r' | b'\n' => Ok(FrontendKey::Enter),
        0x7f | 0x08 => Ok(FrontendKey::Backspace),
        b'\t' => Ok(FrontendKey::Tab),
        0x03 => Ok(FrontendKey::CtrlC),
        0x04 => Ok(FrontendKey::CtrlD),
        0x1b => read_escape_sequence(stdin),

        byte if byte.is_ascii() && !byte.is_ascii_control() => Ok(FrontendKey::Char(byte as char)),

        _ => Ok(FrontendKey::Escape),
    }
}
fn read_escape_sequence(stdin: &mut impl Read) -> io::Result<FrontendKey> {
    let mut seq = [0u8; 1];

    if stdin.read_exact(&mut seq).is_err() {
        return Ok(FrontendKey::Escape);
    }

    if seq[0] != b'[' {
        return Ok(FrontendKey::Escape);
    }

    let mut code = [0u8; 1];
    stdin.read_exact(&mut code)?;

    match code[0] {
        b'A' => Ok(FrontendKey::Up),
        b'B' => Ok(FrontendKey::Down),
        b'C' => Ok(FrontendKey::Right),
        b'D' => Ok(FrontendKey::Left),
        b'H' => Ok(FrontendKey::Home),
        b'F' => Ok(FrontendKey::End),

        b'3' => {
            let mut tilde = [0u8; 1];
            stdin.read_exact(&mut tilde)?;

            if tilde[0] == b'~' {
                Ok(FrontendKey::Delete)
            } else {
                Ok(FrontendKey::Escape)
            }
        }

        _ => Ok(FrontendKey::Escape),
    }
}

fn previous_char_boundary(s: &str, cursor: usize) -> usize {
    s[..cursor]
        .char_indices()
        .last()
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}

fn next_char_boundary(s: &str, cursor: usize) -> usize {
    s[cursor..]
        .char_indices()
        .nth(1)
        .map(|(idx, _)| cursor + idx)
        .unwrap_or(s.len())
}
