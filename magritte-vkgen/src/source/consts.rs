use std::borrow::Cow;

use proc_macro2::{Ident, Span};

use crate::{doc::Queryable, expr::Expr, origin::Origin, symbols::SymbolName, ty::Ty};

use super::Source;

/// A type constant.
#[derive(Debug, Clone, PartialEq)]
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

impl<'a> Const<'a> {
    /// Creates a new constant from its type and value
    #[inline]
    pub const fn new(original_name: &'a str, name: String, ty: Ty<'a>, value: Expr<'a>, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            ty,
            value,
            origin,
        }
    }

    /// Creates a new constant from its type and value with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: &'a str, name: String, ty: Ty<'a>, value: Expr<'a>) -> Self {
        Self::new(original_name, name, ty, value, Origin::Unknown)
    }

    /// Get a reference to the constant's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the constant's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
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
#[derive(Debug, Clone, PartialEq)]
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

impl<'a> ConstAlias<'a> {
    /// Creates a new alias from its original constant
    #[inline]
    pub const fn new(original_name: &'a str, name: String, of: &'a str, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            of: Cow::Borrowed(of),
            origin,
        }
    }

    /// Creates a new alias from its original constant with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: &'a str, name: String, of: &'a str) -> Self {
        Self::new(original_name, name, of, Origin::Unknown)
    }

    /// Get a reference to the alias's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the alias's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
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
