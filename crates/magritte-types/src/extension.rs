use std::borrow::Cow;

use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};

use crate::{Origin, Queryable, SymbolName};

use super::Source;

/// A Vulkan extension
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Extension<'a> {
    /// The name of the extension
    pub name: Cow<'a, str>,

    /// Is the extension disabled?
    pub disabled: bool,

    /// The ID (unique number) of the extension
    pub id: i64,

    /// The type of the extension
    pub ty: ExtensionType,

    /// The minimum core (Vulkan) version
    pub min_core: Origin<'static>,

    /// Other required extensions
    pub required: Vec<Cow<'a, str>>,

    /// The deprecation status
    pub deprecation_status: DeprecationStatus,

    /// The origin associated with this extension
    pub origin: Origin<'a>,
}

impl Extension<'static> {
    /// Creates a new extension from its properties
    pub fn new(
        name: Cow<'static, str>,
        disabled: bool,
        id: i64,
        ty: ExtensionType,
        min_core: Origin<'static>,
        required: Vec<Cow<'static, str>>,
        deprecation_status: DeprecationStatus,
    ) -> Self {
        let origin = Origin::Extension(name.clone(), id, disabled);

        Self {
            name,
            disabled,
            id,
            ty,
            min_core,
            required,
            deprecation_status,
            origin,
        }
    }
}

impl<'a> Extension<'a> {
    /// Get a reference to the extension's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Trims and makes the name into an rustified name.
    pub fn simple_name(&self) -> String {
        self.name().trim_start_matches("VK_").to_snake_case()
    }

    #[cfg(feature = "codegen")]
    pub fn as_ident(&self) -> proc_macro2::Ident {
        proc_macro2::Ident::new(&self.simple_name(), proc_macro2::Span::call_site())
    }

    /// Get a reference to the extension's disabled.
    pub fn disabled(&self) -> bool {
        self.disabled
    }

    /// Get a reference to the extension's id.
    pub fn id(&self) -> i64 {
        self.id
    }

    /// Get a reference to the extension's ty.
    pub fn ty(&self) -> ExtensionType {
        self.ty
    }

    /// Get a reference to the extension's min core.
    pub fn min_core(&self) -> &Origin<'static> {
        &self.min_core
    }

    /// Get a reference to the extension's required extensions.
    pub fn required(&self) -> &[Cow<'a, str>] {
        &self.required
    }

    /// Get a reference to the extension's deprecation status.
    pub fn deprecation_status(&self) -> &DeprecationStatus {
        &self.deprecation_status
    }

    /// Get a reference to the extension's origin.
    #[inline]
    pub const fn origin(&self) -> &Origin<'a> {
        &self.origin
    }

    /// Is another extension required by this extension
    pub fn requires(&self, source: &Source<'a>, other: &str) -> bool {
        for required in self.required() {
            if required.eq(other) {
                return true;
            } else if source
                .extensions
                .get_by_either(required)
                .expect("unknown extension")
                .requires(source, other)
            {
                return true;
            }
        }

        return false;
    }
}

impl<'a> SymbolName<'a> for Extension<'a> {
    fn name(&self) -> Cow<'a, str> {
        self.name.clone()
    }

    fn pretty_name(&self) -> String {
        self.name().to_owned()
    }
}

impl<'a> Queryable<'a> for Extension<'a> {
    fn find<'b>(&'b self, _source: &'b Source<'a>, _name: &str) -> Option<&'b str> {
        None
    }
}

/// The type of extension it is, whether it is device level or instance level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExtensionType {
    /// Is a device level extension
    Device,

    /// Is an instance level extension
    Instance,
}

impl From<&String> for ExtensionType {
    fn from(s: &String) -> Self {
        match &s.to_lowercase() as &str {
            "instance" => Self::Instance,
            "device" => Self::Device,
            _ => unreachable!("unknown extension type: {}", s),
        }
    }
}

/// Deprecation statuses that an extension may have
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeprecationStatus {
    /// The extension is not deprecated
    Current,

    /// The extension was promoted to another extension
    Promoted(String),

    /// The extension was promoted to a Vulkan core version
    PromotedVersion(Origin<'static>),

    /// The extension was deprecated by another extension
    Deprecated(String),

    /// The extension was deprecated by a Vulkan core version
    DeprecatedVersion(Origin<'static>),

    /// The extension was made obsolete by another extension
    Obsoleted(String),

    /// The extension was made obsolete by a Vulkan core version
    ObsoletedVersion(Origin<'static>),
}

impl DeprecationStatus {
    /// Creates a new deprecation status from the components of an extension in the Vulkan
    /// specification
    pub fn new(promoted: Option<&String>, deprecated: Option<&String>, obsoleted: Option<&String>) -> Self {
        match (promoted, deprecated, obsoleted) {
            (Some(v), None, None) => {
                if Origin::match_version(v) {
                    Self::PromotedVersion(Origin::from_core(v))
                } else {
                    Self::Promoted(v.clone())
                }
            },
            (None, Some(v), None) => {
                if Origin::match_version(v) {
                    Self::DeprecatedVersion(Origin::from_core(v))
                } else {
                    Self::Deprecated(v.clone())
                }
            },
            (None, None, Some(v)) => {
                if Origin::match_version(v) {
                    Self::ObsoletedVersion(Origin::from_core(v))
                } else {
                    Self::Obsoleted(v.clone())
                }
            },
            (None, None, None) => Self::Current,
            _ => unreachable!(
                "unknown deprecation status: ({:?}, {:?}, {:?})",
                promoted, deprecated, obsoleted,
            ),
        }
    }
}
