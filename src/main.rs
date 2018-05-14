extern crate regex;

use regex::Regex;
use std::fs;
use std::char;

#[derive(Debug)]
struct Block {
  first: u32,
  last:  u32,
  name:  String,
}

// #[derive(Clone, Copy)]
// enum BlockEnum {
//   Name,
// }

// impl Block {
//   fn first(self) -> u32 {
//   }

//   fn last(self) -> u32 {
//   }

//   fn chart(self) -> &str {
//   }
//
//   fn name(self) -> &str {
//    // full display name
//   }

//   fn surrogates(self) -> bool {
//   }

//   fn boring(self) -> bool {
//   }
// }

// struct Character(char);

// impl display for Character

fn main() -> Result<(), std::io::Error> {
  let line_re = Regex::new(r#"(?x)
    (?P<first> [[:xdigit:]]{4,6}) # first codepoint in block
    [.]{2}                        # ".."
    (?P<last>  [[:xdigit:]]{4,6}) # last codepoint in block
    ;\x20                         # "; "
    (?P<name>  [a-zA-Z0-9\x20-]+) # block name
    $                             # end of line
  "#).unwrap();

  let blocks_txt = fs::read_to_string("dat/Blocks.txt")?;

  let mut blocks = Vec::new();

  for line in blocks_txt.lines() {
    if line.starts_with('#') || line.is_empty() {
      continue;
    }

    let captures = line_re.captures(line).unwrap();

    blocks.push(Block {
      first: u32::from_str_radix(&captures["first"], 16).unwrap(),
      last:  u32::from_str_radix(&captures["last"],  16).unwrap(),
      name:  captures["name"].to_string(),
    });
  }

  for block in blocks {
    println!("{} – https://www.unicode.org/charts/PDF/U{:04X}.pdf", block.name, block.first);
    if block.name == "High Surrogates"                  ||
       block.name == "High Private Use Surrogates"      ||
       block.name == "Low Surrogates"                   ||
       block.name == "Tags"                             || // weird
       block.name == "Variation Selectors Supplement"   || // boring
       block.name == "Supplementary Private Use Area-A" || // empty
       block.name == "Supplementary Private Use Area-B"    // empty
    {
      println!("U+{:04X}–U+{:04X}", block.first, block.last);
      continue;
    }

    for codepoint in block.first..=block.last {
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

  Ok(())
}
