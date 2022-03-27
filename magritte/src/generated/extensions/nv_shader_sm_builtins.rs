use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION")]
pub const NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME")]
pub const NV_SHADER_SM_BUILTINS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_shader_sm_builtins");
///[VkPhysicalDeviceShaderSMBuiltinsPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html) - Structure describing shader SM Builtins properties supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderSmBuiltinsPropertiesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_shader_sm_builtins
///typedef struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           shaderSMCount;
///    uint32_t           shaderWarpsPerSM;
///} VkPhysicalDeviceShaderSMBuiltinsPropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_sm_count`] is the number of SMs on the device.
/// - [`shader_warps_per_sm`] is the maximum number of simultaneously executing warps on an SM.
///# Description
///If the [`PhysicalDeviceShaderSmBuiltinsPropertiesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV`
///# Related
/// - [`VK_NV_shader_sm_builtins`]
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
pub struct PhysicalDeviceShaderSmBuiltinsPropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_sm_count`] is the number of SMs on the
    ///device.
    shader_sm_count: u32,
    ///[`shader_warps_per_sm`] is the maximum number
    ///of simultaneously executing warps on an SM.
    shader_warps_per_sm: u32,
}
///[VkPhysicalDeviceShaderSMBuiltinsFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html) - Structure describing the shader SM Builtins features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderSmBuiltinsFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_shader_sm_builtins
///typedef struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderSMBuiltins;
///} VkPhysicalDeviceShaderSMBuiltinsFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_sm_builtins`] indicates whether the implementation supports the SPIR-V
///   `ShaderSMBuiltinsNV` capability.
///If the [`PhysicalDeviceShaderSmBuiltinsFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderSmBuiltinsFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV`
///# Related
/// - [`VK_NV_shader_sm_builtins`]
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
pub struct PhysicalDeviceShaderSmBuiltinsFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_sm_builtins`] indicates whether
    ///the implementation supports the SPIR-V `ShaderSMBuiltinsNV`
    ///capability.
    shader_sm_builtins: Bool32,
}
