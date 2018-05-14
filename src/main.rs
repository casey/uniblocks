use std::char;
use std::fmt::{self, Formatter, Display};

mod block;

struct Codepoint(u32);

impl Display for Codepoint {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "U+{:04X}", self.0)
  }
}

fn main() -> Result<(), std::io::Error> {
  for block in block::Block::all() {
    println!("{} – {}", block.name(), block.chart());
    if block.surrogates() || block.boring() {
      println!("{}–{}", Codepoint(block.first_codepoint()), Codepoint(block.last_codepoint()));
      continue;
    }

    for codepoint in block.first_codepoint()..=block.last_codepoint() {
      if codepoint % 16 == 0 {
        print!("U+{:04X}x ", codepoint); // fix this
      }
      let c = char::from_u32(codepoint).unwrap();
      if c.is_control() || c == ' ' {
        if codepoint <= 0x0020 {
          print!("{}", char::from_u32(codepoint + 0x2400).unwrap());
        } else if codepoint == 0x007F {
          print!("\u{2421}");
        } else {
          print!("�");
        }
      } else {
        print!("{}", c);
      }
      if codepoint % 16 == 15 {
        println!("");
      } else {
        print!(" ");
      }
    }
    println!();
  }

  // print name of block
  // print each hex range U+000Fx
  // print characters in block (if any characters are double width, all should be
  // pretty box drawing characters (can turn these off)
  // flags to conrol:
  // - print or don't print hex at leading
  // - print or don't print spaces between characters
  // - print or don't print newlines
  // - sub or don't sub missing character symbol
  // (should be able to get pure bytes of all characters)
  // print block(s) by regex
  // print blocks that contain character(s)
  // print into different files
  // print or don't print names, orint or don't print link to chart
  // add uniblocks.txt in root of repo
  // add a nice block to readme
  // html output format

  Ok(())
}
