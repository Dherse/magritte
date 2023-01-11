use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Bitflag, Native, Origin, Queryable, Source, SymbolName, Ty};

/// A type bitmask.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Bitmask<'static> {
    /// Creates a new bitmask from its flag type
    #[inline]
    pub fn new(original_name: String, name: String, bits: Option<String>, origin: Origin<'static>) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name,
            bits: bits.map(Cow::Owned),
            origin,
        }
    }

    /// Creates a bitmask from its flag type with a default origin of unknown
    #[inline]
    pub fn new_no_origin(original_name: String, name: String, bits: Option<String>) -> Self {
        Self::new(original_name, name, bits, Origin::Unknown)
    }
}

impl<'a> Bitmask<'a> {
    /// Get a reference to the bitmask's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the bitmask's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
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
            .and_then(|bits| bits.try_as_bitflag())
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
        let bits = source.resolve_type(self.bits()?)?.try_as_bitflag()?;

        <Bitflag<'a> as Queryable<'a>>::find(bits, source, name)
    }
}
