use pekoms::{
  Parser,
  alt::alt,
};

mod parsers;
use parsers::strn::*;

#[derive(Default)]
pub struct Sys<'a> {
  stack:Vec<i64>,
  def_stack:Vec<Op<'a>>,
  def_mode:bool,
  dictionary:std::collections::HashMap<&'a str,Vec<Op<'a>>>
}

impl<'a> Sys<'a> {
  pub fn exec(&mut self,op:Op<'a>) {
    if self.def_mode {
      match op {
        Op::EndDef(word) => {
          self.dictionary.insert(word,self.def_stack.clone());
          self.def_stack.clear();
          self.def_mode=false;
        },
        Op::StartDef => panic!("NO EMBEDS"),
        o => self.def_stack.push(o)
      }

      return;
    }

    match op {
      Op::Push(n) => self.stack.push(n),

      Op::Add => {
        let n1 = self.stack.pop().expect("stack underflow");
        let n2 = self.stack.pop().expect("stack underflow");
        self.stack.push(n1+n2);
      },

      Op::Sub => {
        let n1 = self.stack.pop().expect("stack underflow");
        let n2 = self.stack.pop().expect("stack underflow");
        self.stack.push(n1-n2);
      },

      Op::Mul => {
        let n1 = self.stack.pop().expect("stack underflow");
        let n2 = self.stack.pop().expect("stack underflow");
        self.stack.push(n1*n2);
      },

      Op::Div => {
        let n1 = self.stack.pop().expect("stack underflow");
        let n2 = self.stack.pop().expect("stack underflow");
        self.stack.push(n2/n1);
      },

      Op::StartDef => {
        self.def_mode = true;
      },

      Op::Word(n) => {
        let ops = self.dictionary.get(n).expect("undefined word").clone();
        for op in ops.into_iter() {
          self.exec(op)
        }
      },

      _ => panic!("invalid opcode")
    }
  }

}

#[derive(Copy,Clone,Debug)]
pub enum Op<'a> {
  Push(i64),
  StartDef,
  EndDef(&'a str),
  Word(&'a str),
  Add,
  Sub,
  Mul,
  Div
}

fn main() {
  let add = pfx("+").map(|_|Op::Add);
  let sub = pfx("-").map(|_|Op::Sub);
  let mul = pfx("*").map(|_|Op::Mul);
  let div = pfx("/").map(|_|Op::Div);
  let def_s = pfx("[").map(|_|Op::StartDef);
  let word = lower_w.map(Op::Word);
  let num = digits.map(|d|Op::Push(d.parse::<i64>().expect("bad parse")));
  let def_e = (pfx("]"),lower_w).map(|(_,w)|Op::EndDef(w));

  let mut sys = Sys::default();

  let operation = alt((num,add,sub,mul,div,def_s,def_e,word));

  //define 10 + as "pz" call pz with 1 as the arg (11) then add 1 then divide by 3 to get 4"
  let mut input = "[10 +]pz 1 pz 1 + 3 /";

  loop {
    if input.is_empty() {
      break;
    }

    let op = operation.clone().map(|op|sys.exec(op));

    (_,input) = op.parse(input).expect("it should go");
    //let (_,r) = optional(ws).parse(input).expect("should ws");
    input = input.trim();
  }

  println!("{:?}",sys.stack);
}

