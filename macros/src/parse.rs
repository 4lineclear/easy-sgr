use crate::form::ToMapform;

pub(super) fn parse_string(s: &str) -> Option<String> {
    let mut buf = String::with_capacity(s.len());
    s.chars()
        .mapform(parse_chars)
        .for_each(|parsed| match parsed {
            Parsed::C(ch) => buf.push(ch),
            Parsed::S(string) => buf.push_str(&string),
            Parsed::Error => (),
        });
    Some(buf)
}
fn parse_chars(chars: &mut impl Iterator<Item = char>) -> Option<Parsed> {
    use Parsed::*;
    fn inner(next: char, chars: &mut impl Iterator<Item = char>) -> Parsed {
        match next {
            '\\' => match chars.next() {
                //quote escapes
                Some('\'') => C('\''),
                Some('"') => C('"'),
                //ascii escapes
                Some('x') => parse_7bit(chars).into(),
                Some('n') => C('\n'),
                Some('r') => C('\r'),
                Some('t') => C('\t'),
                Some('\\') => C('\\'),
                Some('\0') => C('\0'),
                //unicode escape
                Some('u') => parse_24bit(chars).into(),
                //whitespace ignore
                Some('\n') => {
                    for c in chars.by_ref() {
                        let (' ' | '\n' | '\r' | '\t') = c else {
                            return inner(c, chars)
                        };
                    }
                    Error // end of string reached
                }
                _ => Error, // invalid char
            },
            '{' => match chars.next() {
                Some('{') => S(String::from("{{")),
                Some('}') => S(String::from("{}")),
                Some(c) => C(c),
                None => Error,
            },
            '}' => match chars.next() {
                Some('}') => S(String::from("}}")),
                _ => C('}'),
            },
            c => C(c),
        }
    }
    match inner(chars.next()?, chars) {
        Error => None,
        p => Some(p),
    }
}
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

#[derive(Debug)]
enum Parsed {
    C(char),
    S(String),
    Error,
}
impl From<Option<char>> for Parsed {
    fn from(value: Option<char>) -> Self {
        match value {
            Some(c) => Self::C(c),
            None => Self::Error,
        }
    }
}
