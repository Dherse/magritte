use std::borrow::Cow;

use crate::{origin::Origin, symbols::SymbolName};

/// A type alias.
#[derive(Debug, Clone, PartialEq)]
pub struct Alias<'a> {
    /// The original name of the alias
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The original type.
    pub of: Cow<'a, str>,
}

impl<'a> Alias<'a> {
    /// Creates a new alias from its original type
    #[inline]
    pub const fn new(original_name: &'a str, name: String, of: &'a str, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            of: Cow::Borrowed(of),
            origin,
        }
    }

    /// Creates a new alias from its original type with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: &'a str, name: String, of: &'a str) -> Self {
        Self::new(original_name, name, of, Origin::Unknown)
    }

    /// Get a reference to the alias's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the alias's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the alias's origin.
    pub fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the alias's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        self.origin = origin;
    }

    /// Get a reference to the alias's of.
    pub fn of(&self) -> &str {
        self.of.as_ref()
    }
}

impl<'a> SymbolName<'a> for Alias<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }
}
