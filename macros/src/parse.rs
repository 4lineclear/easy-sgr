use std::{num::ParseIntError, str::CharIndices};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UnwrappedLiteral<'a> {
    String(&'a str),
    RawString(&'a str, usize),
}
pub fn unwrap_string(s: &str) -> Option<UnwrappedLiteral> {
    use UnwrappedLiteral::*;
    match s.strip_prefix('r') {
        Some(s) => {
            let len = s.as_bytes().len();
            let s = s.trim_matches('#');
            let diff = len - s.as_bytes().len();
            if diff % 2 == 0 {
                s.strip_prefix('"')?
                    .strip_suffix('"')
                    .map(|s| RawString(s, diff / 2))
            } else {
                None
            }
        }
        None => s.strip_prefix('"')?.strip_suffix('"').map(String),
    }
}
pub fn create_raw_string(s: &str, i: usize) -> String {
    // add space for r#".."#
    let mut buf = String::with_capacity(s.len() + i * 2 + 3);
    buf.push('r');
    (0..i).for_each(|_| buf.push('#'));
    buf.push('"');
    buf.push_str(s);
    buf.push('"');
    (0..i).for_each(|_| buf.push('#'));
    buf
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ParseInt(ParseIntError),
    MissingBracket,
    InvalidColorLen,
    CompilerPassOff,
}
impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

/// Removes escapes, parses keywords into their SGR code counterparts
///
/// # Errors
///
/// When invalid string is inputted such as:
///
/// - Invalid escape
/// - Unclosed bracket
/// - Invalid keyword
///
/// Invalid strings can also be occasionally returned with an Ok(), in
/// these cases the string will continue being parsed> When returned the
/// compiler is expected to deal with the error.
/// The spots where these cases occur be annotated by the comment:
/// `// INVALID HERE` or `INVALID RETURN` when continuing parsing is impossible
pub fn sgr_string<F>(s: &str, check_curly: F) -> Result<String, Error>
where
    F: Fn(char) -> Option<&'static str>,
{
    let mut buf = String::with_capacity(s.len());
    let chars = &mut s.char_indices();
    let mut next: Option<(usize, char)> = chars.next();

    while let Some((_, ch)) = next {
        match ch {
            // should never be ran into outside of testing
            '\\' => {
                if let Some(after_escape) = parse_escape(
                    chars.next().ok_or(Error::CompilerPassOff)?.1,
                    s,
                    chars,
                    &mut buf,
                )? {
                    next = Some(after_escape);
                    continue;
                }
            }
            '{' => parse_param(chars.next(), s, chars, &mut buf, &check_curly)?,
            '}' => match chars.next() {
                Some((_, '}')) => buf.push_str("}}"),
                // INVALID HERE
                after_bracket => {
                    buf.push('}');
                    next = after_bracket;
                    continue;
                }
            },
            ch => buf.push(ch),
        }
        next = chars.next();
    }
    Ok(buf)
}
fn parse_escape(
    next_char: char,
    s: &str,
    chars: &mut CharIndices,
    buf: &mut String,
) -> Result<Option<(usize, char)>, Error> {
    match next_char {
        //quote escapes
        '\'' => buf.push('\''),
        '"' => buf.push('"'),
        //ascii escapes
        'x' => buf.push(parse_7bit(chars, s).ok_or(Error::CompilerPassOff)?),
        'n' => buf.push('\n'),
        'r' => buf.push('\r'),
        't' => buf.push('\t'),
        '\\' => buf.push('\\'),
        '0' => buf.push('\0'),
        //unicode escape
        'u' => buf.push(parse_24bit(chars, s).ok_or(Error::CompilerPassOff)?),
        //whitespace ignore
        '\n' => {
            if let Some(non_whitespace) =
                chars.find(|(_, ch)| !matches!(ch, ' ' | '\n' | '\r' | '\t'))
            {
                return Ok(Some(non_whitespace));
            }
            // end of string reached
        }
        _ => return Err(Error::CompilerPassOff), // invalid char
    }
    Ok(None)
}

/// Parses a format param
///
/// i.e. something within curly braces:
///
/// ```plain
///"{..}"
///   ^^
/// ```
///
/// # Params
/// - `next_char`: the index, char pair after the opening brace
/// - `s`: the full string to parse
/// - `chars`: the string's `char_indices`,
/// with `chars.next()` being the char after `next_char`
/// - `buf`: the string buf to append and return
/// - `check_curly`: fn to check if char is curly
///
/// `check_curly` is used since [`sgr`](super::sgr)
/// follows different rules to the other macros
///
fn parse_param(
    next_char: Option<(usize, char)>,
    s: &str,
    chars: &mut CharIndices,
    buf: &mut String,
    check_curly: impl Fn(char) -> Option<&'static str>,
) -> Result<(), Error> {
    let Some((start, ch)) = next_char else {
        // INVALID HERE
        buf.push( '{');
        return Ok(());
    };
    if let Some(s) = check_curly(ch) {
        buf.push_str(s);
        return Ok(());
    }

    // INVALID RETURN
    let Some(end) = chars.find(|ch| ch.1 == '}') else {
        buf.push_str( &s[start-1..]);
        return Ok(());
    };
    let end = end.0;
    if ch == '[' {
        buf.push_str("\x1b[");
        for s in s[start + 1..end]
            .strip_suffix(']')
            .ok_or(Error::MissingBracket)?
            .split_whitespace()
        {
            parse_sgr(s, buf)?;
            buf.push(';');
        }
        // {[..]} if .. is empty it is parsed as reset
        if buf.pop().unwrap() == '[' {
            buf.push_str("[0");
        }
        buf.push('m');
    } else {
        buf.push_str(&s[start - 1..=end]);
    }
    Ok(())
}
/// Parses 7bit escape(`\x..`) into a char
fn parse_7bit(chars: &mut CharIndices, s: &str) -> Option<char> {
    let (end, _) = chars.nth(1)?;
    let start = end - 1;
    char::from_u32(u32::from_str_radix(&s[start..=end], 16).ok()?)
}
/// Parses 7bit escape(`\u{..}`) into a char
fn parse_24bit(chars: &mut CharIndices, s: &str) -> Option<char> {
    let (start, _) = chars.nth(1)?;
    let (end, _) = chars.find(|ch| ch.1 == '}')?;
    char::from_u32(u32::from_str_radix(&s[start..end], 16).ok()?)
}
/// Parses a SGR keyword from the inputted [`str`]
///
/// # Returns
///
/// - `Err(ParseError)` if `s` is an invalid keyword
///
/// First [`parse_common`] is used, if it fails [`complex_color`] is used
fn parse_sgr(s: &str, buf: &mut String) -> Result<(), Error> {
    if let Some(n) = parse_common(s) {
        n.append_to(buf);
        Ok(())
    } else {
        complex_color(s, buf)
    }
}
/// Parses common keywords
fn parse_common(s: &str) -> Option<u8> {
    match s {
        // styles
        "reset" => Some(0),
        "bold" => Some(1),
        "dim" => Some(2),
        "italic" => Some(3),
        "underline" => Some(4),
        "blink" => Some(5),
        "inverse" => Some(7),
        "hide" => Some(8),
        "strike" => Some(9),
        // undo styles
        "!bold" | "!dim" => Some(22),
        "!italic" => Some(23),
        "!underline" => Some(24),
        "!blink" => Some(25),
        "!inverse" => Some(27),
        "!hide" => Some(28),
        "!strike" => Some(29),
        // foregrounds
        "black" => Some(30),
        "red" => Some(31),
        "green" => Some(32),
        "yellow" => Some(33),
        "blue" => Some(34),
        "magenta" => Some(35),
        "cyan" => Some(36),
        "white" => Some(37),
        "default" => Some(39),
        // backgrounds
        "on-black" => Some(40),
        "on-red" => Some(41),
        "on-green" => Some(42),
        "on-yellow" => Some(43),
        "on-blue" => Some(44),
        "on-magenta" => Some(45),
        "on-cyan" => Some(46),
        "on-white" => Some(47),
        "on-default" => Some(49),
        _ => None,
    }
}
/// Parses more complex color configurations.
///
/// Colors are expected to be one of the following,
/// optionally prefixed by `on-` to indicate being a background color:
///
/// - `u8` -> `(38|48);5;u8`
/// - `u8,u8,u8` -> `(38|48);2;u8;u8;u8`
///
/// And, prefixed with `#` to indicate hex,
/// but without any commas:
///
/// - `#u8` -> `(38|48);5;u8`
/// - `#u8u8u8` -> `(38|48);2;u8;u8;u8`
///
/// so some example colors could be
///
/// - `on-15` -> 48;5;15
/// - `15,115,215` -> 38;2;15;115;215
/// - `#0f` -> 38;5;15
/// - `on-#0f;73;d7` -> 48;2;15;115;215
fn complex_color(s: &str, buf: &mut String) -> Result<(), Error> {
    let (color_code, s) = s.strip_prefix("on-").map_or(("38;", s), |s| ("48;", s));
    buf.push_str(color_code);

    if let Some(s) = s.strip_prefix('#') {
        match s.len() {
            2 => {
                buf.push_str("5;");
                u8::from_str_radix(s, 16)?.append_to(buf);
            }
            6 => {
                buf.push_str("2;");
                u8::from_str_radix(&s[0..2], 16)?.append_to(buf);
                buf.push(';');
                u8::from_str_radix(&s[2..4], 16)?.append_to(buf);
                buf.push(';');
                u8::from_str_radix(&s[4..6], 16)?.append_to(buf);
            }
            _ => return Err(Error::InvalidColorLen),
        }
    } else {
        let parts = s
            .split(',')
            .map(std::str::FromStr::from_str)
            .collect::<Result<Vec<u8>, _>>()?;
        match parts[..] {
            [n] => {
                buf.push_str("5;");
                n.append_to(buf);
            }
            [n1, n2, n3] => {
                buf.push_str("2;");
                n1.append_to(buf);
                buf.push(';');
                n2.append_to(buf);
                buf.push(';');
                n3.append_to(buf);
            }
            _ => return Err(Error::InvalidColorLen),
        }
    }

    Ok(())
}

/// A trait for appending self to a given string
///
/// Similar to [`ToString`] but appends to existing string
/// instead of allocating a new one
trait AppendToString {
    /// Appends self converted to a string to an existing string
    fn append_to(&self, s: &mut String);
}
// this would be cool
// impl<AppendToString> ToString for A {
//     fn to_string(&self) -> String {
//         let mut buf = String::new();
//         self.append_to(&mut buf);
//         buf
//     }
// }
impl AppendToString for u8 {
    fn append_to(&self, s: &mut String) {
        let mut n = *self;
        if n >= 10 {
            if n >= 100 {
                s.push((b'0' + n / 100) as char);
                n %= 100;
            }
            s.push((b'0' + n / 10) as char);
            n %= 10;
        }
        s.push((b'0' + n) as char);
    }
}
