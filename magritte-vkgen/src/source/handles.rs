use std::borrow::Cow;

use proc_macro2::{Ident, Span};

use crate::{
    doc::Queryable,
    origin::Origin,
    symbols::{SymbolName, SymbolTable},
};

use super::Source;

/// A Vulkan handle
#[derive(Debug, Clone, PartialEq)]
pub struct Handle<'a> {
    /// The name of the handle
    pub original_name: Cow<'a, str>,

    /// The rustified name of the handle
    pub name: String,

    /// The parent (owner) of this type
    pub parent: Option<Cow<'a, str>>,

    /// Is the handle dispatchable?
    pub dispatchable: bool,

    /// The origin (extension or Vulkan version)
    pub origin: Origin<'a>,

    /// The functions defined on this handle (if any)
    pub functions: SymbolTable<'a, Cow<'a, str>>,

    /// The function that destroys one (or more) of this type of handle
    pub destroyer: Option<Cow<'a, str>>,
}

impl<'a> Queryable<'a> for Handle<'a> {
    fn find(&self, _: &Source<'a>, _: &str) -> Option<&'a str> {
        None
    }
}

impl<'a> Handle<'a> {
    /// Creates a new handle from its parent and name
    #[inline]
    pub fn new(
        original_name: &'a str,
        name: String,
        parent: Option<Cow<'a, str>>,
        dispatchable: bool,
        origin: Origin<'a>,
    ) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            parent,
            dispatchable,
            origin,
            functions: SymbolTable::default(),
            destroyer: None,
        }
    }

    /// Creates a new handle from its parent with a default origin
    #[inline]
    pub fn new_no_origin(
        original_name: &'a str,
        name: String,
        dispatchable: bool,
        parent: Option<Cow<'a, str>>,
    ) -> Self {
        Self::new(original_name, name, parent, dispatchable, Origin::Unknown)
    }

    /// Get a reference to the handle's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the handle's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
    }

    /// Get a reference to the handle's fields.
    pub fn parent(&self) -> Option<&str> {
        self.parent.as_ref().map(|s| &**s)
    }

    /// Is the handle dispatchable (an opaque pointer) or non-dispatchable (a 64 bit integer)
    #[inline]
    pub fn dispatchable(&self) -> bool {
        self.dispatchable
    }

    /// Get a reference to the handle's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the handle's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        self.origin = origin;
    }

    /// Gets the functions defined on this handle
    #[inline]
    pub const fn functions(&self) -> &SymbolTable<'a, Cow<'a, str>> {
        &self.functions
    }

    /// Adds a function to this handle
    pub fn add_function(&mut self, function: Cow<'a, str>) {
        self.functions.push(function);
    }

    /// Gets the function that destroys one or more of this handle
    pub fn destroyer(&self) -> Option<&str> {
        self.destroyer.as_ref().map(|s| &**s)
    }

    /// Sets the destroyer of this handle
    pub fn set_destroyer(&mut self, function: Cow<'a, str>) {
        self.destroyer = Some(function)
    }

    /// Is this handle a loader
    pub fn is_loader(&self) -> bool {
        ["VkInstance", "VkDevice"].contains(&self.original_name())
    }

    /// Finds the ancestor of this handle that loads it
    pub fn ancestor_loader(&self, source: &Source<'a>) -> Option<Cow<'a, str>> {
        if self.original_name() == "VkDevice" {
            return None;
        }

        if self.original_name() == "VkInstance" {
            return None;
        }

        let mut parent = source.handles.get_by_either(self.parent()?).expect("unknown handle");
        while !parent.is_loader() {
            parent = source
                .handles
                .get_by_either(parent.parent().expect("no parent"))
                .expect("unknown handle");
        }

        Some(parent.original_name.clone())
    }
}

impl<'a> SymbolName<'a> for Handle<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}
