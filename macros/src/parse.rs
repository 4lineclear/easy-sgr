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
pub fn parse_string(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let chars = &mut s.char_indices();
    let mut next = chars.next();
    'outer: while let Some((_, ch)) = next {
        match ch {
            // cannot fail, in the case that it does something is very wrong
            '\\' => match chars.next().unwrap() {
                //quote escapes
                (_, '\'') => buf.push('\''),
                (_, '"') => buf.push('"'),
                //ascii escapes
                (i, 'x') => {
                    buf.push(parse_7bit(i, chars, s).expect("Invalid escape, see compiler error"));
                }
                (_, 'n') => buf.push('\n'),
                (_, 'r') => buf.push('\r'),
                (_, 't') => buf.push('\t'),
                (_, '\\') => buf.push('\\'),
                (_, '0') => buf.push('\0'),
                //unicode escape
                (_, 'u') => {
                    buf.push(parse_24bit(chars, s).expect("Invalid escape, see compiler error"));
                }
                //whitespace ignore
                (_, '\n') => {
                    for (_, c) in chars.by_ref() {
                        let (' ' | '\n' | '\r' | '\t') = c else {
                            continue 'outer;
                        };
                    }
                    // end of string reached
                }
                (_, ch) => panic!(" Invalid escape '\\{ch}', see compile error"), // invalid char
            },
            '{' => match chars.next() {
                Some((_, '{')) => buf.push_str("{{"),
                Some((_, '}')) => buf.push_str("{}"),
                Some((mut i, mut ch)) => {
                    let mut close_found = false;
                    let mut output = None;
                    // ch is the delimiter, if not + | - | # it is a var/format param
                    match ch {
                        '+' | '-' | '#' => (),
                        _ => {
                            let start = i;
                            let Some((end, next_ch)) = find_delimiter(chars, &mut close_found) else {
                                buf.push_str(&s[start-1..]);
                                return buf;
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
                                panic!("Bracket close not found")
                            };
                            let Some(_) = parse_sgr(ch, &s[start..end], &mut buf) else{
                                panic!("Invalid keyword: {}", &s[start..end])
                            };
                            buf.push(';');
                            i = end; // current end is next delimiter's index
                            ch = next_ch; // current next_ch is next delimiter
                        }
                        // cannot fail, in the case that it does something is very wrong
                        buf.pop().unwrap(); // remove last ';'
                        buf.push('m');
                    }
                    if let Some(output) = output {
                        buf.push('{');
                        buf.push_str(output);
                        buf.push('}');
                    }
                }
                // compiler will let user know of error
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

fn parse_7bit(i: usize, chars: &mut CharIndices, s: &str) -> Option<char> {
    let start = i + 1;
    let (end, _) = chars.nth(1)?;
    char::from_u32(u32::from_str_radix(&s[start..=end], 16).ok()?)
}
fn parse_24bit(chars: &mut CharIndices, s: &str) -> Option<char> {
    let (start, _) = chars.nth(1)?;
    let (end, _) = chars.find(|c| c.1 == '}')?;
    char::from_u32(u32::from_str_radix(&s[start..end], 16).ok()?)
}
fn parse_sgr(ch: char, s: &str, buf: &mut String) -> Option<()> {
    match ch {
        '+' => {
            buf.push_str(&parse_add_style(s)?.to_string());
            Some(())
        }
        '-' => {
            buf.push_str(&parse_sub_style(s)?.to_string());
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
        buf.push_str(&n.to_string());
    } else {
        let mut chars = s.chars();
        match chars.next()? {
            'f' => buf.push_str("38;"),
            'b' => buf.push_str("48;"),
            _ => return None,
        }
        if chars.next()? == '(' && chars.next_back()? == ')' {
            let parts = s[2..s.as_bytes().len() - 1]
                .split(',')
                .map(std::str::FromStr::from_str)
                .collect::<Result<Vec<u8>, _>>()
                .ok()?;
            match parts[..] {
                [n] => {
                    buf.push_str("5;");
                    buf.push_str(&n.to_string());
                }
                [n1, n2, n3] => {
                    buf.push_str("2;");
                    buf.push_str(&n1.to_string());
                    buf.push(';');
                    buf.push_str(&n2.to_string());
                    buf.push(';');
                    buf.push_str(&n3.to_string());
                }
                _ => return None,
            };
        } else {
            return None;
        }
    }
    Some(())
}
