use crate::parser::Parser;
use crate::err::AltErr;
// OK all the type signatures in here are really annoying to read
// and that's cause of errors.
// the issue is that I don't enforce a single parser error type
// so if you want an alternative selection from multiple parsers
// it's type has to include all the different error types
// now I guess I could make it easier by forcing you to have the same error
// just like I force you to have the same return type, but I don't want to do
// that cause that's annoying. It's much easier to make returns conform via
// map and such like

pub struct Alt<I,O,P> {
  ps:P,
  _ghost:std::marker::PhantomData<(I,O)>
}

macro_rules! alt_parser_impl {
  ($TG:ident, $($MG:ident),+) => {
    impl<IN:Clone, $TG, $($MG),+> From<($($MG),+)> for Alt<IN,$TG,($($MG,)+)> 
    where $($MG:Parser<IN,$TG>,)+
    {
      fn from(ps:($($MG),+)) -> Self {
        Self {
          ps,
          _ghost:std::marker::PhantomData
        }
      }
    }

    #[allow(non_snake_case)] //you are gonna re-use generic names as variable names
    impl<IN:Clone, $TG, $($MG),+ > Parser<IN,$TG> for Alt<IN,$TG,($($MG,)+)>
    where $($MG:Parser<IN,$TG>,)+
    {
      type Error=AltErr;

      fn parse(&self,txt:IN)->Result<($TG,IN),AltErr> {
        let Alt{ ps:($($MG),+),.. } = self;

        $( if let Ok((v,r)) = $MG.parse(txt.clone()) { return Ok((v,r)); })+
        Err(AltErr{})
      }
    }
  }
}

alt_parser_impl!(Typ, A,B);
alt_parser_impl!(Typ, A,B,C);
alt_parser_impl!(Typ, A,B,C,D);
alt_parser_impl!(Typ, A,B,C,D,E);
alt_parser_impl!(Typ, A,B,C,D,E,F);
alt_parser_impl!(Typ, A,B,C,D,E,F,G);
alt_parser_impl!(Typ, A,B,C,D,E,F,G,H);
alt_parser_impl!(Typ, A,B,C,D,E,F,G,H,I);
alt_parser_impl!(Typ, A,B,C,D,E,F,G,H,I,J);
alt_parser_impl!(Typ, A,B,C,D,E,F,G,H,I,J,K);
alt_parser_impl!(Typ, A,B,C,D,E,F,G,H,I,J,K,L);
alt_parser_impl!(Typ, A,B,C,D,E,F,G,H,I,J,K,L,M);

//this is a convenience method to make constructing an alt easier
pub fn alt<I,O,P:Into<Alt<I,O,P>>>(ps:P) -> Alt<I,O,P> {
  ps.into()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::err::ErrorMsg;

  //some parsers
  fn dog(inp:&str) -> Result<(&str,&str),ErrorMsg> {
    match &inp[0..3] {
      "dog" => Ok((&inp[0..3],&inp[3..])),
      _ => Err("oh no it's all bad".into())  
    }
  }

  fn cat(inp:&str) -> Result<(&str,&str),ErrorMsg> {
    match &inp[0..3] {
      "cat" => Ok((&inp[0..3],&inp[3..])),
      _ => Err("oh no its all bad".into())
    }
  }

  fn fish(inp:&str) -> Result<(&str,&str),ErrorMsg> {
    match &inp[0..4] {
      "fish" => Ok((&inp[0..4],&inp[4..])),
      _ => Err("oh no its all bad".into())
    }
  }

  #[test]
  fn test_alts() {
    let z = alt((dog,cat,fish)).map_err(|_| -> ErrorMsg { 
      "it needs to be a dog, a cat, or a fish".into() 
    });

    let (out,res) = z.parse("dogzone").expect("the dog parser should succeed");
    assert_eq!("dog",out);
    assert_eq!("zone",res);

    let (out,res) = z.parse("fisherman").expect("the fish parser should succeed");
    assert_eq!("fish",out);
    assert_eq!("erman",res);

    let (out,res) = z.parse("catseldorf").expect("the cat parser should succeed");
    assert_eq!("cat",out);
    assert_eq!("seldorf",res);

    let bad = z.parse("if you can't hang with the big dog, get off the porch");
    assert!(bad.is_err());
  }
}
