use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Expr, Origin, Queryable, Source, SymbolName, Ty};

/// A type constant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Const<'a> {
    /// The original name of the constant
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The original type.
    pub ty: Ty<'a>,

    /// The value of the constant
    pub value: Expr<'a>,
}

impl Const<'static> {
    /// Creates a new constant from its type and value
    #[inline]
    pub const fn new(
        original_name: String,
        name: String,
        ty: Ty<'static>,
        value: Expr<'static>,
        origin: Origin<'static>,
    ) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name,
            ty,
            value,
            origin,
        }
    }

    /// Creates a new constant from its type and value with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: String, name: String, ty: Ty<'static>, value: Expr<'static>) -> Self {
        Self::new(original_name, name, ty, value, Origin::Unknown)
    }
}

impl<'a> Const<'a> {
    /// Get a reference to the constant's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the constant's name.
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

    /// Get a reference to the constant's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the constant's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

        self.origin = origin;
    }

    /// Get a reference to the const's ty.
    pub fn ty(&self) -> &Ty<'a> {
        &self.ty
    }

    /// Get a reference to the const's value.
    pub fn value(&self) -> &Expr<'a> {
        &self.value
    }
}

impl<'a> SymbolName<'a> for Const<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for Const<'a> {
    fn find(&self, _: &Source<'a>, _: &str) -> Option<&'a str> {
        None
    }
}

/// A type alias.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstAlias<'a> {
    /// The original name of the constant alias
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The original constant.
    pub of: Cow<'a, str>,
}

impl ConstAlias<'static> {
    /// Creates a new alias from its original constant
    #[inline]
    pub const fn new(original_name: String, name: String, of: String, origin: Origin<'static>) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name,
            of: Cow::Owned(of),
            origin,
        }
    }

    /// Creates a new alias from its original constant with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: String, name: String, of: String) -> Self {
        Self::new(original_name, name, of, Origin::Unknown)
    }
}

impl<'a> ConstAlias<'a> {
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

impl<'a> Queryable<'a> for ConstAlias<'a> {
    fn find(&self, _: &Source<'a>, _: &str) -> Option<&'a str> {
        None
    }
}

impl<'a> SymbolName<'a> for ConstAlias<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}
