use proc_macro2::TokenStream;

use crate::{
    doc::{Documentation},
    imports::Imports,
    source::{Const, Source, ConstAlias}, codegen::alias_of,
};

impl<'a> Const<'a> {
    /// Generates the code for a constant
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // the name as an identifier
        let name = self.as_ident();

        // the type of the constant
        let ty = self.ty().as_const_ty(source, Some(imports));

        // the value of the constant
        let value = self.value().as_const_expr(source, Some(imports));

        // append the doc first
        self.generate_doc(source, doc, out);

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote::quote_each_token! {
            out

            pub const #name: #ty = #value;
        }
    }

    fn generate_doc(&self, source: &Source<'a>, doc: &Documentation, mut out: &mut TokenStream) -> Option<()> {
        // Get the documentation element
        let doc = doc.find(self.original_name());

        let doc = doc?;

        // parse the name section and write it out
        let name = doc.name(source, self)?;
        let names = name.split('\n');
        quote::quote_each_token! {
            out

            #(#[doc = #names])*
        }

        // parse the c spec section and write it out if it exists
        if let Some(specification) = doc.specification(source, self) {
            let specs = specification.split('\n');
            quote::quote_each_token! {
                out

                #[doc = "# C Specifications"]
                #(#[doc = #specs])*
            }
        }

        quote::quote_each_token! {
            out

            #[doc = "For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#VK_MAX_DRIVER_NAME_SIZE)"]
            #[doc = "\n"]
            #[doc = "This documentation is generated from the Vulkan specifications."]
        }

        Some(())
    }
}

impl<'a> ConstAlias<'a> {
    /// Generates the code for a constant
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // get the identifier of the constant
        let name = self.as_ident();

        // find the original constant
        let of = source.constants.get_by_name(self.of()).expect("unknown constant");

        // the identifier of the original constant
        let of_ident = of.as_ident();

        // imports the original constant
        imports.push_origin(of.origin(), of.name());

        // get the type of the constant
        let ty = of.ty().as_const_ty(source, Some(imports));

        // generate the documentation based on the original constant
        of.generate_doc(source, doc, out);

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote::quote_each_token! {
            out

            pub const #name: #ty = #of_ident;
        }
    }
}