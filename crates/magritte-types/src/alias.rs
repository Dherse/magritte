use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Origin, Queryable, SymbolName};

/// A type alias.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alias<'a> {
    /// The original name of the alias
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: Cow<'a, str>,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The original type.
    pub of: Cow<'a, str>,
}

impl Alias<'static> {
    /// Creates a new alias from its original type
    #[inline]
    pub const fn new(original_name: String, name: String, of: String, origin: Origin<'static>) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name: Cow::Owned(name),
            of: Cow::Owned(of),
            origin,
        }
    }

    /// Creates a new alias from its original type with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: String, name: String, of: String) -> Self {
        Self::new(original_name, name, of, Origin::Unknown)
    }
}

impl<'a> Alias<'a> {
    /// Get a reference to the alias's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the alias's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[cfg(feature = "codegen")]
    pub fn as_ident(&self) -> proc_macro2::Ident {
        if self.name().starts_with(char::is_numeric) {
            quote::format_ident!("N{}", self.name)
        } else {
            proc_macro2::Ident::new(self.name(), proc_macro2::Span::call_site())
        }
    }

    #[cfg(feature = "codegen")]
    pub fn as_alias(&self) -> Option<proc_macro2::TokenStream> {
        let original_name = self.original_name();
        (self.name() != self.original_name()).then(|| quote::quote! {
            #[doc(alias = #original_name)]
        })
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

    /// Get a reference to the alias's of.
    pub fn of(&self) -> &str {
        self.of.as_ref()
    }
}

impl<'a> SymbolName<'a> for Alias<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for Alias<'a> {
    fn find<'b>(&'b self, source: &'b crate::Source<'a>, name: &str) -> Option<&'b str> {
        source.resolve(self.of())?.as_type_ref()?.find(source, name)
    }
}
