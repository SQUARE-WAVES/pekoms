use crate::parser::Parser;
/*=============================================================================
This macro implemnts the parser trait for tuples of parsers, and makes them run in sequence,
so for example if you have the parser "word" which matches a bunch of letters
and the parser "comma" which matches a comma, the tuple (word,comma) would be
a parser that matches a word and then a comma
=============================================================================*/
macro_rules! sequential_parser_impl{
  ($($TypGen:ident $MchGen:ident),+) => {
    //since the generic types names aren't snake cased you need this to avoid a million warnings
    #[allow(non_snake_case)]
    impl<Inp:Clone, $($TypGen),+, $($MchGen),+> Parser<Inp,($($TypGen),+)> for ($($MchGen,)+)
    where $($MchGen:Parser<Inp,$TypGen>,)+
    {
      fn parse(&self,txt:Inp)->Option<(($($TypGen),+),Inp)> {
        let ($($MchGen),+) = self;
        $(let ($TypGen,txt) = $MchGen.parse(txt)?;)+
        Some((($($TypGen),+),txt))
      }
    }
  }
}

sequential_parser_impl!(At A,Bt B);
sequential_parser_impl!(At A,Bt B,Ct C);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D,Et E);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D,Et E, Ft F);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D,Et E, Ft F, Gt G);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D,Et E, Ft F, Gt G, Ht H);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D,Et E, Ft F, Gt G, Ht H, It I);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D,Et E, Ft F, Gt G, Ht H, It I, Jt J);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D,Et E, Ft F, Gt G, Ht H, It I, Jt J, Kt K);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D,Et E, Ft F, Gt G, Ht H, It I, Jt J, Kt K, Lt L);
sequential_parser_impl!(At A,Bt B,Ct C,Dt D,Et E, Ft F, Gt G, Ht H, It I, Jt J, Kt K, Lt L, Mt M);


#[cfg(test)]
mod tests
{
  use super::*;

  fn dot(inp:&str) -> Option<(&str,&str)> {
    inp.strip_prefix(".").map(|r|(".",r))
  }

  fn dash(inp:&str) -> Option<(&str,&str)> {
    inp.strip_prefix("-").map(|r|("-",r))
  }

  fn space(inp:&str) -> Option<(&str,&str)> {
    inp.strip_prefix(" ").map(|r|(" ",r))
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
    assert!(bad.is_none(),"the s parser should fail if there is another letter in there");


    let bad = morse_s.parse("..");
    assert!(bad.is_none(),"a sequential parser should fail if there isn't enough input");

    let (out,res) = morse_o.parse("--- HEY!").expect("O shouldn't fail to parse");
    assert_eq!(("-","-","-"),out);
    assert_eq!(" HEY!",res);
    
    let bad = morse_o.parse(".---");
    assert!(bad.is_none());

    let (out,res) = morse_sos.parse("... --- ...").expect("SOS shouldn't fail to parse");
    assert_eq!(((".",".",".")," ",("-","-","-")," ",(".",".",".")),out);
    assert_eq!("",res);

    let bad = morse_sos.parse("...---...");
    assert!(bad.is_none());
  }
}
