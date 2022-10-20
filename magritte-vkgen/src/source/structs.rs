use std::{borrow::Cow, hint::unreachable_unchecked};

use heck::ToSnakeCase;
use proc_macro2::{Ident, Span};
use tracing::{info, span, Level};
use vk_parse::{TypeMemberDefinition, TypeMemberMarkup};

use crate::{
    doc::Queryable,
    origin::Origin,
    symbols::{SymbolName, SymbolTable},
    ty::{Mutability, Native, Ty},
};

use super::{commands::ExternallySynced, Source};

/// A Vulkan struct
#[derive(Debug, Clone, PartialEq)]
pub struct Struct<'a> {
    /// The name of the structure
    pub original_name: Cow<'a, str>,

    /// The rustified name of the structure
    pub name: String,

    /// The names of the types this structure extends.
    pub extends: Vec<Cow<'a, str>>,

    /// Is this type always returned (never constructed)
    pub always_returned: bool,

    /// The fields that compose this struct
    pub fields: SymbolTable<'a, Field<'a>>,

    /// The origin (extension or Vulkan version)
    pub origin: Origin<'a>,

    /// Structures that extend this structure
    pub extended: Vec<Cow<'a, str>>,
}

impl<'a> Struct<'a> {
    /// Creates a new struct from its fields
    #[inline]
    pub fn new(
        original_name: &'a str,
        name: String,
        extends: Vec<Cow<'a, str>>,
        always_returned: bool,
        fields: SymbolTable<'a, Field<'a>>,
        origin: Origin<'a>,
    ) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            extends,
            always_returned,
            fields,
            origin,
            extended: Vec::new(),
        }
    }

    /// Creates a new struct from its fields with a default origin
    #[inline]
    pub fn new_no_origin(
        original_name: &'a str,
        name: String,
        extends: Vec<Cow<'a, str>>,
        always_returned: bool,
        fields: SymbolTable<'a, Field<'a>>,
    ) -> Self {
        Self::new(original_name, name, extends, always_returned, fields, Origin::Unknown)
    }

    /// Get a reference to the struct's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the struct's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
    }

    /// Get a reference to the struct's extends.
    pub fn extends(&self) -> &Vec<Cow<'a, str>> {
        &self.extends
    }

    /// Get a reference to the struct's always returned.
    pub fn always_returned(&self) -> bool {
        self.always_returned
    }

    /// Get a reference to the struct's fields.
    pub fn fields(&self) -> &SymbolTable<'a, Field<'a>> {
        &self.fields
    }

    /// Get a reference to the struct's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the struct's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

        self.origin = origin;
    }

    /// Checks if this structure needs a lifetime
    pub fn has_lifetime(&self, source: &Source<'a>) -> bool {
        self.fields.iter().any(|f| f.has_lifetime(source, true))
    }

    /// Checks if this structure is debug
    pub fn is_debug(&self, source: &Source<'a>) -> bool {
        self.fields.iter().all(|f| f.is_debug(source))
    }

    /// Checks if this structure is copy
    pub fn is_copy(&self, source: &Source<'a>) -> bool {
        self.fields.iter().all(|f| f.is_copy(source))
    }

    /// Checks if this field is partial_eq
    pub fn is_partial_eq(&self, source: &Source<'a>) -> bool {
        self.fields.iter().all(|f| f.is_partial_eq(source))
    }
    /// Checks if this structure is eq
    pub fn is_eq(&self, source: &Source<'a>) -> bool {
        self.fields.iter().all(|f| f.is_eq(source))
    }

    /// Checks if this structure is hash
    pub fn is_hash(&self, source: &Source<'a>) -> bool {
        self.fields.iter().all(|f| f.is_hash(source))
    }

    /// Checks whether the struct can be (de)serialized
    pub fn is_serde(&self, source: &Source<'a>) -> bool {
        self.fields.iter().all(|f| f.is_serde(source))
    }

    /// Gets a field by either its original name or its pretty name
    pub fn get_field(&self, name: &str) -> Option<&Field<'a>> {
        self.fields().get_by_either(name)
    }

    /// Does the structure have a pointer chain? If so, what is
    /// its mutability.
    pub fn has_p_next(&self) -> Option<Mutability> {
        self.fields().iter().find_map(|field| match field.ty() {
            Ty::Pointer(mut_, _) if field.original_name() == "pNext" => Some(*mut_),
            _ => None,
        })
    }

    /// Does the structure contain a pointer
    pub fn has_pointer(&self) -> bool {
        self.fields().iter().any(|field| match field.ty() {
            Ty::Pointer(_, _) | Ty::Slice(_, _, _) => true,
            _ => false,
        })
    }

    /// Does the structure only contain the `p_next` pointer and no other.
    pub fn has_only_p_next(&self) -> bool {
        self.has_p_next().is_some() && self.fields().iter().all(|field| match field.ty() {
            Ty::Pointer(_, _) if field.original_name() == "pNext" => true,
            Ty::Pointer(_, _) | Ty::Slice(_, _, _) => false,
            _ => true,
        })
    }

    /*/// Checks if this structure needs one or more generic type arguments
    pub fn has_generics(&self, source: &Source<'a>) -> bool {
        self.fields.iter().any(|f| f.has_generics(source))
    }*/

    /// Get a reference to the struct's extended.
    pub fn extended(&self) -> &[Cow<str>] {
        self.extended.as_ref()
    }

    /// Get a mutable reference to the struct's extended.
    pub fn extended_mut(&mut self) -> &mut Vec<Cow<'a, str>> {
        &mut self.extended
    }

    /// Adds to the list of structs that extend this struct
    pub fn add_extended(&mut self, extended: Cow<'a, str>) {
        self.extended_mut().push(extended);
    }
}

impl<'a> SymbolName<'a> for Struct<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for Struct<'a> {
    fn find<'b>(&'b self, _: &'b Source<'a>, name: &str) -> Option<&'b str> {
        self.fields().get_by_either(name).map(Field::name)
    }
}

/// A field member of a [`Struct`] or union.
#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a> {
    /// The name of the field.
    pub original_name: Cow<'a, str>,

    /// The rustified name of the field.
    pub name: String,

    /// The type name of this value.
    pub ty: Ty<'a>,

    /// Defines the field that selects which field of a union is filled/required.
    /// This is used in structs to know which union type is expected.
    pub selector: Option<Cow<'a, str>>,

    /// The name of the variant that selects this field.
    /// This is used for union to know which field to use.
    pub selection: Option<Cow<'a, str>>,

    /// Is the field optional in the struct.
    pub optional: Optionality,

    /// Is the field externally synchronized (i.e we take a mutable reference).
    pub externally_synchronized: ExternallySynced<'a>,

    /// The value must be valid (i.e the value is not **always** valid, there are conditions on it)
    pub must_be_valid: bool,

    /// The base value (values?) of this field (used for the structure type)
    pub value: Option<Cow<'a, str>>,
}

impl<'a> Field<'a> {
    /// Constructs a new field from a Vulkan type member
    pub fn from_member(member: &'a TypeMemberDefinition) -> Self {
        let mut name = None;

        for item in &member.markup {
            match item {
                TypeMemberMarkup::Name(value) => name = Some(Cow::Borrowed(value as &str)),
                TypeMemberMarkup::Enum(_) | TypeMemberMarkup::Type(_) | TypeMemberMarkup::Comment(_) => (),
                _ => unreachable!("Unknown type markup: {:?}", item),
            }
        }

        let mut ty = Ty::new(
            &member.code,
            member.altlen.as_ref().or(member.len.as_ref()).map_or("", |s| s as &str),
        )
        .1
        .simplify();

        let original_name = name.expect("missing name");
        let mut name = original_name.to_snake_case();

        let span = span!(Level::INFO, "field", ?original_name, ?name, ?ty);
        let _guard = span.enter();

        if name == "type" {
            name = "type_".to_string();
        } else if name == "p_next" && matches!(ty, Ty::Pointer(_, box Ty::Native(Native::Void))) {
            let mutability = match &ty {
                Ty::Pointer(mut_, _) => mut_,
                _ => unsafe { unreachable_unchecked() },
            };

            ty = Ty::Pointer(
                *mutability,
                if mutability.is_mut() {
                    box Ty::Named(Cow::Borrowed("VkBaseOutStructure"))
                } else {
                    box Ty::Named(Cow::Borrowed("VkBaseInStructure"))
                },
            );
        } else if name.starts_with("p_") {
            name = name.trim_start_matches("p_").to_string();
        }

        info!("processed field");

        Self {
            original_name,
            name,
            ty,
            selector: member.selector.as_ref().map(|s| s as &str).map(Cow::Borrowed),
            selection: member.selection.as_ref().map(|s| s as &str).map(Cow::Borrowed),
            optional: match member
                .optional
                .as_ref()
                .map_or((false, false), |s| (s.contains("true"), s.contains("false")))
            {
                (true, false) => Optionality::Yes,
                (true, true) => Optionality::Sometimes,
                (false, _) => Optionality::No,
            },
            externally_synchronized: ExternallySynced::new(member.externsync.as_ref().map(|s| s as &str)),
            must_be_valid: member
                .noautovalidity
                .as_ref()
                .map_or(false, |s| s.eq_ignore_ascii_case("true")),
            value: member.values.as_ref().map(|s| s as &str).map(Cow::Borrowed),
        }
    }

    /// Get a reference to the field's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the field's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Creates an identifier from the name
    pub fn as_ident(&self) -> Ident {
        Ident::new(self.name(), Span::call_site())
    }

    /// Get a reference to the field's ty.
    pub fn ty(&self) -> &Ty<'a> {
        &self.ty
    }

    /// Get a reference to the field's selector.
    pub fn selector(&self) -> Option<&Cow<str>> {
        self.selector.as_ref()
    }

    /// Get a reference to the field's selection.
    pub fn selection(&self) -> Option<&Cow<str>> {
        self.selection.as_ref()
    }

    /// Get a reference to the field's optional.
    pub fn optional(&self) -> Optionality {
        self.optional
    }

    /// Get a reference to the field's externally synchronized.
    pub fn externally_synchronized(&self) -> &ExternallySynced<'a> {
        &self.externally_synchronized
    }

    /// Get a reference to the field's must be valid.
    pub fn must_be_valid(&self) -> bool {
        self.must_be_valid
    }

    /// Get a reference to the field's value.
    pub fn value(&self) -> Option<&str> {
        self.value.as_ref().map(|s| s as &str)
    }

    /// Does this field has a lifetime
    pub fn has_lifetime(&self, source: &Source<'a>, pointer_has_lifetime: bool) -> bool {
        self.ty().has_lifetime(source, pointer_has_lifetime)
    }

    /// Checks if this field is debug
    pub fn is_debug(&self, source: &Source<'a>) -> bool {
        self.ty().is_debug(source)
    }

    /// Checks if this field is copy
    pub fn is_copy(&self, source: &Source<'a>) -> bool {
        self.ty().is_copy(source)
    }

    /// Checks if this field is copy
    pub fn is_partial_eq(&self, source: &Source<'a>) -> bool {
        self.ty().is_partial_eq(source)
    }

    /// Checks if this field is copy
    pub fn is_eq(&self, source: &Source<'a>) -> bool {
        self.ty().is_eq(source)
    }

    /// Checks if this field is copy
    pub fn is_hash(&self, source: &Source<'a>) -> bool {
        self.ty().is_hash(source)
    }

    /// Checks whether the field can be (de)serialized
    pub fn is_serde(&self, source: &Source<'a>) -> bool {
        self.ty().is_serde(source)
    }

    /*/// Does this field have a generic type argument
    pub fn has_generics(&self, source: &Source<'a>) -> bool {
        self.ty().has_generics(source)
    }*/
}

impl<'a> SymbolName<'a> for Field<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

/// Is the argument optional?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Optionality {
    /// The field is optional
    Yes,

    /// The field is **not** optional
    No,

    /// The field can be optional depending on circumstances
    Sometimes,
}
