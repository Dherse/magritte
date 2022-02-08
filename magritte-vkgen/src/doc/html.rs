use std::borrow::Cow;

use ego_tree::NodeRef;
use nom::combinator::all_consuming;
use regex::{Regex, Replacer};
use scraper::{ElementRef, Node};

use crate::{source::Source, ty::native_raw};

use super::Queryable;

lazy_static::lazy_static! {
    static ref LINK_REGEX: Regex = Regex::new(r"\[`([\w0-9:_]+)`\]\s?::\s?`([\w0-9]+)`").unwrap();
}

/// A visitor that visits nodes in an HTML graph
pub(super) struct Visitor<'a, 'b>
where
    'b: 'a,
{
    pub source: &'a Source<'b>,
    pub this: &'a dyn Queryable,
    pub out: &'a mut String,
    pub code_as_transparent: bool,
}

impl<'a, 'b> Visitor<'a, 'b>
where
    'b: 'a,
{
    pub fn cleanup(&mut self) {
        match LINK_REGEX.replace_all(&*self.out, LinkReplacer(self.source)) {
            Cow::Borrowed(_) => {},
            Cow::Owned(new) => {
                *self.out = new;
            },
        }
    }

    pub fn visit(&mut self, child: NodeRef<Node>) -> Option<()> {
        if child.value().is_element() {
            self.visit_element(ElementRef::wrap(child)?)
        } else if let Some(text) = child.value().as_text() {
            if text.trim().len() == 0 {
                return None;
            }

            self.out.push_str(&text);

            Some(())
        } else {
            None
        }
    }

    pub fn visit_element<'c>(&mut self, element: ElementRef<'c>) -> Option<()> {
        match element.value().name() {
            "a" => self.visit_link(element),
            "code" if !self.code_as_transparent => self.visit_code(element),
            "code" if self.code_as_transparent => self.visit_transparent(element),
            "pre" => self.visit_pre(element),
            "strong" => self.visit_strong(element),
            "em" => self.visit_em(element),
            "sub" => self.visit_sub(element),
            "sup" => self.visit_sup(element),
            "div" | "p" => self.visit_transparent(element),
            "span" => self.visit_span(element),
            "table" => self.visit_table(element),
            "br" => {
                self.out.push('\n');
                Some(())
            },
            other => unreachable!("unsupported element: {}", other),
        }
    }

    fn visit_table<'c>(&mut self, table: ElementRef<'c>) -> Option<()> {
        todo!()
    }

    fn visit_span<'c>(&mut self, span: ElementRef<'c>) -> Option<()> {
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

    fn visit_transparent<'c>(&mut self, element: ElementRef<'c>) -> Option<()> {
        // if there is no children, return `None`
        if !element.has_children() {
            return None;
        }

        for child in element.children() {
            self.visit(child);
        }

        Some(())
    }

    fn visit_pre<'c>(&mut self, pre: ElementRef<'c>) -> Option<()> {
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

    fn visit_sup<'c>(&mut self, sup: ElementRef<'c>) -> Option<()> {
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

    fn visit_sub<'c>(&mut self, sub: ElementRef<'c>) -> Option<()> {
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

    fn visit_em<'c>(&mut self, em: ElementRef<'c>) -> Option<()> {
        // if there is no text, return `None`
        if !em.has_children() {
            return None;
        }

        self.out.push_str("*");

        for child in em.children() {
            self.visit(child);
        }

        self.out.push_str("*");

        Some(())
    }

    fn visit_strong<'c>(&mut self, strong: ElementRef<'c>) -> Option<()> {
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

    fn visit_code<'c>(&mut self, code: ElementRef<'c>) -> Option<()> {
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

    fn visit_link<'c>(&mut self, a: ElementRef<'c>) -> Option<()> {
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
        } else if trimmed == "VK_VERSION_1_2" {
            self.out.push_str("[`crate::vulkan1_2`]");
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
    fn trim_in_place(self: &mut Self);
}

impl TrimInPlace for String {
    fn trim_in_place(self: &'_ mut Self) {
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
