use proc_macro2::{TokenStream, Literal};
use quote::{ToTokens, quote_each_token};

use crate::{ty::Ty, source::Source, imports::Imports, expr::Expr};

pub struct ConstExpr<'a: 'b, 'b>(pub &'b Expr<'a>, pub &'b Source<'a>, pub &'b Imports);

impl<'a: 'b, 'b> ToTokens for ConstExpr<'a, 'b> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        match self.0 {
            Expr::String(str) => str.to_tokens(tokens),
            Expr::Variable(_) => unreachable!("variables are not supported in constant expressions"),
            Expr::Resolve(_, _) => unreachable!("resolves are not supported in constant expressions"),
            Expr::Constant(name) => {
                let constant = self.1.find(name).expect("unknown constant");
                constant.import(self.2);

                constant.as_ident().to_tokens(tokens);
            },
            Expr::ConstantInt(value) => {
                Literal::i64_unsuffixed(*value).to_tokens(tokens);
            },
            Expr::ConstantFloat(value) => {
                Literal::f32_unsuffixed(*value).to_tokens(tokens);
            },
            Expr::Divide(a, b) => {
                let a = ConstExpr(a, self.1, self.2);
                let b = ConstExpr(b, self.1, self.2);

                quote_each_token!{
                    tokens

                    #a / #b
                }
            },
            Expr::Multiply(a, b) => {
                let a = ConstExpr(a, self.1, self.2);
                let b = ConstExpr(b, self.1, self.2);

                quote_each_token!{
                    tokens

                    #a * #b
                }
            },
            Expr::Add(a, b) => {
                let a = ConstExpr(a, self.1, self.2);
                let b = ConstExpr(b, self.1, self.2);

                quote_each_token!{
                    tokens

                    #a + #b
                }
            },
            Expr::Subtract(a, b) => {
                let a = ConstExpr(a, self.1, self.2);
                let b = ConstExpr(b, self.1, self.2);

                quote_each_token!{
                    tokens

                    #a - #b
                }
            },
            Expr::BitwiseNot(a) => {
                let a = ConstExpr(a, self.1, self.2);

                quote_each_token!{
                    tokens

                    !#a
                }
            },
            Expr::Neg(a) => {
                let a = ConstExpr(a, self.1, self.2);

                quote_each_token!{
                    tokens

                    -#a
                }
            },
        }
    }
}

pub struct NativeType<'a: 'b, 'b>(pub &'b Ty<'a>, pub &'b Source<'a>, pub &'b Imports);

impl<'a: 'b, 'b> ToTokens for NativeType<'a, 'b> {
    fn to_tokens(&self, mut tokens: &mut TokenStream) {
        match self.0 {
            Ty::Pointer(mutability, ty) => {
                let mut_ = mutability.to_pointer_tokens();
                let ty = NativeType(ty, self.1, self.2);
                quote_each_token!(tokens *#mut_ #ty);
            },
            Ty::Native(native) => native.to_tokens(tokens),
            Ty::Named(name) => {
                let type_ref = self.1.find_type(name).expect("unknown type");

                self.2.push_origin(type_ref.origin(), type_ref.name());

                let ty = type_ref.as_ident();
                quote_each_token!(tokens #ty);
            },
            Ty::StringArray(len) => {
                self.2.push("std::ffi::c_char");
                let len = ConstExpr(len, self.1, self.2);
                quote_each_token!(tokens [c_char; #len]);
            },
            Ty::NullTerminatedString(mut_) => {
                self.2.push("std::ffi::CStr");

                let mut_ = mut_.to_pointer_tokens();

                quote_each_token!(tokens *#mut_ CStr);
            },
            Ty::Array(ty, len) => {
                let ty = NativeType(ty, self.1, self.2);
                let len = ConstExpr(len, self.1, self.2);

                quote_each_token!(tokens [#ty; #len]);
            },
            Ty::Slice(mut_, ty, _) => {
                let mut_ = mut_.to_pointer_tokens();
                let ty = NativeType(ty, self.1, self.2);

                quote_each_token!(tokens *#mut_ #ty);
            },
        }
    }
}