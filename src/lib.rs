//! This is a very early work-in-progress binding to various Linux Kernel APIs.
//!
//! It is not yet ready for use in your projects. Once version 0.1 or higher
//! is released, you are welcome to start using it :)

#![cfg(target_os="linux")]

pub use std::os::*;
pub use std::os::raw::*;
