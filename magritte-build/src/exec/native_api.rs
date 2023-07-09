use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use magritte_build::{imports::Imports, origin::cond_of, rustfmt::run_rustfmt, OriginVisitor, Visitor};
use magritte_types::{Function, Handle, Origin, Source, TypeRef};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::{
    edge_case::EdgeCase,
    hl::functions::{ArgKind, ArgumentDefinitions, Arguments, ReturnType},
};

pub struct NativeApiVisitor<'b> {
    pub edge_cases: &'b Vec<Box<dyn EdgeCase + Send + Sync + 'static>>,
    pub out: PathBuf,
    pub imports: Imports,
    pub handles: Vec<HandleDef>,
    pub functions: Vec<FunctionDef>,
    pub tt: TokenStream,
}

impl<'b> NativeApiVisitor<'b> {
    pub fn write(mut self) {
        self.imports.push("crate::context::ObjectId");
        self.imports.push("crate::context::Container");
        self.imports.push("crate::Entry");
        self.imports.push("dashmap::DashMap");

        let handle_conds = self.handles.iter().map(|handle| &handle.cond).collect::<Vec<_>>();
        let handle_idents = self.handles.iter().map(|handle| &handle.ident).collect::<Vec<_>>();
        let handle_types = self.handles.iter().map(|handle| &handle.ty).collect::<Vec<_>>();

        let tt = &self.tt;
        let imports = &self.imports;
        let out = quote! {
            #imports

            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
            pub enum Object {
                #(
                    #handle_conds
                    #handle_types,
                )*
            }

            pub struct Api {
                #[doc = "The loading Vulkan entry, required to create Vulkan instances."]
                entry: Entry,
                objects: DashMap<ObjectId, Object>,

                #(
                    #handle_conds
                    #handle_idents: DashMap<ObjectId, Container<#handle_types>>,
                )*
            }

            impl std::fmt::Debug for Api {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    let mut s = f.debug_struct("Api");
                    #(
                        #handle_conds
                        {
                            s.field(stringify!(#handle_idents), &self.#handle_idents.len());
                        }
                    )*

                    s.finish_non_exhaustive()
                }
            }

            impl Api {
                pub fn new(entry: Entry) -> Self {
                    Self {
                        entry,
                        objects: DashMap::new(),

                        #(
                            #handle_conds
                            #handle_idents: DashMap::new(),
                        )*
                    }
                }

                pub fn objects(&self) -> &DashMap<ObjectId, Object> {
                    &self.objects
                }

                pub fn reference_counter_of(&self, object_id: ObjectId) -> Option<usize> {
                    self.objects.get(&object_id).map(|object| match object.value() {
                        #(
                            #handle_conds
                            Object::#handle_types => unsafe {
                                self.#handle_idents ().get(&object_id).unwrap_unchecked().ref_count()
                            },
                        )*
                    })
                }

                pub fn parent_of(&self, object_id: ObjectId) -> Option<ObjectId> {
                    self.objects.get(&object_id).and_then(|object| match object.value() {
                        #(
                            #handle_conds
                            Object::#handle_types => unsafe {
                                self.#handle_idents ().get(&object_id).unwrap_unchecked().parent()
                            },
                        )*
                    })
                }

                #(
                    #handle_conds
                    pub fn #handle_idents(&self) -> &DashMap<ObjectId, Container<#handle_types>> {
                        &self.#handle_idents
                    }
                )*
            }

            #tt
        };

        let out = run_rustfmt(out.to_string()).expect("failed to run rustfmt");
        std::fs::write(self.out.join("api.rs"), out).expect("Failed to write code");
    }
}

impl<'b> Visitor for NativeApiVisitor<'b> {
    type OriginVisitor<'parent> = NativeApiOriginVisitor<'b, 'parent>
    where
        Self: 'parent;

    type VersionVisitor<'parent> = ()
    where
        Self: 'parent;

    type ExtensionVisitor<'parent> = ()
    where
        Self: 'parent;

    fn visit_origin<'a>(&mut self, _source: &Source<'a>, _origin: &Origin<'a>) -> Option<Self::OriginVisitor<'_>> {
        Some(NativeApiOriginVisitor { parent: self })
    }
}

pub struct NativeApiOriginVisitor<'b, 'parent> {
    parent: &'parent mut NativeApiVisitor<'b>,
}

impl<'b, 'parent> Deref for NativeApiOriginVisitor<'b, 'parent> {
    type Target = NativeApiVisitor<'b>;

    fn deref(&self) -> &Self::Target {
        &*self.parent
    }
}

impl<'b, 'parent> DerefMut for NativeApiOriginVisitor<'b, 'parent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent
    }
}

impl<'b, 'parent> OriginVisitor<'parent> for NativeApiOriginVisitor<'b, 'parent> {
    fn visit_type<'a, 'c>(&mut self, source: &Source<'a>, ty: TypeRef<'a, 'c>) {
        return;

        match ty {
            // Aliases are just type aliases, we need to generate a type alias for them.
            TypeRef::Alias(_) => todo!(),

            // Opaque types do not need conversion
            TypeRef::OpaqueType(_) => todo!(),

            // We need to recursively visit the struct and generate an owned raw struct for it.
            // And then borrow the owned raw struct into the raw struct.
            TypeRef::Struct(_) => todo!(),

            // We need to turn the union (which is an enum) into either a raw union
            // or a union and an enum variant. (edge cases)
            TypeRef::Union(_) => todo!(),

            // Handles: we just need to get the `ObjectId` and get the handle from the map.
            TypeRef::Handle(_) => todo!(),

            // Function pointers require several edge cases for handling them using
            // rust closures and using the `pUserData` argument to pass them.
            TypeRef::FunctionPointer(_) => todo!(),

            // Base types are just a simple type aliases, except for the `VkBool32` type.
            TypeRef::Basetype(ty) if ty.original_name() == "VkBool32" => todo!(),

            // Base types are just a simple type aliases.
            TypeRef::Basetype(_) => todo!(),

            // Enums only requires a `.bits()` call to convert to a `i32` or `i64`.
            TypeRef::Bitmask(_) => todo!(),

            // Enums only requires a `.bits()` call to convert to a `i32` or `i64`.
            TypeRef::Bitflag(_) => todo!(),

            // Enums only requires a `.bits()` call to convert to a `i32`.
            TypeRef::Enum(_) => todo!(),
        }
    }

    fn visit_handle<'a>(&mut self, source: &Source<'a>, handle: &Handle<'a>) {
        self.imports.import(source, TypeRef::Handle(handle));

        self.handles.push(HandleDef {
            ident: handle.as_field_ident(),
            drop_ident: handle.as_drop_fn_ident(),
            clone_ident: handle.as_clone_fn_ident(),
            ty: handle.as_ident().to_token_stream(),
            cond: cond_of(source, &Origin::Vulkan1_0, handle.origin()),
        })
    }

    fn visit_function<'a>(&mut self, source: &Source<'a>, function: &Function<'a>) {
        if !self.edge_cases.function_filter(source, function) {
            return;
        }

        let name = function.name().to_string();
        let ident = function.as_ident();
        let cond = cond_of(source, &Origin::Vulkan1_0, function.origin());
        if let Some(handle) = source
            .handles()
            .iter()
            .find(|handle| handle.functions().contains_name(function.original_name()))
        {
            let mut arguments = Arguments::new(self.edge_cases, source, handle, function, &mut self.imports);

            let return_type = arguments.as_return_type();
            let arg_def = arguments.as_function_arguments();
            let args = arguments.args().iter().map(|arg| arg.as_static()).collect::<Vec<_>>();

            let aliases = function
                .aliases()
                .iter()
                .filter_map(|alias| source.command_aliases().get_by_name(alias))
                .map(|alias| FunctionDefAlias {
                    ident: alias.as_ident(),
                    cond: cond_of(source, &Origin::Vulkan1_0, alias.origin()),
                })
                .collect();

            self.functions.push(FunctionDef {
                name,
                ident,
                aliases,
                args,
                return_type,
                arg_def,
                cond,
            });
        }
    }

    fn visit_command<'a>(&mut self, source: &Source<'a>, command: &Function<'a>) {
        self.visit_function(source, command);
    }

    fn finish<'a>(self, _source: &Source<'a>) {}
}

#[derive(Debug, Clone)]
pub struct HandleDef {
    ident: Ident,
    drop_ident: Ident,
    clone_ident: Ident,
    ty: TokenStream,
    cond: Option<TokenStream>,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    name: String,
    ident: Ident,
    aliases: Vec<FunctionDefAlias>,
    args: Vec<ArgKind<'static>>,
    return_type: ReturnType,
    arg_def: ArgumentDefinitions,
    cond: Option<TokenStream>,
}

#[derive(Debug, Clone)]
pub struct FunctionDefAlias {
    ident: Ident,
    cond: Option<TokenStream>,
}
