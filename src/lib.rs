//! # ply
//!
//! [PLY (Polygon File Format)](http://paulbourke.net/dataformats/ply/) file parser for Rust
//!
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

pub mod error;
use error::{PLYError, PLYResult};

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


mod comment;
pub use comment::*;
