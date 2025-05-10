use crate::Parser;

//this is a parser that will always succeed and return an optional value
//it's handy for certain operations
pub const fn optional<I:Clone,O,P:Parser<I,O>>(p:P) -> impl Parser<I,Option<O>,Error=P::Error> {
  move |txt:I| -> Result<(Option<O>,I),P::Error>{
    match p.parse(txt.clone()) {
      Ok((v,r)) => Ok((Some(v),r)),
      Err(..) => Ok((None,txt))
    }
  }
}

//this is a parser that looks at the input and gives you the output without consuming the input
//this lets you do dirty tricks with context if you want or other things like that
pub const fn peek<I:Clone,O,P:Parser<I,O>>(p:P) -> impl Parser<I,O,Error=P::Error> {
  move |txt:I| -> Result<(O,I),P::Error>{
    match p.parse(txt.clone()) {
      Ok((v,_r)) => Ok((v,txt)),
      Err(e) => Err(e)
    }
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  //some little parsers
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

  #[test]
  fn test_peek() {
    let input = ".-";
    let peek_dot = peek(dot);
    let peek_dash = peek(dash);

    let res1 = peek_dot.parse(input);

    match res1 {
      Ok((v,t)) => {
        assert_eq!(v,".","the correct value should be in the option");
        assert_eq!(t,input,"the remainig text should be the same as the input");
      },
      _ => panic!("res1 should be a success")
    };

    let res2 = peek_dash.parse(input);
    assert!(res2.is_err(),"res2 should be an error");
  }
}
