//! # Magritte - VK
//! Vulkan backend for Magritte.

#![warn(clippy::pedantic, clippy::cargo)]
#![deny(missing_docs)]

/// A label, used to name handles.
pub type Label<'a> = Option<&'a str>;
