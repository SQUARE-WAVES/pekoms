use pekoms::{
  Parser,
  alt::alt,
  basics::optional,
  iter::vector::sep_list,
  ErrorMsg
};

mod parsers;
use parsers::strn::*;

#[derive(Debug)]
struct Legend<'a> {
  name:&'a str,
  entries:Vec<(String,u8,u8)>,
}

#[derive(Debug)]
struct BarLine<'a> {
  leg_name:&'a str,
  first_bar:Vec<BarItem<'a>>,
  remains:Vec<BarLineItem<'a>>
}

#[derive(Debug)]
enum BarItem<'a> {
  Rest,
  Tie,
  Trigger(&'a str),
}

#[derive(Debug)]
enum BarLineItem<'a> {
  Bar(Vec<BarItem<'a>>),
  Repeat(usize)
}

#[derive(Debug)]
enum SeqItem<'a> {
  Legend(Legend<'a>),
  BarLine(BarLine<'a>)
}

fn pitch_class(input:&str) -> Result<(isize,&str),ErrorMsg> {
  one_of("aAbBcCdDeEfFgG").map(|pc|{
    match pc {
      'c'|'C' => 0,
      'd'|'D' => 2,
      'e'|'E' => 4,
      'f'|'F' => 5,
      'g'|'G' => 7,
      'a'|'A' => 9,
      'b'|'B' => 11,
      _ => unreachable!("a one of parser returned an impossible character")
    }
  })
  .parse(input)
}

fn modifier(input:&str) -> Result<(isize,&str),ErrorMsg> {
  one_of("b_#").map(|modder|{
    match modder {
      'b' => -1,
      '_' => 0,
      '1' => 1,
      _ => unreachable!("a one of parser returned an impossible character")
    }
  })
  .parse(input)
}

fn octave(input:&str) -> Result<(isize,&str),ErrorMsg> {
  let (d,rest) = digit(input)?;
  let v = d.parse::<isize>().map_err(|_|ErrorMsg::from("couldn't parse digit"))?;
  Ok((v,rest))
}

fn note(input:&str) -> Result<(u8,&str),ErrorMsg> {
  (pitch_class,modifier,octave).map(|(pc,md,oct)|{
    (pc + md + (oct*12) + 24) as u8
  })
  .parse(input)
}

fn midi_num(input:&str) -> Result<(u8,&str),ErrorMsg> {
  digits.parse(input).and_then(|(ds,rest)|{
    let v = ds.parse::<u8>().map_err(|_|ErrorMsg::from("midinum digits didn't parse"))?;
    if v > 127 {
      Err("midi number out of range".into())
    }
    else {
      Ok((v,rest))
    }
  })
}

fn legend_entry(input:&str) -> Result<((String,u8,u8),&str),ErrorMsg> {
  let open = (optional(spaces),alphanum,optional(spaces),pfx("="),optional(spaces))
  .map(|(_,nm,_,_,_)|nm);

  let note_val = alt((midi_num,note))
  .map_err(|_|ErrorMsg::from("couldn't find a note or note number"));

  (optional(spaces),open,pfx("("),optional(spaces),note_val,spaces,midi_num,optional(spaces),pfx(")"))
  .map(|(_,nm,_,_,note,_,vel,_,_)|(nm.to_owned(),note,vel))
  .parse(input)
}

fn legend(input:&str) -> Result<(Legend,&str),ErrorMsg> {
  let open = (word,optional(spaces),pfx(":"),pfx("{"),optional(pfx("\n"))).map(|(nm,_,_,_,_)|nm);
  let entries = sep_list(legend_entry,(pfx(","),optional(ws)));
  (open,entries,optional(ws),pfx("}"))
  .map(|(nm,entries,_,_)|Legend{name:nm,entries}).parse(input)
}

//for bars
fn bar_items(input:&str) -> Result<(Vec<BarItem>,&str),ErrorMsg> {
  let rest = pfx("-").map(|_|BarItem::Rest);
  let tie = pfx("=").map(|_|BarItem::Tie);
  let trig = alphanum.map(BarItem::Trigger);

  let item = alt((trig,tie,rest))
  .map_err(|_|ErrorMsg::from("expected a trigger, a tie,or a rest"));
  
  let entries = sep_list(item,optional(spaces));

  entries.parse(input)
}

fn bar(input:&str) -> Result<(Vec<BarItem>,&str),ErrorMsg> {
  (pfx("["),bar_items,optional(spaces),pfx("]"))
  .map(|(_,items,_,_)|items)
  .parse(input)
}

fn bar_line(input:&str) -> Result<(BarLine,&str),ErrorMsg> {
  let bar_item = bar.map(BarLineItem::Bar);
  
  let repeater = digits.and_then(|d|{
    match d.parse::<usize>() {
      Ok(u) => Ok(BarLineItem::Repeat(u)),
      Err(_) => Err(ErrorMsg::from("couldn't parse to usize"))
    }
  });

  let post_item = alt((bar_item,repeater)).map_err(|_|ErrorMsg::from("wow"));

  (word,bar,optional((pfx(":"),sep_list(post_item,pfx(":")))))
  .map(|(name,first_bar,post_list)| {
    let remains = post_list.map(|(_,items)|items).unwrap_or(vec![]);
    BarLine{leg_name:name,first_bar,remains}
  })
  .parse(input)
}

fn line(input:&str) -> Result<(SeqItem,&str),ErrorMsg> {
  let item = alt((
    legend.map(SeqItem::Legend),
    bar_line.map(SeqItem::BarLine)
  ))
  .map_err(|_|ErrorMsg::from("it's not good"));
  
  let out_style = (optional(ws),item).map(|(_,itm)|itm).parse(input);

  out_style
}

fn sequence(input:&str) -> Result<(Vec<SeqItem>,&str),ErrorMsg> {
  pekoms::iter::vector::plus(line).parse(input)
}

fn main() {
  let input = r"
  hats:{
    1 = (E_1 64),
    2 = (E_1 96),
    3 = (G_1 127)
  }

  kick:{
    X = (C_1 127),
    x = (C_1 72)
  }

  hats[1 - 2 - ]:3:[1 - 3 - ]
  kick[X - - -]:7:[X - x]
  kick[f - f - ]

  snare:{
    1 = (E_1 64),
    2 = (E_1 96),
    3 = (G_1 127)
  }

  snare:{
    X = (33 127),
    x = (33 64),
    O = (33 32),
    o = (33 16)
  }
  ";

  match sequence.parse(input) {
    Ok((vec,rest)) => {
      println!("data:\n{:?} \n\n remains\n{}",vec,rest);
    },
    Err(e) => {
      println!("{}",e)
    }
  }
}

