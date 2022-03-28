//![VK_INTEL_shader_integer_functions2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_INTEL_shader_integer_functions2.html) - device extension
//!# Description
//!This extension adds support for several new integer instructions in SPIR-V
//!for use in graphics shaders.
//!Many of these instructions have pre-existing counterparts in the Kernel
//!environment.The added integer functions are defined by the
//![`SPV_INTEL_shader_integer_functions2`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/INTEL/SPV_INTEL_shader_integer_functions2.html)
//!SPIR-V extension and can be used with the GL_INTEL_shader_integer_functions2
//!GLSL extension.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Ian Romanick [ianromanick](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_INTEL_shader_integer_functions2]
//!   @ianromanick%0A<<Here describe the issue or question you have about the
//!   VK_INTEL_shader_integer_functions2 extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`]
//!# New constants
//! - [`INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME`]
//! - [`INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL`
//!# Version History
//! - Revision 1, 2019-04-30 (Ian Romanick)  - Initial draft
//!# Other info
//! * 2019-04-30
//! * No known IP claims.
//! * - This extension requires [`SPV_INTEL_shader_integer_functions2`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/INTEL/SPV_INTEL_shader_integer_functions2.html).
//!   - This extension provides API support for [`GL_INTEL_shader_integer_functions2`](https://www.khronos.org/registry/OpenGL/extensions/INTEL/INTEL_shader_integer_functions2.txt).
//! * - Ian Romanick, Intel  - Ben Ashbaugh, Intel
//!# Related
//! - [`PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`]
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
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_INTEL_shader_integer_functions2");
///[VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html) - Structure describing shader integer functions that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL`] structure is
///defined as:
///```c
///// Provided by VK_INTEL_shader_integer_functions2
///typedef struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderIntegerFunctions2;
///} VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_integer_functions_2`] indicates that the implementation supports the
///   `IntegerFunctions2INTEL` SPIR-V capability.
///If the `VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTELfeatures`. structure is included in
/// the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///`VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTELfeatures`.  **can**  also be used in the
/// [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL`
///# Related
/// - [`VK_INTEL_shader_integer_functions2`]
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
#[doc(alias = "VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_integer_functions_2`]
    ///indicates that the implementation supports the
    ///`IntegerFunctions2INTEL` SPIR-V capability.
    pub shader_integer_functions_2: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shader_integer_functions_2: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_integer_functions_2`]
    pub fn shader_integer_functions_2_raw(&self) -> Bool32 {
        self.shader_integer_functions_2
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_integer_functions_2`]
    pub fn set_shader_integer_functions_2_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_integer_functions_2 = value;
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
    ///Gets the value of [`Self::shader_integer_functions_2`]
    pub fn shader_integer_functions_2(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_integer_functions_2 as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_integer_functions_2`]
    pub fn shader_integer_functions_2_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_integer_functions_2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_integer_functions_2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the raw value of [`Self::shader_integer_functions_2`]
    pub fn set_shader_integer_functions_2(&mut self, value: bool) -> &mut Self {
        self.shader_integer_functions_2 = value as u8 as u32;
        self
    }
}
