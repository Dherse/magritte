//! # Documentation
//! Documentation processor for generating Vulkan documentation.

mod html;

use std::{borrow::Cow, collections::HashMap, fmt::Write, ops::Deref};

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
    static ref SELECTOR_PARAMETERS_H2: Selector = Selector::parse("h2#_parameters").unwrap();

    static ref SELECTOR_SECTIONBODY: Selector = Selector::parse("div.sectionbody").unwrap();
    static ref SELECTOR_SECTIONBODY_P: Selector = Selector::parse("div.sectionbody > p").unwrap();
    static ref SELECTOR_SECTIONBODY_DIV_PARAGRAPH_P: Selector = Selector::parse("div.sectionbody > div.paragraph > p").unwrap();
}

macro_rules! subsection {
    ($out:ident; $self:expr; $source:expr; $this:expr; $id:ident -> $title:expr) => {
        {
            lazy_static::lazy_static! {
                static ref SELECTOR: Selector = Selector::parse(concat!("h2#", stringify!($id))).unwrap();
            }

            if let Some(text) = $self.visit_selectable($source, $this, None, &SELECTOR, &SELECTOR_SECTIONBODY) {
                write!(&mut $out, "\n\n# {}\n{}", $title, text).expect("failed to write to string");
            }
        }
    };
    ($out:ident; $self:expr; $source:expr; $this:expr; $($id:ident -> $title:expr),*) => {
        $(
            subsection!{ $out; $self; $source; $this; $id -> $title }
        )*
    };
}

/// Documentation files for the *Vulkan* docs.
#[derive(Default, Debug, Clone)]
pub struct Documentation(pub(crate) HashMap<String, Html>);

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
    type Target = HashMap<String, Html>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An object that has queryable children
pub trait Queryable<'a> {
    /// Find the owned value
    fn find<'b>(&'b self, source: &'b Source<'a>, name: &str) -> Option<&'b str>;
}

impl<'a> Queryable<'a> for () {
    fn find(&self, _: &Source<'a>, _: &str) -> Option<&'a str> {
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
        this: &impl Queryable<'b>,
        variants: Option<&mut HashMap<String, String>>,
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
    pub fn specification<'b>(&mut self, source: &Source<'b>, this: &impl Queryable<'b>) -> Option<String> {
        let text = self.visit_selectable(source, this, None, &SELECTOR_SPECIFICATION_H2, &SELECTOR_SECTIONBODY)?;

        Some(format!("\n\n# C Specifications\n{}", text))
    }

    /// Gets the `Name` section as markdown
    pub fn name<'b>(&mut self, source: &Source<'b>, this: &impl Queryable<'b>) -> Option<String> {
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

        Some(text)
    }

    /// Processes the related items
    pub fn related<'b>(&mut self, source: &Source<'b>) -> Option<String> {
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

            Some(format!("\n\n# Related\n{}", text))
        } else {
            None
        }
    }

    /// Processes the members, optionally writing the variants to a map of variants
    pub fn members<'b>(
        &mut self,
        source: &Source<'b>,
        this: &impl Queryable<'b>,
        variants: Option<&mut HashMap<String, String>>,
    ) -> Option<String> {
        let text = self.visit_selectable(source, this, variants, &SELECTOR_MEMBERS_H2, &SELECTOR_SECTIONBODY)?;

        let proc = DOUBLE_WHITE_SPACE_REGEX.replace(&text, " ");
        let text = match proc {
            Cow::Borrowed(_) => text,
            Cow::Owned(text) => text,
        };

        if text.is_empty() {
            return None;
        }

        Some(format!("\n\n# Members\n{text}"))
    }

    /// Processes the parameters, optionally writing the variants to a map of variants
    pub fn parameters<'b>(
        &mut self,
        source: &Source<'b>,
        this: &impl Queryable<'b>,
        variants: Option<&mut HashMap<String, String>>,
    ) -> Option<String> {
        let text = self.visit_selectable(source, this, variants, &SELECTOR_PARAMETERS_H2, &SELECTOR_SECTIONBODY)?;

        let proc = DOUBLE_WHITE_SPACE_REGEX.replace(&text, " ");
        let text = match proc {
            Cow::Borrowed(_) => text,
            Cow::Owned(text) => text,
        };

        if text.is_empty() {
            return None;
        }

        Some(format!("\n\n# Parameters\n{text}"))
    }

    /// Processes the description, optionally writing the variants to a map of variants
    pub fn description<'b>(
        &mut self,
        source: &Source<'b>,
        this: &impl Queryable<'b>,
        variants: Option<&mut HashMap<String, String>>,
    ) -> Option<String> {
        let text = self.visit_selectable(source, this, variants, &SELECTOR_DESCRIPTION_H2, &SELECTOR_SECTIONBODY)?;

        let proc = DOUBLE_WHITE_SPACE_REGEX.replace(&text, " ");
        let text = match proc {
            Cow::Borrowed(_) => text,
            Cow::Owned(text) => text,
        };

        if text.is_empty() {
            return None;
        }

        Some(format!("\n\n# Description\n{text}"))
    }

    /// Processes the new object types, commands, structures, enums, bitmasks, constants, issues and
    /// version history
    pub fn extension<'b>(&mut self, source: &Source<'b>, this: &impl Queryable<'b>) -> String {
        let mut out = String::with_capacity(10 << 10);
        subsection! {
            out;
            self;
            source;
            this;
            _registered_extension_number -> "Registered extension number",
            _revision -> "Revision",
            _extension_and_version_dependencies -> "Dependencies",
            _deprecation_state -> "Deprecation state",
            _contact -> "Contacts",
            _new_macros -> "New macros",
            _new_base_types -> "New base types",
            _new_object_types -> "New object types",
            _new_handles -> "New handles",
            _new_commands -> "New commands",
            _new_structures -> "New structures",
            _new_unions -> "New unions",
            _new_enums -> "New enums",
            _new_bitmasks -> "New bitmasks",
            _new_enum_constants -> "New constants",
            _issues -> "Known issues & F.A.Q.",
            _version_history -> "Version history",
            _other_extension_metadata -> "Other information"
        }

        out
    }

    /// Adds the copyright to the bottom of the documentation
    pub fn copyright(&self) -> &'static str {
        r#"
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)

This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        "#
    }
}
