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
    static ref SELECTOR_MEMBERS_H2: Selector = Selector::parse("h2#_members").unwrap();

    static ref SELECTOR_REVISION_H2: Selector = Selector::parse("h2#_revision").unwrap();
    static ref SELECTOR_DEPRECATION_H2: Selector = Selector::parse("h2#_deprecation_state").unwrap();
    static ref SELECTOR_DEPENDENCIES_H2: Selector = Selector::parse("h2#_extension_and_version_dependencies").unwrap();
    static ref SELECTOR_CONTACT_H2: Selector = Selector::parse("h2#_contact").unwrap();
    static ref SELECTOR_OTHER_EXT_METADATA_H2: Selector = Selector::parse("h2#_other_extension_metadata").unwrap();
    static ref SELECTOR_NEW_OBJ_H2: Selector = Selector::parse("h2#_new_object_types").unwrap();
    static ref SELECTOR_NEW_COMMANDS_H2: Selector = Selector::parse("h2#_new_commands").unwrap();
    static ref SELECTOR_NEW_STRUCTURES_H2: Selector = Selector::parse("h2#_new_structures").unwrap();
    static ref SELECTOR_NEW_ENUMS_H2: Selector = Selector::parse("h2#_new_enums").unwrap();
    static ref SELECTOR_NEW_BITMASKS_H2: Selector = Selector::parse("h2#_new_bitmasks").unwrap();
    static ref SELECTOR_NEW_CONSTS_H2: Selector = Selector::parse("h2#_new_enum_constants").unwrap();
    static ref SELECTOR_ISSUES_H2: Selector = Selector::parse("h2#_issues").unwrap();
    static ref SELECTOR_HISTORY_H2: Selector = Selector::parse("h2#_version_history").unwrap();


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
        self.0.get(name).map(|html| DocRef(html, false))
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
pub struct DocRef<'a>(&'a Html, bool);

impl<'a> DocRef<'a> {
    /// Gets the reference to the underlying HTML values
    pub const fn html(&self) -> &'a Html {
        self.0
    }

    /// Sets the type of documentation generated to module level doc
    pub fn set_mod_level_doc(&mut self) {
        self.1 = true;
    }

    fn visit_selectable<'b>(
        &mut self,
        source: &Source<'b>,
        this: &impl Queryable,
        variants: Option<&mut AHashMap<String, String>>,
        first_selector: &Selector,
        second_selector: &Selector,
    ) -> Option<String> {
        let a = self.html().select(first_selector).next()?;

        let parent = ElementRef::wrap(a.parent()?)?;
        let b = parent.select(second_selector).next()?;

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

        visitor.visit_element(b);

        visitor.cleanup();

        visitor.out.trim_in_place();

        if visitor.out.is_empty() {
            None
        } else {
            Some(visitor.out)
        }
    }

    /// Gets the `C Specification` section as markdown
    pub fn specification<'b>(
        &mut self,
        source: &Source<'b>,
        this: &impl Queryable,
        mut out: &mut TokenStream,
    ) -> Option<()> {
        let text = self.visit_selectable(source, this, None, &SELECTOR_SPECIFICATION_H2, &SELECTOR_SECTIONBODY)?;
        let lines = text.split('\n');

        if self.1 {
            quote::quote_each_token! {
                out

                #![doc = "# C Specifications"]
                #(#![doc = #lines])*
            }
        } else {
            quote::quote_each_token! {
                out

                #[doc = "# C Specifications"]
                #(#[doc = #lines])*
            }
        }

        Some(())
    }

    /// Gets the `Name` section as markdown
    pub fn name<'b>(&mut self, source: &Source<'b>, this: &impl Queryable, mut out: &mut TokenStream) -> Option<()> {
        let mut text = self.visit_selectable(source, this, None, &SELECTOR_NAME_H2, &SELECTOR_SECTIONBODY)?;

        // generate the link for the type itself
        if let Some(index) = text.find(' ') {
            let substr = &text[0..index];
            let link = format!(
                "[{0}](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/{0}.html)",
                substr
            );
            text.replace_range(0..index, &link);
        }

        let proc = DOUBLE_WHITE_SPACE_REGEX.replace(&text, " ");
        let text = match proc {
            Cow::Borrowed(_) => text,
            Cow::Owned(text) => text,
        };

        if text.is_empty() {
            return None;
        }

        let lines = text.split('\n');

        if self.1 {
            quote::quote_each_token! {
                out

                #(#![doc = #lines])*
            }
        } else {
            quote::quote_each_token! {
                out

                #(#[doc = #lines])*
            }
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

            if self.1 {
                quote::quote_each_token! {
                    out

                    #![doc = "# Related"]
                    #(#![doc = #lines])*
                }
            } else {
                quote::quote_each_token! {
                    out

                    #[doc = "# Related"]
                    #(#[doc = #lines])*
                }
            }

            Some(())
        } else {
            None
        }
    }


    /// Processes the members, optionally writing the variants to a map of variants
    pub fn members<'b>(
        &mut self,
        source: &Source<'b>,
        this: &impl Queryable,
        mut out: &mut TokenStream,
        variants: Option<&mut AHashMap<String, String>>,
    ) -> Option<()> {
        let text = self.visit_selectable(source, this, variants, &SELECTOR_MEMBERS_H2, &SELECTOR_SECTIONBODY)?;

        let proc = DOUBLE_WHITE_SPACE_REGEX.replace(&text, " ");
        let text = match proc {
            Cow::Borrowed(_) => text,
            Cow::Owned(text) => text,
        };

        if text.is_empty() {
            return None;
        }

        let lines = text.split('\n');
        if self.1 {
            quote::quote_each_token! {
                out

                #![doc = "# Members"]
                #(#![doc = #lines])*
            }
        } else {
            quote::quote_each_token! {
                out

                #[doc = "# Members"]
                #(#[doc = #lines])*
            }
        }

        Some(())
    }

    /// Processes the description, optionally writing the variants to a map of variants
    pub fn description<'b>(
        &mut self,
        source: &Source<'b>,
        this: &impl Queryable,
        mut out: &mut TokenStream,
        variants: Option<&mut AHashMap<String, String>>,
    ) -> Option<()> {
        let text = self.visit_selectable(source, this, variants, &SELECTOR_DESCRIPTION_H2, &SELECTOR_SECTIONBODY)?;

        let proc = DOUBLE_WHITE_SPACE_REGEX.replace(&text, " ");
        let text = match proc {
            Cow::Borrowed(_) => text,
            Cow::Owned(text) => text,
        };

        if text.is_empty() {
            return None;
        }

        let lines = text.split('\n');
        if self.1 {
            quote::quote_each_token! {
                out

                #![doc = "# Description"]
                #(#![doc = #lines])*
            }
        } else {
            quote::quote_each_token! {
                out

                #[doc = "# Description"]
                #(#[doc = #lines])*
            }
        }

        Some(())
    }

    /// Processes the new object types, commands, structures, enums, bitmasks, constants, issues and
    /// version history
    pub fn extension<'b>(&mut self, source: &Source<'b>, this: &impl Queryable, mut out: &mut TokenStream) {
        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_REVISION_H2, &SELECTOR_SECTIONBODY) {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# Revision"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_DEPRECATION_H2, &SELECTOR_SECTIONBODY) {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# Dependencies"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_DEPENDENCIES_H2, &SELECTOR_SECTIONBODY)
        {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# Dependencies"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_CONTACT_H2, &SELECTOR_SECTIONBODY) {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# Contacts"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_NEW_OBJ_H2, &SELECTOR_SECTIONBODY) {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# New handles"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_NEW_COMMANDS_H2, &SELECTOR_SECTIONBODY)
        {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# New functions & commands"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) =
            self.visit_selectable(source, this, None, &SELECTOR_NEW_STRUCTURES_H2, &SELECTOR_SECTIONBODY)
        {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# New structures"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_NEW_ENUMS_H2, &SELECTOR_SECTIONBODY) {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# New enums"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_NEW_BITMASKS_H2, &SELECTOR_SECTIONBODY)
        {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# New bitmasks"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_NEW_CONSTS_H2, &SELECTOR_SECTIONBODY) {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# New constants"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_ISSUES_H2, &SELECTOR_SECTIONBODY) {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# Known issues & F.A.Q"]
                #(#![doc = #lines])*
            }
        }

        if let Some(text) = self.visit_selectable(source, this, None, &SELECTOR_HISTORY_H2, &SELECTOR_SECTIONBODY) {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# Version History"]
                #(#![doc = #lines])*
            }
        }
        if let Some(text) = self.visit_selectable(
            source,
            this,
            None,
            &SELECTOR_OTHER_EXT_METADATA_H2,
            &SELECTOR_SECTIONBODY,
        ) {
            let lines = text.split('\n');
            quote::quote_each_token! {
                out

                #![doc = "# Other info"]
                #(#![doc = #lines])*
            }
        }
    }

    /// Adds the copyright to the bottom of the documentation
    pub fn copyright(&self, mut out: &mut TokenStream) {
        if self.1 {
            quote::quote_each_token! {
                out

                #![doc = "# Notes and documentation"]
                #![doc = "For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)"]
                #![doc = "\n"]
                #![doc = "This documentation is generated from the Vulkan specification and documentation."]
                #![doc = "The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*."]
                #![doc = "This license explicitely allows adapting the source material as long as proper credit is given."]
            }
        } else {
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
}
