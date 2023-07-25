use std::str::CharIndices;

pub fn parse_literal(s: &str) -> Option<&str> {
    // s.strip_prefix('r')
    //     .map_or(s, |s| s.trim_matches('#'))
    //     .strip_prefix('"')
    //     .and_then(|s| s.strip_suffix('"'))
    //     .ok_or(s)

    s.strip_prefix('"')?.strip_suffix('"')
}

#[allow(clippy::cast_possible_wrap)]
pub fn parse_string(s: &str) -> Option<String> {
    let mut buf = String::with_capacity(s.len()); // most likely too much capacity
    let chars = &mut s.char_indices();
    let mut next = chars.next();
    'outer: while next.is_some() {
        match next?.1 {
            '\\' => match chars.next()? {
                //quote escapes
                (_, '\'') => buf.push('\''),
                (_, '"') => buf.push('"'),
                //ascii escapes
                (i, 'x') => buf.push(parse_7bit(i, chars, s)?),
                (_, 'n') => buf.push('\n'),
                (_, 'r') => buf.push('\r'),
                (_, 't') => buf.push('\t'),
                (_, '\\') => buf.push('\\'),
                (_, '0') => buf.push('\0'),
                //unicode escape
                (_, 'u') => buf.push(parse_24bit(chars, s)?),
                //whitespace ignore
                (_, '\n') => {
                    while let Some(b) = chars.next() {
                        let (' ' | '\n' | '\r' | '\t') = b.1 else {
                            continue 'outer;
                        };
                    }
                    // end of string reached
                }
                _ => return None, // invalid char
            },
            '{' => match chars.next()? {
                (_, '{') => buf.push_str("{{"),
                (_, '}') => buf.push_str("{}"),
                (mut i, mut ch) => {
                    let mut close_found = false;
                    let mut output = None;
                    match ch {
                        '+' | '-' | '#' => (),
                        _ => {
                            let start = i;
                            let (end, next_ch) = until_next(chars, &mut close_found)?;
                            output = Some(&s[start..end]);
                            i = end;
                            ch = next_ch;
                        }
                    }
                    if !close_found {
                        buf.push_str("\x1b[");
                        while !close_found {
                            let start = i + 1;
                            let (end, next_ch) = until_next(chars, &mut close_found)?;
                            buf.push_str(&parse_sgr(ch, &s[start..end])?.to_string());
                            buf.push(';');
                            i = end;
                            ch = next_ch;
                        }
                        buf.pop()?;
                        buf.push('m');
                    }
                    if let Some(output) = output {
                        buf.push('{');
                        buf.push_str(output);
                        buf.push('}');
                    }
                }
            },
            '}' => match chars.next()?.1 {
                '}' => buf.push_str("}}"),
                _ => buf.push('}'),
            },
            c => {
                buf.push(c as char);
            }
        }
        next = chars.next();
    }
    Some(buf)
}
#[inline]
fn until_next(chars: &mut CharIndices, close_found: &mut bool) -> Option<(usize, char)> {
    chars.find(|(_, c)| match c {
        '+' | '-' | '#' => true,
        '}' => {
            *close_found = true;
            true
        }
        _ => false,
    })
}

fn parse_7bit(i: usize, chars: &mut CharIndices, s: &str) -> Option<char> {
    let start = i + 1;
    let (end, _) = chars.nth(2)?;
    char::from_u32(u32::from_str_radix(&s[start..=end], 16).ok()?)
}
fn parse_24bit(chars: &mut CharIndices, s: &str) -> Option<char> {
    let (start, _) = chars.nth(1)?;
    let (end, _) = chars.find(|c| c.1 == '}')?;
    char::from_u32(u32::from_str_radix(&s[start..end], 16).ok()?)
}
fn parse_sgr(ch: char, s: &str) -> Option<u8> {
    match ch {
        '+' => parse_add_style(s),
        '-' => parse_sub_style(s),
        '#' => parse_color(s),
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
fn parse_color(s: &str) -> Option<u8> {
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
