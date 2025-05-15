use crate::parser::Parser;

/*=============================================================================
This macro implemnts the parser trait for tuples of parsers, and makes them run in sequence,
so for example if you have the parser "word" which matches a bunch of letters
and the parser "comma" which matches a comma, the tuple (word,comma) would be
a parser that matches a word and then a comma
=============================================================================*/
macro_rules! sequential_parser_impl {
  ($First:ident) => {
    #[allow(non_snake_case)]
    impl<Inp,$First> Parser<Inp> for ($First, )
    where 
      $First:Parser<Inp>
    {
      type Error= $First::Error;
      type Out = $First::Out;

      fn parse(&self,txt:Inp)->Result<(Self::Out,Inp),Self::Error> {
        self.0.parse(txt)
      }
    }
  };

  ($First:ident, $($Rest:ident),+) => {
    sequential_parser_impl!(__frfr; $First ,$($Rest),+);
    sequential_parser_impl!($($Rest),+);
  };

  (__frfr; $($Parser:ident),+) => {
    //since the generic types names aren't snake cased you need this to avoid a million warnings
    #[allow(non_snake_case)]
    impl<Inp, Er, $($Parser),+> Parser<Inp> for ($($Parser,)+)
    where 
      $($Parser:Parser<Inp,Error=Er>,)+
    {
      type Error= Er;
      type Out = ($($Parser::Out),+);

      fn parse(&self,txt:Inp)->Result<(Self::Out,Inp),Self::Error> {
        let ($($Parser),+) = self;
        $(let ($Parser,txt) = $Parser.parse(txt)?;)+
        Ok((($($Parser),+),txt))
      }
    }
  }
}


sequential_parser_impl!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z);

#[cfg(test)]
mod tests
{
  use super::*;

  fn dot(inp:&str) -> Result<(&str,&str),()> {
    inp.strip_prefix(".").map(|r|(".",r)).ok_or(())
  }

  fn dash(inp:&str) -> Result<(&str,&str),()> {
    inp.strip_prefix("-").map(|r|("-",r)).ok_or(())
  }

  fn space(inp:&str) -> Result<(&str,&str),()> {
    inp.strip_prefix(" ").map(|r|(" ",r)).ok_or(())
  }

  #[test]
  fn seq_parser() {
    let morse_s = (dot,dot,dot);
    let morse_o = (dash,dash,dash);
    let morse_sos = (morse_s,space,morse_o,space,morse_s);
    
    let (out,res) = morse_s.parse("...").expect("S shouldn't fail to parse");
    assert_eq!((".",".","."),out);
    assert_eq!("",res);

    let bad = morse_s.parse("..F.");
    assert!(bad.is_err(),"the s parser should fail if there is another letter in there");


    let bad = morse_s.parse("..");
    assert!(bad.is_err(),"a sequential parser should fail if there isn't enough input");

    let (out,res) = morse_o.parse("--- HEY!").expect("O shouldn't fail to parse");
    assert_eq!(("-","-","-"),out);
    assert_eq!(" HEY!",res);
    
    let bad = morse_o.parse(".---");
    assert!(bad.is_err());

    let (out,res) = morse_sos.parse("... --- ...").expect("SOS shouldn't fail to parse");
    assert_eq!(((".",".",".")," ",("-","-","-")," ",(".",".",".")),out);
    assert_eq!("",res);

    let bad = morse_sos.parse("...---...");
    assert!(bad.is_err());
  }
}
