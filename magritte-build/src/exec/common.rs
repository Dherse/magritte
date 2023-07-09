use std::{
    fs::File,
    ops::{Deref, DerefMut, Not},
    path::PathBuf,
};

use magritte_build::{
    imports::Imports, rustfmt::run_rustfmt, ugly_diff_paths::ugly_diff_paths, OriginVisitor, Visitor,
};
use magritte_types::{
    Basetype, Bitflag, Bitmask, Const, ConstAlias, Enum, Handle, Origin, Ref, Source, Struct, TypeRef,
};
use proc_macro2::TokenStream;
use quote::{quote, quote_each_token, ToTokens};

use crate::{
    edge_case::EdgeCase,
    hl::simple::is_struct_simple,
    native::{
        field::field_type,
        r#const::{constant_type, constant_value},
    },
};

pub struct CommonVisitor<'b> {
    pub edge_cases: &'b Vec<Box<dyn EdgeCase + Send + Sync>>,
    pub doc: PathBuf,
    pub out: PathBuf,
    pub handles: Vec<Handle<'static>>,
}

impl<'b> CommonVisitor<'b> {
    pub fn doc_of_origin<P: AsRef<str>>(&self, doc_dir: P, origin: &Origin<'_>) -> Option<TokenStream> {
        let real_path = origin.as_mod_doc_file_path(&self.doc);

        let doc_dir = doc_dir.as_ref();

        let link = origin.is_opaque().not().then(|| {
            let item = origin.to_core();
            let link =
                format!("# [{item}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{item}.html)\n");
            quote! {
                #![doc = #link]
            }
        });

        real_path.exists().then(|| {
            let doc_path = origin.as_mod_doc_file_string(doc_dir);
            quote! {
                #link
                #![doc = include_str!(#doc_path)]
            }
        })
    }

    pub fn doc_of<P: AsRef<str>>(&self, doc_dir: P, origin: &Origin<'_>, item: &str) -> Option<TokenStream> {
        let real_path = origin.as_doc_dir_path(&self.doc).join(format!("{item}.md"));
        let doc_dir = doc_dir.as_ref();

        let link =
            format!("# [{item}](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/{item}.html)\n");
        real_path.exists().then(|| {
            let doc_path = format!("{doc_dir}/{item}.md");
            quote! {
                #[doc = #link]
                #[doc = include_str!(#doc_path)]
            }
        })
    }

    pub fn doc_of_child<P: AsRef<str>>(&self, doc_dir: P, item: &str, child: &str) -> Option<TokenStream> {
        let real_path = self.doc.join(format!("{item}${child}.md"));
        let doc_dir = doc_dir.as_ref();

        real_path.exists().then(|| {
            let doc_path = format!("{doc_dir}/{item}${child}.md");
            quote! {
                #[doc = include_str!(#doc_path)]
            }
        })
    }
}

impl<'b> Visitor for CommonVisitor<'b> {
    type OriginVisitor<'parent> = CommonOriginVisitor<'b, 'parent>
    where
        Self: 'parent;

    type VersionVisitor<'parent> = ()
    where
        Self: 'parent;

    type ExtensionVisitor<'parent> = ()
    where
        Self: 'parent;

    fn visit_origin<'a>(&mut self, _source: &Source<'a>, origin: &Origin<'a>) -> Option<Self::OriginVisitor<'_>> {
        let mod_path = origin.as_mod_file_path(&self.out);

        std::fs::create_dir_all(mod_path.parent().expect("no parent")).expect("failed to create mod dir");
        File::create(&mod_path).expect("failed to create mod file");

        let absolute = mod_path.canonicalize().expect("failed to canonicalize path");
        let doc_dir = ugly_diff_paths(&self.doc, &absolute).expect("no relative path");

        let doc_dir_path = origin.as_doc_dir_string(&doc_dir);

        let parent = mod_path.parent().expect("missing parent on path");
        if !parent.exists() {
            std::fs::create_dir_all(&parent).expect("failed to create file path");
        }

        Some(CommonOriginVisitor {
            parent: self,
            doc_dir_path,
            mod_path,
            origin: origin.to_static(),
            imports: Imports::new(origin, "crate::common"),
            tt: TokenStream::new(),
        })
    }
}

pub struct CommonOriginVisitor<'b, 'parent> {
    pub parent: &'parent mut CommonVisitor<'b>,
    pub doc_dir_path: String,
    pub mod_path: PathBuf,
    pub origin: Origin<'static>,

    pub tt: TokenStream,
    pub imports: Imports,
}

impl<'b, 'parent> OriginVisitor<'parent> for CommonOriginVisitor<'b, 'parent> {
    fn visit_const<'a>(&mut self, source: &Source<'a>, const_: &Const<'a>) {
        let name = const_.as_ident();
        let ty = constant_type(const_.ty(), &mut self.imports);
        let value = constant_value(const_.value(), source, &mut self.imports);

        let doc = self.doc_of(&self.doc_dir_path, &self.origin, const_.original_name());
        let alias = const_.as_alias();

        let mut out = &mut self.tt;
        quote_each_token! {
            out

            #doc
            #alias
            pub const #name: #ty = #value;
        };
    }

    fn visit_const_alias<'a>(&mut self, source: &Source<'a>, const_alias: &ConstAlias<'a>) {
        let of = source.resolve(const_alias.of()).expect("unknown alias").as_const();

        self.imports.push_origin(source, of.origin(), of.name());

        let name = const_alias.as_ident();
        let alias = const_alias.as_alias();
        let of_ident = of.as_ident();
        let ty = constant_type(of.ty(), &mut self.imports);

        let doc_str = format!("See [`{}`]", of.name());

        let mut out = &mut self.tt;
        quote_each_token! {
            out

            #[doc = #doc_str]
            #alias
            pub const #name: #ty = #of_ident;
        };
    }

    fn visit_base_type<'a>(&mut self, _source: &Source<'a>, base_type: &Basetype<'a>) {
        let name = base_type.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, base_type.original_name());
        let alias = base_type.as_alias();

        let ty = constant_type(base_type.of(), &mut self.imports);

        let mut out = &mut self.tt;
        quote_each_token! {
            out

            #doc
            #alias
            pub type #name = #ty;
        };
    }

    fn visit_bitmask<'a>(&mut self, source: &Source<'a>, bitmask: &Bitmask<'a>) {
        if let Some(bitflag) = bitmask
            .bits()
            .and_then(|bits| source.resolve(bits))
            .and_then(Ref::try_as_bitflag)
        {
            let ty = match bitflag.width() {
                4 => quote! { u32 },
                8 => quote! { u64 },
                other => unreachable!("unknown bit width ({other}) for a mask: {:?}", bitflag),
            };

            self.gen_for_bitmask(source, bitmask, bitflag, &ty);
        } else {
            let name = bitmask.as_ident();
            let alias = bitmask.as_alias();

            let mut out = &mut self.tt;
            quote_each_token! {
                out

                #alias
                #[repr(transparent)]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
                pub struct #name(u32);

                impl #name {
                    #[doc = "Default empty flags"]
                    #[inline]
                    pub const fn empty() -> Self {
                        Self(0)
                    }
                }

                impl Default for #name {
                    #[inline]
                    fn default() -> Self {
                        Self::empty()
                    }
                }

                #[cfg(feature = "native")]
                unsafe impl crate::conv::IntoLowLevel for #name {
                    type LowLevel = Self;

                    unsafe fn into_low_level(&self, context: &std::sync::Arc<crate::context::Context>, bump: &bumpalo::Bump) -> Self::LowLevel {
                        *self
                    }
                }

                #[cfg(feature = "native")]
                unsafe impl crate::conv::FromLowLevel for #name {
                    unsafe fn from_low_level(context: &std::sync::Arc<crate::context::Context>, value: <Self as crate::conv::IntoLowLevel>::LowLevel) -> Self {
                        value
                    }
                }
            }
        }
    }

    fn visit_bitflag<'a>(&mut self, source: &Source<'a>, bitflag: &Bitflag<'a>) {
        let ty = match bitflag.width() {
            4 => quote! { u32 },
            8 => quote! { u64 },
            other => unreachable!("unknown bit width ({other}) for a mask: {:?}", bitflag),
        };

        self.gen_for_biflag(source, bitflag, &ty);
    }

    fn visit_enum<'a>(&mut self, source: &Source<'a>, enum_: &Enum<'a>) {
        self.gen_for_enum(source, enum_);
    }

    fn visit_struct<'a>(&mut self, source: &Source<'a>, struct_: &Struct<'a>) {
        let name = struct_.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, struct_.original_name());
        let alias = struct_.as_alias();

        let is_simple = is_struct_simple(self.edge_cases, source, struct_);
        if !is_simple || !self.edge_cases.type_filter(source, TypeRef::Struct(struct_)) {
            return;
        }

        let fields = struct_
            .fields()
            .iter()
            .map(|field| {
                let name = field.as_ident();
                let doc = self.doc_of_child(&self.doc_dir_path, struct_.original_name(), field.original_name());
                let alias = field.as_alias();
                let ty = field_type(source, field.ty(), &mut self.imports);

                quote! {
                    #doc
                    #alias
                    pub #name: #ty
                }
            })
            .collect::<Vec<_>>();

        let mut out = &mut self.tt;
        quote_each_token! {
            out

            #doc
            #alias
            #[derive(Clone, Copy, PartialEq, Debug, Default)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            #[repr(C)]
            pub struct #name {
                #(#fields),*
            }
        };
    }

    fn finish<'a>(self, _source: &Source<'a>) {
        let mut out = String::with_capacity(1 << 20);

        if let Some(doc) = self.doc_of_origin(&self.doc_dir_path, &self.origin) {
            out.extend_one(doc.to_string());
        }

        out.extend_one(self.imports.to_token_stream().to_string());
        out.extend_one(self.tt.to_string());

        let out = run_rustfmt(out).expect("failed to run rustfmt");
        std::fs::write(&self.mod_path, out).expect("Failed to write code");
    }
}

impl<'b, 'parent> Deref for CommonOriginVisitor<'b, 'parent> {
    type Target = CommonVisitor<'b>;

    fn deref(&self) -> &Self::Target {
        &*self.parent
    }
}

impl<'b, 'parent> DerefMut for CommonOriginVisitor<'b, 'parent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent
    }
}
