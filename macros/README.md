# easy-sgr-macros

The proc-macro implementation for the
[easy-sgr](https://crates.io/crates/easy-sgr) crate

## Syntax

The syntax of this crate is a near mirror copy to that of the
[fmt] module, with one addition: [SGR] [keywords].

These [keywords] get translated to [SGR] codes at compile time,
using them is similar to formatting variables into string literals
within the [fmt] crate, as you use block brackets
within curly brackets: `{[...]}` to specify them.
Each keyword within is to be separated by spaces.

## Examples

```rust
use easy_sgr::println;

println!("{[bold green on-red]}This is colorful");
println!("{[!bold default on-default]}This is not");
```

See the rust docs for more

## Keywords

### Simple

There are a set of 'simple' keywords, which are made up of a word:

- styles
    - `reset | bold | dim | italic | underline | blink | inverse | hide | strike`
- undo styles
    - `!bold | !dim | !italic | !underline | !blink | !inverse | !hide | !strike`
- foregrounds
    - `black | red | green | yellow | blue | magenta | cyan | white | default`
- backgrounds
    - `on-black | on-red | on-green | on-yellow | on-blue | on-magenta | on-cyan | on-white | on-default`
- reset
    - `{[]}`

`reset` is a little different than the others in that it is empty.

[SGR]: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR
[fmt]: std::fmt
[keywords]: #keywords

### Complex

The more complex syntax is entirely reserved for color codes.

Colors are expected to be one of the following,
optionally prefixed by '`on-`' to indicate being a background color:

- `u8` -> `(38|48);5;u8`
- `u8,u8,u8` -> `(38|48);2;u8;u8;u8`

And, prefixed with `#` to indicate hex,
but without any commas:

- `#u8` -> `(38|48);5;u8`
- `#u8u8u8` -> `(38|48);2;u8;u8;u8`

so some example colors could be

- `on-15` -> `48;5;15`
- `15,115,215` -> `38;2;15;115;215`
- `#0f` -> `38;5;15`
- `on-#0f;73;d7` -> `48;2;15;115;215`

## TODO

- [x] redesign project
- [x] Fix spans
- [ ] docs
    - [ ] Testing docs
    - [x] Syntax docs
    - [x] Internal docs

- [ ] Build build-stream crate (maybe)
- [ ] create col_err (maybe)