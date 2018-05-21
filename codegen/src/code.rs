use quote::Tokens;
use syn::{Ident, LitInt, IntSuffix};
use proc_macro2::Span;

use block::Block;
use blocks::Blocks;

pub fn tokens(blocks: Blocks) -> Tokens {
  let variants = &(&blocks)
    .into_iter()
    .map(Block::variant)
    .collect::<Vec<Ident>>();

  let first_codepoints = (&blocks)
    .into_iter()
    .map(|block| LitInt::new(block.first_codepoint as u64, IntSuffix::None, Span::call_site()));

  let last_codepoints = (&blocks)
    .into_iter()
    .map(|block| LitInt::new(block.last_codepoint as u64, IntSuffix::None, Span::call_site()));

  let names = (&blocks)
    .into_iter()
    .map(|block| block.name.as_str());

  quote! {
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Block {
      #(#variants,)*
    }

    impl Block {
      pub fn first_codepoint(self) -> u32 {
        use self::Block::*;
        match self {
          #(#variants => #first_codepoints,)*
        }
      }

      pub fn last_codepoint(self) -> u32 {
        use self::Block::*;
        match self {
          #(#variants => #last_codepoints,)*
        }
      }

      pub fn contains_char(self, c: char) -> bool {
        let codepoint = c as u32;

        codepoint >= self.first_codepoint() && codepoint <= self.last_codepoint()
      }

      pub fn chart(self) -> String {
        format!("https://www.unicode.org/charts/PDF/U{:04X}.pdf", self.first_codepoint())
      }

      pub fn name(self) -> &'static str {
        use self::Block::*;
        match self {
          #(#variants => #names,)*
        }
      }

      pub fn surrogates(self) -> bool {
        use self::Block::*;
        match self {
          HighSurrogates |
          HighPrivateUseSurrogates |
          LowSurrogates => true,
          _ => false,
        }
      }

      pub fn boring(self) -> bool {
        use self::Block::*;
        match self {
          Tags |
          VariationSelectorsSupplement |
          PrivateUseArea |
          SupplementaryPrivateUseAreaA |
          SupplementaryPrivateUseAreaB => true,
          _ => false,
        }
      }

      pub fn all() -> &'static [Block] {
        use self::Block::*;
        &[
          #(#variants,)*
        ]
      }
    }
  }
}
