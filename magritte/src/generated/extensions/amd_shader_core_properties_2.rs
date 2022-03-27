use crate::vulkan1_0::{BaseOutStructure, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_shader_core_properties2");
///[VkPhysicalDeviceShaderCoreProperties2AMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html) - Structure describing shader core properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderCoreProperties2AMD`] structure is defined as:
///```c
///// Provided by VK_AMD_shader_core_properties2
///typedef struct VkPhysicalDeviceShaderCoreProperties2AMD {
///    VkStructureType                   sType;
///    void*                             pNext;
///    VkShaderCorePropertiesFlagsAMD    shaderCoreFeatures;
///    uint32_t                          activeComputeUnitCount;
///} VkPhysicalDeviceShaderCoreProperties2AMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_core_features`] is a bitmask of [`ShaderCorePropertiesFlagBitsAMD`] indicating the
///   set of features supported by the shader core.
/// - [`active_compute_unit_count`] is an unsigned integer value indicating the number of compute
///   units that have been enabled.
///# Description
///If the [`PhysicalDeviceShaderCoreProperties2AMD`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD`
///# Related
/// - [`VK_AMD_shader_core_properties2`]
/// - [`ShaderCorePropertiesFlagsAMD`]
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
pub struct PhysicalDeviceShaderCoreProperties2AMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_core_features`] is a bitmask of
    ///[`ShaderCorePropertiesFlagBitsAMD`] indicating the set of features
    ///supported by the shader core.
    shader_core_features: ShaderCorePropertiesFlagsAMD,
    ///[`active_compute_unit_count`] is an
    ///unsigned integer value indicating the number of compute units that have
    ///been enabled.
    active_compute_unit_count: u32,
}
