use magritte_build::origin::cond_of;
use magritte_types::{Alias, Bit, Bitflag, Bitmask, Source};
use proc_macro2::TokenStream;
use quote::{quote, quote_each_token};

use crate::{
    common::CommonOriginVisitor,
    native::bits::{alias_of, bits_of},
};

impl<'b, 'parent> CommonOriginVisitor<'b, 'parent> {
    pub fn gen_for_bitmask(
        &mut self,
        source: &Source<'_>,
        bitmask: &Bitmask<'_>,
        bitflag: &Bitflag<'_>,
        ty: &TokenStream,
    ) {
        let name = bitmask.as_ident();
        let alias = bitmask.as_alias();
        let doc = self.doc_of(&self.doc_dir_path, &self.origin, bitflag.original_name());

        self.imports.push_origin(source, bitflag.origin(), bitflag.name());

        let bit_cond = cond_of(source, bitmask.origin(), bitflag.origin());
        let bit_name = bitflag.as_ident();

        let conds = bitflag
            .bits()
            .iter()
            .filter(|bit| !bit.origin().is_disabled())
            .map(|a| cond_of(source, bitmask.origin(), a.origin()))
            .chain(
                bitflag
                    .aliases()
                    .iter()
                    .filter(|bit| !bit.origin().is_disabled())
                    .map(|a| cond_of(source, bitmask.origin(), a.origin())),
            )
            .collect::<Vec<_>>();

        let bit_idents = bitflag
            .bits()
            .iter()
            .filter(|bit| !bit.origin().is_disabled())
            .map(Bit::as_ident)
            .chain(
                bitflag
                    .aliases()
                    .iter()
                    .filter(|bit| !bit.origin().is_disabled())
                    .map(Alias::as_ident),
            )
            .collect::<Vec<_>>();

        let bits = bitflag
            .bits()
            .iter()
            .map(|bit| bits_of(self.parent, source, &self.doc_dir_path, bitflag, bit))
            .collect::<Vec<_>>();

        let first = if bit_idents.is_empty() {
            None
        } else {
            Some(quote! { let mut first = true; })
        };

        let aliases = bitflag
            .aliases()
            .iter()
            .map(|alias| alias_of(self.parent, source, &self.doc_dir_path, bitflag, alias))
            .collect::<Vec<_>>();

        let mut out = &mut self.tt;
        quote_each_token! {
            out

            #doc
            #alias
            #[repr(transparent)]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            pub struct #name(#ty);

            impl #name {
                #(#bits)*
                #(#aliases)*

                #[doc = "Default empty flags"]
                #[inline]
                pub const fn empty() -> Self {
                    Self(0)
                }

                #[doc = "Returns a value with all of the flags enabled"]
                #[inline]
                #[allow(unused_mut)]
                pub const fn all() -> Self {
                    let mut all = Self::empty();

                    #(
                        #conds
                        {
                            all |= Self::#bit_idents;
                        }
                    )*

                    all
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

            impl std::ops::BitOr for #name {
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

            impl std::ops::BitXor  for #name {
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

            impl std::ops::BitAnd  for #name {
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

            impl std::ops::Sub for #name {
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

            impl std::ops::Not for #name {
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

            impl FromIterator<#name> for #name {
                fn from_iter<T: IntoIterator<Item = #name>>(iterator: T) -> #name {
                    let mut out = Self::empty();
                    <Self as Extend<#name>>::extend(&mut out, iterator);
                    out
                }
            }

            impl Default for #name {
                fn default() -> Self {
                    Self::empty()
                }
            }

            #bit_cond
            impl From<#bit_name> for #name {
                fn from(bit: #bit_name) -> Self {
                    unsafe {
                        Self::from_bits_unchecked(bit.bits())
                    }
                }
            }

            #bit_cond
            impl Extend<#bit_name> for #name {
                fn extend<T: IntoIterator<Item = #bit_name>>(&mut self, iterator: T) {
                    for i in iterator {
                        Self::insert(self, Self::from(i));
                    }
                }
            }

            #bit_cond
            impl FromIterator<#bit_name> for #name {
                fn from_iter<T: IntoIterator<Item = #bit_name>>(iterator: T) -> #name {
                    let mut out = Self::empty();
                    <Self as Extend<#bit_name>>::extend(&mut out, iterator);
                    out
                }
            }

            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    struct Flags(#name);
                    impl std::fmt::Debug for Flags {
                        #[allow(unused_assignments, unused_mut, unused_variables)]
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                            if self.0 == #name::empty() {
                                f.write_str("empty")?;
                            } else {
                                #first
                                #(
                                    #conds
                                    if self.0.contains(#name::#bit_idents) {
                                        if !first {
                                            f.write_str(" | ")?;
                                        }

                                        first = false;
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

            #[cfg(feature = "native")]
            unsafe impl crate::conv::IntoLowLevel for #name {
                type LowLevel = Self;

                unsafe fn into_low_level(&self, context: &std::sync::Arc<crate::context::Context>, bump: &bumpalo::Bump) -> Self::LowLevel {
                    *self
                }
            }

            #[cfg(feature = "native")]
            unsafe impl crate::conv::FromLowLevel for #name {
                unsafe fn from_low_level(context: &std::sync::Arc<crate::context::Context>, value: <Self as crate::conv::IntoLowLevel>::LowLevel) -> Self {
                    value
                }
            }
        };
    }
}
