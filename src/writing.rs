use std::{fmt, io};

use crate::graphics::{inline::InlineSGR, SGRString};

/// An interfeace for an [`StandardWriter`] to work with
///
/// Does not provide SGR writing capability itself
pub trait CapableWriter: Sized {
    /// The type of error returned by trait methods
    ///
    /// Will typically be [`std::io::Error`] or [`std::fmt::Error`]
    type Error: std::error::Error;
    /// Writes a [`str`] to the inner writer
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    fn write(&mut self, s: &str) -> Result<(), Self::Error>;
}

/// A writer built on top of [`CapableWriter`]
/// that has the ability to work with SGR codes
pub trait SGRWriter: CapableWriter {
    // pub fn escape<'a>(&'a mut self) -> SGRBuilder<'a, W> {
    //     SGRBuilder {
    //         writer: self,
    //         codes: Vec::new(),
    //     }
    // }
    /// Used to check whether the code seperator
    /// ';' should be written
    fn first_write(&self) -> bool;
    /// Sets first_write to the inputted variable
    fn set_first_write(&mut self, first_write: bool);
    /// Writes a [`str`] to the inner writer
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn write_inner(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write(s)
    }
    /// Writes a code to the inner writer
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn write_code(&mut self, code: u8) -> Result<(), Self::Error> {
        if self.first_write() {
            self.set_first_write(false);
        } else {
            self.write(";")?;
        }
        self.write(&code.to_string())
    }
    /// Writes a set of codes throught calling [`StandardWriter::write_code`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn write_multiple(&mut self, codes: &[u8]) -> Result<(), Self::Error> {
        if codes.is_empty() {
            return Ok(());
        }
        self.write_code(codes[0])?;
        for code in &codes[1..] {
            self.write_code(*code)?;
        }
        Ok(())
    }
    /// Writes the SGR sequence starting characters '\x1b['
    ///
    /// Should be eventually followed by [`StandardWriter::end`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn escape(&mut self) -> Result<(), Self::Error> {
        self.set_first_write(true);
        self.write("\x1b[")
    }
    /// Writes the SGR sequence ending character 'm'
    ///
    /// Should be used sometime after [`StandardWriter::escape`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn end(&mut self) -> Result<(), Self::Error> {
        self.write("m")
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::place_all`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn place_sgr(&mut self, sgr: &SGRString) -> Result<(), Self::Error> {
        sgr.place_all(self)
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::clean_all`]
    ///
    /// Supposed to reverse the effects made by [`SGRString::place_all`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn clean_sgr(&mut self, sgr: &SGRString) -> Result<(), Self::Error> {
        sgr.clean_all(self)
    }
    /// Writes the contained SGR codes to the writer throught calling [`InlineSGR::write`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn inline_sgr(&mut self, sgr: &impl InlineSGR) -> Result<(), Self::Error> {
        self.escape()?;
        sgr.write(self)?;
        self.end()
    }
}

/// A Standard SGR writer
#[derive(Debug)]
pub struct StandardWriter<W: CapableWriter> {
    /// A writer capable of writing [`str`]
    pub writer: W,
    /// Keeps track f whether it is currently the first write
    ///
    /// This variable is used to make sure the seperator character ';'
    /// is written when it supposed to
    first_write: bool,
}

impl<W: CapableWriter> StandardWriter<W> {
    /// Creates a new [`StandardWriter<W>`].
    ///
    /// Probably better to use [`StandardWriter::io`] or [`StandardWriter::fmt`]
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            first_write: true,
        }
    }
}
impl<W: std::io::Write> StandardWriter<IoWriter<W>> {
    /// Creates a new [`StandardWriter<W>`] with the provided [`Write`](std::io::Write)
    ///
    /// Uses [`IoWriter`]
    pub fn io(writer: W) -> Self {
        Self {
            writer: IoWriter(writer),
            first_write: true,
        }
    }
}
impl<W: std::fmt::Write> StandardWriter<FmtWriter<W>> {
    /// Creates a new [`StandardWriter<W>`] with the provided [`Write`](std::fmt::Write)
    ///
    /// Uses [`FmtWriter`]
    pub fn fmt(writer: W) -> Self {
        Self {
            writer: FmtWriter(writer),
            first_write: true,
        }
    }
}

impl<W: CapableWriter> CapableWriter for StandardWriter<W> {
    type Error = W::Error;
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.writer.write(s)
    }
}

impl<W: CapableWriter> SGRWriter for StandardWriter<W> {
    #[inline]
    fn first_write(&self) -> bool {
        self.first_write
    }
    #[inline]
    fn set_first_write(&mut self, first_write: bool) {
        self.first_write = first_write;
    }
}

/// Used to implement [`CapableWriter`] for [`Write`](std::io::Write)
#[derive(Debug)]
pub struct IoWriter<W: std::io::Write>(pub W);

impl<W: std::io::Write> CapableWriter for IoWriter<W> {
    type Error = io::Error;

    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_all(s.as_bytes())
    }
}

/// Used to implement [`CapableWriter`] for [`Write`](std::fmt::Write)
#[derive(Debug)]
pub struct FmtWriter<W: std::fmt::Write>(pub W);

impl<W: std::fmt::Write> CapableWriter for FmtWriter<W> {
    type Error = fmt::Error;

    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_str(s)
    }
}

// pub struct SGRBuilder<'a, W: CapableWriter> {
//     writer: &'a mut StandardWriter<W>,
//     codes: Vec<u8>,
// }

// impl<'a, W: CapableWriter> SGRBuilder<'a, W> {
//     pub fn write_codes(&mut self, codes: &[u8]) -> &mut Self {
//         self.codes.extend_from_slice(codes);
//         self
//     }
//     pub fn write_code(&mut self, code: u8) -> &mut Self {
//         self.codes.push(code);
//         self
//     }
//     pub fn end(&mut self) -> Result<(), W::Error> {
//         self.writer.write_multiple(&self.codes)
//     }
// }
