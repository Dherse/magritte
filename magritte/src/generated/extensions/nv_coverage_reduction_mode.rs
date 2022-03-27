use crate::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, SampleCountFlagBits, SampleCountFlags, StructureType,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION")]
pub const NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME")]
pub const NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_coverage_reduction_mode");
///[VkCoverageReductionModeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageReductionModeNV.html) - Specify the coverage reduction mode
///# C Specifications
///Possible values of
///[`PipelineCoverageReductionStateCreateInfoNV::coverage_reduction_mode`],
///specifying how color sample coverage is generated from pixel coverage, are:
///```c
///// Provided by VK_NV_coverage_reduction_mode
///typedef enum VkCoverageReductionModeNV {
///    VK_COVERAGE_REDUCTION_MODE_MERGE_NV = 0,
///    VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV = 1,
///} VkCoverageReductionModeNV;
///```
///# Description
/// - [`CoverageReductionModeMergeNv`] specifies that each color sample will be associated with an
///   implementation-dependent subset of samples in the pixel coverage. If any of those associated
///   samples are covered, the color sample is covered.
/// - [`CoverageReductionModeTruncateNv`] specifies that for color samples present in the color attachments, a color sample is covered if the pixel coverage sample with the same [sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask)i is covered; other pixel coverage samples are discarded.
///# Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`FramebufferMixedSamplesCombinationNV`]
/// - [`PipelineCoverageReductionStateCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCoverageReductionModeNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum CoverageReductionModeNV {
    ///[`CoverageReductionModeMergeNv`] specifies that each color
    ///sample will be associated with an implementation-dependent subset of
    ///samples in the pixel coverage.
    ///If any of those associated samples are covered, the color sample is
    ///covered.
    CoverageReductionModeMergeNv = 0,
    ///[`CoverageReductionModeTruncateNv`] specifies that for color
    ///samples present in the color attachments, a color sample is covered if
    ///the pixel coverage sample with the same
    ///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask)i is
    ///covered; other pixel coverage samples are discarded.
    CoverageReductionModeTruncateNv = 1,
}
impl const Default for CoverageReductionModeNV {
    fn default() -> Self {
        CoverageReductionModeMergeNv
    }
}
impl CoverageReductionModeNV {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkPhysicalDeviceCoverageReductionModeFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCoverageReductionModeFeaturesNV.html) - Structure describing the coverage reduction mode features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCoverageReductionModeFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_coverage_reduction_mode
///typedef struct VkPhysicalDeviceCoverageReductionModeFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           coverageReductionMode;
///} VkPhysicalDeviceCoverageReductionModeFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`coverage_reduction_mode`] indicates whether the implementation supports coverage reduction modes. See [Coverage Reduction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-coverage-reduction).
///If the [`PhysicalDeviceCoverageReductionModeFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceCoverageReductionModeFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV`
///# Related
/// - [`VK_NV_coverage_reduction_mode`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`coverage_reduction_mode`] indicates
    ///whether the implementation supports coverage reduction modes.
    ///See [Coverage Reduction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-coverage-reduction).
    coverage_reduction_mode: Bool32,
}
///[VkPipelineCoverageReductionStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageReductionStateCreateInfoNV.html) - Structure specifying parameters controlling coverage reduction
///# C Specifications
///The [`PipelineCoverageReductionStateCreateInfoNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_coverage_reduction_mode
///typedef struct VkPipelineCoverageReductionStateCreateInfoNV {
///    VkStructureType                                  sType;
///    const void*                                      pNext;
///    VkPipelineCoverageReductionStateCreateFlagsNV    flags;
///    VkCoverageReductionModeNV                        coverageReductionMode;
///} VkPipelineCoverageReductionStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`coverage_reduction_mode`] is a [`CoverageReductionModeNV`] value controlling how color
///   sample coverage is generated from pixel coverage.
///# Description
///If this structure is not included in the [`p_next`] chain, or if the
///extension is not enabled, the default coverage reduction mode is inferred as
///follows:
/// - If the `[`VK_NV_framebuffer_mixed_samples`]` extension is enabled, then it is as if the
///   [`coverage_reduction_mode`] is `VK_COVERAGE_REDUCTION_MODE_MERGE_NV`.
/// - If the `[`VK_AMD_mixed_attachment_samples`]` extension is enabled, then it is as if the
///   [`coverage_reduction_mode`] is `VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV`.
/// - If both `[`VK_NV_framebuffer_mixed_samples`]` and `[`VK_AMD_mixed_attachment_samples`]` are
///   enabled, then the default coverage reduction mode is implementation-dependent.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV`
/// - [`flags`]**must** be `0`
/// - [`coverage_reduction_mode`]**must** be a valid [`CoverageReductionModeNV`] value
///# Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`CoverageReductionModeNV`]
/// - [`PipelineCoverageReductionStateCreateFlagsNV`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: PipelineCoverageReductionStateCreateFlagsNV,
    ///[`coverage_reduction_mode`] is a [`CoverageReductionModeNV`] value
    ///controlling how color sample coverage is generated from pixel coverage.
    coverage_reduction_mode: CoverageReductionModeNV,
}
///[VkFramebufferMixedSamplesCombinationNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferMixedSamplesCombinationNV.html) - Structure specifying a supported sample count combination
///# C Specifications
///The [`FramebufferMixedSamplesCombinationNV`] structure is defined as:
///```c
///// Provided by VK_NV_coverage_reduction_mode
///typedef struct VkFramebufferMixedSamplesCombinationNV {
///    VkStructureType              sType;
///    void*                        pNext;
///    VkCoverageReductionModeNV    coverageReductionMode;
///    VkSampleCountFlagBits        rasterizationSamples;
///    VkSampleCountFlags           depthStencilSamples;
///    VkSampleCountFlags           colorSamples;
///} VkFramebufferMixedSamplesCombinationNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`coverage_reduction_mode`] is a [`CoverageReductionModeNV`] value specifying the coverage
///   reduction mode.
/// - [`rasterization_samples`] is a [`SampleCountFlagBits`] specifying the number of rasterization
///   samples in the supported combination.
/// - [`depth_stencil_samples`] specifies the number of samples in the depth stencil attachment in
///   the supported combination. A value of 0 indicates the combination does not have a depth
///   stencil attachment.
/// - [`color_samples`] specifies the number of color samples in a color attachment in the supported
///   combination. A value of 0 indicates the combination does not have a color attachment.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`CoverageReductionModeNV`]
/// - [`SampleCountFlagBits`]
/// - [`SampleCountFlags`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`coverage_reduction_mode`] is a [`CoverageReductionModeNV`] value
    ///specifying the coverage reduction mode.
    coverage_reduction_mode: CoverageReductionModeNV,
    ///[`rasterization_samples`] is a [`SampleCountFlagBits`] specifying
    ///the number of rasterization samples in the supported combination.
    rasterization_samples: SampleCountFlagBits,
    ///[`depth_stencil_samples`] specifies the number of samples in the depth
    ///stencil attachment in the supported combination.
    ///A value of 0 indicates the combination does not have a depth stencil
    ///attachment.
    depth_stencil_samples: SampleCountFlags,
    ///[`color_samples`] specifies the number of color samples in a color
    ///attachment in the supported combination.
    ///A value of 0 indicates the combination does not have a color attachment.
    color_samples: SampleCountFlags,
}
