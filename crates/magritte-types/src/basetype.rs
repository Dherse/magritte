use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{origin::Origin, symbols::SymbolName, Queryable, Ty};

/// A type basetype.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Basetype<'static> {
    /// Creates a new base type from its fields
    #[inline]
    pub const fn new(original_name: String, name: String, of: Ty<'static>, origin: Origin<'static>) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name,
            of,
            origin,
        }
    }

    /// Creates a new base type from its fields with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: String, name: String, of: Ty<'static>) -> Self {
        Self::new(original_name, name, of, Origin::Unknown)
    }
}

impl<'a> Basetype<'a> {
    /// Get a reference to the basetype's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the basetype's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[cfg(feature = "codegen")]
    pub fn as_ident(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(self.name(), proc_macro2::Span::call_site())
    }

    #[cfg(feature = "codegen")]
    pub fn as_alias(&self) -> Option<proc_macro2::TokenStream> {
        let original_name = self.original_name();
        (self.name() != self.original_name()).then(|| quote::quote! {
            #[doc(alias = #original_name)]
        })
    }

    /// Get a reference to the basetype's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the basetype's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

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

impl<'a> Queryable<'a> for Basetype<'a> {
    fn find<'b>(&'b self, _source: &'b super::Source<'a>, _name: &str) -> Option<&'b str> {
        None
    }
}
