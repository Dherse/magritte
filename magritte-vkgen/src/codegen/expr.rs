use proc_macro2::Literal;
use syn::parse_quote;

use crate::{
    expr::Expr,
    imports::Imports,
    source::{reference::Ref, Source},
};

impl<'a> Expr<'a> {
    /// Turns an expression into a tokenized const-evaluable rust expression.
    pub fn as_const_expr(&self, source: &Source<'a>, imports: Option<&Imports>) -> syn::Expr {
        match self {
            Expr::String(value) => parse_quote! {
                crate::cstr!(#value)
            },
            Expr::Variable(_) => panic!("variable are not supported in const expressions"),
            Expr::Constant(name) => {
                let ref_ = source.find(name).expect("unknown constant");
                let (origin, ident) = match ref_ {
                    Ref::Const(const_) => (const_.origin(), const_.as_ident()),
                    Ref::ConstAlias(const_) => (const_.origin(), const_.as_ident()),
                    other => unreachable!(
                        "a constant should be a constant or a constant alias, found: {:?}",
                        other
                    ),
                };

                if let Some(imports) = imports {
                    imports.push_origin(origin, &ident);

                    parse_quote! { #ident }
                } else {
                    let path = origin.as_path();

                    parse_quote! {
                        #path :: #ident
                    }
                }
            },
            Expr::ConstantInt(int) => {
                let lit = Literal::i64_unsuffixed(*int);

                parse_quote! { #lit }
            },
            Expr::ConstantFloat(float) => {
                let lit = Literal::f32_unsuffixed(*float);

                parse_quote! { #lit }
            },
            Expr::Resolve(_, _) => panic!("resolves are not supported in const expressions"),
            Expr::Divide(a, b) => {
                let a = a.as_const_expr(source, imports);
                let b = b.as_const_expr(source, imports);

                parse_quote! {
                    #a / #b
                }
            },
            Expr::Multiply(a, b) => {
                let a = a.as_const_expr(source, imports);
                let b = b.as_const_expr(source, imports);

                parse_quote! {
                    #a * #b
                }
            },
            Expr::Add(a, b) => {
                let a = a.as_const_expr(source, imports);
                let b = b.as_const_expr(source, imports);

                parse_quote! {
                    #a + #b
                }
            },
            Expr::Subtract(a, b) => {
                let a = a.as_const_expr(source, imports);
                let b = b.as_const_expr(source, imports);

                parse_quote! {
                    #a - #b
                }
            },
            Expr::BitwiseNot(a) => {
                let a = a.as_const_expr(source, imports);

                parse_quote! {
                    !#a
                }
            },
            Expr::Neg(a) => {
                let a = a.as_const_expr(source, imports);

                parse_quote! {
                    - #a
                }
            },
        }
    }
}
