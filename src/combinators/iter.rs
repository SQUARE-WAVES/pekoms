use crate::Parser;

type Pres<I,O,P> = Result<(O,I),<P as Parser<I>>::Error>;

pub const fn accum<I,P,F>(p:P,d:P::Out,mut a:F) -> impl FnOnce(I) -> Pres<I,(),P>
where
  P:Parser<I> + Clone,
  P::Out:PartialEq,
  F:FnMut(P::Out),
{
  move|mut inp| {
    loop {
      let (o,rest) = p.clone().parse(inp)?;
      if o == d {
        break Ok(((),rest))
      }
      else {
        a(o)
      }

      inp = rest;
    }
  }
}


pub const fn sep_list<I,P1,P2,F>(p:P1,sep:P2,mut a:F) -> impl FnOnce(I) -> Pres<I,(),P1> 
where
  I:Clone,
  P1:Parser<I> + Clone,
  P2:Parser<I> + Clone,
  F:FnMut(P1::Out)
{
  move |mut inp| {
    let rem = loop{
      inp = match p.clone().parse(inp.clone()) {
        Ok((o,r)) => {
          a(o);
          r
        },
        Err(_) => break inp,
      };

      inp = match sep.clone().parse(inp.clone()) {
        Ok((_,r)) => r,
        Err(_) => break inp
      };
    };

    Ok(((),rem))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn dig(s:&str) -> Result<(Option<isize>,&str),()> {
    if s.is_empty() {
      Ok((None,s))
    }
    else {
      match &s[0..1].parse::<isize>() {
        Ok(n) => Ok((Some(*n),&s[1..])),
        Err(_) => Err(())
      }
    }
  }

  fn ltr(s:&str) -> Result<((),&str),()> {
    s.strip_prefix(|c:char|c.is_alphabetic()).map(|r|((),r)).ok_or(())
  }

  fn comma(s:&str) -> Result<(&str,&str),()> {
    s.strip_prefix(",").map(|r|(",",r)).ok_or(())
  }

  #[test]
  fn test_accum() {
    let mut val = 0;
    
    let (_,s) = accum(dig,None,|v|val += v.unwrap())
    .parse("123456789").expect("it shouldn't error");

    assert!(s.is_empty(),"string should be used up");
    assert_eq!(val,45);

    let mut val = 0;
    accum(dig,None,|v|val += v.unwrap()).parse("12345%67").expect_err("it should error");
  }

  #[test]
  fn test_sep_list() {
    let mut val = 0;
    let words = "q,f,p,n";

    sep_list(ltr,comma,|_|val+=1)(words).expect("should work fine");
    assert_eq!(val,4);

    let mut val = 0;
    let words = "a,b,c,";

    sep_list(ltr,comma,|_|val+=1)(words).expect("should work fine");
    assert_eq!(val,3);

    let mut val = 0;
    let words = "aa,b,c";

    let (_,rest) = sep_list(ltr,comma,|_|val+=1)(words).expect("should work fine");
    assert_eq!(val,1);
    assert_eq!(rest,&words[1..]);

    let mut val = 0;
    let words = "a,b c";

    let (_,rest) = sep_list(ltr,comma,|_|val+=1)(words).expect("should work fine");
    assert_eq!(val,2);
    assert_eq!(rest," c");
  }
}

