use crate::graphics::{display::InlineAnsi, AnsiString};

pub struct FmtWriter<W: std::fmt::Write> {
    writer: W,
    first_write: bool,
}

impl<W: std::fmt::Write> FmtWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            first_write: true,
        }
    }
}

impl<W: std::fmt::Write> std::fmt::Write for FmtWriter<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.writer.write_str(s)
    }
}

impl<W: std::fmt::Write> AnsiWriter for FmtWriter<W> {
    type Error = std::fmt::Error;

    fn write_code(&mut self, code: u8) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        if self.first_write {
            self.first_write = false;
        } else {
            self.writer.write_char(';')?;
        }
        self.writer.write_str(&code.to_string())
    }

    fn write_inner<'a>(&mut self, string: impl Into<&'a str>) -> Result<(), Self::Error> {
        self.writer.write_str(string.into())
    }
}

pub struct IoWriter<W: std::io::Write> {
    writer: W,
    first_write: bool,
}

impl<W: std::io::Write> IoWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            first_write: true,
        }
    }
}

impl<W: std::io::Write> std::io::Write for IoWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: std::io::Write> AnsiWriter for IoWriter<W> {
    type Error = std::io::Error;

    fn write_code(&mut self, code: u8) -> Result<(), Self::Error> {
        if self.first_write {
            self.first_write = false;
        } else {
            self.writer.write_all(b";")?;
        }
        self.writer.write_all(code.to_string().as_bytes())
    }

    fn write_inner<'a>(&mut self, string: impl Into<&'a str>) -> Result<(), Self::Error> {
        self.writer.write_all(string.into().as_bytes())
    }
}

pub trait AnsiWriter: Sized /*W*/ {
    /// The type of error returned by trait methods
    ///
    /// Will typically be [`std::io::Error`] or [`std::fmt::Error`]
    type Error: std::error::Error;
    /// Writes a code to the inner writer
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`AnsiWriter::Error`]
    fn write_code(&mut self, code: u8) -> Result<(), Self::Error>;
    /// Writes a str to the inner writer
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`AnsiWriter::Error`]
    fn write_inner<'a>(&mut self, string: impl Into<&'a str>) -> Result<(), Self::Error>;
    /// Writes the ansi sequence starting characters '\x1b['
    ///
    /// Uses [`AnsiWriter::write_inner`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`AnsiWriter::Error`]
    fn escape(&mut self) -> Result<(), Self::Error> {
        self.write_inner("\x1b[")
    }
    /// Writes the ansi sequence ending character 'm'
    ///
    /// Uses [`AnsiWriter::write_inner`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`AnsiWriter::Error`]
    fn end(&mut self) -> Result<(), Self::Error> {
        self.write_inner("m")
    }
    /// Writes a set of codes throught calling [`AnsiWriter::write_code`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`AnsiWriter::Error`]
    fn write_multiple(&mut self, codes: &[u8]) -> Result<(), Self::Error> {
        codes.iter().try_for_each(|code| self.write_code(*code))
    }
    /// Writes the contained ANSI codes to the writer through calling [`AnsiString::place`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`AnsiWriter::Error`]
    fn place_ansi(&mut self, ansi: &AnsiString) -> Result<(), Self::Error> {
        ansi.place(self)
    }
    /// Writes the contained ANSI codes to the writer through calling [`AnsiString::clean`]
    ///
    /// Supposed to reverse the effects made by [`AnsiString::place`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`AnsiWriter::Error`]
    fn clean_ansi(&mut self, ansi: &AnsiString) -> Result<(), Self::Error> {
        ansi.clean(self)
    }
    /// Writes the contained ANSI codes to the writer throught calling [`InlineAnsi::write`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`AnsiWriter::Error`]
    fn inline_ansi(&mut self, ansi: &impl InlineAnsi) -> Result<(), Self::Error> {
        ansi.write(self)
    }
}
