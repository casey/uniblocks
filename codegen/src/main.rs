#![recursion_limit="128"]

#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quote;
extern crate proc_macro2;
extern crate regex;
extern crate syn;

mod blocks;
mod block;
mod code;
mod parse_error;

use failure::Error;
use std::fs;

fn main() -> Result<(), Error> {
  let blocks = fs::read_to_string("../Blocks.txt")?.parse()?;
  println!("{}", code::tokens(blocks));
  Ok(())
}
