use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_QUERY_SPEC_VERSION")]
pub const KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_RAY_QUERY_EXTENSION_NAME")]
pub const KHR_RAY_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_ray_query");
///[VkPhysicalDeviceRayQueryFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html) - Structure describing the ray query features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRayQueryFeaturesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_ray_query
///typedef struct VkPhysicalDeviceRayQueryFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           rayQuery;
///} VkPhysicalDeviceRayQueryFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`ray_query`] indicates whether the implementation supports ray query (`OpRayQueryProceedKHR`)
///   functionality.
///If the [`PhysicalDeviceRayQueryFeaturesKHR`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceRayQueryFeaturesKHR`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR`
///# Related
/// - [`VK_KHR_ray_query`]
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
pub struct PhysicalDeviceRayQueryFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`ray_query`] indicates whether the
    ///implementation supports ray query (`OpRayQueryProceedKHR`)
    ///functionality.
    ray_query: Bool32,
}
