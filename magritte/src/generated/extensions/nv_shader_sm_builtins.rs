//![VK_NV_shader_sm_builtins](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_shader_sm_builtins.html) - device extension
//!# Description
//!This extension provides the ability to determine device-specific properties
//!on NVIDIA GPUs.
//!It provides the number of streaming multiprocessors (SMs), the maximum
//!number of warps (subgroups) that can run on an SM, and shader builtins to
//!enable invocations to identify which SM and warp a shader invocation is
//!executing on.This extension enables support for the SPIR-V `ShaderSMBuiltinsNV`
//!capability.These properties and built-ins  **should**  typically only be used for debugging
//!purposes.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.1
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_shader_sm_builtins]
//!   @dgkoch%0A<<Here describe the issue or question you have about the VK_NV_shader_sm_builtins
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderSmBuiltinsFeaturesNV`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceShaderSmBuiltinsPropertiesNV`]
//!# New constants
//! - [`NV_SHADER_SM_BUILTINS_EXTENSION_NAME`]
//! - [`NV_SHADER_SM_BUILTINS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV`
//!# Known issues & F.A.Q.
//!0. What should we call this extension? **RESOLVED** : `NV_shader_sm_builtins`. Other options
//! considered included:  - `NV_shader_smid` - but SMID is really easy to typo/confuse as SIMD.  -
//! `NV_shader_sm_info` - but  **Info**  is typically reserved for input structures
//!# Version history
//! - Revision 1, 2019-05-28 (Daniel Koch)  - Internal revisions
//!# Other information
//! * 2019-05-28
//! * - This extension requires [`SPV_NV_shader_sm_builtins`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shader_sm_builtins.html).
//!   - This extension provides API support for [`GL_NV_shader_sm_builtins`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_shader_sm_builtins.txt)
//! * - Jeff Bolz, NVIDIA  - Eric Werness, NVIDIA
//!# Related
//! - [`PhysicalDeviceShaderSmBuiltinsFeaturesNV`]
//! - [`PhysicalDeviceShaderSmBuiltinsPropertiesNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV`
///# Related
/// - [`nv_shader_sm_builtins`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsPropertiesNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceShaderSmBuiltinsPropertiesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_sm_count`] is the number of SMs on the
    ///device.
    pub shader_sm_count: u32,
    ///[`shader_warps_per_sm`] is the maximum number
    ///of simultaneously executing warps on an SM.
    pub shader_warps_per_sm: u32,
}
impl<'lt> Default for PhysicalDeviceShaderSmBuiltinsPropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV,
            p_next: std::ptr::null_mut(),
            shader_sm_count: 0,
            shader_warps_per_sm: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderSmBuiltinsPropertiesNV<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> PhysicalDeviceShaderSmBuiltinsPropertiesNV<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::shader_sm_count`]
    pub fn shader_sm_count(&self) -> u32 {
        self.shader_sm_count
    }
    ///Gets the value of [`Self::shader_warps_per_sm`]
    pub fn shader_warps_per_sm(&self) -> u32 {
        self.shader_warps_per_sm
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
    ///Gets a mutable reference to the value of [`Self::shader_sm_count`]
    pub fn shader_sm_count_mut(&mut self) -> &mut u32 {
        &mut self.shader_sm_count
    }
    ///Gets a mutable reference to the value of [`Self::shader_warps_per_sm`]
    pub fn shader_warps_per_sm_mut(&mut self) -> &mut u32 {
        &mut self.shader_warps_per_sm
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_sm_count`]
    pub fn set_shader_sm_count(&mut self, value: u32) -> &mut Self {
        self.shader_sm_count = value;
        self
    }
    ///Sets the value of [`Self::shader_warps_per_sm`]
    pub fn set_shader_warps_per_sm(&mut self, value: u32) -> &mut Self {
        self.shader_warps_per_sm = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_sm_count`]
    pub fn with_shader_sm_count(mut self, value: u32) -> Self {
        self.shader_sm_count = value;
        self
    }
    ///Sets the value of [`Self::shader_warps_per_sm`]
    pub fn with_shader_warps_per_sm(mut self, value: u32) -> Self {
        self.shader_warps_per_sm = value;
        self
    }
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
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderSmBuiltinsFeaturesNV`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV`
///# Related
/// - [`nv_shader_sm_builtins`]
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
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsFeaturesNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceShaderSmBuiltinsFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_sm_builtins`] indicates whether
    ///the implementation supports the SPIR-V `ShaderSMBuiltinsNV`
    ///capability.
    pub shader_sm_builtins: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderSmBuiltinsFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            shader_sm_builtins: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderSmBuiltinsFeaturesNV<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> PhysicalDeviceShaderSmBuiltinsFeaturesNV<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::shader_sm_builtins`]
    pub fn shader_sm_builtins_raw(&self) -> Bool32 {
        self.shader_sm_builtins
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_sm_builtins`]
    pub fn set_shader_sm_builtins_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_sm_builtins = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_sm_builtins`]
    pub fn with_shader_sm_builtins_raw(mut self, value: Bool32) -> Self {
        self.shader_sm_builtins = value;
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
    ///Gets the value of [`Self::shader_sm_builtins`]
    pub fn shader_sm_builtins(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_sm_builtins as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_sm_builtins`]
    pub fn shader_sm_builtins_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_sm_builtins as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_sm_builtins as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_sm_builtins`]
    pub fn set_shader_sm_builtins(&mut self, value: bool) -> &mut Self {
        self.shader_sm_builtins = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_sm_builtins`]
    pub fn with_shader_sm_builtins(mut self, value: bool) -> Self {
        self.shader_sm_builtins = value as u8 as u32;
        self
    }
}
