use std::borrow::Cow;

use proc_macro2::{Ident, Span};

use crate::{origin::Origin, symbols::SymbolName, ty::Ty};

/// A type basetype.
#[derive(Debug, Clone, PartialEq)]
pub struct Basetype<'a> {
    /// The original name of the basetype
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The original type.
    pub of: Ty<'a>,
}

impl<'a> Basetype<'a> {
    /// Creates a new base type from its fields
    #[inline]
    pub const fn new(original_name: &'a str, name: String, of: Ty<'a>, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            of,
            origin,
        }
    }

    /// Creates a new base type from its fields with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: &'a str, name: String, of: Ty<'a>) -> Self {
        Self::new(original_name, name, of, Origin::Unknown)
    }

    /// Get a reference to the basetype's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the basetype's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
    }

    /// Get a reference to the basetype's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the basetype's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        self.origin = origin;
    }

    /// Get a reference to the basetype's of.
    pub fn of(&self) -> &Ty<'a> {
        &self.of
    }
}

impl<'a> SymbolName<'a> for Basetype<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}
