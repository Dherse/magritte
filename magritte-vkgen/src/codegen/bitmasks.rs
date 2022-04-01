use ahash::AHashMap;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use tracing::warn;

use crate::{
    codegen::alias_of,
    doc::Documentation,
    imports::Imports,
    origin::Origin,
    source::{Bit, BitFlag, Bitmask, Source},
};

impl<'a> Bit<'a> {
    /// Generates an identifier for flags
    pub fn as_flag_ident(&self) -> Ident {
        let name = self.name().to_case(Case::ScreamingSnake);
        Ident::new(
            &if name.starts_with(char::is_numeric) {
                format!("_{}", self.name().to_case(Case::ScreamingSnake))
            } else {
                self.name().to_case(Case::ScreamingSnake)
            },
            Span::call_site(),
        )
    }

    /// Generate the code for a Bitflag variant
    fn generate_bitmask_variant(&self, parent: &Origin<'a>, doc: &AHashMap<String, String>) -> TokenStream {
        // get the doc of the bit
        let doc = doc.get(self.name()).map_or_else(
            || quote! { #[doc = "No documentation found"]},
            |t| quote! { #[doc = #t] },
        );

        // get the "provided by" of the bit
        let provided_by = (parent != self.origin()).then(|| {
            let path = self.origin().as_path_str();
            let doc = format!("\nProvided by [`{}`]", path);
            quote! {
                #[doc = #doc]
            }
        });

        // get the name
        let name = self.as_flag_ident();
        let value = Literal::i64_unsuffixed(self.value());

        quote! {
            #doc
            #provided_by
            pub const #name: Self = Self(#value)
        }
    }
}

impl<'a> Bitmask<'a> {
    /// Generates the code for an enum
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        out: &mut TokenStream,
    ) {
        if let Some(bit_flag) = self
            .bits()
            .and_then(|bits| source.resolve_type(bits))
            .and_then(|bit| bit.as_bitflag())
        {
            self.generate_code_with_bitflag(source, doc, imports, bit_flag, out);
        } else {
            self.generate_code_without_bitflag(source, doc, out);
        }
    }

    fn generate_code_with_bitflag(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        bit_flag: &BitFlag<'a>,
        mut out: &mut TokenStream,
    ) {
        // the name of the bitflag
        let name = self.as_ident();

        // the name of the bitflag as an identifier
        let bits_name = bit_flag.as_ident();

        // generate the doc for the bitflag
        let variant_docs = bit_flag.generate_doc(source, doc, out).unwrap_or_default();

        // get the underlying bit type
        let ty = match bit_flag.width() {
            4 => quote! { u32 },
            8 => quote! { u64 },
            _ => unreachable!("unknown bit width for a mask: {:?}", self),
        };

        let bits = bit_flag
            .bits()
            .iter()
            .map(|bit| bit.generate_bitmask_variant(self.origin(), &variant_docs));

        let bit_idents = bit_flag.bits().iter().map(Bit::as_flag_ident).collect::<Vec<_>>();

        let first = if bit_idents.is_empty() { None } else { Some(quote! { let mut first = true; }) };

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        imports.push("std::iter::Extend");
        imports.push("std::iter::FromIterator");
        imports.push("std::iter::IntoIterator");

        // dealing with conditional compilation of flag bits
        let bits_conditional_compilation = if bit_flag.origin() != self.origin() {
            if let Some(condition) = bit_flag.origin().conditon() {
                imports.push_str(&format!(
                    r##"
                    {}
                    pub use {}::{};
                "##,
                    bit_flag.origin().feature_gate().unwrap_or_default(),
                    bit_flag.origin().as_path_str(),
                    bit_flag.name()
                ));
                Some(condition)
            } else {
                imports.push_origin(bit_flag.origin(), bit_flag.name());

                None
            }
        } else {
            imports.push_origin(bit_flag.origin(), bit_flag.name());

            None
        };

        quote::quote_each_token! {
            out

            #[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
            #[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            #[repr(transparent)]
            pub struct #name(#ty);

            impl const Default for #name {
                fn default() -> Self {
                    Self(0)
                }
            }

            #bits_conditional_compilation
            impl From<#bits_name> for #name {
                fn from(from: #bits_name) -> Self {
                    unsafe {
                        Self::from_bits_unchecked(from as #ty)
                    }
                }
            }

            impl #name {
                #(#bits;)*

                #[doc = "Default empty flags"]
                #[inline]
                pub const fn empty() -> Self {
                    Self::default()
                }

                #[doc = "Returns a value with all of the flags enabled"]
                #[inline]
                pub const fn all() -> Self {
                    Self::empty()
                        #( | Self::#bit_idents)*
                }

                #[doc = "Returns the raw bits"]
                #[inline]
                pub const fn bits(&self) -> #ty {
                    self.0
                }

                #[doc = "Convert raw bits into a bit flags checking that only valid"]
                #[doc = "bits are contained."]
                #[inline]
                pub const fn from_bits(bits: #ty) -> Option<Self> {
                    if (bits & !Self::all().bits()) == 0 {
                        Some(Self(bits))
                    } else {
                        None
                    }
                }


                #[doc = "Convert raw bits into a bit flags truncating all invalid"]
                #[doc = "bits that may be contained."]
                #[inline]
                pub const fn from_bits_truncate(bits: #ty) -> Self {
                    Self(Self::all().0 & bits)
                }

                #[doc = "Convert raw bits into a bit preserving all bits"]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = "The caller of this function must ensure that all of the bits are valid."]
                #[inline]
                pub const unsafe fn from_bits_unchecked(bits: #ty) -> Self {
                    Self(bits)
                }

                #[doc = "Returns `true` if no flags are currently set"]
                #[inline]
                pub const fn is_empty(&self) -> bool {
                    self.bits() == Self::empty().bits()
                }

                #[doc = "Returns `true` if all flags are currently set"]
                #[inline]
                pub const fn is_all(&self) -> bool {
                    self.bits() == Self::all().bits()
                }

                #[doc = "Returns `true` if there are flags in common to `self` and `other`"]
                #[inline]
                pub const fn intersects(&self, other: Self) -> bool {
                    !Self(self.bits() & other.bits()).is_empty()
                }

                #[doc = "Returns `true` if all of the flags in `other` are contained `self`"]
                #[inline]
                pub const fn contains(&self, other: Self) -> bool {
                    (self.bits() & other.bits()) == other.bits()
                }

                #[doc = "Inserts a set of flags in place"]
                #[inline]
                pub fn insert(&mut self, other: Self) {
                    self.0 |= other.bits()
                }

                #[doc = "Removes a set of flags in place"]
                #[inline]
                pub fn remove(&mut self, other: Self) {
                    self.0 &= !other.bits();
                }

                #[doc = "Toggles a set of flags in place"]
                #[inline]
                pub fn toggle(&mut self, other: Self) {
                    self.0 ^= other.bits();
                }

                #[doc = "Inserts or removes the specified flags depending on the value of `is_insert`"]
                #[inline]
                pub fn set(&mut self, other: Self, is_insert: bool) {
                    if is_insert {
                        self.insert(other);
                    } else {
                        self.remove(other);
                    }
                }

                #[doc = "Returns the intersection between `self` and `other`"]
                #[inline]
                pub const fn intersection(self, other: Self) -> Self {
                    Self(self.bits() & other.bits())
                }

                #[doc = "Returns the union between `self` and `other`"]
                #[inline]
                pub const fn union(self, other: Self) -> Self {
                    Self(self.bits() | other.bits())
                }

                #[doc = "Returns the difference between `self` and `other`"]
                #[inline]
                pub const fn difference(self, other: Self) -> Self {
                    Self(self.bits() & !other.bits())
                }

                #[doc = "Returns the [symmetric difference][sym-diff] between `self` and `other`"]
                #[doc = ""]
                #[doc = "[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference"]
                #[inline]
                pub const fn symmetric_difference(self, other: Self) -> Self {
                    Self(self.bits() ^ other.bits())
                }

                #[doc = "Returns the complement of `self`."]
                #[inline]
                pub const fn complement(self) -> Self {
                    Self::from_bits_truncate(!self.bits())
                }
            }

            impl const std::ops::BitOr for #name {
                type Output = Self;

                #[inline]
                fn bitor(self, other: Self) -> Self {
                    self.union(other)
                }
            }

            impl std::ops::BitOrAssign for #name {
                #[inline]
                fn bitor_assign(&mut self, other: Self) {
                    *self = *self | other;
                }
            }

            impl const std::ops::BitXor  for #name {
                type Output = Self;

                #[inline]
                fn bitxor(self, other: Self) -> Self {
                    self.symmetric_difference(other)
                }
            }

            impl std::ops::BitXorAssign  for #name {
                #[inline]
                fn bitxor_assign(&mut self, other: Self) {
                    *self = *self ^ other;
                }
            }

            impl const std::ops::BitAnd  for #name {
                type Output = Self;

                #[inline]
                fn bitand(self, other: Self) -> Self {
                    self.intersection(other)
                }
            }

            impl std::ops::BitAndAssign  for #name {
                #[inline]
                fn bitand_assign(&mut self, other: Self) {
                    *self = *self & other;
                }
            }

            impl const std::ops::Sub for #name {
                type Output = Self;

                #[inline]
                fn sub(self, other: Self) -> Self {
                    self.difference(other)
                }
            }

            impl std::ops::SubAssign for #name {
                #[inline]
                fn sub_assign(&mut self, other: Self) {
                    *self = *self - other;
                }
            }

            impl const std::ops::Not for #name {
                type Output = Self;

                #[inline]
                fn not(self) -> Self {
                    self.complement()
                }
            }

            impl Extend<#name> for #name {
                fn extend<T: IntoIterator<Item = #name>>(&mut self, iterator: T) {
                    for i in iterator {
                        Self::insert(self, i);
                    }
                }
            }

            #bits_conditional_compilation
            impl Extend<#bits_name> for #name {
                fn extend<T: IntoIterator<Item = #bits_name>>(&mut self, iterator: T) {
                    for i in iterator {
                        Self::insert(self, <Self as From<#bits_name>>::from(i));
                    }
                }
            }

            impl FromIterator<#name> for #name {
                fn from_iter<T: IntoIterator<Item = #name>>(iterator: T) -> #name {
                    let mut out = Self::empty();
                    <Self as Extend<#name>>::extend(&mut out, iterator);
                    out
                }
            }

            #bits_conditional_compilation
            impl FromIterator<#bits_name> for #name {
                fn from_iter<T: IntoIterator<Item = #bits_name>>(iterator: T) -> #name {
                    let mut out = Self::empty();
                    <Self as Extend<#bits_name>>::extend(&mut out, iterator);
                    out
                }
            }

            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    struct Flags(#name);
                    impl std::fmt::Debug for Flags {
                        #[allow(unused_assignments)]
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                            if self.0 == #name::empty() {
                                f.write_str("empty")?;
                            } else {
                                #first
                                #(
                                    if self.0.contains(#name::#bit_idents) {
                                        if !first {
                                            first = false;
                                            f.write_str(" | ")?;
                                        }

                                        f.write_str(stringify!(#bit_idents))?;
                                    }
                                )*
                            }

                            Ok(())
                        }
                    }

                    f.debug_tuple(stringify!(#name))
                        .field(&Flags(*self))
                        .finish()
                }
            }
        }
    }

    fn generate_code_without_bitflag(&self, source: &Source<'a>, doc: &mut Documentation, mut out: &mut TokenStream) {
        warn!("Bitmask without a bitflag: {}", self.original_name());

        // the name of the bitflag
        let name = self.as_ident();

        self.generate_doc(source, doc, out);

        quote::quote_each_token! {
            out

            #[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
            #[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            #[repr(transparent)]
            pub struct #name(u32);

            impl const Default for #name {
                fn default() -> Self {
                    Self(0)
                }
            }

            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    f.debug_tuple(stringify!(#name))
                        .field(&self.0)
                        .finish()
                }
            }
        }
    }

    /// Generates the documentation for an empty bitmask
    pub(super) fn generate_doc(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        out: &mut TokenStream,
    ) -> Option<()> {
        if let Some(mut doc) = doc.find(self.original_name()) {
            // parse the name section and write it out
            doc.name(source, self, out);

            // parse the c spec section and write it out
            doc.specification(source, self, out);

            // parse the related elements and write them out
            doc.related(source, out);

            // adds the copyright of the Vulkan docs
            doc.copyright(out);

            Some(())
        } else {
            warn!("No documentation for {}", self.original_name());

            // add the default no doc comment
            Documentation::no_doc(out);

            None
        }
    }
}
