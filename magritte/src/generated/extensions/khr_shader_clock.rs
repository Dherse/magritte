use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_CLOCK_SPEC_VERSION")]
pub const KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHADER_CLOCK_EXTENSION_NAME")]
pub const KHR_SHADER_CLOCK_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_shader_clock");
///[VkPhysicalDeviceShaderClockFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderClockFeaturesKHR.html) - Structure describing features supported by VK_KHR_shader_clock
///# C Specifications
///The [`PhysicalDeviceShaderClockFeaturesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_shader_clock
///typedef struct VkPhysicalDeviceShaderClockFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderSubgroupClock;
///    VkBool32           shaderDeviceClock;
///} VkPhysicalDeviceShaderClockFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_subgroup_clock`] indicates whether shaders **can** perform `Subgroup` scoped clock
///   reads.
/// - [`shader_device_clock`] indicates whether shaders **can** perform [`Device`] scoped clock
///   reads.
///If the [`PhysicalDeviceShaderClockFeaturesKHR`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderClockFeaturesKHR`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR`
///# Related
/// - [`VK_KHR_shader_clock`]
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
pub struct PhysicalDeviceShaderClockFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_subgroup_clock`] indicates
    ///whether shaders **can** perform `Subgroup` scoped clock reads.
    shader_subgroup_clock: Bool32,
    ///[`shader_device_clock`] indicates whether
    ///shaders **can** perform [`Device`] scoped clock reads.
    shader_device_clock: Bool32,
}
