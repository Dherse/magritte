use magritte_build::imports::Imports;
use magritte_types::{Expr, Mutability, Source, Ty};
use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};

pub fn constant_type<'a>(ty: &Ty<'a>, imports: &mut Imports) -> TokenStream {
    match ty {
        Ty::Native(native) => native.to_token_stream(),
        Ty::NullTerminatedString(Mutability::Const) => {
            imports.push("std::ffi::CStr");
            quote! { &'static CStr}
        },
        _ => unreachable!("constant can be either native values or null-terminated strings"),
    }
}

pub fn constant_value<'a>(expr: &Expr<'a>, source: &Source<'a>, imports: &mut Imports) -> TokenStream {
    match expr {
        Expr::String(val) => {
            imports.push("crate::cstr");

            quote! {
                cstr!(#val)
            }
        },
        Expr::Constant(cst) => {
            let a = source.find(cst).expect("unknown constant");
            let value = a.as_ident();

            imports.push_origin(source, a.origin(), a.name());

            quote! { #value }
        },
        Expr::ConstantInt(cst) => {
            let lit = Literal::i64_unsuffixed(*cst);
            quote! { #lit }
        },
        Expr::ConstantFloat(cst) => {
            let lit = Literal::f32_unsuffixed(*cst);
            quote! { #lit }
        },
        Expr::BitwiseNot(cst) => {
            let value = constant_value(cst, source, imports);
            quote! { !#value }
        },
        Expr::Neg(cst) => {
            let value = constant_value(cst, source, imports);
            quote! { -#value }
        },
        Expr::Divide(a, b) => {
            let a = constant_value(a, source, imports);
            let b = constant_value(b, source, imports);
            quote! { #a / #b }
        },
        Expr::Multiply(a, b) => {
            let a = constant_value(a, source, imports);
            let b = constant_value(b, source, imports);
            quote! { #a * #b }
        },
        Expr::Add(a, b) => {
            let a = constant_value(a, source, imports);
            let b = constant_value(b, source, imports);
            quote! { #a + #b }
        },
        Expr::Subtract(a, b) => {
            let a = constant_value(a, source, imports);
            let b = constant_value(b, source, imports);
            quote! { #a - #b }
        },
        other => unreachable!("Unsupported expr: `{}`", other),
    }
}