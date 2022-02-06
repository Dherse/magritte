use std::borrow::Cow;

use crate::{origin::Origin, symbols::SymbolName};

/// A Vulkan handle
#[derive(Debug, Clone, PartialEq)]
pub struct Handle<'a> {
    /// The name of the handle
    pub original_name: Cow<'a, str>,

    /// The rustified name of the handle
    pub name: String,

    /// The parent (owner) of this type
    pub parent: Option<Cow<'a, str>>,

    /// The origin (extension or Vulkan version)
    pub origin: Origin<'a>,
}

impl<'a> Handle<'a> {
    /// Creates a new handle from its parent and name
    #[inline]
    pub fn new(original_name: &'a str, name: String, parent: Option<Cow<'a, str>>, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            parent,
            origin,
        }
    }

    /// Creates a new handle from its parent with a default origin
    #[inline]
    pub fn new_no_origin(original_name: &'a str, name: String, parent: Option<Cow<'a, str>>) -> Self {
        Self::new(original_name, name, parent, Origin::Unknown)
    }

    /// Get a reference to the handle's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the handle's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the handle's fields.
    pub fn parent(&self) -> Option<&Cow<'a, str>> {
        self.parent.as_ref()
    }

    /// Get a reference to the handle's origin.
    pub fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the handle's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        self.origin = origin;
    }
}

impl<'a> SymbolName<'a> for Handle<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }
}
