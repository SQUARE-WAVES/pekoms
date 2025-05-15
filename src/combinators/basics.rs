use crate::Parser;

//this is a parser that will always succeed and return an optional value
//it's handy for certain operations
pub const fn optional<I,P>(p:P) -> impl Parser<I,Out=Option<P::Out>,Error=P::Error> 
where
  I:Clone,
  P:Parser<I>
{
  move |txt:I| -> Result<(Option<P::Out>,I),P::Error> {
    match p.parse(txt.clone()) {
      Ok((v,r)) => Ok((Some(v),r)),
      Err(..) => Ok((None,txt))
    }
  }
}

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

  #[test]
  fn test_optional() {

    let input = ".";
    let opt_dot = optional(dot);
    let opt_dash = optional(dash);
    
    let res1 = opt_dot.parse(input);
    match res1 {
      Ok((Some(v),t)) => {
        assert_eq!(v,".","the correct value should be in the option");
        assert_eq!(t,"","the correct remainig text should be returned");
      },
      _ => panic!("res1 should be a success")
    };

    let res2 = opt_dash.parse(input);
    match res2 {
      Ok((None,t)) => {
        assert_eq!(t,input,"the unaltered input should return on a fail") 
      },
      _ => panic!("res2 should be a None")
    }
  }
}
