//this is a module for other examples to depend on
//the errors are really bad right now, we can probably make them better
pub fn lower_w(input:&str) -> Result<(&str,&str),usize> {
  let cs = input.chars();
  let l = cs.take_while(|c|c.is_ascii_lowercase()).count();
  if l==0 {
    Err(0)
  }
  else {
    Ok((&input[0..l],&input[l..]))
  }
}

pub fn digits(input:&str) -> Result<(&str,&str),usize> {
  let cs = input.chars();
  let l = cs.take_while(|c|c.is_ascii_digit()).count();
  if l == 0 {
    Err(1)
  }
  else {
    Ok((&input[0..l],&input[l..]))
  }
}

pub fn decimal_digits(input:&str) -> Result<(&str,&str),usize>  {
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
    Err(2)
  }
  else {
    Ok((&input[0..l],&input[l..]))
  }
}

pub fn int(input:&str) -> Result<(&str,&str),usize> {
  match input.strip_prefix("-") {
    Some(rest) => digits(rest).map(|(rem,res)|(&input[0..rem.len()+1],res)),
    None => digits(input)
  }
}

pub fn float(input:&str) -> Result<(&str,&str),usize> {
  match input.strip_prefix("-") {
    Some(rest) => decimal_digits(rest).map(|(rem,res)|(&input[0..rem.len()+1],res)),
    None => decimal_digits(input)
  }
}

pub fn ws(input:&str) -> Result<(usize,&str),usize> {
  let cs = input.chars();
  let l = cs.take_while(|c|c.is_ascii_whitespace()).count();
  if l == 0 {
    Err(3)
  }
  else {
    Ok((l,&input[l..]))
  }
}

pub fn spaces(input:&str) -> Result<(usize,&str),usize> {
  let cs = input.chars();
  let l = cs.take_while(|c|*c == ' ').count();
  if l == 0 {
    Err(4)
  }
  else {
    Ok((l,&input[l..]))
  }
}

pub fn end(input:&str) -> Result<(&str,&str),usize> {
  if input.is_empty() {
    Ok(("",""))
  }
  else {
    Err(1000)
  }
}

pub fn pfx(word: &'static str) -> impl Fn(&str)->Result<(&str,&str),usize> {
  move |inp|inp.strip_prefix(word).ok_or(5).map(|rest|(word,rest))
}

//this is kinda bad because it doesn't check that the last guy
//is a " so you could have a string with an open quote but no closer
pub fn quoted(input:&str) -> Result<(&str,&str),usize> {
  input.strip_prefix("\"")
  .ok_or(6)
  .and_then(|rest|{
    rest.char_indices().take_while(|(_i,c)|*c != '"').last()
    .ok_or(6)
    .map(|(i,_c)|(&rest[0..(i+1)],&rest[(i+2)..]))
  })
}
