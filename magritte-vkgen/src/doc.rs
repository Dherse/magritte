//! # Documentation
//! Documentation processor for generating Vulkan documentation.

use std::ops::Deref;

use ahash::AHashMap;
use scraper::Html;

/// Documentation files for the *Vulkan* docs.
#[derive(Default, Debug, Clone)]
#[repr(transparent)]
pub struct Documentation(pub(crate) AHashMap<String, Html>);

// [`Html`] contains a ton of [`std::rc::Rc`] which are not thread safe.
// But in this case, the entire bunch of `Rc`s is sent to the main thread all at once
// making it safe.
unsafe impl Send for Documentation {}

impl Deref for Documentation {
    type Target = AHashMap<String, Html>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
