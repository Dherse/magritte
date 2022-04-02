use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};

use crate::{
    doc::Documentation,
    imports::Imports,
    source::{Function, FunctionPointer, FunctionPointerArgument, Source},
};

impl<'a> Function<'a> {
    /// Turns this function into an equivalent function pointer name
    pub fn as_fn_pointer_name(&self) -> String {
        format!("FN{}", self.name().to_case(Case::UpperCamel))
    }

    /// Turns this function into an equivalent function pointer identifier
    pub fn as_fn_pointer_ident(&self) -> Ident {
        Ident::new(&self.as_fn_pointer_name(), Span::call_site())
    }

    /// Turns this function into an equivalent function pointer type
    pub fn as_fn_pointer(&self) -> FunctionPointer<'a> {
        FunctionPointer {
            original_name: self.original_name.clone(),
            name: self.as_fn_pointer_name(),
            arguments: self
                .arguments()
                .iter()
                .map(|arg| FunctionPointerArgument {
                    original_name: arg.original_name.clone(),
                    name: arg.name.clone(),
                    ty: arg.ty.clone(),
                })
                .collect(),
            return_type: self.return_type().cloned(),
            origin: self.origin().clone(),
        }
    }

    /// Generates the function code as a type
    pub fn generate_type_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        out: &mut TokenStream,
    ) {
        self.as_fn_pointer().generate_code(source, doc, imports, out);
    }
}
