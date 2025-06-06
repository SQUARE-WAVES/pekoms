use crate::Parser;
use crate::basics::optional;

pub struct ParseIter<'a,I,P> {
  parser:&'a P,
  input:I,
}

impl<I,P> Iterator for ParseIter<'_,I,P>
where
  I:Clone,
  P:Parser<I>
{
  type Item = P::Out;

  fn next(&mut self) -> Option<Self::Item> {
    let inp = self.input.clone();
    match self.parser.parse(inp) {
      Ok((out,rest)) => {
        self.input = rest;
        Some(out)
      }
      Err(_) => {
        None
      }
    }
  }
}

impl<'a,I,P> ParseIter<'a,I,P>
where
  I:Clone,
  P:Parser<I>
{
  pub fn new(parser:&'a P,input:I) -> Self {
    Self {
      parser,
      input
    }
  }
 
  pub fn remains(self) -> I {
    self.input
  }
}

pub mod vector {
  use super::*;

  pub const fn star<I,P>(parser:P) -> impl Parser<I,Out=Vec<P::Out>,Error=P::Error> 
  where
    I:Clone,
    P:Parser<I>
  {
    move |txt:I| {
      let mut iter = ParseIter::new(&parser,txt);
      let v = (&mut iter).collect();
      Ok((v,iter.input))
    }
  }
  
  pub const fn plus<I,P>(parser:P) -> impl Parser<I,Out=Vec<P::Out>,Error=P::Error> 
    where
    I:Clone,
    P:Parser<I>
  {
    move |txt:I| {
      let (iv,rest) = parser.parse(txt)?;
      let mut iter = ParseIter::new(&parser,rest);
      let v = std::iter::once(iv).chain(&mut iter).collect();
      Ok((v,iter.input))
    }
  }

  pub const fn sep_list<I,P,P2>(item:P,sep:P2) -> impl Parser<I,Out=Vec<P::Out>,Error=P::Error> 
  where
    I:Clone,
    P:Parser<I>,
    P2:Parser<I,Error=P::Error>
  {
    let parse_seq = (item,optional(sep));
    move |txt:I| {
      
      let mut iter = ParseIter::new(&parse_seq,txt);
      let v = (&mut iter).map(|(o,_)|o).collect();
      Ok((v,iter.input))
    }
  }

  //this is a sep list with a minimum of one match!
  pub const fn sep_list_plus<I,P,P2>(i:P,sp:P2) -> impl Parser<I,Out=Vec<P::Out>,Error=P::Error> 
  where
    I:Clone,
    P:Parser<I>,
    P2:Parser<I,Error=P::Error>
  {
    let parse_seq = (i,optional(sp));
    move |txt:I| {
      match parse_seq.parse(txt) {
        Err(e) => Err(e),

        Ok(((item,None),rest)) => Ok((vec![item],rest)),

        Ok(((item,Some(_)),rest)) => {
          let mut iter = ParseIter::new(&parse_seq,rest);
          let mapped = (&mut iter).map(|(i,_)|i);
          let v = std::iter::once(item).chain(mapped).collect();
          Ok((v,iter.input))
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  //anything but a comma
  fn guy(input: &str) -> Result<(&str,&str),()> {
    match input.chars().next() {
      None => Err(()),
      Some(s) if s.is_alphanumeric() => Ok(input.split_at(s.len_utf8())),
      _ => Err(())
    }
  }

  fn bad_guy(input: &str) -> Result<(&str,&str),()> {
    match input{
      "" => Err(()),
      s if s.starts_with("f") => Err(()),
      s => Ok((&s[0..1],&input[1..]))
    }
  }

  fn comma(input: &str) -> Result<(&str,&str),()> {
    input.strip_prefix(",").map(|r|(",",r)).ok_or(())
  }

  #[test]
  fn check_star_vec() {
    let z = vector::star(guy);

    let (out,res) = z.parse("fish").expect("it should go!");
    assert_eq!(vec!["f","i","s","h"],out,"the parse should go right");
    assert_eq!("",res,"all the input should be used up");

    let z = vector::star(bad_guy);

    let (out,res) = z.parse("dangfish").expect("it should go!");
    assert_eq!(vec!["d","a","n","g"],out,"the short parse should go right");
    assert_eq!("fish",res,"some input should remain");
  }

  #[test]
  fn check_plus_vec() {
    let z = vector::plus(bad_guy);

    let bad = z.parse("fish");
    assert!(bad.is_err(),"the line fish should fail to parse");

    let (out,res) = z.parse("stabs").expect("it should go!");
    assert_eq!(vec!["s","t","a","b","s"],out,"the non-fish parse should go right");
    assert_eq!("",res,"the input should all get used up");
  }

  #[test]
  fn check_sep_list_vec() {
    let z = vector::sep_list(guy,comma);

    let (out,res) = z.parse(",f,i,s,h").expect("rudy can't fail");
    assert!(out.is_empty(),"no items should be found in the first list");
    assert_eq!(",f,i,s,h",res,"no text should be consumed");

    let (out,res) = z.parse("s,t,a,b,s").expect("it should go!");
    assert_eq!(vec!["s","t","a","b","s"],out,"the non-fish parse should go right");
    assert_eq!("",res,"the input should all get used up");
  }

  #[test]
  fn check_sep_list_plus_vec() {
    let z = vector::sep_list_plus(guy,comma);

    z.parse(",f,i,s,h").expect_err("rudy should fail!");

    let (out,res) = z.parse("s,t,a,b,s").expect("it should go!");
    assert_eq!(vec!["s","t","a","b","s"],out,"the non-fish parse should go right");
    assert_eq!("",res,"the input should all get used up");


    //this should match just one item with no separator
    let (out,res) = z.parse("s ffff").expect("it should go!");
    assert_eq!(vec!["s"],out);
    assert_eq!(" ffff",res);
  }
}


