use proc_macro2::{Ident, Span, TokenStream};

use crate::{
    doc::Documentation,
    imports::Imports,
    source::{Const, Source},
};

impl<'a> Const<'a> {
    /// Generates the code for a constant
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &Documentation,
        imports: &mut Imports,
        out: &mut TokenStream,
    ) {
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
    }
}
