use std::borrow::Cow;

use ahash::AHashMap;
use ego_tree::NodeRef;
use nom::combinator::all_consuming;
use regex::{Regex, Replacer};
use scraper::{ElementRef, Node, Selector};
use tracing::error;

use crate::{source::Source, ty::native_raw};

use super::Queryable;

lazy_static::lazy_static! {
    static ref LINK_REGEX: Regex = Regex::new(r"\[`([\w0-9:_]+)`\]\s?::\s?`([\w0-9]+)`").unwrap();
    static ref FULL_LINK_REGEX: Regex = Regex::new(r"\[`([\w0-9_]+::)?([\w0-9_]+)`\]").unwrap();

    static ref DLIST_DL_DD: Selector = Selector::parse("dl > dd").unwrap();
    static ref ULIST_UL_LI: Selector = Selector::parse("ul > li").unwrap();
    static ref OLIST_OL_LI: Selector = Selector::parse("ol > li").unwrap();
}

/// A visitor that visits nodes in an HTML graph
pub(super) struct Visitor<'a, 'b, T: Queryable>
where
    'b: 'a,
{
    pub source: &'a Source<'b>,
    pub this: &'a T,
    pub out: String,
    pub code_as_transparent: bool,
    pub in_item: bool,
    pub found: bool,
    pub level: u32,
    pub variants: Option<&'a mut AHashMap<String, String>>,
}

impl<'a, 'b, T: Queryable> Visitor<'a, 'b, T>
where
    'b: 'a,
{
    pub fn cleanup(&mut self) {
        match LINK_REGEX.replace_all(&*self.out, LinkReplacer(self.source)) {
            Cow::Borrowed(_) => {},
            Cow::Owned(new) => {
                self.out = new;
            },
        }
    }

    pub fn visit(&mut self, child: NodeRef<Node>) -> Option<()> {
        if child.value().is_element() {
            self.visit_element(ElementRef::wrap(child)?)
        } else if let Some(text) = child.value().as_text() {
            if text.trim().is_empty() {
                return None;
            }

            self.out.push_str(text);

            Some(())
        } else {
            None
        }
    }

    pub fn visit_element(&mut self, element: ElementRef<'_>) -> Option<()> {
        match element.value().name() {
            "a" => self.visit_link(element),
            "code" if !self.code_as_transparent => self.visit_code(element),
            "code" if self.code_as_transparent => self.visit_transparent(element),
            "pre" => self.visit_pre(element),
            "strong" => self.visit_strong(element),
            "em" | "i" => self.visit_em(element),
            "sub" => self.visit_sub(element),
            "sup" => self.visit_sup(element),
            "div" | "p" => self.visit_transparent(element),
            "span" => self.visit_span(element),
            "table" => self.visit_table(element),
            "h1" => self.visit_title(element, 1),
            "h2" => self.visit_title(element, 2),
            "h3" => self.visit_title(element, 3),
            "h4" => self.visit_title(element, 4),
            "h5" => self.visit_title(element, 5),
            "h6" => self.visit_title(element, 6),
            "br" => {
                self.out.push('\n');
                Some(())
            },
            // there is a bug where the `glossary` tag gets misgenerated
            "glossary" | "glossary," => self.visit_transparent(element),
            other => unreachable!("unsupported element: {}", other),
        }
    }

    fn visit_title(&mut self, element: ElementRef<'_>, height: u32) -> Option<()> {
        // if there is no children, return `None`
        if !element.has_children() {
            return None;
        }

        match height {
            1 => self.out.push_str("\n# "),
            2 => self.out.push_str("\n## "),
            3 => self.out.push_str("\n### "),
            _ => self.out.push_str("\n** "),
        }

        for child in element.children() {
            self.visit(child);
        }

        if height > 3 {
            self.out.push_str("**\n");
        } else {
            self.out.push('\n');
        }

        Some(())
    }

    #[allow(clippy::unused_self)]
    fn visit_table(&mut self, _table: ElementRef<'_>) -> Option<()> {
        error!("need to implement table");
        None
    }

    fn visit_span(&mut self, span: ElementRef<'_>) -> Option<()> {
        // if there is no children, return `None`
        if !span.has_children() {
            return None;
        }

        // ignore VUID stuff
        if span.value().classes().any(|c| c == "vuid") {
            return None;
        }

        // push katex code as is
        if span.value().classes().any(|c| c == "katex") {
            self.out.push_str(&span.html());

            return Some(());
        }

        // process all other types of span as being ignored
        self.visit_transparent(span)
    }

    fn visit_transparent(&mut self, element: ElementRef<'_>) -> Option<()> {
        // if there is no children, return `None`
        if !element.has_children() {
            return None;
        }

        match element.value().attr("class") {
            Some("dlist") => self.visit_list(element, |_| "*", &DLIST_DL_DD),
            Some("ulist") => self.visit_list(element, |_| "-", &ULIST_UL_LI),
            Some("olist arabic") => self.visit_list(element, |i| format!("{}.", i), &OLIST_OL_LI),
            Some("olist loweralpha") => self.visit_list(element, |i| format!("{}.", i), &OLIST_OL_LI),
            _ => {
                for child in element.children() {
                    self.visit(child);
                }

                Some(())
            },
        }
    }

    fn visit_list<F, A>(&mut self, div: ElementRef<'_>, index_fn: F, selector: &Selector) -> Option<()>
    where
        F: Fn(usize) -> A,
        A: AsRef<str>,
    {
        let select = div.select(selector);

        let mut temp = String::new();

        let mut cnt = 0;
        self.level += 1;
        for (i, elem) in select.enumerate() {
            if elem.ancestors().nth(1).unwrap() != *div {
                continue;
            }

            cnt += 1;

            self.out.push('\n');
            for _ in 1..self.level {
                self.out.push(' ');
            }

            self.in_item = true;
            self.found = false;

            self.out.push_str(index_fn(i).as_ref());
            self.out.push(' ');

            temp.clear();
            std::mem::swap(&mut self.out, &mut temp);

            for child in elem.children() {
                self.visit(child);
            }

            if let Some(variants) = &mut self.variants {
                if let Some(captures) = FULL_LINK_REGEX.captures(&self.out) {
                    let len = captures.len();
                    let name = captures.get(len - 1).unwrap().as_str();

                    if !variants.contains_key(name) {
                        variants.insert(name.to_string(), self.out.trim().to_string());
                    }
                }
            }

            // just trying to avoid useless empty lines
            self.out = self.out.replace('\n', " ").to_string();

            std::mem::swap(&mut self.out, &mut temp);
            self.out.push_str(&temp);
        }

        self.in_item = false;
        self.level -= 1;

        self.out.push('\n');

        (cnt == 0).then(|| ())
    }

    fn visit_pre(&mut self, pre: ElementRef<'_>) -> Option<()> {
        // if there is no text, return `None`
        if !pre.has_children() {
            return None;
        }

        self.out.push_str("\n```c\n");

        let old = self.code_as_transparent;
        self.code_as_transparent = true;
        for child in pre.children() {
            self.visit(child);
        }

        self.code_as_transparent = old;

        self.out.push_str("\n```\n");

        Some(())
    }

    fn visit_sup(&mut self, sup: ElementRef<'_>) -> Option<()> {
        // if there is no text, return `None`
        if !sup.has_children() {
            return None;
        }

        self.out.push_str("<sup>");

        for child in sup.children() {
            self.visit(child);
        }

        self.out.push_str("</sup>");

        Some(())
    }

    fn visit_sub(&mut self, sub: ElementRef<'_>) -> Option<()> {
        // if there is no text, return `None`
        if !sub.has_children() {
            return None;
        }

        self.out.push_str("<sub>");

        for child in sub.children() {
            self.visit(child);
        }

        self.out.push_str("</sub>");

        Some(())
    }

    fn visit_em(&mut self, em: ElementRef<'_>) -> Option<()> {
        // if there is no text, return `None`
        if !em.has_children() {
            return None;
        }

        self.out.push('*');

        for child in em.children() {
            self.visit(child);
        }

        self.out.push('*');

        Some(())
    }

    fn visit_strong(&mut self, strong: ElementRef<'_>) -> Option<()> {
        // if there is no text, return `None`
        if !strong.has_children() {
            return None;
        }

        self.out.push_str("**");

        for child in strong.children() {
            self.visit(child);
        }

        self.out.push_str("**");

        Some(())
    }

    fn visit_code(&mut self, code: ElementRef<'_>) -> Option<()> {
        // if there is no code, return `None`
        if !code.has_children() {
            return None;
        }

        if children_count(&code) == 1 {
            let first = code.first_child().unwrap();
            if let Some(text) = first.value().as_text() {
                let text = text.trim();

                if let Some(name) = self.this.find(text) {
                    self.out.push_str("[`");
                    self.out.push_str(name);
                    self.out.push_str("`]");
                } else if let Some(ref_) = self.source.find(text) {
                    self.out.push_str("[`");
                    self.out.push_str(ref_.name());
                    self.out.push_str("`]");
                } else {
                    self.out.push('`');
                    self.visit(first);
                    self.out.push('`');
                }
            } else {
                self.out.push('`');
                self.visit(first);
                self.out.push('`');
            }
        } else {
            self.out.push('`');
            for child in code.children() {
                self.visit(child);
            }
            self.out.push('`');
        }

        Some(())
    }

    fn visit_link(&mut self, a: ElementRef<'_>) -> Option<()> {
        // broken links in the Vulkan docs, see https://github.com/KhronosGroup/Vulkan-Docs/issues/1723
        const BUGS: &[&str] = &[
            "#renderpass",
            "#synchronization-queue-transfers",
            "#synchronization-image-layout-transitions",
            "#features-separateDepthStencilLayouts",
            "#features-geometryShader",
            "#features-tessellationShader",
            "#features-conditionalRendering",
            "#features-fragmentDensityMap",
            "#features-transformFeedback",
            "#features-meshShader",
            "#features-taskShader",
            "#features-shadingRateImage",
            "#features-synchronization2",
            "#fxvertex-input-extraction",
            "#features-dedicatedAllocationImageAliasing",
            "#features-subgroup-quad",
            "#interfaces",
            "#descriptorsets-pipelinelayout-consistency",
            "#features-pipelineCreationCacheControl",
            "#resources-image-format-features",
            "#formats-requiring-sampler-ycbcr-conversion",
            "#copies-images-format-compatibility",
            "#formats-compatible-planes",
            "#blocked-image",
            "#copies-buffers-images-addressing",
            "#features-subpassShading",
            "#features-invocationMask",
            "#features-rayQuery",
            "#fundamentals-threadingbehavior",
            "#acceleration-structure-inactive-prims",
            "#resources-memory-aliasing",
        ];

        let href = a.value().attr("href")?;
        let trimmed = href.trim_end_matches(".html");

        if let Some(reference) = self.source.find(trimmed) {
            self.out.push_str("[`");
            self.out.push_str(&reference.as_ident().to_string());
            self.out.push_str("`]");
            return Some(());
        } else if let Ok((_, native)) = all_consuming(native_raw)(trimmed) {
            self.out.push_str("[`");
            self.out.push_str(&native.as_ident().to_string());
            self.out.push_str("`]");
            return Some(());
        } else if trimmed == "VK_VERSION_1_0" {
            self.out.push_str("[`crate::vulkan1_0`]");
            return Some(());
        } else if trimmed == "VK_VERSION_1_1" {
            self.out.push_str("[`crate::vulkan1_1`]");
            return Some(());
        } else if trimmed == "VK_VERSION_1_2" {
            self.out.push_str("[`crate::vulkan1_2`]");
            return Some(());
        } else if trimmed == "VK_VERSION_1_3" {
            self.out.push_str("[`crate::vulkan1_3`]");
            return Some(());
        } else if trimmed == "VK_API_VERSION_1_0" {
            self.out.push_str("[`crate::utils::Version::VULKAN1_0`]");
            return Some(());
        } else if trimmed == "VK_API_VERSION_1_1" {
            self.out.push_str("[`crate::utils::Version::VULKAN1_1`]");
            return Some(());
        } else if trimmed == "VK_API_VERSION_1_2" {
            self.out.push_str("[`crate::utils::Version::VULKAN1_2`]");
            return Some(());
        } else if trimmed == "VK_API_VERSION_1_3" {
            self.out.push_str("[`crate::utils::Version::VULKAN1_3`]");
            return Some(());
        } else if trimmed == "VK_NULL_HANDLE" {
            self.out.push_str("[`crate::utils::Handle::null`]");
            return Some(());
        }

        self.out.push('[');

        a.children().for_each(|child| {
            self.visit(child);
        });

        self.out.push_str("](");

        if BUGS.contains(&href) {
            self.out
                .push_str("https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html");
            self.out.push_str(href);
        } else if href == "vkAllocationFunction_return_rules.html" {
            self.out.push_str("https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#vkAllocationFunction_return_rules");
        } else if href.starts_with("http") {
            self.out.push_str(href);
        }

        self.out.push(')');

        Some(())
    }
}

fn children_count(node: &NodeRef<Node>) -> usize {
    if let (Some(first), Some(last)) = (node.first_child(), node.last_child()) {
        // for some reason, they don't expose the `into_index` function in the public
        // interface smh.
        let a: usize = unsafe { std::mem::transmute(last.id()) };

        let b: usize = unsafe { std::mem::transmute(first.id()) };

        a - b + 1
    } else {
        0
    }
}

pub(super) trait TrimInPlace {
    fn trim_in_place(&mut self);
}

impl TrimInPlace for String {
    fn trim_in_place(&mut self) {
        let (start, len): (*const u8, usize) = {
            let self_trimmed: &str = self.trim();
            (self_trimmed.as_ptr(), self_trimmed.len())
        };

        unsafe {
            core::ptr::copy(start, self.as_bytes_mut().as_mut_ptr(), len);
        }

        self.truncate(len);
    }
}

struct LinkReplacer<'a, 'b>(&'b Source<'a>)
where
    'a: 'b;

impl<'a, 'b> Replacer for LinkReplacer<'a, 'b>
where
    'a: 'b,
{
    fn replace_append(&mut self, caps: &regex::Captures<'_>, dst: &mut String) {
        let pretty_name = caps.get(1).unwrap().as_str().split("::").last().unwrap();
        let elem = caps.get(2).unwrap().as_str();

        let replaced = self
            .0
            .find(pretty_name)
            .and_then(|r| r.as_type_ref())
            .and_then(|r| r.find(self.0, elem));

        if let Some(elem) = replaced {
            dst.push_str(&format!("[`{}::{}`]", caps.get(1).unwrap().as_str(), elem));
        } else {
            dst.push_str(&format!(
                "[`{}`]`::{}`",
                caps.get(1).unwrap().as_str(),
                caps.get(2).unwrap().as_str()
            ));
        }
    }
}
