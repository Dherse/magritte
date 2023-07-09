use std::{borrow::Cow, collections::HashSet, ops::Deref};

use heck::ToUpperCamelCase;
use magritte_build::imports::Imports;
use magritte_types::{
    Expr, Function, FunctionArgument, Handle, Mutability, Native, Optionality, Source, SymbolTable, Ty, TypeRef,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::{
    edge_case::EdgeCase,
    native::{field::field_type, r#const::constant_value},
};

pub struct Arguments<'a> {
    args: Vec<ArgKind<'a>>,
    edge_cases: &'a Vec<Box<dyn EdgeCase + Send + Sync>>,
    source: &'a Source<'a>,
    handle: &'a Handle<'a>,
    function: &'a Function<'a>,
    imports: &'a mut Imports,
}

impl<'a> Arguments<'a> {
    pub fn new(
        edge_cases: &'a Vec<Box<dyn EdgeCase + Send + Sync>>,
        source: &'a Source<'a>,
        handle: &'a Handle<'a>,
        function: &'a Function<'a>,
        imports: &'a mut Imports,
    ) -> Self {
        Self {
            args: ArgKind::from_args(source, handle.original_name(), function.arguments()),
            edge_cases,
            source,
            handle,
            function,
            imports,
        }
    }

    pub fn as_return_type(&mut self) -> ReturnType {
        let result = self.function.return_type() == Some(&Ty::Named(Cow::Borrowed("VkResult")));

        let mut items = Vec::new();
        for kind in &self.args {
            match kind {
                ArgKind::ValueWrittenTo(_, inner) | ArgKind::ValueWrittenToChained(_, inner) => match inner {
                    Ty::Named(name) => {
                        let ty = self.source.resolve_type(name).expect("unknown type");
                        let ident = ty.as_ident();
                        self.imports.import(self.source, ty);

                        if matches!(ty, TypeRef::Basetype(b) if b.of() == &Ty::Named(Cow::Borrowed("VkBool32"))) {
                            items.push(quote! { bool });
                        } else {
                            items.push(ident.to_token_stream());
                        }
                    },
                    Ty::Pointer(_, inner) => {
                        let ty = self.source.resolve_type(inner.as_named()).expect("unknown type");
                        let ident = ty.as_ident();
                        self.imports.import(self.source, ty);

                        if matches!(ty, TypeRef::Basetype(b) if b.of() == &Ty::Named(Cow::Borrowed("VkBool32"))) {
                            items.push(quote! { bool });
                        } else {
                            items.push(ident.to_token_stream());
                        }
                    },
                    Ty::Native(native) => {
                        items.push(native.to_token_stream());
                    },
                    other => unreachable!(
                        "ValueWrittenTo should only be used for named, pointer, native types: {:#?}",
                        other
                    ),
                },
                ArgKind::ArrayWrittenTo(_, inner, _) => match inner {
                    Ty::Named(name) => {
                        let ty = self.source.resolve_type(name).expect("unknown type");
                        let ident = ty.as_ident();
                        self.imports.import(self.source, ty);

                        if matches!(ty, TypeRef::Basetype(b) if b.of() == &Ty::Named(Cow::Borrowed("VkBool32"))) {
                            items.push(quote! { SmallVec<[bool; 1]> });
                        } else {
                            items.push(quote! { SmallVec<[#ident; 1]> });
                        }
                    },
                    Ty::Native(Native::Void) => {
                        items.push(quote! { Vec<u8> });
                    },
                    Ty::Native(native) => {
                        items.push(quote! { Vec<#native> });
                    },
                    other => unreachable!("ArrayWrittenTo should only be used for named types: {:#?}", other),
                },
                _ => {},
            }
        }

        ReturnType { items, result }
    }

    pub fn as_function_arguments(&mut self) -> ArgumentDefinitions {
        let mut generics = Vec::new();
        let mut self_ = None;
        let mut args = Vec::with_capacity(self.args.len());
        for kind in &self.args {
            match kind {
                ArgKind::Regular(arg) => args.push(Argument {
                    ident: arg.as_ident(),
                    ty: hl_argument_type(
                        self.edge_cases,
                        self.source,
                        self.handle,
                        self.function,
                        arg,
                        self.imports,
                    ),
                    optional: false,
                }),
                ArgKind::Optional(arg) => args.push(Argument {
                    ident: arg.as_ident(),
                    ty: hl_argument_type(
                        self.edge_cases,
                        self.source,
                        self.handle,
                        self.function,
                        arg,
                        self.imports,
                    ),
                    optional: true,
                }),
                ArgKind::Array(arg) => args.push(Argument {
                    ident: arg.as_ident(),
                    ty: hl_argument_type(
                        self.edge_cases,
                        self.source,
                        self.handle,
                        self.function,
                        arg,
                        self.imports,
                    ),
                    optional: arg.optionality().is_optional(),
                }),
                ArgKind::Passthrough(arg) => args.push(Argument {
                    ident: arg.as_ident(),
                    ty: hl_argument_type(
                        self.edge_cases,
                        self.source,
                        self.handle,
                        self.function,
                        arg,
                        self.imports,
                    ),
                    optional: arg.optionality().is_optional(),
                }),
                ArgKind::This(arg) => {
                    let mut_ = arg.externally_synced().is_yes().then(|| quote! { mut });
                    self_ = Some(Argument {
                        ident: format_ident!("self"),
                        ty: quote! { &#mut_ self },
                        optional: true,
                    });
                },
                ArgKind::CStr(arg) => {
                    let ident = format_ident!("{}", arg.name().to_upper_camel_case());
                    generics.push(Generics {
                        ident: ident.clone(),
                        bounds: quote! { AsRef<str> },
                    });

                    args.push(Argument {
                        ident: arg.as_ident(),
                        ty: quote! { #ident },
                        optional: arg.optionality().is_optional(),
                    })
                },
                // These are output values, so we don't need to pass them in.
                ArgKind::ValueWrittenTo(_, _) | ArgKind::ArrayWrittenTo(_, _, _) | ArgKind::AllocationCallback(_) => {},

                // These are handled by the `len` function of the input slice/array.
                ArgKind::LenByArray(_, _) => {},
                ArgKind::ValueWrittenToChained(arg, _) => {
                    if let Ty::Named(name) = arg.ty() {
                        if let TypeRef::Struct(s) = self.source.resolve_type(name).expect("unknown type") {
                            let name = s.as_ident();
                            args.push(Argument {
                                ident: arg.as_ident(),
                                ty: quote! { Option<#name> },
                                optional: false,
                            })
                        }
                    }
                },
                ArgKind::LenExtraCall(arg, _) => args.push(Argument {
                    ident: arg.as_ident(),
                    ty: quote! { Option<usize> },
                    optional: false,
                }),
            }
        }

        ArgumentDefinitions { args, self_, generics }
    }

    pub fn args(&self) -> &[ArgKind] {
        self.args.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct ReturnType {
    pub items: Vec<TokenStream>,
    pub result: bool,
}

#[derive(Debug, Clone)]
pub struct ArgumentDefinitions {
    pub args: Vec<Argument>,
    pub self_: Option<Argument>,
    pub generics: Vec<Generics>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub ident: Ident,
    pub ty: TokenStream,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct Generics {
    pub ident: Ident,
    pub bounds: TokenStream,
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
    LenByArray(FunctionArgument<'a>, Vec<(usize, FunctionArgument<'a>)>),

    /// The allocation callback argument
    AllocationCallback(FunctionArgument<'a>),

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
    LenExtraCall(FunctionArgument<'a>, Vec<(usize, FunctionArgument<'a>)>),
}

impl<'a> ArgKind<'a> {
    pub fn from_args<'b>(
        source: &Source<'b>,
        handle_name: &str,
        args: &SymbolTable<'b, FunctionArgument<'b>>,
    ) -> Vec<Self>
    where
        'b: 'a,
    {
        let mut out = Vec::new();

        let mut lengths = HashSet::new();
        for (i, arg) in args.iter().enumerate() {
            // Check for the allocation callbacks
            if arg.ty() == &Ty::Pointer(Mutability::Const, Box::new(Ty::Named(Cow::Borrowed("VkAllocationCallbacks")))) {
                out.push(ArgKind::AllocationCallback(arg.clone()));
                continue;
            }

            // We check if we are a handle
            if i == 0 && arg.ty() == &Ty::Named(Cow::Borrowed(handle_name)) {
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
                .collect::<Vec<_>>();

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
            | ArgKind::AllocationCallback(arg)
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

    pub fn as_static(&self) -> ArgKind<'static> {
        match self {
            ArgKind::Regular(arg) => ArgKind::Regular(arg.as_static()),
            ArgKind::Array(arg) => ArgKind::Array(arg.as_static()),
            ArgKind::Optional(arg) => ArgKind::Optional(arg.as_static()),
            ArgKind::Passthrough(arg) => ArgKind::Passthrough(arg.as_static()),
            ArgKind::This(arg) => ArgKind::This(arg.as_static()),
            ArgKind::CStr(arg) => ArgKind::CStr(arg.as_static()),
            ArgKind::ValueWrittenTo(arg, inner) => ArgKind::ValueWrittenTo(arg.as_static(), inner.as_static()),
            ArgKind::ValueWrittenToChained(arg, inner) => {
                ArgKind::ValueWrittenToChained(arg.as_static(), inner.as_static())
            },
            ArgKind::ArrayWrittenTo(arg, inner, len) => {
                ArgKind::ArrayWrittenTo(arg.as_static(), inner.as_static(), len.as_static())
            },
            ArgKind::LenByArray(arg, len) => {
                ArgKind::LenByArray(arg.as_static(), len.iter().map(|(i, a)| (*i, a.as_static())).collect())
            },
            ArgKind::AllocationCallback(arg) => ArgKind::AllocationCallback(arg.as_static()),
            ArgKind::LenExtraCall(arg, len) => {
                ArgKind::LenExtraCall(arg.as_static(), len.iter().map(|(i, a)| (*i, a.as_static())).collect())
            },
        }
    }
}

impl<'a> Deref for ArgKind<'a> {
    type Target = FunctionArgument<'a>;

    fn deref(&self) -> &Self::Target {
        self.arg()
    }
}

pub fn hl_argument_type<'a>(
    edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
    source: &Source<'a>,
    owner: &Handle<'_>,
    function: &Function<'_>,
    argument: &FunctionArgument<'_>,
    imports: &mut Imports,
) -> TokenStream {
    if let Some(edge_case) = edge_cases.function_argument_ty(source, imports, owner, function, argument) {
        return edge_case;
    }

    let mut wrapper_fn = edge_cases
        .function_argument_wrapper(source, imports, owner, function, argument)
        .unwrap_or_else(|| Box::new(|ty| ty));

    let mut current = Some(argument.ty());
    while let Some(next) = current.take() {
        match next {
            Ty::Pointer(mut_, type_) => {
                let mut_ = matches!(mut_, Mutability::Mutable).then(|| quote! { mut });
                wrapper_fn = Box::new(move |ty| {
                    quote! {
                        &#mut_ #ty
                    }
                });

                current = Some(type_)
            },
            ty @ Ty::Named(name) => {
                return wrapper_fn(
                    edge_cases
                        .function_argument_named_ty(source, imports, owner, function, argument, name)
                        .unwrap_or_else(|| field_type(source, ty, imports)),
                )
            },
            ty @ Ty::Native(_) => return wrapper_fn(field_type(source, ty, imports)),
            Ty::StringArray(_) | Ty::NullTerminatedString(_) => return wrapper_fn(quote! { String }),
            Ty::Array(ty, len) => {
                if &**ty == &Ty::Native(magritte_types::Native::Char) {
                    return wrapper_fn(quote! { String });
                }

                let len = constant_value(len, source, imports);

                wrapper_fn = Box::new(move |ty| {
                    quote! {
                        [#ty; #len as usize]
                    }
                });

                current = Some(ty);
            },
            Ty::Slice(mut_, ty, _) => {
                let mut_ = matches!(mut_, Mutability::Mutable).then(|| quote! { mut });
                wrapper_fn = Box::new(move |ty| {
                    quote! {
                        &#mut_ [#ty]
                    }
                });

                current = Some(ty);
            },
        }
    }

    unreachable!()
}
