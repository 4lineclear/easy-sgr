
# Syntax

Works the same as the [`fmt`](std::fmt) class of macros,
with set keywords replaced with SGR codes.
These keywords are invoked within curly brackets
in a similar way variables are captured.
each keyword is prefixed with a delimiter that determines
what type of SGR code it will be.

There are three basic types:

- `+` -> Add
    - Reset
    - Everything under the 'Remove Style'
- `-` -> Remove Style
    - `Bold` `Dim` `Italic` `Underline` `Blinking` `Inverse` `Hidden` `Strikethrough`
- `#` -> Color
    - `BlackFg` `RedFg` `GreenFg` `YellowFg` `BlueFg` `MagentaFg`
`CyanFg` `WhiteFg` `DefaultFg` `BlackBg` `RedBg` `GreenBg`
`YellowBg` `BlueBg` `MagentaBg` `CyanBg` `WhiteBg` `DefaultBg`
- `&` -> Format param capture
    - Anything put in normal curly braces

Color is special in that th you

# See also

- [`easy_sgr`](https://docs.rs/easy-sgr/latest/easy_sgr/)
- [`std::fmt`]
