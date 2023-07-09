use magritte_build::imports::Imports;
use magritte_types::{Field, Source, Ty, TypeRef};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    edge_case::EdgeCase,
    native::{field::field_type, r#const::constant_value},
};

pub fn hl_field_type<'a>(
    edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
    source: &Source<'a>,
    owner: TypeRef<'_, '_>,
    field: &Field<'_>,
    imports: &mut Imports,
) -> TokenStream {
    if let Some(edge_case) = edge_cases.field_ty(source, imports, owner, field) {
        return edge_case;
    }

    let mut wrapper_fn = edge_cases
        .field_wrapper_ty(edge_cases, source, imports, owner, field)
        .unwrap_or_else(|| Box::new(|ty| ty));

    let mut current = Some(field.ty());
    while let Some(next) = current.take() {
        match next {
            Ty::Pointer(_, type_) => current = Some(type_),
            ty @ Ty::Named(name) => {
                return wrapper_fn(
                    edge_cases
                        .field_named_ty(source, imports, owner, field, name)
                        .unwrap_or_else(|| field_type(source, ty, imports)),
                )
            },
            ty @ Ty::Native(_) => return wrapper_fn(field_type(source, ty, imports)),
            Ty::StringArray(_) | Ty::NullTerminatedString(_) => return wrapper_fn(quote::quote! { String }),
            Ty::Array(ty, len) => {
                if &**ty == &Ty::Native(magritte_types::Native::Char) {
                    return wrapper_fn(quote::quote! { String });
                }

                let len = constant_value(len, source, imports);

                wrapper_fn = Box::new(move |ty| {
                    quote! {
                        [#ty; #len as usize]
                    }
                });

                current = Some(ty);
            },
            Ty::Slice(_, ty, _) => {
                imports.push("smallvec::SmallVec");
                wrapper_fn = Box::new(move |ty| {
                    quote! {
                        SmallVec<[#ty; 8]>
                    }
                });

                current = Some(ty);
            },
        }
    }

    unreachable!()
}
