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
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_core_features`] is a bitmask of
    ///[`ShaderCorePropertiesFlagBitsAMD`] indicating the set of features
    ///supported by the shader core.
    shader_core_features: ShaderCorePropertiesFlagsAMD,
    ///[`active_compute_unit_count`] is an
    ///unsigned integer value indicating the number of compute units that have
    ///been enabled.
    active_compute_unit_count: u32,
}
impl<'lt> Default for PhysicalDeviceShaderCoreProperties2AMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shader_core_features: Default::default(),
            active_compute_unit_count: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderCoreProperties2AMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::shader_core_features`]
    pub fn shader_core_features(&self) -> ShaderCorePropertiesFlagsAMD {
        self.shader_core_features
    }
    ///Gets the value of [`Self::active_compute_unit_count`]
    pub fn active_compute_unit_count(&self) -> u32 {
        self.active_compute_unit_count
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::shader_core_features`]
    pub fn shader_core_features_mut(&mut self) -> &mut ShaderCorePropertiesFlagsAMD {
        &mut self.shader_core_features
    }
    ///Gets a mutable reference to the value of [`Self::active_compute_unit_count`]
    pub fn active_compute_unit_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::shader_core_features`]
    pub fn set_shader_core_features(
        &mut self,
        value: crate::extensions::amd_shader_core_properties_2::ShaderCorePropertiesFlagsAMD,
    ) -> &mut Self {
        self.shader_core_features = value;
        self
    }
    ///Sets the raw value of [`Self::active_compute_unit_count`]
    pub fn set_active_compute_unit_count(&mut self, value: u32) -> &mut Self {
        self.active_compute_unit_count = value;
        self
    }
}
