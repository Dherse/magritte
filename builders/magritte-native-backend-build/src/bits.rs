use magritte_build::origin::cond_of;
use magritte_types::{Bit, Source, Bitflag, Alias, SymbolTable, Origin, Enum};
use proc_macro2::{Literal, TokenStream, Ident};
use quote::{quote, quote_each_token};

use crate::visitor::{NativeBackendVisitor, NativeBackendOriginVisitor};

pub fn bits_of<'a, P: AsRef<str>>(
    parent: &mut NativeBackendVisitor,
    source: &Source<'a>,
    doc_dir: P,
    owner: &Bitflag<'a>,
    bit: &Bit<'a>,
) -> Option<TokenStream> {
    if bit.origin().is_disabled() {
        return None;
    }

    let name = bit.as_ident();
    let original_name = bit.original_name();
    let alias = bit.as_alias();
    let value = Literal::i64_unsuffixed(bit.value());

    let doc = parent.doc_of_child(doc_dir, owner.original_name(), original_name);
    let cond = cond_of(source, owner.origin(), bit.origin());

    Some(quote! {
        #doc
        #alias
        #cond
        pub const #name: Self = Self(#value);
    })
}

enum BitKind<'a> {
    Bit(&'a Bit<'a>),
    Alias(&'a Alias<'a>),
}

impl<'a> BitKind<'a> {
    fn origin(&self) -> &Origin<'a> {
        match self {
            Self::Bit(bit) => bit.origin(),
            Self::Alias(alias) => alias.origin(),
        }
    }

    fn as_ident(&self) -> proc_macro2::Ident {
        match self {
            Self::Bit(bit) => bit.as_ident(),
            Self::Alias(alias) => alias.as_ident(),
        }
    }
}

pub fn alias_of<'a, P: AsRef<str>>(
    parent: &mut NativeBackendVisitor,
    source: &Source<'a>,
    doc_dir: P,
    owner: &Bitflag<'a>,
    alias: &Alias<'a>,
) -> Option<TokenStream> {
    if alias.origin().is_disabled() {
        return None;
    }

    let name = alias.as_ident();
    let original_name = alias.original_name();
    let alias_doc = alias.as_alias();

    let bit = owner.bits().get_by_either(alias.of()).map(BitKind::Bit).or_else(|| owner.aliases().get_by_either(alias.of()).map(BitKind::Alias)).expect("unknown alias");
    if bit.origin().is_disabled() {
        return None;
    }

    let bit_ident = bit.as_ident();

    let doc = parent.doc_of_child(doc_dir, owner.original_name(), original_name);
    let cond = cond_of(source, owner.origin(), alias.origin());

    Some(quote! {
        #doc
        #alias_doc
        #cond
        pub const #name: Self = Self:: #bit_ident;
    })
}
pub trait EquivalentBitflag<'a> {
    fn bits(&self) -> &SymbolTable<'a, Bit<'a>>;

    fn aliases(&self) -> &SymbolTable<'a, Alias<'a>>;

    fn original_name(&self) -> &str;

    fn as_ident(&self) -> Ident;

    fn as_alias(&self) -> Option<TokenStream>;

    fn origin(&self) -> &Origin<'a>;
}

impl<'a> EquivalentBitflag<'a> for Bitflag<'a> {
    fn bits(&self) -> &SymbolTable<'a, Bit<'a>> {
        self.bits()
    }

    fn aliases(&self) -> &SymbolTable<'a, Alias<'a>> {
        self.aliases()
    }

    fn original_name(&self) -> &str {
        self.original_name()
    }

    fn origin(&self) -> &Origin<'a> {
        self.origin()
    }

    fn as_ident(&self) -> Ident {
        self.as_ident()
    }

    fn as_alias(&self) -> Option<TokenStream> {
        self.as_alias()
    }
}

impl<'a> EquivalentBitflag<'a> for Enum<'a> {
    fn bits(&self) -> &SymbolTable<'a, Bit<'a>> {
        self.variants()
    }

    fn aliases(&self) -> &SymbolTable<'a, Alias<'a>> {
        self.aliases()
    }

    fn original_name(&self) -> &str {
        self.original_name()
    }

    fn origin(&self) -> &Origin<'a> {
        self.origin()
    }

    fn as_ident(&self) -> Ident {
        self.as_ident()
    }

    fn as_alias(&self) -> Option<TokenStream> {
        self.as_alias()
    }
}

impl<'parent> NativeBackendOriginVisitor<'parent> {
    pub fn gen_for_biflag<'a>(&mut self, source: &Source<'a>, bitflag: &impl EquivalentBitflag<'a>, ty: &TokenStream) {

        let name = bitflag.as_ident();
        let alias = bitflag.as_alias();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, bitflag.original_name());

        let bits = bitflag.bits().iter().map(|bit| bits_of_bitflag(
            self.parent,
            source, 
            &self.doc_dir_path,
            bitflag,
            bit,
        )).collect::<Vec<_>>();

        let aliases = bitflag.aliases().iter().map(|bit| alias_of_bitflag(
            self.parent,
            source, 
            &self.doc_dir_path,
            bitflag,
            bit,
        )).collect::<Vec<_>>();

        let bit_idents = bitflag.bits().iter().map(|bit| bit.as_ident());
        let conds = bitflag.bits().iter().map(|bit| cond_of(source, bitflag.origin(), bit.origin()));

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #doc
            #alias
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            #[non_exhaustive]
            pub struct #name(#ty);

            impl #name {
                #(#bits)*
                #(#aliases)*

                #[doc = "Default empty flags"]
                #[inline]
                pub const fn empty() -> Self {
                    Self(0)
                }

                #[doc = "The bits of this variant"]
                #[inline]
                pub const fn bits(&self) -> #ty {
                    self.0
                }

                #[doc = "Builds a bitmask from the bits of this variant"]
                #[inline]
                pub const fn from_bits(bits: #ty) -> Option<Self> {
                    match bits {
                        #(
                            #conds
                            x if x == Self::#bit_idents.bits() => Some(Self(x)),
                        )*
                        _ => None,
                    }
                }

                #[doc = "Builds a bitmask from the bits of this variant without validating it"]
                #[inline]
                pub const unsafe fn from_bits_unchecked(bits: #ty) -> Self {
                    Self(bits)
                }
            }
        }
    }
}

pub fn bits_of_bitflag<'a, P: AsRef<str>>(
    parent: &mut NativeBackendVisitor,
    source: &Source<'a>,
    doc_dir: P,
    owner: &impl EquivalentBitflag<'a>,
    bit: &Bit<'a>,
) -> Option<TokenStream> {
    if bit.origin().is_disabled() {
        return None;
    }

    let name = bit.as_ident();
    let alias = bit.as_alias();
    let original_name = bit.original_name();
    let value = Literal::i64_unsuffixed(bit.value());

    let doc = parent.doc_of_child(doc_dir, owner.original_name(), original_name);
    let cond = cond_of(source, owner.origin(), bit.origin());

    Some(quote! {
        #doc
        #alias
        #cond
        pub const #name: Self = Self(#value);
    })
}

pub fn alias_of_bitflag<'a, P: AsRef<str>>(
    parent: &mut NativeBackendVisitor,
    source: &Source<'a>,
    doc_dir: P,
    owner: &impl EquivalentBitflag<'a>,
    alias: &Alias<'a>,
) -> Option<TokenStream> {
    if alias.origin().is_disabled() {
        return None;
    }

    let name = alias.as_ident();
    let doc_alias = alias.as_alias();
    let original_name = alias.original_name();

    let bit = owner.bits().get_by_either(alias.of()).map(BitKind::Bit).or_else(|| owner.aliases().get_by_either(alias.of()).map(BitKind::Alias)).expect("unknown alias");
    if bit.origin().is_disabled() {
        return None;
    }

    let bit_ident = bit.as_ident();

    let doc = parent.doc_of_child(doc_dir, owner.original_name(), original_name);
    let cond = cond_of(source, owner.origin(), alias.origin());

    Some(quote! {
        #doc
        #doc_alias
        #cond
        pub const #name: Self = Self::#bit_ident;
    })
}