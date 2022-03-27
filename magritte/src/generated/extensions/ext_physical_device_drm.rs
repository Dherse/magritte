use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION")]
pub const EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME")]
pub const EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_physical_device_drm");
///[VkPhysicalDeviceDrmPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDrmPropertiesEXT.html) - Structure containing DRM information of a physical device
///# C Specifications
///The [`PhysicalDeviceDrmPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_physical_device_drm
///typedef struct VkPhysicalDeviceDrmPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           hasPrimary;
///    VkBool32           hasRender;
///    int64_t            primaryMajor;
///    int64_t            primaryMinor;
///    int64_t            renderMajor;
///    int64_t            renderMinor;
///} VkPhysicalDeviceDrmPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`has_primary`] is a boolean indicating whether the physical device has a DRM primary node.
/// - [`has_render`] is a boolean indicating whether the physical device has a DRM render node.
/// - [`primary_major`] is the DRM primary node major number, if any.
/// - [`primary_minor`] is the DRM primary node minor number, if any.
/// - [`render_major`] is the DRM render node major number, if any.
/// - [`render_minor`] is the DRM render node minor number, if any.
///# Description
///If the [`PhysicalDeviceDrmPropertiesEXT`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These are properties of the DRM information of a
/// physical device.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_physical_device_drm`]
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
pub struct PhysicalDeviceDrmPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`has_primary`] is a boolean indicating whether the physical device has
    ///a DRM primary node.
    has_primary: Bool32,
    ///[`has_render`] is a boolean indicating whether the physical device has
    ///a DRM render node.
    has_render: Bool32,
    ///[`primary_major`] is the DRM primary node major number, if any.
    primary_major: i64,
    ///[`primary_minor`] is the DRM primary node minor number, if any.
    primary_minor: i64,
    ///[`render_major`] is the DRM render node major number, if any.
    render_major: i64,
    ///[`render_minor`] is the DRM render node minor number, if any.
    render_minor: i64,
}
