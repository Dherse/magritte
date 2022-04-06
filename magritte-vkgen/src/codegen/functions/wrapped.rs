use std::{borrow::Cow, ops::Deref};

use ahash::AHashSet;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use smallvec::SmallVec;

use crate::{
    expr::Expr,
    imports::Imports,
    source::{reference::TypeRef, ExternallySynced, Field, Function, FunctionArgument, Handle, Optionality, Source},
    symbols::SymbolTable,
    ty::{Mutability, Native, Ty},
};

#[derive(Debug, Clone)]
pub struct ExtraCallInfo {
    pub arguments: SmallVec<[usize; 1]>,
    pub length_index: usize,
    pub size_ty: Native,
    pub param_name: Ident,
}

#[derive(Default)]
pub struct StatefulFunctionGeneratorState {
    pub unsafe_: bool,

    pub lifetime: bool,

    pub this: bool,

    /// The argument definitions of the function
    /// Of the form:
    /// - `#name: #ty`
    pub function_args: Vec<TokenStream>,

    /// The call arguments of this function.
    /// This is a list of functions (one per argument) that
    /// takes in an optional *getter* path. This getter path is
    /// the path towards the state. If this path is empty, it will
    /// consider the path to be local (i.e the state is defined **in** the function).
    pub call_args: Vec<Box<dyn Fn(Option<TokenStream>) -> TokenStream>>,

    pub extra_call_info: Option<ExtraCallInfo>,

    /// Additional assertions.
    pub asserts: Vec<TokenStream>,

    /// Additional state creation, should **not** allocate!
    pub states: Vec<TokenStream>,

    /*/// The index of the next generic iterator parameter
    pub generic_index_iter: usize,

    /// The index of the next generic value parameter
    pub generic_index_value: usize,

    /// The generic type names of the function
    ///
    /// Generic arguments are **not** part of the state!
    /// They are self-contained within the function, **always**.
    pub generic_idents: Vec<Ident>,

    /// The generic bounds of each generic argument
    /// Of the form:
    /// - `#bound`
    ///
    /// The generated code should look like:
    /// - `#generic: #bound`
    ///
    /// Generic arguments are **not** part of the state!
    /// They are self-contained within the function, **always**.
    pub generic_bounds: Vec<TokenStream>,

    /// The name of the states
    pub state_idents: Vec<Ident>,

    /// The state definitions
    /// Of the form:
    /// - `#ty`
    ///
    /// The generated code should look like either:
    /// - `#name: #ty,`
    /// - `let #name: #ty = #init;`
    ///
    /// depending on wether the state is part of a function or
    /// part of a state structure.
    pub state_types: Vec<TokenStream>,

    /// The state initialization
    /// Of the form:
    /// - `#init`
    ///
    /// The generated code should look like either:
    /// - `#name: #init,`
    /// - `let mut #name: #ty = #init;`
    ///
    /// depending on wether the state is part of a function or
    /// part of a state structure.
    pub state_initializations: Vec<TokenStream>,

    pub state_reset: Vec<Box<dyn Fn(Option<TokenStream>) -> TokenStream>>,

    pub state_set: Vec<Box<dyn Fn(Option<TokenStream>) -> TokenStream>>,*/
    /// The return value initializers.
    pub return_initializations: Vec<TokenStream>,

    /// The return type, if any
    pub return_types: Vec<TokenStream>,

    /// The return value, if any
    pub return_values: Vec<TokenStream>,
}

impl StatefulFunctionGeneratorState {
    pub fn new<'a>(
        source: &Source<'a>,
        imports: &Imports,
        handle: &Handle<'a>,
        fun: Function<'a>,
        state_lifetime: Option<TokenStream>,
    ) -> Self {
        let mut this = Self::default();
        this.lifetime = true;

        let handle_ident = handle.as_ident();

        // We are collecting the argument kinds
        let kinds = ArgKind::from_slice(source, handle.original_name(), fun.arguments().clone());

        // We process the argument kinds in a second stage, this makes it easier to reuse the `kind` code
        // for the stateless process and makes it easier to understand the logic.
        for (i, kind) in kinds.into_iter().enumerate() {
            match kind {
                ArgKind::Regular(arg) | ArgKind::Optional(arg) => {
                    this.push_regular(source, imports, arg, &state_lifetime)
                },
                ArgKind::Array(arg) => this.push_array(source, imports, arg, &state_lifetime),
                ArgKind::This(arg) => this.push_this(arg, &handle_ident),
                ArgKind::Passthrough(arg) => this.push_passthrough(source, imports, arg),
                ArgKind::CStr(arg) => this.push_cstr(imports, arg, &state_lifetime),
                ArgKind::ValueWrittenTo(arg, inner) => this.push_value_written_to(source, imports, handle, arg, inner),
                ArgKind::ValueWrittenToChained(arg, inner) => {
                    this.push_value_written_to_chained(source, imports, arg, inner)
                },
                ArgKind::ArrayWrittenTo(arg, inner, len) => {
                    this.push_array_written_to(source, imports, handle, &fun, arg, inner, len)
                },
                ArgKind::LenByArray(arg, len_of) => this.push_len_by_array(source, imports, arg, len_of),
                ArgKind::LenExtraCall(arg, len_of) => this.push_len_by_extra_call(i, arg, len_of),
            }
        }

        if let Some(ret) = fun.return_type() {
            match ret {
                Ty::Named(name) if name == "VkResult" => {},
                Ty::Named(_) => {
                    this.return_types
                        .push(ret.as_ty(source, Some(imports)).0.to_token_stream());

                    this.return_values.push(
                        match ret.c_to_rust_converter(source, Mutability::Const, quote! { _return }, None, true) {
                            Some((v, _)) => v,
                            None => quote! { _return },
                        },
                    );
                },
                Ty::Native(Native::Void) => {},
                Ty::Native(native) => {
                    this.return_types.push(native.to_token_stream());
                    this.return_values.push(quote! { _return });
                },
                _ => unreachable!("unsupported return type `{:?}` for {}", ret, fun.original_name()),
            }
        }

        this
    }

    fn push_len_by_extra_call<'a>(
        &mut self,
        i: usize,
        arg: FunctionArgument<'a>,
        len_of: SmallVec<[(usize, FunctionArgument<'a>); 1]>,
    ) {
        assert!(self.extra_call_info.is_none());
        let name = arg.as_ident();

        self.function_args.push(quote! {
            #name: Option<usize>
        });

        self.extra_call_info = Some(ExtraCallInfo {
            arguments: len_of.into_iter().map(|(i, _)| i).collect(),
            length_index: i,
            param_name: name.clone(),
            size_ty: arg.ty().as_ptr().1.as_native(),
        });

        self.call_args.push(box move |_| {
            quote! {
                &mut #name
            }
        });
    }

    fn push_len_by_array<'a>(
        &mut self,
        source: &Source<'a>,
        imports: &Imports,
        arg: FunctionArgument<'a>,
        len_of: SmallVec<[(usize, FunctionArgument<'a>); 1]>,
    ) {
        let mut fields = len_of.into_iter().map(|(_, a)| a).filter(|arg| match arg.ty() {
            Ty::Slice(mut_, _, _) => *mut_ == Mutability::Const,
            ty => unreachable!("only a slice can have a dynamic length, of: {:?}", ty),
        });

        let first = fields.next().expect("found length with no dependents");
        let first_ident = first.as_ident();

        let ty = first.ty().clone();
        let (_, _, len) = ty.as_slice();

        let expr = len
            .pivot("len", &*len.variables().pop().expect("no variable in length"))
            .as_expr(source, &|_| quote! { len }, Some(imports));

        self.asserts.extend(fields.map(|arg| {
            let other = arg.as_ident();
            if matches!(first.optionality(), Optionality::No) {
                quote! {
                    debug_assert!((|len: usize| #expr)(#first_ident.len()) == #other.len());
                }
            } else {
                quote! {
                    debug_assert!((|len: usize| #expr)(#first_ident.map_or(0, |i| i.len())) == #other.len());
                }
            }
        }));

        let name = arg.as_ident();
        self.states.push(if matches!(first.optionality(), Optionality::No) {
            quote! {
                let #name = (|len: usize| #expr)(#first_ident.len()) as _;
            }
        } else {
            quote! {
                let #name = (|len: usize| #expr)(#first_ident.map_or(0, |i| i.len())) as _;
            }
        });

        self.call_args.push(box move |_| name.to_token_stream());
    }

    fn push_array_written_to<'a>(
        &mut self,
        source: &Source<'a>,
        imports: &Imports,
        handle: &Handle<'a>,
        fun: &Function<'a>,
        arg: FunctionArgument<'a>,
        inner: Ty<'a>,
        len: Expr<'a>,
    ) {
        imports.smallvec();

        let mut ty = inner.as_raw_ty(source, Some(imports), true).0.to_token_stream();
        let len = len.as_expr(
            source,
            &|name| {
                if let Some(arg) = fun.arguments().get_by_either(name) {
                    arg.as_ident().to_token_stream()
                } else {
                    fun.arguments()
                        .iter()
                        .map(FunctionArgument::ty)
                        .filter_map(|ty| match ty {
                            Ty::Named(name) | Ty::Pointer(_, box Ty::Named(name)) => Some(name),
                            _ => None,
                        })
                        .filter_map(|name| source.resolve_type(name))
                        .filter_map(TypeRef::as_struct)
                        .find_map(|struct_| struct_.fields().get_by_either(name))
                        .map(Field::as_ident)
                        .unwrap()
                        .to_token_stream()
                }
            },
            Some(imports),
        );

        let ret_ident = arg.as_ident();

        self.return_initializations.push(quote! {
            let mut #ret_ident = SmallVec::<#ty>::from_elem(Default::default(), #len as usize);
        });

        self.return_values.push(quote! {
            #ret_ident
        });

        if let Ty::Named(name) = inner {
            if let Some(out) = source.handles.get_by_either(&name) {
                let parent = if out.parent() != Some(handle.original_name()) {
                    /*if let Some(existing_parent) = fun.arguments().iter()
                        .find(|arg| match arg.ty() {
                            Ty::Named(named) if named == out.parent().unwrap() => true,
                            _ => false
                        })
                    {
                        existing_parent.as_ident().to_token_stream()
                    } else {
                        self.lifetime = false;

                        let hdl = source.handles.get_by_either(out.parent().unwrap()).unwrap().as_ident();
                        self.function_args.push(quote! {
                            parent: &'parent Unique<'this, #hdl>
                        });
                        quote! { parent }
                    }*/

                    self.lifetime = false;

                    let hdl = source.handles.get_by_either(out.parent().unwrap()).unwrap().as_ident();
                    self.function_args.push(quote! {
                        parent: &'parent Unique<'this, #hdl>
                    });
                    quote! { parent }
                } else {
                    quote! { self }
                };

                ty = if self.lifetime {
                    quote! {
                        Unique<'this, #ty>
                    }
                } else {
                    quote! {
                        Unique<'parent, #ty>
                    }
                };

                *self.return_values.last_mut().unwrap() = quote! {
                    #ret_ident.into_iter().map(|i| Unique::new(#parent, i, ())).collect()
                };
            }
        }

        self.return_types.push(quote! {
            SmallVec<#ty>
        });

        self.call_args.push(box move |_| {
            quote! { #ret_ident.as_mut_ptr() }
        });
    }

    fn push_value_written_to_chained<'a>(
        &mut self,
        source: &Source<'a>,
        imports: &Imports,
        arg: FunctionArgument<'a>,
        inner: Ty<'a>,
    ) {
        self.return_types
            .push(inner.as_raw_ty(source, Some(imports), true).0.to_token_stream());
        let (ty, _) = inner.as_raw_ty(source, Some(imports), true);

        let ret_ident = arg.as_ident();

        // This does some optimization by using [`std::mem::zeroed`] or [`MaybeUninit::uninit`] to
        // try and avoid using `memset`s as much as possible.
        match inner {
            Ty::Named(named) => match source.resolve_type(&named).expect("unknown type") {
                TypeRef::Struct(s) => {
                    imports.push("std::mem::MaybeUninit");

                    let null = if s.has_p_next() == Some(Mutability::Mutable) {
                        quote! { null_mut }
                    } else {
                        quote! { null }
                    };

                    self.function_args.push(quote! {
                        #ret_ident: Option<#ty>
                    });

                    self.return_values.push(quote! {
                        {
                            #ret_ident.p_next = std::ptr::#null ();
                            #ret_ident
                        }
                    });

                    self.return_initializations.push(quote! {
                        let mut #ret_ident = #ret_ident.unwrap_or_else(|| MaybeUninit::<#ty>::zeroed().assume_init());
                    });

                    self.call_args.push(box move |_| {
                        quote! { &mut #ret_ident }
                    });
                },
                _ => unreachable!("Only structs can have a `p_next` field"),
            },
            _ => unreachable!("Only structs can have a `p_next` field"),
        }
    }

    fn push_value_written_to<'a>(
        &mut self,
        source: &Source<'a>,
        imports: &Imports,
        handle: &Handle<'a>,
        arg: FunctionArgument<'a>,
        inner: Ty<'a>,
    ) {
        self.return_types
            .push(inner.as_raw_ty(source, Some(imports), true).0.to_token_stream());
        let (ty, _) = inner.as_raw_ty(source, Some(imports), true);

        let ret_ident = arg.as_ident();

        // This does some optimization by using [`std::mem::zeroed`] or [`MaybeUninit::uninit`] to
        // try and avoid using `memset`s as much as possible.
        match inner {
            Ty::Named(named) => match source.resolve_type(&named).expect("unknown type") {
                TypeRef::Struct(s) => {
                    imports.push("std::mem::MaybeUninit");
                    self.return_values.push(quote! { #ret_ident.assume_init() });

                    if s.has_pointer() {
                        // If we have pointers, we initialize the return value as a set of zero.
                        // This avoids Vulkan following pointers to `p_next` chains and causing
                        // segfaults.
                        self.return_initializations.push(quote! {
                            let mut #ret_ident = MaybeUninit::<#ty>::zeroed();
                        });
                    } else {
                        // If we don't contain pointer, we can just use random uninitialized data.
                        self.return_initializations.push(quote! {
                            let mut #ret_ident = MaybeUninit::<#ty>::uninit();
                        });
                    }

                    self.call_args.push(box move |_| {
                        quote! { #ret_ident.as_mut_ptr() }
                    });
                },
                TypeRef::Handle(out) => {
                    imports.push("std::mem::MaybeUninit");

                    let parent = if out.parent() != Some(handle.original_name()) {
                        self.lifetime = false;

                        let hdl = source.handles.get_by_either(out.parent().unwrap()).unwrap().as_ident();
                        self.function_args.push(quote! {
                            parent: &'parent Unique<'this, #hdl>
                        });
                        quote! { parent }
                    } else {
                        quote! { self }
                    };

                    // set the output value to be a boolean
                    let last = self.return_types.pop().unwrap();
                    self.return_types.push(if self.lifetime {
                        quote! {
                            Unique<'this, #last>
                        }
                    } else {
                        quote! {
                            Unique<'parent, #last>
                        }
                    });

                    // For handle, we just contain a `u64` so we can just use uninitialized value, it
                    // doesn't matter.
                    self.return_initializations.push(quote! {
                        let mut #ret_ident = MaybeUninit::<#ty>::uninit();
                    });

                    self.return_values
                        .push(quote! { Unique::new(#parent, #ret_ident.assume_init(), ()) });

                    self.call_args.push(box move |_| quote! { #ret_ident.as_mut_ptr() });
                },
                // Special case of boolean values
                TypeRef::Basetype(b) if b.original_name() == "VkBool32" => {
                    self.return_initializations.push(quote! {
                        let mut #ret_ident: u32 = 0;
                    });

                    // set the output value to be a boolean
                    *self.return_types.last_mut().unwrap() = quote! {
                        bool
                    };

                    self.return_values.push(quote! { #ret_ident != 0 });

                    self.call_args.push(box move |_| {
                        quote! {  &mut #ret_ident }
                    });
                },
                TypeRef::Union(_) | TypeRef::Basetype(_) | TypeRef::OpaqueType(_) | TypeRef::FunctionPointer(_) => {
                    self.return_initializations.push(quote! {
                        let mut #ret_ident = std::mem::zeroed();
                    });

                    self.return_values.push(quote! { #ret_ident });

                    self.call_args.push(box move |_| {
                        quote! { &mut #ret_ident }
                    });
                },
                TypeRef::Enum(_) | TypeRef::Bitmask(_) | TypeRef::BitFlag(_) => {
                    self.return_initializations.push(quote! {
                        let mut #ret_ident = #ty::empty();
                    });

                    self.return_values.push(quote! { #ret_ident });

                    self.call_args.push(box move |_| {
                        quote! { &mut #ret_ident }
                    });
                },
                _ => unreachable!("resolve type should not be an alias"),
            },
            Ty::Pointer(_, _) => {
                // for pointer we use a null pointer
                self.return_initializations.push(quote! {
                    let mut #ret_ident = std::ptr::null_mut();
                });

                self.return_values.push(quote! { #ret_ident });

                self.call_args.push(box move |_| {
                    quote! { &mut #ret_ident }
                });
            },
            _ => {
                // if we are any other type, we just use the default
                self.return_initializations.push(quote! {
                    let mut #ret_ident = Default::default();
                });

                self.return_values.push(quote! { #ret_ident });

                self.call_args.push(box move |_| {
                    quote! { &mut #ret_ident }
                });
            },
        }
    }

    fn push_cstr<'a>(&mut self, imports: &Imports, arg: FunctionArgument<'a>, state_lifetime: &Option<TokenStream>) {
        imports.push("std::ffi::CStr");

        let name = arg.as_ident();

        if matches!(arg.optionality(), Optionality::Yes) {
            self.function_args.push(quote! { #name: Option<&#state_lifetime CStr> });
            self.call_args
                .push(box move |_| quote! { #name.map(CStr::as_ptr).unwrap_or_else(std::ptr::null) });
        } else {
            self.function_args.push(quote! { #name: &#state_lifetime CStr });
            self.call_args.push(box move |_| quote! { #name.as_ptr() });
        }
    }

    fn push_passthrough<'a>(&mut self, source: &Source<'a>, imports: &Imports, arg: FunctionArgument<'a>) {
        let name = arg.as_ident();
        let (ty, _) = arg.ty().as_raw_ty(source, Some(imports), true);

        if matches!(arg.optionality(), Optionality::Yes) {
            self.function_args.push(quote! { #name: Option<#ty> });

            match arg.ty().is_void_ptr_mut() {
                Some(Mutability::Const) => self
                    .call_args
                    .push(box move |_| quote! { #name.unwrap_or_else(std::ptr::null) }),
                Some(Mutability::Mutable) => self
                    .call_args
                    .push(box move |_| quote! { #name.unwrap_or_else(std::ptr::null_mut) }),
                None => self.call_args.push(box move |_| quote! { #name.unwrap_or_default() }),
            }
        } else {
            self.function_args.push(quote! { #name: #ty });
            self.call_args.push(box move |_| quote! { #name });
        }
    }

    fn push_this<'a>(&mut self, arg: FunctionArgument<'a>, handle_ident: &Ident) {
        self.this = true;
        if matches!(arg.externally_synced(), ExternallySynced::Yes) {
            self.function_args
                .push(quote! { self: &'this mut Unique<'a, #handle_ident> });
            self.call_args.push(box move |_| quote! { self.as_raw() });
        } else {
            self.function_args
                .push(quote! { self: &'this Unique<'a, #handle_ident>  });
            self.call_args.push(box move |_| quote! { self.as_raw() });
        }
    }

    fn push_array<'a>(
        &mut self,
        source: &Source<'a>,
        imports: &Imports,
        arg: FunctionArgument<'a>,
        state_lifetime: &Option<TokenStream>,
    ) {
        let name = arg.as_ident();

        match arg.ty() {
            Ty::Array(inner, len) => {
                let len = len.as_expr(
                    source,
                    &|name| Ident::new(name, Span::call_site()).to_token_stream(),
                    Some(imports),
                );

                match &**inner {
                    Ty::Native(native) => {
                        let ty = native.to_token_stream();

                        if matches!(arg.optionality(), Optionality::Sometimes | Optionality::Yes) {
                            self.function_args.push(quote! { #name: Option<[#ty; #len]> });
                            self.call_args.push(box move |_| quote! { #name.unwrap_or_default() });
                        } else {
                            self.function_args.push(quote! { #name: [#ty; #len] });
                            self.call_args.push(box move |_| quote! { #name });
                        }
                    },
                    Ty::Named(named) => {
                        let ty = source.resolve_type(named).expect("missing type");
                        ty.import(imports);

                        let ty = ty.as_ident();
                        if matches!(arg.optionality(), Optionality::Sometimes | Optionality::Yes) {
                            self.function_args.push(quote! { #name: Option<[#ty; #len]> });
                            self.call_args.push(box move |_| quote! { #name.unwrap_or_default() });
                        } else {
                            self.function_args.push(quote! { #name: [#ty; #len] });
                            self.call_args.push(box move |_| quote! { #name });
                        }
                    },
                    other => unimplemented!("only native and named types are supported in arrays, for: {:?}", other),
                }
            },
            Ty::Slice(Mutability::Const, inner, _) => {
                imports.push("crate::SmallVec");

                let (ty, _) = inner.as_raw_ty(source, None, false);
                if matches!(arg.optionality(), Optionality::Sometimes | Optionality::Yes) {
                    self.function_args
                        .push(quote! { #name: Option<&#state_lifetime [#ty]> });
                    self.call_args
                        .push(box move |_| quote! { #name.map(|slice| slice.as_ptr()).unwrap_or_else(std::ptr::null) });
                } else {
                    self.function_args.push(quote! { #name: &#state_lifetime [#ty] });
                    self.call_args.push(box move |_| quote! { #name.as_ptr() });
                }
                /*
                match &**inner {
                    Ty::Native(native) => {
                        let ty = native.to_token_stream();

                        if matches!(arg.optionality(), Optionality::Sometimes | Optionality::Yes) {
                            self.function_args
                                .push(quote! { #name: Option<&#state_lifetime [#ty]> });
                            self.call_args.push(box move |_| quote! { #name.map(|slice| slice.as_ptr()).unwrap_or_else(std::ptr::null) });
                        } else {
                            self.function_args.push(quote! { #name: &#state_lifetime [#ty] });
                            self.call_args.push(box move |_| quote! { #name.as_ptr() });
                        }
                    },
                    Ty::Pointer(Mutability::Const, box Ty::Named(named)) => {
                        // TODO: yeet this
                        match source.resolve_type(named).expect("missing type") {
                            ty @ TypeRef::Struct(_) => {
                                ty.import(imports);
                                let ty = ty.as_ident();

                                if matches!(arg.optionality(), Optionality::Sometimes | Optionality::Yes) {
                                    self.function_args
                                        .push(quote! { #name: Option<&#state_lifetime [&#state_lifetime #ty]> });
                                    self.call_args.push(box move |_| quote! { #name.map(|slice| slice.as_ptr()).unwrap_or_else(std::ptr::null) });
                                } else {
                                    self.function_args
                                        .push(quote! { #name: &#state_lifetime [&#state_lifetime #ty] });
                                    self.call_args.push(box move |_| quote! { #name.as_ptr() });
                                }
                            },
                            _ => unimplemented!("Slices of pointers is only supported for structs"),
                        }
                    },
                    Ty::Pointer(Mutability::Const, box Ty::Native(Native::UInt(4))) => {
                        self.unsafe_ = true;

                        if matches!(arg.optionality(), Optionality::Sometimes | Optionality::Yes) {
                            self.function_args
                                .push(quote! { #name: Option<&#state_lifetime [*const u32]> });
                            self.call_args.push(box move |_| quote! { #name.map(|slice| slice.as_ptr()).unwrap_or_else(std::ptr::null) });
                        } else {
                            self.function_args.push(quote! { #name: &#state_lifetime [*const u32] });
                            self.call_args.push(box move |_| quote! { #name.as_ptr() });
                        }
                    },
                    Ty::Named(named) => match source.resolve_type(named).expect("missing type") {
                        ty @ TypeRef::Handle(_) => {
                            ty.import(imports);
                            let ty = ty.as_ident();

                            if matches!(arg.optionality(), Optionality::Sometimes | Optionality::Yes) {
                                self.function_args.push(quote! { #name: Option<&[#ty]> });

                                let name_clone = name.clone();
                                self.call_args.push(box move |state| {
                                    quote! {
                                        #name_clone.map_or_else(|| std::ptr::null(), |v| v.as_ptr())
                                    }
                                });
                            } else {
                                self.function_args.push(quote! { #name: #ty });

                                let name_clone = name.clone();
                                self.call_args.push(box move |state| {
                                    quote! {
                                        #name_clone.as_ptr()
                                    }
                                });
                            }
                        },
                        ty => {
                            ty.import(imports);

                            let ty = ty.as_ident();
                            if matches!(arg.optionality(), Optionality::Sometimes | Optionality::Yes) {
                            } else {
                                self.function_args.push(quote! { #name: [#ty; #len] });
                                self.call_args.push(box move |_| quote! { #name });
                            }
                        },
                    },
                    other => unimplemented!("only native and named types are supported in arrays, for: {:?}", other),
                } */
            },
            other => unimplemented!("arrays should only be slices or fixed-sized arrays, for: {:?}", other),
        }
    }

    fn push_regular<'a>(
        &mut self,
        source: &Source<'a>,
        imports: &Imports,
        arg: FunctionArgument<'a>,
        state_lifetime: &Option<TokenStream>,
    ) {
        let name = arg.as_ident();
        match arg.ty() {
            Ty::Pointer(mut_, ty) => {
                let null = mut_.to_null_tokens();
                let mut_ptr = mut_.to_pointer_tokens();
                let mut_ = mut_.to_ref_token();
                let (ty, _) = ty.as_raw_ty(source, Some(imports), true);

                if matches!(arg.optionality(), Optionality::Yes) {
                    self.function_args
                        .push(quote! { #name: Option<&#state_lifetime #mut_ #ty> });
                    self.call_args.push(
                        box move |_| quote! { #name.map(|v| v as *#mut_ptr #ty).unwrap_or_else(std::ptr::#null) },
                    );
                } else {
                    self.function_args.push(quote! { #name: &#state_lifetime #mut_ #ty });
                    self.call_args.push(box move |_| quote! { #name as *#mut_ptr #ty });
                }
            },
            Ty::Native(nat) => {
                // TODO: can improve this by transforming byte length into `Size`.
                if matches!(arg.optionality(), Optionality::Yes) {
                    self.function_args.push(quote! { #name: #nat });
                    self.call_args.push(box move |_| quote! { #name as _});
                } else {
                    self.function_args.push(quote! { #name: Option<#nat> });
                    self.call_args
                        .push(box move |_| quote! { #name.unwrap_or_default() as _});
                }
            },
            Ty::Named(named) => {
                let type_ = source.resolve_type(named).expect("unknown type");
                type_.import(imports);

                let ident = type_.as_ident();

                match type_ {
                    TypeRef::Handle(_) => {
                        if matches!(arg.optionality(), Optionality::Yes) {
                            // The argument
                            self.function_args.push(quote! { #name: Option<#ident> });

                            // The call to `as_raw`
                            self.call_args.push(box move |_| quote! { #name.unwrap_or_default() });
                        } else {
                            // The argument
                            self.function_args.push(quote! { #name: #ident });

                            // The call to `as_raw`
                            self.call_args.push(box move |_| quote! { #name });
                        }
                    },
                    TypeRef::Struct(struct_) => {
                        let (rustified, _) = arg.ty().as_ty(source, Some(imports));
                        self.function_args.push(quote! { #name: #rustified });

                        let rust_to_c = if let Some((_, rust_to_c)) = arg.ty().rust_to_c_converter(
                            source,
                            &|field, ident| {
                                let field = struct_.get_field(field).unwrap().as_ident();
                                quote! {
                                    self.#field = #ident;
                                }
                            },
                            &|field| struct_.get_field(field).unwrap().ty().clone(),
                            arg.name(),
                            None,
                        ) {
                            rust_to_c
                        } else {
                            arg.as_ident().to_token_stream()
                        };

                        self.call_args.push(box move |_| quote! { #rust_to_c });
                    },
                    _ => {
                        let (rustified, _) = arg.ty().as_ty(source, Some(imports));
                        self.function_args.push(quote! { #name: #rustified });

                        let rust_to_c = arg.ty().rust_to_c_converter_inline(arg.name());

                        self.call_args.push(box move |_| quote! { #rust_to_c });
                    },
                }
            },
            Ty::StringArray(_) => unimplemented!("not needed yet"),
            Ty::NullTerminatedString(Mutability::Const) => {
                imports.push("std::ffi::CStr");

                if matches!(arg.optionality(), Optionality::Yes) {
                    self.function_args.push(quote! { #name: Option<&#state_lifetime CStr> });
                    self.call_args
                        .push(box move |_| quote! { #name.map(CStr::as_ptr).unwrap_or(std::ptr::null) });
                } else {
                    self.function_args.push(quote! { #name: &#state_lifetime CStr });
                    self.call_args.push(box move |_| quote! { #name.as_ptr() });
                }
            },
            Ty::NullTerminatedString(Mutability::Mutable) => todo!("unsupported mutable CStr"),
            Ty::Array(inner, len) => {
                let len = len.as_expr(
                    source,
                    &|name| Ident::new(name, Span::call_site()).to_token_stream(),
                    None,
                );

                match &**inner {
                    Ty::Native(native) => {
                        if matches!(arg.optionality(), Optionality::Yes) {
                            self.function_args.push(quote! { #name: Option<[#native; #len]> });
                            self.call_args.push(box move |_| quote! { #name.unwrap_or_default() });
                        } else {
                            self.function_args.push(quote! { #name: [#native; #len] });
                            self.call_args.push(box move |_| quote! { #name });
                        }
                    },
                    Ty::Named(named) => {
                        let type_ = source.resolve_type(named).expect("unknown type");
                        type_.import(imports);

                        let ident = type_.as_ident();

                        if matches!(arg.optionality(), Optionality::Yes) {
                            self.function_args.push(quote! { #name: Option<[#ident; #len]> });
                            self.call_args.push(box move |_| quote! { #name.unwrap_or_default() });
                        } else {
                            self.function_args.push(quote! { #name: [#ident; #len] });
                            self.call_args.push(box move |_| quote! { #name });
                        }
                    },
                    _ => unimplemented!("inner types of arrays must be named or native"),
                }
            },
            other => unimplemented!("regular arguments cannot contain {:?}", other),
        };
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ArgKind<'a> {
    /// A regular value that may need transformation.
    Regular(FunctionArgument<'a>),

    /// An array of values that may need transformation.
    Array(FunctionArgument<'a>),

    /// An optional value that may need transformation.
    Optional(FunctionArgument<'a>),

    /// A value that **does not** need transformation.
    Passthrough(FunctionArgument<'a>),

    /// A handle to the current value.
    This(FunctionArgument<'a>),

    /// An immutable `std::ffi::CStr`.
    CStr(FunctionArgument<'a>),

    /// A value we will write to.
    ValueWrittenTo(FunctionArgument<'a>, Ty<'a>),

    /// A value we will write to that has a pointer chain.
    ValueWrittenToChained(FunctionArgument<'a>, Ty<'a>),

    /// An array of values we will write to that may need transformation.
    ArrayWrittenTo(FunctionArgument<'a>, Ty<'a>, Expr<'a>),

    /// A slice whose length is determined from the array itself
    LenByArray(FunctionArgument<'a>, SmallVec<[(usize, FunctionArgument<'a>); 1]>),

    /// We obtain the length of the output value by doing an extra call
    /// to the underlying function.
    ///
    /// This is used when Vulkan has to return an array/vec of values.
    ///
    /// The first value is the slice, the second is the length
    ///
    /// This is characterized by the following argument pair:
    ///  - `*mut Value`: the array of values to be output
    ///  - `*mut usize`: the number of values to be output as a mutable pointer
    LenExtraCall(FunctionArgument<'a>, SmallVec<[(usize, FunctionArgument<'a>); 1]>),
}

impl<'a> Deref for ArgKind<'a> {
    type Target = FunctionArgument<'a>;

    fn deref(&self) -> &Self::Target {
        self.arg()
    }
}

impl<'a> ArgKind<'a> {
    pub fn from_slice<'b>(source: &Source<'b>, handle: &str, args: SymbolTable<'b, FunctionArgument<'b>>) -> Vec<Self>
    where
        'b: 'a,
    {
        let mut out = Vec::new();

        let mut lengths = AHashSet::new();
        for (i, arg) in args.iter().enumerate() {
            // We check if we are a handle
            if i == 0 && arg.ty().eq(&Ty::Named(Cow::Borrowed(handle))) {
                out.push(ArgKind::This(arg.clone()));
                continue;
            }

            // We check if we are a void pointer
            if arg.ty().is_void_ptr() {
                out.push(ArgKind::Passthrough(arg.clone()));
                continue;
            }

            // We check if we are a null-terminated string
            if arg.ty().is_cstr() {
                out.push(ArgKind::CStr(arg.clone()));
                continue;
            }

            // Collect all of the arguments we have the length of
            let length_of = args
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_, a)| a.len().map_or(false, |a| a == arg.original_name()))
                .collect::<SmallVec<[_; 1]>>();

            // If we are the length of a void pointer argument, the length becomes a passthrough
            if !length_of.is_empty() && length_of.iter().any(|(_, a)| a.ty().is_void_ptr()) {
                out.push(ArgKind::Passthrough(arg.clone()));
                continue;
            }

            if !length_of.is_empty() {
                // TODO: Ty::Slice was mutable, check if both mutable and const are valid!
                let has_slice = length_of.iter().find(|(_, arg)| matches!(arg.ty(), Ty::Slice(_, _, _)));
                match (arg.optionality(), has_slice) {
                    (Optionality::Sometimes, Some((_, _))) => {
                        out.push(ArgKind::LenExtraCall(arg.clone(), length_of));
                        continue;
                    },
                    _ => {
                        out.push(ArgKind::LenByArray(arg.clone(), length_of));
                        continue;
                    },
                }
            }

            match (arg.optionality(), arg.ty(), arg.ty().length()) {
                (Optionality::No, Ty::Pointer(Mutability::Mutable, ty), None) => match &**ty {
                    Ty::Named(name) => match source.resolve_type(name).expect("unknown type") {
                        TypeRef::Struct(struct_) if struct_.has_p_next() == Some(Mutability::Mutable) => {
                            out.push(ArgKind::ValueWrittenToChained(arg.clone(), (&**ty).clone()));
                            continue;
                        },
                        _ => {
                            out.push(ArgKind::ValueWrittenTo(arg.clone(), (&**ty).clone()));
                            continue;
                        },
                    },
                    _ => {
                        out.push(ArgKind::ValueWrittenTo(arg.clone(), (&**ty).clone()));
                        continue;
                    },
                },
                (Optionality::No, Ty::Slice(Mutability::Mutable, ty, _), Some(len))
                | (Optionality::Yes, Ty::Slice(Mutability::Mutable, ty, _), Some(len)) => {
                    if let Some(var) = len.variables().pop() {
                        lengths.insert(var);
                    }

                    out.push(ArgKind::ArrayWrittenTo(arg.clone(), (&**ty).clone(), len.clone()));
                    continue;
                },
                (Optionality::Yes, _, None) => {
                    out.push(ArgKind::Optional(arg.clone()));
                    continue;
                },
                (_, _, Some(_)) => {
                    out.push(ArgKind::Array(arg.clone()));
                    continue;
                },
                (_, _, None) => {
                    out.push(ArgKind::Regular(arg.clone()));
                    continue;
                },
            }
        }

        // remove all duplicate arguments.
        out.dedup();

        out
    }

    /// Gets the underlying argument
    pub fn arg(&self) -> &FunctionArgument<'a> {
        match self {
            ArgKind::Regular(arg)
            | ArgKind::Array(arg)
            | ArgKind::Optional(arg)
            | ArgKind::Passthrough(arg)
            | ArgKind::This(arg)
            | ArgKind::CStr(arg)
            | ArgKind::ValueWrittenTo(arg, _)
            | ArgKind::ValueWrittenToChained(arg, _)
            | ArgKind::ArrayWrittenTo(arg, _, _)
            | ArgKind::LenByArray(arg, _)
            | ArgKind::LenExtraCall(arg, _) => arg,
        }
    }
}
