use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Origin, Queryable, Source, SymbolName, SymbolTable, Ty};

/// A Vulkan function pointer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionPointer<'a> {
    /// The name of the function pointer
    pub original_name: Cow<'a, str>,

    /// The rustified name of the function pointer
    pub name: String,

    /// The parent (owner) of this function pointer
    #[serde(borrow = "'a")]
    pub arguments: SymbolTable<'a, FunctionPointerArgument<'a>>,

    /// The return type of this function pointer
    pub return_type: Option<Ty<'a>>,

    /// The origin (extension or Vulkan version)
    pub origin: Origin<'a>,
}

impl FunctionPointer<'static> {
    /// Creates a new function pointer from its arguments and name
    #[inline]
    pub fn new(
        original_name: String,
        name: String,
        arguments: SymbolTable<'static, FunctionPointerArgument<'static>>,
        return_type: Option<Ty<'static>>,
        origin: Origin<'static>,
    ) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name,
            arguments,
            return_type,
            origin,
        }
    }

    /// Creates a new function pointer from its arguments with a default origin
    #[inline]
    pub fn new_no_origin(
        original_name: String,
        mut name: String,
        return_type: Option<Ty<'static>>,
        arguments: SymbolTable<'static, FunctionPointerArgument<'static>>,
    ) -> Self {
        if name == "type" {
            name = "type_".to_string();
        }

        Self::new(original_name, name, arguments, return_type, Origin::Unknown)
    }
}

impl<'a> FunctionPointer<'a> {
    /// Get a reference to the function pointer's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the function pointer's name.
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

    /// Check whether any of the variant is opaque
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    /// Get a reference to the function pointer argument's ty.
    pub fn ty(&self) -> &Ty<'a> {
        &self.ty
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
