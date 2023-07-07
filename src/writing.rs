use std::{fmt, io};

use crate::graphics::{inline::InlineSGR, SGRString};

/// An interfeace for an [`SGRWriter`] to work with
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
/// A writer built on top of a [`CapableWriter`]
/// that has the ability to work with SGR codes
pub trait SGRWriter: CapableWriter {
    /// Returns whether the writer can do a smart clean
    fn previous_codes(&self) -> Option<&Vec<u8>> {
        None
    }
    fn set_previous_codes(&mut self, _graphics: Vec<u8>) {}
    /// Writes a [`str`] to the inner writer
    ///
    /// Is a shortcut to [`CapableWriter::write`] without having to import it
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn write_inner(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write(s)
    }
    fn escape<'a>(&'a mut self) -> SGRBuilder<'a, Self> {
        SGRBuilder {
            writer: self,
            codes: Vec::new(),
        }
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::place_all`]
    ///
    /// Does not [Escape](SGRWriter::escape) or [End](SGRWriter::end) the sequence
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn place_sgr(&mut self, sgr: &SGRString) -> Result<(), Self::Error> {
        let mut builder = self.escape();
        sgr.place_all(&mut builder);
        builder.end()
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::clean_all`]
    ///
    /// Supposed to reverse the effects made by [`SGRString::place_all`]
    ///
    /// Does not [Escape](SGRWriter::escape) or [End](SGRWriter::end) the sequence
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn clean_sgr(&mut self, sgr: &SGRString) -> Result<(), Self::Error> {
        let mut builder = self.escape();
        sgr.clean_all(&mut builder);
        builder.end()
    }
    /// Writes the contained SGR codes to the writer throught calling [`InlineSGR::write`]
    ///
    /// [Escapes](SGRWriter::escape) and [Ends](SGRWriter::end) the sequence
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn inline_sgr(&mut self, sgr: &impl InlineSGR) -> Result<(), Self::Error> {
        let mut builder = self.escape();
        sgr.write(&mut builder);
        builder.end()
    }
}
/// A Standard SGR writer
#[derive(Debug)]
pub struct StandardWriter<W: CapableWriter> {
    /// A writer capable of writing [`str`]
    pub writer: W,
}
impl<W: CapableWriter> StandardWriter<W> {
    /// Creates a new [`StandardWriter<W>`].
    ///
    /// Probably better to use [`StandardWriter::io`] or [`StandardWriter::fmt`]
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}
impl<W: std::io::Write> StandardWriter<IoWriter<W>> {
    /// Creates a new [`StandardWriter<W>`] with the provided [`Write`](std::io::Write)
    ///
    /// Uses [`IoWriter`]
    pub fn io(writer: W) -> Self {
        Self {
            writer: IoWriter(writer),
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
impl<W: CapableWriter> SGRWriter for StandardWriter<W> {}
/// Used to implement [`CapableWriter`] for [`std::io::Write`]
#[derive(Debug)]
pub struct IoWriter<W: std::io::Write>(pub W);
impl<W: std::io::Write> CapableWriter for IoWriter<W> {
    type Error = io::Error;
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_all(s.as_bytes())
    }
}
/// Used to implement [`CapableWriter`] for [`std::fmt::Write`]
#[derive(Debug)]
pub struct FmtWriter<W: std::fmt::Write>(pub W);
impl<W: std::fmt::Write> CapableWriter for FmtWriter<W> {
    type Error = fmt::Error;
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_str(s)
    }
}
/// A more advenced [`StandardWriter`]
///
/// Has the ability to do a smart clean
#[derive(Debug)]
pub struct AdvancedWriter<W: CapableWriter> {
    /// A writer capable of writing SGR codes
    pub writer: StandardWriter<W>,
    /// The previous state of graphics
    #[allow(dead_code)]
    previous_codes: Vec<Vec<u8>>,
}
impl<W: CapableWriter> AdvancedWriter<W> {
    /// Creates a new [`AdvancedWriter<W>`].
    ///
    /// Probably better to use [`AdvancedWriter::io`] or [`AdvancedWriter::fmt`]
    pub fn new(writer: W) -> Self {
        Self {
            writer: StandardWriter::new(writer),
            previous_codes: Vec::new(),
        }
    }
}
impl<W: std::io::Write> AdvancedWriter<IoWriter<W>> {
    /// Creates a new [`AdvancedWriter<W>`] with the provided [`Write`](std::io::Write)
    ///
    /// Uses [`IoWriter`]
    pub fn io(writer: W) -> Self {
        Self {
            writer: StandardWriter::io(writer),
            previous_codes: Vec::new(),
        }
    }
}
impl<W: std::fmt::Write> AdvancedWriter<FmtWriter<W>> {
    /// Creates a new [`AdvancedWriter<W>`] with the provided [`Write`](std::fmt::Write)
    ///
    /// Uses [`FmtWriter`]
    pub fn fmt(writer: W) -> Self {
        Self {
            writer: StandardWriter::fmt(writer),
            previous_codes: Vec::new(),
        }
    }
}
impl<W: CapableWriter> CapableWriter for AdvancedWriter<W> {
    type Error = W::Error;
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.writer.write(s)
    }
}
impl<W: CapableWriter> SGRWriter for AdvancedWriter<W> {
    #[inline]
    fn previous_codes(&self) -> Option<&Vec<u8>> {
        Some(&self.previous_codes[self.previous_codes.len().checked_sub(2)?])
    }
    #[inline]
    fn set_previous_codes(&mut self, graphics: Vec<u8>) {
        self.previous_codes.push(graphics);
    }
}
#[derive(Debug)]
pub struct SGRBuilder<'a, W: SGRWriter> {
    writer: &'a mut W,
    codes: Vec<u8>,
}

impl<'a, W: SGRWriter> SGRBuilder<'a, W> {
    #[inline]
    pub fn write_code(&mut self, code: u8) {
        self.codes.push(code);
    }
    #[inline]
    pub fn write_codes(&mut self, codes: &[u8]) {
        self.codes.extend_from_slice(codes);
    }
    #[inline]
    pub fn chain_code(&mut self, code: u8) -> &mut Self {
        self.codes.push(code);
        self
    }
    #[inline]
    pub fn chain_codes(&mut self, codes: &[u8]) -> &mut Self {
        self.codes.extend_from_slice(codes);
        self
    }
    #[inline]
    pub fn smart_clean(&mut self) {
        self.codes = match self.writer.previous_codes() {
            Some(codes) if codes.len() != 0 => codes.clone(),
            _ => vec![0],
        }
    }
    pub fn end(&mut self) -> Result<(), W::Error> {
        self.writer.set_previous_codes(self.codes.clone());

        if self.codes.is_empty() {
            Ok(())
        } else {
            self.writer.write("\x1b[")?;
            self.writer.write_inner(&self.codes[0].to_string())?;

            for code in &self.codes[1..] {
                self.writer.write(";")?;
                self.writer.write(&code.to_string())?;
            }
            self.codes.clear();
            self.writer.write("m")
        }
    }
}
