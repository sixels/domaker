use domaker_base::location::Location;
use lalrpop_util::{lexer::Token, ParseError};


#[derive(Debug)]
pub struct ParseErrors {
    pub filename: String,
    pub errors: Vec<Error>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid token at {location}")]
    InvalidToken { location: Location },
    #[error("expected {expected:?} at {location}")]
    ExpectedToken { location: Location, expected: &'static str },
}

pub type LalrpopError<'input> = ParseError<usize, Token<'input>, &'static str>;

impl<'input> From<LalrpopError<'input>> for Error {
    fn from(error: LalrpopError<'input>) -> Self {
        match error {
            ParseError::InvalidToken { location } => Error::InvalidToken {
                location: Location::offset(location),
            },
            ParseError::UnrecognizedEof { location, expected } => Error::ExpectedToken { location: Location::new(offset), expected: () }
            _ => todo!(),
        }
    }
}
