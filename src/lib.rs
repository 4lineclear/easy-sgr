//! An easy-to-use library for adding graphical ANSI codes or [`SGR`][SGR] escape sequences to your project.
//! Its main strengths are the multitude of methods that are provided,
//! and the lack of dependencies; compile times should be pretty good.
//!
//! This library does not support the usage of non-[`SGR`][SGR] ANSI escape sequences
//!
//! ## Installation
//!
//! Add this to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! easy-sgr="0.0.8"
//! ```
//!
//! ## Usage
//!
//! ### `Color` and `Style` enums
//!
//! The simplest way to color text, using these two enums allows you to
//! work inline of a string literal when using a macro such as
//! `println!`, `writeln!` or `format!`:
//!
//! ```rust
//! use easy_sgr::{Color::*, Style::*};
//!
//! println!("{Italic}{RedFg}This should be italic & red!{Reset}");
//! ```
//!
//! `Color` and `Style` are both enums that implement `Display`: when they
//! are printed a matching [`SGR`][SGR] code is written.
//!
//! This method is the best when it comes to simplicity, but has drawbacks;
//! using it rewrites the sequence escape  `\x1b[` and the sequence end `m` repeatedly.
//! In this example this is what would be written:
//!
//! ```plain
//! \x1b[3m\x1b[31mThis should be italic & red!\x1b[0m
//! ```
//!
//! This would not be much of an issue for the vast majority of use cases.
//!
//! ### `EasySGR` trait
//!
//! This is similar to the method above but uses the `EasySGR` trait.
//! This trait is implemented by anything that implements Into\<AnsiString\> including Style and Color.
//! Its main purpose is to provide functions for chaining [`SGR`][SGR] codes.
//!
//! The example above can be achieved using it as such:
//!
//! ```rust
//! use easy_sgr::{ Color::*, EasySGR, Style::*};
//!
//! let sgr = Italic.color(RedFg);
//!
//! println!("{sgr}This should be italic & red!{Reset}");
//! ```
//!
//! Now the output would look something like this:
//!
//! ```plain
//! \x1b[31;3mThis should be italic & red!\x1b[0m
//! ```
//!
//! Instead of a rewriting the entire sequence, the separator character `;` is used instead.
//!
//! Doing this avoids the issue of rewriting the Escape and End sequences,
//! though is more expensive to use as it allocates an `SGRString`.
//!
//! ### `SGRString` struct
//!
//! `SGRString` is the type returned by all `EasySGR` functions, it encapsulates all
//! possible [`SGR`][SGR] sequences. You can use it to reproduce the previous examples as such:
//!
//! ```rust
//! use easy_sgr::{Color::*, EasySGR, Style::*};
//!
//! let text = "This should be italic & red!"
//!     .to_sgr()
//!     .style(Italic)
//!     .color(RedFg);
//! println!("{text}");
//! ```
//!
//! You can forgo `.to_sgr()` as `.style(..)`, `.color(..)` and all other `EasySGR` functions
//! can be directly called on the string literal and other types that implement it.
//!
//! The method above still uses the `EasySGR` trait, you can go without it like here:
//!
//! ```rust
//! use easy_sgr::{ColorKind, SGRString, StyleKind};
//!
//! let mut text = SGRString::from("This should be italic & red!");
//! text.italic = StyleKind::Place;
//! text.foreground = ColorKind::Red;
//!
//! println!("{text}")
//! ```
//!
//! ### `SGRWriter` struct
//!
//! The writer can also be used directly, instead of using the above methods:
//!
//! ```rust
//! use std::io::{stdout, Write};
//! use easy_sgr::{Color::*, EasySGR, SGRWriter, Style::*};
//!
//! let mut writer = SGRWriter::from(stdout());
//! writer.sgr(&Italic.color(RedFg)).unwrap();
//! writer.write_inner("This should be italic & red!").unwrap();
//! writer.sgr(&Reset).unwrap();
//! ```
//!
//! or, when writing to a String
//!
//! ```rust
//! use easy_sgr::{Color::*, EasySGR, SGRWriter, Style::*};
//!
//! let stylized_string = {
//!     let mut writer = SGRWriter::from(String::new());
//!     writer.sgr(&Italic.color(RedFg)).unwrap();
//!     writer.write_inner("This should be italic & red!").unwrap();
//!     writer.sgr(&Reset).unwrap();
//!     writer.internal()
//! };
//! ```
//!
//! ## Features
//!
//! ### `partial`
//!
//! This feature changes the way that the `discrete` module works,
//! enabling it causes it's types to not write the sequence escape and end.
//!
//! This means to achieve the same affect as above you must do this:
//!
//! ```rust
//! use easy_sgr::{Color::*, Seq::*, Style::*};
//!
//! println!("{Esc}{Italic};{RedFg}{End}This should be italic & red!{Esc}{Reset}{End}");
//! ```
//!
//! resulting in the string:
//!
//! ```plain
//! \x1b[3;31mThis should be italic & red!\x1b[0m
//! ```
//!
//! This feature exchanges ease of use for verbosity, resulting in more control.
//!
//! ## Structure
//!
//! easy-sgr is split into three modules:
//!
//! - discrete
//!     - Contains types that can be used inline of a string literal
//!     - The types, `Seq`, `Color` & `Style` are all able to function independently
//!     - They all implement the `DiscreteSGR` type to aid in this
//!     - The `DiscreteSGR` types can all work with an `SGRString`
//! - graphics
//!     - Centerpiece is `SGRString` & `EasySGR`
//!     - `SGRString` is a `String` with the ability to write [`SGR`][SGR] codes
//!     - `EasySGR` is a trait for chaining [`SGR`][SGR] codes to create a `SGRString`
//!     - `EasySGR` is blanket implemented by everything that implements `Into<SGRString>`
//!     - This includes:
//!         - `SGRString`
//!         - `Color`
//!         - `Style`
//!         - `&str`
//!         - `String`
//!         - `&String`
//! - writing
//!     - Implements `SGRWriter` & `SGRBuilder`
//!     - Used by other modules to do writing
//!
//! Though no modules really will be seen in usage,
//! as all the types they contain are reexported.
//!
//! [SGR]: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR
//!
//! ## TODO for `1.0.0` release
//!
//! - [x] Add inline that doesn't write escape itself
//! - [x] Add `get_writer` method to `writing` module
//!     - [x] Consider removing `SGRWriter`
//!     - [x] Consider adding an associated type to `CapableWriter`
//! - [ ] Add examples to docs
//!     - [x] `discrete`
//!     - [ ] `graphics`
//!     - [ ] `writing`
//! - [ ] Implement `FromStr` for [`SGR`][SGR] types
//!     - [x] Implement more complex `from_str` for `Color`
//!     - [x] Move to new submodule
//!     - [ ] Add underlying const functions
//!     - [ ] Set it as a feature
//! - [ ] Macros (`SGRise`) (`0.1.0`)
//! - [ ] Add parser(`deSGR`)
//!     - [ ] Add parsing from ansi codes
//!     - [ ] Add parsing for `SGRString`
//! - [ ] `EasySGR` implementation that doesn't allocate an `SGRString`
//! - [ ] (maybe) create smart clean system
#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    missing_docs,
    rustdoc::all
)]
#![warn(missing_debug_implementations)]
#![allow(clippy::enum_glob_use)]
/// Implements SGR types that can be used standalone of a [`SGRString`]
///
/// These types exist outside the context of a [`SGRString`], but
/// can be used in conjunction of one through the use of [`EasySGR`]
pub mod discrete;
/// Contains the standard SGR implementations.
///
/// Makes use of the [`writers`](writing) to write `SGR` codes to a writer
pub mod graphics;
/// Contains various structs and traits to help in writing `SGR` codes
pub mod writing;

pub use discrete::*;
pub use graphics::*;
pub use writing::*;

pub use easy_sgr_macros::*;
