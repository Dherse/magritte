use std::{borrow::Cow, ops::Deref};

use crate::symbols::SymbolName;

/// A Vulkan tag
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tag<'a>(Cow<'a, str>);

impl<'a> Tag<'a> {
    /// Creates a new tag from a string reference.
    #[inline]
    pub const fn new(name: &'a str) -> Self {
        Self(Cow::Borrowed(name))
    }

    /// Creates the same tag but with an underscore preceeding it
    #[inline]
    pub fn with_underscore(&self) -> String {
        format!("_{}", self.0)
    }
}

impl<'a> From<&'a vk_parse::Tag> for Tag<'a> {
    fn from(tag: &'a vk_parse::Tag) -> Self {
        Self::new(&tag.name)
    }
}

impl<'a> Deref for Tag<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> SymbolName<'a> for Tag<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.0.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_string()
    }
}
