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

fn null(input:&str) -> Result<(Element,&str),()> {
  pfx("null").map(|_|Element::Null).parse(input)
}

fn boolean(input:&str) -> Result<(Element,&str),()> {
  let t = pfx("true").map(|_|Element::Bool(true));
  let f = pfx("false").map(|_|Element::Bool(false));

  alt((t,f)).parse(input).map_err(|_|())
}

fn num(input:&str) -> Result<(Element,&str),()> {
  float.and_then(|out|
    out.parse::<f64>()
    .map_err(|_|())
    .map(Element::Number)
  )
  .parse(input)
}

fn txt(input:&str) -> Result<(Element,&str),()> {
  quoted.map(Element::String).parse(input)
}

fn sep(input:&str) -> Result<((),&str),()> {
  (optional(ws),pfx(","),optional(ws)).map(|_|()).parse(input)
}

fn elem(input:&str) -> Result<(Element,&str),()> {
  alt((list,obj,null,boolean,num,txt))
  .map_err(|_|())
  .parse(input)
}

fn list_elems(inp:&str) -> Result<(Vec<Element>,&str),()>{
  let mut out : Vec<Element> = vec![];
  let (_,r) =iter::sep_list(elem,sep,|e|out.push(e)).parse(inp)?;
  Ok((out,r))
}

fn list(input:&str) -> Result<(Element,&str),()> {
  let seq = (pfx("["),optional(ws),list_elems,optional(ws),pfx("]"));
  seq.map(|(_open,_gap,elems,_end_gap,_close)|Element::List(elems)).parse(input)
}

type ObjPair<'a> = (&'a str,Element<'a>);

fn obj_pairs(inp:&str) -> Result<(Vec<ObjPair>,&str),()> {
  let mut out : Vec<(&str,Element)> = vec![];
  let pair = (quoted,optional(ws),pfx(":"),optional(ws),elem).map(|(k,_,_,_,v)|(k,v));
  let (_,r) =iter::sep_list(pair,sep,|e|out.push(e)).parse(inp)?;
  Ok((out,r))
}

fn obj(input:&str) -> Result<(Element,&str),()> {
  let seq = (pfx("{"),optional(ws),obj_pairs,optional(ws),pfx("}"));
  seq.map(|(_open,_gap,pairs,_end_gap,_close)|Element::Obj(pairs)).parse(input)
}

fn main() {
  let out = elem.parse("{\"cats\" : [null, 1,\n\n true, -3.2, \"hogs\", false] }");
  println!("{out:?}");
  
  let out = elem.parse("null");
  println!("{out:?}");

  let out = elem.parse("{\"a\":\"b\",   \"fish\":[   1,2,3,4  ]}");
  println!("{out:?}");
}
