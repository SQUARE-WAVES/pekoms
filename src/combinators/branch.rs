use crate::Parser;

pub struct Branch<I,ER,P> {
  ps:P,
  _ghost:std::marker::PhantomData<(I,ER)>
}

impl<I,ER,P> Branch<I,ER,P> {
  pub const fn new(ps:P) -> Self {
    Self{ps,_ghost:std::marker::PhantomData }
  }
}

macro_rules! branch_impl {
  ($P:ident) => {
  };
  ($First:ident, $($Rest:ident),+) => {
    branch_impl!(__frfr; $First, $($Rest),+);
    branch_impl!($($Rest),+);
  };
  (__frfr; $($Seg:ident),+) => {
    impl<IN:Clone, OUT,ER, $($Seg),+> From<($($Seg),+)> for Branch<IN,ER,($($Seg,)+)> 
    where $($Seg:Parser<IN,Out=OUT,Error=Option<ER>>,)+
    {
      fn from(ps:($($Seg),+)) -> Self {
        Self {
          ps,
          _ghost:std::marker::PhantomData
        }
      }
    }

    #[allow(non_snake_case)] //you are gonna re-use generic names as variable names
    impl<IN:Clone, OUT,ER, $($Seg),+ > Parser<IN> for Branch<IN,ER,($($Seg,)+)>
    where $($Seg:Parser<IN,Out=OUT,Error=Option<ER>>),+
    {
      type Error= Option<ER>;
      type Out = OUT;

      fn parse(&self,txt:IN)->Result<(OUT,IN),Self::Error> {
        let Branch{ ps:($($Seg),+),.. } = self;

        //this works cause let A = match A will get the variables right
        $(
          match $Seg.parse(txt.clone()) { 
            Ok((v,r)) => return Ok((v,r)),
            Err(None) => (),
            Err(Some(e)) => return Err(Some(e))
          };
        )+

        Err(None)
      }
    }

  }
}

branch_impl!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z);

pub const fn branch<I,E,P:Into<Branch<I,E,P>>>(ps:P) -> Branch<I,E,P> {
  Branch::new(ps)
}

#[cfg(test)]
mod test {
  use super::*;

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
    let seq1 = (dot,dash,dot).map(|_|1).map_err(|e| Some(e).take_if(|e|*e != 1).map(|_|"seq1"));
    let seq2 = (dash,dot,dash,dot,dot).map(|_|2).map_err(|e|Some(e).take_if(|e|*e!=2).map(|_|"seq2"));
    let seq3 = (space,dot,dash,dot,dot).map(|_|3).map_err(|e|Some(e).take_if(|e|*e!=3).map(|_|"seq3"));

    let b = branch((seq1,seq2,seq3));
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
