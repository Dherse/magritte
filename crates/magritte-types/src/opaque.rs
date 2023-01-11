use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Origin, Queryable, Source, SymbolName};

/// An opaque external type, usually represented by a void pointer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpaqueType<'a> {
    /// The original name of the type (with the Vk, tag and everything)
    pub original_name: Cow<'a, str>,

    /// The header dependency of this type
    pub requires: Cow<'a, str>,

    /// The origin of this type
    pub origin: Origin<'a>,
}

impl OpaqueType<'static> {
    /// Creates a new opaque type from its fields
    #[inline]
    pub const fn new(original_name: String, requires: String, origin: Origin<'static>) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            requires: Cow::Owned(requires),
            origin,
        }
    }
}

impl<'a> OpaqueType<'a> {
    /// Get a reference to the alias's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the alias's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the alias's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

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

    fn pretty_name(&self) -> String {
        self.name().to_string()
    }
}

impl<'a> Queryable<'a> for OpaqueType<'a> {
    fn find(&self, _: &Source<'a>, _: &str) -> Option<&'a str> {
        None
    }
}
