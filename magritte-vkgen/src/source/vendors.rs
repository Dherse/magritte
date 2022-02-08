use std::{borrow::Cow, ops::Deref};

use crate::symbols::SymbolName;

/// A Vulkan vendor
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vendor<'a>(Cow<'a, str>, u32);

impl<'a> Vendor<'a> {
    /// Creates a new vendor from a string reference and an ID.
    #[inline]
    pub const fn new(name: &'a str, id: u32) -> Self {
        Self(Cow::Borrowed(name), id)
    }

    /// Gets a reference to the name
    #[inline]
    pub fn name(&self) -> &str {
        &self.0
    }

    /// Gets a reference to the ID.
    #[inline]
    pub const fn id(&self) -> u32 {
        self.1
    }
}

impl<'a> From<&'a vk_parse::VendorId> for Vendor<'a> {
    fn from(vendor: &'a vk_parse::VendorId) -> Self {
        Self::new(&vendor.name, vendor.id)
    }
}

impl<'a> Deref for Vendor<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> SymbolName<'a> for Vendor<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.0.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_string()
    }
}
