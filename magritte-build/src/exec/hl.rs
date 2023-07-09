pub mod expr;
mod fields;
pub mod functions;
pub mod simple;

use std::{
    fs::File,
    ops::{Deref, DerefMut, Not},
    path::PathBuf,
};

use heck::ToUpperCamelCase;
use magritte_build::{
    imports::Imports,
    origin::{cond_of, hl_cond_of},
    rustfmt::run_rustfmt,
    ugly_diff_paths::ugly_diff_paths,
    OriginVisitor, Visitor,
};
use magritte_parse::Ty;
use magritte_types::{
    Alias, Basetype, Bitflag, Bitmask, Const, ConstAlias, Enum, Field, Handle, Mutability, Origin, Source, Struct,
    TypeRef, Union,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_each_token, ToTokens};

use crate::{
    edge_case::{EdgeCase, StructFieldConvert},
    hl::{fields::hl_field_type, functions::Arguments, simple::is_struct_simple},
};

pub struct HighLevelVisitor<'a> {
    pub doc: PathBuf,
    pub out: PathBuf,
    pub edge_cases: &'a Vec<Box<dyn EdgeCase + Send + Sync>>,
}

impl<'a> HighLevelVisitor<'a> {
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

impl<'a> Visitor for HighLevelVisitor<'a> {
    type OriginVisitor<'parent> = HighLevelOriginVisitor<'a, 'parent>
    where
        Self: 'parent;

    type VersionVisitor<'parent> = ()
    where
        Self: 'parent;

    type ExtensionVisitor<'parent> = ()
    where
        Self: 'parent;

    fn visit_origin<'b>(&mut self, _source: &Source<'b>, origin: &Origin<'b>) -> Option<Self::OriginVisitor<'_>> {
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

        Some(HighLevelOriginVisitor {
            parent: self,
            doc_dir_path,
            mod_path,
            origin: origin.to_static(),
            out: TokenStream::new(),
            imports: Imports::new(origin, "crate"),
        })
    }
}

pub struct HighLevelOriginVisitor<'a, 'parent> {
    pub(crate) parent: &'parent mut HighLevelVisitor<'a>,
    pub(crate) doc_dir_path: String,
    pub(crate) mod_path: PathBuf,
    pub(crate) origin: Origin<'static>,

    pub(crate) out: TokenStream,
    pub(crate) imports: Imports,
}

macro_rules! impl_common_use {
    ($this:ident, $name:ident) => {
        $this.imports.push_str(format!(
            "pub use {}::{};",
            $this.origin.as_rust_path("crate::common"),
            $name.name()
        ));
    };
}

impl<'c, 'parent> OriginVisitor<'parent> for HighLevelOriginVisitor<'c, 'parent> {
    fn visit_const<'a>(&mut self, _source: &Source<'a>, const_: &Const<'a>) {
        impl_common_use!(self, const_);
    }

    fn visit_const_alias<'a>(&mut self, _source: &Source<'a>, const_alias: &ConstAlias<'a>) {
        impl_common_use!(self, const_alias);
    }

    fn visit_base_type<'a>(&mut self, _source: &Source<'a>, base_type: &Basetype<'a>) {
        impl_common_use!(self, base_type);
    }

    fn visit_bitmask<'a>(&mut self, _source: &Source<'a>, bitmask: &Bitmask<'a>) {
        impl_common_use!(self, bitmask);
    }

    fn visit_bitflag<'a>(&mut self, _source: &Source<'a>, bitflag: &Bitflag<'a>) {
        impl_common_use!(self, bitflag);
    }

    fn visit_enum<'a>(&mut self, _source: &Source<'a>, enum_: &Enum<'a>) {
        impl_common_use!(self, enum_);
    }

    fn visit_handle<'a>(&mut self, source: &Source<'a>, handle: &Handle<'a>) {
        let edge_cases = self.edge_cases;
        let owner = TypeRef::Handle(handle);

        if !edge_cases.type_filter(source, owner) {
            return;
        }

        let name = handle.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, handle.original_name());
        let alias = handle.as_alias();

        let drop_fn = handle.as_drop_fn_ident();
        let clone_fn = handle.as_clone_fn_ident();

        self.imports.push("std::sync::Arc");
        self.imports.push("crate::context::Context");
        self.imports.push("crate::context::Container");
        self.imports.push("crate::context::ObjectId");
        self.imports.push("crate::context::Error");

        let fns = handle
            .functions
            .iter()
            .filter_map(|f| {
                source
                    .functions()
                    .get_by_either(f)
                    .or_else(|| source.commands().get_by_either(f).map(|f| &**f))
            })
            .filter(|f| edge_cases.function_filter(source, f))
            .map(|f| {
                let mut arguments = Arguments::new(edge_cases, source, handle, f, &mut self.imports);

                let args = arguments.as_function_arguments();
                let ret = arguments.as_return_type();

                let self_ = args.self_.expect("missing self argument");
                let self_def = self_.ty;
                let self_call = self_.ident;

                let generics = args.generics.is_empty().not().then(|| {
                    let arg = args.generics.iter().map(|a| a.ident.clone());
                    let bound = args.generics.iter().map(|a| a.bounds.clone());

                    quote! { < #(#arg: #bound),* >}
                });

                let arguments = args
                    .args
                    .iter()
                    .map(|a| {
                        let ident = &a.ident;
                        let ty = &a.ty;
                        quote! {
                            #ident: #ty
                        }
                    })
                    .collect::<Vec<_>>();

                let argument_calls = args.args.iter().map(|a| &a.ident).collect::<Vec<_>>();

                let res_inner = match ret.items.len() {
                    0 => quote! { () },
                    1 => ret.items.first().expect("missing first item").clone(),
                    _ => {
                        let items = &ret.items;
                        quote! { (#(#items),*) }
                    },
                };

                let res = if ret.result {
                    quote! { -> Result<#res_inner, Error> }
                } else {
                    quote! { -> #res_inner }
                };

                let cond = hl_cond_of(source, handle.origin(), f.origin());
                let alias = f.as_alias();
                let doc = self.doc_of(&self.doc_dir_path, f.origin(), f.original_name());

                let name = f.as_ident();
                quote! {
                    #cond
                    #alias
                    #doc
                    pub fn #name #generics (#self_def, #(#arguments),*) #res {
                        self.context. #name (#self_call, #(#argument_calls),*)
                    }
                }
            })
            .collect::<Vec<TokenStream>>();

        self.imports.serde();

        let native_path = self.origin.as_rust_path_tokens("crate::native");
        let name_field = handle.as_field_ident();

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #doc
            #alias
            #[derive( Debug)]
            pub struct #name {
                context: Arc<Context>,
                id: ObjectId,
            }

            impl Clone for #name {
                fn clone(&self) -> Self {
                    self.context. #clone_fn (self.id);
                    Self { context: Arc::clone(&self.context), id: self.id }
                }
            }

            #[cfg(feature = "serde")]
            impl serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    self.id.serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>
                {
                    let id = ObjectId::deserialize(deserializer)?;
                    crate::context::CONTEXT.with(|context| {
                        let borrow = context.borrow();
                        let context = borrow.as_ref().expect("Context not set.");

                        Ok(Self { context: Arc::clone(context), id })
                    })
                }
            }

            impl Drop for #name {
                fn drop(&mut self) {
                    if !std::thread::panicking() {
                        self.context. #drop_fn (&self.id);
                    }
                }
            }

            impl PartialEq for #name {
                fn eq(&self, other: &Self) -> bool {
                    self.id == other.id
                }
            }

            impl #name {
                #[doc = "Creates a new instance of this handle from its core components."]
                pub(crate) const unsafe fn new(context: Arc<Context>, id: ObjectId) -> Self {
                    Self { context, id }
                }

                #[doc = "Gets the object id"]
                pub fn id(&self) -> &ObjectId {
                    &self.id
                }

                #[doc = "Gets a reference to the context"]
                pub fn context(&self) -> &Context {
                    &self.context
                }

                #[doc = "Gets a reference to the context wrapped in an [`Arc`]"]
                pub fn arc_context(&self) -> &Arc<Context> {
                    &self.context
                }

                #(#fns)*
            }

            #[cfg(feature = "native")]
            unsafe impl crate::conv::IntoLowLevel for #name {
                type LowLevel = #native_path :: #name;

                unsafe fn into_low_level(&self, context: &std::sync::Arc<crate::context::Context>, bump: &bumpalo::Bump) -> Self::LowLevel {
                    *context. #name_field ().get(&self.id).expect("unknwon handle").handle()
                }
            }

            #[cfg(feature = "native")]
            unsafe impl crate::conv::FromLowLevel for #name {
                unsafe fn from_low_level(context: &std::sync::Arc<crate::context::Context>, value: <Self as crate::conv::IntoLowLevel>::LowLevel) -> Self {
                    let object_id = ObjectId::random();
                    context. #name_field ().insert(object_id, Container::new(value));
                    Self {
                        context: context.clone(),
                        id: object_id,
                    }
                }
            }
        };
    }

    fn visit_union<'a>(&mut self, source: &Source<'a>, union_: &Union<'a>) {
        let edge_cases = self.edge_cases;
        let owner = TypeRef::Union(union_);

        if !edge_cases.type_filter(source, owner) {
            return;
        }

        let name = union_.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, union_.original_name());
        let alias = union_.as_alias();

        let copy = union_.is_copy(source).then(|| quote! {, Copy});

        let variants = union_
            .fields()
            .iter()
            .map(|field| field.as_union_ident())
            .collect::<Vec<_>>();
        let variant_docs = union_
            .fields()
            .iter()
            .map(|field| self.doc_of(&self.doc_dir_path, &self.origin, field.original_name()))
            .collect::<Vec<_>>();
        let types = union_
            .fields()
            .iter()
            .map(|field| hl_field_type(edge_cases, source, owner, field, &mut self.imports))
            .collect::<Vec<_>>();

        self.imports.serde();

        let native_path = self.origin.as_rust_path_tokens("crate::native");
        let variant_field_ident = union_.fields().iter().map(|field| field.as_ident()).collect::<Vec<_>>();
        let converter = if union_.fields().iter().all(|field| field.selection().is_some()) {
            let selection = union_
                .fields()
                .iter()
                .next()
                .expect("missing fields")
                .selection()
                .expect("missing selection");
            let enum_ = source
                .enums()
                .iter()
                .find(|enum_| enum_.variants().contains_name(selection))
                .expect("missing enum");

            let enum_name = enum_.as_ident();

            self.imports.import(source, TypeRef::Enum(enum_));

            let enum_variants = union_
                .fields()
                .iter()
                .map(|field| {
                    let variant = field.selection().expect("missing selection");
                    let variant = enum_.variants().get_by_either(variant).expect("missing variant");
                    variant.as_ident()
                })
                .collect::<Vec<_>>();

            quote! {
                #[cfg(feature = "native")]
                unsafe impl crate::conv::IntoLowLevel for #name {
                    type LowLevel = (#enum_name, #native_path :: #name);

                    unsafe fn into_low_level(&self, context: &std::sync::Arc<crate::context::Context>, bump: &bumpalo::Bump) -> Self::LowLevel {
                        match self {
                            #(
                                Self::#variants (v) => #native_path :: #name { #variant_field_ident: (v.into_low_level(context, bump)) },
                            )*
                        }
                    }
                }

                #[cfg(feature = "native")]
                unsafe impl crate::conv::FromLowLevel for #name {
                    unsafe fn from_low_level(context: &std::sync::Arc<crate::context::Context>, (variant, value): <Self as crate::conv::IntoLowLevel>::LowLevel) -> Self {
                        match variant {
                            #(
                                #enum_variants => Self::#variants (value.#variant_field_ident.into_high_level(context)),
                            )*
                        }
                    }
                }
            }
        } else {
            quote! {
                #[cfg(feature = "native")]
                unsafe impl crate::conv::IntoLowLevel for #name {
                    type LowLevel = #native_path :: #name;

                    unsafe fn into_low_level(&self, context: &std::sync::Arc<crate::context::Context>, bump: &bumpalo::Bump) -> Self::LowLevel {
                        match self {
                            #(
                                Self::#variants (v) => #native_path :: #name { #variant_field_ident: (v.into_low_level(context, bump)) },
                            )*
                        }
                    }
                }

                #[cfg(feature = "native")]
                unsafe impl crate::conv::FromLowLevel for #name {
                    unsafe fn from_low_level(context: &std::sync::Arc<crate::context::Context>, _value: <Self as crate::conv::IntoLowLevel>::LowLevel) -> Self {
                        unreachable!("cannot convert native union to high level if it does not have a selection");
                    }
                }
            }
        };

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #doc
            #alias
            #[derive(Clone, PartialEq, Debug #copy)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            pub enum #name {
                #(
                    #variant_docs
                    #variants (#types),
                )*
            }

            #(
                impl From<#types> for #name {
                    fn from(v: #types) -> Self {
                        Self::#variants (v)
                    }
                }
            )*

            #converter
        }
    }

    #[allow(unused_variables)]
    fn visit_struct<'a>(&mut self, source: &Source<'a>, struct_: &Struct<'a>) {
        let edge_cases = self.edge_cases;
        let owner = TypeRef::Struct(struct_);

        if !edge_cases.type_filter(source, owner) {
            return;
        }

        let name = struct_.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, struct_.original_name());
        let alias = struct_.as_alias();

        let is_simple = is_struct_simple(self.edge_cases, source, struct_);
        if is_simple {
            self.imports.push_str(format!(
                "pub use {}::{};",
                self.origin.as_rust_path("crate::common"),
                struct_.name()
            ));
        }

        let fields = struct_
            .fields()
            .iter()
            .filter(|field| edge_cases.field_filter(source, owner, field))
            .collect::<Vec<_>>();

        let field_defs = fields
            .iter()
            .map(|field| {
                let name = edge_cases
                    .field_rename(source, owner, field)
                    .unwrap_or_else(|| field.as_ident());
                let doc = self.doc_of_child(&self.doc_dir_path, struct_.original_name(), field.original_name());
                let alias = field.as_alias();
                let ty = hl_field_type(edge_cases, source, owner, field, &mut self.imports);

                quote! {
                    #doc
                    #alias
                    pub #name: #ty
                }
            })
            .collect::<Vec<_>>();

        let copy = fields
            .iter()
            .all(|field| field.is_copy(source))
            .then(|| quote! {, Copy});
        let default = (edge_cases.struct_default_filter(source, struct_) && struct_.is_default(source))
            .then(|| quote! {, Default});

        let struct_impl = edge_cases.struct_impl(source, &mut self.imports, struct_);

        // Create the getters for the fields
        let getters = fields
            .iter()
            .map(|field| {
                let name = edge_cases
                    .field_rename(source, owner, field)
                    .unwrap_or_else(|| field.as_ident());
                let ty = hl_field_type(edge_cases, source, owner, field, &mut self.imports);
                let doc_str = format!("Get a reference to the `{}` field.", name);

                let ty = field
                    .is_copy(source)
                    .not()
                    .then(|| quote! { &#ty })
                    .unwrap_or_else(|| ty.to_token_stream());
                let ref_ = field.is_copy(source).not().then(|| quote! { & });

                quote! {
                    #[doc = #doc_str]
                    pub fn #name(&self) -> #ty {
                        #ref_ self.#name
                    }
                }
            })
            .collect::<Vec<_>>();

        let mut mut_getters = Vec::with_capacity(fields.len());
        let mut setters = Vec::with_capacity(fields.len());
        let mut builder_setters = Vec::with_capacity(fields.len());
        if !struct_.always_returned() {
            // Create the mut getters for the fields
            fields
                .iter()
                .map(|field| {
                    let name = edge_cases
                        .field_rename(source, owner, field)
                        .unwrap_or_else(|| field.as_ident());
                    let ty = hl_field_type(edge_cases, source, owner, field, &mut self.imports);
                    let ident = format_ident!("{}_mut", name);
                    let doc_str = format!("Get a mutable reference to the `{}` field.", name);

                    quote! {
                        #[doc = #doc_str]
                        pub fn #ident(&mut self) -> &mut #ty {
                            &mut self.#name
                        }
                    }
                })
                .collect_into::<Vec<_>>(&mut mut_getters);

            // Create the setters for the fields
            fields
                .iter()
                .map(|field| {
                    let name = edge_cases
                        .field_rename(source, owner, field)
                        .unwrap_or_else(|| field.as_ident());
                    let ident = format_ident!("set_{}", name);
                    let ty = hl_field_type(edge_cases, source, owner, field, &mut self.imports);
                    let doc_str = format!("Sets the `{}` field.", name);

                    quote! {
                        #[doc = #doc_str]
                        pub fn #ident(&mut self, #name: #ty) -> &mut Self {
                            self.#name = #name;

                            self
                        }
                    }
                })
                .collect_into::<Vec<_>>(&mut setters);

            // Create the builder setters for the fields
            fields
                .iter()
                .map(|field| {
                    let name = edge_cases
                        .field_rename(source, owner, field)
                        .unwrap_or_else(|| field.as_ident());
                    let ident = format_ident!("with_{}", name);
                    let ty = hl_field_type(edge_cases, source, owner, field, &mut self.imports);
                    let doc_str = format!("Sets the `{}` field in a builder way.", name);

                    quote! {
                        #[doc = #doc_str]
                        pub fn #ident(mut self, #name: #ty) -> Self {
                            self.#name = #name;

                            self
                        }
                    }
                })
                .collect_into::<Vec<_>>(&mut builder_setters);
        }

        let struct_code = is_simple.not().then(|| {
            quote! {
                #doc
                #alias
                #[derive(Clone, PartialEq, Debug #copy #default)]
                #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
                pub struct #name {
                    #(#field_defs),*
                }

                #struct_impl
            }
        });

        let ext_name = struct_.as_pointer_chain_ident();
        let extensions = struct_.extended().is_empty().not().then(|| {
            let extensions = struct_
                .extended()
                .iter()
                .filter_map(|ext| source.structs().get_by_either(ext))
                .filter(|s| !s.origin().is_disabled())
                .collect::<Vec<_>>();

            let ext_types = extensions.iter().map(|ext| {
                self.imports.import(source, TypeRef::Struct(ext));

                ext.as_ident()
            }).collect::<Vec<_>>();

            let structure_type = source.enums().get_by_either("VkStructureType").expect("missing StructureType");
            let ext_variants = extensions.iter().map(|ext| {
                let s_type = ext.fields().get_by_either("sType").expect("missing sType field");

                let variant = structure_type.variants().get_by_either(&s_type.value().expect("missing default value")).expect("missing variant");
                variant.as_enum_ident()
            }).collect::<Vec<_>>();

            let low_level_ext_types = extensions.iter().map(|ext| {
                self.imports.import(source, TypeRef::Struct(ext));

                let ident = ext.as_ident();
                let path = ext.origin().as_rust_path_tokens("crate::native");

                quote! { #path :: #ident }
            }).collect::<Vec<_>>();

            let conds = extensions.iter().map(|ext| {
                cond_of(source, struct_.origin(), ext.origin())
            }).collect::<Vec<_>>();

            let ext_docs = extensions.iter().map(|ext| {
                format!("Contains a type [`{}`] for extending [`{}`]", ext.as_ident(), struct_.as_ident())
            });

            let doc_str = format!("Extensions for [`{}`]", struct_.as_ident());

            let copy = extensions.iter().all(|ext| ext.fields().iter()
                .filter(|field| edge_cases.field_filter(source, TypeRef::Struct(ext), field))
                .all(|field| field.is_copy(source))).then(|| quote! {, Copy});

            let (mut_, native_name) = match struct_.has_p_next().expect("Struct has no pNext") {
                Mutability::Mutable => (quote! { mut }, format_ident!("BaseOutStructure")),
                Mutability::Const => (quote! { const }, format_ident!("BaseInStructure")),
            };
            
            quote! {
                #[derive(Clone, PartialEq, Debug #copy)]
                #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
                #[doc = #doc_str]
                pub enum #ext_name {
                    #(
                        #conds
                        #[doc = #ext_docs]
                        #ext_types ( #ext_types ),
                    )*
                }

                #[cfg(feature = "native")]
                unsafe impl crate::conv::IntoLowLevel for #ext_name {
                    type LowLevel = *#mut_ crate::native::vulkan1_0::#native_name;
            
                    unsafe fn into_low_level(&self, context: &std::sync::Arc<crate::context::Context>, bump: &bumpalo::Bump) -> Self::LowLevel {
                        match self {
                            #(
                                #conds
                                Self::#ext_types (ext) => (bump.alloc(ext.into_low_level(context, bump)) as *mut #low_level_ext_types).cast(),
                            )*
                            other => unreachable!("unexpected variant {:?}", other)
                        }
                    }
                }

                #[cfg(feature = "native")]
                unsafe impl crate::conv::FromLowLevel for #ext_name {
                    unsafe fn from_low_level(context: &std::sync::Arc<crate::context::Context>, value: <Self as crate::conv::IntoLowLevel>::LowLevel) -> Self {
                        assert!(!value.is_null());

                        match (*value).s_type {
                            #(
                                #conds
                                crate::native::vulkan1_0::StructureType::#ext_variants => Self::#ext_types (#ext_types::from_low_level(context, std::ptr::read(value.cast::<#low_level_ext_types>()))),
                            )*
                            other => panic!("Structure type {:?} is not a member of {}", other, stringify!(#name))
                        }
                    }
                }

                #(
                    #conds
                    impl From<#ext_types> for #ext_name {
                        fn from(ext: #ext_types) -> Self {
                            Self::#ext_types (ext)
                        }
                    }

                    #conds
                    impl TryInto<#ext_types> for #ext_name {
                        type Error = #ext_name;

                        fn try_into(self) -> Result<#ext_types, Self::Error> {
                            match self {
                                Self::#ext_types (ext) => Ok(ext),
                                other => Err(other),
                            }
                        }
                    }
                )*
            }
        });

        let extension_functions = extensions.is_some().then(|| {
            quote! {
                #[doc = "Adds an extension to the struct"]
                pub fn with_extension(mut self, ext: impl Into<#ext_name>) -> Self {
                    self.extensions.push(ext.into());

                    self
                }
            }
        });

        self.imports.serde();

        let native_path = self.origin.as_rust_path_tokens("crate::native");

        self.imports.push("std::sync::Arc");
        self.imports.push("crate::context::Context");

        let field_camel_names = fields
            .iter()
            .map(|field| format_ident!("{}", field.name().to_upper_camel_case()))
            .collect::<Vec<_>>();
        let field_names = fields.iter().map(|s| *s).map(Field::as_ident).collect::<Vec<_>>();

        let mut pre = TokenStream::new();
        let mut pre_hl = TokenStream::new();
        let native_fields = struct_
            .fields()
            .iter()
            .map(|field| {
                let name = field.as_ident();
                if edge_cases.field_filter(source, owner, field) {
                    let rename = edge_cases
                        .field_rename(source, owner, field)
                        .unwrap_or_else(|| field.as_ident());
                    let value = edge_cases
                        .field_convert(edge_cases, source, struct_, field, &mut self.imports)
                        .unwrap_or_else(|| StructFieldConvert {
                            pre: None,
                            call: if let Ty::Pointer(mut_, _) = field.ty() {
                                let mut_ = match mut_ {
                                    Mutability::Const => quote! { as *const _ },
                                    Mutability::Mutable => quote! { as *mut _ },
                                };

                                quote! { bump.alloc(self.#rename.into_low_level(context, bump)) #mut_ }
                            } else {
                                quote! { self.#rename.into_low_level(context, bump) }
                            },
                        });

                    pre.extend(value.pre);

                    let call = value.call;
                    quote! { #name: #call  }
                } else {
                    let value = edge_cases
                        .field_convert(edge_cases, source, struct_, field, &mut self.imports)
                        .expect(&format!(
                            "Field `{}` of struct `{}` is not convertible",
                            field.name(),
                            struct_.name()
                        ));

                    pre.extend(value.pre);

                    let call = value.call;
                    quote! { #name: #call  }
                }
            })
            .collect::<Vec<_>>();

        let hl_fields = fields.iter().map(|field| {
            let name = field.as_ident();
            let rename = edge_cases
                .field_rename(source, owner, field)
                .unwrap_or_else(|| field.as_ident());
            let value = edge_cases
                .field_convert_to_hl(edge_cases, source, struct_, field, &mut self.imports)
                .unwrap_or_else(|| StructFieldConvert {
                    pre: None,
                    call: if let Ty::Pointer(_, _) = field.ty() {
                        quote! { crate::conv::FromLowLevel::from_low_level(context, *value.#name) }
                    } else {
                        quote! { crate::conv::FromLowLevel::from_low_level(context, value.#name) }
                    },
                });

            pre_hl.extend(value.pre);

            let call = value.call;
            quote! { #rename: #call  }
        }).collect::<Vec<_>>();

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #struct_code

            impl #name {
                #extension_functions

                #(#getters)*
                #(#mut_getters)*
                #(#setters)*
                #(#builder_setters)*
            }

            #[cfg(feature = "native")]
            unsafe impl crate::conv::IntoLowLevel for #name {
                type LowLevel = #native_path :: #name;

                unsafe fn into_low_level(&self, context: &std::sync::Arc<crate::context::Context>, bump: &bumpalo::Bump) -> Self::LowLevel {
                    #pre

                    #native_path :: #name {
                        #(#native_fields),*
                    }
                }
            }

            #[cfg(feature = "native")]
            unsafe impl crate::conv::FromLowLevel for #name {
                unsafe fn from_low_level(context: &std::sync::Arc<crate::context::Context>, value: <Self as crate::conv::IntoLowLevel>::LowLevel) -> Self {
                    #pre_hl

                    Self {
                        #(#hl_fields),*
                    }
                }
            }

            #extensions
        };
    }

    fn visit_type_alias<'a>(&mut self, source: &Source<'a>, alias: &Alias<'a>) {
        let edge_cases = self.edge_cases;
        let owner = TypeRef::Alias(alias);

        if !edge_cases.type_filter(source, owner) {
            return;
        }

        let name = alias.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, alias.original_name());
        let alias_def = alias.as_alias();

        let of = source.find_type(alias.of()).expect("type not found");
        let ty = of.as_ident();
        self.imports.import(source, of);

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #doc
            #alias_def
            pub type #name = #ty;
        };
    }

    fn finish<'a>(self, _source: &Source<'a>) {
        let mut out = String::with_capacity(1 << 20);

        if let Some(doc) = self.doc_of_origin(&self.doc_dir_path, &self.origin) {
            out.extend_one(doc.to_string());
        }

        out.extend_one(self.imports.to_token_stream().to_string());
        out.extend_one(self.out.to_string());

        let out = run_rustfmt(out).expect("failed to run rustfmt");
        std::fs::write(&self.mod_path, out).expect("Failed to write code");
    }
}

impl<'a, 'parent> Deref for HighLevelOriginVisitor<'a, 'parent> {
    type Target = HighLevelVisitor<'a>;

    fn deref(&self) -> &Self::Target {
        &*self.parent
    }
}

impl<'a, 'parent> DerefMut for HighLevelOriginVisitor<'a, 'parent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent
    }
}
