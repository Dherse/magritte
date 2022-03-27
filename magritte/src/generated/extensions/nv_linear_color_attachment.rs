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
///[`PhysicalDeviceLinearColorAttachmentFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be
    /// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV`
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseOutStructure<'lt>,
    ///[`linear_color_attachment`] indicates
    ///whether the implementation supports renderable [Linear Color
    ///Attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary)
    linear_color_attachment: Bool32,
}
