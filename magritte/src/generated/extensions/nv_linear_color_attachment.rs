//![VK_NV_linear_color_attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_linear_color_attachment.html) - device extension
//!# Description
//!This extension expands support for using `VK_IMAGE_TILING_LINEAR` images
//!as color attachments when all the color attachments in the render pass
//!instance have `VK_IMAGE_TILING_LINEAR` tiling.
//!This extension adds a new flag bit
//!`VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV` that extends the
//!existing [`FormatFeatureFlagBits2KHR`] bits.
//!This flag  **can**  be set for renderable color formats in the
//![`FormatProperties3KHR::linear_tiling_features`] format properties
//!structure member.
//!Formats with the `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
//!flag  **may**  be used as color attachments as long as all the color attachments
//!in the render pass instance have `VK_IMAGE_TILING_LINEAR` tiling, and
//!the formats their images views are created with have
//![`FormatProperties3KHR::linear_tiling_features`] which include
//!`VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`.
//!This extension supports both dynamic rendering and traditional render
//!passes.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - sourav parmar [souravpNV](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_linear_color_attachment]
//!   @souravpNV%0A<<Here describe the issue or question you have about the
//!   VK_NV_linear_color_attachment extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceLinearColorAttachmentFeaturesNV`]
//!# New constants
//! - [`NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME`]
//! - [`NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV`
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:  - `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
//!# Version History
//! - Revision 1, 2021-11-29 (sourav parmar)  - Initial draft
//!# Other info
//! * 2021-12-02
//! * - This extension requires [`VK_KHR_format_feature_flags2`]
//! * - Pat Brown, NVIDIA  - Piers Daniell, NVIDIA  - Sourav Parmar, NVIDIA
//!# Related
//! - [`PhysicalDeviceLinearColorAttachmentFeaturesNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION")]
pub const NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME")]
pub const NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_linear_color_attachment");
///[VkPhysicalDeviceLinearColorAttachmentFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html) - Structure describing whether <> rendering is supported by the implementation
///# C Specifications
///The [`PhysicalDeviceLinearColorAttachmentFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_linear_color_attachment
///typedef struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           linearColorAttachment;
///} VkPhysicalDeviceLinearColorAttachmentFeaturesNV;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`linear_color_attachment`] indicates whether the implementation supports renderable [Linear Color Attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary)
///If the [`PhysicalDeviceLinearColorAttachmentFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceLinearColorAttachmentFeaturesNV`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV`
///# Related
/// - [`VK_NV_linear_color_attachment`]
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
#[doc(alias = "VkPhysicalDeviceLinearColorAttachmentFeaturesNV")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] **must**  be
    /// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV`
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`linear_color_attachment`] indicates
    ///whether the implementation supports renderable [Linear Color
    ///Attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary)
    pub linear_color_attachment: Bool32,
}
impl<'lt> Default for PhysicalDeviceLinearColorAttachmentFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            linear_color_attachment: 0,
        }
    }
}
impl<'lt> PhysicalDeviceLinearColorAttachmentFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::linear_color_attachment`]
    pub fn linear_color_attachment_raw(&self) -> Bool32 {
        self.linear_color_attachment
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::linear_color_attachment`]
    pub fn set_linear_color_attachment_raw(&mut self, value: Bool32) -> &mut Self {
        self.linear_color_attachment = value;
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
    ///Gets the value of [`Self::linear_color_attachment`]
    pub fn linear_color_attachment(&self) -> bool {
        unsafe { std::mem::transmute(self.linear_color_attachment as u8) }
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
    ///Gets a mutable reference to the value of [`Self::linear_color_attachment`]
    pub fn linear_color_attachment_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.linear_color_attachment as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.linear_color_attachment as *mut Bool32)
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
    ///Sets the raw value of [`Self::linear_color_attachment`]
    pub fn set_linear_color_attachment(&mut self, value: bool) -> &mut Self {
        self.linear_color_attachment = value as u8 as u32;
        self
    }
}
