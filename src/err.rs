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
