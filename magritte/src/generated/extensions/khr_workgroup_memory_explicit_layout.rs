use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_workgroup_memory_explicit_layout");
///[VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR.html) - Structure describing the workgroup storage explicit layout features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`] structure
///is defined as:
///```c
///// Provided by VK_KHR_workgroup_memory_explicit_layout
///typedef struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           workgroupMemoryExplicitLayout;
///    VkBool32           workgroupMemoryExplicitLayoutScalarBlockLayout;
///    VkBool32           workgroupMemoryExplicitLayout8BitAccess;
///    VkBool32           workgroupMemoryExplicitLayout16BitAccess;
///} VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`workgroup_memory_explicit_layout`] indicates whether the implementation supports the SPIR-V
///   `WorkgroupMemoryExplicitLayoutKHR` capability.
/// - [`workgroup_memory_explicit_layout_scalar_block_layout`] indicates whether the implementation
///   supports scalar alignment for laying out Workgroup Blocks.
/// - [`workgroup_memory_explicit_layout_8_bit_access`] indicates whether objects in the `Workgroup`
///   storage class with the `Block` decoration **can** have 8-bit integer members. If this feature
///   is not enabled, 8-bit integer members **must** not be used in such objects. This also
///   indicates whether shader modules **can** declare the
///   `WorkgroupMemoryExplicitLayout8BitAccessKHR` capability.
/// - [`workgroup_memory_explicit_layout_16_bit_access`] indicates whether objects in the
///   `Workgroup` storage class with the `Block` decoration **can** have 16-bit integer and 16-bit
///   floating-point members. If this feature is not enabled, 16-bit integer or 16-bit
///   floating-point members **must** not be used in such objects. This also indicates whether
///   shader modules **can** declare the `WorkgroupMemoryExplicitLayout16BitAccessKHR` capability.
///If the [`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR`
///# Related
/// - [`VK_KHR_workgroup_memory_explicit_layout`]
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
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`workgroup_memory_explicit_layout`] indicates whether the implementation
    ///supports the SPIR-V `WorkgroupMemoryExplicitLayoutKHR` capability.
    workgroup_memory_explicit_layout: Bool32,
    ///[`workgroup_memory_explicit_layout_scalar_block_layout`] indicates whether
    ///the implementation supports scalar alignment for laying out Workgroup
    ///Blocks.
    workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
    ///[`workgroup_memory_explicit_layout_8_bit_access`] indicates whether objects
    ///in the `Workgroup` storage class with the `Block` decoration **can**
    ///have 8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members **must** not be used
    ///in such objects.
    ///This also indicates whether shader modules **can** declare the
    ///`WorkgroupMemoryExplicitLayout8BitAccessKHR` capability.
    workgroup_memory_explicit_layout_8_bit_access: Bool32,
    ///[`workgroup_memory_explicit_layout_16_bit_access`] indicates whether objects
    ///in the `Workgroup` storage class with the `Block` decoration **can**
    ///have 16-bit integer and 16-bit floating-point members.
    ///If this feature is not enabled, 16-bit integer or 16-bit floating-point
    ///members **must** not be used in such objects.
    ///This also indicates whether shader modules **can** declare the
    ///`WorkgroupMemoryExplicitLayout16BitAccessKHR` capability.
    workgroup_memory_explicit_layout_16_bit_access: Bool32,
}
