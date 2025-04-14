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
  Null,
  String(&'a str),
  Number(f64),
  Bool(bool),
  List(Vec<Element<'a>>),
  Obj(Vec<(&'a str,Element<'a>)>)
}

fn null(input:&str) -> Option<(Element,&str)> {
  pfx("null").map(|_|Element::Null).parse(input)
}

fn boolean(input:&str) -> Option<(Element,&str)> {
  let t = pfx("true").map(|_|Element::Bool(true));
  let f = pfx("false").map(|_|Element::Bool(false));

  alt((t,f)).parse(input)
}

fn num(input:&str) -> Option<(Element,&str)> {
  float.and_then(|out|out.parse::<f64>().ok().map(Element::Number)).parse(input)
}

fn txt(input:&str) -> Option<(Element,&str)> {
  quoted.map(Element::String).parse(input)
}

fn sep(input:&str) -> Option<((),&str)> {
  (optional(ws),pfx(","),optional(ws)).map(|_|()).parse(input)
}

fn elem(input:&str) -> Option<(Element,&str)> {
  alt((null,boolean,num,txt,list,obj)).parse(input)
}

fn list(input:&str) -> Option<(Element,&str)> {
  use iter::vector::sep_list;

  let seq = (pfx("["),optional(ws),sep_list(elem,sep),optional(ws),pfx("]"));
  seq.map(|(_open,_gap,elems,_end_gap,_close)|Element::List(elems)).parse(input)
}

fn obj(input:&str) -> Option<(Element,&str)> {
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
