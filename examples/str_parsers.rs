//this is a module for other examples to depend on
pub fn lower_w(input:&str) -> Option<(&str,&str)> {
  let cs = input.chars();
  let l = cs.take_while(|c|c.is_ascii_lowercase()).count();
  if l==0 {
    None
  }
  else {
    Some((&input[0..l],&input[l..]))
  }
}

pub fn digits(input:&str) -> Option<(&str,&str)> {
  let cs = input.chars();
  let l = cs.take_while(|c|c.is_ascii_digit()).count();
  if l == 0 {
    None
  }
  else {
    Some((&input[0..l],&input[l..]))
  }
}

pub fn decimal_digits(input:&str) -> Option<(&str,&str)>  {
  let cs = input.chars();
  let l = cs.take_while({
    let mut dot = false;
    move |c| {
      if *c == '.' && !dot {
        dot = true;
        true
      }
      else {
        c.is_ascii_digit()
      }
    }
  }).count();

  if l == 0 {
    None
  }
  else {
    Some((&input[0..l],&input[l..]))
  }
}

pub fn int(input:&str) -> Option<(&str,&str)> {
  input.strip_prefix("-")
  .and_then(|rest|digits(rest))
  .map(|(rem,res)|(&input[0..rem.len()+1],res))
  .or_else(||digits(input))
}

pub fn float(input:&str) -> Option<(&str,&str)> {
  input.strip_prefix("-")
  .and_then(|rest|decimal_digits(rest))
  .map(|(rem,res)|(&input[0..rem.len()+1],res))
  .or_else(||decimal_digits(input))
}


pub fn ws(input:&str) -> Option<(usize,&str)> {
  let cs = input.chars();
  let l = cs.take_while(|c|c.is_ascii_whitespace()).count();
  if l == 0 {
    None
  }
  else {
    Some((l,&input[l..]))
  }
}

pub fn spaces(input:&str) -> Option<(usize,&str)> {
  let cs = input.chars();
  let l = cs.take_while(|c|*c == ' ').count();
  if l == 0 {
    None
  }
  else {
    Some((l,&input[l..]))
  }
}

pub fn end(input:&str) -> Option<(&str,&str)> {
  if input.is_empty() {
    Some(("",""))
  }
  else {
    None
  }
}

pub fn pfx(word: &'static str) -> impl Fn(&str)->Option<(&str,&str)> {
  move |inp|inp.strip_prefix(word).map(|rest|(word,rest))
}

pub fn quoted(input:&str) -> Option<(&str,&str)> {
  input.strip_prefix("\"")
  .and_then(|rest|{
    rest.char_indices().take_while(|(_i,c)|*c != '"').last()
    .map(|(i,_c)|(&rest[0..(i+1)],&rest[(i+2)..]))
  })
}
