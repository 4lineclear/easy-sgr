use crate::form::ToTransform;

/// An error that occurred while parsing
pub(super) enum ParseError {
    LendCompiler,
}
trait OrLend<T> {
    fn lend(self) -> Result<T, ParseError>;
}

impl<T> OrLend<T> for Option<T> {
    #[inline]
    fn lend(self) -> Result<T, ParseError> {
        self.ok_or(ParseError::LendCompiler)
    }
}
fn lend() -> Result<char, ParseError> {
    Err(ParseError::LendCompiler)
}

pub(super) fn parse_string(chars: &mut impl Iterator<Item = char>) -> Result<String, ParseError> {
    #[inline]
    fn helper(chars: &mut impl Iterator<Item = char>) -> Option<char> {
        parse_chars(chars).ok()
    }
    Ok(chars.transform(helper).collect())
}
fn parse_chars(chars: &mut impl Iterator<Item = char>) -> Result<char, ParseError> {
    fn inner(next: char, chars: &mut impl Iterator<Item = char>) -> Result<char, ParseError> {
        match next {
            '\\' => match chars.next().lend()? {
                //quote escapes
                '\'' => Ok('\''),
                '"' => Ok('"'),
                //ascii escapes
                'x' => parse_7bit(chars).lend(),
                'n' => Ok('\n'),
                'r' => Ok('\r'),
                't' => Ok('\t'),
                '\\' => Ok('\\'),
                '\0' => Ok('\0'),
                //unicode escape
                'u' => parse_24bit(chars).lend(),
                //whitespace ignore
                '\n' => {
                    for c in chars.by_ref() {
                        let (' ' | '\n' | '\r' | '\t') = c else {
                            return inner(c, chars)
                        };
                    }
                    lend() // end of string reached
                }
                _ => lend(), // invalid char
            },
            '{' => match chars.next().lend()? {
                '{' => Ok('{'),
                c => Ok(c),
            },
            '}' => match chars.next().lend()? {
                '}' => Ok('}'),
                c => Ok(c),
            },
            c => Ok(c),
        }
    }
    inner(chars.next().lend()?, chars)
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
