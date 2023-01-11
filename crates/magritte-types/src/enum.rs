use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Alias, Bit, Origin, Queryable, Source, SymbolName, SymbolTable};

/// A type enum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enum<'a> {
    /// The original name of the enum
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The variants.
    #[serde(borrow = "'a")]
    pub variants: SymbolTable<'a, Bit<'a>>,

    /// The aliases of other variants
    pub aliases: SymbolTable<'a, Alias<'a>>,
}

impl Enum<'static> {
    /// Creates a new enum from its variants and type
    #[inline]
    pub const fn new(
        original_name: String,
        name: String,
        variants: SymbolTable<'static, Bit<'static>>,
        aliases: SymbolTable<'static, Alias<'static>>,
        origin: Origin<'static>,
    ) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name,
            variants,
            aliases,
            origin,
        }
    }

    /// Creates a enum from its variants and type with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(
        original_name: String,
        name: String,
        variants: SymbolTable<'static, Bit<'static>>,
        aliases: SymbolTable<'static, Alias<'static>>,
    ) -> Self {
        Self::new(original_name, name, variants, aliases, Origin::Unknown)
    }
}

impl<'a> Enum<'a> {
    /// Get a reference to the enum's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the enum's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the enum's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the enum's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

        self.origin = origin;
    }

    /// Get a reference to the enum's variants.
    pub fn variants(&self) -> &SymbolTable<'a, Bit<'a>> {
        &self.variants
    }

    /// Get a mutable reference to the enum's variants.
    pub fn variants_mut(&mut self) -> &mut SymbolTable<'a, Bit<'a>> {
        &mut self.variants
    }

    /// Get a reference to the enum's aliases.
    pub fn aliases(&self) -> &SymbolTable<'a, Alias<'a>> {
        &self.aliases
    }

    /// Get a mutable reference to the enum's aliases.
    pub fn aliases_mut(&mut self) -> &mut SymbolTable<'a, Alias<'a>> {
        &mut self.aliases
    }
}

impl<'a> SymbolName<'a> for Enum<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for Enum<'a> {
    fn find<'b>(&'b self, _: &'b Source<'a>, name: &str) -> Option<&'b str> {
        self.variants
            .get_by_either(name)
            .map(Bit::name)
            .or_else(|| self.aliases.get_by_either(name).map(Alias::name))
    }
}
