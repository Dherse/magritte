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

    /// Renaming for custom aliases
    pub rename: Option<Cow<'a, str>>,

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
        let parent = if parent == Some(Cow::Borrowed("VkSurfaceKHR")) && original_name == "VkSwapchainKHR" {
            Some(Cow::Borrowed("VkDevice"))
        } else if parent == Some(Cow::Borrowed("VkDevice")) && original_name == "VkImageView" {
            Some(Cow::Borrowed("VkImage"))
        } else {
            parent
        };

        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            rename: None,
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

    /// Get a mutable reference to the handle's fields.
    pub fn parent_mut(&mut self) -> &mut Option<Cow<'a, str>> {
        &mut self.parent
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
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

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

    /// Find a given ancestor in this ancestor tree
    pub fn find_ancestors(&self, source: &Source<'a>, ancestor: &str) -> Option<usize> {
        let mut i = 0;
        let mut parent = self
            .parent()
            .map(|parent| source.handles.get_by_either(parent).unwrap());
        while let Some(p) = parent {
            i += 1;
            if p.original_name() == ancestor {
                return Some(i);
            }

            parent = p.parent().map(|parent| source.handles.get_by_either(parent).unwrap());
        }

        None
    }

    /// Find the entry in thisd ancestry tree
    pub fn find_entry_ancestors(&self, source: &Source) -> usize {
        let mut i = 1;
        let mut parent = self
            .parent()
            .map(|parent| source.handles.get_by_either(parent).unwrap());
        while let Some(p) = parent {
            i += 1;
            parent = p.parent().map(|parent| source.handles.get_by_either(parent).unwrap());
        }

        i
    }

    pub(crate) fn find_instance_ancestors(&self, source: &Source<'a>) -> Option<usize> {
        self.find_ancestors(source, "VkInstance")
    }

    pub(crate) fn find_physical_device_ancestors(&self, source: &Source<'a>) -> Option<usize> {
        self.find_ancestors(source, "VkPhysicalDevice")
    }

    pub(crate) fn find_device_ancestors(&self, source: &Source<'a>) -> Option<usize> {
        self.find_ancestors(source, "VkDevice")
    }

    pub(crate) fn find_command_pool_ancestors(&self, source: &Source<'a>) -> Option<usize> {
        self.find_ancestors(source, "VkCommandPool")
    }

    pub(crate) fn find_descriptor_pool_ancestors(&self, source: &Source<'a>) -> Option<usize> {
        self.find_ancestors(source, "VkDescriptorPool")
    }

    pub(crate) fn find_display_ancestors(&self, source: &Source<'a>) -> Option<usize> {
        self.find_ancestors(source, "VkDisplayKHR")
    }

    pub(crate) fn find_surface_ancestors(&self, source: &Source<'a>) -> Option<usize> {
        self.find_ancestors(source, "VkSurfaceKHR")
    }

    pub(crate) fn find_video_session_ancestors(&self, source: &Source<'a>) -> Option<usize> {
        self.find_ancestors(source, "VkVideoSessionKHR")
    }
}

impl<'a> SymbolName<'a> for Handle<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.rename.clone().unwrap_or_else(|| self.original_name.clone())
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}
