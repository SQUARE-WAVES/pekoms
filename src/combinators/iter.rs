use crate::Parser;
use std::error::Error;

fn run_parser<I,O,P,S,CB>(txt:I,parser:&P,mut state:S,collect:&CB) -> Result<(S,I),P::Error> 
where
    I:Clone,
    P:Parser<I,O>,
    CB:Fn(&mut S,O)
{
  let mut residual = txt;
  while let Ok((v,res)) = parser.parse(residual.clone()) {
    collect(&mut state,v);
    residual = res;
  }

  Ok((state,residual))
}

fn run_wheel<I,O,O2,E,P,P2,S,CB>(txt:I,w1:&P,w2:&P2,mut state:S,collect:&CB) -> Result<(S,I),E> 
where
    I:Clone,
    E:std::fmt::Debug,
    P:Parser<I,O,Error=E>,
    P2:Parser<I,O2,Error=E>,
    CB:Fn(&mut S,O)
{
  let mut residual = txt;
  let mut turn = 0;

  loop {
    if turn == 0 {
      if let Ok((v,res)) = w1.parse(residual.clone()) {
        collect(&mut state,v);
        residual = res;
        turn = 1;
        continue;
      }
      else {
        return Ok((state,residual))
      }
    }
    
    if let Ok((_v,res)) = w2.parse(residual.clone()) {
      residual = res;
      turn = 0;
    } 
    else {
      return Ok((state,residual))
    }
  }
}

pub mod vector {
  use super::*;

  pub const fn star<I:Clone,O,P:Parser<I,O>>(parser:P) -> impl Parser<I,Vec<O>> {
    move |txt:I| {
      let outs : Vec<O> = vec![];
      let collect = |st:&mut Vec<O>,val:O|st.push(val);
      run_parser(txt,&parser,outs,&collect)
    }
  }
  
  pub const fn plus<I:Clone,O,P:Parser<I,O>>(parser:P) -> impl Parser<I,Vec<O>> {
    move |txt:I| {
      let (init_val,init_resid) = parser.parse(txt)?;
      let outs : Vec<O> = vec![init_val];
      let collect = |st:&mut Vec<O>,val:O|st.push(val);
      run_parser(init_resid,&parser,outs,&collect)
    }
  }

  pub const fn sep_list<I,O,O2,E,P,P2>(item:P,sep:P2) -> impl Parser<I,Vec<O>,Error=E> 
  where
    I:Clone,
    E:Error,
    P:Parser<I,O,Error=E>,
    P2:Parser<I,O2,Error=E>
  {
    move |txt:I| {
      let outs : Vec<O> = vec![];
      let collect = |st:&mut Vec<O>,val:O|st.push(val);
      run_wheel(txt,&item,&sep,outs,&collect)
    }
  }
}

pub mod generic {
  use super::*;

  pub const fn star<I,O,P,S,Init,CB>(parser:P,init:Init,collect:CB) -> impl Parser<I,S> 
  where
    I:Clone,
    P:Parser<I,O>,
    Init:Fn() -> S,
    CB:Fn(&mut S,O)
  {
    move |txt:I| {
      let outs = init();
      run_parser(txt,&parser,outs,&collect)
    }
  }

  pub const fn plus<I,O,P,S,Init,CB>(parser:P,init:Init,collect:CB) -> impl Parser<I,S> 
  where
    I:Clone,
    P:Parser<I,O>,
    Init:Fn(O) -> S,
    CB:Fn(&mut S,O)
  {
    move |txt:I| {
      let (init_v,init_res) = parser.parse(txt)?;
      let outs = init(init_v);
      run_parser(init_res,&parser,outs,&collect)
    }
  }

  pub const fn sep_list<I,O,O2,E,P,P2,S,Ini,CB>(it:P,sp:P2,init:Ini,collect:CB) -> impl Parser<I,S> 
  where
    I:Clone,
    E:Error,
    P:Parser<I,O,Error=E>,
    P2:Parser<I,O2,Error=E>,
    Ini:Fn() -> S,
    CB:Fn(&mut S,O)
  {
    move |txt:I| {
      let outs = init();
      run_wheel(txt,&it,&sp,outs,&collect)
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::err::ErrorMsg;

  //anything but a comma
  fn guy(input: &str) -> Result<(&str,&str),ErrorMsg> {
    match input {
      "" => Err("it's over".into()),
      s if s.starts_with(",") => Err("it's not good".into()),
      s => Ok((&s[0..1],&s[1..]))
    }
  }

  fn bad_guy(input: &str) -> Result<(&str,&str),ErrorMsg> {
    match input{
      "" => Err("it's over".into()),
      s if s.starts_with("f") => Err("it's the bad one!".into()),
      s => Ok((&s[0..1],&input[1..]))
    }
  }

  fn comma(input: &str) -> Result<(&str,&str),ErrorMsg> {
    input.strip_prefix(",").map(|r|(",",r)).ok_or("it's no good".into())
  }

  #[test]
  fn check_star_vec() {
    let z = vector::star(guy);

    let (out,res) = z.parse("fish").expect("it should go!");
    assert_eq!(vec!["f","i","s","h"],out,"the parse should go right");
    assert_eq!("",res,"all the input should be used up");

    let z = vector::star(bad_guy);

    let (out,res) = z.parse("dangfish").expect("it should go!");
    println!("!!!!! out:{:?}, res:{}",out,res);
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
  fn check_star_generic() {
    let start = ||String::new();
    let collect = |st:&mut String,letter|st.push_str(letter);
    let z = generic::star(guy,&start,&collect);

    let (out,res) = z.parse("fish").expect("it should go!");
    assert_eq!("fish",out,"the parse should go right");
    assert_eq!("",res,"all the input should be used up");

    let z = generic::star(bad_guy,&start,&collect);

    let (out,res) = z.parse("dangfish").expect("it should go!");
    println!("!!!!! out:{:?}, res:{}",out,res);
    assert_eq!("dang",out,"the short parse should go right");
    assert_eq!("fish",res,"some input should remain");
  }

  #[test]
  fn check_plus_generic() {
    let start = |word|String::from(word);
    let collect = |st:&mut String,letter|st.push_str(letter);
    let z = generic::plus(guy,&start,&collect);

    let (out,res) = z.parse("fish").expect("it should go!");
    assert_eq!("fish",out,"the parse should go right");
    assert_eq!("",res,"all the input should be used up");

    let z = generic::plus(bad_guy,&start,&collect);

    let (out,res) = z.parse("dangfish").expect("it should go!");
    println!("!!!!! out:{:?}, res:{}",out,res);
    assert_eq!("dang",out,"the short parse should go right");
    assert_eq!("fish",res,"some input should remain");
  }

  #[test]
  fn check_sep_list_generic() {
    let start = ||String::new();
    let collect = |st:&mut String,letter|st.push_str(letter);

    let z = generic::sep_list(guy,comma,&start,&collect);

    let (out,res) = z.parse(",f,i,s,h").expect("rudy can't fail");
    assert!(out.is_empty(),"no items should be found in the first list");
    assert_eq!(",f,i,s,h",res,"no text should be consumed");

    let (out,res) = z.parse("s,t,a,b,s").expect("it should go!");
    assert_eq!("stabs",out,"the non-fish parse should go right");
    assert_eq!("",res,"the input should all get used up");
  }
}


