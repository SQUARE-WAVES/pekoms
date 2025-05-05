use pekoms::ErrorMsg;

//this is a module for other examples to depend on
//the errors are really bad right now, we can probably make them better
pub fn lower_w(input:&str) -> Result<(&str,&str),ErrorMsg> {
  let cs = input.chars();
  let l = cs.take_while(|c|c.is_ascii_lowercase()).count();
  if l==0 {
    Err("not a lower_w".into())
  }
  else {
    Ok((&input[0..l],&input[l..]))
  }
}

pub fn word(input:&str) -> Result<(&str,&str),ErrorMsg> {
  //first thing needs to be a letter
  input.chars().next().and_then(|c|{
    if c.is_ascii_alphabetic() {
      Some(())
    }
    else {
      None
    }
  })
  .ok_or("expected first char to be a letter")?;

  let finale = input.char_indices().find(|(_i,c)|{
    !(c.is_ascii_alphanumeric() || *c == '_')
  });

  match finale {
    Some((i,_c)) => Ok(input.split_at(i)),
    None => Ok((input,""))
  }
}

pub fn alphanum(input:&str) -> Result<(&str,&str),ErrorMsg> {
  let split_point = input.char_indices().find(|(_,c)|!c.is_alphanumeric())
  .map(|(i,_)|i)
  .unwrap_or(input.len());

  if split_point == 0 {
    Err("expected alphanumeric character".into())
  }
  else {
    Ok(input.split_at(split_point))
  }
}

pub fn digits(input:&str) -> Result<(&str,&str),ErrorMsg> {
  let cs = input.chars();
  let l = cs.take_while(|c|c.is_ascii_digit()).count();
  if l == 0 {
    Err("not digits".into())
  }
  else {
    Ok((&input[0..l],&input[l..]))
  }
}

pub fn digit(input:&str) -> Result<(&str,&str),ErrorMsg> {
  input.chars().next()
  .and_then(|c|{
    if c.is_ascii_digit() {
      let len = c.len_utf8();
      Some(input.split_at(len))
    }
    else {
      None
    }
  })
  .ok_or("not a digit".into())
}

pub fn decimal_digits(input:&str) -> Result<(&str,&str),ErrorMsg>  {
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
    Err("not decimal digits".into())
  }
  else {
    Ok((&input[0..l],&input[l..]))
  }
}

pub fn int(input:&str) -> Result<(&str,&str),ErrorMsg> {
  match input.strip_prefix("-") {
    Some(rest) => digits(rest).map(|(rem,res)|(&input[0..rem.len()+1],res)),
    None => digits(input)
  }
}

pub fn float(input:&str) -> Result<(&str,&str),ErrorMsg> {
  match input.strip_prefix("-") {
    Some(rest) => decimal_digits(rest).map(|(rem,res)|(&input[0..rem.len()+1],res)),
    None => decimal_digits(input)
  }
}

pub fn ws(input:&str) -> Result<(usize,&str),ErrorMsg> {
  let cs = input.chars();
  let l = cs.take_while(|c|c.is_ascii_whitespace()).count();
  if l == 0 {
    Err("not whitespace".into())
  }
  else {
    //todo::fix this up to use char indices,
    //this is only safe-ish cause we know ascii whitespace is one letter
    Ok((l,&input[l..])) 
  }
}

pub fn spaces(input:&str) -> Result<(usize,&str),ErrorMsg> {
  let cs = input.chars();
  let l = cs.take_while(|c|*c == ' ').count();
  if l == 0 {
    Err("not spaces".into())
  }
  else {
    Ok((l,&input[l..]))
  }
}

pub fn end(input:&str) -> Result<(&str,&str),ErrorMsg> {
  if input.is_empty() {
    Ok(("",""))
  }
  else {
    Err("not the end".into())
  }
}

pub const fn pfx(word: &'static str) -> impl Fn(&str)->Result<(&str,&str),ErrorMsg> {
  move |inp| {
    inp.strip_prefix(word) 
    .map(|rest|(word,rest))
    .ok_or(word.into())
  }
}

//this is the slow way of doing this, it's fine for small char_sets though:
pub const fn one_of(char_set:&'static str) -> impl Fn(&str)->Result<(char,&str),ErrorMsg> {
  move |inp| {
    let c : char = inp.chars().next().ok_or(ErrorMsg::from("unexpected empty string"))?;
    
    if char_set.contains(c) {
      Ok((c,&inp[c.len_utf8()..])) //you gotta be careful slicing &str
    }
    else {
      Err("its bad".into())
    }
  }
}

pub fn quoted(input:&str) -> Result<(&str,&str),ErrorMsg> {
  input.strip_prefix("\"")
  .ok_or("no start quote".into())
  .and_then(|rest|{
    rest.char_indices().find(|(_i,c)|*c == '"')
    .ok_or("no end quote".into()) 
    .map(|(i,_c)|rest.split_at(i+1)) //this i+1 is safe because we know " is len 1
  })
}
