use std::io::{BufWriter, Write};

use crate::graphics::{inline::InlineSGR, SGRString};

/// A writer built with SGR code integration
///
/// Provides a set of functions to make writing SGR codes easier
pub trait SGRWriter: Sized /*W*/ {
    /// The type of error returned by trait methods
    ///
    /// Will typically be [`std::io::Error`] or [`std::fmt::Error`]
    type Error: std::error::Error;
    /// Writes a code to the inner writer
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    fn write_code(&mut self, code: u8) -> Result<(), Self::Error>;
    /// Writes a str to the inner writer
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    fn write_inner<'a>(&mut self, string: impl Into<&'a str>) -> Result<(), Self::Error>;
    /// Writes the SGR sequence starting characters '\x1b['
    ///
    /// Uses [`SGRWriter::write_inner`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    fn escape(&mut self) -> Result<(), Self::Error> {
        self.write_inner("\x1b[")
    }
    /// Writes the SGR sequence ending character 'm'
    ///
    /// Uses [`SGRWriter::write_inner`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    fn end(&mut self) -> Result<(), Self::Error> {
        self.write_inner("m")
    }
    /// Writes a set of codes throught calling [`SGRWriter::write_code`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    fn write_multiple(&mut self, codes: &[u8]) -> Result<(), Self::Error> {
        codes.iter().try_for_each(|code| self.write_code(*code))
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::place`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    fn place_sgr(&mut self, sgr: &SGRString) -> Result<(), Self::Error> {
        sgr.place(self)
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::clean`]
    ///
    /// Supposed to reverse the effects made by [`SGRString::place`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    fn clean_sgr(&mut self, sgr: &SGRString) -> Result<(), Self::Error> {
        sgr.clean(self)
    }
    /// Writes the contained SGR codes to the writer throught calling [`InlineSGR::write`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    fn inline_sgr(&mut self, sgr: &impl InlineSGR) -> Result<(), Self::Error> {
        sgr.write(self)
    }
}
/// [`SGRWriter`] for [`std::fmt::Write`]
#[derive(Debug)]
pub struct FmtWriter<W: std::fmt::Write> {
    /// The internal writer
    pub writer: W,
    first_write: bool,
}
impl<W: std::fmt::Write> FmtWriter<W> {
    /// Creates a writer with the given [`std::fmt::Write`]
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
impl<W: std::fmt::Write> SGRWriter for FmtWriter<W> {
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
/// [`SGRWriter`] for [`std::io::Write`]
#[derive(Debug)]
pub struct IoWriter<W: std::io::Write> {
    /// The internal writer
    pub writer: W,
    first_write: bool,
}
impl<W: std::io::Write> IoWriter<W> {
    /// Creates a writer with the given [`std::io::Write`]
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
impl<W: std::io::Write> SGRWriter for IoWriter<W> {
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
/// A [`BufWriter`] implementation with [`SGRWriter`] functionality
#[derive(Debug)]
pub struct SGRBufWriter<W: std::io::Write> {
    /// The internal writer
    pub writer: BufWriter<W>,
    first_write: bool,
}
impl<W: std::io::Write> std::io::Write for SGRBufWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
impl<W: std::io::Write> SGRWriter for SGRBufWriter<W> {
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
