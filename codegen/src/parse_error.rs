use std::num::ParseIntError;

#[derive(Debug, Fail)]
pub enum ParseError {
  #[fail(display = "bad block line: {}", line)]
  Line{line: String},
  #[fail(display = "bad codepoint hex: {}", parse_int_error)]
  Codepoint{parse_int_error: ParseIntError},
}

