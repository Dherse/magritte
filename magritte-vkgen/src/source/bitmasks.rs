use std::borrow::Cow;

use proc_macro2::{Ident, Span};

use crate::{
    doc::Queryable,
    origin::Origin,
    symbols::SymbolName,
    ty::{Native, Ty},
};

use super::{BitFlag, Source};

/// A type bitmask.
#[derive(Debug, Clone, PartialEq)]
pub struct Bitmask<'a> {
    /// The original name of the struct
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The flag types.
    pub bits: Option<Cow<'a, str>>,
}

impl<'a> Bitmask<'a> {
    /// Creates a new bitmask from its flag type
    #[inline]
    pub const fn new(original_name: &'a str, name: String, bits: Option<&'a str>, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            bits: bits.map(Cow::Borrowed),
            origin,
        }
    }

    /// Creates a bitmask from its flag type with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: &'a str, name: String, bits: Option<&'a str>) -> Self {
        Self::new(original_name, name, bits, Origin::Unknown)
    }

    /// Get a reference to the bitmask's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the bitmask's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
    }

    /// Get a reference to the bitmask's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the bitmask's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

        self.origin = origin;
    }

    /// Get a reference to the bitmask's of.
    pub fn bits(&self) -> Option<&str> {
        self.bits.as_ref().map(|s| s as &str)
    }

    /// Gets the storage type of this bitflag
    pub fn ty(&self, source: &Source<'a>) -> Ty<'a> {
        self.bits()
            .and_then(|bits| source.resolve_type(bits))
            .and_then(|bits| bits.as_bitflag())
            .map_or_else(|| Ty::Native(Native::UInt(4)), |bits| bits.ty())
    }
}

impl<'a> SymbolName<'a> for Bitmask<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for Bitmask<'a> {
    fn find<'b>(&'b self, source: &'b Source<'a>, name: &str) -> Option<&'b str> {
        let bits = source.resolve_type(self.bits()?)?.as_bitflag()?;

        <BitFlag<'a> as Queryable<'a>>::find(bits, source, name)
    }
}
