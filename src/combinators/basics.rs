use crate::Parser;

//this is a parser that will always succeed and return an optional value
//it's handy for certain operations
pub fn optional<I:Clone,O,P:Parser<I,O>>(p:P) -> impl Parser<I,Option<O>,Error=P::Error> {
  move |txt:I| -> Result<(Option<O>,I),P::Error>{
    match p.parse(txt.clone()) {
      Ok((v,r)) => Ok((Some(v),r)),
      Err(..) => Ok((None,txt))
    }
  }
}
