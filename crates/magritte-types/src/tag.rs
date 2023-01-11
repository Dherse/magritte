use std::{borrow::Cow, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::SymbolName;

/// A Vulkan tag
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag<'a>(Cow<'a, str>);

impl Tag<'static> {
    /// Creates a new tag from a string reference.
    #[inline]
    pub const fn new(name: String) -> Self {
        Self(Cow::Owned(name))
    }

    pub fn as_static(self) -> Tag<'static> {
        Tag(Cow::Owned(self.0.into_owned()))
    }
}

impl<'a> Tag<'a> {
    /// Creates the same tag but with an underscore preceeding it
    #[inline]
    pub fn with_underscore(&self) -> String {
        format!("_{}", self.0)
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
