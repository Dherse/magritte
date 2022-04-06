//![VK_NV_framebuffer_mixed_samples](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_framebuffer_mixed_samples.html) - device extension
//!# Description
//!This extension allows multisample rendering with a raster and depth/stencil
//!sample count that is larger than the color sample count.
//!Rasterization and the results of the depth and stencil tests together
//!determine the portion of a pixel that is “covered”.
//!It can be useful to evaluate coverage at a higher frequency than color
//!samples are stored.
//!This coverage is then “reduced” to a collection of covered color samples,
//!each having an opacity value corresponding to the fraction of the color
//!sample covered.
//!The opacity can optionally be blended into individual color samples.Rendering with fewer color
//! samples than depth/stencil samples greatly
//!reduces the amount of memory and bandwidth consumed by the color buffer.
//!However, converting the coverage values into opacity introduces artifacts
//!where triangles share edges and  **may**  not be suitable for normal triangle
//!mesh rendering.One expected use case for this functionality is Stencil-then-Cover path
//!rendering (similar to the OpenGL GL_NV_path_rendering extension).
//!The stencil step determines the coverage (in the stencil buffer) for an
//!entire path at the higher sample frequency, and then the cover step draws
//!the path into the lower frequency color buffer using the coverage
//!information to antialias path edges.
//!With this two-step process, internal edges are fully covered when
//!antialiasing is applied and there is no corruption on these edges.The key features of this
//! extension are:
//! - It allows render pass and framebuffer objects to be created where the number of samples in the
//!   depth/stencil attachment in a subpass is a multiple of the number of samples in the color
//!   attachments in the subpass.
//! - A coverage reduction step is added to Fragment Operations which converts a set of covered
//!   raster/depth/stencil samples to a set of color samples that perform blending and color writes.
//!   The coverage reduction step also includes an optional coverage modulation step, multiplying
//!   color values by a fractional opacity corresponding to the number of associated
//!   raster/depth/stencil samples covered.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_framebuffer_mixed_samples]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_NV_framebuffer_mixed_samples extension>>)
//!# New structures
//! - Extending [`PipelineMultisampleStateCreateInfo`]:  -
//!   [`PipelineCoverageModulationStateCreateInfoNV`]
//!# New enums
//! - [`CoverageModulationModeNV`]
//!# New bitmasks
//! - [`PipelineCoverageModulationStateCreateFlagsNV`]
//!# New constants
//! - [`NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME`]
//! - [`NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV`
//!# Version History
//! - Revision 1, 2017-06-04 (Jeff Bolz)  - Internal revisions
//!# Other info
//! * 2017-06-04
//! * - Jeff Bolz, NVIDIA
//!# Related
//! - [`CoverageModulationModeNV`]
//! - [`PipelineCoverageModulationStateCreateFlagsNV`]
//! - [`PipelineCoverageModulationStateCreateInfoNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
/// - [`NONE`] specifies that no components are multiplied by the modulation factor.
/// - [`RGB`] specifies that the red, green, and blue components are multiplied by the modulation
///   factor.
/// - [`ALPHA`] specifies that the alpha component is multiplied by the modulation factor.
/// - [`RGBA`] specifies that all components are multiplied by the modulation factor.
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
#[repr(transparent)]
pub struct CoverageModulationModeNV(i32);
impl const Default for CoverageModulationModeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl CoverageModulationModeNV {
    ///[`NONE`] specifies that no components
    ///are multiplied by the modulation factor.
    pub const NONE: Self = Self(0);
    ///[`RGB`] specifies that the red, green,
    ///and blue components are multiplied by the modulation factor.
    pub const RGB: Self = Self(1);
    ///[`ALPHA`] specifies that the alpha
    ///component is multiplied by the modulation factor.
    pub const ALPHA: Self = Self(2);
    ///[`RGBA`] specifies that all components
    ///are multiplied by the modulation factor.
    pub const RGBA: Self = Self(3);
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
///[VkPipelineCoverageModulationStateCreateFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateFlagsNV.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_NV_framebuffer_mixed_samples
///typedef VkFlags VkPipelineCoverageModulationStateCreateFlagsNV;
///```
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineCoverageModulationStateCreateFlagsNV(u32);
impl const Default for PipelineCoverageModulationStateCreateFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PipelineCoverageModulationStateCreateFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PipelineCoverageModulationStateCreateFlagsNV))
            .field(&self.0)
            .finish()
    }
}
///[VkPipelineCoverageModulationStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageModulationStateCreateInfoNV.html) - Structure specifying parameters controlling coverage modulation
///# C Specifications
///As part of coverage reduction, fragment color values  **can**  also be modulated
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
///   a table in [`coverage_modulation_table`].
/// - [`coverage_modulation_table_count`] is the number of elements in
///   [`coverage_modulation_table`].
/// - [`coverage_modulation_table`] is a table of modulation factors containing a value for each
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
/// - R = [`coverage_modulation_table`][popcount(associated coverage bits)-1]
///Note that the table does not have an entry for popcount(associated
///coverage bits) = 0, because such samples would have been killed.The values of
/// [`coverage_modulation_table`] **may**  be rounded to an
///implementation-dependent precision, which is at least as fine as 1 /
///N, and clamped to [0,1].For each color attachment with a floating point or normalized color
/// format,
///each fragment output color value is replicated to M values which  **can**
///each be modulated (multiplied) by that color sample’s associated value of
///R.
///Which components are modulated is controlled by
///[`coverage_modulation_mode`].If this structure is not included in the [`p_next`] chain, it is as
/// if
///[`coverage_modulation_mode`] is `VK_COVERAGE_MODULATION_MODE_NONE_NV`.If the [coverage reduction mode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-coverage-reduction) is
///`VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV`, each color sample is
///associated with only a single coverage sample.
///In this case, it is as if [`coverage_modulation_mode`] is
///`VK_COVERAGE_MODULATION_MODE_NONE_NV`.
///## Valid Usage
/// - If [`coverage_modulation_table_enable`] is [`TRUE`], [`coverage_modulation_table_count`]
///   **must**  be equal to the number of rasterization samples divided by the number of color
///   samples in the subpass
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV`
/// - [`flags`] **must**  be `0`
/// - [`coverage_modulation_mode`] **must**  be a valid [`CoverageModulationModeNV`] value
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
#[doc(alias = "VkPipelineCoverageModulationStateCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineCoverageModulationStateCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    ///[`coverage_modulation_mode`] is a [`CoverageModulationModeNV`] value
    ///controlling which color components are modulated.
    pub coverage_modulation_mode: CoverageModulationModeNV,
    ///[`coverage_modulation_table_enable`] controls whether the modulation
    ///factor is looked up from a table in [`coverage_modulation_table`].
    pub coverage_modulation_table_enable: Bool32,
    ///[`coverage_modulation_table_count`] is the number of elements in
    ///[`coverage_modulation_table`].
    pub coverage_modulation_table_count: u32,
    ///[`coverage_modulation_table`] is a table of modulation factors
    ///containing a value for each number of covered samples.
    pub coverage_modulation_table: *const f32,
}
impl<'lt> Default for PipelineCoverageModulationStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coverage_modulation_mode: Default::default(),
            coverage_modulation_table_enable: 0,
            coverage_modulation_table_count: 0,
            coverage_modulation_table: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineCoverageModulationStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::coverage_modulation_table_enable`]
    pub fn coverage_modulation_table_enable_raw(&self) -> Bool32 {
        self.coverage_modulation_table_enable
    }
    ///Gets the raw value of [`Self::coverage_modulation_table`]
    pub fn coverage_modulation_table_raw(&self) -> *const f32 {
        self.coverage_modulation_table
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::coverage_modulation_table_enable`]
    pub fn set_coverage_modulation_table_enable_raw(mut self, value: Bool32) -> Self {
        self.coverage_modulation_table_enable = value;
        self
    }
    ///Sets the raw value of [`Self::coverage_modulation_table`]
    pub fn set_coverage_modulation_table_raw(mut self, value: *const f32) -> Self {
        self.coverage_modulation_table = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> PipelineCoverageModulationStateCreateFlagsNV {
        self.flags
    }
    ///Gets the value of [`Self::coverage_modulation_mode`]
    pub fn coverage_modulation_mode(&self) -> CoverageModulationModeNV {
        self.coverage_modulation_mode
    }
    ///Gets the value of [`Self::coverage_modulation_table_enable`]
    pub fn coverage_modulation_table_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.coverage_modulation_table_enable as u8) }
    }
    ///Gets the value of [`Self::coverage_modulation_table_count`]
    pub fn coverage_modulation_table_count(&self) -> u32 {
        self.coverage_modulation_table_count
    }
    ///Gets the value of [`Self::coverage_modulation_table`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn coverage_modulation_table(&self) -> &[f32] {
        std::slice::from_raw_parts(
            self.coverage_modulation_table,
            self.coverage_modulation_table_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineCoverageModulationStateCreateFlagsNV {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::coverage_modulation_mode`]
    pub fn coverage_modulation_mode_mut(&mut self) -> &mut CoverageModulationModeNV {
        &mut self.coverage_modulation_mode
    }
    ///Gets a mutable reference to the value of [`Self::coverage_modulation_table_enable`]
    pub fn coverage_modulation_table_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.coverage_modulation_table_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.coverage_modulation_table_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::coverage_modulation_table_count`]
    pub fn coverage_modulation_table_count_mut(&mut self) -> &mut u32 {
        &mut self.coverage_modulation_table_count
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
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        mut self,
        value: crate::extensions::nv_framebuffer_mixed_samples::PipelineCoverageModulationStateCreateFlagsNV,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::coverage_modulation_mode`]
    pub fn set_coverage_modulation_mode(
        mut self,
        value: crate::extensions::nv_framebuffer_mixed_samples::CoverageModulationModeNV,
    ) -> Self {
        self.coverage_modulation_mode = value;
        self
    }
    ///Sets the value of [`Self::coverage_modulation_table_enable`]
    pub fn set_coverage_modulation_table_enable(mut self, value: bool) -> Self {
        self.coverage_modulation_table_enable = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::coverage_modulation_table_count`]
    pub fn set_coverage_modulation_table_count(mut self, value: u32) -> Self {
        self.coverage_modulation_table_count = value;
        self
    }
    ///Sets the value of [`Self::coverage_modulation_table`]
    pub fn set_coverage_modulation_table(mut self, value: &'lt [f32]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.coverage_modulation_table = value.as_ptr();
        self.coverage_modulation_table_count = len_;
        self
    }
}
