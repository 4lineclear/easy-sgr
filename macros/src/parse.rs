use std::str::CharIndices;

#[derive(Debug)]
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
            s.strip_prefix('"')?
                .strip_suffix('"')
                .map(|s| RawString(s, diff / 2))
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
// TODO remove all panics, return Result instead
/// Removes escapes, parses keywords into their SGR code counterparts
///
/// # Panics
///
/// When invalid string is inputted:
///
/// - Invalid escape
/// - Unclosed bracket
/// - Invalid keyword
///
/// Other than that, the string returned may be an invalid string literal.
/// In these cases, the rust compiler should alert the user of the error.
pub fn sgr_string(s: &str) -> Option<String> {
    let mut buf = String::with_capacity(s.len());
    let chars = &mut s.char_indices();
    let mut next: Option<(usize, char)> = chars.next();

    while let Some((_, ch)) = next {
        match ch {
            // unwrap cannot fail, in the case that it does something is very wrong
            '\\' => match chars
                .next()
                .expect("Unwrapping char following escape failed, should never fail")
                .1
            {
                //quote escapes
                '\'' => buf.push('\''),
                '"' => buf.push('"'),
                //ascii escapes
                'x' => buf.push(parse_7bit(chars, s)?),
                'n' => buf.push('\n'),
                'r' => buf.push('\r'),
                't' => buf.push('\t'),
                '\\' => buf.push('\\'),
                '0' => buf.push('\0'),
                //unicode escape
                'u' => buf.push(parse_24bit(chars, s)?),
                //whitespace ignore
                '\n' => {
                    if let Some(non_whitespace) =
                        chars.find(|(_, ch)| !matches!(ch, ' ' | '\n' | '\r' | '\t'))
                    {
                        next = Some(non_whitespace);
                        continue;
                    }
                    // end of string reached
                }
                _ => return None, // invalid char
            },
            '{' => buf = parse_param(chars.next(), s, chars, buf),
            '}' => match chars.next() {
                Some((_, '}')) => buf.push_str("}}"),
                // ignores invalid bracket, continues parsing
                // compiler will let user know of error
                after_bracket => {
                    buf.push('}');
                    next = after_bracket;
                    continue; // skip calling: next = chars.next();
                }
            },
            ch => buf.push(ch),
        }
        next = chars.next();
    }
    Some(buf)
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
/// - `ch`: the char after the opening brace
/// - `i`: the index of the opening brace plus one(index of `ch`)
/// - `s`: the full string to parse
/// - `chars`: the string's `char_indices`, with chars.next() being the char after ch
/// - `buf`: the string buf to append and return
///
/// # Returns
///
/// `buf` with the parsed param appended
///
/// # Errors
///
/// Returns `Err(String)` when an unclosed closed brace is found.
///
/// # Panics
///
/// When an
fn parse_param(
    next_char: Option<(usize, char)>,
    s: &str,
    chars: &mut CharIndices,
    mut buf: String,
) -> String {
    let Some((start, ch)) = next_char else {
        // string, compiler will let user know of error
        return buf + "{"
    };
    if ch == '}' {
        return buf + "{}";
    } else if ch == '{' {
        return buf + "{{";
    }

    let end = chars.find(|ch| ch.1 == '}').expect("Param not closed").0;
    if ch == '[' {
        buf.push_str("\x1b[");
        for s in s[start + 1..end]
            .strip_suffix(']')
            .expect("Expected a ending square bracket")
            .split_whitespace()
        {
            assert!(parse_sgr(s, &mut buf).is_some(), "Invalid keyword {s}");
            buf.push(';');
        }
        buf.pop().unwrap();
        buf + "m"
    } else {
        buf.push_str(&s[start - 1..=end]);
        buf
    }
}
/// Parses 7bit escape(`\x..`) into a char
fn parse_7bit(chars: &mut CharIndices, s: &str) -> Option<char> {
    let (end, _) = chars.nth(1)?;
    let start = end - 2;
    char::from_u32(u32::from_str_radix(&s[start..=end], 16).ok()?)
}
/// Parses 7bit escape(`\u{..}`) into a char
fn parse_24bit(chars: &mut CharIndices, s: &str) -> Option<char> {
    let (start, _) = chars.nth(1)?;
    let (end, _) = chars.find(|ch| ch.1 == '}')?;
    char::from_u32(u32::from_str_radix(&s[start..end], 16).ok()?)
}
fn parse_sgr(s: &str, buf: &mut String) -> Option<()> {
    if let Some(n) = parse_common(s) {
        n.append_to(buf);
        Some(())
    } else {
        complex_color(s, buf)
    }
}
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
fn complex_color(s: &str, buf: &mut String) -> Option<()> {
    let (color_code, s) = s.strip_prefix("on-").map_or(("38;", s), |s| ("48;", s));
    buf.push_str(color_code);

    if let Some(s) = s.strip_prefix('#') {
        match s.len() {
            2 => {
                buf.push_str("5;");
                u8::from_str_radix(s, 16).ok()?.append_to(buf);
            }
            6 => {
                buf.push_str("2;");
                u8::from_str_radix(&s[0..2], 16).ok()?.append_to(buf);
                buf.push(';');
                u8::from_str_radix(&s[2..4], 16).ok()?.append_to(buf);
                buf.push(';');
                u8::from_str_radix(&s[4..6], 16).ok()?.append_to(buf);
            }
            _ => return None,
        }
    } else {
        let parts = s
            .split(',')
            .map(std::str::FromStr::from_str)
            .collect::<Result<Vec<u8>, _>>()
            .ok()?;
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
            _ => return None,
        }
    }

    Some(())
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
