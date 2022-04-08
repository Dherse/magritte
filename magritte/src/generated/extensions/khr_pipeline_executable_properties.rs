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
//! - [`get_pipeline_executable_internal_representations_khr`]
//! - [`get_pipeline_executable_properties_khr`]
//! - [`get_pipeline_executable_statistics_khr`]
//!# New structures
//! - [`PipelineExecutableInfoKHR`]
//! - [`PipelineExecutableInternalRepresentationKHR`]
//! - [`PipelineExecutablePropertiesKHR`]
//! - [`PipelineExecutableStatisticKHR`]
//! - [`PipelineInfoKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]
//!# New enums
//! - [`PipelineExecutableStatisticFormatKHR`]
//!# New constants
//! - [`KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME`]
//! - [`KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION`]
//! - Extending [`PipelineCreateFlagBits`]:  -
//!   `VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR`  -
//!   `VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) What should we call the pieces of the pipeline which are produced by the
//!compilation process and about which you can query properties and statistics? **RESOLVED** : Call
//! them “executables”.
//!The name “binary” was used in early drafts of the extension but it was
//!determined that “pipeline binary” could have a fairly broad meaning (such
//!as a binary serialized form of an entire pipeline) and was too big of a
//!namespace for the very specific needs of this extension.
//!# Version History
//! - Revision 1, 2019-05-28 (Jason Ekstrand)  - Initial draft
//!# Other info
//! * 2019-05-28
//! * No known IP claims.
//! * - Jason Ekstrand, Intel  - Ian Romanick, Intel  - Kenneth Graunke, Intel  - Baldur Karlsson,
//!   Valve  - Jesse Hall, Google  - Jeff Bolz, Nvidia  - Piers Daniel, Nvidia  - Tobias Hector, AMD
//!   - Jan-Harald Fredriksen, ARM  - Tom Olson, ARM  - Daniel Koch, Nvidia  - Spencer Fricke,
//!   Samsung
//!# Related
//! - [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]
//! - [`PipelineExecutableInfoKHR`]
//! - [`PipelineExecutableInternalRepresentationKHR`]
//! - [`PipelineExecutablePropertiesKHR`]
//! - [`PipelineExecutableStatisticFormatKHR`]
//! - [`PipelineExecutableStatisticKHR`]
//! - [`PipelineExecutableStatisticValueKHR`]
//! - [`PipelineInfoKHR`]
//! - [`get_pipeline_executable_internal_representations_khr`]
//! - [`get_pipeline_executable_properties_khr`]
//! - [`get_pipeline_executable_statistics_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    core::MAX_DESCRIPTION_SIZE,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, Device, Pipeline, ShaderStageFlags, StructureType, VulkanResultCodes,
    },
    AsRaw, SmallVec, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    os::raw::c_char,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_pipeline_executable_properties");
///[vkGetPipelineExecutablePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) - Get the executables associated with a pipeline
///# C Specifications
///When a pipeline is created, its state and shaders are compiled into zero or
///more device-specific executables, which are used when executing commands
///against that pipeline.
///To query the properties of these pipeline executables, call:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///VkResult vkGetPipelineExecutablePropertiesKHR(
///    VkDevice                                    device,
///    const VkPipelineInfoKHR*                    pPipelineInfo,
///    uint32_t*                                   pExecutableCount,
///    VkPipelineExecutablePropertiesKHR*          pProperties);
///```
///# Parameters
/// - [`device`] is the device that created the pipeline.
/// - [`p_pipeline_info`] describes the pipeline being queried.
/// - [`p_executable_count`] is a pointer to an integer related to the number of pipeline
///   executables available or queried, as described below.
/// - [`p_properties`] is either `NULL` or a pointer to an array of
///   [`PipelineExecutablePropertiesKHR`] structures.
///# Description
///If [`p_properties`] is `NULL`, then the number of pipeline executables
///associated with the pipeline is returned in [`p_executable_count`].
///Otherwise, [`p_executable_count`] **must**  point to a variable set by the user
///to the number of elements in the [`p_properties`] array, and on return the
///variable is overwritten with the number of structures actually written to
///[`p_properties`].
///If [`p_executable_count`] is less than the number of pipeline executables
///associated with the pipeline, at most [`p_executable_count`] structures will
///be written, and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available properties were
///returned.
///## Valid Usage
/// - [`pipelineExecutableInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineExecutableInfo)
///   **must**  be enabled
/// - `pipeline` member of [`p_pipeline_info`] **must**  have been created with [`device`]
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_pipeline_info`] **must**  be a valid pointer to a valid [`PipelineInfoKHR`] structure
/// - [`p_executable_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_executable_count`] is not `0`, and [`p_properties`] is not
///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
///   [`p_executable_count`][`PipelineExecutablePropertiesKHR`] structures
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Device`]
/// - [`PipelineExecutablePropertiesKHR`]
/// - [`PipelineInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
pub type FNGetPipelineExecutablePropertiesKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_pipeline_info: *const PipelineInfoKHR<'lt>,
        p_executable_count: *mut u32,
        p_properties: *mut PipelineExecutablePropertiesKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetPipelineExecutableStatisticsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html) - Get compile time statistics associated with a pipeline executable
///# C Specifications
///Each pipeline executable  **may**  have a set of statistics associated with it
///that are generated by the pipeline compilation process.
///These statistics  **may**  include things such as instruction counts, amount of
///spilling (if any), maximum number of simultaneous threads, or anything else
///which  **may**  aid developers in evaluating the expected performance of a
///shader.
///To query the compile-time statistics associated with a pipeline executable,
///call:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///VkResult vkGetPipelineExecutableStatisticsKHR(
///    VkDevice                                    device,
///    const VkPipelineExecutableInfoKHR*          pExecutableInfo,
///    uint32_t*                                   pStatisticCount,
///    VkPipelineExecutableStatisticKHR*           pStatistics);
///```
///# Parameters
/// - [`device`] is the device that created the pipeline.
/// - [`p_executable_info`] describes the pipeline executable being queried.
/// - [`p_statistic_count`] is a pointer to an integer related to the number of statistics available
///   or queried, as described below.
/// - [`p_statistics`] is either `NULL` or a pointer to an array of
///   [`PipelineExecutableStatisticKHR`] structures.
///# Description
///If [`p_statistics`] is `NULL`, then the number of statistics associated
///with the pipeline executable is returned in [`p_statistic_count`].
///Otherwise, [`p_statistic_count`] **must**  point to a variable set by the user
///to the number of elements in the [`p_statistics`] array, and on return the
///variable is overwritten with the number of structures actually written to
///[`p_statistics`].
///If [`p_statistic_count`] is less than the number of statistics associated
///with the pipeline executable, at most [`p_statistic_count`] structures will
///be written, and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available statistics were
///returned.
///## Valid Usage
/// - [`pipelineExecutableInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineExecutableInfo)
///   **must**  be enabled
/// - `pipeline` member of [`p_executable_info`] **must**  have been created with [`device`]
/// - `pipeline` member of [`p_executable_info`] **must**  have been created with
///   `VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_executable_info`] **must**  be a valid pointer to a valid [`PipelineExecutableInfoKHR`]
///   structure
/// - [`p_statistic_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_statistic_count`] is not `0`, and [`p_statistics`] is not
///   `NULL`, [`p_statistics`] **must**  be a valid pointer to an array of
///   [`p_statistic_count`][`PipelineExecutableStatisticKHR`] structures
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Device`]
/// - [`PipelineExecutableInfoKHR`]
/// - [`PipelineExecutableStatisticKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
pub type FNGetPipelineExecutableStatisticsKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR<'lt>,
        p_statistic_count: *mut u32,
        p_statistics: *mut PipelineExecutableStatisticKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[vkGetPipelineExecutableInternalRepresentationsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) - Get internal representations of the pipeline executable
///# C Specifications
///Each pipeline executable  **may**  have one or more text or binary internal
///representations associated with it which are generated as part of the
///compile process.
///These  **may**  include the final shader assembly, a binary form of the compiled
///shader, or the shader compiler’s internal representation at any number of
///intermediate compile steps.
///To query the internal representations associated with a pipeline executable,
///call:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///VkResult vkGetPipelineExecutableInternalRepresentationsKHR(
///    VkDevice                                    device,
///    const VkPipelineExecutableInfoKHR*          pExecutableInfo,
///    uint32_t*                                   pInternalRepresentationCount,
///    VkPipelineExecutableInternalRepresentationKHR* pInternalRepresentations);
///```
///# Parameters
/// - [`device`] is the device that created the pipeline.
/// - [`p_executable_info`] describes the pipeline executable being queried.
/// - [`p_internal_representation_count`] is a pointer to an integer related to the number of
///   internal representations available or queried, as described below.
/// - [`p_internal_representations`] is either `NULL` or a pointer to an array of
///   [`PipelineExecutableInternalRepresentationKHR`] structures.
///# Description
///If [`p_internal_representations`] is `NULL`, then the number of internal
///representations associated with the pipeline executable is returned in
///[`p_internal_representation_count`].
///Otherwise, [`p_internal_representation_count`] **must**  point to a variable set
///by the user to the number of elements in the [`p_internal_representations`]
///array, and on return the variable is overwritten with the number of
///structures actually written to [`p_internal_representations`].
///If [`p_internal_representation_count`] is less than the number of internal
///representations associated with the pipeline executable, at most
///[`p_internal_representation_count`] structures will be written, and
///`VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
///indicate that not all the available representations were returned.While the details of the
/// internal representations remain
///implementation-dependent, the implementation  **should**  order the internal
///representations in the order in which they occur in the compiled pipeline
///with the final shader assembly (if any) last.
///## Valid Usage
/// - [`pipelineExecutableInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineExecutableInfo)
///   **must**  be enabled
/// - `pipeline` member of [`p_executable_info`] **must**  have been created with [`device`]
/// - `pipeline` member of [`p_executable_info`] **must**  have been created with
///   `VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_executable_info`] **must**  be a valid pointer to a valid [`PipelineExecutableInfoKHR`]
///   structure
/// - [`p_internal_representation_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_internal_representation_count`] is not `0`, and
///   [`p_internal_representations`] is not `NULL`, [`p_internal_representations`] **must**  be a
///   valid pointer to an array of
///   [`p_internal_representation_count`][`PipelineExecutableInternalRepresentationKHR`] structures
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Device`]
/// - [`PipelineExecutableInfoKHR`]
/// - [`PipelineExecutableInternalRepresentationKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
pub type FNGetPipelineExecutableInternalRepresentationsKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR<'lt>,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR<'lt>,
    ) -> VulkanResultCodes,
>;
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
/// - [`BOOL32`] specifies that the statistic is returned as a 32-bit boolean value which  **must**
///   be either [`TRUE`] or [`FALSE`] and  **should**  be read from the `b32` field of
///   [`PipelineExecutableStatisticValueKHR`].
/// - [`INT64`] specifies that the statistic is returned as a signed 64-bit integer and  **should**
///   be read from the `i64` field of [`PipelineExecutableStatisticValueKHR`].
/// - [`UINT64`] specifies that the statistic is returned as an unsigned 64-bit integer and
///   **should**  be read from the `u64` field of [`PipelineExecutableStatisticValueKHR`].
/// - [`FLOAT64`] specifies that the statistic is returned as a 64-bit floating-point value and
///   **should**  be read from the `f64` field of [`PipelineExecutableStatisticValueKHR`].
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineExecutableStatisticFormatKHR(i32);
impl const Default for PipelineExecutableStatisticFormatKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl PipelineExecutableStatisticFormatKHR {
    ///[`BOOL32`] specifies that
    ///the statistic is returned as a 32-bit boolean value which  **must**  be
    ///either [`TRUE`] or [`FALSE`] and  **should**  be read from the
    ///`b32` field of [`PipelineExecutableStatisticValueKHR`].
    pub const BOOL32: Self = Self(0);
    ///[`INT64`] specifies that
    ///the statistic is returned as a signed 64-bit integer and  **should**  be read
    ///from the `i64` field of [`PipelineExecutableStatisticValueKHR`].
    pub const INT64: Self = Self(1);
    ///[`UINT64`] specifies that
    ///the statistic is returned as an unsigned 64-bit integer and  **should**  be
    ///read from the `u64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    pub const UINT64: Self = Self(2);
    ///[`FLOAT64`] specifies that
    ///the statistic is returned as a 64-bit floating-point value and  **should**
    ///be read from the `f64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    pub const FLOAT64: Self = Self(3);
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
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for PipelineExecutableStatisticFormatKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PipelineExecutableStatisticFormatKHR))
            .field(match *self {
                Self::BOOL32 => &"BOOL32",
                Self::INT64 => &"INT64",
                Self::UINT64 => &"UINT64",
                Self::FLOAT64 => &"FLOAT64",
                other => unreachable!(
                    concat!(
                        "invalid value for",
                        stringify!(PipelineExecutableStatisticFormatKHR),
                        ": {:?}"
                    ),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for PipelineExecutableStatisticFormatKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::BOOL32 => &"BOOL32",
            Self::INT64 => &"INT64",
            Self::UINT64 => &"UINT64",
            Self::FLOAT64 => &"FLOAT64",
            other => unreachable!(
                concat!(
                    "invalid value for",
                    stringify!(PipelineExecutableStatisticFormatKHR),
                    ": {:?}"
                ),
                other
            ),
        })
    }
}
///[VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html) - Structure describing whether pipeline executable properties are available
///# C Specifications
///The [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`] structure
///is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           pipelineExecutableInfo;
///} VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_executable_info`] indicates that the implementation supports reporting properties
///   and statistics about the pipeline executables associated with a compiled pipeline.
///If the [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`] **can**  also be used in the
/// [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`pipeline_executable_info`]
    ///indicates that the implementation supports reporting properties and
    ///statistics about the pipeline executables associated with a compiled
    ///pipeline.
    pub pipeline_executable_info: Bool32,
}
impl<'lt> Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            pipeline_executable_info: 0,
        }
    }
}
impl<'lt> PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::pipeline_executable_info`]
    pub fn pipeline_executable_info_raw(&self) -> Bool32 {
        self.pipeline_executable_info
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_executable_info`]
    pub fn set_pipeline_executable_info_raw(mut self, value: Bool32) -> Self {
        self.pipeline_executable_info = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::pipeline_executable_info`]
    pub fn pipeline_executable_info(&self) -> bool {
        unsafe { std::mem::transmute(self.pipeline_executable_info as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_executable_info`]
    pub fn pipeline_executable_info_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.pipeline_executable_info as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.pipeline_executable_info as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::pipeline_executable_info`]
    pub fn set_pipeline_executable_info(mut self, value: bool) -> Self {
        self.pipeline_executable_info = value as u8 as u32;
        self
    }
}
///[VkPipelineInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoKHR.html) - Structure describing a pipeline
///# C Specifications
///The [`PipelineInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkPipeline         pipeline;
///} VkPipelineInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline`] is a [`Pipeline`] handle.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Pipeline`]
/// - [`StructureType`]
/// - [`get_pipeline_executable_properties_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`pipeline`] is a [`Pipeline`] handle.
    pub pipeline: Pipeline,
}
impl<'lt> Default for PipelineInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_INFO_KHR,
            p_next: std::ptr::null(),
            pipeline: Default::default(),
        }
    }
}
impl<'lt> PipelineInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::pipeline`]
    pub fn pipeline(&self) -> Pipeline {
        self.pipeline
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::pipeline`]
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::pipeline`]
    pub fn set_pipeline(mut self, value: crate::vulkan1_0::Pipeline) -> Self {
        self.pipeline = value;
        self
    }
}
///[VkPipelineExecutablePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutablePropertiesKHR.html) - Structure describing a pipeline executable
///# C Specifications
///The [`PipelineExecutablePropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineExecutablePropertiesKHR {
///    VkStructureType       sType;
///    void*                 pNext;
///    VkShaderStageFlags    stages;
///    char                  name[VK_MAX_DESCRIPTION_SIZE];
///    char                  description[VK_MAX_DESCRIPTION_SIZE];
///    uint32_t              subgroupSize;
///} VkPipelineExecutablePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stages`] is a bitmask of zero or more [`ShaderStageFlagBits`] indicating which shader stages
///   (if any) were principally used as inputs to compile this pipeline executable.
/// - [`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8
///   string which is a short human readable name for this pipeline executable.
/// - [`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated
///   UTF-8 string which is a human readable description for this pipeline executable.
/// - [`subgroup_size`] is the subgroup size with which this pipeline executable is dispatched.
///# Description
///Not all implementations have a 1:1 mapping between shader stages and
///pipeline executables and some implementations  **may**  reduce a given shader
///stage to fixed function hardware programming such that no pipeline
///executable is available.
///No guarantees are provided about the mapping between shader stages and
///pipeline executables and [`stages`] **should**  be considered a best effort
///hint.
///Because the application  **cannot**  rely on the [`stages`] field to provide an
///exact description, [`name`] and [`description`] provide a human readable
///name and description which more accurately describes the given pipeline
///executable.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`ShaderStageFlags`]
/// - [`StructureType`]
/// - [`get_pipeline_executable_properties_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineExecutablePropertiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineExecutablePropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`stages`] is a bitmask of zero or more [`ShaderStageFlagBits`]
    ///indicating which shader stages (if any) were principally used as inputs
    ///to compile this pipeline executable.
    pub stages: ShaderStageFlags,
    ///[`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a short human
    ///readable name for this pipeline executable.
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    ///[`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a human readable
    ///description for this pipeline executable.
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    ///[`subgroup_size`] is the subgroup size with which this pipeline
    ///executable is dispatched.
    pub subgroup_size: u32,
}
impl<'lt> Default for PipelineExecutablePropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_EXECUTABLE_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            stages: Default::default(),
            name: [b'\0' as i8; MAX_DESCRIPTION_SIZE as usize],
            description: [b'\0' as i8; MAX_DESCRIPTION_SIZE as usize],
            subgroup_size: 0,
        }
    }
}
impl<'lt> PipelineExecutablePropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::stages`]
    pub fn stages(&self) -> ShaderStageFlags {
        self.stages
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &[c_char; MAX_DESCRIPTION_SIZE as usize] {
        &self.name
    }
    ///Gets the value of [`Self::description`]
    pub fn description(&self) -> &[c_char; MAX_DESCRIPTION_SIZE as usize] {
        &self.description
    }
    ///Gets the value of [`Self::subgroup_size`]
    pub fn subgroup_size(&self) -> u32 {
        self.subgroup_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::stages`]
    pub fn stages_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.stages
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut [c_char; MAX_DESCRIPTION_SIZE as usize] {
        &mut self.name
    }
    ///Gets a mutable reference to the value of [`Self::description`]
    pub fn description_mut(&mut self) -> &mut [c_char; MAX_DESCRIPTION_SIZE as usize] {
        &mut self.description
    }
    ///Gets a mutable reference to the value of [`Self::subgroup_size`]
    pub fn subgroup_size_mut(&mut self) -> &mut u32 {
        &mut self.subgroup_size
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::stages`]
    pub fn set_stages(mut self, value: crate::vulkan1_0::ShaderStageFlags) -> Self {
        self.stages = value;
        self
    }
    ///Sets the value of [`Self::name`]
    pub fn set_name(mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE as usize]) -> Self {
        self.name = value;
        self
    }
    ///Sets the value of [`Self::description`]
    pub fn set_description(
        mut self,
        value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.description = value;
        self
    }
    ///Sets the value of [`Self::subgroup_size`]
    pub fn set_subgroup_size(mut self, value: u32) -> Self {
        self.subgroup_size = value;
        self
    }
}
///[VkPipelineExecutableInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInfoKHR.html) - Structure describing a pipeline executable to query for associated statistics or internal representations
///# C Specifications
///The [`PipelineExecutableInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineExecutableInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkPipeline         pipeline;
///    uint32_t           executableIndex;
///} VkPipelineExecutableInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline`] is the pipeline to query.
/// - [`executable_index`] is the index of the pipeline executable to query in the array of
///   executable properties returned by [`get_pipeline_executable_properties_khr`].
///# Description
///## Valid Usage
/// - [`executable_index`] **must**  be less than the number of pipeline executables associated with
///   [`pipeline`] as returned in the `pExecutableCount` parameter of
///   [`get_pipeline_executable_properties_khr`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Pipeline`]
/// - [`StructureType`]
/// - [`get_pipeline_executable_internal_representations_khr`]
/// - [`get_pipeline_executable_statistics_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineExecutableInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineExecutableInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`pipeline`] is the pipeline to query.
    pub pipeline: Pipeline,
    ///[`executable_index`] is the index of the pipeline executable to query
    ///in the array of executable properties returned by
    ///[`get_pipeline_executable_properties_khr`].
    pub executable_index: u32,
}
impl<'lt> Default for PipelineExecutableInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_EXECUTABLE_INFO_KHR,
            p_next: std::ptr::null(),
            pipeline: Default::default(),
            executable_index: 0,
        }
    }
}
impl<'lt> PipelineExecutableInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::pipeline`]
    pub fn pipeline(&self) -> Pipeline {
        self.pipeline
    }
    ///Gets the value of [`Self::executable_index`]
    pub fn executable_index(&self) -> u32 {
        self.executable_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::pipeline`]
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
    ///Gets a mutable reference to the value of [`Self::executable_index`]
    pub fn executable_index_mut(&mut self) -> &mut u32 {
        &mut self.executable_index
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::pipeline`]
    pub fn set_pipeline(mut self, value: crate::vulkan1_0::Pipeline) -> Self {
        self.pipeline = value;
        self
    }
    ///Sets the value of [`Self::executable_index`]
    pub fn set_executable_index(mut self, value: u32) -> Self {
        self.executable_index = value;
        self
    }
}
///[VkPipelineExecutableStatisticKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticKHR.html) - Structure describing a compile-time pipeline executable statistic
///# C Specifications
///The [`PipelineExecutableStatisticKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineExecutableStatisticKHR {
///    VkStructureType                           sType;
///    void*                                     pNext;
///    char                                      name[VK_MAX_DESCRIPTION_SIZE];
///    char                                      description[VK_MAX_DESCRIPTION_SIZE];
///    VkPipelineExecutableStatisticFormatKHR    format;
///    VkPipelineExecutableStatisticValueKHR     value;
///} VkPipelineExecutableStatisticKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8
///   string which is a short human readable name for this statistic.
/// - [`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated
///   UTF-8 string which is a human readable description for this statistic.
/// - [`format`] is a [`PipelineExecutableStatisticFormatKHR`] value specifying the format of the
///   data found in [`value`].
/// - [`value`] is the value of this statistic.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`PipelineExecutableStatisticFormatKHR`]
/// - [`PipelineExecutableStatisticValueKHR`]
/// - [`StructureType`]
/// - [`get_pipeline_executable_statistics_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineExecutableStatisticKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineExecutableStatisticKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a short human
    ///readable name for this statistic.
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    ///[`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a human readable
    ///description for this statistic.
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    ///[`format`] is a [`PipelineExecutableStatisticFormatKHR`] value
    ///specifying the format of the data found in [`value`].
    pub format: PipelineExecutableStatisticFormatKHR,
    ///[`value`] is the value of this statistic.
    pub value: PipelineExecutableStatisticValueKHR,
}
impl<'lt> Default for PipelineExecutableStatisticKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_EXECUTABLE_STATISTIC_KHR,
            p_next: std::ptr::null_mut(),
            name: [b'\0' as i8; MAX_DESCRIPTION_SIZE as usize],
            description: [b'\0' as i8; MAX_DESCRIPTION_SIZE as usize],
            format: Default::default(),
            value: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> PipelineExecutableStatisticKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &[c_char; MAX_DESCRIPTION_SIZE as usize] {
        &self.name
    }
    ///Gets the value of [`Self::description`]
    pub fn description(&self) -> &[c_char; MAX_DESCRIPTION_SIZE as usize] {
        &self.description
    }
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> PipelineExecutableStatisticFormatKHR {
        self.format
    }
    ///Gets the value of [`Self::value`]
    pub fn value(&self) -> PipelineExecutableStatisticValueKHR {
        self.value
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut [c_char; MAX_DESCRIPTION_SIZE as usize] {
        &mut self.name
    }
    ///Gets a mutable reference to the value of [`Self::description`]
    pub fn description_mut(&mut self) -> &mut [c_char; MAX_DESCRIPTION_SIZE as usize] {
        &mut self.description
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut PipelineExecutableStatisticFormatKHR {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::value`]
    pub fn value_mut(&mut self) -> &mut PipelineExecutableStatisticValueKHR {
        &mut self.value
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::name`]
    pub fn set_name(mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE as usize]) -> Self {
        self.name = value;
        self
    }
    ///Sets the value of [`Self::description`]
    pub fn set_description(
        mut self,
        value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.description = value;
        self
    }
    ///Sets the value of [`Self::format`]
    pub fn set_format(
        mut self,
        value: crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticFormatKHR,
    ) -> Self {
        self.format = value;
        self
    }
    ///Sets the value of [`Self::value`]
    pub fn set_value(
        mut self,
        value: crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR,
    ) -> Self {
        self.value = value;
        self
    }
}
///[VkPipelineExecutableInternalRepresentationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html) - Structure describing the textual form of a pipeline executable internal representation
///# C Specifications
///The [`PipelineExecutableInternalRepresentationKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineExecutableInternalRepresentationKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    char               name[VK_MAX_DESCRIPTION_SIZE];
///    char               description[VK_MAX_DESCRIPTION_SIZE];
///    VkBool32           isText;
///    size_t             dataSize;
///    void*              pData;
///} VkPipelineExecutableInternalRepresentationKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8
///   string which is a short human readable name for this internal representation.
/// - [`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated
///   UTF-8 string which is a human readable description for this internal representation.
/// - [`is_text`] specifies whether the returned data is text or opaque data. If [`is_text`] is
///   [`TRUE`] then the data returned in [`data`] is text and is guaranteed to be a null-terminated
///   UTF-8 string.
/// - [`data_size`] is an integer related to the size, in bytes, of the internal representation’s
///   data, as described below.
/// - [`data`] is either `NULL` or a pointer to a block of data into which the implementation will
///   write the internal representation.
///# Description
///If [`data`] is `NULL`, then the size, in bytes, of the internal
///representation data is returned in [`data_size`].
///Otherwise, [`data_size`] must be the size of the buffer, in bytes, pointed
///to by [`data`] and on return [`data_size`] is overwritten with the
///number of bytes of data actually written to [`data`] including any
///trailing null character.
///If [`data_size`] is less than the size, in bytes, of the internal
///representation’s data, at most [`data_size`] bytes of data will be written
///to [`data`], and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available representation was
///returned.If [`is_text`] is [`TRUE`] and [`data`] is not `NULL` and
///[`data_size`] is not zero, the last byte written to [`data`] will be a
///null character.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Bool32`]
/// - [`StructureType`]
/// - [`get_pipeline_executable_internal_representations_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineExecutableInternalRepresentationKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a short human
    ///readable name for this internal representation.
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    ///[`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a human readable
    ///description for this internal representation.
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    ///[`is_text`] specifies whether the returned data is text or opaque data.
    ///If [`is_text`] is [`TRUE`] then the data returned in [`data`]
    ///is text and is guaranteed to be a null-terminated UTF-8 string.
    pub is_text: Bool32,
    ///[`data_size`] is an integer related to the size, in bytes, of the
    ///internal representation’s data, as described below.
    pub data_size: usize,
    ///[`data`] is either `NULL` or a pointer to a block of data into which
    ///the implementation will write the internal representation.
    pub data: *mut c_void,
}
impl<'lt> Default for PipelineExecutableInternalRepresentationKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR,
            p_next: std::ptr::null_mut(),
            name: [b'\0' as i8; MAX_DESCRIPTION_SIZE as usize],
            description: [b'\0' as i8; MAX_DESCRIPTION_SIZE as usize],
            is_text: 0,
            data_size: 0,
            data: std::ptr::null_mut(),
        }
    }
}
impl<'lt> PipelineExecutableInternalRepresentationKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::is_text`]
    pub fn is_text_raw(&self) -> Bool32 {
        self.is_text
    }
    ///Gets the raw value of [`Self::data`]
    pub fn data_raw(&self) -> *mut c_void {
        self.data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::is_text`]
    pub fn set_is_text_raw(mut self, value: Bool32) -> Self {
        self.is_text = value;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data_raw(mut self, value: *mut c_void) -> Self {
        self.data = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &[c_char; MAX_DESCRIPTION_SIZE as usize] {
        &self.name
    }
    ///Gets the value of [`Self::description`]
    pub fn description(&self) -> &[c_char; MAX_DESCRIPTION_SIZE as usize] {
        &self.description
    }
    ///Gets the value of [`Self::is_text`]
    pub fn is_text(&self) -> bool {
        unsafe { std::mem::transmute(self.is_text as u8) }
    }
    ///Gets the value of [`Self::data_size`]
    pub fn data_size(&self) -> usize {
        self.data_size
    }
    ///Gets the value of [`Self::data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn data(&self) -> &[c_void] {
        std::slice::from_raw_parts(self.data, self.data_size as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut [c_char; MAX_DESCRIPTION_SIZE as usize] {
        &mut self.name
    }
    ///Gets a mutable reference to the value of [`Self::description`]
    pub fn description_mut(&mut self) -> &mut [c_char; MAX_DESCRIPTION_SIZE as usize] {
        &mut self.description
    }
    ///Gets a mutable reference to the value of [`Self::is_text`]
    pub fn is_text_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.is_text as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.is_text as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::data_size`]
    pub fn data_size_mut(&mut self) -> &mut usize {
        &mut self.data_size
    }
    ///Gets a mutable reference to the value of [`Self::data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn data_mut(&mut self) -> &mut [c_void] {
        std::slice::from_raw_parts_mut(self.data, self.data_size as usize)
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::name`]
    pub fn set_name(mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE as usize]) -> Self {
        self.name = value;
        self
    }
    ///Sets the value of [`Self::description`]
    pub fn set_description(
        mut self,
        value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE as usize],
    ) -> Self {
        self.description = value;
        self
    }
    ///Sets the value of [`Self::is_text`]
    pub fn set_is_text(mut self, value: bool) -> Self {
        self.is_text = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::data_size`]
    pub fn set_data_size(mut self, value: usize) -> Self {
        self.data_size = value;
        self
    }
    ///Sets the value of [`Self::data`]
    pub fn set_data(mut self, value: &'lt mut [std::ffi::c_void]) -> Self {
        let len_ = value.len() as usize;
        let len_ = len_;
        self.data = value.as_mut_ptr();
        self.data_size = len_;
        self
    }
}
///[VkPipelineExecutableStatisticValueKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html) - A union describing a pipeline executable statistic
///# C Specifications
///The [`PipelineExecutableStatisticValueKHR`] union is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef union VkPipelineExecutableStatisticValueKHR {
///    VkBool32    b32;
///    int64_t     i64;
///    uint64_t    u64;
///    double      f64;
///} VkPipelineExecutableStatisticValueKHR;
///```
///# Members
/// - [`b_32`] is the 32-bit boolean value if the [`PipelineExecutableStatisticFormatKHR`] is
///   `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR`.
/// - [`i_64`] is the signed 64-bit integer value if the [`PipelineExecutableStatisticFormatKHR`] is
///   `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR`.
/// - [`u_64`] is the unsigned 64-bit integer value if the [`PipelineExecutableStatisticFormatKHR`]
///   is `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR`.
/// - [`f_64`] is the 64-bit floating-point value if the [`PipelineExecutableStatisticFormatKHR`] is
///   `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR`.
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Bool32`]
/// - [`PipelineExecutableStatisticKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineExecutableStatisticValueKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub union PipelineExecutableStatisticValueKHR {
    ///[`b_32`] is the 32-bit boolean value if the
    ///[`PipelineExecutableStatisticFormatKHR`] is
    ///`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR`.
    pub b_32: Bool32,
    ///[`i_64`] is the signed 64-bit integer value if the
    ///[`PipelineExecutableStatisticFormatKHR`] is
    ///`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR`.
    pub i_64: i64,
    ///[`u_64`] is the unsigned 64-bit integer value if the
    ///[`PipelineExecutableStatisticFormatKHR`] is
    ///`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR`.
    pub u_64: u64,
    ///[`f_64`] is the 64-bit floating-point value if the
    ///[`PipelineExecutableStatisticFormatKHR`] is
    ///`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR`.
    pub f_64: f64,
}
impl Default for PipelineExecutableStatisticValueKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
impl Device {
    ///[vkGetPipelineExecutablePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html) - Get the executables associated with a pipeline
    ///# C Specifications
    ///When a pipeline is created, its state and shaders are compiled into zero or
    ///more device-specific executables, which are used when executing commands
    ///against that pipeline.
    ///To query the properties of these pipeline executables, call:
    ///```c
    ///// Provided by VK_KHR_pipeline_executable_properties
    ///VkResult vkGetPipelineExecutablePropertiesKHR(
    ///    VkDevice                                    device,
    ///    const VkPipelineInfoKHR*                    pPipelineInfo,
    ///    uint32_t*                                   pExecutableCount,
    ///    VkPipelineExecutablePropertiesKHR*          pProperties);
    ///```
    ///# Parameters
    /// - [`device`] is the device that created the pipeline.
    /// - [`p_pipeline_info`] describes the pipeline being queried.
    /// - [`p_executable_count`] is a pointer to an integer related to the number of pipeline
    ///   executables available or queried, as described below.
    /// - [`p_properties`] is either `NULL` or a pointer to an array of
    ///   [`PipelineExecutablePropertiesKHR`] structures.
    ///# Description
    ///If [`p_properties`] is `NULL`, then the number of pipeline executables
    ///associated with the pipeline is returned in [`p_executable_count`].
    ///Otherwise, [`p_executable_count`] **must**  point to a variable set by the user
    ///to the number of elements in the [`p_properties`] array, and on return the
    ///variable is overwritten with the number of structures actually written to
    ///[`p_properties`].
    ///If [`p_executable_count`] is less than the number of pipeline executables
    ///associated with the pipeline, at most [`p_executable_count`] structures will
    ///be written, and `VK_INCOMPLETE` will be returned instead of
    ///`VK_SUCCESS`, to indicate that not all the available properties were
    ///returned.
    ///## Valid Usage
    /// - [`pipelineExecutableInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineExecutableInfo)
    ///   **must**  be enabled
    /// - `pipeline` member of [`p_pipeline_info`] **must**  have been created with [`device`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_pipeline_info`] **must**  be a valid pointer to a valid [`PipelineInfoKHR`] structure
    /// - [`p_executable_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_executable_count`] is not `0`, and [`p_properties`] is not
    ///   `NULL`, [`p_properties`] **must**  be a valid pointer to an array of
    ///   [`p_executable_count`][`PipelineExecutablePropertiesKHR`] structures
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_KHR_pipeline_executable_properties`]
    /// - [`Device`]
    /// - [`PipelineExecutablePropertiesKHR`]
    /// - [`PipelineInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPipelineExecutablePropertiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_pipeline_executable_properties_khr<'lt>(
        self: &Unique<Device>,
        p_pipeline_info: &PipelineInfoKHR<'lt>,
        p_executable_count: Option<usize>,
    ) -> VulkanResult<SmallVec<PipelineExecutablePropertiesKHR<'lt>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_pipeline_executable_properties()
            .and_then(|vtable| vtable.get_pipeline_executable_properties_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_pipeline_executable_properties()
            .and_then(|vtable| vtable.get_pipeline_executable_properties_khr())
            .unwrap_unchecked();
        let mut p_executable_count = match p_executable_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(
                    self.as_raw(),
                    p_pipeline_info as *const PipelineInfoKHR<'lt>,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            },
        };
        let mut p_properties = SmallVec::<PipelineExecutablePropertiesKHR<'lt>>::from_elem(
            Default::default(),
            p_executable_count as usize,
        );
        let _return = _function(
            self.as_raw(),
            p_pipeline_info as *const PipelineInfoKHR<'lt>,
            &mut p_executable_count,
            p_properties.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => VulkanResult::Success(_return, p_properties),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetPipelineExecutableStatisticsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html) - Get compile time statistics associated with a pipeline executable
    ///# C Specifications
    ///Each pipeline executable  **may**  have a set of statistics associated with it
    ///that are generated by the pipeline compilation process.
    ///These statistics  **may**  include things such as instruction counts, amount of
    ///spilling (if any), maximum number of simultaneous threads, or anything else
    ///which  **may**  aid developers in evaluating the expected performance of a
    ///shader.
    ///To query the compile-time statistics associated with a pipeline executable,
    ///call:
    ///```c
    ///// Provided by VK_KHR_pipeline_executable_properties
    ///VkResult vkGetPipelineExecutableStatisticsKHR(
    ///    VkDevice                                    device,
    ///    const VkPipelineExecutableInfoKHR*          pExecutableInfo,
    ///    uint32_t*                                   pStatisticCount,
    ///    VkPipelineExecutableStatisticKHR*           pStatistics);
    ///```
    ///# Parameters
    /// - [`device`] is the device that created the pipeline.
    /// - [`p_executable_info`] describes the pipeline executable being queried.
    /// - [`p_statistic_count`] is a pointer to an integer related to the number of statistics
    ///   available or queried, as described below.
    /// - [`p_statistics`] is either `NULL` or a pointer to an array of
    ///   [`PipelineExecutableStatisticKHR`] structures.
    ///# Description
    ///If [`p_statistics`] is `NULL`, then the number of statistics associated
    ///with the pipeline executable is returned in [`p_statistic_count`].
    ///Otherwise, [`p_statistic_count`] **must**  point to a variable set by the user
    ///to the number of elements in the [`p_statistics`] array, and on return the
    ///variable is overwritten with the number of structures actually written to
    ///[`p_statistics`].
    ///If [`p_statistic_count`] is less than the number of statistics associated
    ///with the pipeline executable, at most [`p_statistic_count`] structures will
    ///be written, and `VK_INCOMPLETE` will be returned instead of
    ///`VK_SUCCESS`, to indicate that not all the available statistics were
    ///returned.
    ///## Valid Usage
    /// - [`pipelineExecutableInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineExecutableInfo)
    ///   **must**  be enabled
    /// - `pipeline` member of [`p_executable_info`] **must**  have been created with [`device`]
    /// - `pipeline` member of [`p_executable_info`] **must**  have been created with
    ///   `VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_executable_info`] **must**  be a valid pointer to a valid
    ///   [`PipelineExecutableInfoKHR`] structure
    /// - [`p_statistic_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_statistic_count`] is not `0`, and [`p_statistics`] is not
    ///   `NULL`, [`p_statistics`] **must**  be a valid pointer to an array of
    ///   [`p_statistic_count`][`PipelineExecutableStatisticKHR`] structures
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_KHR_pipeline_executable_properties`]
    /// - [`Device`]
    /// - [`PipelineExecutableInfoKHR`]
    /// - [`PipelineExecutableStatisticKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPipelineExecutableStatisticsKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_pipeline_executable_statistics_khr<'lt>(
        self: &Unique<Device>,
        p_executable_info: &PipelineExecutableInfoKHR<'lt>,
        p_statistic_count: Option<usize>,
    ) -> VulkanResult<SmallVec<PipelineExecutableStatisticKHR<'lt>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_pipeline_executable_properties()
            .and_then(|vtable| vtable.get_pipeline_executable_statistics_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_pipeline_executable_properties()
            .and_then(|vtable| vtable.get_pipeline_executable_statistics_khr())
            .unwrap_unchecked();
        let mut p_statistic_count = match p_statistic_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(
                    self.as_raw(),
                    p_executable_info as *const PipelineExecutableInfoKHR<'lt>,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            },
        };
        let mut p_statistics =
            SmallVec::<PipelineExecutableStatisticKHR<'lt>>::from_elem(Default::default(), p_statistic_count as usize);
        let _return = _function(
            self.as_raw(),
            p_executable_info as *const PipelineExecutableInfoKHR<'lt>,
            &mut p_statistic_count,
            p_statistics.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => VulkanResult::Success(_return, p_statistics),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetPipelineExecutableInternalRepresentationsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html) - Get internal representations of the pipeline executable
    ///# C Specifications
    ///Each pipeline executable  **may**  have one or more text or binary internal
    ///representations associated with it which are generated as part of the
    ///compile process.
    ///These  **may**  include the final shader assembly, a binary form of the compiled
    ///shader, or the shader compiler’s internal representation at any number of
    ///intermediate compile steps.
    ///To query the internal representations associated with a pipeline executable,
    ///call:
    ///```c
    ///// Provided by VK_KHR_pipeline_executable_properties
    ///VkResult vkGetPipelineExecutableInternalRepresentationsKHR(
    ///    VkDevice                                    device,
    ///    const VkPipelineExecutableInfoKHR*          pExecutableInfo,
    ///    uint32_t*                                   pInternalRepresentationCount,
    ///    VkPipelineExecutableInternalRepresentationKHR* pInternalRepresentations);
    ///```
    ///# Parameters
    /// - [`device`] is the device that created the pipeline.
    /// - [`p_executable_info`] describes the pipeline executable being queried.
    /// - [`p_internal_representation_count`] is a pointer to an integer related to the number of
    ///   internal representations available or queried, as described below.
    /// - [`p_internal_representations`] is either `NULL` or a pointer to an array of
    ///   [`PipelineExecutableInternalRepresentationKHR`] structures.
    ///# Description
    ///If [`p_internal_representations`] is `NULL`, then the number of internal
    ///representations associated with the pipeline executable is returned in
    ///[`p_internal_representation_count`].
    ///Otherwise, [`p_internal_representation_count`] **must**  point to a variable set
    ///by the user to the number of elements in the [`p_internal_representations`]
    ///array, and on return the variable is overwritten with the number of
    ///structures actually written to [`p_internal_representations`].
    ///If [`p_internal_representation_count`] is less than the number of internal
    ///representations associated with the pipeline executable, at most
    ///[`p_internal_representation_count`] structures will be written, and
    ///`VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
    ///indicate that not all the available representations were returned.While the details of the
    /// internal representations remain
    ///implementation-dependent, the implementation  **should**  order the internal
    ///representations in the order in which they occur in the compiled pipeline
    ///with the final shader assembly (if any) last.
    ///## Valid Usage
    /// - [`pipelineExecutableInfo`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineExecutableInfo)
    ///   **must**  be enabled
    /// - `pipeline` member of [`p_executable_info`] **must**  have been created with [`device`]
    /// - `pipeline` member of [`p_executable_info`] **must**  have been created with
    ///   `VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_executable_info`] **must**  be a valid pointer to a valid
    ///   [`PipelineExecutableInfoKHR`] structure
    /// - [`p_internal_representation_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_internal_representation_count`] is not `0`, and
    ///   [`p_internal_representations`] is not `NULL`, [`p_internal_representations`] **must**  be
    ///   a valid pointer to an array of
    ///   [`p_internal_representation_count`][`PipelineExecutableInternalRepresentationKHR`]
    ///   structures
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`VK_KHR_pipeline_executable_properties`]
    /// - [`Device`]
    /// - [`PipelineExecutableInfoKHR`]
    /// - [`PipelineExecutableInternalRepresentationKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPipelineExecutableInternalRepresentationsKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_pipeline_executable_internal_representations_khr<'lt>(
        self: &Unique<Device>,
        p_executable_info: &PipelineExecutableInfoKHR<'lt>,
        p_internal_representation_count: Option<usize>,
    ) -> VulkanResult<SmallVec<PipelineExecutableInternalRepresentationKHR<'lt>>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_pipeline_executable_properties()
            .and_then(|vtable| vtable.get_pipeline_executable_internal_representations_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_pipeline_executable_properties()
            .and_then(|vtable| vtable.get_pipeline_executable_internal_representations_khr())
            .unwrap_unchecked();
        let mut p_internal_representation_count = match p_internal_representation_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(
                    self.as_raw(),
                    p_executable_info as *const PipelineExecutableInfoKHR<'lt>,
                    &mut v,
                    std::ptr::null_mut(),
                );
                v
            },
        };
        let mut p_internal_representations = SmallVec::<PipelineExecutableInternalRepresentationKHR<'lt>>::from_elem(
            Default::default(),
            p_internal_representation_count as usize,
        );
        let _return = _function(
            self.as_raw(),
            p_executable_info as *const PipelineExecutableInfoKHR<'lt>,
            &mut p_internal_representation_count,
            p_internal_representations.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => {
                VulkanResult::Success(_return, p_internal_representations)
            },
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_pipeline_executable_properties`
pub struct DeviceKhrPipelineExecutablePropertiesVTable {
    ///See [`FNGetPipelineExecutablePropertiesKhr`] for more information.
    pub get_pipeline_executable_properties_khr: FNGetPipelineExecutablePropertiesKhr,
    ///See [`FNGetPipelineExecutableStatisticsKhr`] for more information.
    pub get_pipeline_executable_statistics_khr: FNGetPipelineExecutableStatisticsKhr,
    ///See [`FNGetPipelineExecutableInternalRepresentationsKhr`] for more information.
    pub get_pipeline_executable_internal_representations_khr: FNGetPipelineExecutableInternalRepresentationsKhr,
}
impl DeviceKhrPipelineExecutablePropertiesVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_pipeline_executable_properties_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPipelineExecutablePropertiesKHR").as_ptr(),
                ))
            },
            get_pipeline_executable_statistics_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPipelineExecutableStatisticsKHR").as_ptr(),
                ))
            },
            get_pipeline_executable_internal_representations_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPipelineExecutableInternalRepresentationsKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_pipeline_executable_properties_khr`]. See
    /// [`FNGetPipelineExecutablePropertiesKhr`] for more information.
    pub fn get_pipeline_executable_properties_khr(&self) -> FNGetPipelineExecutablePropertiesKhr {
        self.get_pipeline_executable_properties_khr
    }
    ///Gets [`Self::get_pipeline_executable_statistics_khr`]. See
    /// [`FNGetPipelineExecutableStatisticsKhr`] for more information.
    pub fn get_pipeline_executable_statistics_khr(&self) -> FNGetPipelineExecutableStatisticsKhr {
        self.get_pipeline_executable_statistics_khr
    }
    ///Gets [`Self::get_pipeline_executable_internal_representations_khr`]. See
    /// [`FNGetPipelineExecutableInternalRepresentationsKhr`] for more information.
    pub fn get_pipeline_executable_internal_representations_khr(
        &self,
    ) -> FNGetPipelineExecutableInternalRepresentationsKhr {
        self.get_pipeline_executable_internal_representations_khr
    }
}
