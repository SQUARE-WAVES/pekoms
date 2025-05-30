Mr Pekoms
=========

This is a parser combinator library I wrote as a learning exercise. I wanted to get some experience doing
really functional type functional programming with rust and so parser combinators seemed like a way to do that
the goal was to use a bunch of higher order functions and closures, get some practice with macros and types and making
generic stuff.

# so did it work? Did you learn stuff?

I think I learned a bit, I definitely got some practice figuring out how to make closures work better
and when and where simple, i.e. non proc macros can save some time.

# what's left to do?

## errors
Ok so this library doesn't specify much when it comes to errors, that's cause it's hard to know what kind of errors
you will need depending on what kinda input you are parsing. Parsers have to return a result with an error, but that's it.
This is a bit annoying cause good error handling is one of the harder parts of actually writing parsers. and that's a kind
of thing you'd want something like this to help with.

As well there is a problem with some of the compilation errors on the stuff implemented with macros i.e. alt/branch/sequential. 
Basically if you fail one of their implementation bounds you get a big C++ template esque error that isn't very helpful,
say for example you wanted a sequence that looked like (p1,p2,p3) and p2 implemented a different error type than the other 2
well seqs all need the same error type, so you would get an error like "the trait bound parser<{input}> isn't satisfied for (a,b,c) .... blah blah blah"
but it won't tell you what exactly is wrong. 

## types of input
right now the library doesn't really say anthing about the kind of input you can parse. If you want to use a lot of the combinators
your input will need the `Clone` trait. this is becase a lot of things need to backtrack.

Clone is kind of a tricky trait cause it doesn't tell you anything about what it takes to clone something.
Now most of the time this isn't a big deal, stuff like &str, or &[u8] are all clone, and they are also just trivial to copy
but other things aren't. Like the readers you get from files and such like that.

## some more kinds of combinators
there are probably other kinds of useful combinators I haven't thought of yet.

## macro hygine
this library uses macros to implement some traits on sets of tuples. It's stuff you can't really use arrays for 
because arryas have to be homogenously typed and parsers, at least the way I'm doing them, tend to all have different types. 
You can do some tricks with dyn but I think it's preferable not to.

That being said some of the ways I've written the macros aren't the greatest. I need to figure out how to get them to give
better errors

## some helpers for actually doing a parser right
based on my time using this there are some convenience things that I end up making over again. For example when parsing
&strs it's nice to have a data type for the input that keeps track of your parsers position in the original source file.

If you start with a string and do &x[10..20] or something it's not easy to take the resulting &str and ask
what it's position in the original source was. you can get a pointer offset, but that's not exactly safe, cause you can't
validate the &strs provenance. like if our memory looked like [str1,something,something,str2] and I had a pointer to str2
if I asked for it's offset from str1 it would give that to me.


# should I use this in a project?
you can if you want but it's probably not as robust as something like [nom](https://github.com/rust-bakery/nom)
I don't plan on putting it on crates.io or anything given that there is already a popular parser combinator library.

# how do I build it?
this is just a straight up cargo build. You do it in the normal cargo ways:

to build:
```
cargo build
```

to run tests:
```
cargo test
```

to run examples:
```
cargo run --example <name of example here>
```

# whats with the name?

"parser combinators" could be shortened to "p-coms" and there is a minor character in a japanese comic named "mr pekoms"
who is a cat-person that is also kind of a gangster or something. IDK I liked it, and you gotta name this stuff something.

# how do I use this to parse stuff?
as I said, I mostly wrote this as a learning exercise and there are better supported parser combinator libraries out there ([nom](https://github.com/rust-bakery/nom) is the big one) but It wouldn't be much of a project if it wasn't usable, so here is how it's done.

parser combinators are a relatively simple idea, you start with a set of basic functions that just match things, and combine
them with higher-order functions that make them do bigger and badder things. You really don't need a library for it, but
since you will use those higher-order functions over and over again a library helps keep things organized. As well, in rust
you mostly do higher order functions with traits and generics so it's nice to have the trait consolidated.

In pekoms the parser trait looks like this:

```rust
trait Parser<I:Clone,O> {
    type Error:std::error::Error;
    fn parse(input:I) -> Result<(O,I),Self::Error>
}
```

this trait is generically implemented for anything that looks like `Fn(I)->Result<(O,I),E>`, so you can use
regular old functions as your parsers for a really simple example:

```rust
fn basic_parser_that_matches_the_letter_m(input:&str) -> Result<(&str,&str),ParseError> {
    match &input[0..1] {
        "m" => Ok(("m",&input[1..])) //if the first letter is m, return it, and the rest of the input string
        _ => Err(ParseError::new("it didn't match the letter 'm'!!!!")
    }
}

fn 
```

or you can go higher level and use closures:

```rust
fn prefix_parser(pfx:&str) -> impl Fn(&str) -> Result<(&str,&str),ParseError> {
    move |input:&str| {
        input.strip_prefix(pfx) //check if its the prefix,
        .map(|rest|(pfx,rest)) //return the prefix and the rest,
        .ok_or(ParseError::new("prefix didn't match (this is a bad errror!)"))
    }
}
```

then you can combine parsers together to make more complicated ones using built in
traits and parsers
```rust
use pekoms::Parser;

fn letters(input:&str) -> Result<(&str,&str),ParseError> {
    let count = input
    .chars()
    .take_while(|c|c.is_ascii_lowercase() || c.is_ascii_uppercase())
    .count();
   
    if count > 0 {
        Ok((&input[0..c],&input[c..]))
    }
    else {
        Err(ParseError::new("no letters found!"))
    }
}

//notice this returns a different output type! That's fine!
fn spaces(input:&str) -> Result<(usize,&str),ParseError> {
    let count = input
    .chars()
    .take_while(|c|c.is_ascii_lowercase() || c.is_ascii_uppercase())
    .count();
   
    if count > 0 {
        Ok((&input[0..c],&input[c..]))
    }
    else {
        Err(ParseError::new("no letters found!"))
    }
}

//now we can combine them to match something like "<first_name> <last_name>"

struct Name<'a,'b> {
    pub first:&'a str,
    pub last:&'b str
}

fn first_last(input:&str) -> Result<(Name,&str),ParseError> {
    //a tuple means, match this one, then this next one
    (letters,spaces,letters)
    .parse(input)
    .map(|(first,_space_count,last)| {
        Name{first,last}
    })
}
```

there are a lot of other combinators to choose from and I will probably write more!

in the 
examples folder you can see some more complicated stuff. The simplest is sexprs.rs which parses S-Expressions (like lisp code) now this
isn't really a heavyweight test as S-Expressions are famously easy to parse, but it does show the basic structure of 
starting with simple functions that do things like match strings and then move up to abstract types, as well as using recursion.
For a (somewhat) more complicated language there is an example json parser. It can for sure parse basic json, but I haven't really tested it 
against any crazy stuff.

for an example of parsing something that isn't strings, there is also wav_file.rs which parses the header out of wave files, it uses &[u8] as it's
input type
