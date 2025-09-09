//the main parser trait, its just a function that
//takes an input and returns an option with an output
//and a residual input so for example a parser which reads
//the first value of a string might look like FnOnce(&str) -> Result<(&str,&str),()>

pub trait Parser<I> {
  type Error;
  type Out;

  fn parse(self,input:I) -> Result<(Self::Out,I),Self::Error>;

  //really these should be const
  fn map<O2,F:FnOnce(Self::Out)->O2>(self,f:F) -> Map<I,O2,Self,F>
  where
    Self: std::marker::Sized
  {
    Map{
      p:self,
      mapper:f,
      _ghost:std::marker::PhantomData
    }
  }

  fn and_then<O2,F>(self,f:F) -> AndThen<I,O2,Self,F>
  where 
    Self: std::marker::Sized,
    F:Fn(Self::Out)->Result<O2,Self::Error>
  {
    AndThen {
      p:self,
      next:f,
      _ghost:std::marker::PhantomData
    }
  }

  fn map_err<E2,F:FnOnce(Self::Error) -> E2>(self,f:F) -> MapErr<I,E2,Self,F>
  where
    Self: std::marker::Sized
  {
    MapErr {
      p:self,
      mapper:f,
      _ghost:std::marker::PhantomData
    }
  }
}

//Blanket impl for parsing functions
impl<I,O,E,F:FnOnce(I)->Result<(O,I),E>> Parser<I> for F {
  type Error=E;
  type Out=O;

  fn parse(self, txt:I) -> Result<(O,I),E> {
    self(txt)
  }
}

pub struct Map<I,O2,P,F> {
  p:P,
  mapper:F,
  _ghost:std::marker::PhantomData<(I,O2)>
}

impl<I,O2,P,F> Parser<I> for Map<I,O2,P,F> 
where
  P:Parser<I>,
  F:FnOnce(P::Out) -> O2
{
  type Out=O2;
  type Error=P::Error;

  fn parse(self,inp:I) -> Result<(Self::Out,I),Self::Error> {
    match self.p.parse(inp) {
      Ok((v,r)) => Ok(((self.mapper)(v),r)),
      Err(e) => Err(e)
    }
  }
}

impl <I,O2,P,F> Clone for Map<I,O2,P,F>
where
  P:Clone,
  F:Clone
{
  fn clone(&self) -> Self {
    Map{
      p:self.p.clone(),
      mapper:self.mapper.clone(),
      _ghost:std::marker::PhantomData
    }
  }
}

pub struct MapErr<I,E2,P,F> {
  p:P,
  mapper:F,
  _ghost:std::marker::PhantomData<(I,E2)>
}

impl<I,E2,P,F> Parser<I> for MapErr<I,E2,P,F> 
where
  P:Parser<I>,
  F:FnOnce(P::Error) -> E2
{
  type Out=P::Out;
  type Error=E2;

  fn parse(self,inp:I) -> Result<(Self::Out,I),Self::Error> {
    match self.p.parse(inp) {
      Ok((v,r)) => Ok((v,r)),
      Err(e) => Err((self.mapper)(e))
    }
  }
}

impl <I,E2,P,F> Clone for MapErr<I,E2,P,F>
where
  P:Clone,
  F:Clone
{
  fn clone(&self) -> Self {
    MapErr{
      p:self.p.clone(),
      mapper:self.mapper.clone(),
      _ghost:std::marker::PhantomData
    }
  }
}

pub struct AndThen<I,O2,P,F> {
  p:P,
  next:F,
  _ghost:std::marker::PhantomData<(I,O2)>
}

impl<I,O2,P,F> Parser<I> for AndThen<I,O2,P,F> 
where
  P:Parser<I>,
  F:FnOnce(P::Out) -> Result<O2,P::Error>
{
  type Out=O2;
  type Error=P::Error;

  fn parse(self,inp:I) -> Result<(Self::Out,I),Self::Error> {
    match self.p.parse(inp) {
      Ok((v,r)) => match (self.next)(v) {
        Ok(v2) => Ok((v2,r)),
        Err(e) => Err(e)
      },
      Err(e) => Err(e)
    }
  }
}

impl <I,O2,P,F> Clone for AndThen<I,O2,P,F>
where
  P:Clone,
  F:Clone
{
  fn clone(&self) -> Self {
    AndThen{
      p:self.p.clone(),
      next:self.next.clone(),
      _ghost:std::marker::PhantomData
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn guy(input: &str) -> Result<(&str,&str),()> {
    match input {
      "" => Err(()),
      s => Ok((&s[0..1],&s[1..]))
    }
  }

  fn fzer(input: &str) -> Result<usize,()>{
    match input {
      "f" => Err(()),
      s => Ok(s.len())
    }
  }

  #[test]
  fn check_map() {
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

    let z = guy.and_then(fzer);
    let bad = z.parse("fish");
    assert!(bad.is_err(),"the word fish should fail to parse");
  }
}
