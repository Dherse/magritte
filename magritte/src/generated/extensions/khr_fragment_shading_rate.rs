//![VK_KHR_fragment_shading_rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_fragment_shading_rate.html) - device extension
//!# Description
//!This extension adds the ability to change the rate at which fragments are
//!shaded.
//!Rather than the usual single fragment invocation for each pixel covered by a
//!primitive, multiple pixels can be shaded by a single fragment shader
//!invocation.Up to three methods are available to the application to change the fragment
//!shading rate:
//! - [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-pipeline),
//!   which allows the
//!specification of a rate per-draw.
//! - [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive),
//!   which allows the
//!specification of a rate per primitive, specified during shading.
//! - [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-attachment),
//!   which allows the
//!specification of a rate per-region of the framebuffer, specified in a
//!specialized image attachment.Additionally, these rates can all be specified and combined in
//! order to
//!adjust the overall detail in the image at each point.This functionality can be used to focus
//! shading efforts where higher levels
//!of detail are needed in some parts of a scene compared to others.
//!This can be particularly useful in high resolution rendering, or for XR
//!contexts.This extension also adds support for the `SPV_KHR_fragment_shading_rate`
//!extension which enables setting the
//![primitive fragment shading
//!rate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-fragment-shading-rate-primitive), and allows querying the final shading rate from a fragment shader.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_create_renderpass2`]`
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_fragment_shading_rate]
//!   @tobski%0A<<Here describe the issue or question you have about the
//!   VK_KHR_fragment_shading_rate extension>>)
//!# New functions & commands
//! - [`CmdSetFragmentShadingRateKHR`]
//! - [`GetPhysicalDeviceFragmentShadingRatesKHR`]
//!# New structures
//! - [`PhysicalDeviceFragmentShadingRateKHR`]
//! - Extending [`GraphicsPipelineCreateInfo`]:
//! - [`PipelineFragmentShadingRateStateCreateInfoKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceFragmentShadingRateFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceFragmentShadingRatePropertiesKHR`]
//! - Extending [`SubpassDescription2`]:
//! - [`FragmentShadingRateAttachmentInfoKHR`]
//!# New enums
//! - [`FragmentShadingRateCombinerOpKHR`]
//!# New constants
//! - [`KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME`]
//! - [`KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:
//! - `VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR`
//! - Extending [`DynamicState`]:
//! - `VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR`
//! - Extending [`FormatFeatureFlagBits`]:
//! - `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
//! - Extending [`ImageLayout`]:
//! - `VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR`
//! - Extending [`ImageUsageFlagBits`]:
//! - `VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
//! - Extending [`PipelineStageFlagBits`]:
//! - `VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR`
//! - `VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR`If
//!   [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:
//! - `VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
//!# Version History
//! - Revision 1, 2020-05-06 (Tobias Hector)
//! - Initial revision
//! - Revision 2, 2021-09-30 (Jon Leech)
//! - Add interaction with `[`VK_KHR_format_feature_flags2`]` to `vk.xml`
//!# Other info
//! * 2021-09-30
//!*
//! - This extension requires
//![`SPV_KHR_fragment_shading_rate`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_fragment_shading_rate.html).
//! - This extension provides API support for
//![`GL_EXT_fragment_shading_rate`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_fragment_shading_rate.txt)
//!*
//! - Tobias Hector, AMD
//! - Guennadi Riguer, AMD
//! - Matthaeus Chajdas, AMD
//! - Pat Brown, Nvidia
//! - Matthew Netsch, Qualcomm
//! - Slawomir Grajewski, Intel
//! - Jan-Harald Fredriksen, Arm
//! - Jeff Bolz, Nvidia
//! - Arseny Kapoulkine, Roblox
//! - Contributors to the VK_NV_shading_rate_image specification
//! - Contributors to the VK_EXT_fragment_density_map specification
//!# Related
//! - [`FragmentShadingRateAttachmentInfoKHR`]
//! - [`FragmentShadingRateCombinerOpKHR`]
//! - [`PhysicalDeviceFragmentShadingRateFeaturesKHR`]
//! - [`PhysicalDeviceFragmentShadingRateKHR`]
//! - [`PhysicalDeviceFragmentShadingRatePropertiesKHR`]
//! - [`PipelineFragmentShadingRateStateCreateInfoKHR`]
//! - [`CmdSetFragmentShadingRateKHR`]
//! - [`GetPhysicalDeviceFragmentShadingRatesKHR`]
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
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION")]
pub const KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME")]
pub const KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_fragment_shading_rate");
///[VkFragmentShadingRateCombinerOpKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateCombinerOpKHR.html) - Control how fragment shading rates are combined
///# C Specifications
///The equation used for each combiner operation is defined by
///[`FragmentShadingRateCombinerOpKHR`]:
///```c
///// Provided by VK_KHR_fragment_shading_rate
///typedef enum VkFragmentShadingRateCombinerOpKHR {
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR = 0,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR = 1,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR = 2,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR = 3,
///    VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR = 4,
///} VkFragmentShadingRateCombinerOpKHR;
///```
///# Description
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP`] specifies a combiner
///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>.
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE`] specifies a
///combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = B<sub>xy</sub>.
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_MIN`] specifies a combiner
///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = min(A<sub>xy</sub>,B<sub>xy</sub>).
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_MAX`] specifies a combiner
///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = max(A<sub>xy</sub>,B<sub>xy</sub>).
/// - [`FRAGMENT_SHADING_RATE_COMBINER_OP_MUL`] specifies a combiner
///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>*B<sub>xy</sub>.where
/// combine(A<sub>xy</sub>,B<sub>xy</sub>) is the combine operation, and A<sub>xy</sub>
///and B<sub>xy</sub> are the inputs to the operation.If [`fragmentShadingRateStrictMultiplyCombiner`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateStrictMultiplyCombiner) is [`FALSE`], using
///[`FRAGMENT_SHADING_RATE_COMBINER_OP_MUL`] with values of 1 for both
///A and B in the same dimension results in the value 2 being produced for that
///dimension.
///See the definition of [`fragmentShadingRateStrictMultiplyCombiner`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-fragmentShadingRateStrictMultiplyCombiner) for more information.These operations are performed in a component-wise fashion.
///# Related
/// - [`VK_KHR_fragment_shading_rate`]
/// - [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
/// - [`PipelineFragmentShadingRateStateCreateInfoKHR`]
/// - [`CmdSetFragmentShadingRateEnumNV`]
/// - [`CmdSetFragmentShadingRateKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFragmentShadingRateCombinerOpKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct FragmentShadingRateCombinerOpKHR(i32);
impl const Default for FragmentShadingRateCombinerOpKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for FragmentShadingRateCombinerOpKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("FragmentShadingRateCombinerOpKHR")
            .field(match *self {
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP => &"FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP",
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE => &"FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE",
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_MIN => &"FRAGMENT_SHADING_RATE_COMBINER_OP_MIN",
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_MAX => &"FRAGMENT_SHADING_RATE_COMBINER_OP_MAX",
                Self::FRAGMENT_SHADING_RATE_COMBINER_OP_MUL => &"FRAGMENT_SHADING_RATE_COMBINER_OP_MUL",
                other => unreachable!("invalid value for `FragmentShadingRateCombinerOpKHR`: {:?}", other),
            })
            .finish()
    }
}
impl FragmentShadingRateCombinerOpKHR {
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>.
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP: Self = Self(0);
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE`] specifies a
    ///combiner operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = B<sub>xy</sub>.
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE: Self = Self(1);
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_MIN`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = min(A<sub>xy</sub>,B<sub>xy</sub>).
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_MIN: Self = Self(2);
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_MAX`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = max(A<sub>xy</sub>,B<sub>xy</sub>).
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_MAX: Self = Self(3);
    ///[`FRAGMENT_SHADING_RATE_COMBINER_OP_MUL`] specifies a combiner
    ///operation of combine(A<sub>xy</sub>,B<sub>xy</sub>) = A<sub>xy</sub>*B<sub>xy</sub>.
    pub const FRAGMENT_SHADING_RATE_COMBINER_OP_MUL: Self = Self(4);
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
