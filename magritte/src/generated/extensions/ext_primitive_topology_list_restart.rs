use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_primitive_topology_list_restart");
///[VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html) - Structure describing whether list type primitives can support primitive restart
///# C Specifications
///The [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`] structure
///is defined as:
///```c
///// Provided by VK_EXT_primitive_topology_list_restart
///typedef struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           primitiveTopologyListRestart;
///    VkBool32           primitiveTopologyPatchListRestart;
///} VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT;
///```
///# Members
///The members of the
///[`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`] structure
///describe the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`primitive_topology_list_restart`] indicates that list type primitives,
///   `VK_PRIMITIVE_TOPOLOGY_POINT_LIST`, `VK_PRIMITIVE_TOPOLOGY_LINE_LIST`,
///   `VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST`, `VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY` and
///   `VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY`, **can** use the primitive restart index
///   value in index buffers.
/// - [`primitive_topology_patch_list_restart`] indicates that the
///   `VK_PRIMITIVE_TOPOLOGY_PATCH_LIST` topology **can** use the primitive restart index value in
///   index buffers.
///If the [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT`
///# Related
/// - [`VK_EXT_primitive_topology_list_restart`]
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
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`primitive_topology_list_restart`] indicates that list type primitives,
    ///`VK_PRIMITIVE_TOPOLOGY_POINT_LIST`,
    ///`VK_PRIMITIVE_TOPOLOGY_LINE_LIST`,
    ///`VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST`,
    ///`VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY` and
    ///`VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY`, **can** use the
    ///primitive restart index value in index buffers.
    primitive_topology_list_restart: Bool32,
    ///[`primitive_topology_patch_list_restart`] indicates that the
    ///`VK_PRIMITIVE_TOPOLOGY_PATCH_LIST` topology **can** use the primitive
    ///restart index value in index buffers.
    primitive_topology_patch_list_restart: Bool32,
}
