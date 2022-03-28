//![VK_NV_clip_space_w_scaling](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_clip_space_w_scaling.html) - device extension
//!# Description
//!Virtual Reality (VR) applications often involve a post-processing step to
//!apply a “barrel” distortion to the rendered image to correct the
//!“pincushion” distortion introduced by the optics in a VR device.
//!The barrel distorted image has lower resolution along the edges compared to
//!the center.
//!Since the original image is rendered at high resolution, which is uniform
//!across the complete image, a lot of pixels towards the edges do not make it
//!to the final post-processed image.This extension provides a mechanism to render VR scenes at a
//! non-uniform
//!resolution, in particular a resolution that falls linearly from the center
//!towards the edges.
//!This is achieved by scaling the w coordinate of the vertices in the
//!clip space before perspective divide.
//!The clip space w coordinate of the vertices  **can**  be offset as of a
//!function of x and y coordinates as follows:w' = w +  Ax +  ByIn the intended use case for
//! viewport position scaling, an application
//!should use a set of four viewports, one for each of the four quadrants of a
//!Cartesian coordinate system.
//!Each viewport is set to the dimension of the image, but is scissored to the
//!quadrant it represents.
//!The application should specify A and B coefficients of the
//!w-scaling equation above, that have the same value, but different
//!signs, for each of the viewports.
//!The signs of A and B should match the signs of x and
//!y for the quadrant that they represent such that the value of w'
//!will always be greater than or equal to the original w value for the
//!entire image.
//!Since the offset to w, (Ax +  By), is always positive, and
//!increases with the absolute values of x and y, the effective
//!resolution will fall off linearly from the center of the image to its edges.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Eric Werness [ewerness-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_clip_space_w_scaling]
//!   @ewerness-nv%0A<<Here describe the issue or question you have about the
//!   VK_NV_clip_space_w_scaling extension>>)
//!# New functions & commands
//! - [`CmdSetViewportWScalingNV`]
//!# New structures
//! - [`ViewportWScalingNV`]
//! - Extending [`PipelineViewportStateCreateInfo`]:  -
//!   [`PipelineViewportWScalingStateCreateInfoNV`]
//!# New constants
//! - [`NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME`]
//! - [`NV_CLIP_SPACE_W_SCALING_SPEC_VERSION`]
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!1) Is the pipeline struct name too long? **RESOLVED** : It fits with the naming convention.2)
//! Separate W scaling section or fold into coordinate transformations? **RESOLVED** : Leaving it as
//! its own section for now.
//!# Version History
//! - Revision 1, 2017-02-15 (Eric Werness)  - Internal revisions
//!# Other info
//! * 2017-02-15
//! * - Eric Werness, NVIDIA  - Kedarnath Thangudu, NVIDIA
//!# Related
//! - [`PipelineViewportWScalingStateCreateInfoNV`]
//! - [`ViewportWScalingNV`]
//! - [`CmdSetViewportWScalingNV`]
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
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION")]
pub const NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME")]
pub const NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_clip_space_w_scaling");
///[VkViewportWScalingNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewportWScalingNV.html) - Structure specifying a viewport
///# C Specifications
///The [`ViewportWScalingNV`] structure is defined as:
///```c
///// Provided by VK_NV_clip_space_w_scaling
///typedef struct VkViewportWScalingNV {
///    float    xcoeff;
///    float    ycoeff;
///} VkViewportWScalingNV;
///```
///# Members
/// - [`xcoeff`] and [`ycoeff`] are the viewport’s W scaling factor for x and y respectively.
///# Related
/// - [`VK_NV_clip_space_w_scaling`]
/// - [`PipelineViewportWScalingStateCreateInfoNV`]
/// - [`CmdSetViewportWScalingNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkViewportWScalingNV")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ViewportWScalingNV {
    ///[`xcoeff`] and [`ycoeff`] are the viewport’s W scaling factor for x
    ///and y respectively.
    pub xcoeff: f32,
    ///No documentation found
    pub ycoeff: f32,
}
impl Default for ViewportWScalingNV {
    fn default() -> Self {
        Self {
            xcoeff: 0.0,
            ycoeff: 0.0,
        }
    }
}
impl ViewportWScalingNV {
    ///Gets the value of [`Self::xcoeff`]
    pub fn xcoeff(&self) -> f32 {
        self.xcoeff
    }
    ///Gets the value of [`Self::ycoeff`]
    pub fn ycoeff(&self) -> f32 {
        self.ycoeff
    }
    ///Gets a mutable reference to the value of [`Self::xcoeff`]
    pub fn xcoeff_mut(&mut self) -> &mut f32 {
        &mut self.xcoeff
    }
    ///Gets a mutable reference to the value of [`Self::ycoeff`]
    pub fn ycoeff_mut(&mut self) -> &mut f32 {
        &mut self.ycoeff
    }
    ///Sets the raw value of [`Self::xcoeff`]
    pub fn set_xcoeff(&mut self, value: f32) -> &mut Self {
        self.xcoeff = value;
        self
    }
    ///Sets the raw value of [`Self::ycoeff`]
    pub fn set_ycoeff(&mut self, value: f32) -> &mut Self {
        self.ycoeff = value;
        self
    }
}
///[VkPipelineViewportWScalingStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html) - Structure specifying parameters of a newly created pipeline viewport W scaling state
///# C Specifications
///The [`PipelineViewportWScalingStateCreateInfoNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_clip_space_w_scaling
///typedef struct VkPipelineViewportWScalingStateCreateInfoNV {
///    VkStructureType                sType;
///    const void*                    pNext;
///    VkBool32                       viewportWScalingEnable;
///    uint32_t                       viewportCount;
///    const VkViewportWScalingNV*    pViewportWScalings;
///} VkPipelineViewportWScalingStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`viewport_w_scaling_enable`] controls whether viewport  **W**  scaling is enabled.
/// - [`viewport_count`] is the number of viewports used by  **W**  scaling, and  **must**  match
///   the number of viewports in the pipeline if viewport  **W**  scaling is enabled.
/// - [`viewport_w_scalings`] is a pointer to an array of [`ViewportWScalingNV`] structures defining
///   the  **W**  scaling parameters for the corresponding viewports. If the viewport  **W**
///   scaling state is dynamic, this member is ignored.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV`
/// - [`viewport_count`] **must**  be greater than `0`
///# Related
/// - [`VK_NV_clip_space_w_scaling`]
/// - [`Bool32`]
/// - [`StructureType`]
/// - [`ViewportWScalingNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineViewportWScalingStateCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineViewportWScalingStateCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`viewport_w_scaling_enable`] controls whether viewport  **W**  scaling is
    ///enabled.
    pub viewport_w_scaling_enable: Bool32,
    ///[`viewport_count`] is the number of viewports used by  **W**  scaling, and
    /// **must**  match the number of viewports in the pipeline if viewport  **W**
    ///scaling is enabled.
    pub viewport_count: u32,
    ///[`viewport_w_scalings`] is a pointer to an array of
    ///[`ViewportWScalingNV`] structures defining the  **W**  scaling
    ///parameters for the corresponding viewports.
    ///If the viewport  **W**  scaling state is dynamic, this member is ignored.
    pub viewport_w_scalings: *const ViewportWScalingNV,
}
impl<'lt> Default for PipelineViewportWScalingStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            viewport_w_scaling_enable: 0,
            viewport_count: 0,
            viewport_w_scalings: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineViewportWScalingStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::viewport_w_scaling_enable`]
    pub fn viewport_w_scaling_enable_raw(&self) -> Bool32 {
        self.viewport_w_scaling_enable
    }
    ///Gets the raw value of [`Self::viewport_w_scalings`]
    pub fn viewport_w_scalings_raw(&self) -> *const ViewportWScalingNV {
        self.viewport_w_scalings
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_w_scaling_enable`]
    pub fn set_viewport_w_scaling_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.viewport_w_scaling_enable = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_w_scalings`]
    pub fn set_viewport_w_scalings_raw(&mut self, value: *const ViewportWScalingNV) -> &mut Self {
        self.viewport_w_scalings = value;
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
    ///Gets the value of [`Self::viewport_w_scaling_enable`]
    pub fn viewport_w_scaling_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.viewport_w_scaling_enable as u8) }
    }
    ///Gets the value of [`Self::viewport_count`]
    pub fn viewport_count(&self) -> u32 {
        self.viewport_count
    }
    ///Gets the value of [`Self::viewport_w_scalings`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn viewport_w_scalings(&self) -> &[ViewportWScalingNV] {
        std::slice::from_raw_parts(self.viewport_w_scalings, self.viewport_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::viewport_w_scaling_enable`]
    pub fn viewport_w_scaling_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.viewport_w_scaling_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.viewport_w_scaling_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::viewport_count`]
    pub fn viewport_count_mut(&mut self) -> &mut u32 {
        &mut self.viewport_count
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
    ///Sets the raw value of [`Self::viewport_w_scaling_enable`]
    pub fn set_viewport_w_scaling_enable(&mut self, value: bool) -> &mut Self {
        self.viewport_w_scaling_enable = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::viewport_count`]
    pub fn set_viewport_count(&mut self, value: u32) -> &mut Self {
        self.viewport_count = value;
        self
    }
    ///Sets the raw value of [`Self::viewport_w_scalings`]
    pub fn set_viewport_w_scalings(
        &mut self,
        value: &'lt [crate::extensions::nv_clip_space_w_scaling::ViewportWScalingNV],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.viewport_w_scalings = value.as_ptr();
        self.viewport_count = len_;
        self
    }
}
