//this is just a simple type that implements error for the tests
//I kinda hate having it in this crate but it can't think of a
//less cumbersome thing
#[derive(Debug)]
pub struct ErrorMsg<'a> {
  msg:&'a str
}

impl std::fmt::Display for ErrorMsg<'_>{
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    f.write_str(self.msg)
  }
}

impl std::error::Error for ErrorMsg<'_> {}

impl<'a> From<&'a str> for ErrorMsg<'a> {
  fn from(v:&'a str) -> Self {
    Self { msg:v }
  }
}


pub struct AltErr<I:Clone> { pub inp:I }

impl<I:Clone> std::fmt::Debug for AltErr<I>{
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    f.write_str("AltErr")
  }
}

impl<I:Clone> std::fmt::Display for AltErr<I>{
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    f.write_str("None of the options were found")
  }
}

impl<I:Clone> std::error::Error for AltErr<I>{}

impl<I:Clone> From<I> for AltErr<I> {
  fn from(v:I) -> Self { Self{inp:v.clone()} }
}
