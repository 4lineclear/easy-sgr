// pub fn condense<'a>(text: &'a str) -> Result<String, std::fmt::Error> {
//     let bytes = text.as_bytes();
//     let mut new_text = String::with_capacity(bytes.len());
//     let mut search_step = SearchType::FindEscape1;

//     for b in bytes {
//         new_text.write_char(*b as char)?;
//         search_step = match search_step {
//             SearchType::FindEscape1 if *b == b'\x1b' => SearchType::FindEscape2,
//             SearchType::FindEscape2 if *b == b'[' => SearchType::Escaped,
//             SearchType::Escaped => match b {
//                 b'0'..=b'9' => search_step,
//                 b';' => search_step,
//                 _ => return Err(std::fmt::Error::default()),
//             },
//             _ => search_step,
//         }
//     }

//     Ok(new_text)
// }
// #[macro_export]
// macro_rules! format_ansi {
//     ($fmt:expr $(,$arg:tt)+) => {{
//         use $crate::{AnsiString, writer::Ansi};
//         trait GetAnsi : Sized{
//             fn get_ansi<A: Ansi>(&self) -> Option<&Self> {
//                 Some(self)
//             }
//         }
//         impl GetAnsi for AnsiString {}
//         trait GetOther : Sized{
//             fn get_other(&self) -> Option<Self> {
//                 None
//             }
//         }
//         impl<T> GetOther for T {}

//         format!($fmt $(,$arg)*)
//     }};
// }

// #[derive(Debug, Default, Clone)]
// pub struct AnsiAggregate {
//     pub strings: Vec<AnsiString>,
// }

// pub fn parse<'a>(src: &[u8]) -> Result<AnsiAggregate, ParseError> {
//     let mut code_buffer = 0u8;

//     let mut start = 0;

//     let mut search_type = SearchType::FindEscape1;
//     let mut codes = Vec::new();

//     let mut ansi_aggregate = AnsiAggregate::default();

//     for (i, b) in src.into_iter().enumerate() {
//         match search_type {
//             SearchType::FindEscape1 if *b == b'\x1b' => search_type = SearchType::FindEscape2,
//             SearchType::FindEscape2 if *b == b'[' => search_type = SearchType::Escaped,
//             SearchType::Escaped => match *b {
//                 b'0'..=b'9' => {
//                     code_buffer = match code_buffer.checked_add(*b - b'0') {
//                         Some(n) => n,
//                         None => return Err(ParseError::Overflow),
//                     };
//                 }
//                 b';' => {
//                     codes.push(code_buffer);
//                     code_buffer = 0;
//                 }
//                 b'm' => {
//                     codes.push(code_buffer);

//                     ansi_aggregate
//                         .strings
//                         .push(string_from_parts(&src[start..i], &codes));

//                     code_buffer = 0;
//                     start = i + 1;
//                     search_type = SearchType::FindEscape1;
//                 }
//                 _ => return Err(ParseError::NonDigit),
//             },
//             _ => (),
//         }
//     }
//     Ok(AnsiAggregate::default())
// }

// enum SearchType {
//     FindEscape1,
//     FindEscape2,
//     Escaped,
// }

// fn string_from_parts(string: &[u8], codes: &[u8]) -> AnsiString {
//     let mut graphics = Graphics::default();
//     for code in codes {
//         match code {
//             0 => graphics.reset = true,
//             1 => graphics.bold = true,
//             2 => graphics.dim = true,
//             3 => graphics.italic = true,
//             4 => graphics.underline = true,
//             5 => graphics.blinking = true,
//             7 => graphics.inverse = true,
//             8 => graphics.hidden = true,
//             9 => graphics.strikethrough = true,
//             _ => ()
//         }
//     }
//     AnsiString::default()
// }

// #[derive(Debug, Error)]
// pub enum ParseError {
//     #[error("Overflowed, number too high")]
//     Overflow,
//     #[error("Not a digit")]
//     NonDigit,
// }

// #[inline]
// pub fn parse_u8(src: &[u8]) -> Option<u8> {
//     match src.len() {
//         1 => match src[0] {
//             b'0'..=b'9' => Some(src[0] - b'0'),
//             _ => return None,
//         },
//         2 => match (src[0], src[1]) {
//             (b'0'..=b'9', b'0'..=b'9') => Some((src[0] - b'0') * 10 + (src[1] - b'0')),
//             _ => return None,
//         },
//         3 => match (src[0], src[1], src[2]) {
//             (b'0', b'0'..=b'9', b'0'..=b'9') => Some((src[1] - b'0') * 10 + (src[2] - b'0')),
//             (b'1', b'0'..=b'9', b'0'..=b'9') => Some(100 + (src[1] - b'0') * 10 + (src[2] - b'0')),
//             (b'2', b'0'..=b'4', b'0'..=b'9') => Some(200 + (src[1] - b'0') * 10 + (src[2] - b'0')),
//             (b'2', b'5', b'0'..=b'5') => Some(250 + (src[2] - b'0')),
//             _ => return None,
//         },
//         0 | _ => None,
//     }
// }
