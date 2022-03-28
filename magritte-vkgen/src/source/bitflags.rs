use std::borrow::Cow;

use proc_macro2::{Ident, Span};

use crate::{
    doc::Queryable,
    origin::Origin,
    symbols::{SymbolName, SymbolTable},
};

use super::{Alias, Source};

/// A type bit flags.
#[derive(Debug, Clone, PartialEq)]
pub struct BitFlag<'a> {
    /// The original name of the bit flags
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The width (in bytes) of the flag
    pub width: u8,

    /// The flags.
    pub bits: SymbolTable<'a, Bit<'a>>,

    /// The aliases of other flags
    pub aliases: SymbolTable<'a, Alias<'a>>,
}

impl<'a> BitFlag<'a> {
    /// Creates a new bit flags from its flags and type
    #[inline]
    pub const fn new(
        original_name: &'a str,
        name: String,
        width: u8,
        bits: SymbolTable<'a, Bit<'a>>,
        aliases: SymbolTable<'a, Alias<'a>>,
        origin: Origin<'a>,
    ) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            width,
            bits,
            origin,
            aliases,
        }
    }

    /// Creates a bit flags from its flags and type with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(
        original_name: &'a str,
        name: String,
        width: u8,
        bits: SymbolTable<'a, Bit<'a>>,
        aliases: SymbolTable<'a, Alias<'a>>,
    ) -> Self {
        Self::new(original_name, name, width, bits, aliases, Origin::Unknown)
    }

    /// Get a reference to the bit flags's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the bit flags's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
    }

    /// Get a reference to the bit flags's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the bit flags's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        self.origin = origin;
    }

    /// Get a reference to the bit flags's width.
    pub fn width(&self) -> u8 {
        self.width
    }

    /// Get a reference to the bit flags's bits.
    pub fn bits(&self) -> &SymbolTable<'a, Bit<'a>> {
        &self.bits
    }

    /// Get a mutable reference to the bit flags's bits.
    pub fn bits_mut(&mut self) -> &mut SymbolTable<'a, Bit<'a>> {
        &mut self.bits
    }

    /// Get a reference to the bit flags's aliases.
    pub fn aliases(&self) -> &SymbolTable<'a, Alias<'a>> {
        &self.aliases
    }

    /// Get a mutable reference to the bit flags's aliases.
    pub fn aliases_mut(&mut self) -> &mut SymbolTable<'a, Alias<'a>> {
        &mut self.aliases
    }
}

impl<'a> SymbolName<'a> for BitFlag<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for BitFlag<'a> {
    fn find<'b>(&'b self, _: &'b Source<'a>, name: &str) -> Option<&'b str> {
        self.bits
            .get_by_either(name)
            .map(Bit::name)
            .or_else(|| self.aliases.get_by_either(name).map(Alias::name))
    }
}

/// A bit that is part of a bitflags
#[derive(Debug, Clone, PartialEq)]
pub struct Bit<'a> {
    /// The original name of the bit
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The bit that has to be set to `1`
    pub value: i64,
}

impl<'a> Bit<'a> {
    /// Creates a new bit from its value
    #[inline]
    pub const fn new(original_name: &'a str, name: String, value: i64, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            value,
            origin,
        }
    }

    /// Creates a bit from its value with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: &'a str, name: String, value: i64) -> Self {
        Self::new(original_name, name, value, Origin::Unknown)
    }

    /// Get a reference to the bit's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the bit's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        if self.name().starts_with(char::is_numeric) {
            Ident::new(&format!("_{}", self.name()), Span::call_site())
        } else {
            Ident::new(self.name(), Span::call_site())
        }
    }

    /// Get a reference to the bit's origin.
    pub fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the bit's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        self.origin = origin;
    }

    /// Get a reference to the bit's value.
    pub const fn value(&self) -> i64 {
        self.value
    }
}

impl<'a> SymbolName<'a> for Bit<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}
