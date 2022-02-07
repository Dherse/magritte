use std::borrow::Cow;

use crate::{
    origin::Origin,
    symbols::{SymbolName, SymbolTable},
};

use super::Field;

/// A Vulkan union
#[derive(Debug, Clone, PartialEq)]
pub struct Union<'a> {
    /// The name of the union
    pub original_name: Cow<'a, str>,

    /// The rustified name of the union
    pub name: String,

    /// The fields that compose this struct
    pub fields: SymbolTable<'a, Field<'a>>,

    /// The origin (extension or Vulkan version)
    pub origin: Origin<'a>,
}

impl<'a> Union<'a> {
    /// Creates a new union from its fields
    #[inline]
    pub fn new(original_name: &'a str, name: String, fields: SymbolTable<'a, Field<'a>>, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            fields,
            origin,
        }
    }

    /// Creates a new union from its fields with a default origin
    #[inline]
    pub fn new_no_origin(original_name: &'a str, name: String, fields: SymbolTable<'a, Field<'a>>) -> Self {
        Self::new(original_name, name, fields, Origin::Unknown)
    }

    /// Get a reference to the union's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the union's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the union's fields.
    pub fn fields(&self) -> &SymbolTable<'a, Field<'a>> {
        &self.fields
    }

    /// Get a reference to the union's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the union's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        self.origin = origin;
    }
}

impl<'a> SymbolName<'a> for Union<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }
}
