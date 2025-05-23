use crate::parser::Parser;
//this is a vehicle to implement the parser trait
//I'm already using tuples to handle sequential
//parsing.

//it's important to remember that order matters with this
//in the case of an ambiguous parse the first one that matches
//will always win, so for example if you were trying to parse
//something where you can match either "alf" as a keyword
//or just any random word, you have to try "alf" as a keyword
//first!
pub struct Alt<I,P> {
  ps:P,
  _ghost:std::marker::PhantomData<I>
}

impl<I,P> Alt<I,P> {
  pub const fn new(ps:P) -> Self {
    Self{ps,_ghost:std::marker::PhantomData }
  }
}

macro_rules! alt_parser_impl {

  ($P:ident) => {
  };
  ($First:ident, $($Rest:ident),+) => {
    alt_parser_impl!(__frfr; $First, $($Rest),+);
    alt_parser_impl!($($Rest),+);
  };
  (__frfr; $($PARSER:ident),+) => {
    impl<IN:Clone, OUT, $($PARSER),+> From<($($PARSER),+)> for Alt<IN,($($PARSER,)+)> 
    where $($PARSER:Parser<IN,Out=OUT>,)+
    {
      fn from(ps:($($PARSER),+)) -> Self {
        Self {
          ps,
          _ghost:std::marker::PhantomData
        }
      }
    }

    #[allow(non_snake_case)] //you are gonna re-use generic names as variable names
    impl<IN:Clone, OUT, $($PARSER),+ > Parser<IN> for Alt<IN,($($PARSER,)+)>
    where $($PARSER:Parser<IN,Out=OUT>,)+
    {
      type Error=($($PARSER::Error),+);
      type Out = OUT;

      fn parse(&self,txt:IN)->Result<(OUT,IN),Self::Error> {
        let Alt{ ps:($($PARSER),+),.. } = self;

        //this works cause let A = match A will get the variables right
        $( let $PARSER = match $PARSER.parse(txt.clone()) { 
          Ok((v,r)) => return Ok((v,r)),
          Err(e) => e
        };)+

        Err(($($PARSER),+))
      }
    }
  }
}

//we could do some macro recursion to get rid of these pyramids
//but I think this makes the macro itself easier to read
alt_parser_impl!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z);

//this is a convenience method to make constructing an alt easier
// the into trait ends up just being kinda helpful as it's safer to
// use the const new
pub const fn alt<I,P:Into<Alt<I,P>>>(ps:P) -> Alt<I,P> {
  Alt::new(ps)
}

#[cfg(test)]
mod tests {
  use super::*;

  //some parsers
  fn dog(inp:&str) -> Result<(&str,&str),&'static str> {
    match &inp[0..3] {
      "dog" => Ok((&inp[0..3],&inp[3..])),
      _ => Err("oh no")  
    }
  }

  fn cat(inp:&str) -> Result<(&str,&str),usize> {
    match &inp[0..3] {
      "cat" => Ok((&inp[0..3],&inp[3..])),
      _ => Err(15)
    }
  }

  fn fish(inp:&str) -> Result<(&str,&str),bool> {
    match &inp[0..4] {
      "fish" => Ok((&inp[0..4],&inp[4..])),
      _ => Err(false)
    }
  }

  #[test]
  fn test_alts() {
    let z = alt((dog,cat,fish));

    let (out,res) = z.parse("dogzone").expect("the dog parser should succeed");
    assert_eq!("dog",out);
    assert_eq!("zone",res);

    let (out,res) = z.parse("fisherman").expect("the fish parser should succeed");
    assert_eq!("fish",out);
    assert_eq!("erman",res);

    let (out,res) = z.parse("catseldorf").expect("the cat parser should succeed");
    assert_eq!("cat",out);
    assert_eq!("seldorf",res);

    let bad = z.parse("if you can't hang with the big dog, get off the porch")
    .expect_err("this should error all over the place");
    assert!(matches!(bad,("oh no",15,false)));
  }
}
