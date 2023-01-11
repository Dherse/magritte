use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::Optionality;

use super::{ExternallySynced, Mutability, Origin, Queryable, Source, SymbolName, SymbolTable, Ty};

/// A Vulkan struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(borrow = "'a")]
    pub fields: SymbolTable<'a, Field<'a>>,

    /// The origin (extension or Vulkan version)
    pub origin: Origin<'a>,

    /// Structures that extend this structure
    pub extended: Vec<Cow<'a, str>>,
}

impl Struct<'static> {
    /// Creates a new struct from its fields
    #[inline]
    pub fn new(
        original_name: String,
        name: String,
        extends: Vec<Cow<'static, str>>,
        always_returned: bool,
        fields: SymbolTable<'static, Field<'static>>,
        origin: Origin<'static>,
    ) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
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
        original_name: String,
        name: String,
        extends: Vec<Cow<'static, str>>,
        always_returned: bool,
        fields: SymbolTable<'static, Field<'static>>,
    ) -> Self {
        Self::new(original_name, name, extends, always_returned, fields, Origin::Unknown)
    }
}

impl<'a> Struct<'a> {
    /// Get a reference to the struct's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the struct's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
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

    /// Checks if this struct contains opaque fields and is therefore
    /// incompatible with WASM (at least wihout manual translation)
    pub fn has_opaque(&self, source: &Source<'a>) -> bool {
        self.fields().iter().any(|f| f.is_opaque(source))
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
        self.has_p_next().is_some()
            && self.fields().iter().all(|field| match field.ty() {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(borrow = "'a")]
    pub externally_synchronized: ExternallySynced<'a>,

    /// The value must be valid (i.e the value is not **always** valid, there are conditions on it)
    pub must_be_valid: bool,

    /// The base value (values?) of this field (used for the structure type)
    pub value: Option<Cow<'a, str>>,
}

impl<'a> Field<'a> {
    /// Get a reference to the field's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the field's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Checks whether this field is an element of a pointer chain
    pub fn is_p_next(&self) -> bool {
        self.original_name() == "pNext"
    }

    /// Checks whether this field is the structure type field
    pub fn is_s_type(&self) -> bool {
        self.original_name() == "sType"
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

    pub fn is_opaque(&self, source: &Source<'a>) -> bool {
        // List of allowed "opaque" types:
        // - pNext: the pointer chain, not actually opaque
        // - pData: refers to byte arrays which are *mut c_void
        // - pTag: refers to byte arrays which are *mut c_void
        // - hostAddress: refers to an address in memory for VkDeviceOrHostAddressKHR, will need manual impl
        // - pInitialData: refers to byte arrays which are *mut c_void
        // - pShaderGroupCaptureReplayHandle: used for Vulkan replay, will not be available in non-native
        //   mode
        // - user data, will need special handling in WASM mode
        if self.original_name() == "pNext"
            || self.original_name() == "pData"
            || self.original_name() == "pTag"
            || self.original_name() == "hostAddress"
            || self.original_name() == "pInitialData"
            || self.original_name() == "pShaderGroupCaptureReplayHandle"
            || self.original_name() == "pUserData"
        {
            false
        } else {
            self.ty().is_opaque(source)
        }
    }
}

impl<'a> SymbolName<'a> for Field<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}
