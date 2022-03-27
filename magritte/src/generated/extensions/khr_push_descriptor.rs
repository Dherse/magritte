use crate::vulkan1_0::{BaseOutStructure, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION")]
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME")]
pub const KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_push_descriptor");
///[VkPhysicalDevicePushDescriptorPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html) - Structure describing push descriptor limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDevicePushDescriptorPropertiesKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_push_descriptor
///typedef struct VkPhysicalDevicePushDescriptorPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxPushDescriptors;
///} VkPhysicalDevicePushDescriptorPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_push_descriptors`] is the maximum number of descriptors that **can** be used in a
///   descriptor set created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` set.
///# Description
///If the [`PhysicalDevicePushDescriptorPropertiesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR`
///# Related
/// - [`VK_KHR_push_descriptor`]
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
pub struct PhysicalDevicePushDescriptorPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_push_descriptors`] is the maximum
    ///number of descriptors that **can** be used in a descriptor set created with
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` set.
    max_push_descriptors: u32,
}
