use crate::Parser;

//this sucks but you need to do newtypes to do traits good
pub struct OptPars<P,I> {
  pars:P,
  _ghost:std::marker::PhantomData<I>
}

impl<I:Clone,P:Parser<I>> Parser<I> for OptPars<P,I> {
  type Out=Option<P::Out>;
  type Error=P::Error;

  fn parse(self,input:I) -> Result<(Self::Out,I),Self::Error> {
    match self.pars.parse(input.clone()) {
      Ok((v,r)) => Ok((Some(v),r)),
      Err(..) => Ok((None,input))
    }
  }
}

impl<I:Clone,P:Clone> Clone for OptPars<P,I> {
  fn clone(&self) -> Self {
    OptPars{
      pars:self.pars.clone(),
      _ghost:std::marker::PhantomData
    }
  }
}


//this is a parser that will always succeed and return an optional value
//I'm having this return the function trait as opposed to the parser
//trait to get the auto clone impl
pub const fn optional<I,P>(p:P) -> OptPars<P,I>
where
  I:Clone,
  P:Parser<I> + Clone
{
  OptPars{
    pars:p,
    _ghost:std::marker::PhantomData
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
