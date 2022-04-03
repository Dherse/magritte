//![VK_NV_coverage_reduction_mode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_coverage_reduction_mode.html) - device extension
//!# Description
//!When using a framebuffer with mixed samples, a per-fragment coverage
//!reduction operation is performed which generates color sample coverage from
//!the pixel coverage.
//!This extension defines the following modes to control how this reduction is
//!performed.
//! - Merge: When there are more samples in the pixel coverage than color samples, there is an
//!   implementation-dependent association of each pixel coverage sample to a color sample. In the
//!   merge mode, the color sample coverage is computed such that only if any associated sample in
//!   the pixel coverage is covered, the color sample is covered. This is the default mode.
//! - Truncate: When there are more raster samples (N) than color samples(M), there is one to one
//!   association of the first M raster samples to the M color samples; other raster samples are
//!   ignored.
//!When the number of raster samples is equal to the color samples, there is a
//!one to one mapping between them in either of the above modes.The new command
//![`get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`] can
//!be used to query the various raster, color, depth/stencil sample count and
//!reduction mode combinations that are supported by the implementation.
//!This extension would allow an implementation to support the behavior of both
//![`VK_NV_framebuffer_mixed_samples`] and [`VK_AMD_mixed_attachment_samples`]
//!extensions simultaneously.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_NV_framebuffer_mixed_samples`]`
//!# Contacts
//! - Kedarnath Thangudu [kthangudu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_coverage_reduction_mode]
//!   @kthangudu%0A<<Here describe the issue or question you have about the
//!   VK_NV_coverage_reduction_mode extension>>)
//!# New functions & commands
//! - [`get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`]
//!# New structures
//! - [`FramebufferMixedSamplesCombinationNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceCoverageReductionModeFeaturesNV`]
//! - Extending [`PipelineMultisampleStateCreateInfo`]:  -
//!   [`PipelineCoverageReductionStateCreateInfoNV`]
//!# New enums
//! - [`CoverageReductionModeNV`]
//!# New bitmasks
//! - [`PipelineCoverageReductionStateCreateFlagsNV`]
//!# New constants
//! - [`NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME`]
//! - [`NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV`
//!   - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV`
//!# Version History
//! - Revision 1, 2019-01-29 (Kedarnath Thangudu)  - Internal revisions
//!# Other info
//! * 2019-01-29
//! * - Kedarnath Thangudu, NVIDIA  - Jeff Bolz, NVIDIA
//!# Related
//! - [`CoverageReductionModeNV`]
//! - [`FramebufferMixedSamplesCombinationNV`]
//! - [`PhysicalDeviceCoverageReductionModeFeaturesNV`]
//! - [`PipelineCoverageReductionStateCreateFlagsNV`]
//! - [`PipelineCoverageReductionStateCreateInfoNV`]
//! - [`get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, Instance, PhysicalDevice, SampleCountFlagBits, SampleCountFlags,
    StructureType, VulkanResultCodes,
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
///[vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html) - Query supported sample count combinations
///# C Specifications
///To query the set of mixed sample combinations of coverage reduction mode,
///rasterization samples and color, depth, stencil attachment sample counts
///that are supported by a physical device, call:
///```c
///// Provided by VK_NV_coverage_reduction_mode
///VkResult vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t*                                   pCombinationCount,
///    VkFramebufferMixedSamplesCombinationNV*     pCombinations);
///```
/// # Parameters
/// - [`physical_device`] is the physical device from which to query the set of combinations.
/// - [`p_combination_count`] is a pointer to an integer related to the number of combinations
///   available or queried, as described below.
/// - [`p_combinations`] is either `NULL` or a pointer to an array of
///   [`FramebufferMixedSamplesCombinationNV`] values, indicating the supported combinations of
///   coverage reduction mode, rasterization samples, and color, depth, stencil attachment sample
///   counts.
/// # Description
/// If [`p_combinations`] is `NULL`, then the number of supported combinations
/// for the given [`physical_device`] is returned in [`p_combination_count`].
/// Otherwise, [`p_combination_count`] **must**  point to a variable set by the user
/// to the number of elements in the [`p_combinations`] array, and on return
/// the variable is overwritten with the number of values actually written to
/// [`p_combinations`].
/// If the value of [`p_combination_count`] is less than the number of
/// combinations supported for the given [`physical_device`], at most
/// [`p_combination_count`] values will be written to [`p_combinations`], and
/// `VK_INCOMPLETE` will be returned instead of `VK_SUCCESS`, to
/// indicate that not all the supported values were returned.
/// ## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_combination_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_combination_count`] is not `0`, and [`p_combinations`] is not
///   `NULL`, [`p_combinations`] **must**  be a valid pointer to an array of
///   [`p_combination_count`][`FramebufferMixedSamplesCombinationNV`] structures
///
/// ## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`FramebufferMixedSamplesCombinationNV`]
/// - [`PhysicalDevice`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV")]
pub type FNGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNv = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV<'lt>,
    ) -> VulkanResultCodes,
>;
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
/// # Description
/// - [`CoverageReductionModeMergeNv`] specifies that each color sample will be associated with an
///   implementation-dependent subset of samples in the pixel coverage. If any of those associated
///   samples are covered, the color sample is covered.
/// - [`CoverageReductionModeTruncateNv`] specifies that for color samples present in the color attachments, a color sample is covered if the pixel coverage sample with the same [sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask)i is covered; other pixel coverage samples are discarded.
/// # Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`FramebufferMixedSamplesCombinationNV`]
/// - [`PipelineCoverageReductionStateCreateInfoNV`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCoverageReductionModeNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
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
        Self::CoverageReductionModeMergeNv
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
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkPipelineCoverageReductionStateCreateFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageReductionStateCreateFlagsNV.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_NV_coverage_reduction_mode
///typedef VkFlags VkPipelineCoverageReductionStateCreateFlagsNV;
///```
/// # Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`PipelineCoverageReductionStateCreateInfoNV`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineCoverageReductionStateCreateFlagsNV(u32);
impl const Default for PipelineCoverageReductionStateCreateFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PipelineCoverageReductionStateCreateFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PipelineCoverageReductionStateCreateFlagsNV))
            .field(&self.0)
            .finish()
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
/// # Members
/// This structure describes the following feature:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`coverage_reduction_mode`] indicates whether the implementation supports coverage reduction modes. See [Coverage Reduction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-coverage-reduction).
/// If the [`PhysicalDeviceCoverageReductionModeFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceCoverageReductionModeFeaturesNV`] **can**  also be used in the [`p_next`] chain
/// of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV`
/// # Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceCoverageReductionModeFeaturesNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`coverage_reduction_mode`] indicates
    ///whether the implementation supports coverage reduction modes.
    ///See [Coverage Reduction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-coverage-reduction).
    pub coverage_reduction_mode: Bool32,
}
impl<'lt> Default for PhysicalDeviceCoverageReductionModeFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceCoverageReductionModeFeaturesNv,
            p_next: std::ptr::null_mut(),
            coverage_reduction_mode: 0,
        }
    }
}
impl<'lt> PhysicalDeviceCoverageReductionModeFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::coverage_reduction_mode`]
    pub fn coverage_reduction_mode_raw(&self) -> Bool32 {
        self.coverage_reduction_mode
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::coverage_reduction_mode`]
    pub fn set_coverage_reduction_mode_raw(&mut self, value: Bool32) -> &mut Self {
        self.coverage_reduction_mode = value;
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
    ///Gets the value of [`Self::coverage_reduction_mode`]
    pub fn coverage_reduction_mode(&self) -> bool {
        unsafe { std::mem::transmute(self.coverage_reduction_mode as u8) }
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
    ///Gets a mutable reference to the value of [`Self::coverage_reduction_mode`]
    pub fn coverage_reduction_mode_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.coverage_reduction_mode as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.coverage_reduction_mode as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::coverage_reduction_mode`]
    pub fn set_coverage_reduction_mode(&mut self, value: bool) -> &mut Self {
        self.coverage_reduction_mode = value as u8 as u32;
        self
    }
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`coverage_reduction_mode`] is a [`CoverageReductionModeNV`] value controlling how color
///   sample coverage is generated from pixel coverage.
/// # Description
/// If this structure is not included in the [`p_next`] chain, or if the
/// extension is not enabled, the default coverage reduction mode is inferred as
/// follows:
/// - If the `[`VK_NV_framebuffer_mixed_samples`]` extension is enabled, then it is as if the
///   [`coverage_reduction_mode`] is `VK_COVERAGE_REDUCTION_MODE_MERGE_NV`.
/// - If the `[`VK_AMD_mixed_attachment_samples`]` extension is enabled, then it is as if the
///   [`coverage_reduction_mode`] is `VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV`.
/// - If both `[`VK_NV_framebuffer_mixed_samples`]` and `[`VK_AMD_mixed_attachment_samples`]` are
///   enabled, then the default coverage reduction mode is implementation-dependent.
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV`
/// - [`flags`] **must**  be `0`
/// - [`coverage_reduction_mode`] **must**  be a valid [`CoverageReductionModeNV`] value
/// # Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`CoverageReductionModeNV`]
/// - [`PipelineCoverageReductionStateCreateFlagsNV`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineCoverageReductionStateCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineCoverageReductionStateCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: PipelineCoverageReductionStateCreateFlagsNV,
    ///[`coverage_reduction_mode`] is a [`CoverageReductionModeNV`] value
    ///controlling how color sample coverage is generated from pixel coverage.
    pub coverage_reduction_mode: CoverageReductionModeNV,
}
impl<'lt> Default for PipelineCoverageReductionStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PipelineCoverageReductionStateCreateInfoNv,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coverage_reduction_mode: Default::default(),
        }
    }
}
impl<'lt> PipelineCoverageReductionStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> PipelineCoverageReductionStateCreateFlagsNV {
        self.flags
    }
    ///Gets the value of [`Self::coverage_reduction_mode`]
    pub fn coverage_reduction_mode(&self) -> CoverageReductionModeNV {
        self.coverage_reduction_mode
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineCoverageReductionStateCreateFlagsNV {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::coverage_reduction_mode`]
    pub fn coverage_reduction_mode_mut(&mut self) -> &mut CoverageReductionModeNV {
        &mut self.coverage_reduction_mode
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::nv_coverage_reduction_mode::PipelineCoverageReductionStateCreateFlagsNV,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::coverage_reduction_mode`]
    pub fn set_coverage_reduction_mode(
        &mut self,
        value: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
    ) -> &mut Self {
        self.coverage_reduction_mode = value;
        self
    }
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
/// # Members
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
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`VK_NV_coverage_reduction_mode`]
/// - [`CoverageReductionModeNV`]
/// - [`SampleCountFlagBits`]
/// - [`SampleCountFlags`]
/// - [`StructureType`]
/// - [`get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFramebufferMixedSamplesCombinationNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FramebufferMixedSamplesCombinationNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`coverage_reduction_mode`] is a [`CoverageReductionModeNV`] value
    ///specifying the coverage reduction mode.
    pub coverage_reduction_mode: CoverageReductionModeNV,
    ///[`rasterization_samples`] is a [`SampleCountFlagBits`] specifying
    ///the number of rasterization samples in the supported combination.
    pub rasterization_samples: SampleCountFlagBits,
    ///[`depth_stencil_samples`] specifies the number of samples in the depth
    ///stencil attachment in the supported combination.
    ///A value of 0 indicates the combination does not have a depth stencil
    ///attachment.
    pub depth_stencil_samples: SampleCountFlags,
    ///[`color_samples`] specifies the number of color samples in a color
    ///attachment in the supported combination.
    ///A value of 0 indicates the combination does not have a color attachment.
    pub color_samples: SampleCountFlags,
}
impl<'lt> Default for FramebufferMixedSamplesCombinationNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::FramebufferMixedSamplesCombinationNv,
            p_next: std::ptr::null_mut(),
            coverage_reduction_mode: Default::default(),
            rasterization_samples: Default::default(),
            depth_stencil_samples: Default::default(),
            color_samples: Default::default(),
        }
    }
}
impl<'lt> FramebufferMixedSamplesCombinationNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::coverage_reduction_mode`]
    pub fn coverage_reduction_mode(&self) -> CoverageReductionModeNV {
        self.coverage_reduction_mode
    }
    ///Gets the value of [`Self::rasterization_samples`]
    pub fn rasterization_samples(&self) -> SampleCountFlagBits {
        self.rasterization_samples
    }
    ///Gets the value of [`Self::depth_stencil_samples`]
    pub fn depth_stencil_samples(&self) -> SampleCountFlags {
        self.depth_stencil_samples
    }
    ///Gets the value of [`Self::color_samples`]
    pub fn color_samples(&self) -> SampleCountFlags {
        self.color_samples
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
    ///Gets a mutable reference to the value of [`Self::coverage_reduction_mode`]
    pub fn coverage_reduction_mode_mut(&mut self) -> &mut CoverageReductionModeNV {
        &mut self.coverage_reduction_mode
    }
    ///Gets a mutable reference to the value of [`Self::rasterization_samples`]
    pub fn rasterization_samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.rasterization_samples
    }
    ///Gets a mutable reference to the value of [`Self::depth_stencil_samples`]
    pub fn depth_stencil_samples_mut(&mut self) -> &mut SampleCountFlags {
        &mut self.depth_stencil_samples
    }
    ///Gets a mutable reference to the value of [`Self::color_samples`]
    pub fn color_samples_mut(&mut self) -> &mut SampleCountFlags {
        &mut self.color_samples
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::coverage_reduction_mode`]
    pub fn set_coverage_reduction_mode(
        &mut self,
        value: crate::extensions::nv_coverage_reduction_mode::CoverageReductionModeNV,
    ) -> &mut Self {
        self.coverage_reduction_mode = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_samples`]
    pub fn set_rasterization_samples(&mut self, value: crate::vulkan1_0::SampleCountFlagBits) -> &mut Self {
        self.rasterization_samples = value;
        self
    }
    ///Sets the raw value of [`Self::depth_stencil_samples`]
    pub fn set_depth_stencil_samples(&mut self, value: crate::vulkan1_0::SampleCountFlags) -> &mut Self {
        self.depth_stencil_samples = value;
        self
    }
    ///Sets the raw value of [`Self::color_samples`]
    pub fn set_color_samples(&mut self, value: crate::vulkan1_0::SampleCountFlags) -> &mut Self {
        self.color_samples = value;
        self
    }
}
///The V-table of [`Instance`] for functions from VK_NV_coverage_reduction_mode
pub struct InstanceNvCoverageReductionModeVTable {
    ///See [`FNGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNv`] for more
    /// information.
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        FNGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNv,
}
impl InstanceNvCoverageReductionModeVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Instance) -> Self
    where
        F: Fn(Instance, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV"),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_supported_framebuffer_mixed_samples_combinations_nv`]. See
    /// [`FNGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNv`] for more information.
    pub fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        &self,
    ) -> FNGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNv {
        self.get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
    }
}
