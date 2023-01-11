use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Native, Origin, Queryable, SymbolName, SymbolTable, Ty};

use super::{Alias, Source};

/// A type bit flags.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bitflag<'a> {
    /// The original name of the bit flags
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The width (in bytes) of the flag
    pub width: u8,

    /// The flags.
    #[serde(borrow = "'a")]
    pub bits: SymbolTable<'a, Bit<'a>>,

    /// The aliases of other flags
    pub aliases: SymbolTable<'a, Alias<'a>>,
}

impl Bitflag<'static> {
    /// Creates a new bit flags from its flags and type
    #[inline]
    pub const fn new(
        original_name: String,
        name: String,
        width: u8,
        bits: SymbolTable<'static, Bit<'static>>,
        aliases: SymbolTable<'static, Alias<'static>>,
        origin: Origin<'static>,
    ) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
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
        original_name: String,
        name: String,
        width: u8,
        bits: SymbolTable<'static, Bit<'static>>,
        aliases: SymbolTable<'static, Alias<'static>>,
    ) -> Self {
        Self::new(original_name, name, width, bits, aliases, Origin::Unknown)
    }
}

impl<'a> Bitflag<'a> {
    /// Get a reference to the bit flags's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the bit flags's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the bit flags's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the bit flags's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

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

    /// Gets the storage type of this bitflag
    pub fn ty(&self) -> Ty<'a> {
        match self.width() {
            4 => Ty::Native(Native::UInt(4)),
            8 => Ty::Native(Native::UInt(8)),
            _ => unreachable!("unsupported bit width"),
        }
    }
}

impl<'a> SymbolName<'a> for Bitflag<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for Bitflag<'a> {
    fn find<'b>(&'b self, _: &'b Source<'a>, name: &str) -> Option<&'b str> {
        self.bits
            .get_by_either(name)
            .map(Bit::name)
            .or_else(|| self.aliases.get_by_either(name).map(Alias::name))
    }
}

/// A bit that is part of a bitflags
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Bit<'static> {
    /// Creates a new bit from its value
    #[inline]
    pub const fn new(original_name: String, name: String, value: i64, origin: Origin<'static>) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name,
            value,
            origin,
        }
    }

    /// Creates a bit from its value with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: String, name: String, value: i64) -> Self {
        Self::new(original_name, name, value, Origin::Unknown)
    }
}

impl<'a> Bit<'a> {
    /// Get a reference to the bit's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the bit's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the bit's origin.
    pub fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the bit's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

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
