//![VK_NV_fragment_coverage_to_color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_coverage_to_color.html) - device extension
//!# Description
//!This extension allows the fragment coverage value, represented as an integer
//!bitmask, to be substituted for a color output being written to a
//!single-component color attachment with integer components (e.g.
//!`VK_FORMAT_R8_UINT`).
//!The functionality provided by this extension is different from simply
//!writing the [`SampleMask`] fragment shader output, in that the coverage
//!value written to the framebuffer is taken after stencil test and depth test,
//!as well as after fragment operations such as alpha-to-coverage.This functionality may be useful
//! for deferred rendering algorithms, where
//!the second pass needs to know which samples belong to which original
//!fragments.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_fragment_coverage_to_color]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_NV_fragment_coverage_to_color extension>>)
//!# New structures
//! - Extending [`PipelineMultisampleStateCreateInfo`]:  -
//!   [`PipelineCoverageToColorStateCreateInfoNV`]
//!# New bitmasks
//! - [`PipelineCoverageToColorStateCreateFlagsNV`]
//!# New constants
//! - [`NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME`]
//! - [`NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV`
//!# Version history
//! - Revision 1, 2017-05-21 (Jeff Bolz)  - Internal revisions
//!# Other information
//! * 2017-05-21
//! * - Jeff Bolz, NVIDIA
//!# Related
//! - [`PipelineCoverageToColorStateCreateFlagsNV`]
//! - [`PipelineCoverageToColorStateCreateInfoNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_fragment_coverage_to_color");
///[VkPipelineCoverageToColorStateCreateFlagsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateFlagsNV.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_NV_fragment_coverage_to_color
///typedef VkFlags VkPipelineCoverageToColorStateCreateFlagsNV;
///```
/// # Related
/// - [`nv_fragment_coverage_to_color`]
/// - [`PipelineCoverageToColorStateCreateInfoNV`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineCoverageToColorStateCreateFlagsNV(u32);
impl const Default for PipelineCoverageToColorStateCreateFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PipelineCoverageToColorStateCreateFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PipelineCoverageToColorStateCreateFlagsNV))
            .field(&self.0)
            .finish()
    }
}
///[VkPipelineCoverageToColorStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html) - Structure specifying whether fragment coverage replaces a color
///# C Specifications
///The [`PipelineCoverageToColorStateCreateInfoNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_fragment_coverage_to_color
///typedef struct VkPipelineCoverageToColorStateCreateInfoNV {
///    VkStructureType                                sType;
///    const void*                                    pNext;
///    VkPipelineCoverageToColorStateCreateFlagsNV    flags;
///    VkBool32                                       coverageToColorEnable;
///    uint32_t                                       coverageToColorLocation;
///} VkPipelineCoverageToColorStateCreateInfoNV;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`coverage_to_color_enable`] controls whether the fragment coverage value replaces a fragment
///   color output.
/// - [`coverage_to_color_location`] controls which fragment shader color output value is replaced.
/// # Description
/// If the [`p_next`] chain of [`PipelineMultisampleStateCreateInfo`]
/// includes a [`PipelineCoverageToColorStateCreateInfoNV`] structure, then
/// that structure controls whether the fragment coverage is substituted for a
/// fragment color output and, if so, which output is replaced.If [`coverage_to_color_enable`] is
/// [`TRUE`], the
/// [coverage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) replaces the first
/// component of the color value corresponding to the fragment shader output
/// location with `Location` equal to [`coverage_to_color_location`] and
/// `Index` equal to zero.
/// If the color attachment format has fewer bits than the coverage mask, the
/// low bits of the sample coverage mask are taken without any clamping.
/// If the color attachment format has more bits than the coverage mask, the
/// high bits of the sample coverage mask are filled with zeros.If [`coverage_to_color_enable`] is
/// [`FALSE`], these operations are
/// skipped.
/// If this structure is not included in the [`p_next`] chain, it is as if
/// [`coverage_to_color_enable`] is [`FALSE`].
/// ## Valid Usage
/// - If [`coverage_to_color_enable`] is [`TRUE`], then the render pass subpass indicated by
///   [`GraphicsPipelineCreateInfo::render_pass`] and [`GraphicsPipelineCreateInfo::subpass`]
///   **must**  have a color attachment at the location selected by [`coverage_to_color_location`],
///   with a [`Format`] of `VK_FORMAT_R8_UINT`, `VK_FORMAT_R8_SINT`, `VK_FORMAT_R16_UINT`,
///   `VK_FORMAT_R16_SINT`, `VK_FORMAT_R32_UINT`, or `VK_FORMAT_R32_SINT`
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV`
/// - [`flags`] **must**  be `0`
/// # Related
/// - [`nv_fragment_coverage_to_color`]
/// - [`Bool32`]
/// - [`PipelineCoverageToColorStateCreateFlagsNV`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineCoverageToColorStateCreateInfoNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineCoverageToColorStateCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: PipelineCoverageToColorStateCreateFlagsNV,
    ///[`coverage_to_color_enable`] controls whether the fragment coverage value
    ///replaces a fragment color output.
    pub coverage_to_color_enable: Bool32,
    ///[`coverage_to_color_location`] controls which fragment shader color
    ///output value is replaced.
    pub coverage_to_color_location: u32,
}
impl<'lt> Default for PipelineCoverageToColorStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
            coverage_to_color_enable: 0,
            coverage_to_color_location: 0,
        }
    }
}
impl<'lt> PipelineCoverageToColorStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::coverage_to_color_enable`]
    pub fn coverage_to_color_enable_raw(&self) -> Bool32 {
        self.coverage_to_color_enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::coverage_to_color_enable`]
    pub fn set_coverage_to_color_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.coverage_to_color_enable = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::coverage_to_color_enable`]
    pub fn with_coverage_to_color_enable_raw(mut self, value: Bool32) -> Self {
        self.coverage_to_color_enable = value;
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
    pub fn flags(&self) -> PipelineCoverageToColorStateCreateFlagsNV {
        self.flags
    }
    ///Gets the value of [`Self::coverage_to_color_enable`]
    pub fn coverage_to_color_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.coverage_to_color_enable as u8) }
    }
    ///Gets the value of [`Self::coverage_to_color_location`]
    pub fn coverage_to_color_location(&self) -> u32 {
        self.coverage_to_color_location
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineCoverageToColorStateCreateFlagsNV {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::coverage_to_color_enable`]
    pub fn coverage_to_color_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.coverage_to_color_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.coverage_to_color_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::coverage_to_color_location`]
    pub fn coverage_to_color_location_mut(&mut self) -> &mut u32 {
        &mut self.coverage_to_color_location
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateFlagsNV,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::coverage_to_color_enable`]
    pub fn set_coverage_to_color_enable(&mut self, value: bool) -> &mut Self {
        self.coverage_to_color_enable = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::coverage_to_color_location`]
    pub fn set_coverage_to_color_location(&mut self, value: u32) -> &mut Self {
        self.coverage_to_color_location = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(
        mut self,
        value: crate::extensions::nv_fragment_coverage_to_color::PipelineCoverageToColorStateCreateFlagsNV,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::coverage_to_color_enable`]
    pub fn with_coverage_to_color_enable(mut self, value: bool) -> Self {
        self.coverage_to_color_enable = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::coverage_to_color_location`]
    pub fn with_coverage_to_color_location(mut self, value: u32) -> Self {
        self.coverage_to_color_location = value;
        self
    }
}
