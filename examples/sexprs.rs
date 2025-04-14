use pekoms::{
  Parser,
  alt::alt,
  iter,
  basics::optional
};

mod str_parsers;
use str_parsers::*;


#[derive(Debug)]
enum Element<'a> {
  Number(i64),
  Symbol(&'a str),
  Text(&'a str),
  Expr(&'a str,Vec<Element<'a>>)
}

fn num(input:&str) -> Option<(Element,&str)> {
  int.and_then(|out|out.parse::<i64>().ok().map(Element::Number)).parse(input)
}

fn sym(input:&str) -> Option<(Element,&str)> {
  lower_w.map(Element::Symbol).parse(input)
}

fn txt(input:&str) -> Option<(Element,&str)> {
  quoted.map(Element::Text).parse(input)
}

fn expr(input:&str) -> Option<(Element,&str)> {
  use iter::vector::sep_list;

  let elem = alt((num,sym,txt,expr));
  let seq = (pfx("("),optional(ws),lower_w,optional(ws),sep_list(elem,ws),optional(ws),pfx(")"));

  seq.map(|(_,_,s,_,elems,_,_)|Element::Expr(s,elems)).parse(input)
}

fn main() {
  let out = expr("(dogs (hogs 15)  (     logs  \"the entire constitution here\" )     )");
  println!("{:?}",out);
}
