use std::{borrow::Cow, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::symbols::SymbolName;

/// A Vulkan vendor
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vendor<'a>(Cow<'a, str>, u32);

impl Vendor<'static> {
    /// Creates a new vendor from a string and an ID.
    pub const fn new(name: String, id: u32) -> Self {
        Self(Cow::Owned(name), id)
    }
}

impl<'a> Vendor<'a> {
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
