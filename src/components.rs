//! Atomic-design component library.
//!
//! - `atoms`: smallest building blocks
//! - `molecules`: compositions of atoms
//! - `organisms`: header / footer chrome
//! - `styles`: shared class strings and SVG snippets
//!
//! Pages live in `crate::pages`.

pub mod atoms;
pub mod molecules;
pub mod organisms;
pub mod styles;

pub use atoms::*;
pub use molecules::*;
pub use organisms::*;
pub use styles::*;
