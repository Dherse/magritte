//![VK_ARM_rasterization_order_attachment_access](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_ARM_rasterization_order_attachment_access.html) - device extension
//!# Description
//!Renderpasses, and specifically
//![subpass
//!self-dependencies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-barriers-subpass-self-dependencies) enable much of the same functionality as the framebuffer
//!fetch and pixel local storage extensions did for OpenGL ES.
//!But certain techniques such as programmable blending are awkward or
//!impractical to implement with these alone, in part because a self-dependency
//!is required every time a fragment will read a value at a given sample
//!coordinate.This extension extends the mechanism of input attachments to allow access to
//!framebuffer attachments when used as both input and color, or depth/stencil,
//!attachments from one fragment to the next, in rasterization order, without
//!explicit synchronization.See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more
//!information.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jan-Harald Fredriksen [janharaldfredriksen-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_ARM_rasterization_order_attachment_access]
//!   @janharaldfredriksen-arm%0A<<Here describe the issue or question you have about the
//!   VK_ARM_rasterization_order_attachment_access extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
//!# New enums
//! - [`PipelineColorBlendStateCreateFlagBits`]
//! - [`PipelineDepthStencilStateCreateFlagBits`]
//!# New constants
//! - [`ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME`]
//! - [`ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION`]
//! - Extending [`PipelineColorBlendStateCreateFlagBits`]:  -
//!   `VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM`
//! - Extending [`PipelineDepthStencilStateCreateFlagBits`]:  -
//!   `VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM`
//!   - `VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM`
//! - Extending [`SubpassDescriptionFlagBits`]:  -
//!   `VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM`  -
//!   `VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM`  -
//!   `VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM`
//!# Known issues & F.A.Q
//!1) Is there any interaction with the `[`VK_KHR_dynamic_rendering`]`
//!extension?No.
//!This extension only affects reads from input attachments.
//!Render pass instances begun with [`CmdBeginRenderingKHR`] do not have
//!input attachments and a different mechanism will be needed to provide
//!similar functionality in this case.
//!# Version History
//! - Revision 1, 2021-11-12 (Jan-Harald Fredriksen)  - Initial draft
//!# Other info
//! * 2021-11-12
//! * No known IP claims.
//! * - Tobias Hector, AMD  - Jan-Harald Fredriksen, Arm
//!# Related
//! - [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
//! - [`PipelineColorBlendStateCreateFlagBits`]
//! - [`PipelineDepthStencilStateCreateFlagBits`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_ARM_rasterization_order_attachment_access");
///[VkPipelineColorBlendStateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html) - Bitmask specifying additional parameters of an image
///# C Specifications
///Bits which  **can**  be set in the
///[`PipelineColorBlendStateCreateInfo::flags`] parameter are:
///```c
///// Provided by VK_ARM_rasterization_order_attachment_access
///typedef enum VkPipelineColorBlendStateCreateFlagBits {
///  // Provided by VK_ARM_rasterization_order_attachment_access
///    VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM =
/// 0x00000001,
///} VkPipelineColorBlendStateCreateFlagBits;
///```
///# Description
/// - [`RasterizationOrderAttachmentAccessArm`] indicates that access to color and input attachments
///   will have implicit framebuffer-local memory dependencies, allowing applications to express custom
///   blending operations in a fragment shader. See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop)
///   for more information.
///# Related
/// - [`VK_ARM_rasterization_order_attachment_access`]
/// - [`PipelineColorBlendStateCreateFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineColorBlendStateCreateFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum PipelineColorBlendStateCreateFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`RasterizationOrderAttachmentAccessArm`]
    ///indicates that access to color and input attachments will have implicit
    ///framebuffer-local memory dependencies, allowing applications to express
    ///custom blending operations in a fragment shader.
    ///See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more
    ///information.
    RasterizationOrderAttachmentAccessArm = 1,
}
impl const Default for PipelineColorBlendStateCreateFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl PipelineColorBlendStateCreateFlagBits {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkPipelineDepthStencilStateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html) - Bitmask specifying additional depth/stencil state information.
///# C Specifications
///Bits which  **can**  be set in the
///[`PipelineDepthStencilStateCreateInfo::flags`] parameter are:
///```c
///// Provided by VK_ARM_rasterization_order_attachment_access
///typedef enum VkPipelineDepthStencilStateCreateFlagBits {
///  // Provided by VK_ARM_rasterization_order_attachment_access
///    VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM =
/// 0x00000001,
///  // Provided by VK_ARM_rasterization_order_attachment_access
///    VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM
/// = 0x00000002,
///} VkPipelineDepthStencilStateCreateFlagBits;
///```
///# Description
/// - [`RasterizationOrderAttachmentDepthAccessArm`] indicates that access to the depth aspects of depth/stencil and input attachments will have implicit framebuffer-local memory dependencies. See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more information.
/// - [`RasterizationOrderAttachmentStencilAccessArm`] indicates that access to the stencil aspects of depth/stencil and input attachments will have implicit framebuffer-local memory dependencies. See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more information.
///# Related
/// - [`VK_ARM_rasterization_order_attachment_access`]
/// - [`PipelineDepthStencilStateCreateFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineDepthStencilStateCreateFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum PipelineDepthStencilStateCreateFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`RasterizationOrderAttachmentDepthAccessArm`]
    ///indicates that access to the depth aspects of depth/stencil and input
    ///attachments will have implicit framebuffer-local memory dependencies.
    ///See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more
    ///information.
    RasterizationOrderAttachmentDepthAccessArm = 1,
    ///[`RasterizationOrderAttachmentStencilAccessArm`]
    ///indicates that access to the stencil aspects of depth/stencil and input
    ///attachments will have implicit framebuffer-local memory dependencies.
    ///See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more
    ///information.
    RasterizationOrderAttachmentStencilAccessArm = 2,
}
impl const Default for PipelineDepthStencilStateCreateFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl PipelineDepthStencilStateCreateFlagBits {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM.html) - Structure describing whether rasterization order attachment access can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
///structure is defined as:
///```c
///// Provided by VK_ARM_rasterization_order_attachment_access
///typedef struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           rasterizationOrderColorAttachmentAccess;
///    VkBool32           rasterizationOrderDepthAttachmentAccess;
///    VkBool32           rasterizationOrderStencilAttachmentAccess;
///} VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM;
///```
///# Members
///The members of the
///[`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
///structure describe the following features:
///# Description
/// - [`rasterization_order_color_attachment_access`] indicates that rasterization order access to
///   color and input attachments is supported by the implementation.
/// - [`rasterization_order_depth_attachment_access`] indicates that rasterization order access to
///   the depth aspect of depth/stencil and input attachments is supported by the implementation.
/// - [`rasterization_order_stencil_attachment_access`] indicates that rasterization order access to
///   the stencil aspect of depth/stencil and input attachments is supported by the implementation.
///If the [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
///structure is included in the [`p_next`] chain of
///[`PhysicalDeviceFeatures2`], it is filled with values indicating whether
///the feature is supported.
///[`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`] **can**
///also be used in the [`p_next`] chain of [`DeviceCreateInfo`] to enable
///features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM`
///# Related
/// - [`VK_ARM_rasterization_order_attachment_access`]
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
#[doc(alias = "VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be
    /// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM`
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`rasterization_order_color_attachment_access`] indicates that
    ///rasterization order access to color and input attachments is supported
    ///by the implementation.
    pub rasterization_order_color_attachment_access: Bool32,
    ///[`rasterization_order_depth_attachment_access`] indicates that
    ///rasterization order access to the depth aspect of depth/stencil and
    ///input attachments is supported by the implementation.
    pub rasterization_order_depth_attachment_access: Bool32,
    ///[`rasterization_order_stencil_attachment_access`] indicates that
    ///rasterization order access to the stencil aspect of depth/stencil and
    ///input attachments is supported by the implementation.
    pub rasterization_order_stencil_attachment_access: Bool32,
}
impl<'lt> Default for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesArm,
            p_next: std::ptr::null_mut(),
            rasterization_order_color_attachment_access: 0,
            rasterization_order_depth_attachment_access: 0,
            rasterization_order_stencil_attachment_access: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::rasterization_order_color_attachment_access`]
    pub fn rasterization_order_color_attachment_access_raw(&self) -> Bool32 {
        self.rasterization_order_color_attachment_access
    }
    ///Gets the raw value of [`Self::rasterization_order_depth_attachment_access`]
    pub fn rasterization_order_depth_attachment_access_raw(&self) -> Bool32 {
        self.rasterization_order_depth_attachment_access
    }
    ///Gets the raw value of [`Self::rasterization_order_stencil_attachment_access`]
    pub fn rasterization_order_stencil_attachment_access_raw(&self) -> Bool32 {
        self.rasterization_order_stencil_attachment_access
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_color_attachment_access`]
    pub fn set_rasterization_order_color_attachment_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.rasterization_order_color_attachment_access = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_depth_attachment_access`]
    pub fn set_rasterization_order_depth_attachment_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.rasterization_order_depth_attachment_access = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_stencil_attachment_access`]
    pub fn set_rasterization_order_stencil_attachment_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.rasterization_order_stencil_attachment_access = value;
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
    ///Gets the value of [`Self::rasterization_order_color_attachment_access`]
    pub fn rasterization_order_color_attachment_access(&self) -> bool {
        unsafe { std::mem::transmute(self.rasterization_order_color_attachment_access as u8) }
    }
    ///Gets the value of [`Self::rasterization_order_depth_attachment_access`]
    pub fn rasterization_order_depth_attachment_access(&self) -> bool {
        unsafe { std::mem::transmute(self.rasterization_order_depth_attachment_access as u8) }
    }
    ///Gets the value of [`Self::rasterization_order_stencil_attachment_access`]
    pub fn rasterization_order_stencil_attachment_access(&self) -> bool {
        unsafe { std::mem::transmute(self.rasterization_order_stencil_attachment_access as u8) }
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
    /// [`Self::rasterization_order_color_attachment_access`]
    pub fn rasterization_order_color_attachment_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.rasterization_order_color_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.rasterization_order_color_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::rasterization_order_depth_attachment_access`]
    pub fn rasterization_order_depth_attachment_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.rasterization_order_depth_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.rasterization_order_depth_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::rasterization_order_stencil_attachment_access`]
    pub fn rasterization_order_stencil_attachment_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.rasterization_order_stencil_attachment_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.rasterization_order_stencil_attachment_access as *mut Bool32)
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
    ///Sets the raw value of [`Self::rasterization_order_color_attachment_access`]
    pub fn set_rasterization_order_color_attachment_access(&mut self, value: bool) -> &mut Self {
        self.rasterization_order_color_attachment_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_depth_attachment_access`]
    pub fn set_rasterization_order_depth_attachment_access(&mut self, value: bool) -> &mut Self {
        self.rasterization_order_depth_attachment_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::rasterization_order_stencil_attachment_access`]
    pub fn set_rasterization_order_stencil_attachment_access(&mut self, value: bool) -> &mut Self {
        self.rasterization_order_stencil_attachment_access = value as u8 as u32;
        self
    }
}
