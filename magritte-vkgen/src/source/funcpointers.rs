use std::borrow::Cow;

use proc_macro2::{Ident, Span};

use crate::{
    doc::Queryable,
    origin::Origin,
    symbols::{SymbolName, SymbolTable},
    ty::Ty,
};

use super::Source;

/// A Vulkan function pointer
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionPointer<'a> {
    /// The name of the function pointer
    pub original_name: Cow<'a, str>,

    /// The rustified name of the function pointer
    pub name: String,

    /// The parent (owner) of this function pointer
    pub arguments: SymbolTable<'a, FunctionPointerArgument<'a>>,

    /// The return type of this function pointer
    pub return_type: Option<Ty<'a>>,

    /// The origin (extension or Vulkan version)
    pub origin: Origin<'a>,
}

impl<'a> FunctionPointer<'a> {
    /// Creates a new function pointer from its arguments and name
    #[inline]
    pub fn new(
        original_name: &'a str,
        name: String,
        arguments: SymbolTable<'a, FunctionPointerArgument<'a>>,
        return_type: Option<Ty<'a>>,
        origin: Origin<'a>,
    ) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            arguments,
            return_type,
            origin,
        }
    }

    /// Creates a new function pointer from its arguments with a default origin
    #[inline]
    pub fn new_no_origin(
        original_name: &'a str,
        mut name: String,
        return_type: Option<Ty<'a>>,
        arguments: SymbolTable<'a, FunctionPointerArgument<'a>>,
    ) -> Self {
        if name == "type" {
            name = "type_".to_string();
        }

        Self::new(original_name, name, arguments, return_type, Origin::Unknown)
    }

    /// Get a reference to the function pointer's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the function pointer's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
    }

    /// Gets a reference to the optional return type
    #[inline]
    pub fn return_type(&self) -> Option<&Ty<'a>> {
        self.return_type.as_ref()
    }

    /// Get a reference to the function pointer's arguments.
    pub fn arguments(&self) -> &SymbolTable<'a, FunctionPointerArgument<'a>> {
        &self.arguments
    }

    /// Get a reference to the function pointer's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the function pointer's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

        self.origin = origin;
    }

    /// Checks whether the functions needs a lifetime
    pub fn has_lifetime(&self, source: &Source<'a>) -> bool {
        self.arguments.iter().any(|a| a.has_lifetime(source, false))
    }

    pub fn has_opaque(&self, source: &Source<'a>) -> bool {
        self.arguments().iter().any(|a| a.is_opaque(source))
    }
}

impl<'a> SymbolName<'a> for FunctionPointer<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for FunctionPointer<'a> {
    fn find<'b>(&'b self, _: &'b Source<'a>, name: &str) -> Option<&'b str> {
        self.arguments.get_by_either(name).map(FunctionPointerArgument::name)
    }
}

/// A function pointer argument
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionPointerArgument<'a> {
    /// The name of the argument
    pub original_name: Cow<'a, str>,

    /// The name of the argument
    pub name: String,

    /// The type of the argument
    pub ty: Ty<'a>,
}

impl<'a> FunctionPointerArgument<'a> {
    /// Creates a new function poiter from its components
    pub fn new(original_name: Cow<'a, str>, name: String, ty: Ty<'a>) -> Self {
        Self {
            original_name,
            name,
            ty,
        }
    }

    /// Get a reference to the function pointer argument's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the function pointer argument's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
    }

    /// Get a reference to the function pointer argument's ty.
    pub fn ty(&self) -> &Ty<'a> {
        &self.ty
    }

    /// Checks whether the argument needs a lifetime
    pub fn has_lifetime(&self, source: &Source<'a>, pointer_has_lifetime: bool) -> bool {
        todo!()
        // self.ty().has_lifetime(source, pointer_has_lifetime)
    }

    pub fn is_opaque(&self, source: &Source<'a>) -> bool {
        self.ty().is_opaque(source)
    }
}

impl<'a> SymbolName<'a> for FunctionPointerArgument<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}
