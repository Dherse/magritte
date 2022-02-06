use std::borrow::Cow;

use crate::{origin::Origin, symbols::SymbolName};

/// An opaque external type, usually represented by a void pointer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpaqueType<'a> {
    /// The original name of the type (with the Vk, tag and everything)
    pub original_name: Cow<'a, str>,

    /// The rustified name of the type
    pub name: String,

    /// The header dependency of this type
    pub requires: Cow<'a, str>,

    /// The origin of this type
    pub origin: Origin<'a>,
}

impl<'a> OpaqueType<'a> {
    /// Creates a new opaque type from its fields
    #[inline]
    pub const fn new(original_name: &'a str, name: String, requires: &'a str, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            requires: Cow::Borrowed(requires),
            origin,
        }
    }

    /// Creates a new opaque type from its fields with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: &'a str, name: String, requires: &'a str) -> Self {
        Self::new(original_name, name, requires, Origin::Unknown)
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

    /// Get a reference to the alias's requires.
    pub fn requires(&self) -> &str {
        self.requires.as_ref()
    }
}

impl<'a> SymbolName<'a> for OpaqueType<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }
}
