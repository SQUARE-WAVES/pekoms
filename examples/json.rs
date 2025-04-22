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
  Null,
  String(&'a str),
  Number(f64),
  Bool(bool),
  List(Vec<Element<'a>>),
  Obj(Vec<(&'a str,Element<'a>)>)
}

fn null(input:&str) -> Result<(Element,&str),usize> {
  pfx("null").map(|_|Element::Null).parse(input)
}

fn boolean(input:&str) -> Result<(Element,&str),usize> {
  let t = pfx("true").map(|_|Element::Bool(true));
  let f = pfx("false").map(|_|Element::Bool(false));

  alt((t,f)).parse(input).map_err(|_|44)
}

fn num(input:&str) -> Result<(Element,&str),usize> {
  float.and_then(|out|
    out.parse::<f64>()
    .map_err(|_|44)
    .map(Element::Number)
  )
  .parse(input)
}

fn txt(input:&str) -> Result<(Element,&str),usize> {
  quoted.map(Element::String).parse(input)
}

fn sep(input:&str) -> Result<((),&str),usize> {
  (optional(ws),pfx(","),optional(ws)).map(|_|()).parse(input)
}

fn elem(input:&str) -> Result<(Element,&str),usize> {
  alt((null,boolean,num,txt,list,obj)).map_err(|_|44).parse(input)
}

fn list(input:&str) -> Result<(Element,&str),usize> {
  use iter::vector::sep_list;

  let seq = (pfx("["),optional(ws),sep_list(elem,sep),optional(ws),pfx("]"));
  seq.map(|(_open,_gap,elems,_end_gap,_close)|Element::List(elems)).parse(input)
}

fn obj(input:&str) -> Result<(Element,&str),usize> {
  use iter::vector::sep_list;
  let pair = (quoted,optional(ws),pfx(":"),optional(ws),elem).map(|(k,_,_,_,v)|(k,v));
  let seq = (pfx("{"),optional(ws),sep_list(pair,sep),optional(ws),pfx("}"));

  seq.map(|(_open,_gap,pairs,_end_gap,_close)|Element::Obj(pairs)).parse(input)
}

fn main() {
  let out = elem.parse("{\"cats\" : [null, 1,\n\n true, -3.2, \"hogs\", false] }");
  println!("{:?}",out);
  
  let out = elem.parse("null");
  println!("{:?}",out);

  let out = elem.parse("{\"a\":\"b\",   \"fish\":[   1,2,3,4  ]}");
  println!("{:?}",out);
}
