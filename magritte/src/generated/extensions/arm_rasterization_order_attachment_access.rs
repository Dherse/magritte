use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
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
///[`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]**can**
///also be used in the [`p_next`] chain of [`DeviceCreateInfo`] to enable
///features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`]**must** be
    /// `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM`
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseOutStructure<'lt>,
    ///[`rasterization_order_color_attachment_access`] indicates that
    ///rasterization order access to color and input attachments is supported
    ///by the implementation.
    rasterization_order_color_attachment_access: Bool32,
    ///[`rasterization_order_depth_attachment_access`] indicates that
    ///rasterization order access to the depth aspect of depth/stencil and
    ///input attachments is supported by the implementation.
    rasterization_order_depth_attachment_access: Bool32,
    ///[`rasterization_order_stencil_attachment_access`] indicates that
    ///rasterization order access to the stencil aspect of depth/stencil and
    ///input attachments is supported by the implementation.
    rasterization_order_stencil_attachment_access: Bool32,
}
