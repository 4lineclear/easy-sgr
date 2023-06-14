use crate::{END, ESCAPE};

pub struct AnsiFmt<W: std::fmt::Write> {
    writer: W,
    first_write: bool,
}

impl<W: std::fmt::Write> AnsiFmt<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            first_write: true,
        }
    }
}

impl<W: std::fmt::Write> std::fmt::Write for AnsiFmt<W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.writer.write_str(s)
    }
}

impl<W: std::fmt::Write> AnsiWriter for AnsiFmt<W> {
    type Error = std::fmt::Error;

    fn escape(&mut self) -> Result<(), Self::Error> {
        self.first_write = true;
        self.writer.write_str(ESCAPE)
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        self.writer.write_char(END)
    }

    fn write_code(&mut self, code: u8) -> Result<(), Self::Error>
    where
        Self: Sized,
    {
        match self.first_write {
            true => self.first_write = false,
            false => self.writer.write_char(';')?,
        }
        self.writer.write_str(&code.to_string())
    }
}

pub struct BufAnsiWriter<W: std::io::Write> {
    writer: W,
    first_write: bool,
}

impl<W: std::io::Write> BufAnsiWriter<W> {
    pub fn new(writer: W, first_write: bool) -> Self {
        Self {
            writer,
            first_write,
        }
    }
}

impl<W: std::io::Write> std::io::Write for BufAnsiWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: std::io::Write> AnsiWriter for BufAnsiWriter<W> {
    type Error = std::io::Error;

    fn escape(&mut self) -> Result<(), Self::Error> {
        self.writer.write(ESCAPE.as_bytes())?;
        Ok(())
    }

    fn end(&mut self) -> Result<(), Self::Error> {
        self.writer.write(&[END as u8])?;
        Ok(())
    }
    fn write_code(&mut self, code: u8) -> Result<(), Self::Error> {
        match self.first_write {
            true => self.first_write = false,
            false => {
                self.writer.write(b";")?;
            }
        }
        self.writer.write(&code.to_string().as_bytes())?;
        Ok(())
    }
}

pub trait AnsiWriter {
    type Error;

    fn escape(&mut self) -> Result<(), Self::Error>;
    fn end(&mut self) -> Result<(), Self::Error>;

    fn write_code(&mut self, code: u8) -> Result<(), Self::Error>;

    fn write_all(&mut self, codes: &[u8]) -> Result<(), Self::Error> {
        for code in codes {
            self.write_code(*code)?;
        }
        Ok(())
    }
}

pub trait Ansi {
    fn place_ansi<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: AnsiWriter;
    fn clear_ansi<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: AnsiWriter;
    fn no_places(&self) -> bool;
    fn no_clears(&self) -> bool;
}
