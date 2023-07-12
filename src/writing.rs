use std::{fmt, io};

use crate::{DiscreteSGR, SGRString};

/// An interface for an [`SGRWriter`] to work with
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
    /// Writes a [`str`] to the inner writer
    ///
    /// A shortcut to [`CapableWriter::write`] without having to import it
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn write_inner(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write(s)
    }
    /// Returns a [`SGRBuilder`] to allow for writing SGR codes
    ///
    /// This is to be used for directly writing `SGR` codes
    #[inline]
    fn escape(&'_ mut self) -> SGRBuilder<'_, Self> {
        SGRBuilder {
            writer: self,
            codes: Vec::new(),
        }
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::place_all`]
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
    /// Writes the contained SGR codes to the writer through calling [`DiscreteSGR::write`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn inline_sgr(&mut self, sgr: &impl DiscreteSGR) -> Result<(), Self::Error> {
        let mut builder = self.escape();
        sgr.write(&mut builder);
        builder.end()
    }
    /// Writes the contained SGR codes to the writer
    ///
    /// Uses [`EasyWrite`] so the it can be used for both
    /// [`SGRString`] and [`DiscreteSGR`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    fn sgr(&mut self, sgr: &impl EasyWrite<Self>) -> Result<(), Self::Error> {
        let mut builder = self.escape();
        sgr.sgr(&mut builder);
        builder.end()
    }
    /// Writes the contained SGR codes to the writer
    ///
    /// Does not write the escape or end sequences
    ///
    /// Uses [`EasyWrite`] so the it can be used for both
    /// [`SGRString`] and [`DiscreteSGR`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    #[cfg(feature = "partial")]
    fn partial_sgr(&mut self, sgr: &impl EasyWrite<Self>) -> Result<(), Self::Error> {
        let mut builder = self.escape();
        sgr.sgr(&mut builder);
        builder.partial_end()
    }
}
/// A Standard SGR writer
#[derive(Debug, Clone)]
pub struct StandardWriter<W: CapableWriter> {
    /// A writer capable of writing a [`str`]
    pub writer: W,
}
impl<W: CapableWriter> From<W> for StandardWriter<W> {
    fn from(value: W) -> Self {
        Self { writer: value }
    }
}
impl<W: std::fmt::Write> From<W> for StandardWriter<FmtWriter<W>> {
    fn from(value: W) -> Self {
        Self {
            writer: FmtWriter(value),
        }
    }
}
impl<W: std::io::Write> From<W> for StandardWriter<IoWriter<W>> {
    fn from(value: W) -> Self {
        Self {
            writer: IoWriter(value),
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
#[derive(Debug, Clone)]
pub struct IoWriter<W: std::io::Write>(pub W);
impl<W: std::io::Write> CapableWriter for IoWriter<W> {
    type Error = io::Error;
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_all(s.as_bytes())
    }
}
/// Used to implement [`CapableWriter`] for [`std::fmt::Write`]
#[derive(Debug, Clone)]
pub struct FmtWriter<W: std::fmt::Write>(pub W);
impl<W: std::fmt::Write> CapableWriter for FmtWriter<W> {
    type Error = fmt::Error;
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_str(s)
    }
}

/// Builds an SGR sequence
#[derive(Debug)]
pub struct SGRBuilder<'a, W: SGRWriter> {
    writer: &'a mut W,
    codes: Vec<u8>,
}

impl<'a, W: SGRWriter> SGRBuilder<'a, W> {
    /// Writes a code to the internal buffer
    ///
    /// Does not perform any IO operations
    #[inline]
    pub fn write_code(&mut self, code: u8) {
        self.codes.push(code);
    }
    /// Writes codes to the internal buffer
    ///
    /// Does not perform any IO operations
    #[inline]
    pub fn write_codes(&mut self, codes: &[u8]) {
        self.codes.extend_from_slice(codes);
    }
    /// Writes a code to the internal buffer
    ///
    /// Does not perform any IO operations
    ///
    /// Returns self to allow for chaining
    #[inline]
    pub fn chain_code(&mut self, code: u8) -> &mut Self {
        self.codes.push(code);
        self
    }
    /// Writes codes to the internal buffer
    ///
    /// Does not perform any IO operations
    ///
    /// Returns self to allow for chaining
    #[inline]
    pub fn chain_codes(&mut self, codes: &[u8]) -> &mut Self {
        self.codes.extend_from_slice(codes);
        self
    }
    /// Writes buffered codes to the writer
    ///
    /// Performs IO operations with the internal [`SGRWriter`]
    ///
    /// # Errors
    ///
    /// Writing failed
    pub fn end(&mut self) -> Result<(), W::Error> {
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
    /// Writes buffered codes to the writer
    ///
    /// Does not write the escape or end sequences
    ///
    /// Performs IO operations with the internal [`SGRWriter`]
    ///
    /// # Errors
    ///
    /// Writing failed
    pub fn partial_end(&mut self) -> Result<(), W::Error> {
        if !self.codes.is_empty() {
            self.writer.write_inner(&self.codes[0].to_string())?;

            for code in &self.codes[1..] {
                self.writer.write(";")?;
                self.writer.write(&code.to_string())?;
            }
            self.codes.clear();
        }
        Ok(())
    }
}

/// Helps to make writing easier
///
/// Allows to use the same method for both
/// [`SGRString`] and [`DiscreteSGR`] types
pub trait EasyWrite<W: SGRWriter> {
    /// Writes a set of codes to the builder
    fn sgr(&self, builder: &mut SGRBuilder<W>);
}

impl<W: SGRWriter> EasyWrite<W> for SGRString {
    /// Writes a set of codes to the builder
    ///
    /// Uses [`SGRString::place_all`]
    fn sgr(&self, builder: &mut SGRBuilder<W>) {
        self.place_all(builder);
    }
}

impl<W: SGRWriter, D: DiscreteSGR> EasyWrite<W> for D {
    /// Writes a set of codes to the builder
    ///
    /// Uses [`DiscreteSGR::write`]
    fn sgr(&self, builder: &mut SGRBuilder<W>) {
        self.write(builder);
    }
}
