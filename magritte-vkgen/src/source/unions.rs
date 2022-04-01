use std::borrow::Cow;

use proc_macro2::{Ident, Span};

use crate::{
    doc::Queryable,
    origin::Origin,
    symbols::{SymbolName, SymbolTable},
};

use super::{Field, Source};

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

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
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

    /// Checks if this union needs a lifetime
    pub fn has_lifetime(&self, source: &Source<'a>, ) -> bool {
        self.fields.iter().any(|f| f.has_lifetime(source, false))
    }

    /// Checks if this union is copy
    pub fn is_copy(&self, source: &Source<'a>) -> bool {
        self.fields().iter().all(|f| f.is_copy(source))
    }
}

impl<'a> SymbolName<'a> for Union<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for Union<'a> {
    fn find<'b>(&'b self, _: &'b Source<'a>, name: &str) -> Option<&'b str> {
        self.fields.get_by_name(name).map(Field::name)
    }
}
