//! [![Build status](https://github.com/4lineclear/easy-sgr/actions/workflows/rust.yml/badge.svg)](https://github.com/4lineclear/easy-sgr/actions)
//!
//! An easy to use library for adding [SGR][SGR] escape sequences to your project.
//! Its main strengths are the multitude of methods that is provided, and the
//! lack of dependencies; compile times should be pretty good.
//!
//! This library does not support usage of non [SGR][SGR] ANSI escape sequences
//!
//! ## Documentation
//!
//! Not yet made
//!
//! ## Installation
//!
//! Not yet published
//!
//! ## Usage
//!
//! ### `Color` and `Style`
//!
//! The simplest way to color text, using these two `enums` allows you to
//! work inline of a string literal when using a macro such as
//! `println!`, `writeln!` or `format!`:
//!
//! ```rust
//! use easy_sgr::{Clear::Reset, Color::*, Style::*};
//!
//! println!("{Italic}{RedFg}This should be italic & red!{Reset}");
//! ```
//!
//! `Color` and `Style` are both `enums` that implement `Display`, meaning when they
//! are printed a matching [SGR][SGR] code is written. The `Reset` at the end is also a `style`,
//! it resets everything including any applied colors
//!
//! This method is the best when it comes to simplicity, but has drawbacks;
//! using it rewrites the Escape sequence `\x1b[` and the sequence End `m` repeatedly,
//! in this example this is what would be written:
//!
//! ```plain
//! \x1b[3m\x1b[31mThis should be italic & red!\x1b[0m
//! ```
//!
//! ### `EasySGR` trait
//!
//! This is similar to method as above, but using the `EasySGR` trait.
//! This trait is implemented by anything that implements `Into<AnsiString>` including both `Style` and `Color`.
//! It's main purpose is to provide functions for chaining `SGR` codes.
//!
//! The example above can be achieved using it as such:
//!
//! ```rust
//! use easy_sgr::{
//!     Clear::Reset, Color::*,
//!     Style::*, EasySGR,
//! };
//!
//! let sgr = Italic.color(RedFg);
//!
//! println!("{sgr}This should be italic & red!{Reset}");
//! ```
//!
//! Doing this avoids the issue of rewriting the Escape and End sequences,
//! though is more expensive to use as it makes use of `SGRString`.
//!
//! ### Using `SGRString`
//!
//! `SGRString` is the type returned by all `EasySGR` functions, it encapsulates all
//! possible SGR sequences. You can use it to reproduce the previous examples as such:
//!
//! ```rust
//! use easy_sgr::{
//!     Clear::Reset, Color::*,
//!     Style::*, EasySGR,
//! };
//!
//! let text = "This should be italic & red!"
//!     .to_sgr()
//!     .style(Italic)
//!     .color(RedFg);
//! println!("{text}");
//! ```
//!
//! You can actually forgo `.to_sgr()`, as all functions in the `EasySGR`
//! work for anything that implements `Into<SGRString>`, so `.style(..)` and
//! `.color(..)` can be directly called on the string literal.
//!
//! The method above still uses the `EasySGR` trait, you can go without it as shown below:
//!
//! ```rust
//! use easy_sgr::{ColorKind, Clear::Reset, SGRString, StyleKind};
//!
//! let mut text = SGRString::from("This should be italic & red!");
//! text.italic = StyleKind::Place;
//! text.foreground = ColorKind::Red;
//!
//! println!("{text}")
//! ```
//!
//! ### `SGRWriter`
//!
//! The writer can also be used directly, instead of a using the above methods:
//!
//! ```rust
//! use std::io::{stdout, Write};
//!
//! use easy_sgr::{
//!     writing::{SGRWriter, StandardWriter},
//!     Clear::Reset,
//!     Color::*,
//!     EasySGR,
//!     Style::*,
//! };
//! let mut writer = StandardWriter::io(stdout());
//! writer.place_sgr(&Italic.color(RedFg)).unwrap();
//! writer.write_inner("This should be italic & red!").unwrap();
//! writer.inline_sgr(&Reset).unwrap();
//! ```
//!
//! or, when writing to a String
//!
//! ```rust
//! use easy_sgr::{
//!     writing::{SGRWriter, StandardWriter},
//!     Clear::Reset,
//!     Color::*,
//!     EasySGR,
//!     Style::*,
//! };
//! let stylized_string = {
//!     let mut writer = StandardWriter::fmt(String::new());
//!     writer.place_sgr(&Italic.color(RedFg)).unwrap();
//!     writer.write_inner("This should be italic & red!").unwrap();
//!     writer.inline_sgr(&Reset).unwrap();
//!     writer.writer.0
//! };
//! ```
//!
//! ## Structure
//!
//! <!-- - Style
//!     - Encapsulates the different styles you can add to a string:
//!         - Reset
//!         - Bold
//!         - Dim
//!         - Italic
//!         - Underline
//!         - Blinking
//!         - Inverse
//!         - Hidden
//!         - Strikethrough
//!     - Also includes matching variants to undo these styles
//! - Color
//!     - Encapsulates different ways to color text
//!         - A set of standard colors
//!         - An 8 bit color representation
//!         - A RGB color representation
//!         - A Default variant
//! - SGRString -->
//!
//! [SGR]: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR
//!
//! ## TODO goals to publish
//!
//! - [ ] Docs
//!     - [x] [Crate](src/lib.rs) level docs
//!     - [x] Module level docs
//!     - [x] [graphics module](src/graphics/mod.rs) docs
//!     - [x] [writing module](src/writing.rs) docs
//!     - [ ] Clean up all docs
//!     - [ ] Add examples to docs
//! - [ ] Rewrite [`writers`](src/writing.rs)
//!     - [x] Do the rewrite
//!     - [x] Add trait
//!     - [x] Fix docs after adding trait
//!     - [x] Add Advanced Writer
//!     - [x] Write docs
//!     - [ ] Add `BufWriter`
//!     - [ ] Add `SGRBuilder`
//! - [ ] Unique Clear behaviours
//!
//! ## TODO goals past publishing
//!
//! - [ ] Parser (`deSGR`)
//! - [ ] Macros (`SGRise`)
#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::enum_glob_use)]
#![warn(missing_docs)]
#![warn(rustdoc::all)]
#![warn(missing_debug_implementations)]

/// Contains the standard SGR implementations.
///
/// Makes use of the [`writers`](writing) to write `SGR` codes to a writer
pub mod graphics;
/// Contains various SGR writers, most importantly the [`CapableWriter`](writing::CapableWriter) trait
pub mod writing;

pub use graphics::inline::*;
pub use graphics::*;
