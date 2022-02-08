//! # Documentation
//! Documentation processor for generating Vulkan documentation.

mod html;

use std::{borrow::Cow, ops::Deref};

use ahash::AHashMap;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::source::Source;

use self::html::{TrimInPlace, Visitor};

lazy_static::lazy_static! {
    static ref DOUBLE_WHITE_SPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
    static ref SELECTOR_NAME_H2: Selector = Selector::parse("h2#_name").unwrap();
    static ref SELECTOR_SPECIFICATION_H2: Selector = Selector::parse("h2#_c_specification").unwrap();
    static ref SELECTOR_SECTIONBODY: Selector = Selector::parse("div.sectionbody").unwrap();
    static ref SELECTOR_SECTIONBODY_P: Selector = Selector::parse("div.sectionbody > p").unwrap();
}

/// Documentation files for the *Vulkan* docs.
#[derive(Default, Debug, Clone)]
#[repr(transparent)]
pub struct Documentation(pub(crate) AHashMap<String, Html>);

// [`Html`] contains a ton of [`std::rc::Rc`] which are not thread safe.
// But in this case, the entire bunch of `Rc`s is sent to the main thread all at once
// making it safe.
unsafe impl Send for Documentation {}

impl Documentation {
    /// Tries to find a documnetation element in the list of documentations
    pub fn find(&self, name: &str) -> Option<DocRef<'_>> {
        if let Some(doc) = self.0.get(name) {
            Some(DocRef(doc))
        } else {
            None
        }
    }
}

impl Deref for Documentation {
    type Target = AHashMap<String, Html>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An object that has queryable children
pub trait Queryable {
    /// Find the owned value
    fn find(&self, name: &str) -> Option<&str>;
}

/// A documentation element
pub struct DocRef<'a>(&'a Html);

impl<'a> DocRef<'a> {
    /// Gets the reference to the underlying HTML values
    pub const fn html(&self) -> &'a Html {
        self.0
    }

    /// Gets the `C Specification` section as markdown
    pub fn specification<'b>(&self, source: &Source<'b>, this: &dyn Queryable) -> Option<String> where {
        let h2 = self.html().select(&SELECTOR_SPECIFICATION_H2).next()?;

        let parent = ElementRef::wrap(h2.parent()?)?;
        let div = parent.select(&SELECTOR_SECTIONBODY).next()?;

        let mut out = String::new();

        let mut visitor = Visitor {
            source,
            this,
            out: &mut out,
            code_as_transparent: false,
        };

        visitor.visit_element(div);

        visitor.cleanup();

        out.trim_in_place();

        Some(out)
    }

    /// Gets the `Name` section as markdown
    pub fn name<'b>(&self, source: &Source<'b>, this: &dyn Queryable) -> Option<String> where {
        let h2 = self.html().select(&SELECTOR_NAME_H2).next()?;

        let parent = ElementRef::wrap(h2.parent()?)?;
        let p = parent.select(&SELECTOR_SECTIONBODY).next()?;

        let mut out = String::new();

        let mut visitor = Visitor {
            source,
            this,
            out: &mut out,
            code_as_transparent: false,
        };

        visitor.visit_element(p);

        visitor.cleanup();

        // generate the link for the type itself
        if let Some(index) = out.find(' ') {
            let substr = &out[0..index];
            let link = format!(
                "[{0}](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{0}.html)",
                substr
            );
            out.replace_range(0..index, &link);
        }

        out.trim_in_place();

        Some(match DOUBLE_WHITE_SPACE_REGEX.replace(&out, " ") {
            Cow::Borrowed(_) => out,
            Cow::Owned(out) => out,
        })
    }
}
