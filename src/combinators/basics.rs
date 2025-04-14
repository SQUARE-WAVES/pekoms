use crate::Parser;

//this is a parser that will always succeed and return an optional value
//it's handy for certain operations
pub fn optional<I:Clone,O,P:Parser<I,O>>(p:P) -> impl Parser<I,Option<O>> {
  move |txt:I|{
    let v = p.parse(txt.clone()).map_or((None,txt),|(out,res)|(Some(out),res));
    //gotta wrap it
    Some(v)
  }
}
