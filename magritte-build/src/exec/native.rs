use std::{
    fs::File,
    ops::{Deref, DerefMut, Not},
    path::PathBuf,
};

use magritte_build::{
    imports::Imports, rustfmt::run_rustfmt, ugly_diff_paths::ugly_diff_paths, OriginVisitor, Visitor,
};
use magritte_parse::Field;
use magritte_types::{
    Alias, Basetype, Bitflag, Bitmask, CommandAlias, Const, ConstAlias, Enum, FunctionPointer, Handle, OpaqueType,
    Origin, Ref, Source, Struct, TypeRef, Union,
};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_each_token, ToTokens};

use field::field_type;

use crate::{edge_case::EdgeCase, hl::simple::is_struct_simple};

pub mod bitflags;
pub mod bits;
pub mod r#const;
pub mod field;

pub struct NativeVisitor<'b> {
    pub edge_cases: &'b Vec<Box<dyn EdgeCase + Send + Sync>>,
    pub doc: PathBuf,
    pub out: PathBuf,
}

impl<'b> NativeVisitor<'b> {
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

impl<'b> Visitor for NativeVisitor<'b> {
    type OriginVisitor<'parent> = NativeBackendOriginVisitor<'b, 'parent>
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

        Some(NativeBackendOriginVisitor {
            parent: self,
            doc_dir_path,
            mod_path,
            origin: origin.to_static(),
            out: TokenStream::new(),
            imports: Imports::new(origin, "crate::native"),
            vtable: Vec::new(),
        })
    }
}

pub struct VTableItem {
    pub name: Ident,
    pub ty: Ident,
    pub original_name: String,
}

pub struct NativeBackendOriginVisitor<'b, 'parent> {
    pub(crate) parent: &'parent mut NativeVisitor<'b>,
    pub(crate) doc_dir_path: String,
    pub(crate) mod_path: PathBuf,
    pub(crate) origin: Origin<'static>,

    pub(crate) out: TokenStream,
    pub(crate) imports: Imports,

    pub(crate) vtable: Vec<VTableItem>,
}

macro_rules! impl_common_use {
    ($this:ident, $name:ident) => {
        let name = $name.as_ident();
        let path = $this.origin.as_rust_path_tokens("crate::common");

        let mut out = &mut $this.out;
        quote_each_token! {
            out

            pub use #path :: #name;
        }
    };
}

impl<'b, 'parent> OriginVisitor<'parent> for NativeBackendOriginVisitor<'b, 'parent> {
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

    fn visit_opaque_type<'a>(&mut self, _source: &Source<'a>, opaque_type: &OpaqueType<'a>) {
        let name = opaque_type.as_ident();

        let ty = match opaque_type.original_name() {
            "Display" => quote! { std::ffi::c_void },
            "VisualID" => quote! { std::os::raw::c_ulong },
            "Window" => quote! {  std::os::raw::c_ulong },
            "RROutput" => quote! { std::os::raw::c_ulong },
            "wl_display" => quote! { std::ffi::c_void },
            "wl_surface" => quote! { std::ffi::c_void },
            "HINSTANCE" => quote! { isize },
            "HWND" => quote! { isize },
            "HMONITOR" => quote! { isize },
            "HANDLE" => quote! { isize },
            "SECURITY_ATTRIBUTES" => quote! { std::ffi::c_void },
            "DWORD" => quote! { u32 },
            "LPCWSTR" => quote! { *const u16 },
            "xcb_connection_t" => quote! { std::ffi::c_void },
            "xcb_visualid_t" => quote! { u32 },
            "xcb_window_t" => quote! { u32 },
            "IDirectFB" => quote! { std::ffi::c_void },
            "IDirectFBSurface" => quote! { std::ffi::c_void },
            "zx_handle_t" => quote! { u32 },
            "GgpStreamDescriptor" => quote! { u32 },
            "GgpFrameToken" => quote! { u64 },
            "_screen_context" => quote! { std::ffi::c_void },
            "_screen_window" => quote! { std::ffi::c_void },
            other => unreachable!("unknown opaque type: {other}"),
        };

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #[allow(non_camel_case_types)]
            #[doc = "An opaque type that is defined outside of vulkan."]
            pub type #name = #ty;
        };
    }

    fn visit_type_alias<'a>(&mut self, source: &Source<'a>, alias: &Alias<'a>) {
        // this check is necessary because some flags are not actually defined.
        if let Some(of) = source.resolve(alias.of()).and_then(Ref::as_type_ref) {
            self.imports.push_origin(source, of.origin(), of.name());

            let name = alias.as_ident();
            let of_ident = of.as_ident();
            let doc_alias = alias.as_alias();

            let doc_str = format!("See [`{}`]", of.name());

            let mut out = &mut self.out;
            quote_each_token! {
                out

                #[doc = #doc_str]
                #doc_alias
                pub type #name = #of_ident;
            };
        }
    }

    fn visit_struct<'a>(&mut self, source: &Source<'a>, struct_: &Struct<'a>) {
        let name = struct_.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, struct_.original_name());
        let alias = struct_.as_alias();

        let is_simple = is_struct_simple(self.edge_cases, source, struct_);
        if is_simple && self.edge_cases.type_filter(source, TypeRef::Struct(struct_)) {
            self.imports.push_str(format!(
                "pub use {}::{};",
                self.origin.as_rust_path("crate::common"),
                struct_.name()
            ));

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

        let field_idents = struct_.fields().iter().map(Field::as_ident);

        let field_default_values = struct_
            .fields()
            .iter()
            .map(|field| {
                self.edge_cases
                    .native_field_default(source, TypeRef::Struct(struct_), field, &mut self.imports)
            })
            .map(|value| value.unwrap_or_else(|| quote! { unsafe { std::mem::zeroed() } }))
            .collect::<Vec<_>>();

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #doc
            #alias
            #[derive(Clone, Copy)]
            #[repr(C)]
            pub struct #name {
                #(#fields),*
            }

            impl Default for #name {
                fn default() -> Self {
                    Self {
                        #(#field_idents: #field_default_values),*
                    }
                }
            }
        };
    }

    fn visit_union<'a>(&mut self, source: &Source<'a>, union_: &Union<'a>) {
        let name = union_.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, union_.original_name());
        let alias = union_.as_alias();

        let fields = union_
            .fields()
            .iter()
            .map(|field| {
                let name = field.as_ident();
                let doc = self.doc_of_child(&self.doc_dir_path, union_.original_name(), field.original_name());
                let ty = field_type(source, field.ty(), &mut self.imports);
                let alias = field.as_alias();

                quote! {
                    #doc
                    #alias
                    pub #name: #ty
                }
            })
            .collect::<Vec<_>>();

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #doc
            #alias
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub union #name {
                #(#fields),*
            }
        };
    }

    fn visit_handle<'a>(&mut self, _source: &Source<'a>, handle: &Handle<'a>) {
        let name = handle.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, handle.original_name());
        let alias = handle.as_alias();

        let storage = if handle.dispatchable() {
            quote! { *mut std::ffi::c_void }
        } else {
            quote! { u64 }
        };

        let null = if handle.dispatchable() {
            quote! { ::std::ptr::null_mut() as _ }
        } else {
            quote! { 0 }
        };

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #doc
            #[derive(Clone, Copy, PartialEq, Eq)]
            #alias
            #[repr(transparent)]
            pub struct #name(#storage);

            impl #name {
                pub const fn null() -> Self {
                    Self(#null)
                }

                pub const fn raw(&self) -> #storage {
                    self.0
                }
            }

            impl Default for #name {
                fn default() -> Self {
                    Self::null()
                }
            }
        };
    }

    fn visit_function_pointer<'a>(&mut self, source: &Source<'a>, function_pointer: &FunctionPointer<'a>) {
        let name = function_pointer.as_ident();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, function_pointer.original_name());
        let alias = function_pointer.as_alias();

        let args = function_pointer
            .arguments()
            .iter()
            .map(|arg| {
                let name = arg.as_ident();
                let ty = field_type(source, arg.ty(), &mut self.imports);

                quote! {
                    #name: #ty
                }
            })
            .collect::<Vec<_>>();

        let return_type = function_pointer
            .return_type()
            .map(|ty| field_type(source, ty, &mut self.imports))
            .map(|ty| quote! { -> #ty });

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #doc
            #alias
            pub type #name = unsafe extern "system" fn(#(#args),*) #return_type;
        };
    }

    fn visit_command_alias<'a>(&mut self, source: &Source<'a>, command_alias: &CommandAlias<'a>) {
        let of = source.resolve(command_alias.of()).expect("unknown alias").as_function();
        let of_fp = of.as_function_pointer();

        self.imports.push_origin(source, of.origin(), of_fp.name());

        let name = command_alias.as_fn_pointer_ident();
        let alias = command_alias.as_alias();
        let of_ident = of_fp.as_ident();
        let doc_str = format!("See [`{}`]", of.name());

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #[doc = #doc_str]
            #alias
            pub type #name = #of_ident;
        };

        self.vtable.push(VTableItem {
            name,
            ty: of_ident,
            original_name: command_alias.original_name().to_string(),
        });
    }

    fn visit_function<'a>(&mut self, source: &Source<'a>, function: &magritte_types::Function<'a>) {
        let fn_ptr = function.as_function_pointer();
        self.visit_function_pointer(source, &fn_ptr);

        self.vtable.push(VTableItem {
            name: function.as_ident(),
            ty: fn_ptr.as_ident(),
            original_name: function.original_name().to_string(),
        });
    }

    fn visit_command<'a>(&mut self, source: &Source<'a>, command: &magritte_types::Function<'a>) {
        self.visit_function(source, &*command);
    }

    fn finish<'a>(self, _source: &Source<'a>) {
        let mut out = String::with_capacity(1 << 20);

        if let Some(doc) = self.doc_of_origin(&self.doc_dir_path, &self.origin) {
            out.extend_one(doc.to_string());
        }

        if !self.vtable.is_empty() {
            // eprintln!("TODO: vtable");
        }

        out.extend_one(self.imports.to_token_stream().to_string());
        out.extend_one(self.out.to_string());

        let out = run_rustfmt(out).expect("failed to run rustfmt");
        std::fs::write(&self.mod_path, out).expect("Failed to write code");
    }
}

impl<'b, 'parent> Deref for NativeBackendOriginVisitor<'b, 'parent> {
    type Target = NativeVisitor<'b>;

    fn deref(&self) -> &Self::Target {
        &*self.parent
    }
}

impl<'b, 'parent> DerefMut for NativeBackendOriginVisitor<'b, 'parent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent
    }
}
