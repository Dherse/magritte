use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use heck::ToUpperCamelCase;
use serde::{Deserialize, Serialize};

use crate::{Field, FunctionPointer, FunctionPointerArgument, Origin, Queryable, Source, SymbolName, SymbolTable, Ty};

/// A function defined in Vulkan
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function<'a> {
    /// The original name of the alias
    pub original_name: Cow<'a, str>,

    /// A rename
    pub rename: Option<Cow<'a, str>>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The return type of this function
    pub return_type: Ty<'a>,

    /// The list of success codes of this function
    pub success_codes: Vec<Cow<'a, str>>,

    /// The list of error codes of this function
    pub error_codes: Vec<Cow<'a, str>>,

    /// The arguments of this function
    #[serde(borrow = "'a")]
    pub arguments: SymbolTable<'a, FunctionArgument<'a>>,

    /// The aliases of this function
    pub aliases: SymbolTable<'a, Cow<'a, str>>,
}

impl Function<'static> {
    /// Creates a new function from its fields
    pub fn new(
        original_name: String,
        mut name: String,
        origin: Origin<'static>,
        return_type: Ty<'static>,
        success_codes: Vec<Cow<'static, str>>,
        error_codes: Vec<Cow<'static, str>>,
        arguments: SymbolTable<'static, FunctionArgument<'static>>,
    ) -> Self {
        if name == "type" {
            name = "type_".to_string();
        }

        Self {
            original_name: Cow::Owned(original_name),
            name,
            rename: None,
            origin,
            return_type,
            success_codes,
            error_codes,
            arguments,
            aliases: SymbolTable::default(),
        }
    }

    /// Creates a new function from its fields with a default origin
    pub fn new_no_origin(
        original_name: String,
        name: String,
        return_type: Ty<'static>,
        success_codes: Vec<Cow<'static, str>>,
        error_codes: Vec<Cow<'static, str>>,
        arguments: SymbolTable<'static, FunctionArgument<'static>>,
    ) -> Self {
        Self::new(
            original_name,
            name,
            Origin::Unknown,
            return_type,
            success_codes,
            error_codes,
            arguments,
        )
    }
}

impl<'a> Function<'a> {
    /// Get a reference to the function's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the function's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[cfg(feature = "codegen")]
    pub fn as_ident(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(self.name(), proc_macro2::Span::call_site())
    }

    #[cfg(feature = "codegen")]
    pub fn as_alias(&self) -> Option<proc_macro2::TokenStream> {
        let original_name = self.original_name();
        (self.name() != self.original_name()).then(|| {
            quote::quote! {
                #[doc(alias = #original_name)]
            }
        })
    }

    /// Get a reference to the function's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the function's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

        self.origin = origin;
    }

    /// Get a reference to the function's return type.
    pub fn return_type(&self) -> Option<&Ty<'a>> {
        if self.return_type.is_void() {
            None
        } else {
            Some(&self.return_type)
        }
    }

    /// Get a reference to the function's success codes.
    pub fn success_codes(&self) -> &[Cow<'a, str>] {
        &self.success_codes
    }

    /// Get a reference to the function's error codes.
    pub fn error_codes(&self) -> &[Cow<'a, str>] {
        &self.error_codes
    }

    /// Get a reference to the function's arguments.
    pub fn arguments(&self) -> &SymbolTable<'a, FunctionArgument<'a>> {
        &self.arguments
    }

    /// Get a mutable reference to the function's arguments.
    pub fn arguments_mut(&mut self) -> &mut SymbolTable<'a, FunctionArgument<'a>> {
        &mut self.arguments
    }

    /// Get a reference to the function's aliases.
    pub fn aliases(&self) -> &SymbolTable<'a, Cow<'a, str>> {
        &self.aliases
    }

    /// Get a mutable reference to the function's aliases.
    pub fn aliases_mut(&mut self) -> &mut SymbolTable<'a, Cow<'a, str>> {
        &mut self.aliases
    }

    /// Turns this function into an equivalent function pointer name
    pub fn as_fn_pointer_name(&self) -> String {
        format!("FN{}", self.name().to_upper_camel_case())
    }

    /// Turns this function into an equivalent function pointer type
    pub fn as_function_pointer(&self) -> FunctionPointer<'a> {
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
}

impl<'a> SymbolName<'a> for Function<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.rename.clone().unwrap_or_else(|| self.original_name.clone())
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for Function<'a> {
    fn find<'b>(&'b self, _: &'b Source<'a>, name: &str) -> Option<&'b str> {
        self.arguments.get_by_either(name).map(FunctionArgument::name)
    }
}

/// An argument of a function or command
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionArgument<'a> {
    /// The original name of the alias
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The name of the field, if any, that indicated the length of this field
    pub len: Option<Cow<'a, str>>,

    /// Is the parameter not always valid (i.e if `true`, requires validation)
    pub no_auto_validity: bool,

    /// Whether the argument is optional
    pub optionality: Optionality,

    /// The type of the argument
    pub ty: Ty<'a>,

    /// Whether the argument (or one of its members) must be
    /// externally synchronized.
    #[serde(borrow = "'a")]
    pub externally_synced: ExternallySynced<'a>,
}

impl<'a> FunctionArgument<'a> {
    pub fn as_static(&self) -> FunctionArgument<'static> {
        FunctionArgument {
            original_name: Cow::Owned(self.original_name.to_string()),
            name: self.name.clone(),
            len: self.len.as_ref().map(|len| Cow::Owned(len.to_string())),
            no_auto_validity: self.no_auto_validity,
            optionality: self.optionality,
            ty: self.ty.as_static(),
            externally_synced: self.externally_synced.as_static(),
        }
    }
    /// Get a reference to the argument's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the argument's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    #[cfg(feature = "codegen")]
    pub fn as_ident(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(self.name(), proc_macro2::Span::call_site())
    }

    #[cfg(feature = "codegen")]
    pub fn as_alias(&self) -> Option<proc_macro2::TokenStream> {
        let original_name = self.original_name();
        (self.name() != self.original_name()).then(|| {
            quote::quote! {
                #[doc(alias = #original_name)]
            }
        })
    }

    pub fn as_field(&self) -> Field<'a> {
        Field {
            original_name: self.original_name.clone(),
            name: self.name.clone(),
            ty: self.ty.clone(),
            selector: None,
            selection: None,
            optional: Optionality::No,
            externally_synchronized: ExternallySynced::No,
            must_be_valid: true,
            value: None,
        }
    }

    /// Gets a reference to the argument's length.
    pub fn len(&self) -> Option<&str> {
        self.len.as_deref()
    }

    /// Get a reference to the function argument's optionality.
    pub fn optionality(&self) -> Optionality {
        self.optionality
    }

    /// Get a reference to the function argument's ty.
    pub fn ty(&self) -> &Ty<'a> {
        &self.ty
    }

    /// Get a mutable reference to the function argument's ty.
    pub fn ty_mut(&mut self) -> &mut Ty<'a> {
        &mut self.ty
    }

    /// Get a reference to the function argument's externally synced.
    pub fn externally_synced(&self) -> &ExternallySynced<'a> {
        &self.externally_synced
    }
}

impl<'a> SymbolName<'a> for FunctionArgument<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

/// A command executed as part of a command buffer
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command<'a> {
    /// The original name of the alias
    #[serde(borrow = "'a")]
    pub function: Function<'a>,

    /// Whether the command should be part of a renderpass
    pub renderpass: RenderpassFlags,

    /// Whether the command can be used in a primary/secondary buffer
    pub buffer_level: BufferLevelFlags,

    /// What type of queue it needs to be submitted into
    pub queues: QueueFlags,
}

impl<'a> Command<'a> {
    /// Creates a new command from a base function and its properties.
    pub const fn new(
        function: Function<'a>,
        renderpass: RenderpassFlags,
        buffer_level: BufferLevelFlags,
        queues: QueueFlags,
    ) -> Self {
        Self {
            function,
            renderpass,
            buffer_level,
            queues,
        }
    }

    /// Get a reference to the command's renderpass.
    pub fn renderpass(&self) -> RenderpassFlags {
        self.renderpass
    }

    /// Get a reference to the command's buffer level.
    pub fn buffer_level(&self) -> BufferLevelFlags {
        self.buffer_level
    }

    /// Get a reference to the command's queues.
    pub fn queues(&self) -> QueueFlags {
        self.queues
    }
}

impl<'a> Deref for Command<'a> {
    type Target = Function<'a>;

    fn deref(&self) -> &Self::Target {
        &self.function
    }
}

impl<'a> DerefMut for Command<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.function
    }
}

impl<'a> SymbolName<'a> for Command<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.function.name().to_owned()
    }
}

/// A command alias.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandAlias<'a> {
    /// The original name of the alias
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The original command.
    pub of: Cow<'a, str>,
}

impl CommandAlias<'static> {
    /// Creates a new alias from its original command
    #[inline]
    pub const fn new(original_name: String, name: String, of: String, origin: Origin<'static>) -> Self {
        Self {
            original_name: Cow::Owned(original_name),
            name,
            of: Cow::Owned(of),
            origin,
        }
    }

    /// Creates a new alias from its original command with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: String, name: String, of: String) -> Self {
        Self::new(original_name, name, of, Origin::Unknown)
    }
}

impl<'a> CommandAlias<'a> {
    /// Get a reference to the alias's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the alias's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the alias's function pointer-like name.
    pub fn as_fn_pointer_name(&self) -> String {
        format!("FN{}", self.name().to_upper_camel_case())
    }

    #[cfg(feature = "codegen")]
    pub fn as_ident(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(self.name(), proc_macro2::Span::call_site())
    }

    #[cfg(feature = "codegen")]
    pub fn as_fn_pointer_ident(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&self.as_fn_pointer_name(), proc_macro2::Span::call_site())
    }

    #[cfg(feature = "codegen")]
    pub fn as_alias(&self) -> Option<proc_macro2::TokenStream> {
        let original_name = self.original_name();
        (self.name() != self.original_name()).then(|| {
            quote::quote! {
                #[doc(alias = #original_name)]
            }
        })
    }

    /// Get a reference to the alias's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the alias's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        // Gate that ensures that we don't "downgrade" origins
        if self.origin.is_vulkan() {
            return;
        }

        self.origin = origin;
    }

    /// Get a reference to the alias's of.
    pub fn of(&self) -> &str {
        self.of.as_ref()
    }
}

impl<'a> SymbolName<'a> for CommandAlias<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    /// In which context can a command be used
    pub struct RenderpassFlags: u32 {
        /// Inside of a render pass
        const INSIDE = 1;

        /// Outside of a render pass
        const OUTSIDE = 2;

        /// Either inside or outside of a render pass
        const BOTH = Self::INSIDE.bits() | Self::OUTSIDE.bits();
    }
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    /// What buffer level can a command be used on
    pub struct BufferLevelFlags: u32 {
        /// From a primary level buffer only
        const PRIMARY = 1;

        /// From a secondary level buffer only
        const SECONDARY = 2;

        /// From either buffer levels
        const BOTH = Self::PRIMARY.bits() | Self::SECONDARY.bits();
    }
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    /// What queue can this command be submitted into
    pub struct QueueFlags: u32 {
        /// A graphics queue
        const GRAPHICS = 1;

        /// A compute queue
        const COMPUTE = 2;

        /// A transfer queue
        const TRANSFER = 4;

        /// A video decode queue
        const DECODE = 8;

        /// A video encode queue
        const ENCODE = 16;

        /// Any "regular" queue (i.e graphics, compute or transfer)
        const ALL = Self::GRAPHICS.bits() | Self::COMPUTE.bits() | Self::TRANSFER.bits();
    }
}

/// Is the argument optional?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Optionality {
    /// The field is optional
    Yes,

    /// The field is **not** optional
    No,

    /// The field can be optional depending on circumstances
    Sometimes,
}

impl Optionality {
    /// Returns true if the argument is optional
    #[inline]
    pub fn is_optional(self) -> bool {
        self != Optionality::No
    }

    /// Returns true if the argument is not optional
    pub fn is_required(self) -> bool {
        self == Optionality::No
    }
}

/// The external synchronization requierments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExternallySynced<'a> {
    /// The value must be externally synchronized
    Yes,

    /// The value does not need to be externally synchronized
    No,

    /// Multiple values need to be externally synchronized
    Multiple(Vec<ExternallySynced<'a>>),

    /// A variable (inside of the argument) must be externally synchronized
    Variable(Cow<'a, str>),

    /// All the values in an array must be synchronized
    All(Box<ExternallySynced<'a>>),

    /// A value obtained by resolving a pointer must be externally synchronized
    Resolve(Box<ExternallySynced<'a>>, Box<ExternallySynced<'a>>),

    /// All of the values obtained by resolving a pointer must be externally synchronized
    ForEach(Box<ExternallySynced<'a>>, Box<ExternallySynced<'a>>),
}

impl<'a> ExternallySynced<'a> {
    /// The type does not require any external synchronization
    pub fn is_no(&self) -> bool {
        matches!(self, Self::No)
    }

    /// The type does require any external synchronization
    pub fn is_yes(&self) -> bool {
        !self.is_no()
    }

    pub fn as_static(&self) -> ExternallySynced<'static> {
        match self {
            Self::Yes => ExternallySynced::Yes,
            Self::No => ExternallySynced::No,
            Self::Multiple(vals) => ExternallySynced::Multiple(vals.into_iter().map(Self::as_static).collect()),
            Self::Variable(vals) => ExternallySynced::Variable(Cow::Owned(vals.to_string())),
            Self::All(vals) => ExternallySynced::All(Box::new(vals.as_static())),
            Self::Resolve(a, b) => ExternallySynced::Resolve(Box::new(a.as_static()), Box::new(b.as_static())),
            Self::ForEach(a, b) => ExternallySynced::ForEach(Box::new(a.as_static()), Box::new(b.as_static())),
        }
    }
}
