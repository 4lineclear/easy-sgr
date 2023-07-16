use crate::form::ToTransform;

pub(super) fn parse_string(chars: &mut impl Iterator<Item = char>) -> String {
    chars.transform(parse_chars).collect()
}
fn parse_chars(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    fn inner(next: char, chars: &mut impl Iterator<Item = char>) -> Option<char> {
        match next {
            '\\' => match chars.next()? {
                //quote escapes
                '\'' => Some('\''),
                '"' => Some('"'),
                //ascii escapes
                'x' => parse_7bit(chars),
                'n' => Some('\n'),
                'r' => Some('\r'),
                't' => Some('\t'),
                '\\' => Some('\\'),
                '\0' => Some('\0'),
                //unicode escape
                'u' => parse_24bit(chars),
                //whitespace ignore
                '\n' => {
                    for c in chars.by_ref() {
                        let (' ' | '\n' | '\r' | '\t') = c else {
                            return inner(c, chars)
                        };
                    }
                    None // end of string reached
                }
                _ => None, // invalid char
            },
            '{' => match chars.next()? {
                '{' => Some('{'),
                c => Some(c),
            },
            '}' => match chars.next()? {
                '}' => Some('}'),
                c => Some(c),
            },
            c => Some(c),
        }
    }
    inner(chars.next()?, chars)
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
