use std::borrow::Cow;

use magritte_build::imports::Imports;
use magritte_types::{
    Expr, Field, Function, FunctionArgument, Handle, Mutability, Native, Optionality, Source, Struct, Ty, TypeRef,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::{expr::Pivot, hl::{expr::converter_value, simple::is_struct_simple}, native::r#const::constant_value};

lazy_static::lazy_static! {
    static ref CONST_BASE_PTR: Ty<'static> = Ty::Pointer(Mutability::Const, Box::new(Ty::Named(Cow::Borrowed("VkBaseInStructure"))));
    static ref MUT_BASE_PTR: Ty<'static> = Ty::Pointer(Mutability::Mutable, Box::new(Ty::Named(Cow::Borrowed("VkBaseOutStructure"))));
    static ref CONST_VOID_PTR: Ty<'static> = Ty::Pointer(Mutability::Const, Box::new(Ty::Native(Native::Void)));
    static ref MUT_VOID_PTR: Ty<'static> = Ty::Pointer(Mutability::Mutable, Box::new(Ty::Native(Native::Void)));
    static ref UUID: Ty<'static> = Ty::Array(Box::new(Ty::Native(Native::UInt(1))), Expr::Constant(Cow::Borrowed("VK_UUID_SIZE")));
    static ref LEN_U32: Ty<'static> = Ty::Native(Native::UInt(4));
    static ref LEN_U64: Ty<'static> = Ty::Native(Native::UInt(8));
    static ref LEN_USIZE: Ty<'static> = Ty::Native(Native::USize);
}

pub struct StructFieldConvert {
    pub pre: Option<TokenStream>,
    pub call: TokenStream,
}

pub trait EdgeCase {
    #[allow(unused_variables)]
    fn native_field_default(
        &self,
        source: &Source<'_>,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<TokenStream> {
        None
    }

    #[allow(unused_variables)]
    fn type_filter(&self, source: &Source<'_>, ty: TypeRef<'_, '_>) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn field_filter(&self, source: &Source<'_>, owner: TypeRef<'_, '_>, field: &Field<'_>) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn field_rename(&self, source: &Source<'_>, owner: TypeRef<'_, '_>, field: &Field<'_>) -> Option<Ident> {
        None
    }

    #[allow(unused_variables)]
    fn field_convert(
        &self,
        edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        struct_: &Struct<'_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        None
    }

    #[allow(unused_variables)]
    fn field_convert_to_hl(
        &self,
        edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        struct_: &Struct<'_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        None
    }

    #[allow(unused_variables)]
    fn field_ty(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<TokenStream> {
        None
    }

    #[allow(unused_variables)]
    fn field_wrapper_ty(
        &self,
        edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<Box<dyn FnOnce(TokenStream) -> TokenStream>> {
        None
    }

    #[allow(unused_variables)]
    fn field_named_ty(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
        name: &str,
    ) -> Option<TokenStream> {
        None
    }

    #[allow(unused_variables)]
    fn struct_impl(&self, source: &Source<'_>, imports: &mut Imports, owner: &Struct<'_>) -> Option<TokenStream> {
        None
    }

    #[allow(unused_variables)]
    fn struct_default_filter(&self, source: &Source<'_>, owner: &Struct<'_>) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn function_filter(&self, source: &Source<'_>, function: &Function<'_>) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn function_argument_ty(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: &Handle<'_>,
        function: &Function<'_>,
        argument: &FunctionArgument<'_>,
    ) -> Option<TokenStream> {
        None
    }

    #[allow(unused_variables)]
    fn function_argument_wrapper(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: &Handle<'_>,
        function: &Function<'_>,
        argument: &FunctionArgument<'_>,
    ) -> Option<Box<dyn FnOnce(TokenStream) -> TokenStream>> {
        None
    }

    #[allow(unused_variables)]
    fn function_argument_named_ty(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: &Handle<'_>,
        function: &Function<'_>,
        argument: &FunctionArgument<'_>,
        name: &str,
    ) -> Option<TokenStream> {
        None
    }
}

impl EdgeCase for Vec<Box<dyn EdgeCase + Send + Sync>> {
    fn native_field_default(
        &self,
        source: &Source<'_>,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<TokenStream> {
        self.iter()
            .find_map(|edge_case| edge_case.native_field_default(source, owner, field, imports))
    }

    fn type_filter(&self, source: &Source<'_>, ty: TypeRef<'_, '_>) -> bool {
        self.iter().all(|edge_case| edge_case.type_filter(source, ty))
    }

    fn field_filter(&self, source: &Source<'_>, owner: TypeRef<'_, '_>, field: &Field<'_>) -> bool {
        self.iter()
            .all(|edge_case| edge_case.field_filter(source, owner, field))
    }

    fn field_rename(&self, source: &Source<'_>, owner: TypeRef<'_, '_>, field: &Field<'_>) -> Option<Ident> {
        self.iter()
            .find_map(|edge_case| edge_case.field_rename(source, owner, field))
    }

    fn field_convert(
        &self,
        edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        struct_: &Struct<'_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        self.iter()
            .find_map(|edge_case| edge_case.field_convert(edge_cases, source, struct_, field, imports))
    }

    fn field_convert_to_hl(
        &self,
        edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        struct_: &Struct<'_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        self.iter()
            .find_map(|edge_case| edge_case.field_convert_to_hl(edge_cases, source, struct_, field, imports))
    }

    fn field_ty(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<TokenStream> {
        self.iter()
            .find_map(|edge_case| edge_case.field_ty(source, imports, owner, field))
    }

    fn field_wrapper_ty(
        &self,
        edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<Box<dyn FnOnce(TokenStream) -> TokenStream>> {
        self.iter()
            .find_map(|edge_case| edge_case.field_wrapper_ty(edge_cases, source, imports, owner, field))
    }

    fn field_named_ty(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
        name: &str,
    ) -> Option<TokenStream> {
        self.iter()
            .find_map(|edge_case| edge_case.field_named_ty(source, imports, owner, field, name))
    }

    fn struct_default_filter(&self, source: &Source<'_>, owner: &Struct<'_>) -> bool {
        self.iter()
            .all(|edge_case| edge_case.struct_default_filter(source, owner))
    }

    fn struct_impl(&self, source: &Source<'_>, imports: &mut Imports, owner: &Struct<'_>) -> Option<TokenStream> {
        self.iter()
            .filter_map(|edge_case| edge_case.struct_impl(source, imports, owner))
            .fold(None, |acc, item| {
                Some(match acc {
                    Some(acc) => quote! {
                        #acc
                        #item
                    },
                    None => item,
                })
            })
    }
    fn function_filter(&self, source: &Source<'_>, function: &Function<'_>) -> bool {
        self.iter().all(|edge_case| edge_case.function_filter(source, function))
    }

    fn function_argument_ty(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: &Handle<'_>,
        function: &Function<'_>,
        argument: &FunctionArgument<'_>,
    ) -> Option<TokenStream> {
        self.iter()
            .find_map(|edge_case| edge_case.function_argument_ty(source, imports, owner, function, argument))
    }

    fn function_argument_wrapper(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: &Handle<'_>,
        function: &Function<'_>,
        argument: &FunctionArgument<'_>,
    ) -> Option<Box<dyn FnOnce(TokenStream) -> TokenStream>> {
        self.iter()
            .find_map(|edge_case| edge_case.function_argument_wrapper(source, imports, owner, function, argument))
    }

    fn function_argument_named_ty(
        &self,
        source: &Source<'_>,
        imports: &mut Imports,
        owner: &Handle<'_>,
        function: &Function<'_>,
        argument: &FunctionArgument<'_>,
        name: &str,
    ) -> Option<TokenStream> {
        self.iter().find_map(|edge_case| {
            edge_case.function_argument_named_ty(source, imports, owner, function, argument, name)
        })
    }
}

#[derive(Clone, Copy, Default)]
pub struct HandleLowLevelConverterEdgeCase;

impl EdgeCase for HandleLowLevelConverterEdgeCase {
    fn field_convert_to_hl(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        _struct_: &Struct<'_>,
        field: &Field<'_>,
        _imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if field.optional().is_optional() {
            if let Ty::Named(name) = field.ty() {
                if let Some(handle) = source.handles().get_by_either(name) {
                    let name = field.as_ident();
                    let path = handle.origin().as_rust_path_tokens("crate::native");
                    let ty = handle.as_ident();

                    Some(StructFieldConvert {
                        pre: None,
                        call: quote! {
                            if value.#name == #path:: #ty ::null() {
                                None
                            } else {
                                Some(crate::conv::FromLowLevel::from_low_level(context, value.#name))
                            }
                        },
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct StringArrayEdgeCase;

impl EdgeCase for StringArrayEdgeCase {
    fn field_convert(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        _struct_: &Struct<'_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if let Ty::StringArray(len) | Ty::Array(box Ty::Native(Native::Char), len) = field.ty() {
            let value = field.as_ident();
            let value_bytes = format_ident!("{}_bytes", value);

            let len = constant_value(len, source, imports);

            let pre = quote! {
                let #value_bytes = self. #value .as_bytes();

                debug_assert!(memchr::memchr(0, #value_bytes).is_none(), "string array contains null characters");
                debug_assert!(#value_bytes.len() <= #len as usize, "string is too long for the backing array");

                let mut #value: [std::ffi::c_char; #len as usize] = [0; #len as usize];
                #value[0..#value_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
                    #value_bytes.as_ptr() as *const std::ffi::c_char,
                    #value_bytes.len(),
                ));
            };

            Some(StructFieldConvert {
                pre: Some(pre),
                call: value.to_token_stream(),
            })
        } else {
            None
        }
    }

    fn field_convert_to_hl(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        _source: &Source<'_>,
        _struct_: &Struct<'_>,
        field: &Field<'_>,
        _imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if let Ty::StringArray(_) | Ty::Array(box Ty::Native(Native::Char), _) = field.ty() {
            let value = field.as_ident();
            let value_cstr = format_ident!("{}_cstr", value);

            let pre = quote! {
                let #value_cstr = std::ffi::CStr::from_ptr(value.#value.as_ptr());

                let #value = #value_cstr.to_string_lossy().into_owned();
            };

            Some(StructFieldConvert {
                pre: Some(pre),
                call: value.to_token_stream(),
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default)]

pub struct SmallVecEdgeCase;

impl EdgeCase for SmallVecEdgeCase {
    fn field_convert(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        _source: &Source<'_>,
        _struct_: &Struct<'_>,
        field: &Field<'_>,
        _imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if let Ty::Slice(mut_, _, _) = field.ty() {
            let value = field.as_ident();
            let ptr_kind = match mut_ {
                Mutability::Const => quote! { as_ptr },
                Mutability::Mutable => quote! { as_mut_ptr },
            };

            let pre = quote! {
                let #value = bump.alloc_slice_fill_iter(self. #value .iter().map(|x| x.into_low_level(context, bump))). #ptr_kind ().cast();
            };

            Some(StructFieldConvert {
                pre: Some(pre),
                call: value.to_token_stream(),
            })
        } else {
            None
        }
    }

    fn field_convert_to_hl(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        struct_: &Struct<'_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if let Ty::Slice(_, _, len) = field.ty() {
            if let Some(len_field) = len.variables().pop() {
                let len_field = struct_.fields().get_by_either(&len_field).expect("missing length field");
                let len_ident = len_field.as_ident();
    
                let len_value = if len_field.ty().is_native() {
                    converter_value(len, &|_| quote! { value.#len_ident }, source, imports)
                } else {
                    converter_value(len, &|_| quote! { value.#len_ident .bits() }, source, imports)
                };
    
                let value = field.as_ident();
                let value_len = format_ident!("{}_len", value);
                let pre = quote! {
                    let #value_len = #len_value;
                    let mut #value = SmallVec::with_capacity(#value_len as usize);
                    for i in 0..#value_len {
                        #value.push(crate::conv::FromLowLevel::from_low_level(context, value.#value.add(i as usize).read()));
                    }
                };
    
                Some(StructFieldConvert {
                    pre: Some(pre),
                    call: value.to_token_stream(),
                })
            } else {
                let len_value = constant_value(len, source, imports);
    
                let value = field.as_ident();
                let pre = quote! {
                    let mut #value = SmallVec::with_capacity(#len_value as usize);
                    for i in 0..#len_value {
                        #value.push(crate::conv::FromLowLevel::from_low_level(context, value.#value.add(i as usize).read()));
                    }
                };
    
                Some(StructFieldConvert {
                    pre: Some(pre),
                    call: value.to_token_stream(),
                })
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct VersionEdgeCase;

impl VersionEdgeCase {
    const FIELDS: &'static [&'static str] = &[
        "api_version",
        "driver_version",
        "spec_version",
        "implementation_version",
        "application_version",
        "engine_version",
    ];
}

impl EdgeCase for VersionEdgeCase {
    fn field_ty(
        &self,
        _source: &Source<'_>,
        imports: &mut Imports,
        _owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<TokenStream> {
        Self::FIELDS.contains(&field.original_name()).then(|| {
            imports.push("crate::Version");

            quote! { Version }
        })
    }
}

#[derive(Clone, Copy, Default)]
pub struct OpaqueFilterEdgeCase;

impl EdgeCase for OpaqueFilterEdgeCase {
    fn type_filter(&self, source: &Source<'_>, ty: TypeRef<'_, '_>) -> bool {
        !ty.has_opaque(source)
    }

    fn function_filter(&self, source: &Source<'_>, function: &Function<'_>) -> bool {
        !function.arguments().iter().any(|arg| arg.ty.is_opaque(source))
    }
}

#[derive(Clone, Copy, Default)]
pub struct AllocationCallbackEdgeCase;

impl EdgeCase for AllocationCallbackEdgeCase {
    fn type_filter(&self, _source: &Source<'_>, ty: TypeRef<'_, '_>) -> bool {
        ty.original_name() != "VkAllocationCallbacks"
    }
}

#[derive(Clone, Copy, Default)]
pub struct ComponentMappingEdgeCase;

impl EdgeCase for ComponentMappingEdgeCase {
    fn struct_default_filter(&self, _source: &Source<'_>, owner: &Struct<'_>) -> bool {
        owner.original_name() != "VkComponentMapping"
    }

    fn struct_impl(&self, _source: &Source<'_>, _imports: &mut Imports, owner: &Struct<'_>) -> Option<TokenStream> {
        if owner.original_name() != "VkComponentMapping" {
            return None;
        }

        // Generate all the swizzle combinations
        let mut swizzles = Vec::new();
        const TYPES: &[char] = &['R', 'G', 'B', 'A'];
        for &r in TYPES {
            let r_ident = format_ident!("{}", r);

            for &g in TYPES {
                let g_ident = format_ident!("{}", g);

                for &b in TYPES {
                    let b_ident = format_ident!("{}", b);

                    for &a in TYPES {
                        let a_ident = format_ident!("{}", a);

                        let name = format_ident!(
                            "{}{}{}{}",
                            r.to_ascii_lowercase(),
                            g.to_ascii_lowercase(),
                            b.to_ascii_lowercase(),
                            a.to_ascii_lowercase()
                        );

                        if r == a && g == a && b == a {
                            let doc_str = format!("Splats a new `ComponentMapping` with the given component: [`ComponentSwizzle::{r_ident}`]");
                            swizzles.push(quote! {
                                #[doc = #doc_str]
                                #[inline]
                                pub const fn #name() -> Self {
                                    Self::splat(
                                        ComponentSwizzle::#r_ident,
                                    )
                                }
                            });
                        } else {
                            let doc_str = format!("Creates a new `ComponentMapping` with the given components. (`r`: [`ComponentSwizzle::{r_ident}`], `g`: [`ComponentSwizzle::{g_ident}`], `b`: [`ComponentSwizzle::{b_ident}`], `a`: [`ComponentSwizzle::{a_ident}`])");
                            swizzles.push(quote! {
                                #[doc = #doc_str]
                                #[inline]
                                pub const fn #name() -> Self {
                                    Self::new(
                                        ComponentSwizzle::#r_ident,
                                        ComponentSwizzle::#g_ident,
                                        ComponentSwizzle::#b_ident,
                                        ComponentSwizzle::#a_ident,
                                    )
                                }
                            });
                        }
                    }
                }
            }
        }

        let name = owner.as_ident();
        Some(quote! {
            impl Default for #name {
                #[inline]
                fn default() -> Self {
                    Self::splat(ComponentSwizzle::Identity)
                }
            }

            impl #name {
                #[doc = "Creates a new `ComponentMapping` with the given components."]
                #[inline]
                pub const fn new(r: ComponentSwizzle, g: ComponentSwizzle, b: ComponentSwizzle, a: ComponentSwizzle) -> Self {
                    Self { r, g, b, a }
                }

                #[doc = "Creates a new `ComponentMapping` with all components set to the given swizzle."]
                #[inline]
                pub const fn splat(swizzle: ComponentSwizzle) -> Self {
                    Self::new(swizzle, swizzle, swizzle, swizzle)
                }

                #[doc = "Creates a new `ComponentMapping` with all components set to [`ComponentSwizzle::Identity`]."]
                #[inline]
                pub const fn identity() -> Self {
                    Self::default()
                }

                #[doc = "Creates a new `ComponentMapping` with all components set to [`ComponentSwizzle::Zero`]."]
                #[inline]
                pub const fn zero() -> Self {
                    Self {
                        r: ComponentSwizzle::Zero,
                        g: ComponentSwizzle::Zero,
                        b: ComponentSwizzle::Zero,
                        a: ComponentSwizzle::Zero,
                    }
                }

                #[doc = "Creates a new `ComponentMapping` with all components set to [`ComponentSwizzle::One`]."]
                #[inline]
                pub const fn one() -> Self {
                    Self {
                        r: ComponentSwizzle::One,
                        g: ComponentSwizzle::One,
                        b: ComponentSwizzle::One,
                        a: ComponentSwizzle::One,
                    }
                }

                #(#swizzles)*
            }
        })
    }
}

#[derive(Clone, Copy, Default)]
pub struct BaseStructFilter;

impl EdgeCase for BaseStructFilter {
    fn type_filter(&self, _source: &Source<'_>, ty: TypeRef<'_, '_>) -> bool {
        if ty.original_name() == "VkBaseInStructure" || ty.original_name() == "VkBaseOutStructure" {
            return false;
        }

        true
    }
}

#[derive(Clone, Copy, Default)]
pub struct Bool32EdgeCase;

impl EdgeCase for Bool32EdgeCase {
    fn field_named_ty(
        &self,
        _source: &Source<'_>,
        _imports: &mut Imports,
        _owner: TypeRef<'_, '_>,
        _field: &Field<'_>,
        name: &str,
    ) -> Option<TokenStream> {
        (name == "VkBool32").then(|| quote! { bool })
    }
}

#[derive(Clone, Copy, Default)]
pub struct OptionalFieldEdgeCase;

impl EdgeCase for OptionalFieldEdgeCase {
    fn field_wrapper_ty(
        &self,
        edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        _imports: &mut Imports,
        _owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<Box<dyn FnOnce(TokenStream) -> TokenStream>> {
        if matches!(field.optional(), Optionality::Yes | Optionality::Sometimes) {
            // Natives can just be zero/false.
            if field.ty().is_native()
                || field.ty().is_bool32()
                || field.ty().is_device_size()
                || field.ty().is_array()
                || field.ty().is_slice()
            {
                return None;
            }

            // Bitflag, enum and bitmasks can just be zero.
            if let Ty::Named(name) = field.ty() {
                let ref_ = source.resolve_type(name).expect("unknown type");
                if matches!(ref_, TypeRef::Bitflag(_) | TypeRef::Bitmask(_) | TypeRef::Enum(_)) {
                    return None;
                }

                // Basetypes can just be zero.
                if let TypeRef::Basetype(basetype) = ref_ {
                    if basetype.of().is_native() {
                        return None;
                    }
                }

                // Types that are copy have default and are easy enough to handle
                if let TypeRef::Struct(struct_) = ref_ {
                    if struct_.is_copy(source) || is_struct_simple(edge_cases, source, struct_) {
                        return None;
                    }
                }
            }

            return Some(Box::new(|ty| quote! { Option<#ty> }));
        }

        None
    }

    fn field_convert(
        &self,
        edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        _struct_: &Struct<'_>,
        field: &Field<'_>,
        _imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if matches!(field.optional(), Optionality::Yes | Optionality::Sometimes) {
            // Natives can just be zero/false.
            if field.ty().is_native()
                || field.ty().is_bool32()
                || field.ty().is_device_size()
                || field.ty().is_array()
                || field.ty().is_slice()
            {
                return None;
            }

            // Bitflag, enum and bitmasks can just be zero.
            if let Ty::Named(name) = field.ty() {
                let ref_ = source.resolve_type(name).expect("unknown type");
                if matches!(ref_, TypeRef::Bitflag(_) | TypeRef::Bitmask(_) | TypeRef::Enum(_)) {
                    return None;
                }

                // Basetypes can just be zero.
                if let TypeRef::Basetype(basetype) = ref_ {
                    if basetype.of().is_native() {
                        return None;
                    }
                }

                // Types that are copy have default and are easy enough to handle
                if let TypeRef::Struct(struct_) = ref_ {
                    if struct_.is_copy(source) || is_struct_simple(edge_cases, source, struct_) {
                        return None;
                    }
                }
            }

            let default = match field.ty() {
                Ty::Slice(mut_, _, _) | Ty::Pointer(mut_, _) | Ty::NullTerminatedString(mut_) => {
                    let mut_ = match mut_ {
                        Mutability::Const => quote! { null },
                        Mutability::Mutable => quote! { null_mut },
                    };
                    quote! { std::ptr::#mut_   }
                },
                _ => quote! { Default::default },
            };

            let value = field.as_ident();
            return Some(StructFieldConvert {
                pre: None,
                call: if let Ty::Pointer(mut_, _) = field.ty() {
                    let mut_ = match mut_ {
                        Mutability::Const => quote! { as *const _ },
                        Mutability::Mutable => quote! { as *mut _ },
                    };

                    quote! {
                       self . #value .as_ref().map(|v|  bump.alloc(v.into_low_level(context, bump)) #mut_) . unwrap_or_else(#default)
                    }
                } else {
                    quote! {
                        self . #value .as_ref().map(|v| v.into_low_level(context, bump)) . unwrap_or_else(#default)
                    }
                },
            });
        }

        None
    }
}

#[derive(Clone, Copy, Default)]
pub struct LenEdgeCase;

impl EdgeCase for LenEdgeCase {
    fn field_filter(&self, _source: &Source<'_>, owner: TypeRef<'_, '_>, field: &Field<'_>) -> bool {
        if field.ty() == &*LEN_U32 || field.ty() == &*LEN_U64 || field.ty() == &*LEN_USIZE {
            if let TypeRef::Struct(struct_) = owner {
                if struct_.fields().iter().any(|f| f.has_length(field.original_name())) {
                    return false;
                }
            }
        }

        true
    }

    fn field_convert(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        struct_: &Struct<'_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if field.ty() == &*LEN_U32 || field.ty() == &*LEN_U64 || field.ty() == &*LEN_USIZE {
            if let Some(from_field) = struct_.fields().iter().find(|f| f.has_length(field.original_name())) {
                let len_ident = format_ident!("len_{}", from_field.name());
                let len = from_field.ty().length().expect("missing length");

                let pivot = len.pivot("len", &len.variables().pop().expect("no variable in length"));

                let len_value = converter_value(&pivot, &|_| quote! { #len_ident }, source, imports);

                let array = from_field.as_ident();

                let ty = if field.ty() == &*LEN_U32 {
                    quote! { u32 }
                } else if field.ty() == &*LEN_U64 {
                    quote! { u64 }
                } else {
                    quote! { usize }
                };

                return Some(StructFieldConvert {
                    pre: Some(quote! {
                        let #len_ident = self. #array .len() as #ty;
                    }),
                    call: len_value,
                });
            }
        }

        None
    }
}

pub struct UserDataEdgeCase;

impl UserDataEdgeCase {
    const FIELDS: &'static [&'static str] = &["pUserData"];
}

impl EdgeCase for UserDataEdgeCase {
    fn field_ty(
        &self,
        _source: &Source<'_>,
        imports: &mut Imports,
        _owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<TokenStream> {
        if Self::FIELDS.contains(&field.original_name()) && (field.ty().is_void_ptr_mut() || field.ty().is_void_ptr()) {
            imports.push("std::any::Any");

            Some(quote! {
                Box<dyn Any + Send + Sync + 'static>
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct PDataEdgeCase;

impl PDataEdgeCase {
    const FIELDS: &'static [&'static str] = &["pData", "pInitialData", "pTag", "pShaderGroupCaptureReplayHandle"];
}

impl EdgeCase for PDataEdgeCase {
    fn field_filter(&self, _source: &Source<'_>, _owner: TypeRef<'_, '_>, field: &Field<'_>) -> bool {
        field.original_name() != "pShaderGroupCaptureReplayHandle"
    }

    fn field_ty(
        &self,
        _source: &Source<'_>,
        _imports: &mut Imports,
        _owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<TokenStream> {
        if Self::FIELDS.contains(&field.original_name()) && (field.ty().is_void_ptr_mut() || field.ty().is_void_ptr()) {
            Some(quote! {
                Vec<u8>
            })
        } else {
            None
        }
    }

    fn field_convert(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        _source: &Source<'_>,
        _struct_: &Struct<'_>,
        field: &Field<'_>,
        _imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if Self::FIELDS.contains(&field.original_name()) && (field.ty().is_void_ptr_mut() || field.ty().is_void_ptr()) {
            let value = field.as_ident();
            let as_ptr = match field.ty().as_ptr_or_slice().0 {
                Mutability::Const => quote! { as_ptr },
                Mutability::Mutable => quote! { as_mut_ptr },
            };

            Some(StructFieldConvert {
                pre: None,
                call: quote! {
                    self. #value . #as_ptr ().cast()
                },
            })
        } else {
            None
        }
    }

    fn field_convert_to_hl(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        struct_: &Struct<'_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if Self::FIELDS.contains(&field.original_name()) && (field.ty().is_void_ptr_mut() || field.ty().is_void_ptr()) {
            let len = field.ty().length().expect("missing length");
            let len_field = len.variables().pop().expect("missing length field");
            let len_field = struct_.fields().get_by_either(&len_field).expect("missing length field");
            let len_ident = len_field.as_ident();

            let len_value = if len_field.ty().is_native() {
                converter_value(len, &|_| quote! { value.#len_ident }, source, imports)
            } else {
                converter_value(len, &|_| quote! { value.#len_ident .bits() }, source, imports)
            };
    
            let value = field.as_ident();
            let value_ptr = format_ident!("{}_ptr", value);
            let value_len = format_ident!("{}_len", value);
            let pre = quote! {
                let #value_len = #len_value;
                let mut #value = Vec::with_capacity(#value_len as usize);
                let #value_ptr = value.#value as *const u8;
                for i in 0..#value_len {
                    #value.push(crate::conv::FromLowLevel::from_low_level(context, #value_ptr.add(i as usize).read()));
                }
            };

            Some(StructFieldConvert {
                pre: Some(pre),
                call: value.to_token_stream(),
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct PNextEdgeCase;

impl EdgeCase for PNextEdgeCase {
    fn field_filter(&self, _source: &Source<'_>, owner: TypeRef<'_, '_>, field: &Field<'_>) -> bool {
        if field.original_name() == "pNext" && (field.ty() == &*CONST_BASE_PTR || field.ty() == &*MUT_BASE_PTR) {
            // Check whether the structure is extending another or is extended by another.
            // If it only ever extends then the field can be skipped all together.
            let struct_ = owner.as_struct();
            if struct_.extended().is_empty() {
                return false;
            }
        }

        true
    }

    fn field_rename(&self, _source: &Source<'_>, owner: TypeRef<'_, '_>, field: &Field<'_>) -> Option<Ident> {
        if field.original_name() == "pNext" && (field.ty() == &*CONST_BASE_PTR || field.ty() == &*MUT_BASE_PTR) {
            // Check whether the structure is extending another or is extended by another.
            // If it only ever extends then the field can be skipped all together.
            let struct_ = owner.as_struct();
            if struct_.extended().is_empty() {
                return None;
            }

            return Some(Ident::new("extensions", Span::call_site()));
        }

        None
    }

    fn field_convert(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        _source: &Source<'_>,
        struct_: &Struct<'_>,
        field: &Field<'_>,
        _imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if field.original_name() == "pNext" && (field.ty() == &*CONST_BASE_PTR || field.ty() == &*MUT_BASE_PTR) {
            let null = if field.ty() == &*CONST_BASE_PTR {
                quote! { null }
            } else {
                quote! { null_mut }
            };

            if struct_.extended().is_empty() {
                return Some(StructFieldConvert {
                    pre: None,
                    call: quote! { std::ptr:: #null () },
                });
            }

            let pre = quote! {
                let mut next = std::ptr::#null ();

                let mut extensions = self.extensions.iter();
                while let Some(ext) = extensions.next() {
                    // First we get the next extension as a raw pointer
                    let ext = ext.into_low_level(context, bump);

                    (*ext).next = next;
                    next = ext;
                }
            };

            return Some(StructFieldConvert {
                pre: Some(pre),
                call: quote! { next },
            });
        }

        None
    }

    fn field_convert_to_hl(
            &self,
            _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
            _source: &Source<'_>,
            struct_: &Struct<'_>,
            field: &Field<'_>,
            _imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if field.original_name() == "pNext" && (field.ty() == &*CONST_BASE_PTR || field.ty() == &*MUT_BASE_PTR) {
            if struct_.extended().is_empty() {
                return None;
            }

            let pre = quote! {
                let mut next = value.p_next;
                let mut extensions = SmallVec::new();
                while !next.is_null() {
                    extensions.push(crate::conv::FromLowLevel::from_low_level(context, next));
                    next = std::ptr::read(next).next;
                }
            };

            return Some(StructFieldConvert {
                pre: Some(pre),
                call: quote! { extensions },
            });
        }

        None
    }

    fn field_ty(
        &self,
        _source: &Source<'_>,
        imports: &mut Imports,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<TokenStream> {
        if field.original_name() == "pNext" && (field.ty() == &*CONST_BASE_PTR || field.ty() == &*MUT_BASE_PTR) {
            // Check whether the structure is extending another or is extended by another.
            // If it only ever extends then the field can be skipped all together.
            let struct_ = owner.as_struct();
            if struct_.extended().is_empty() {
                return None;
            }

            imports.push("smallvec::SmallVec");

            let owner_next_ident = owner.as_struct().as_pointer_chain_ident();
            Some(quote! {
                SmallVec<[#owner_next_ident; 1]>
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct UUIDEdgeCase;

impl EdgeCase for UUIDEdgeCase {
    fn field_ty(
        &self,
        _source: &Source<'_>,
        _imports: &mut Imports,
        _owner: TypeRef<'_, '_>,
        field: &Field<'_>,
    ) -> Option<TokenStream> {
        if field.ty() == &*UUID {
            Some(quote! { uuid::Uuid })
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct STypeFilter;

impl EdgeCase for STypeFilter {
    fn native_field_default(
        &self,
        source: &Source<'_>,
        owner: TypeRef<'_, '_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<TokenStream> {
        (field.is_s_type()
            && owner.original_name() != "VkBaseInStructure"
            && owner.original_name() != "VkBaseOutStructure")
            .then(|| {
                let value = field.value().expect("missing value for `sType`");

                let s_type = source
                    .enums()
                    .get_by_name("VkStructureType")
                    .expect("missing `VkStructureType` enum");
                let variant = s_type
                    .variants()
                    .iter()
                    .find(|variant| variant.original_name() == value)
                    .expect("missing variant for `sType`");

                let variant_ident = variant.as_enum_ident();

                imports.import(source, TypeRef::Enum(s_type));

                quote! {
                    StructureType::#variant_ident
                }
            })
    }

    fn field_filter(&self, _source: &Source<'_>, _owner: TypeRef<'_, '_>, field: &Field<'_>) -> bool {
        !field.is_s_type()
    }

    fn field_convert(
        &self,
        _edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &Source<'_>,
        _struct_: &Struct<'_>,
        field: &Field<'_>,
        imports: &mut Imports,
    ) -> Option<StructFieldConvert> {
        if field.is_s_type() {
            let value = field.value().expect("missing value for `sType`");

            let s_type = source
                .enums()
                .get_by_name("VkStructureType")
                .expect("missing `VkStructureType` enum");
            let variant = s_type
                .variants()
                .iter()
                .find(|variant| variant.original_name() == value)
                .expect("missing variant for `sType`");

            let variant_ident = variant.as_enum_ident();

            imports.import(source, TypeRef::Enum(s_type));

            Some(StructFieldConvert {
                pre: None,
                call: quote! { StructureType::#variant_ident },
            })
        } else {
            None
        }
    }
}
