use std::str::FromStr;
use std::{vec, slice};

use parse_error::ParseError;
use block::Block;

#[derive(Debug)]
pub struct Blocks {
  blocks: Vec<Block>,
}

impl<'a> FromStr for Blocks {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let blocks = s.lines()
     .filter(|line| !line.starts_with('#'))
     .filter(|line| !line.is_empty())
     .map(|line| line.parse())
     .collect::<Result<Vec<Block>, ParseError>>()?;

    Ok(Blocks{blocks})
  }
}

impl IntoIterator for Blocks {
    type Item = Block;
    type IntoIter = vec::IntoIter<Block>;

    fn into_iter(self) -> Self::IntoIter {
      self.blocks.into_iter()
    }
}

impl<'a> IntoIterator for &'a Blocks {
    type Item = &'a Block;
    type IntoIter = slice::Iter<'a, Block>;

    fn into_iter(self) -> slice::Iter<'a, Block> { 
      (&self.blocks).into_iter()
    }
}
