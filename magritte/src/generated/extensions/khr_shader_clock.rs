//![VK_KHR_shader_clock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_clock.html) - device extension
//!# Description
//!This extension advertises the SPIR-V `ShaderClockKHR` capability for
//!Vulkan, which allows a shader to query a real-time or monotonically
//!incrementing counter at the subgroup level or across the device level.
//!The two valid SPIR-V scopes for `OpReadClockKHR` are `Subgroup` and
//![`Device`].When using GLSL source-based shading languages, the `clockRealtime*EXT`()
//!timing functions map to the `OpReadClockKHR` instruction with a scope of
//![`Device`], and the `clock*ARB`() timing functions map to the
//!`OpReadClockKHR` instruction with a scope of `Subgroup`.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Aaron Hagan [ahagan](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_clock]
//!   @ahagan%0A<<Here describe the issue or question you have about the VK_KHR_shader_clock
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderClockFeaturesKHR`]
//!# New constants
//! - [`KHR_SHADER_CLOCK_EXTENSION_NAME`]
//! - [`KHR_SHADER_CLOCK_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR`
//!# Version History
//! - Revision 1, 2019-4-25 (Aaron Hagan)  - Initial revision
//!# Other info
//! * 2019-4-25
//! * No known IP claims.
//! * - This extension requires [`SPV_KHR_shader_clock`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_shader_clock.html).
//!   - This extension provides API support for [`ARB_shader_clock`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_shader_clock.txt)
//!   and [`EXT_shader_realtime_clock`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_shader_realtime_clock.txt)
//! * - Aaron Hagan, AMD  - Daniel Koch, NVIDIA
//!# Related
//! - [`PhysicalDeviceShaderClockFeaturesKHR`]
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
/// # Members
/// This structure describes the following features:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_subgroup_clock`] indicates whether shaders  **can**  perform `Subgroup` scoped clock
///   reads.
/// - [`shader_device_clock`] indicates whether shaders  **can**  perform [`Device`] scoped clock
///   reads.
/// If the [`PhysicalDeviceShaderClockFeaturesKHR`] structure is included in the [`p_next`] chain of
/// the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceShaderClockFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR`
/// # Related
/// - [`khr_shader_clock`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceShaderClockFeaturesKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceShaderClockFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_subgroup_clock`] indicates
    ///whether shaders  **can**  perform `Subgroup` scoped clock reads.
    pub shader_subgroup_clock: Bool32,
    ///[`shader_device_clock`] indicates whether
    ///shaders  **can**  perform [`Device`] scoped clock reads.
    pub shader_device_clock: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderClockFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            shader_subgroup_clock: 0,
            shader_device_clock: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderClockFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::shader_subgroup_clock`]
    pub fn shader_subgroup_clock_raw(&self) -> Bool32 {
        self.shader_subgroup_clock
    }
    ///Gets the raw value of [`Self::shader_device_clock`]
    pub fn shader_device_clock_raw(&self) -> Bool32 {
        self.shader_device_clock
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_subgroup_clock`]
    pub fn set_shader_subgroup_clock_raw(mut self, value: Bool32) -> Self {
        self.shader_subgroup_clock = value;
        self
    }
    ///Sets the raw value of [`Self::shader_device_clock`]
    pub fn set_shader_device_clock_raw(mut self, value: Bool32) -> Self {
        self.shader_device_clock = value;
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
    ///Gets the value of [`Self::shader_subgroup_clock`]
    pub fn shader_subgroup_clock(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_subgroup_clock as u8) }
    }
    ///Gets the value of [`Self::shader_device_clock`]
    pub fn shader_device_clock(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_device_clock as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_subgroup_clock`]
    pub fn shader_subgroup_clock_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_subgroup_clock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_subgroup_clock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_device_clock`]
    pub fn shader_device_clock_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_device_clock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_device_clock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_subgroup_clock`]
    pub fn set_shader_subgroup_clock(mut self, value: bool) -> Self {
        self.shader_subgroup_clock = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_device_clock`]
    pub fn set_shader_device_clock(mut self, value: bool) -> Self {
        self.shader_device_clock = value as u8 as u32;
        self
    }
}
