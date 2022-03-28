//! Name conversion utilities

use std::borrow::Cow;

use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use regex::{Regex, Replacer};

use crate::source::Tag;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new("_([0-9]+)").unwrap();
}

/// Converts a Vulkan type name into a rustified type name
pub fn type_name<'a>(name: &'a str, tag_list: &[Tag<'a>]) -> String {
    let trimmed = name.trim_start_matches("Vk");

    let mut index_tag = None;
    for tag in tag_list {
        if let Some(index) = trimmed.rfind(tag as &str) {
            index_tag = Some(index);
        }
    }

    let (no_tag, tag) = if let Some(tag) = index_tag {
        let (no_tag, tag) = trimmed.split_at(tag);

        (no_tag, Some(tag))
    } else {
        (trimmed, None)
    };

    let mut trimmed = no_tag.to_case(Case::UpperCamel);

    // Special case for results
    if no_tag == "Result" {
        return "VulkanResultCodes".to_owned();
    }

    if let Some(tag) = &tag {
        trimmed.push_str(tag);
    }

    deal_with_numbers(&mut trimmed);

    trimmed
}

/// Finds the tag of a type name
#[inline]
pub fn tag_of_type<'a>(name: &str, tag_list: &'a [Tag<'a>]) -> Option<&'a Tag<'a>> {
    for tag in tag_list {
        if name.contains(tag as &str) {
            return Some(tag);
        }
    }

    None
}

/// Converts a Vulkan constant name into a rustified constant name.
pub fn const_name<'a>(name: &'a str, parent_tag: Option<&Tag<'a>>) -> String {
    let mut trimmed = name.trim_start_matches("VK_").to_owned();

    if let Some(parent_tag) = parent_tag {
        let with_underscore = parent_tag.with_underscore();

        trimmed.remove_matches(&with_underscore);
    }

    // for constants we **DO NOT** deal with numbers being moved by `to_case`
    // Vulkan contains a bunch of constants that were renamed to be more consistent
    // and when calling `deal_with_numbers`, they would all be homogenized and be
    // in conflict. This is therefore purposeful!
    // deal_with_numbers(&mut trimmed);

    trimmed
}

/// Converts a Vulkan enum name into a rustified enum name
pub fn enum_name<'a>(name: &'a str, parent_tag: Option<&Tag<'a>>, parent: Option<&str>) -> String {
    let trimmed = name.trim_start_matches("VK_").to_owned();

    let cases = if let Some(parent_tag) = parent_tag {
        let with_underscore = parent_tag.with_underscore();

        trimmed.trim_start_matches(&with_underscore)
    } else {
        &trimmed
    }
    .to_case(Case::UpperCamel);

    if let Some(parent) = parent {
        cases.trim_start_matches(parent.trim_start_matches("Vk")).to_string()
    } else {
        cases
    }
}

/// Converts a Vulkan enum name into a rustified bit name
pub fn bit_name<'a>(name: &'a str, parent_tag: Option<&Tag<'a>>, parent: Option<&str>) -> String {
    let trimmed = name.trim_start_matches("VK_").replace("_BIT", "");

    let cases = if let Some(parent_tag) = parent_tag {
        let with_underscore = parent_tag.with_underscore();

        trimmed.trim_start_matches(&with_underscore)
    } else {
        &trimmed
    }
    .to_case(Case::UpperCamel);

    if let Some(parent) = parent {
        cases
            .trim_start_matches(parent.trim_start_matches("Vk").trim_end_matches("FlagBits"))
            .to_string()
    } else {
        cases
    }
}

/// Converts a Vulkan function pointer name into a rustified function pointer name
pub fn funcpointer_name<'a>(name: &'a str, tag_list: &[Tag<'a>]) -> String {
    let no_pfn = name.trim_start_matches("PFN_");
    let has_pfn = name != no_pfn;

    let trimmed = no_pfn.trim_start_matches("vk");

    let mut index_tag = None;
    for tag in tag_list {
        if let Some(index) = trimmed.rfind(tag as &str) {
            index_tag = Some(index);
        }
    }

    let (no_tag, tag) = if let Some(tag) = index_tag {
        let (no_tag, tag) = trimmed.split_at(tag);

        (no_tag, Some(tag))
    } else {
        (trimmed, None)
    };

    let mut trimmed = no_tag.to_case(Case::UpperCamel);

    if has_pfn {
        trimmed.insert_str(0, "PFN");
    }

    if let Some(tag) = &tag {
        trimmed.push_str(tag);
    }

    deal_with_numbers(&mut trimmed);

    trimmed
}

fn deal_with_numbers(value: &mut String) {
    match NUMBER_REGEX.replace(value, TrailingNumberReplacer) {
        Cow::Borrowed(_) => (),
        Cow::Owned(new) => {
            *value = new;
        },
    }
}

struct TrailingNumberReplacer;

impl Replacer for TrailingNumberReplacer {
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        dst.push_str(caps.get(1).unwrap().as_str());
    }
}
