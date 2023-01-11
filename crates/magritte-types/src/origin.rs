//! # Origin
//! An origin is **where** a Vulkan spec element comes from.
//! This can be the base spec, a specific Vulkan version or an extension.

use std::borrow::Cow;

use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};

use crate::{DeprecationStatus, Queryable, Source, SymbolName};

/// The origin of an element of the Vulkan spec
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Origin<'a> {
    /// The origin is unknown
    Unknown,

    /// The core Vulkan specification
    Core,

    /// An extension with its name and whether it is disabled or not
    Extension(Cow<'a, str>, i64, bool),

    /// Vulkan 1.0
    Vulkan1_0,

    /// Vulkan 1.1
    Vulkan1_1,

    /// Vulkan 1.2
    Vulkan1_2,

    /// Vulkan 1.3
    Vulkan1_3,

    /// An opaque external library
    Opaque,
}

impl<'a> Origin<'a> {
    /// Checks whether a string matches a Vulkan core version string or not
    pub fn match_version(s: &str) -> bool {
        matches!(
            s,
            "VK_VERSION_1_0" | "1.0" | "VK_VERSION_1_1" | "1.1" | "VK_VERSION_1_2" | "1.2" | "VK_VERSION_1_3" | "1.3"
        )
    }

    /// Creates a new origin from a core version string
    pub fn from_core(value: &str) -> Self {
        match value {
            "VK_VERSION_1_0" | "1.0" => Self::Vulkan1_0,
            "VK_VERSION_1_1" | "1.1" => Self::Vulkan1_1,
            "VK_VERSION_1_2" | "1.2" => Self::Vulkan1_2,
            "VK_VERSION_1_3" | "1.3" => Self::Vulkan1_3,
            s => unreachable!("unknown vulkan version: {}", s),
        }
    }

    /// Gets the major part of the version from an origin.
    ///
    /// # Panics
    /// Panics if the origin is not a Vulkan version
    pub fn major(&self) -> u32 {
        match self {
            Self::Vulkan1_0 => 1,
            Self::Vulkan1_1 => 1,
            Self::Vulkan1_2 => 1,
            Self::Vulkan1_3 => 1,
            _ => unreachable!("unknown vulkan version"),
        }
    }

    /// Gets the minor part of the version from an origin.
    ///
    /// # Panics
    /// Panics if the origin is not a Vulkan version
    pub fn minor(&self) -> u32 {
        match self {
            Self::Vulkan1_0 => 0,
            Self::Vulkan1_1 => 1,
            Self::Vulkan1_2 => 2,
            Self::Vulkan1_3 => 3,
            _ => unreachable!("unknown vulkan version"),
        }
    }

    /// Is the origin unknown
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Is the origin an extension
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension(_, _, _))
    }

    /// Is the origin the base Vulkan spec
    pub const fn is_vulkan(&self) -> bool {
        matches!(
            self,
            Self::Core | Self::Vulkan1_0 | Self::Vulkan1_1 | Self::Vulkan1_2 | Self::Vulkan1_3
        )
    }

    /// Is the origin disabled
    /// If this is an extension, whether or not it is disabled
    pub const fn is_disabled(&self) -> bool {
        matches!(self, Self::Unknown | Self::Extension(_, _, true))
    }

    /// Is the origin an opaque external type
    pub const fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque)
    }

    /// Is this type always present (i.e it's part of the base Vulkan spec)
    pub fn always(&self) -> bool {
        matches!(self, Origin::Core | Origin::Vulkan1_0)
    }

    /// Gets the ID of an extension, panics otherwise.
    pub fn id(&self) -> i64 {
        match self {
            Self::Extension(_, id, _) => *id,
            Self::Core | Self::Vulkan1_0 | Self::Vulkan1_1 | Self::Vulkan1_2 | Self::Vulkan1_3 => 0,
            other => panic!("not an extension: {:?}", other),
        }
    }

    /// Converts an origin into the same origin with a static lifetime
    pub fn to_static(&self) -> Origin<'static> {
        match self {
            Origin::Unknown => Origin::Unknown,
            Origin::Core => Origin::Core,
            Origin::Extension(name, id, disabled) => Origin::Extension(Cow::Owned(name.to_string()), *id, *disabled),
            Origin::Vulkan1_0 => Origin::Vulkan1_0,
            Origin::Vulkan1_1 => Origin::Vulkan1_1,
            Origin::Vulkan1_2 => Origin::Vulkan1_2,
            Origin::Vulkan1_3 => Origin::Vulkan1_3,
            Origin::Opaque => Origin::Opaque,
        }
    }

    /// Gets the name/module name of the origin
    pub fn as_name(&self) -> String {
        match self {
            Origin::Unknown => panic!("unknown origin cannot be turned into a module"),
            Origin::Core => "core".to_string(),
            Origin::Extension(_, _, true) => panic!("cannot write files for disabled extensions"),
            Origin::Extension(ext, _, _) => ext.trim_start_matches("VK_").to_snake_case(),
            Origin::Vulkan1_0 => "vulkan1_0".to_string(),
            Origin::Vulkan1_1 => "vulkan1_1".to_string(),
            Origin::Vulkan1_2 => "vulkan1_2".to_string(),
            Origin::Vulkan1_3 => "vulkan1_3".to_string(),
            Origin::Opaque => "native".to_string(),
        }
    }

    /// Generate the feature flags (if any) for this origin
    pub fn feature_flag<'b>(&self, source: &Source<'b>) -> Option<Vec<String>> {
        match self {
            Origin::Unknown => panic!("unknown origin cannot be turned into a module"),
            Origin::Extension(_, _, true) => panic!("cannot write files for disabled extensions"),
            Origin::Extension(ext, _, _) => {
                let extension = source.extensions.get_by_name(ext).expect("unknown extension");
                match extension.deprecation_status() {
                    DeprecationStatus::Current
                    | DeprecationStatus::Obsoleted(_)
                    | DeprecationStatus::ObsoletedVersion(_)
                    | DeprecationStatus::DeprecatedVersion(_)
                    | DeprecationStatus::Deprecated(_) => Some(vec![ext.to_string()]),
                    DeprecationStatus::PromotedVersion(by) => match by {
                        Self::Vulkan1_2 | Self::Vulkan1_3 => Some(vec![ext.to_string(), by.name().to_string()]),
                        _ => Some(vec![ext.to_string()]),
                    },
                    DeprecationStatus::Promoted(by) => Some(vec![ext.to_string(), by.to_string()]),
                }
            },
            /*Origin::Core
            | Origin::Opaque
            | Origin::Vulkan1_0
            | Origin::Vulkan1_1
            | Origin::Vulkan1_2
            | Origin::Vulkan1_3 => None,*/
            Origin::Core | Origin::Opaque | Origin::Vulkan1_0 | Origin::Vulkan1_1 => None,
            Origin::Vulkan1_2 => Some(vec!["VULKAN_1_2".to_owned()]),
            Origin::Vulkan1_3 => Some(vec!["VULKAN_1_3".to_owned()]),
        }
    }

    /// Is another origin required for this origin
    pub fn requires(&self, source: &Source<'a>, other: &Origin) -> bool {
        match (self, other) {
            (Origin::Extension(name, _, _), Origin::Extension(other, _, _)) => source
                .extensions
                .get_by_either(name)
                .expect("unknown extension")
                .requires(source, other),
            (Origin::Extension(name, _, _), Origin::Vulkan1_0) => {
                match source
                    .extensions
                    .get_by_either(name)
                    .expect("unknown extension")
                    .min_core()
                {
                    Origin::Vulkan1_0 => true,
                    Origin::Vulkan1_1 => false,
                    Origin::Vulkan1_2 => false,
                    Origin::Vulkan1_3 => false,
                    _ => unreachable!("unknown core version"),
                }
            },
            (Origin::Extension(name, _, _), Origin::Vulkan1_1) => {
                match source
                    .extensions
                    .get_by_either(name)
                    .expect("unknown extension")
                    .min_core()
                {
                    Origin::Vulkan1_0 => true,
                    Origin::Vulkan1_1 => true,
                    Origin::Vulkan1_2 => false,
                    Origin::Vulkan1_3 => false,
                    _ => unreachable!("unknown core version"),
                }
            },
            (Origin::Extension(name, _, _), Origin::Vulkan1_2) => {
                match source
                    .extensions
                    .get_by_either(name)
                    .expect("unknown extension")
                    .min_core()
                {
                    Origin::Vulkan1_0 => true,
                    Origin::Vulkan1_1 => true,
                    Origin::Vulkan1_2 => true,
                    Origin::Vulkan1_3 => false,
                    _ => unreachable!("unknown core version"),
                }
            },
            (Origin::Extension(name, _, _), Origin::Vulkan1_3) => {
                match source
                    .extensions
                    .get_by_either(name)
                    .expect("unknown extension")
                    .min_core()
                {
                    Origin::Vulkan1_0 => true,
                    Origin::Vulkan1_1 => true,
                    Origin::Vulkan1_2 => true,
                    Origin::Vulkan1_3 => true,
                    _ => unreachable!("unknown core version"),
                }
            },
            _ => false,
        }
    }
}

impl<'a> Default for Origin<'a> {
    fn default() -> Self {
        Self::Unknown
    }
}

impl<'a> SymbolName<'a> for Origin<'a> {
    fn name(&self) -> Cow<'a, str> {
        match self {
            Origin::Unknown => panic!("Unknown origin cannot be turned into a name"),
            Origin::Core => Cow::Borrowed("CORE"),
            Origin::Extension(ext, _, _) => ext.clone(),
            Origin::Vulkan1_0 => Cow::Borrowed("VULKAN_1_0"),
            Origin::Vulkan1_1 => Cow::Borrowed("VULKAN_1_1"),
            Origin::Vulkan1_2 => Cow::Borrowed("VULKAN_1_2"),
            Origin::Vulkan1_3 => Cow::Borrowed("VULKAN_1_3"),
            Origin::Opaque => Cow::Borrowed("OPAQUE"),
        }
    }

    fn pretty_name(&self) -> String {
        self.name().to_string()
    }
}

impl<'a> Queryable<'a> for Origin<'a> {
    fn find<'b>(&'b self, _source: &'b Source<'a>, _name: &str) -> Option<&'b str> {
        None
    }
}
