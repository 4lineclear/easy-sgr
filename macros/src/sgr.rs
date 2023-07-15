use std::fmt::{Debug, Display};

pub fn sgrargs<'a>(source: &'a str) -> Option<SgrArgs> {
    let args: Vec<_> = get_spans(&mut source.bytes().enumerate())?
        .iter()
        .map(|s| (*s, &source[(s.start + 1)..s.end]))
        .map(SgrArg::try_from)
        .collect::<Result<_, _>>()
        .ok()?;
    let mut pieces = Vec::with_capacity(args.len() + 1);
    if args.len() > 0 {
        let mut current = args[0].span;
        pieces.push(&source[..current.start]);
        for arg in &args[1..] {
            pieces.push(&source[(current.end + 1)..arg.span.start]);
            current = arg.span;
        }
        pieces.push(&source[(current.end + 1)..]);

        // "| |"
        // " | |"
        // "| | "
        // " | | "
    } else {
        pieces.push(source)
    }
    // for arg in &args {
    //     if arg.
    // }
    Some(SgrArgs { pieces, args })
}

fn get_spans(bytes: &mut impl Iterator<Item = (usize, u8)>) -> Option<Vec<Span>> {
    let mut spans = Vec::new();
    while let Some((i, ch)) = bytes.next() {
        if ch == b'{' {
            while let Some((i2, ch)) = bytes.next() {
                if ch == b'}' {
                    spans.push(Span { start: i, end: i2 });
                    break;
                }
            }
        }
    }
    Some(spans)
}

#[derive(Debug)]
pub struct SgrArgs<'a> {
    // Format string pieces to print.
    pieces: Vec<&'a str>,

    args: Vec<SgrArg>,
}
impl<'a> Display for SgrArgs<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(&self, f)
    }
}
#[derive(Debug)]
pub struct SgrArg {
    span: Span,
    kind: ArgKind,
}

#[derive(Debug)]
pub enum ArgKind {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blinking,
    Inverse,
    Hidden,
    Strikethrough,
    NotBold,
    NotDim,
    NotItalic,
    NotUnderline,
    NotBlinking,
    NotInverse,
    NotHidden,
    NotStrikethrough,
    BlackFg,
    RedFg,
    GreenFg,
    YellowFg,
    BlueFg,
    MagentaFg,
    CyanFg,
    WhiteFg,
    DefaultFg,
    BlackBg,
    RedBg,
    GreenBg,
    YellowBg,
    BlueBg,
    MagentaBg,
    CyanBg,
    WhiteBg,
    DefaultBg,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl<'a> TryFrom<(Span, &'a str)> for SgrArg {
    type Error = ();

    fn try_from((span, s): (Span, &'a str)) -> Result<Self, Self::Error> {
        use ArgKind::*;
        dbg!(s);
        Ok(Self {
            span,
            kind: match s {
                "Reset" => Reset,
                "Bold" => Bold,
                "Dim" => Dim,
                "Italic" => Italic,
                "Underline" => Underline,
                "Blinking" => Blinking,
                "Inverse" => Inverse,
                "Hidden" => Hidden,
                "Strikethrough" => Strikethrough,
                "NotBold" => NotBold,
                "NotDim" => NotDim,
                "NotItalic" => NotItalic,
                "NotUnderline" => NotUnderline,
                "NotBlinking" => NotBlinking,
                "NotInverse" => NotInverse,
                "NotHidden" => NotHidden,
                "NotStrikethrough" => NotStrikethrough,
                "BlackFg" => BlackFg,
                "RedFg" => RedFg,
                "GreenFg" => GreenFg,
                "YellowFg" => YellowFg,
                "BlueFg" => BlueFg,
                "MagentaFg" => MagentaFg,
                "CyanFg" => CyanFg,
                "WhiteFg" => WhiteFg,
                "DefaultFg" => DefaultFg,
                "BlackBg" => BlackBg,
                "RedBg" => RedBg,
                "GreenBg" => GreenBg,
                "YellowBg" => YellowBg,
                "BlueBg" => BlueBg,
                "MagentaBg" => MagentaBg,
                "CyanBg" => CyanBg,
                "WhiteBg" => WhiteBg,
                "DefaultBg" => DefaultBg,
                _ => return Err(()),
            },
        })
    }
}
