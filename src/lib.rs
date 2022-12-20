//! # ply
//!
//! [PLY (Polygon File Format)](http://paulbourke.net/dataformats/ply/) file parser for Rust
//!

pub mod error;

pub(crate) mod ply_value;
pub use ply_value::{PLYValue, PLYValueTypeName};

pub(crate) mod reader;
pub(crate) mod writer;

mod file;
pub use file::*;

mod format;
pub use format::*;

mod element;
pub use element::*;

mod properties;
pub use properties::*;

mod payload;
pub use payload::*;

mod comment;
pub use comment::*;
