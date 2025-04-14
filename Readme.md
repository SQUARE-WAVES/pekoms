Mr Pekoms
=========

This is a parser combinator library I wrote as a learning exercise. I wanted to get some experience doing
really functional type functional programming with rust and so parser combinators seemed like a way to do that
the goal was to use a bunch of higher order functions and closures, get some practice with macros and making
generic stuff.

# so did it work? Did you learn stuff?

I think I learned a bit, I definitely got some practice figuring out how to make closures work better
and when and where simple, i.e. non proc macros can save some time.

# what's left to do?

## errors

right now this library sucks WRT errors. It just uses Option, meaning if something fails to parse, or you try
and run some kinda function on the value you get from parsing that might fail, it just returns None, so you don't 
really know what happened. The issue is that since any parser or post parsing function might use a different
error type you have to make everything using N parser combinators generic across N error types. Which is a huge
pain to write and makes everything harder to read.

The best solution is to force everything into a single error type, but then you have to either depend on something
like (anyhow)[https://github.com/dtolnay/anyhow] or you have to build in an error type and force everyone to comply
to it.

(anyhow)[https://github.com/dtolnay/anyhow] is a good crate and I think people should use it in projects, but it feels
really bad to make people include it from a package, and even if I don't think anyone is going to use this I want to
design it right, so I'm probably gonna make an error type.

## streams and other types of input

right now there isn't anything specific to prevent you from using a stream to parse stuff, except that the library probably
won't play well with std::io. The issue is that it expects your input type to be immutable, so it can backtrack, however
std::io::Read work that way, it changes it's position every time you read more data. Now if you also have std::io::Seek you can
backtrack, but it's an operation to do so.

It's almost certainly possible to make an efficient wrapper, that uses a pointer to a buffer and a std::io::Read type, where
you create a new value each time you read further, maybe you can do this with all the cursor stuff in std::io, I don't know
yet. If I end up having a problem where I want to do this kinda thing. I'll figure out a way to handle this but for now I'm not
really that concerned about it.

# should I use this in a project?
you can if you want but it's probably not as robust as something like (nom)[https://github.com/rust-bakery/nom]
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
