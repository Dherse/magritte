use std::fmt::Write;

use proc_macro2::{Literal, TokenStream};
use pyo3::Python;
use quote::quote;
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
                    (#a / #b)
                }
            },
            Expr::Multiply(a, b) => {
                let a = a.as_const_expr(source, imports);
                let b = b.as_const_expr(source, imports);

                parse_quote! {
                    (#a * #b)
                }
            },
            Expr::Add(a, b) => {
                let a = a.as_const_expr(source, imports);
                let b = b.as_const_expr(source, imports);

                parse_quote! {
                    (#a + #b)
                }
            },
            Expr::Subtract(a, b) => {
                let a = a.as_const_expr(source, imports);
                let b = b.as_const_expr(source, imports);

                parse_quote! {
                    (#a - #b)
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

    /// Pivots the expression to solve the expression
    pub fn pivot(&self, replace: &'a str) -> Expr<'a> {
        let as_string = format!("{} - {}", self.to_string(), replace);

        let result = Box::leak(box Python::with_gil(|py| {
            let mut code = "import sympy\n".to_string();

            writeln!(code, "{0} = sympy.Symbol('{0}')", replace).unwrap();

            for variable in self.variables() {
                writeln!(code, "{0} = sympy.Symbol('{0}')", variable).unwrap();
            }

            for constant in self.constants() {
                writeln!(code, "{0} = sympy.Symbol('{0}')", constant).unwrap();
            }

            writeln!(code, "eq = {}", as_string).unwrap();
    
            py.run(&code, None, None)?;

            py.eval(&format!("str(sympy.solveset(eq, {}))", replace), None, None)?.extract::<String>()
        }).expect("failed to pivot the expression"));
        
        Expr::new(&result[1..result.len() - 1])
    }

    /// Turns an expression into a tokenized  evaluable rust expression.
    pub fn as_expr<F>(&self, source: &Source<'a>, getter: &F, imports: Option<&Imports>) -> TokenStream where F: Fn(&str) -> TokenStream {
        match self {
            Expr::String(value) => quote! {
                crate::cstr!(#value)
            },
            Expr::Variable(var) => getter(var),
            Expr::Resolve(a, b) => {
                let a = a.as_expr(source, getter, imports);
                let b = b.as_expr(source, getter, imports);

                quote! { #a . #b }
            },
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

                    quote! { #ident }
                } else {
                    let path = origin.as_path();

                    quote! {
                        #path :: #ident
                    }
                }
            },
            Expr::ConstantInt(int) => {
                let lit = Literal::i64_unsuffixed(*int);

                quote! { #lit }
            },
            Expr::ConstantFloat(float) => {
                let lit = Literal::f32_unsuffixed(*float);

                quote! { #lit }
            },
            Expr::Divide(a, b) => {
                let a = a.as_expr(source, getter, imports);
                let b = b.as_expr(source, getter, imports);

                quote! {
                    #a / #b
                }
            },
            Expr::Multiply(a, b) => {
                let a = a.as_expr(source, getter, imports);
                let b = b.as_expr(source, getter, imports);

                quote! {
                    #a * #b
                }
            },
            Expr::Add(a, b) => {
                let a = a.as_expr(source, getter, imports);
                let b = b.as_expr(source, getter, imports);

                quote! {
                    #a + #b
                }
            },
            Expr::Subtract(a, b) => {
                let a = a.as_expr(source, getter, imports);
                let b = b.as_expr(source, getter, imports);

                quote! {
                    #a - #b
                }
            },
            Expr::BitwiseNot(a) => {
                let a = a.as_expr(source, getter, imports);

                quote! {
                    !#a
                }
            },
            Expr::Neg(a) => {
                let a = a.as_expr(source, getter, imports);

                quote! {
                    - #a
                }
            },
        }
    }
}

impl<'a> ToString for Expr<'a> {
    fn to_string(&self) -> String {
        match self {
            Expr::String(str) => str.to_owned(),
            Expr::Variable(var) => var.to_string(),
            Expr::Constant(cst) => cst.to_string(),
            Expr::ConstantInt(cst) => cst.to_string(),
            Expr::ConstantFloat(cst) => cst.to_string(),
            Expr::Resolve(a, b) => format!("{}->{}", a.to_string(), b.to_string()),
            Expr::Divide(a, b) => format!("({} / {})", a.to_string(), b.to_string()),
            Expr::Multiply(a, b) => format!("({} * {})", a.to_string(), b.to_string()),
            Expr::Add(a, b) => format!("({} + {})", a.to_string(), b.to_string()),
            Expr::Subtract(a, b) => format!("({} - {})", a.to_string(), b.to_string()),
            Expr::BitwiseNot(a) => format!("(~{})", a.to_string()),
            Expr::Neg(a) => format!("(-{})", a.to_string()),
        }
    }
}