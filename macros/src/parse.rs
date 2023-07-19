// pub(super) fn parse_string(src: String) -> String {
//     parse_literal(&src).and_then(parse_inner).unwrap_or(src)
// }
pub(super) fn parse_literal(s: &str) -> Option<&str> {
    // s.strip_prefix('r')
    //     .map_or(s, |s| s.trim_matches('#'))
    //     .strip_prefix('"')
    //     .and_then(|s| s.strip_suffix('"'))
    //     .ok_or(s)

    s.strip_prefix('"')?.strip_suffix('"')
}
pub(super) fn parse_string(s: &str) -> Option<String> {
    let mut buf = String::with_capacity(s.len()); // most likely too much capacity
    let mut sgr_buf = String::new();
    let chars = &mut s.chars();
    let mut next = chars.next();
    'outer: while let Some(ch) = next {
        match ch {
            '\\' => match chars.next()? {
                //quote escapes
                '\'' => buf.push('\''),
                '"' => buf.push('"'),
                //ascii escapes
                'x' => buf.push(parse_7bit(chars)?),
                'n' => buf.push('\n'),
                'r' => buf.push('\r'),
                't' => buf.push('\t'),
                '\\' => buf.push('\\'),
                '0' => buf.push('\0'),
                //unicode escape
                'u' => buf.push(parse_24bit(chars)?),
                //whitespace ignore
                '\n' => {
                    for c in chars.by_ref() {
                        let (' ' | '\n' | '\r' | '\t') = c else {
                            next = Some(c);
                            continue 'outer;
                        };
                    }
                    // end of string reached
                }
                _ => return None, // invalid char
            },
            '{' => match chars.next()? {
                '{' => buf.push_str("{{"),
                '}' => buf.push_str("{}"),
                '!' => {
                    chars
                        .by_ref()
                        .take_while(|ch| ch != &'}')
                        .for_each(|ch| sgr_buf.push(ch));
                    match parse_sgr(&sgr_buf) {
                        Some(string) => {
                            buf.push_str("\x1b[");
                            buf.push_str(string);
                            buf.push('m');
                        }
                        None => {
                            buf.push('{');
                            buf.push_str(&sgr_buf);
                            buf.push('}');
                        }
                    }
                    sgr_buf.clear()
                }
                ch => {
                    buf.push('{');
                    buf.push(ch);
                    chars
                        .by_ref()
                        .take_while(|ch| ch != &'}')
                        .for_each(|ch| buf.push(ch));
                    buf.push('}');
                }
            },
            '}' => match chars.next()? {
                '}' => buf.push_str("}}"),
                _ => buf.push('}'),
            },
            c => buf.push(c),
        }
        next = chars.next();
    }
    Some(buf)
}

// fn parse_char(buf: &mut str) {}

fn parse_7bit(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    let mut src = String::with_capacity(2);
    src.push(chars.next()?);
    src.push(chars.next()?);

    char::from_u32(u32::from_str_radix(&src, 16).ok()?)
}
fn parse_24bit(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    chars.next()?;
    let src: String = chars.take_while(|&c| c != '}').collect();

    char::from_u32(u32::from_str_radix(&src, 16).ok()?)
}
fn parse_sgr(s: &str) -> Option<&str> {
    match s {
        "Reset" => Some("0"),
        "Bold" => Some("1"),
        "Dim" => Some("2"),
        "Italic" => Some("3"),
        "Underline" => Some("4"),
        "Blinking" => Some("5"),
        "Inverse" => Some("7"),
        "Hidden" => Some("8"),
        "Strikethrough" => Some("9"),
        "NotBold" => Some("22"),
        "NotDim" => Some("22"),
        "NotItalic" => Some("23"),
        "NotUnderline" => Some("24"),
        "NotBlinking" => Some("25"),
        "NotInverse" => Some("27"),
        "NotHidden" => Some("28"),
        "NotStrikethrough" => Some("29"),
        "BlackFg" => Some("30"),
        "RedFg" => Some("31"),
        "GreenFg" => Some("32"),
        "YellowFg" => Some("33"),
        "BlueFg" => Some("34"),
        "MagentaFg" => Some("35"),
        "CyanFg" => Some("36"),
        "WhiteFg" => Some("37"),
        "DefaultFg" => Some("38"),
        "BlackBg" => Some("40"),
        "RedBg" => Some("41"),
        "GreenBg" => Some("42"),
        "YellowBg" => Some("43"),
        "BlueBg" => Some("44"),
        "MagentaBg" => Some("45"),
        "CyanBg" => Some("46"),
        "WhiteBg" => Some("47"),
        "DefaultBg" => Some("48"),
        _ => None,
    }
}
