#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    UnterminatedSingleQuote,
    UnterminatedDoubleQuote,
    DanglingEscape,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuoteState {
    None,
    Single,
    Double,
}
pub fn parse(input: &str) -> Result<Command, ParseError> {
    let tokens = tokenize(input)?;

    if tokens.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    Ok(Command {
        program: tokens[0].clone(),
        args: tokens[1..].to_vec(),
    })
}
fn tokenize(input: &str) -> Result<Vec<String>, ParseError> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut quote = QuoteState::None;
    let mut token_started = false;
    while let Some(ch) = chars.next() {
        match (quote, ch) {
            (QuoteState::None, '\'') => {
                quote = QuoteState::Single;
                token_started = true;
            }

            (QuoteState::Single, '\'') => {
                quote = QuoteState::None;
            }

            (QuoteState::None, '"') => {
                quote = QuoteState::Double;
                token_started = true;
            }

            (QuoteState::Double, '"') => {
                quote = QuoteState::None;
            }

            (QuoteState::None, ch) if ch.is_whitespace() => {
                if token_started {
                    tokens.push(std::mem::take(&mut current));
                    token_started = false;
                }
            }

            (QuoteState::None | QuoteState::Double, '\\') => {
                let Some(next) = chars.next() else {
                    return Err(ParseError::DanglingEscape);
                };

                current.push(next);
                token_started = true;
            }

            (_, ch) => {
                current.push(ch);
                token_started = true;
            }
        }
    }
    match quote {
        QuoteState::None => {}
        QuoteState::Single => return Err(ParseError::UnterminatedSingleQuote),
        QuoteState::Double => return Err(ParseError::UnterminatedDoubleQuote),
    }

    if token_started {
        tokens.push(current);
    }

    Ok(tokens)
}
