//the main parser trait, its just a function that
//takes an input and returns an option with an output
//and a residual input so for example a parser which reads
//the first value of a string might look like fn(&str) -> Option<(&str,&str)>
//
//the input has Clone because you might need to look at it more than once with
//things like AND and OR parsers, however in practice it will usually be a kind of reference
//like &str or &[u8] or &[something else], most of the cloning will just be a pointer

pub trait Parser<I:Clone,O> {
  type Error:std::fmt::Debug;

  fn parse(&self,input:I) -> Result<(O,I),Self::Error>;

  //Output mods
  //these are handy functions that allow you to modify the output of an existing parser
  //they are analagous to the same methods on the Result type, as they basically
  //just make a thunk which uses the result methods on the non-residual portion
  //of the output
  
  fn map<O2,F:Fn(O)->O2>(self,f:F) -> impl Parser<I,O2,Error=Self::Error> 
  where Self: std::marker::Sized
  {
    move |i|{
      self.parse(i).map(|(out,residual)|{
        (f(out),residual)
      })
    }
  }

  fn and_then<O2,F:Fn(O)->Result<O2,Self::Error>>(self,f:F) -> impl Parser<I,O2,Error=Self::Error>
  where Self: std::marker::Sized
  {
    move |i|{
      self.parse(i).and_then(|(out,residual)|{
        f(out).map(|r|(r,residual))
      })
    }
  }

  fn map_err<E2:std::fmt::Debug,F:Fn(Self::Error) -> E2>(self,f:F) -> impl Parser<I,O,Error=E2> 
  where Self: std::marker::Sized
  {
    move |i|{
      self.parse(i).map_err(&f)
    }
  }

}

impl<I:Clone,O,E:std::fmt::Debug,F:Fn(I)->Result<(O,I),E>> Parser<I,O> for F {
  type Error=E;

  fn parse(&self, txt:I) -> Result<(O,I),E> {
    self(txt)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn guy(input: &str) -> Result<(&str,&str),&str> {
    match input {
      "" => Err("it's bad"),
      s => Ok((&s[0..1],&s[1..]))
    }
  }

  fn fzer(input: &str) -> Result<usize,&str> {
    match input {
      "f" => Err("it's no good"),
      s => Ok(s.len())
    }
  }

  #[test]
  fn check_map() {
    //check to see if mapping is good here
    let z = guy.map(|s|s.chars().next().unwrap() as u32);
    let (out,res) = z.parse("fishburns").expect("the parsing shouldn't fail");
    assert_eq!(102,out,"the parser should match the first letter and convert it to an int");
    assert_eq!("ishburns",res,"the rest of the string should be the residual");
  }

  #[test]
  fn check_and_then() {
    let z = guy.and_then(fzer);
    let (out,res) = z.parse("gibbles").expect("gibbles should parse");
    assert_eq!(out,1,"the parser should match the first letter count it");
    assert_eq!("ibbles",res,"the rest of the string should be the residual");

    let bad = z.parse("fish");
    assert!(bad.is_err(),"the word fish should fail to parse");
  }
}
