use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION")]
pub const EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME")]
pub const EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_fragment_shader_interlock");
///[VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT.html) - Structure describing fragment shader interlock features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_fragment_shader_interlock
///typedef struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fragmentShaderSampleInterlock;
///    VkBool32           fragmentShaderPixelInterlock;
///    VkBool32           fragmentShaderShadingRateInterlock;
///} VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_shader_sample_interlock`] indicates that the implementation supports the
///   `FragmentShaderSampleInterlockEXT` SPIR-V capability.
/// - [`fragment_shader_pixel_interlock`] indicates that the implementation supports the
///   `FragmentShaderPixelInterlockEXT` SPIR-V capability.
/// - [`fragment_shader_shading_rate_interlock`] indicates that the implementation supports the
///   `FragmentShaderShadingRateInterlockEXT` SPIR-V capability.
///If the [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT`
///# Related
/// - [`VK_EXT_fragment_shader_interlock`]
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
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`fragment_shader_sample_interlock`] indicates that the implementation
    ///supports the `FragmentShaderSampleInterlockEXT` SPIR-V capability.
    fragment_shader_sample_interlock: Bool32,
    ///[`fragment_shader_pixel_interlock`] indicates that the implementation
    ///supports the `FragmentShaderPixelInterlockEXT` SPIR-V capability.
    fragment_shader_pixel_interlock: Bool32,
    ///[`fragment_shader_shading_rate_interlock`] indicates that the
    ///implementation supports the `FragmentShaderShadingRateInterlockEXT`
    ///SPIR-V capability.
    fragment_shader_shading_rate_interlock: Bool32,
}
