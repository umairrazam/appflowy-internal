#![allow(clippy::module_inception)]

pub use document_pad::*;
pub(crate) use extensions::*;
pub use view::*;

mod document_pad;
mod extensions;
pub mod history;
mod view;
