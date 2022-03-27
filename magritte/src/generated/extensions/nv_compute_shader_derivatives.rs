use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_compute_shader_derivatives");
///[VkPhysicalDeviceComputeShaderDerivativesFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html) - Structure describing compute shader derivative features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_compute_shader_derivatives
///typedef struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           computeDerivativeGroupQuads;
///    VkBool32           computeDerivativeGroupLinear;
///} VkPhysicalDeviceComputeShaderDerivativesFeaturesNV;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`compute_derivative_group_quads`] indicates that the implementation supports the
///   `ComputeDerivativeGroupQuadsNV` SPIR-V capability.
/// - [`compute_derivative_group_linear`] indicates that the implementation supports the
///   `ComputeDerivativeGroupLinearNV` SPIR-V capability.
///See [Quad shader scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-quad) for more information.If the `VkPhysicalDeviceComputeShaderDerivativesFeaturesNVfeatures`. structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///`VkPhysicalDeviceComputeShaderDerivativesFeaturesNVfeatures`. **can** also be used in the
/// [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV`
///# Related
/// - [`VK_NV_compute_shader_derivatives`]
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
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`compute_derivative_group_quads`] indicates that the implementation
    ///supports the `ComputeDerivativeGroupQuadsNV` SPIR-V capability.
    compute_derivative_group_quads: Bool32,
    ///[`compute_derivative_group_linear`] indicates that the implementation
    ///supports the `ComputeDerivativeGroupLinearNV` SPIR-V capability.
    compute_derivative_group_linear: Bool32,
}
