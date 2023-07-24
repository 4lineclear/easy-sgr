pub fn parse_literal(s: &str) -> Option<&str> {
    // s.strip_prefix('r')
    //     .map_or(s, |s| s.trim_matches('#'))
    //     .strip_prefix('"')
    //     .and_then(|s| s.strip_suffix('"'))
    //     .ok_or(s)

    s.strip_prefix('"')?.strip_suffix('"')
}
trait Next {
    fn next(&self, i: &mut usize) -> Option<&u8>;
}
impl Next for &[u8] {
    fn next(&self, i: &mut usize) -> Option<&u8> {
        *i += 1;
        self.get(*i)
    }
}
#[allow(clippy::cast_possible_wrap)]
pub fn parse_string(s: &str) -> Option<String> {
    let mut buf = String::with_capacity(s.len()); // most likely too much capacity
    let bytes = s.as_bytes();
    let i = &mut 0;
    'outer: while *i < bytes.len() {
        match bytes[*i] {
            b'\\' => match bytes.next(i)? {
                //quote escapes
                b'\'' => buf.push('\''),
                b'"' => buf.push('"'),
                //ascii escapes
                b'x' => buf.push(parse_7bit(bytes, i)?),
                b'n' => buf.push('\n'),
                b'r' => buf.push('\r'),
                b't' => buf.push('\t'),
                b'\\' => buf.push('\\'),
                b'0' => buf.push('\0'),
                //unicode escape
                b'u' => buf.push(parse_24bit(bytes, i)?),
                //whitespace ignore
                b'\n' => {
                    while let Some(c) = bytes.next(i) {
                        let (b' ' | b'\n' | b'\r' | b'\t') = c else {
                            continue 'outer;
                        };
                    }
                    // end of string reached
                }
                _ => return None, // invalid char
            },
            b'{' => match bytes.next(i)? {
                b'{' => buf.push_str("{{"),
                b'}' => buf.push_str("{}"),
                ch => {
                    let mut close_found = false;
                    let mut output = None;
                    match ch {
                        b'+' | b'-' | b'#' => (),
                        _ => {
                            let start = *i;
                            until_next(bytes, i, &mut close_found);
                            output = Some(&s[start..*i]);
                        }
                    }
                    if !close_found {
                        buf.push_str("\x1b[");
                        while !close_found {
                            let start = *i;
                            until_next(bytes, i, &mut close_found);
                            buf.push_str(&parse_sgr(bytes[start], &s[start + 1..*i])?.to_string());
                            buf.push(';');
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
            b'}' => match bytes.next(i)? {
                b'}' => buf.push_str("}}"),
                _ => buf.push('}'),
            },
            c => {
                if (c as i8) >= -0x40 {
                    buf.push(c as char);
                } else {
                    let start = *i;
                    while let Some(&c) = bytes.next(i) {
                        if (c as i8) >= -0x40 {
                            *i -= 1;
                            break;
                        }
                    }
                    buf.pop()?;
                    buf.push_str(std::str::from_utf8(&bytes[start - 1..=*i]).ok()?);
                }
            }
        }
        *i += 1;
    }
    Some(buf)
}
#[inline]
fn until_next(bytes: &[u8], i: &mut usize, close_found: &mut bool) {
    while let Some(&c) = bytes.next(i) {
        match c {
            b'+' | b'-' | b'#' => return,
            b'}' => {
                *close_found = true;
                return;
            }
            _ => (),
        }
    }
}

fn parse_7bit(chars: &[u8], i: &mut usize) -> Option<char> {
    let mut src = String::with_capacity(2);
    src.push(*chars.next(i)? as char);
    src.push(*chars.next(i)? as char);

    char::from_u32(u32::from_str_radix(&src, 16).ok()?)
}
fn parse_24bit(chars: &[u8], i: &mut usize) -> Option<char> {
    let mut src = String::new();

    chars.next(i)?;
    while let Some(&c) = chars.next(i) {
        if c == b'}' {
            break;
        }
        src.push(c as char);
    }

    char::from_u32(u32::from_str_radix(&src, 16).ok()?)
}
fn parse_sgr(ch: u8, sgr_buf: &str) -> Option<u8> {
    match ch {
        b'+' => parse_add_style(sgr_buf),
        b'-' => parse_sub_style(sgr_buf),
        b'#' => parse_color(sgr_buf),
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
        "DefaultFg" => Some(38),
        "BlackBg" => Some(40),
        "RedBg" => Some(41),
        "GreenBg" => Some(42),
        "YellowBg" => Some(43),
        "BlueBg" => Some(44),
        "MagentaBg" => Some(45),
        "CyanBg" => Some(46),
        "WhiteBg" => Some(47),
        "DefaultBg" => Some(48),
        _ => None,
    }
}
