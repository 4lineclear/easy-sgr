# Syntax

The syntax of this crate is a near mirror copy to that of the
[fmt] module, with one addition: [SGR] [keywords].

These [keywords] get translated to [SGR] codes at compile time,
using them is similar to formatting variables into string literals
within the [fmt] crate, as you use block brackets
within curly brackets: `{[...]}` to specify them.
Each keyword within is to be separated by spaces.

## Examples

Using styles + reset:

```rust
// or the easy_sgr crate
use easy_sgr_macros::println;

println!("Using styles is easy!{[bold]}");
println!("This is bold{[!bold]}");
println!("This is not{[italic strike underline]}");
println!("This text has lots of style{[]}");
println!("And this one is reset back to normal, I could also use {[reset]}");
```

Using simple colors:

```rust
use easy_sgr_macros::eprintln;

eprintln!("This text is normal{[green]}");
eprintln!("This text is green{[default]}");
eprintln!("Back to normal{[on-red]}");
eprintln!("Now a red background");
eprintln!("{[blue]}With blue text{[]}");
eprintln!("{{[]}} is used to reset(but is escaped here)");
```

Using complex colors:

```rust
use std::io::{stdout, Write};
use easy_sgr_macros::writeln;

let mut stdout = stdout();
writeln!(stdout, "{[15]}This is possible too");
writeln!(stdout, "{[#0f]}With hex as well");
writeln!(stdout, "{[]}Resetting works here too");
writeln!(stdout, "{[on-15]}So do backgrounds");
writeln!(stdout, "{[]}{[on-255,0,0]}RGB is possible too");
writeln!(stdout, "{[#0000ff]}And hex again");
```

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

`reset` is a little different than the other in that it is empty.

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
- `on-#0f73d7` -> `48;2;15;115;215`

## Examples of syntax malfunctions

```rust compile_fail
use easy_sgr_macros::sgr;
let missing_literal = sgr!();
```

```rust compile_fail
use easy_sgr_macros::sgr;
let color_len = sgr!("{[#000]}");
```

```rust compile_fail
use easy_sgr_macros::sgr;
let color_len = sgr!("{[0,0]}");
```

```rust compile_fail
use easy_sgr_macros::sgr;
let invalid_keyword = sgr!("{[this_is_invalid]}");
```
