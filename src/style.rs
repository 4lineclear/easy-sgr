use crate::{
    graphics::Graphics,
    writer::{AnsiFmt, AnsiWriter},
};

#[derive(Debug, Default, Clone, Copy)]
pub enum Style {
    #[default]
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blinking,
    Inverse,
    Hidden,
    Strikethrough,

    ClearBold,
    ClearDim,
    ClearItalic,
    ClearUnderline,
    ClearBlinking,
    ClearInverse,
    ClearHidden,
    ClearStrikethrough,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Style::*;
        let mut fmt = AnsiFmt::new(f);
        fmt.escape()?;

        fmt.write_code(match self {
            Reset => 0,
            Bold => 1,
            Dim => 2,
            Italic => 3,
            Underline => 4,
            Blinking => 5,
            Inverse => 7,
            Hidden => 8,
            Strikethrough => 9,

            ClearBold => 22,
            ClearDim => 22,
            ClearItalic => 23,
            ClearUnderline => 24,
            ClearBlinking => 25,
            ClearInverse => 27,
            ClearHidden => 28,
            ClearStrikethrough => 29,
        })?;
        fmt.end()
    }
}

impl Style {
    pub fn style(self, other: Self) -> Graphics {
        Graphics::default().style(self).style(other)
    }
}
