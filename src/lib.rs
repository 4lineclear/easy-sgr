//! A library for help in using [SGR][SGR] escape sequences.
//! Its main strength is the number of ways this is done.
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
//! `println!`, `writeln!` or `format!`.
//! 
//! ```rust
//! use flc_easy_sgr::{Color::*, Style::*};
//! 
//! println!("{Italic}{RedFg}This should be italic & red!{Reset}");
//! ```
//! 
//! `Color` and `Style` are both `enums` that implement `Display`, meaning when they
//! are printed a matching [SGR][SGR] code is written. `Reset` is also a style,
//! it resets everything including any applied colors
//! 
//! This method is the best when it comes to simplicity, but has drawbacks:
//! using it rewrites the Escape sequence `\x1b[` and the sequence End `m` repeatedly.
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
//! use flc_easy_sgr::{Color::*, EasySGR, Style::*};
//! 
//! println!("{}This should be italic & red!{Reset}", Italic.color(RedFg));
//! ```
//! 
//! Doing this avoids the issue of rewriting the Escape and End sequences.
//! 
//! ### Using `SGRString`
//! 
//! `SGRString` is the type returned by all `EasySGR`, it encapsulates all
//! possible SGR sequences. You can use it to reproduce the previous examples as such:
//! 
//! ```rust
//! use flc_easy_sgr::graphics::{
//!     inline::{Color::*, Style::*},
//!     EasySGR,
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
//! work for anything that implements `Into<SGRString>`.
//! 
//! The method above still uses the `EasySGR` trait, you can go without it as shown below:
//! 
//! ```rust
//! use flc_easy_sgr::{ColorKind, SGRString, StyleKind};
//! 
//! let mut text = SGRString::from("This should be italic & red!");
//! text.italic = StyleKind::Place;
//! text.foreground = ColorKind::Red;
//! 
//! println!("{text}")
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
//! ## Todo
//! 
//! - [ ] Docs
//!     - [x] [Crate](src/lib.rs) level docs
//!     - [x] Module level docs
//!     - [x] [graphics module](src/graphics/mod.rs) docs
//!     - [ ] Clean up all docs
//!     - [ ] Add examples to docs
//! - [ ] Parser (`deSGR`)
//! - [ ] Macros (`SGRise`)
//! - [ ] Unique Clear behaviours
//! - [x] Add `BufWriter` to [writing module](src/writing.rs)
#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::enum_glob_use)]
#![warn(missing_docs)]
#![warn(rustdoc::all)]
#![warn(missing_debug_implementations)]

/// Contains the standard SGR implementations except the writers
pub mod graphics;
/// Contains various SGR writers, most importantly the [`SGRWriter`](writing::SGRWriter) trait
pub mod writing;

pub use graphics::*;
pub use graphics::inline::*;
