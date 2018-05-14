use regex::Regex;
use std::str::FromStr;
use syn::Ident;

use parse_error::ParseError;

#[derive(Debug)]
pub struct Block {
  pub name:            String,
  pub first_codepoint: u32,
  pub last_codepoint:  u32,
}

impl Block {
  pub fn variant(&self) -> Ident {
    self.name.chars()
      .filter(|&c| c != ' ' && c != '-')
      .collect::<String>()
      .into()
  }
}

impl FromStr for Block {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref LINE_RE: Regex = Regex::new(r#"(?x)
        (?P<first> [[:xdigit:]]{4,6}) # first codepoint in block
        [.]{2}                        # ".."
        (?P<last>  [[:xdigit:]]{4,6}) # last codepoint in block
        ;\x20                         # "; "
        (?P<name>  [a-zA-Z0-9\x20-]+) # block name
        $                             # end of line
      "#).unwrap();
    }

    let captures = LINE_RE.captures(s).ok_or_else(|| ParseError::Line{line: s.to_string()})?;

    let name = captures["name"].to_string();

    let first_codepoint = u32::from_str_radix(&captures["first"], 16)
      .map_err(|parse_int_error| ParseError::Codepoint{parse_int_error})?;

    let last_codepoint = u32::from_str_radix(&captures["last"], 16)
      .map_err(|parse_int_error| ParseError::Codepoint{parse_int_error})?;

    Ok(Block{name, first_codepoint, last_codepoint})
  }
}

