//![VK_KHR_pipeline_executable_properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_executable_properties.html) - device extension
//!# Description
//!When a pipeline is created, its state and shaders are compiled into zero or
//!more device-specific executables, which are used when executing commands
//!against that pipeline.
//!This extension adds a mechanism to query properties and statistics about the
//!different executables produced by the pipeline compilation process.
//!This is intended to be used by debugging and performance tools to allow them
//!to provide more detailed information to the user.
//!Certain compile-time shader statistics provided through this extension may
//!be useful to developers for debugging or performance analysis.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jason Ekstrand [jekstrand](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_pipeline_executable_properties]
//!   @jekstrand%0A<<Here describe the issue or question you have about the
//!   VK_KHR_pipeline_executable_properties extension>>)
//!# New functions & commands
//! - [`GetPipelineExecutableInternalRepresentationsKHR`]
//! - [`GetPipelineExecutablePropertiesKHR`]
//! - [`GetPipelineExecutableStatisticsKHR`]
//!# New structures
//! - [`PipelineExecutableInfoKHR`]
//! - [`PipelineExecutableInternalRepresentationKHR`]
//! - [`PipelineExecutablePropertiesKHR`]
//! - [`PipelineExecutableStatisticKHR`]
//! - [`PipelineInfoKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]
//!# New enums
//! - [`PipelineExecutableStatisticFormatKHR`]
//!# New constants
//! - [`KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME`]
//! - [`KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION`]
//! - Extending [`PipelineCreateFlagBits`]:
//! - `VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR`
//! - `VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR`
//! - `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`
//! - `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`
//! - `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`
//! - `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) What should we call the pieces of the pipeline which are produced by the
//!compilation process and about which you can query properties and statistics?**RESOLVED**: Call
//! them “executables”.
//!The name “binary” was used in early drafts of the extension but it was
//!determined that “pipeline binary” could have a fairly broad meaning (such
//!as a binary serialized form of an entire pipeline) and was too big of a
//!namespace for the very specific needs of this extension.
//!# Version History
//! - Revision 1, 2019-05-28 (Jason Ekstrand)
//! - Initial draft
//!# Other info
//! * 2019-05-28
//! * No known IP claims.
//!*
//! - Jason Ekstrand, Intel
//! - Ian Romanick, Intel
//! - Kenneth Graunke, Intel
//! - Baldur Karlsson, Valve
//! - Jesse Hall, Google
//! - Jeff Bolz, Nvidia
//! - Piers Daniel, Nvidia
//! - Tobias Hector, AMD
//! - Jan-Harald Fredriksen, ARM
//! - Tom Olson, ARM
//! - Daniel Koch, Nvidia
//! - Spencer Fricke, Samsung
//!# Related
//! - [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]
//! - [`PipelineExecutableInfoKHR`]
//! - [`PipelineExecutableInternalRepresentationKHR`]
//! - [`PipelineExecutablePropertiesKHR`]
//! - [`PipelineExecutableStatisticFormatKHR`]
//! - [`PipelineExecutableStatisticKHR`]
//! - [`PipelineExecutableStatisticValueKHR`]
//! - [`PipelineInfoKHR`]
//! - [`GetPipelineExecutableInternalRepresentationsKHR`]
//! - [`GetPipelineExecutablePropertiesKHR`]
//! - [`GetPipelineExecutableStatisticsKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_pipeline_executable_properties");
///[VkPipelineExecutableStatisticFormatKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html) - Enum describing a pipeline executable statistic
///# C Specifications
///The [`PipelineExecutableStatisticFormatKHR`] enum is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef enum VkPipelineExecutableStatisticFormatKHR {
///    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR = 0,
///    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR = 1,
///    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR = 2,
///    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR = 3,
///} VkPipelineExecutableStatisticFormatKHR;
///```
///# Description
/// - [`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32`] specifies that
///the statistic is returned as a 32-bit boolean value which **must** be
///either [`TRUE`] or [`FALSE`] and **should** be read from the
///`b32` field of [`PipelineExecutableStatisticValueKHR`].
/// - [`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64`] specifies that
///the statistic is returned as a signed 64-bit integer and **should** be read
///from the `i64` field of [`PipelineExecutableStatisticValueKHR`].
/// - [`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64`] specifies that
///the statistic is returned as an unsigned 64-bit integer and **should** be
///read from the `u64` field of
///[`PipelineExecutableStatisticValueKHR`].
/// - [`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64`] specifies that
///the statistic is returned as a 64-bit floating-point value and **should**
///be read from the `f64` field of
///[`PipelineExecutableStatisticValueKHR`].
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`PipelineExecutableStatisticKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineExecutableStatisticFormatKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineExecutableStatisticFormatKHR(i32);
impl const Default for PipelineExecutableStatisticFormatKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PipelineExecutableStatisticFormatKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PipelineExecutableStatisticFormatKHR")
            .field(match *self {
                Self::PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32 => &"PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32",
                Self::PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64 => &"PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64",
                Self::PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64 => &"PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64",
                Self::PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64 => &"PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64",
                other => unreachable!("invalid value for `PipelineExecutableStatisticFormatKHR`: {:?}", other),
            })
            .finish()
    }
}
impl PipelineExecutableStatisticFormatKHR {
    ///[`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32`] specifies that
    ///the statistic is returned as a 32-bit boolean value which **must** be
    ///either [`TRUE`] or [`FALSE`] and **should** be read from the
    ///`b32` field of [`PipelineExecutableStatisticValueKHR`].
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32: Self = Self(0);
    ///[`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64`] specifies that
    ///the statistic is returned as a signed 64-bit integer and **should** be read
    ///from the `i64` field of [`PipelineExecutableStatisticValueKHR`].
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64: Self = Self(1);
    ///[`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64`] specifies that
    ///the statistic is returned as an unsigned 64-bit integer and **should** be
    ///read from the `u64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64: Self = Self(2);
    ///[`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64`] specifies that
    ///the statistic is returned as a 64-bit floating-point value and **should**
    ///be read from the `f64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64: Self = Self(3);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
