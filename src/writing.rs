use std::{fmt, io};

use crate::graphics::{inline::InlineSGR, SGRString};

pub trait SGRWriter: Sized {
    type Error: std::error::Error;

    fn write_inner(&mut self, s: &str) -> Result<(), Self::Error>;
}

pub struct StandardWriter<W: SGRWriter> {
    pub writer: W,
    first_write: bool,
}

impl<W: SGRWriter> StandardWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            first_write: true,
        }
    }
    // pub fn escape<'a>(&'a mut self) -> SGRBuilder<'a, W> {
    //     SGRBuilder {
    //         writer: self,
    //         codes: Vec::new(),
    //     }
    // }
    pub fn write_code(&mut self, code: u8) -> Result<(), W::Error> {
        if !self.first_write {
            self.write_inner(";")?;
        } else {
            self.first_write = false
        }
        self.write_inner(&code.to_string())
    }
    pub fn write_multiple(&mut self, codes: &[u8]) -> Result<(), W::Error> {
        if codes.is_empty() {
            return Ok(());
        }
        self.write_code(codes[0]);
        for code in &codes[1..] {
            self.write_code(*code)?;
        }
        Ok(())
    }
    pub fn escape(&mut self) -> Result<(), W::Error> {
        self.first_write = true;
        self.write_inner("\x1b[")
    }
    pub fn end(&mut self) -> Result<(), W::Error> {
        self.write_inner("m")
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::place_all`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    pub fn place_sgr(&mut self, sgr: &SGRString) -> Result<(), W::Error> {
        sgr.place_all(self)
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::clean_all`]
    ///
    /// Supposed to reverse the effects made by [`SGRString::place_all`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    pub fn clean_sgr(&mut self, sgr: &SGRString) -> Result<(), W::Error> {
        sgr.clean_all(self)
    }
    /// Writes the contained SGR codes to the writer throught calling [`InlineSGR::write`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`SGRWriter::Error`]
    pub fn inline_sgr(&mut self, sgr: &impl InlineSGR) -> Result<(), W::Error> {
        self.write_inner("\x1b[")?;
        sgr.write(self)?;
        self.write_inner("m")
    }
}
impl<W: std::io::Write> StandardWriter<IoWriter<W>> {
    pub fn io(writer: W) -> Self {
        Self {
            writer: IoWriter(writer),
            first_write: true,
        }
    }
}
impl<W: std::fmt::Write> StandardWriter<FmtWriter<W>> {
    pub fn fmt(writer: W) -> Self {
        Self {
            writer: FmtWriter(writer),
            first_write: true,
        }
    }
}

impl<W: SGRWriter> SGRWriter for StandardWriter<W> {
    type Error = W::Error;

    fn write_inner(&mut self, s: &str) -> Result<(), Self::Error> {
        self.writer.write_inner(s)
    }
}

pub struct IoWriter<W: std::io::Write>(pub W);

impl<W: std::io::Write> SGRWriter for IoWriter<W> {
    type Error = io::Error;

    fn write_inner(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_all(s.as_bytes())
    }
}

pub struct FmtWriter<W: std::fmt::Write>(pub W);

impl<W: std::fmt::Write> SGRWriter for FmtWriter<W> {
    type Error = fmt::Error;

    fn write_inner(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_str(s)
    }
}

// pub struct SGRBuilder<'a, W: SGRWriter> {
//     writer: &'a mut StandardWriter<W>,
//     codes: Vec<u8>,
// }

// impl<'a, W: SGRWriter> SGRBuilder<'a, W> {
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
