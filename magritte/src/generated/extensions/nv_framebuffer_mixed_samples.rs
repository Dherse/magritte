use crate::vulkan1_0::{BaseInStructure, Bool32, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_framebuffer_mixed_samples");
///[VkCoverageModulationModeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoverageModulationModeNV.html) - Specify the coverage modulation mode
///# C Specifications
///Possible values of
///[`PipelineCoverageModulationStateCreateInfoNV::coverage_modulation_mode`],
///specifying which color components are modulated, are:
///```c
///// Provided by VK_NV_framebuffer_mixed_samples
///typedef enum VkCoverageModulationModeNV {
///    VK_COVERAGE_MODULATION_MODE_NONE_NV = 0,
///    VK_COVERAGE_MODULATION_MODE_RGB_NV = 1,
///    VK_COVERAGE_MODULATION_MODE_ALPHA_NV = 2,
///    VK_COVERAGE_MODULATION_MODE_RGBA_NV = 3,
///} VkCoverageModulationModeNV;
///```
///# Description
/// - [`CoverageModulationModeNoneNv`] specifies that no components are multiplied by the modulation
///   factor.
/// - [`CoverageModulationModeRgbNv`] specifies that the red, green, and blue components are
///   multiplied by the modulation factor.
/// - [`CoverageModulationModeAlphaNv`] specifies that the alpha component is multiplied by the
///   modulation factor.
/// - [`CoverageModulationModeRgbaNv`] specifies that all components are multiplied by the
///   modulation factor.
///# Related
/// - [`VK_NV_framebuffer_mixed_samples`]
/// - [`PipelineCoverageModulationStateCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCoverageModulationModeNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum CoverageModulationModeNV {
    ///[`CoverageModulationModeNoneNv`] specifies that no components
    ///are multiplied by the modulation factor.
    CoverageModulationModeNoneNv = 0,
    ///[`CoverageModulationModeRgbNv`] specifies that the red, green,
    ///and blue components are multiplied by the modulation factor.
    CoverageModulationModeRgbNv = 1,
    ///[`CoverageModulationModeAlphaNv`] specifies that the alpha
    ///component is multiplied by the modulation factor.
    CoverageModulationModeAlphaNv = 2,
    ///[`CoverageModulationModeRgbaNv`] specifies that all components
    ///are multiplied by the modulation factor.
    CoverageModulationModeRgbaNv = 3,
}
impl const Default for CoverageModulationModeNV {
    fn default() -> Self {
        CoverageModulationModeNoneNv
    }
}
impl CoverageModulationModeNV {
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
///[VkPipelineCoverageModulationStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html) - Structure specifying parameters controlling coverage modulation
///# C Specifications
///As part of coverage reduction, fragment color values **can** also be modulated
///(multiplied) by a value that is a function of fraction of covered
///rasterization samples associated with that color sample.Pipeline state controlling coverage
/// modulation is specified through the
///members of the [`PipelineCoverageModulationStateCreateInfoNV`]
///structure.The [`PipelineCoverageModulationStateCreateInfoNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_framebuffer_mixed_samples
///typedef struct VkPipelineCoverageModulationStateCreateInfoNV {
///    VkStructureType                                   sType;
///    const void*                                       pNext;
///    VkPipelineCoverageModulationStateCreateFlagsNV    flags;
///    VkCoverageModulationModeNV                        coverageModulationMode;
///    VkBool32                                          coverageModulationTableEnable;
///    uint32_t                                          coverageModulationTableCount;
///    const float*                                      pCoverageModulationTable;
///} VkPipelineCoverageModulationStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`coverage_modulation_mode`] is a [`CoverageModulationModeNV`] value controlling which color
///   components are modulated.
/// - [`coverage_modulation_table_enable`] controls whether the modulation factor is looked up from
///   a table in [`p_coverage_modulation_table`].
/// - [`coverage_modulation_table_count`] is the number of elements in
///   [`p_coverage_modulation_table`].
/// - [`p_coverage_modulation_table`] is a table of modulation factors containing a value for each
///   number of covered samples.
///# Description
///If [`coverage_modulation_table_enable`] is [`FALSE`], then for each
///color sample the associated bits of the pixel coverage are counted and
///divided by the number of associated bits to produce a modulation factor
///R in the range (0,1] (a value of zero would have been killed due
///to a color coverage of 0).
///Specifically:
/// - N = value of `rasterizationSamples`
/// - M = value of [`AttachmentDescription::samples`] for any color attachments
/// - R = popcount(associated coverage bits) / (N / M)
///If [`coverage_modulation_table_enable`] is [`TRUE`], the value R
///is computed using a programmable lookup table.
///The lookup table has N / M elements, and the element of the table is
///selected by:
/// - R = [`p_coverage_modulation_table`][popcount(associated coverage bits)-1]
///Note that the table does not have an entry for popcount(associated
///coverage bits) = 0, because such samples would have been killed.The values of
/// [`p_coverage_modulation_table`]**may** be rounded to an
///implementation-dependent precision, which is at least as fine as 1 /
///N, and clamped to [0,1].For each color attachment with a floating point or normalized color
/// format,
///each fragment output color value is replicated to M values which **can**
///each be modulated (multiplied) by that color sample’s associated value of
///R.
///Which components are modulated is controlled by
///[`coverage_modulation_mode`].If this structure is not included in the [`p_next`] chain, it is as
/// if
///[`coverage_modulation_mode`] is `VK_COVERAGE_MODULATION_MODE_NONE_NV`.If the [coverage reduction mode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-coverage-reduction) is
///`VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV`, each color sample is
///associated with only a single coverage sample.
///In this case, it is as if [`coverage_modulation_mode`] is
///`VK_COVERAGE_MODULATION_MODE_NONE_NV`.Valid Usage
/// - If [`coverage_modulation_table_enable`] is [`TRUE`],
///   [`coverage_modulation_table_count`]**must** be equal to the number of rasterization samples
///   divided by the number of color samples in the subpass
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV`
/// - [`flags`]**must** be `0`
/// - [`coverage_modulation_mode`]**must** be a valid [`CoverageModulationModeNV`] value
///# Related
/// - [`VK_NV_framebuffer_mixed_samples`]
/// - [`Bool32`]
/// - [`CoverageModulationModeNV`]
/// - [`PipelineCoverageModulationStateCreateFlagsNV`]
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
pub struct PipelineCoverageModulationStateCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: PipelineCoverageModulationStateCreateFlagsNV,
    ///[`coverage_modulation_mode`] is a [`CoverageModulationModeNV`] value
    ///controlling which color components are modulated.
    coverage_modulation_mode: CoverageModulationModeNV,
    ///[`coverage_modulation_table_enable`] controls whether the modulation
    ///factor is looked up from a table in [`p_coverage_modulation_table`].
    coverage_modulation_table_enable: Bool32,
    ///[`coverage_modulation_table_count`] is the number of elements in
    ///[`p_coverage_modulation_table`].
    coverage_modulation_table_count: u32,
    ///[`p_coverage_modulation_table`] is a table of modulation factors
    ///containing a value for each number of covered samples.
    p_coverage_modulation_table: *mut f32,
}
