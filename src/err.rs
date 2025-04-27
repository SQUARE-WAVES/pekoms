//this is just a simple type that implements error for the tests
//I kinda hate having it here but then the other options are
//you have to import some kinda kitchen sink crate like derive_more
//or you have to have a crate that is just an error type
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


//this is a special error for the alt combinator
//it basically says "we didn't find the thing you were looking for"
pub struct AltErr {}

impl std::fmt::Debug for AltErr{
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    f.write_str("AltErr")
  }
}

impl std::fmt::Display for AltErr{
  fn fmt(&self,f:&mut std::fmt::Formatter) -> Result<(),std::fmt::Error> {
    f.write_str("None of the options were found")
  }
}

impl std::error::Error for AltErr{}

impl From<()> for AltErr {
  fn from(_v:()) -> Self { Self{} }
}
