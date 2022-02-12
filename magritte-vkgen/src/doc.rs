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
    static ref SELECTOR_DESCRIPTION_H2: Selector = Selector::parse("h2#_description").unwrap();
    static ref SELECTOR_SECTIONBODY: Selector = Selector::parse("div.sectionbody").unwrap();
    static ref SELECTOR_SECTIONBODY_P: Selector = Selector::parse("div.sectionbody > p").unwrap();
    static ref SELECTOR_SECTIONBODY_DIV_PARAGRAPH_P: Selector = Selector::parse("div.sectionbody > div.paragraph > p").unwrap();
}

/// Documentation files for the *Vulkan* docs.
#[derive(Default, Debug, Clone)]
pub struct Documentation(pub(crate) AHashMap<String, Html>);

// [`Html`] contains a ton of [`std::rc::Rc`] which are not thread safe.
// But in this case, the entire bunch of `Rc`s is sent to the main thread all at once
// making it safe.
unsafe impl Send for Documentation {}

impl Documentation {
    /// Tries to find a documnetation element in the list of documentations
    pub fn find(&mut self, name: &str) -> Option<DocRef<'_>> {
        self.0.get(name).map(DocRef)
    }

    /// Adds the default "no doc" comment to an element
    pub fn no_doc(mut out: &mut TokenStream) {
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
pub struct DocRef<'a>(&'a Html);

impl<'a> DocRef<'a> {
    /// Gets the reference to the underlying HTML values
    pub const fn html(&self) -> &'a Html {
        self.0
    }

    /// Gets the `C Specification` section as markdown
    pub fn specification<'b>(
        &mut self,
        source: &Source<'b>,
        this: &impl Queryable,
        mut out: &mut TokenStream,
    ) -> Option<()> {
        let h2 = self.html().select(&SELECTOR_SPECIFICATION_H2).next()?;

        let parent = ElementRef::wrap(h2.parent()?)?;
        let div = parent.select(&SELECTOR_SECTIONBODY).next()?;

        let mut visitor = Visitor {
            source,
            this,
            out: String::new(),
            code_as_transparent: false,
            in_item: false,
            found: false,
            level: 0,
            variants: None,
        };

        visitor.visit_element(div);

        visitor.cleanup();

        visitor.out.trim_in_place();

        if visitor.out.is_empty() {
            return None;
        }

        let lines = visitor.out.split('\n');
        quote::quote_each_token! {
            out

            #[doc = "# C Specifications"]
            #(#[doc = #lines])*
        }

        Some(())
    }

    /// Gets the `Name` section as markdown
    pub fn name<'b>(&mut self, source: &Source<'b>, this: &impl Queryable, mut out: &mut TokenStream) -> Option<()> {
        let h2 = self.html().select(&SELECTOR_NAME_H2).next()?;

        let parent = ElementRef::wrap(h2.parent()?)?;
        let p = parent.select(&SELECTOR_SECTIONBODY).next()?;

        let mut visitor = Visitor {
            source,
            this,
            out: String::new(),
            code_as_transparent: false,
            in_item: false,
            found: false,
            level: 0,
            variants: None,
        };

        visitor.visit_element(p);

        visitor.cleanup();

        // generate the link for the type itself
        if let Some(index) = visitor.out.find(' ') {
            let substr = &visitor.out[0..index];
            let link = format!(
                "[{0}](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{0}.html)",
                substr
            );
            visitor.out.replace_range(0..index, &link);
        }

        visitor.out.trim_in_place();

        let proc = DOUBLE_WHITE_SPACE_REGEX.replace(&visitor.out, " ");
        let text = match proc {
            Cow::Borrowed(_) => visitor.out,
            Cow::Owned(text) => text,
        };

        if text.is_empty() {
            return None;
        }

        let lines = text.split('\n');
        quote::quote_each_token! {
            out

            #(#[doc = #lines])*
        }

        Some(())
    }

    /// Processes the related items
    pub fn related<'b>(&mut self, source: &Source<'b>, mut out: &mut TokenStream) -> Option<()> {
        let h2 = self.html().select(&SELECTOR_RELATED_H2).next()?;

        let parent = ElementRef::wrap(h2.parent()?)?;
        let p = parent.select(&SELECTOR_SECTIONBODY_DIV_PARAGRAPH_P).next()?;

        let mut text = Some(String::new());
        let mut is_any = false;
        for child in p.children() {
            if child.value().is_element() {
                is_any = true;

                let child_ref = ElementRef::wrap(child).unwrap();

                text.as_mut().unwrap().push_str("- ");

                let mut visitor = Visitor {
                    source,
                    this: &(),
                    out: text.take().unwrap(),
                    code_as_transparent: false,
                    in_item: false,
                    found: false,
                    level: 0,
                    variants: None,
                };

                visitor.visit_element(child_ref);

                visitor.cleanup();

                visitor.out.push('\n');

                text = Some(visitor.out);
            }
        }

        if is_any {
            let text = text.take().unwrap();

            if text.is_empty() {
                return None;
            }

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

    /// Processes the description, optionally writing the variants to a map of variants
    pub fn description<'b>(
        &mut self,
        source: &Source<'b>,
        this: &impl Queryable,
        mut out: &mut TokenStream,
        variants: Option<&mut AHashMap<String, String>>,
    ) -> Option<()> {
        let h2 = self.html().select(&SELECTOR_DESCRIPTION_H2).next()?;

        let parent = ElementRef::wrap(h2.parent()?)?;
        let div = parent.select(&SELECTOR_SECTIONBODY).next()?;

        let mut visitor = Visitor {
            source,
            this,
            out: String::new(),
            code_as_transparent: false,
            in_item: false,
            found: false,
            level: 0,
            variants,
        };

        visitor.visit_element(div);

        visitor.cleanup();

        visitor.out.trim_in_place();

        let proc = DOUBLE_WHITE_SPACE_REGEX.replace(&visitor.out, " ");
        let text = match proc {
            Cow::Borrowed(_) => visitor.out,
            Cow::Owned(text) => text,
        };

        if text.is_empty() {
            return None;
        }

        let lines = text.split('\n');
        quote::quote_each_token! {
            out

            #[doc = "# Description"]
            #(#[doc = #lines])*
        }

        Some(())
    }

    /// Adds the copyright to the bottom of the documentation
    pub fn copyright(mut out: &mut TokenStream) {
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
