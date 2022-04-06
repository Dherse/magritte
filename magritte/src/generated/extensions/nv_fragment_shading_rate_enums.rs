//![VK_NV_fragment_shading_rate_enums](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_shading_rate_enums.html) - device extension
//!# Description
//!This extension builds on the fragment shading rate functionality provided by
//!the VK_KHR_fragment_shading_rate extension, adding support for
//!“supersample” fragment shading rates that trigger multiple fragment shader
//!invocations per pixel as well as a “no invocations” shading rate that
//!discards any portions of a primitive that would use that shading rate.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_fragment_shading_rate`]`
//!# Contacts
//! - Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_fragment_shading_rate_enums]
//!   @nvpbrown%0A<<Here describe the issue or question you have about the
//!   VK_NV_fragment_shading_rate_enums extension>>)
//!# New functions & commands
//! - [`cmd_set_fragment_shading_rate_enum_nv`]
//!# New structures
//! - Extending [`GraphicsPipelineCreateInfo`]:  -
//!   [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`]
//! - Extending [`PhysicalDeviceProperties2`]:  -
//!   [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`]
//!# New enums
//! - [`FragmentShadingRateNV`]
//! - [`FragmentShadingRateTypeNV`]
//!# New constants
//! - [`NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME`]
//! - [`NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!0. Why was this extension created?   How should it be named? **RESOLVED** : The primary goal of
//! this extension was to expose support for supersample and “no invocations” shading rates, which
//! are supported by the VK_NV_shading_rate_image extension but not by VK_KHR_fragment_shading_rate.
//! Because VK_KHR_fragment_shading_rate specifies the primitive shading rate using a fragment size
//! in pixels, it lacks a good way to specify supersample rates. To deal with this, we defined enums
//! covering shading rates supported by the KHR extension as well as the new shading rates and added
//! structures and APIs accepting shading rate enums instead of fragment sizes.Since this extension
//! adds two different types of shading rates, both expressed using enums, we chose the extension
//! name VK_NV_fragment_shading_rate_enums.
//!1. Is this a standalone extension? **RESOLVED** : No, this extension requires
//! VK_KHR_fragment_shading_rate. In order to use the features of this extension, applications must
//! enable the relevant features of KHR extension.
//!2. How are the shading rate enums used, and how were the enum values assigned? **RESOLVED** :
//! The shading rates supported by the enums in this extension are accepted as pipeline, primitive,
//! and attachment shading rates and behave identically. For the shading rates also supported by the
//! KHR extension, the values assigned to the corresponding enums are identical to the values
//! already used for the primitive and attachment shading rates in the KHR extension. For those
//! enums, bits 0 and 1 specify the base two logarithm of the fragment height and bits 2 and 3
//! specify the base two logarithm of the fragment width. For the new shading rates added by this
//! extension, we chose to use 11 through 14 (10 plus the base two logarithm of the invocation
//! count) for the supersample rates and 15 for the “no invocations” rate. None of those values are
//! supported as primitive or attachment shading rates by the KHR extension.
//!3. Between this extension, VK_KHR_fragment_shading_rate, and VK_NV_shading_rate_image, there are
//! three different ways to specify shading rate state in a pipeline. How should we handle this?
//! **RESOLVED** : We do not allow the concurrent use of VK_NV_shading_rate_image and
//! VK_KHR_fragment_shading_rate; it is an error to enable shading rate features from both
//! extensions. But we do allow applications to enable this extension together with
//! VK_KHR_fragment_shading_rate together. While we expect that applications will never attach
//! pipeline CreateInfo structures for both this extension and the KHR extension concurrently,
//! Vulkan does not have any precedent forbidding such behavior and instead typically treats a
//! pipeline created without an extension-specific CreateInfo structure as equivalent to one
//! containing default values specified by the extension. Rather than adding such a rule considering
//! the presence or absence of our new CreateInfo structure, we instead included a `shadingRateType`
//! member to [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] that selects between using state
//! specified by that structure and state specified by
//! [`PipelineFragmentShadingRateStateCreateInfoKHR`].
//!# Version History
//! - Revision 1, 2020-09-02 (pbrown)  - Internal revisions
//!# Other info
//! * 2020-09-02
//! * - Pat Brown, NVIDIA  - Jeff Bolz, NVIDIA
//!# Related
//! - [`FragmentShadingRateNV`]
//! - [`FragmentShadingRateTypeNV`]
//! - [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`]
//! - [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`]
//! - [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
//! - [`cmd_set_fragment_shading_rate_enum_nv`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Device, SampleCountFlagBits, StructureType},
    AsRaw, Unique,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_fragment_shading_rate_enums");
///[vkCmdSetFragmentShadingRateEnumNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html) - Set pipeline fragment shading rate dynamically for a command buffer using enums
///# C Specifications
///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the pipeline fragment
///shading rate and combiner operation, call:
///```c
///// Provided by VK_NV_fragment_shading_rate_enums
///void vkCmdSetFragmentShadingRateEnumNV(
///    VkCommandBuffer                             commandBuffer,
///    VkFragmentShadingRateNV                     shadingRate,
///    const VkFragmentShadingRateCombinerOpKHR    combinerOps[2]);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`shading_rate`] specifies a [`FragmentShadingRateNV`] enum indicating the pipeline fragment
///   shading rate for subsequent drawing commands.
/// - [`combiner_ops`] specifies a [`FragmentShadingRateCombinerOpKHR`] determining how the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline),
///   [primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive),
///   and [attachment shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment)
///   are [combined](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-combining)
///   for fragments generated by subsequent drawing commands.
///# Description
///This command sets the pipeline fragment shading rate and combiner operation
///for subsequent drawing commands when the graphics pipeline is created with
///`VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR` set in
///[`PipelineDynamicStateCreateInfo::dynamic_states`].
///Otherwise, this state is specified by the
///[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] values used to
///create the currently active pipeline.
///## Valid Usage
/// - If [`pipelineFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineFragmentShadingRate)
///   is not enabled, [`shading_rate`] **must**  be
///   `VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV`
/// - If [`supersampleFragmentShadingRates`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-supersampleFragmentShadingRates)
///   is not enabled, [`shading_rate`] **must**  not be
///   `VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV`,
///   `VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV`,
///   `VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV`, or
///   `VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV`
/// - If [`noInvocationFragmentShadingRates`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-noInvocationFragmentShadingRates)
///   is not enabled, [`shading_rate`] **must**  not be `VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV`
/// - [`fragmentShadingRateEnums`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-fragmentShadingRateEnums)
///   **must**  be enabled
/// - One of [`pipelineFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineFragmentShadingRate),
///   [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate),
///   or [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate)
///   **must**  be enabled
/// - If the [`primitiveFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate)
///   is not enabled, [`combiner_ops`][0]  **must**  be
///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`
/// - If the [`attachmentFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate)
///   is not enabled, [`combiner_ops`][1]  **must**  be
///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`
/// - If the [`fragmentSizeNonTrivialCombinerOps`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateNonTrivialCombinerOps)
///   limit is not supported, elements of [`combiner_ops`] **must**  be either
///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR` or
///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR`
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`shading_rate`] **must**  be a valid [`FragmentShadingRateNV`] value
/// - Any given element of [`combiner_ops`] **must**  be a valid
///   [`FragmentShadingRateCombinerOpKHR`] value
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_NV_fragment_shading_rate_enums`]
/// - [`CommandBuffer`]
/// - [`FragmentShadingRateCombinerOpKHR`]
/// - [`FragmentShadingRateNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
pub type FNCmdSetFragmentShadingRateEnumNv = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
    ),
>;
///[VkFragmentShadingRateNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateNV.html) - Enumeration with fragment shading rates
///# C Specifications
///If the `fragmentShadingRateEnums` feature is enabled, fragment shading
///rates may be specified using the [`FragmentShadingRateNV`] enumerated
///type defined as:
///```c
///// Provided by VK_NV_fragment_shading_rate_enums
///typedef enum VkFragmentShadingRateNV {
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV = 0,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV = 1,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV = 4,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV = 5,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV = 6,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV = 9,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV = 10,
///    VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV = 11,
///    VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV = 12,
///    VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV = 13,
///    VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV = 14,
///    VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV = 15,
///} VkFragmentShadingRateNV;
///```
///# Description
/// - [`1_INVOCATION_PER_PIXEL`] specifies a fragment size of 1x1 pixels.
/// - [`1_INVOCATION_PER1_X_2_PIXELS`] specifies a fragment size of 1x2 pixels.
/// - [`1_INVOCATION_PER2_X_1_PIXELS`] specifies a fragment size of 2x1 pixels.
/// - [`1_INVOCATION_PER2_X_2_PIXELS`] specifies a fragment size of 2x2 pixels.
/// - [`1_INVOCATION_PER2_X_4_PIXELS`] specifies a fragment size of 2x4 pixels.
/// - [`1_INVOCATION_PER4_X_2_PIXELS`] specifies a fragment size of 4x2 pixels.
/// - [`1_INVOCATION_PER4_X_4_PIXELS`] specifies a fragment size of 4x4 pixels.
/// - [`2_INVOCATIONS_PER_PIXEL`] specifies a fragment size of 1x1 pixels, with two fragment shader
///   invocations per fragment.
/// - [`4_INVOCATIONS_PER_PIXEL`] specifies a fragment size of 1x1 pixels, with four fragment shader
///   invocations per fragment.
/// - [`8_INVOCATIONS_PER_PIXEL`] specifies a fragment size of 1x1 pixels, with eight fragment
///   shader invocations per fragment.
/// - [`16_INVOCATIONS_PER_PIXEL`] specifies a fragment size of 1x1 pixels, with sixteen fragment
///   shader invocations per fragment.
/// - [`NO_INVOCATIONS`] specifies that any portions of a primitive that use that shading rate
///   should be discarded without invoking any fragment shader.
///To use the shading rates
///[`2_INVOCATIONS_PER_PIXEL`],
///[`4_INVOCATIONS_PER_PIXEL`],
///[`8_INVOCATIONS_PER_PIXEL`], and
///[`16_INVOCATIONS_PER_PIXEL`] as a pipeline,
///primitive, or attachment shading rate, the
///`supersampleFragmentShadingRates` feature  **must**  be enabled.
///To use the shading rate [`NO_INVOCATIONS`] as
///a pipeline, primitive, or attachment shading rate, the
///`noInvocationFragmentShadingRates` feature  **must**  be enabled.
///# Related
/// - [`VK_NV_fragment_shading_rate_enums`]
/// - [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
/// - [`cmd_set_fragment_shading_rate_enum_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFragmentShadingRateNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct FragmentShadingRateNV(i32);
impl const Default for FragmentShadingRateNV {
    fn default() -> Self {
        Self(0)
    }
}
impl FragmentShadingRateNV {
    ///[`1_INVOCATION_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels.
    pub const _1_INVOCATION_PER_PIXEL: Self = Self(0);
    ///[`1_INVOCATION_PER1_X_2_PIXELS`] specifies
    ///a fragment size of 1x2 pixels.
    pub const _1_INVOCATION_PER1_X_2_PIXELS: Self = Self(1);
    ///[`1_INVOCATION_PER2_X_1_PIXELS`] specifies
    ///a fragment size of 2x1 pixels.
    pub const _1_INVOCATION_PER2_X_1_PIXELS: Self = Self(4);
    ///[`1_INVOCATION_PER2_X_2_PIXELS`] specifies
    ///a fragment size of 2x2 pixels.
    pub const _1_INVOCATION_PER2_X_2_PIXELS: Self = Self(5);
    ///[`1_INVOCATION_PER2_X_4_PIXELS`] specifies
    ///a fragment size of 2x4 pixels.
    pub const _1_INVOCATION_PER2_X_4_PIXELS: Self = Self(6);
    ///[`1_INVOCATION_PER4_X_2_PIXELS`] specifies
    ///a fragment size of 4x2 pixels.
    pub const _1_INVOCATION_PER4_X_2_PIXELS: Self = Self(9);
    ///[`1_INVOCATION_PER4_X_4_PIXELS`] specifies
    ///a fragment size of 4x4 pixels.
    pub const _1_INVOCATION_PER4_X_4_PIXELS: Self = Self(10);
    ///[`2_INVOCATIONS_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels, with two fragment shader invocations per
    ///fragment.
    pub const _2_INVOCATIONS_PER_PIXEL: Self = Self(11);
    ///[`4_INVOCATIONS_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels, with four fragment shader invocations per
    ///fragment.
    pub const _4_INVOCATIONS_PER_PIXEL: Self = Self(12);
    ///[`8_INVOCATIONS_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels, with eight fragment shader invocations per
    ///fragment.
    pub const _8_INVOCATIONS_PER_PIXEL: Self = Self(13);
    ///[`16_INVOCATIONS_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels, with sixteen fragment shader invocations
    ///per fragment.
    pub const _16_INVOCATIONS_PER_PIXEL: Self = Self(14);
    ///[`NO_INVOCATIONS`] specifies that any
    ///portions of a primitive that use that shading rate should be discarded
    ///without invoking any fragment shader.
    pub const NO_INVOCATIONS: Self = Self(15);
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
///[VkFragmentShadingRateTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateTypeNV.html) - Enumeration with fragment shading rate types
///# C Specifications
///The [`FragmentShadingRateTypeNV`] enumerated type specifies whether a
///graphics pipeline gets its pipeline fragment shading rates and combiners
///from the [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure
///or the [`PipelineFragmentShadingRateStateCreateInfoKHR`] structure.
///```c
///// Provided by VK_NV_fragment_shading_rate_enums
///typedef enum VkFragmentShadingRateTypeNV {
///    VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV = 0,
///    VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV = 1,
///} VkFragmentShadingRateTypeNV;
///```
///# Description
/// - [`FRAGMENT_SIZE`] specifies that a graphics pipeline should obtain its pipeline fragment
///   shading rate and shading rate combiner state from the
///   [`PipelineFragmentShadingRateStateCreateInfoKHR`] structure and that any state specified by
///   the [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure should be ignored.
/// - [`ENUMS`] specifies that a graphics pipeline should obtain its pipeline fragment shading rate
///   and shading rate combiner state from the [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
///   structure and that any state specified by the
///   [`PipelineFragmentShadingRateStateCreateInfoKHR`] structure should be ignored.
///# Related
/// - [`VK_NV_fragment_shading_rate_enums`]
/// - [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFragmentShadingRateTypeNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct FragmentShadingRateTypeNV(i32);
impl const Default for FragmentShadingRateTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl FragmentShadingRateTypeNV {
    ///[`FRAGMENT_SIZE`] specifies that a
    ///graphics pipeline should obtain its pipeline fragment shading rate and
    ///shading rate combiner state from the
    ///[`PipelineFragmentShadingRateStateCreateInfoKHR`] structure and that
    ///any state specified by the
    ///[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure
    ///should be ignored.
    pub const FRAGMENT_SIZE: Self = Self(0);
    ///[`ENUMS`] specifies that a graphics
    ///pipeline should obtain its pipeline fragment shading rate and shading
    ///rate combiner state from the
    ///[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure and
    ///that any state specified by the
    ///[`PipelineFragmentShadingRateStateCreateInfoKHR`] structure should
    ///be ignored.
    pub const ENUMS: Self = Self(1);
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
///[VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html) - Structure indicating support for fragment shading rate enums
///# C Specifications
///The [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_fragment_shading_rate_enums
///typedef struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fragmentShadingRateEnums;
///    VkBool32           supersampleFragmentShadingRates;
///    VkBool32           noInvocationFragmentShadingRates;
///} VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_shading_rate_enums`] indicates that the implementation supports specifying fragment
///   shading rates using the [`FragmentShadingRateNV`] enumerated type.
/// - [`supersample_fragment_shading_rates`] indicates that the implementation supports fragment
///   shading rate enum values indicating more than one invocation per fragment.
/// - [`no_invocation_fragment_shading_rates`] indicates that the implementation supports a fragment
///   shading rate enum value indicating that no fragment shaders should be invoked when that
///   shading rate is used.
///If the [`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentShadingRateEnumsFeaturesNV`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV`
///# Related
/// - [`VK_NV_fragment_shading_rate_enums`]
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
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`fragment_shading_rate_enums`]
    ///indicates that the implementation supports specifying fragment shading
    ///rates using the [`FragmentShadingRateNV`] enumerated type.
    pub fragment_shading_rate_enums: Bool32,
    ///[`supersample_fragment_shading_rates`] indicates that the implementation
    ///supports fragment shading rate enum values indicating more than one
    ///invocation per fragment.
    pub supersample_fragment_shading_rates: Bool32,
    ///[`no_invocation_fragment_shading_rates`] indicates that the implementation
    ///supports a fragment shading rate enum value indicating that no fragment
    ///shaders should be invoked when that shading rate is used.
    pub no_invocation_fragment_shading_rates: Bool32,
}
impl<'lt> Default for PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            fragment_shading_rate_enums: 0,
            supersample_fragment_shading_rates: 0,
            no_invocation_fragment_shading_rates: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentShadingRateEnumsFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::fragment_shading_rate_enums`]
    pub fn fragment_shading_rate_enums_raw(&self) -> Bool32 {
        self.fragment_shading_rate_enums
    }
    ///Gets the raw value of [`Self::supersample_fragment_shading_rates`]
    pub fn supersample_fragment_shading_rates_raw(&self) -> Bool32 {
        self.supersample_fragment_shading_rates
    }
    ///Gets the raw value of [`Self::no_invocation_fragment_shading_rates`]
    pub fn no_invocation_fragment_shading_rates_raw(&self) -> Bool32 {
        self.no_invocation_fragment_shading_rates
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shading_rate_enums`]
    pub fn set_fragment_shading_rate_enums_raw(mut self, value: Bool32) -> Self {
        self.fragment_shading_rate_enums = value;
        self
    }
    ///Sets the raw value of [`Self::supersample_fragment_shading_rates`]
    pub fn set_supersample_fragment_shading_rates_raw(mut self, value: Bool32) -> Self {
        self.supersample_fragment_shading_rates = value;
        self
    }
    ///Sets the raw value of [`Self::no_invocation_fragment_shading_rates`]
    pub fn set_no_invocation_fragment_shading_rates_raw(mut self, value: Bool32) -> Self {
        self.no_invocation_fragment_shading_rates = value;
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
    ///Gets the value of [`Self::fragment_shading_rate_enums`]
    pub fn fragment_shading_rate_enums(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shading_rate_enums as u8) }
    }
    ///Gets the value of [`Self::supersample_fragment_shading_rates`]
    pub fn supersample_fragment_shading_rates(&self) -> bool {
        unsafe { std::mem::transmute(self.supersample_fragment_shading_rates as u8) }
    }
    ///Gets the value of [`Self::no_invocation_fragment_shading_rates`]
    pub fn no_invocation_fragment_shading_rates(&self) -> bool {
        unsafe { std::mem::transmute(self.no_invocation_fragment_shading_rates as u8) }
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
    ///Gets a mutable reference to the value of [`Self::fragment_shading_rate_enums`]
    pub fn fragment_shading_rate_enums_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shading_rate_enums as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shading_rate_enums as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::supersample_fragment_shading_rates`]
    pub fn supersample_fragment_shading_rates_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.supersample_fragment_shading_rates as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.supersample_fragment_shading_rates as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::no_invocation_fragment_shading_rates`]
    pub fn no_invocation_fragment_shading_rates_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.no_invocation_fragment_shading_rates as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.no_invocation_fragment_shading_rates as *mut Bool32)
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
    ///Sets the value of [`Self::fragment_shading_rate_enums`]
    pub fn set_fragment_shading_rate_enums(mut self, value: bool) -> Self {
        self.fragment_shading_rate_enums = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::supersample_fragment_shading_rates`]
    pub fn set_supersample_fragment_shading_rates(mut self, value: bool) -> Self {
        self.supersample_fragment_shading_rates = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::no_invocation_fragment_shading_rates`]
    pub fn set_no_invocation_fragment_shading_rates(mut self, value: bool) -> Self {
        self.no_invocation_fragment_shading_rates = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html) - Structure describing fragment shading rate limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_fragment_shading_rate_enums
///typedef struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
///    VkStructureType          sType;
///    void*                    pNext;
///    VkSampleCountFlagBits    maxFragmentShadingRateInvocationCount;
///} VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_fragment_shading_rate_invocation_count`] is a [`SampleCountFlagBits`] value indicating
///   the maximum number of fragment shader invocations per fragment supported in pipeline,
///   primitive, and attachment fragment shading rates.
///# Description
///If the [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.These properties are related to [fragment
///shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate).
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV`
/// - [`max_fragment_shading_rate_invocation_count`] **must**  be a valid [`SampleCountFlagBits`]
///   value
///# Related
/// - [`VK_NV_fragment_shading_rate_enums`]
/// - [`SampleCountFlagBits`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_fragment_shading_rate_invocation_count`] is a
    ///[`SampleCountFlagBits`] value indicating the maximum number of
    ///fragment shader invocations per fragment supported in pipeline,
    ///primitive, and attachment fragment shading rates.
    pub max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
}
impl<'lt> Default for PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            max_fragment_shading_rate_invocation_count: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceFragmentShadingRateEnumsPropertiesNV<'lt> {
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
    ///Gets the value of [`Self::max_fragment_shading_rate_invocation_count`]
    pub fn max_fragment_shading_rate_invocation_count(&self) -> SampleCountFlagBits {
        self.max_fragment_shading_rate_invocation_count
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
    ///Gets a mutable reference to the value of
    /// [`Self::max_fragment_shading_rate_invocation_count`]
    pub fn max_fragment_shading_rate_invocation_count_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.max_fragment_shading_rate_invocation_count
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
    ///Sets the value of [`Self::max_fragment_shading_rate_invocation_count`]
    pub fn set_max_fragment_shading_rate_invocation_count(
        mut self,
        value: crate::vulkan1_0::SampleCountFlagBits,
    ) -> Self {
        self.max_fragment_shading_rate_invocation_count = value;
        self
    }
}
///[VkPipelineFragmentShadingRateEnumStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html) - Structure specifying parameters controlling the fragment shading rate using rate enums
///# C Specifications
///The [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_fragment_shading_rate_enums
///typedef struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkFragmentShadingRateTypeNV           shadingRateType;
///    VkFragmentShadingRateNV               shadingRate;
///    VkFragmentShadingRateCombinerOpKHR    combinerOps[2];
///} VkPipelineFragmentShadingRateEnumStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shading_rate_type`] specifies a [`FragmentShadingRateTypeNV`] value indicating whether
///   fragment shading rates are specified using fragment sizes or [`FragmentShadingRateNV`] enums.
/// - [`shading_rate`] specifies a [`FragmentShadingRateNV`] value indicating the pipeline fragment
///   shading rate.
/// - [`combiner_ops`] specifies [`FragmentShadingRateCombinerOpKHR`] values determining how the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline),
///   [primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive),
///   and [attachment shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment)
///   are [combined](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-combining)
///   for fragments generated by drawing commands using the created pipeline.
///# Description
///If the [`p_next`] chain of [`GraphicsPipelineCreateInfo`] includes a
///[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure, then
///that structure includes parameters controlling the pipeline fragment shading
///rate.If this structure is not present, [`shading_rate_type`] is considered to be
///equal to `VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV`,
///[`shading_rate`] is considered to be equal to
///`VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV`, and both elements
///of [`combiner_ops`] are considered to be equal to
///`VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV`
/// - [`shading_rate_type`] **must**  be a valid [`FragmentShadingRateTypeNV`] value
/// - [`shading_rate`] **must**  be a valid [`FragmentShadingRateNV`] value
/// - Any given element of [`combiner_ops`] **must**  be a valid
///   [`FragmentShadingRateCombinerOpKHR`] value
///# Related
/// - [`VK_NV_fragment_shading_rate_enums`]
/// - [`FragmentShadingRateCombinerOpKHR`]
/// - [`FragmentShadingRateNV`]
/// - [`FragmentShadingRateTypeNV`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineFragmentShadingRateEnumStateCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`shading_rate_type`] specifies a [`FragmentShadingRateTypeNV`]
    ///value indicating whether fragment shading rates are specified using
    ///fragment sizes or [`FragmentShadingRateNV`] enums.
    pub shading_rate_type: FragmentShadingRateTypeNV,
    ///[`shading_rate`] specifies a [`FragmentShadingRateNV`] value
    ///indicating the pipeline fragment shading rate.
    pub shading_rate: FragmentShadingRateNV,
    ///[`combiner_ops`] specifies [`FragmentShadingRateCombinerOpKHR`]
    ///values determining how the
    ///[pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline),
    ///[primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive), and
    ///[attachment shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment)
    ///are [combined](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-combining) for fragments
    ///generated by drawing commands using the created pipeline.
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
}
impl<'lt> Default for PipelineFragmentShadingRateEnumStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            shading_rate_type: Default::default(),
            shading_rate: Default::default(),
            combiner_ops: [Default::default(); 2 as usize],
        }
    }
}
impl<'lt> PipelineFragmentShadingRateEnumStateCreateInfoNV<'lt> {
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
    ///Gets the value of [`Self::shading_rate_type`]
    pub fn shading_rate_type(&self) -> FragmentShadingRateTypeNV {
        self.shading_rate_type
    }
    ///Gets the value of [`Self::shading_rate`]
    pub fn shading_rate(&self) -> FragmentShadingRateNV {
        self.shading_rate
    }
    ///Gets the value of [`Self::combiner_ops`]
    pub fn combiner_ops(&self) -> &[FragmentShadingRateCombinerOpKHR; 2 as usize] {
        &self.combiner_ops
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate_type`]
    pub fn shading_rate_type_mut(&mut self) -> &mut FragmentShadingRateTypeNV {
        &mut self.shading_rate_type
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate`]
    pub fn shading_rate_mut(&mut self) -> &mut FragmentShadingRateNV {
        &mut self.shading_rate
    }
    ///Gets a mutable reference to the value of [`Self::combiner_ops`]
    pub fn combiner_ops_mut(&mut self) -> &mut [FragmentShadingRateCombinerOpKHR; 2 as usize] {
        &mut self.combiner_ops
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
    ///Sets the value of [`Self::shading_rate_type`]
    pub fn set_shading_rate_type(
        mut self,
        value: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateTypeNV,
    ) -> Self {
        self.shading_rate_type = value;
        self
    }
    ///Sets the value of [`Self::shading_rate`]
    pub fn set_shading_rate(
        mut self,
        value: crate::extensions::nv_fragment_shading_rate_enums::FragmentShadingRateNV,
    ) -> Self {
        self.shading_rate = value;
        self
    }
    ///Sets the value of [`Self::combiner_ops`]
    pub fn set_combiner_ops(
        mut self,
        value: [crate::extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR; 2 as usize],
    ) -> Self {
        self.combiner_ops = value;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdSetFragmentShadingRateEnumNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html) - Set pipeline fragment shading rate dynamically for a command buffer using enums
    ///# C Specifications
    ///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the pipeline fragment
    ///shading rate and combiner operation, call:
    ///```c
    ///// Provided by VK_NV_fragment_shading_rate_enums
    ///void vkCmdSetFragmentShadingRateEnumNV(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkFragmentShadingRateNV                     shadingRate,
    ///    const VkFragmentShadingRateCombinerOpKHR    combinerOps[2]);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`shading_rate`] specifies a [`FragmentShadingRateNV`] enum indicating the pipeline
    ///   fragment shading rate for subsequent drawing commands.
    /// - [`combiner_ops`] specifies a [`FragmentShadingRateCombinerOpKHR`] determining how the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline),
    ///   [primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive),
    ///   and [attachment shading rates](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment)
    ///   are [combined](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-combining)
    ///   for fragments generated by subsequent drawing commands.
    ///# Description
    ///This command sets the pipeline fragment shading rate and combiner operation
    ///for subsequent drawing commands when the graphics pipeline is created with
    ///`VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR` set in
    ///[`PipelineDynamicStateCreateInfo::dynamic_states`].
    ///Otherwise, this state is specified by the
    ///[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] values used to
    ///create the currently active pipeline.
    ///## Valid Usage
    /// - If [`pipelineFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineFragmentShadingRate)
    ///   is not enabled, [`shading_rate`] **must**  be
    ///   `VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV`
    /// - If [`supersampleFragmentShadingRates`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-supersampleFragmentShadingRates)
    ///   is not enabled, [`shading_rate`] **must**  not be
    ///   `VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV`,
    ///   `VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV`,
    ///   `VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV`, or
    ///   `VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV`
    /// - If [`noInvocationFragmentShadingRates`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-noInvocationFragmentShadingRates)
    ///   is not enabled, [`shading_rate`] **must**  not be
    ///   `VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV`
    /// - [`fragmentShadingRateEnums`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-fragmentShadingRateEnums)
    ///   **must**  be enabled
    /// - One of [`pipelineFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineFragmentShadingRate),
    ///   [`primitiveFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate),
    ///   or [`attachmentFragmentShadingRate`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate)
    ///   **must**  be enabled
    /// - If the [`primitiveFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-primitiveFragmentShadingRate)
    ///   is not enabled, [`combiner_ops`][0]  **must**  be
    ///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`
    /// - If the [`attachmentFragmentShadingRate` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-attachmentFragmentShadingRate)
    ///   is not enabled, [`combiner_ops`][1]  **must**  be
    ///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR`
    /// - If the [`fragmentSizeNonTrivialCombinerOps`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateNonTrivialCombinerOps)
    ///   limit is not supported, elements of [`combiner_ops`] **must**  be either
    ///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR` or
    ///   `VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR`
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`shading_rate`] **must**  be a valid [`FragmentShadingRateNV`] value
    /// - Any given element of [`combiner_ops`] **must**  be a valid
    ///   [`FragmentShadingRateCombinerOpKHR`] value
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_NV_fragment_shading_rate_enums`]
    /// - [`CommandBuffer`]
    /// - [`FragmentShadingRateCombinerOpKHR`]
    /// - [`FragmentShadingRateNV`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv<'a: 'this, 'b: 'a + 'this, 'this>(
        self: &'this mut Unique<'a, 'b, CommandBuffer>,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .nv_fragment_shading_rate_enums()
            .and_then(|vtable| vtable.cmd_set_fragment_shading_rate_enum_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .nv_fragment_shading_rate_enums()
            .and_then(|vtable| vtable.cmd_set_fragment_shading_rate_enum_nv())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), shading_rate, combiner_ops);
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_NV_fragment_shading_rate_enums`
pub struct DeviceNvFragmentShadingRateEnumsVTable {
    ///See [`FNCmdSetFragmentShadingRateEnumNv`] for more information.
    pub cmd_set_fragment_shading_rate_enum_nv: FNCmdSetFragmentShadingRateEnumNv,
}
impl DeviceNvFragmentShadingRateEnumsVTable {
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
            cmd_set_fragment_shading_rate_enum_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdSetFragmentShadingRateEnumNV").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::cmd_set_fragment_shading_rate_enum_nv`]. See
    /// [`FNCmdSetFragmentShadingRateEnumNv`] for more information.
    pub fn cmd_set_fragment_shading_rate_enum_nv(&self) -> FNCmdSetFragmentShadingRateEnumNv {
        self.cmd_set_fragment_shading_rate_enum_nv
    }
}
