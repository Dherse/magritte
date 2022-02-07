use std::{borrow::Cow, hint::unreachable_unchecked};

use ahash::AHashSet;
use convert_case::{Case, Casing};
use tracing::{info, span, Level};
use vk_parse::{TypeMemberDefinition, TypeMemberMarkup};

use crate::{
    origin::Origin,
    symbols::{SymbolName, SymbolTable},
    ty::{Native, Ty},
};

use super::commands::ExternallySynced;

/// A Vulkan struct
#[derive(Debug, Clone, PartialEq)]
pub struct Struct<'a> {
    /// The name of the structure
    pub original_name: Cow<'a, str>,

    /// The rustified name of the structure
    pub name: String,

    /// The names of the types this structure extends.
    pub extends: AHashSet<Cow<'a, str>>,

    /// Is this type always returned (never constructed)
    pub always_returned: bool,

    /// The fields that compose this struct
    pub fields: SymbolTable<'a, Field<'a>>,

    /// The origin (extension or Vulkan version)
    pub origin: Origin<'a>,
}

impl<'a> Struct<'a> {
    /// Creates a new struct from its fields
    #[inline]
    pub fn new(
        original_name: &'a str,
        name: String,
        extends: AHashSet<Cow<'a, str>>,
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
        }
    }

    /// Creates a new struct from its fields with a default origin
    #[inline]
    pub fn new_no_origin(
        original_name: &'a str,
        name: String,
        extends: AHashSet<Cow<'a, str>>,
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

    /// Get a reference to the struct's extends.
    pub fn extends(&self) -> &AHashSet<Cow<'a, str>> {
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
    pub fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the struct's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        self.origin = origin;
    }
}

impl<'a> SymbolName<'a> for Struct<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
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
                TypeMemberMarkup::Enum(_) => (),
                TypeMemberMarkup::Type(_) => (),
                TypeMemberMarkup::Comment(_) => (),
                _ => unreachable!("Unknown type markup: {:?}", item),
            }
        }

        let mut ty = Ty::new(
            &member.code,
            member
                .altlen
                .as_ref()
                .or(member.len.as_ref())
                .map(|s| s as &str)
                .unwrap_or(""),
        )
        .1
        .simplify();

        let original_name = name.expect("missing name");
        let mut name = original_name.to_case(Case::Snake);

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
}

impl<'a> SymbolName<'a> for Field<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Optionality {
    /// The field is optional
    Yes,

    /// The field is **not** optional
    No,

    /// The field can be optional depending on circumstances
    Sometimes,
}
