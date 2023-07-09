use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Field, Origin, Queryable, Source, SymbolName, SymbolTable};

/// A Vulkan union
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Union<'a> {
    /// The name of the union
    pub original_name: Cow<'a, str>,

    /// The rustified name of the union
    pub name: String,

    /// The fields that compose this struct
    #[serde(borrow = "'a")]
    pub fields: SymbolTable<'a, Field<'a>>,

    /// The origin (extension or Vulkan version)
    pub origin: Origin<'a>,
}

impl Union<'static> {
    /// Creates a new union from its fields
    #[inline]
    pub fn new(
        original_name: String,
        name: String,
        fields: SymbolTable<'static, Field<'static>>,
        origin: Origin<'static>,
    ) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name,
            fields,
            origin,
        }
    }

    /// Creates a new union from its fields with a default origin
    #[inline]
    pub fn new_no_origin(original_name: String, name: String, fields: SymbolTable<'static, Field<'static>>) -> Self {
        Self::new(original_name, name, fields, Origin::Unknown)
    }
}

impl<'a> Union<'a> {
    /// Get a reference to the union's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the union's name.
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
        (self.name() != self.original_name()).then(|| {
            quote::quote! {
                #[doc(alias = #original_name)]
            }
        })
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
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

        self.origin = origin;
    }

    pub fn has_opaque(&self, source: &Source<'a>) -> bool {
        self.fields().iter().any(|f| f.is_opaque(source))
    }

    pub fn is_copy(&self, source: &Source<'a>) -> bool {
        self.fields().iter().all(|field| field.ty().is_copy(source))
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
