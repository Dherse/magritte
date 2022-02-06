mod sync;

use convert_case::{Case, Casing};
pub use sync::ExternallySynced;
use tracing::{info, span, Level};

use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use smallvec::SmallVec;

use crate::{
    origin::Origin,
    symbols::{SymbolName, SymbolTable},
    ty::Ty,
};

use super::structs::Optionality;

/// A function defined in Vulkan
#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    /// The original name of the alias
    pub original_name: Cow<'a, str>,

    /// The cleaned up and "rust-ified" name
    pub name: String,

    /// The origin (extension, core, vulkan version)
    pub origin: Origin<'a>,

    /// The return type of this command
    pub return_type: Ty<'a>,

    /// The list of success codes of this command
    pub success_codes: SmallVec<[Cow<'a, str>; 1]>,

    /// The list of error codes of this command
    pub error_codes: SmallVec<[Cow<'a, str>; 1]>,

    /// The arguments of this command
    pub arguments: SymbolTable<'a, FunctionArgument<'a>>,
}

impl<'a> Function<'a> {
    /// Creates a new function from its fields
    pub fn new(
        original_name: &'a str,
        name: String,
        origin: Origin<'a>,
        return_type: Ty<'a>,
        success_codes: SmallVec<[Cow<'a, str>; 1]>,
        error_codes: SmallVec<[Cow<'a, str>; 1]>,
        arguments: SymbolTable<'a, FunctionArgument<'a>>,
    ) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            origin,
            return_type,
            success_codes,
            error_codes,
            arguments,
        }
    }

    /// Creates a new function from its fields with a default origin
    pub fn new_no_origin(
        original_name: &'a str,
        name: String,
        return_type: Ty<'a>,
        success_codes: SmallVec<[Cow<'a, str>; 1]>,
        error_codes: SmallVec<[Cow<'a, str>; 1]>,
        arguments: SymbolTable<'a, FunctionArgument<'a>>,
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

    /// Get a reference to the function's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the function's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the function's origin.
    pub fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the function's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
        self.origin = origin;
    }

    /// Get a reference to the function's return type.
    pub fn return_type(&self) -> &Ty<'a> {
        &self.return_type
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
}

impl<'a> SymbolName<'a> for Function<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.original_name.clone()
    }
}

/// An argument of a function or command
#[derive(Debug, Clone, PartialEq)]
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
    pub externally_synced: ExternallySynced<'a>,
}

impl<'a> FunctionArgument<'a> {
    /// Creates a new function argument from its name and type
    #[inline]
    pub fn new(param: &'a vk_parse::CommandParam, ty: Ty<'a>) -> Self {
        let original_name = &param.definition.name as &str;

        let span = span!(Level::INFO, "argument", ?original_name, ?ty);
        let _guard = span.enter();

        let name = original_name.to_case(Case::Snake);
        info!(?name, "generated rustified name");

        let len = param
            .altlen
            .as_ref()
            .or(param.len.as_ref())
            .map(|s| s as &str)
            .map(Cow::Borrowed);
        info!(?len, "parsed len field");

        let optionality = match param
            .optional
            .as_ref()
            .map_or((false, false), |s| (s.contains("true"), s.contains("false")))
        {
            (true, false) => Optionality::Yes,
            (true, true) => Optionality::Sometimes,
            (false, _) => Optionality::No,
        };
        info!(?optionality, "parsed optionality");

        let no_auto_validity = param.noautovalidity.as_ref().map(|v| v == "true").unwrap_or(false);
        info!(?no_auto_validity, "parsed auto validity");

        let externally_synced = ExternallySynced::new(param.externsync.as_ref().map(|s| s as &str));

        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            len,
            optionality,
            no_auto_validity,
            externally_synced,
            ty,
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

    /// Gets a reference to the argument's length.
    pub fn len(&self) -> Option<&str> {
        self.len.as_ref().map(|s| s as &str)
    }

    /// Get a reference to the function argument's optionality.
    pub fn optionality(&self) -> Optionality {
        self.optionality
    }

    /// Get a reference to the function argument's ty.
    pub fn ty(&self) -> &Ty<'a> {
        &self.ty
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
}

/// A command executed as part of a command buffer
#[derive(Debug, Clone, PartialEq)]
pub struct Command<'a> {
    /// The original name of the alias
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
}

/// A command alias.
#[derive(Debug, Clone, PartialEq)]
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

impl<'a> CommandAlias<'a> {
    /// Creates a new alias from its original command
    #[inline]
    pub const fn new(original_name: &'a str, name: String, of: &'a str, origin: Origin<'a>) -> Self {
        Self {
            original_name: Cow::Borrowed(original_name),
            name,
            of: Cow::Borrowed(of),
            origin,
        }
    }

    /// Creates a new alias from its original command with a default origin of unknown
    #[inline]
    pub const fn new_no_origin(original_name: &'a str, name: String, of: &'a str) -> Self {
        Self::new(original_name, name, of, Origin::Unknown)
    }

    /// Get a reference to the alias's original name.
    pub fn original_name(&self) -> &str {
        self.original_name.as_ref()
    }

    /// Get a reference to the alias's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the alias's origin.
    pub fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Set the alias's origin.
    pub fn set_origin(&mut self, origin: Origin<'a>) {
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
}

bitflags::bitflags! {
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
