use pekoms::{
  Parser,
  alt::alt,
  iter,
  basics::optional
};

mod parsers;
use parsers::strn::*;

//since I just print the values here,
//it gives you a dead code warning
#[allow(dead_code)]
#[derive(Debug)]
enum Element<'a> {
  Number(i64),
  Symbol(&'a str),
  Text(&'a str),
  Expr(&'a str,Vec<Element<'a>>)
}

fn num(input:&str) -> Result<(Element,&str),usize> {
  int(input).and_then( |(out,res)| {
    out.parse::<i64>()
    .map(|n|(Element::Number(n),res))
    .map_err(|_|11) 
  })
}

fn sym(input:&str) -> Result<(Element,&str),usize> {
  lower_w(input).map(|(s,res)|(Element::Symbol(s),res))
}

fn txt(input:&str) -> Result<(Element,&str),usize> {
  quoted(input).map(|(s,res)|(Element::Text(s),res))
}

fn expr(input:&str) -> Result<(Element,&str),usize> {
  use iter::vector::sep_list;

  let elem = alt((num,sym,txt,expr)).map_err(|_|44);

  let seq = (
    pfx("("),
    optional(ws), 
    lower_w, 
    optional(ws), 
    sep_list(elem,ws), 
    optional(ws), 
    pfx(")")
  );

  seq.map(|(_,_,s,_,elems,_,_)|Element::Expr(s,elems)).parse(input)
}

fn main() {
  let out = expr("(dogs (hogs 15)  (     logs  \"the entire constitution here\" )     )");
  println!("{:?}",out);
}
