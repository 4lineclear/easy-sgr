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
                .map(|s| RawString(s, diff))
        }
        None => s.strip_prefix('"')?.strip_suffix('"').map(String),
    }
}
pub fn parse_raw_string(s: &str, i: usize) -> String {
    let mut buf = String::with_capacity(s.len() + i);
    let chars = &mut s.char_indices();
    let mut next = chars.next();
    buf.push('r');
    buf.push('"');
    (0..i).for_each(|_| buf.push('#'));

    while let Some((_, ch)) = next {
        match ch {
            '{' => match chars.next() {
                Some((_, '{')) => buf.push_str("{{"),
                Some((_, '}')) => buf.push_str("{}"),
                Some((i, ch)) => {
                    buf = match parse_param(ch, i, s, chars, buf) {
                        Ok(s) => s,
                        Err(s) => return s,
                    }
                }
                // unclosed bracket, compiler will let user know of error
                None => buf.push('{'),
            },
            '}' => match chars.next() {
                Some((_, '}')) => buf.push_str("}}"),
                // ignores invalid bracket, continues parsing
                // compiler will let user know of error
                _ => buf.push('}'),
            },
            ch => buf.push(ch),
        }
        next = chars.next();
    }
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
#[allow(clippy::cast_possible_wrap)]
pub fn parse_string(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let chars = &mut s.char_indices();
    let mut next = chars.next();
    'outer: while let Some((_, ch)) = next {
        match ch {
            // unwrap cannot fail, in the case that it does something is very wrong
            '\\' => match chars.next().unwrap().1 {
                //quote escapes
                '\'' => buf.push('\''),
                '"' => buf.push('"'),
                //ascii escapes
                'x' => buf.push(parse_7bit(chars, s).expect("Invalid escape, see compiler error")),
                'n' => buf.push('\n'),
                'r' => buf.push('\r'),
                't' => buf.push('\t'),
                '\\' => buf.push('\\'),
                '0' => buf.push('\0'),
                //unicode escape
                'u' => buf.push(parse_24bit(chars, s).expect("Invalid escape, see compiler error")),
                //whitespace ignore
                '\n' => {
                    for (i, c) in chars.by_ref() {
                        let (' ' | '\n' | '\r' | '\t') = c else {
                            next = Some((i,c));
                            continue 'outer; // skip calling: next = chars.next();
                        };
                    }
                    // end of string reached
                }
                ch => panic!("Invalid escape '\\{ch}', see compile error"), // invalid char
            },
            '{' => match chars.next() {
                Some((_, '{')) => buf.push_str("{{"),
                Some((_, '}')) => buf.push_str("{}"),
                Some((i, ch)) => {
                    buf = match parse_param(ch, i, s, chars, buf) {
                        Ok(s) => s,
                        Err(s) => return s,
                    }
                }
                // unclosed bracket, compiler will let user know of error
                None => buf.push('{'),
            },
            '}' => match chars.next() {
                Some((_, '}')) => buf.push_str("}}"),
                // ignores invalid bracket, continues parsing
                // compiler will let user know of error
                _ => buf.push('}'),
            },
            ch => buf.push(ch),
        }
        next = chars.next();
    }
    buf
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
    mut ch: char,
    mut i: usize,
    s: &str,
    chars: &mut CharIndices,
    mut buf: String,
) -> Result<String, String> {
    let mut close_found = false;
    let mut output = None;
    // ch is the delimiter, if not + | - | # it is a var/format param
    match ch {
        '+' | '-' | '#' => (),
        _ => {
            let start = i;
            let Some((end, next_ch)) = find_delimiter(chars, &mut close_found) else {
                // ignores invalid bracket, add rest of string to buf
                // compiler will let user know of error
                return Err(buf + &s[start-1..]);// -1 to include bracket
            };
            output = Some(&s[start..end]);
            i = end; // current end is next delimiter's index
            ch = next_ch; // current next_ch is next delimiter
        }
    }
    if !close_found {
        buf.push_str("\x1b[");
        while !close_found {
            let start = i + 1; // char at i is the delimiter, add by one to ignore it
            let Some((end, next_ch)) = find_delimiter(chars, &mut close_found) else {
                panic!("Close bracket not found")
            };
            assert!(
                // parse_sgr should append the string to the buf
                // assert! is to check that an error hasn't occurred
                parse_sgr(ch, &s[start..end], &mut buf).is_some(),
                "Invalid keyword: {}",
                &s[start..end]
            );
            buf.push(';');
            i = end; // current end is next delimiter's index
            ch = next_ch; // current next_ch is next delimiter
        }
        // unwrap cannot fail, in the case that it does something is very wrong
        buf.pop().unwrap(); // removes last ';'
        buf.push('m');
    }
    if let Some(output) = output {
        buf.push('{');
        buf.push_str(output);
        buf.push('}');
    }
    Ok(buf)
}
/// Finds next valid delimiter
#[inline]
fn find_delimiter(chars: &mut CharIndices, close_found: &mut bool) -> Option<(usize, char)> {
    chars.find(|(_, c)| match c {
        '+' | '-' | '#' => true,
        '}' => {
            *close_found = true;
            true
        }
        _ => false,
    })
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
    let (end, _) = chars.find(|c| c.1 == '}')?;
    char::from_u32(u32::from_str_radix(&s[start..end], 16).ok()?)
}
fn parse_sgr(ch: char, s: &str, buf: &mut String) -> Option<()> {
    match ch {
        '+' => {
            parse_add_style(s)?.append_to(buf);
            Some(())
        }
        '-' => {
            parse_sub_style(s)?.append_to(buf);
            Some(())
        }
        '#' => parse_color(s, buf),
        _ => None,
    }
}
fn parse_add_style(s: &str) -> Option<u8> {
    match s {
        "Reset" => Some(0),
        "Bold" => Some(1),
        "Dim" => Some(2),
        "Italic" => Some(3),
        "Underline" => Some(4),
        "Blinking" => Some(5),
        "Inverse" => Some(7),
        "Hidden" => Some(8),
        "Strikethrough" => Some(9),
        _ => None,
    }
}
fn parse_sub_style(s: &str) -> Option<u8> {
    match s {
        "Bold" | "Dim" => Some(22),
        "Italic" => Some(23),
        "Underline" => Some(24),
        "Blinking" => Some(25),
        "Inverse" => Some(27),
        "Hidden" => Some(28),
        "Strikethrough" => Some(29),
        _ => None,
    }
}
fn parse_color(s: &str, buf: &mut String) -> Option<()> {
    #[inline]
    fn parse_color_simple(s: &str) -> Option<u8> {
        match s {
            "BlackFg" => Some(30),
            "RedFg" => Some(31),
            "GreenFg" => Some(32),
            "YellowFg" => Some(33),
            "BlueFg" => Some(34),
            "MagentaFg" => Some(35),
            "CyanFg" => Some(36),
            "WhiteFg" => Some(37),
            "DefaultFg" => Some(39),
            "BlackBg" => Some(40),
            "RedBg" => Some(41),
            "GreenBg" => Some(42),
            "YellowBg" => Some(43),
            "BlueBg" => Some(44),
            "MagentaBg" => Some(45),
            "CyanBg" => Some(46),
            "WhiteBg" => Some(47),
            "DefaultBg" => Some(49),
            _ => None,
        }
    }
    if let Some(n) = parse_color_simple(s) {
        n.append_to(buf);
    } else {
        let mut chars = s.chars();
        match chars.next()? {
            'f' => buf.push_str("38;"),
            'b' => buf.push_str("48;"),
            _ => return None,
        }
        let (left, right) = (chars.next()?, chars.next_back()?);
        // x[..] -> ..
        let s = &s[2..s.as_bytes().len() - 1];
        match (left, right) {
            ('(', ')') => {
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
            ('[', ']') => match s.len() {
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
            },
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
