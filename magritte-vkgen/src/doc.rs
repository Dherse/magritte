//! # Documentation
//! Documentation processor for generating Vulkan documentation.

mod html;

use std::{borrow::Cow, ops::Deref};

use ahash::AHashMap;
use proc_macro2::TokenStream;
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::source::Source;

use self::html::{TrimInPlace, Visitor};

lazy_static::lazy_static! {
    static ref DOUBLE_WHITE_SPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
    static ref SELECTOR_NAME_H2: Selector = Selector::parse("h2#_name").unwrap();
    static ref SELECTOR_SPECIFICATION_H2: Selector = Selector::parse("h2#_c_specification").unwrap();
    static ref SELECTOR_RELATED_H2: Selector = Selector::parse("h2#_see_also").unwrap();
    static ref SELECTOR_SECTIONBODY: Selector = Selector::parse("div.sectionbody").unwrap();
    static ref SELECTOR_SECTIONBODY_P: Selector = Selector::parse("div.sectionbody > p").unwrap();
    static ref SELECTOR_SECTIONBODY_DIV_PARAGRAPH_P: Selector = Selector::parse("div.sectionbody > div.paragraph > p").unwrap();
}

/// Documentation files for the *Vulkan* docs.
#[derive(Default, Debug, Clone)]
pub struct Documentation(pub(crate) AHashMap<String, Html>, pub(crate) String);

// [`Html`] contains a ton of [`std::rc::Rc`] which are not thread safe.
// But in this case, the entire bunch of `Rc`s is sent to the main thread all at once
// making it safe.
unsafe impl Send for Documentation {}

impl Documentation {
    /// Tries to find a documnetation element in the list of documentations
    pub fn find(&mut self, name: &str) -> Option<DocRef<'_>> {
        if let Some(doc) = self.0.get(name) {
            Some(DocRef(doc, &mut self.1))
        } else {
            None
        }
    }

    /// Adds the default "no doc" comment to an element
    pub fn no_doc(&self, mut out: &mut TokenStream) {
        quote::quote_each_token! {
            out

            #[doc = "This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)."]
            #[doc = "See the module level documentation where a description may be given."]
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

impl Queryable for () {
    fn find(&self, _: &str) -> Option<&str> {
        None
    }
}

/// A documentation element
pub struct DocRef<'a>(&'a Html, &'a mut String);

impl<'a> DocRef<'a> {
    /// Gets the reference to the underlying HTML values
    pub const fn html(&self) -> &'a Html {
        self.0
    }

    /// Gets the `C Specification` section as markdown
    pub fn specification<'b>(&mut self, source: &Source<'b>, this: &dyn Queryable, mut out: &mut TokenStream) -> Option<()> {
        self.1.clear();

        let h2 = self.html().select(&SELECTOR_SPECIFICATION_H2).next()?;

        let parent = ElementRef::wrap(h2.parent()?)?;
        let div = parent.select(&SELECTOR_SECTIONBODY).next()?;

        let text = &mut self.1;

        let mut visitor = Visitor {
            source,
            this,
            out: text,
            code_as_transparent: false,
        };

        visitor.visit_element(div);

        visitor.cleanup();

        text.trim_in_place();

        let lines = text.split('\n');
        quote::quote_each_token! {
            out

            #[doc = "# C Specifications"]
            #(#[doc = #lines])*
        }

        Some(())
    }

    /// Gets the `Name` section as markdown
    pub fn name<'b>(&mut self, source: &Source<'b>, this: &dyn Queryable, mut out: &mut TokenStream) -> Option<()> {
        self.1.clear();

        let h2 = self.html().select(&SELECTOR_NAME_H2).next()?;

        let parent = ElementRef::wrap(h2.parent()?)?;
        let p = parent.select(&SELECTOR_SECTIONBODY).next()?;

        let text = &mut self.1;

        let mut visitor = Visitor {
            source,
            this,
            out: text,
            code_as_transparent: false,
        };

        visitor.visit_element(p);

        visitor.cleanup();

        // generate the link for the type itself
        if let Some(index) = text.find(' ') {
            let substr = &text[0..index];
            let link = format!(
                "[{0}](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{0}.html)",
                substr
            );
            text.replace_range(0..index, &link);
        }

        text.trim_in_place();

        let mut proc = DOUBLE_WHITE_SPACE_REGEX.replace(&text, " ");
        let text = match &mut proc {
            Cow::Borrowed(_) => text,
            Cow::Owned(text) => text,
        };

        let lines = text.split('\n');
        quote::quote_each_token! {
            out

            #(#[doc = #lines])*
        }

        Some(())
    }

    /// Processes the related items
    pub fn related<'b>(&mut self, source: &Source<'b>, mut out: &mut TokenStream) -> Option<()> {
        self.1.clear();
        
        let h2 = self.html().select(&SELECTOR_RELATED_H2).next()?;

        let parent = ElementRef::wrap(h2.parent()?)?;
        let p = parent.select(&SELECTOR_SECTIONBODY_DIV_PARAGRAPH_P).next()?;

        let text = &mut self.1;

        let mut is_any = false;
        for child in p.children() {
            if child.value().is_element() {
                is_any = true;

                let child_ref = ElementRef::wrap(child).unwrap();

                text.push_str("- ");

                let mut visitor = Visitor {
                    source,
                    this: &(),
                    out: text,
                    code_as_transparent: false,
                };

                visitor.visit_element(child_ref);

                visitor.cleanup();

                text.push('\n');
            }
        }

        if is_any {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out
    
                #[doc = "# Related"]
                #(#[doc = #lines])*
            }

            Some(())
        } else {

            None
        }
    }

    /// Adds the copyright to the bottom of the documentation
    pub fn copyright(&self, mut out: &mut TokenStream) {
        quote::quote_each_token! {
            out

            #[doc = "# Notes and documentation"]
            #[doc = "For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)"]
            #[doc = "\n"]
            #[doc = "This documentation is generated from the Vulkan specification and documentation."]
            #[doc = "The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*."]
            #[doc = "This license explicitely allows adapting the source material as long as proper credit is given."]
        }
    }
}
