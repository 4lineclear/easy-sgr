pub fn parse_literal(s: &str) -> Option<&str> {
    // s.strip_prefix('r')
    //     .map_or(s, |s| s.trim_matches('#'))
    //     .strip_prefix('"')
    //     .and_then(|s| s.strip_suffix('"'))
    //     .ok_or(s)

    s.strip_prefix('"')?.strip_suffix('"')
}
struct Bytes<'a> {
    bytes: &'a [u8],
    i: usize,
}

// impl<'a> std::ops::Index<usize> for Bytes<'a> {
//     type Output = u8;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.bytes[index]
//     }
// }

impl<'a> Bytes<'a> {
    const fn current(&self) -> u8 {
        self.bytes[self.i]
    }
    const fn remaining(&self) -> bool {
        self.i < self.bytes.len()
    }
    fn next(&mut self) -> Option<u8> {
        self.i += 1;
        // SAFETY: index is checked
        if self.i < self.bytes.len() {
            unsafe { Some(*self.bytes.get_unchecked(self.i)) }
        } else {
            None
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
pub fn parse_string(s: &str) -> Option<String> {
    let mut buf = String::with_capacity(s.len()); // most likely too much capacity
    let bytes = &mut Bytes {
        bytes: s.as_bytes(),
        i: 0,
    };
    'outer: while bytes.remaining() {
        match bytes.current() {
            b'\\' => match bytes.next()? {
                //quote escapes
                b'\'' => buf.push('\''),
                b'"' => buf.push('"'),
                //ascii escapes
                b'x' => buf.push(parse_7bit(bytes, s)?),
                b'n' => buf.push('\n'),
                b'r' => buf.push('\r'),
                b't' => buf.push('\t'),
                b'\\' => buf.push('\\'),
                b'0' => buf.push('\0'),
                //unicode escape
                b'u' => buf.push(parse_24bit(bytes, s)?),
                //whitespace ignore
                b'\n' => {
                    while let Some(b) = bytes.next() {
                        let (b' ' | b'\n' | b'\r' | b'\t') = b else {
                            continue 'outer;
                        };
                    }
                    // end of string reached
                }
                _ => return None, // invalid char
            },
            b'{' => match bytes.next()? {
                b'{' => buf.push_str("{{"),
                b'}' => buf.push_str("{}"),
                ch => {
                    let mut close_found = false;
                    let mut output = None;
                    match ch {
                        b'+' | b'-' | b'#' => (),
                        _ => {
                            let start = bytes.i;
                            until_next(bytes, &mut close_found);
                            output = Some(&s[start..bytes.i]);
                        }
                    }
                    if !close_found {
                        buf.push_str("\x1b[");
                        while !close_found {
                            let delim = bytes.i;
                            let start = bytes.i + 1;
                            until_next(bytes, &mut close_found);
                            buf.push_str(
                                &parse_sgr(bytes.bytes[delim], &s[start..bytes.i])?.to_string(),
                            );
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
            b'}' => match bytes.next()? {
                b'}' => buf.push_str("}}"),
                _ => buf.push('}'),
            },
            b => {
                if (b as i8) >= -0x40 {
                    buf.push(b as char);
                } else {
                    let start = bytes.i - 1;
                    while let Some(c) = bytes.next() {
                        if (c as i8) >= -0x40 {
                            bytes.i -= 1;
                            break;
                        }
                    }
                    buf.pop()?;
                    buf.push_str(std::str::from_utf8(&bytes.bytes[start..=bytes.i]).ok()?);
                }
            }
        }
        bytes.i += 1;
    }
    Some(buf)
}
#[inline]
fn until_next(bytes: &mut Bytes, close_found: &mut bool) {
    while let Some(c) = bytes.next() {
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

fn parse_7bit(bytes: &mut Bytes, s: &str) -> Option<char> {
    let start = bytes.i + 1;
    bytes.i += 2;
    char::from_u32(u32::from_str_radix(dbg!(&s[start..=bytes.i]), 16).ok()?)
}
fn parse_24bit(bytes: &mut Bytes, s: &str) -> Option<char> {
    bytes.i += 2;
    let start = bytes.i;
    while let Some(c) = bytes.next() {
        if c == b'}' {
            break;
        }
    }
    char::from_u32(u32::from_str_radix(&s[start..bytes.i], 16).ok()?)
}
fn parse_sgr(ch: u8, s: &str) -> Option<u8> {
    match ch {
        b'+' => parse_add_style(s),
        b'-' => parse_sub_style(s),
        b'#' => parse_color(s),
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
