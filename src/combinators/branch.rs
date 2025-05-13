use crate::Parser;

//this combinator is a compromise with alt
//alt tries to run each parser to the end, and succeeds
//the instant it gets one that makes it all the way
//branch here will try and run the first one it sees
//that doesn't give it a specific error message
//and if it doesn't finish that it will send back
//the error it got. It's useful when you are parsing
//big unambiguous sequences.
//
//for example, in json a value in an object can be a string
//or some other things, but if it starts with " that means
//it's got to be a string, if it starts with " and then
//somewhere down the line failes to find the end " or something
//there is no point in trying to see if it's a list or an object

trait Segment<E1,E2> 
{
  fn cont(&self,e:&E1) -> bool;
  fn err_map(&self,e:E1) -> E2;
}

pub struct SegParse<E,I,O,P,F1,F2> {
  parser:P,
  f1:F1,
  f2:F2,
  _ghost:std::marker::PhantomData<(I,O,E)>
}

impl<E,I,O,P,F1,F2> Segment<P::Error,E> for SegParse<E,I,O,P,F1,F2> 
where
  P:Parser<I,O>,
  F1:Fn(&P::Error) -> bool,
  F2:Fn(P::Error) -> E
{
  fn cont(&self,e:&P::Error) -> bool {
    (self.f1)(e)
  }

  fn err_map(&self,e:P::Error) -> E {
    (self.f2)(e)
  }
}

impl<E,I,O,P,F1,F2> Parser<I,O> for SegParse<E,I,O,P,F1,F2> 
where
  P:Parser<I,O>,
{
  type Error=P::Error;
  fn parse(&self,txt:I) -> Result<(O,I),Self::Error> {
    self.parser.parse(txt)
  }
}

pub struct Branch<E,I,O,P> {
  segments:P,
  _ghost:std::marker::PhantomData<(I,O,E)>
}

impl<E,I,O,P> Branch<E,I,O,P> {
  pub const fn new(segments:P) -> Self {
    Self{segments,_ghost:std::marker::PhantomData}
  }
}


macro_rules! branch_parser_impl {
  ($TYPE:ident; $($PARSER:ident),+) => {
    impl<ER,IN, $TYPE, $($PARSER),+> From<($($PARSER),+)> for Branch<ER,IN,$TYPE,($($PARSER,)+)>
    where $( $PARSER:Parser<IN,$TYPE> + Segment<$PARSER::Error,ER>, )+
    {
      fn from(segs:($($PARSER),+)) -> Self{
        Self::new(segs)
      }
    }

    #[allow(non_snake_case)] //you are gonna re-use generic names as variable names
    impl<ER,IN,$TYPE,$($PARSER),+> Parser<IN,$TYPE> for Branch<ER,IN,$TYPE,($($PARSER),+)>
    where
      IN:Clone,
      $( $PARSER:Parser<IN,$TYPE> + Segment<$PARSER::Error,ER>, )+
    {
      type Error=Option<ER>;

      fn parse(&self,txt:IN) -> Result<($TYPE,IN),Self::Error> {
        let Branch{ segments:($($PARSER),+),.. } = self;

        $( match $PARSER.parse(txt.clone()) {
          Ok(v) => return Ok(v),
          Err(e) => {
            if !$PARSER.cont(&e) {
              return Err(Some($PARSER.err_map(e)))
            }
          }
        };)+

        Err(None)
      }
    }
  }
}

branch_parser_impl!(Typ; A,B);
branch_parser_impl!(Typ; A,B,C);
branch_parser_impl!(Typ; A,B,C,D);
branch_parser_impl!(Typ; A,B,C,D,E);
branch_parser_impl!(Typ; A,B,C,D,E,F);
branch_parser_impl!(Typ; A,B,C,D,E,F,G);
branch_parser_impl!(Typ; A,B,C,D,E,F,G,H);
branch_parser_impl!(Typ; A,B,C,D,E,F,G,H,I);
branch_parser_impl!(Typ; A,B,C,D,E,F,G,H,I,J);
branch_parser_impl!(Typ; A,B,C,D,E,F,G,H,I,J,K);
branch_parser_impl!(Typ; A,B,C,D,E,F,G,H,I,J,K,L);
branch_parser_impl!(Typ; A,B,C,D,E,F,G,H,I,J,K,L,M);


//a nicer export
pub fn seg<E,I,O,P,F1,F2>(p:P,f1:F1,f2:F2) -> SegParse<E,I,O,P,F1,F2>
where
  P:Parser<I,O>,
  F1:Fn(&P::Error) -> bool,
  F2:Fn(P::Error) -> E
{
  SegParse{parser:p,f1,f2,_ghost:std::marker::PhantomData}
}

pub const fn branch<E,I,O,P:Into<Branch<E,I,O,P>>>(segs:P) -> Branch<E,I,O,P> {
  Branch::new(segs)
}

#[cfg(test)]
mod tests {
  use super::*;
  //some parsers
  fn dot(inp:&str) -> Result<(&str,&str),usize> {
    inp.strip_prefix(".").map(|r|(".",r)).ok_or(1)
  }

  fn dash(inp:&str) -> Result<(&str,&str),usize> {
    inp.strip_prefix("-").map(|r|("-",r)).ok_or(2)
  }

  fn space(inp:&str) -> Result<(&str,&str),usize> {
    inp.strip_prefix(" ").map(|r|(" ",r)).ok_or(3)
  }


  #[test]
  fn test_branch() {
    let seq1 = (dot,dash,dot).map(|_|1);
    let seq2 = (dash,dot,dash,dot,dot).map(|_|2);
    let seq3 = (space,dot,dash,dot,dot).map(|_|3);

    let seg1 = seg(seq1,|v|*v==1,|_|"seq1");
    let seg2 = seg(seq2,|v|*v==2,|_|"seq2");
    let seg3 = seg(seq3,|v|*v==3,|_|"seq3");

    let b = branch((seg1,seg2,seg3));

    let (good1,_) = b.parse(".-.").expect("1st seq shouldn't fail");
    assert_eq!(good1,1);

    let (good2,_) = b.parse("-.-..").expect("2nd seq shouldn't fail");
    assert_eq!(good2,2);

    let (good3,_) = b.parse(" .-..").expect("3rd seq shouldn't fail");
    assert_eq!(good3,3);

    let bad1 = b.parse(". .").expect_err("the first segment should fail");
    assert!(matches!(bad1,Some("seq1")));

    let bad2 = b.parse("- .").expect_err("the 2nd segment should fail");
    assert!(matches!(bad2,Some("seq2")));

    let bad3 = b.parse(" - ").expect_err("the 3nd segment should fail");
    assert!(matches!(bad3, Some("seq3")));

    let bad4 = b.parse("#asdf").expect_err("all segments should fail");
    assert!(bad4.is_none());
    
  }
}
