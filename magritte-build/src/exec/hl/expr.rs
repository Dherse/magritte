use magritte_build::imports::Imports;
use magritte_parse::{Expr, Source};
use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub fn converter_value<'a, F: Fn(&str) -> TokenStream>(
    expr: &Expr<'a>,
    getter: &F,
    source: &Source<'a>,
    imports: &mut Imports,
) -> TokenStream {
    match expr {
        Expr::String(val) => {
            imports.push("crate::cstr");

            quote! {
                cstr!(#val)
            }
        },
        Expr::Variable(var) => getter(var),
        Expr::Resolve(a, b) => {
            let a = converter_value(a, getter, source, imports);
            let b = converter_value(b, getter, source, imports);

            quote! {
                self. #a . #b
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
            let value = converter_value(cst, getter, source, imports);
            quote! { !#value }
        },
        Expr::Neg(cst) => {
            let value = converter_value(cst, getter, source, imports);
            quote! { -#value }
        },
        Expr::Divide(a, b) => {
            let a = converter_value(a, getter, source, imports);
            let b = converter_value(b, getter, source, imports);
            quote! { (#a) / (#b) }
        },
        Expr::Multiply(a, b) => {
            let a = converter_value(a, getter, source, imports);
            let b = converter_value(b, getter, source, imports);
            quote! { (#a) * (#b) }
        },
        Expr::Add(a, b) => {
            let a = converter_value(a, getter, source, imports);
            let b = converter_value(b, getter, source, imports);
            quote! { (#a) + (#b) }
        },
        Expr::Subtract(a, b) => {
            let a = converter_value(a, getter, source, imports);
            let b = converter_value(b, getter, source, imports);
            quote! { (#a) - (#b) }
        },
    }
}
