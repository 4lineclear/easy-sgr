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
    'outer: while let Some((_, ch)) = next {
        match ch {
            '\\' => match chars.next()? {
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
                    match ch {
                        '+' | '-' | '#' => (),
                        _ => {
                            let start = i;
                            let Some((end, next_ch)) = find_delimiter(chars, &mut close_found) else {
                                buf.push_str(&s[start-1..]);
                                return Some(buf);
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
                                buf.push_str(&s[start-2..]);
                                return Some(buf);
                            };
                            let Some(_) = parse_sgr(ch, &s[start..end], &mut buf) else{
                                panic!("Invalid keyword: {}", &s[start..end])
                            };
                            buf.push(';');
                            i = end; // current end is next delimiter's index
                            ch = next_ch; // current next_ch is next delimiter
                        }
                        buf.pop()?; // remove last ';'
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
    Some(buf)
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
    match s {
        "BlackFg" => buf.push_str(&30.to_string()),
        "RedFg" => buf.push_str(&31.to_string()),
        "GreenFg" => buf.push_str(&32.to_string()),
        "YellowFg" => buf.push_str(&33.to_string()),
        "BlueFg" => buf.push_str(&34.to_string()),
        "MagentaFg" => buf.push_str(&35.to_string()),
        "CyanFg" => buf.push_str(&36.to_string()),
        "WhiteFg" => buf.push_str(&37.to_string()),
        "DefaultFg" => buf.push_str(&39.to_string()),
        "BlackBg" => buf.push_str(&40.to_string()),
        "RedBg" => buf.push_str(&41.to_string()),
        "GreenBg" => buf.push_str(&42.to_string()),
        "YellowBg" => buf.push_str(&43.to_string()),
        "BlueBg" => buf.push_str(&44.to_string()),
        "MagentaBg" => buf.push_str(&45.to_string()),
        "CyanBg" => buf.push_str(&46.to_string()),
        "WhiteBg" => buf.push_str(&47.to_string()),
        "DefaultBg" => buf.push_str(&49.to_string()),
        _ => {
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
                        buf.push_str("2;");
                        buf.push_str(&n.to_string());
                    }
                    [n1, n2, n3] => {
                        buf.push_str("5;");
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
    }
    Some(())
}
