//![VK_EXT_fragment_shader_interlock](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_fragment_shader_interlock.html) - device extension
//!# Description
//!This extension adds support for the `FragmentShaderPixelInterlockEXT`,
//!`FragmentShaderSampleInterlockEXT`, and
//!`FragmentShaderShadingRateInterlockEXT` capabilities from the
//!`SPV_EXT_fragment_shader_interlock` extension to Vulkan.Enabling these capabilities provides a
//! critical section for fragment shaders
//!to avoid overlapping pixels being processed at the same time, and certain
//!guarantees about the ordering of fragment shader invocations of fragments of
//!overlapping pixels.This extension can be useful for algorithms that need to access per-pixel
//!data structures via shader loads and stores.
//!Algorithms using this extension can access per-pixel data structures in
//!critical sections without other invocations accessing the same per-pixel
//!data.
//!Additionally, the ordering guarantees are useful for cases where the API
//!ordering of fragments is meaningful.
//!For example, applications may be able to execute programmable blending
//!operations in the fragment shader, where the destination buffer is read via
//!image loads and the final value is written via image stores.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_fragment_shader_interlock]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_fragment_shader_interlock extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`]
//!# New constants
//! - [`EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME`]
//! - [`EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2019-05-24 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2019-05-02
//! * - This extension requires [`SPV_EXT_fragment_shader_interlock`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_fragment_shader_interlock.html)
//!   - This extension provides API support for [`GL_ARB_fragment_shader_interlock`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_fragment_shader_interlock.txt)
//! * - Daniel Koch, NVIDIA  - Graeme Leese, Broadcom  - Jan-Harald Fredriksen, Arm  - Jason
//!   Ekstrand, Intel  - Jeff Bolz, NVIDIA  - Ruihao Zhang, Qualcomm  - Slawomir Grajewski, Intel  -
//!   Spencer Fricke, Samsung
//!# Related
//! - [`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`]
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
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentShaderInterlockFeaturesEXT`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`fragment_shader_sample_interlock`] indicates that the implementation
    ///supports the `FragmentShaderSampleInterlockEXT` SPIR-V capability.
    pub fragment_shader_sample_interlock: Bool32,
    ///[`fragment_shader_pixel_interlock`] indicates that the implementation
    ///supports the `FragmentShaderPixelInterlockEXT` SPIR-V capability.
    pub fragment_shader_pixel_interlock: Bool32,
    ///[`fragment_shader_shading_rate_interlock`] indicates that the
    ///implementation supports the `FragmentShaderShadingRateInterlockEXT`
    ///SPIR-V capability.
    pub fragment_shader_shading_rate_interlock: Bool32,
}
impl<'lt> Default for PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceFragmentShaderInterlockFeaturesExt,
            p_next: std::ptr::null_mut(),
            fragment_shader_sample_interlock: 0,
            fragment_shader_pixel_interlock: 0,
            fragment_shader_shading_rate_interlock: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::fragment_shader_sample_interlock`]
    pub fn fragment_shader_sample_interlock_raw(&self) -> Bool32 {
        self.fragment_shader_sample_interlock
    }
    ///Gets the raw value of [`Self::fragment_shader_pixel_interlock`]
    pub fn fragment_shader_pixel_interlock_raw(&self) -> Bool32 {
        self.fragment_shader_pixel_interlock
    }
    ///Gets the raw value of [`Self::fragment_shader_shading_rate_interlock`]
    pub fn fragment_shader_shading_rate_interlock_raw(&self) -> Bool32 {
        self.fragment_shader_shading_rate_interlock
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shader_sample_interlock`]
    pub fn set_fragment_shader_sample_interlock_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shader_sample_interlock = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shader_pixel_interlock`]
    pub fn set_fragment_shader_pixel_interlock_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shader_pixel_interlock = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shader_shading_rate_interlock`]
    pub fn set_fragment_shader_shading_rate_interlock_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shader_shading_rate_interlock = value;
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
    ///Gets the value of [`Self::fragment_shader_sample_interlock`]
    pub fn fragment_shader_sample_interlock(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shader_sample_interlock as u8) }
    }
    ///Gets the value of [`Self::fragment_shader_pixel_interlock`]
    pub fn fragment_shader_pixel_interlock(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shader_pixel_interlock as u8) }
    }
    ///Gets the value of [`Self::fragment_shader_shading_rate_interlock`]
    pub fn fragment_shader_shading_rate_interlock(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shader_shading_rate_interlock as u8) }
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
    ///Gets a mutable reference to the value of [`Self::fragment_shader_sample_interlock`]
    pub fn fragment_shader_sample_interlock_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shader_sample_interlock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shader_sample_interlock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::fragment_shader_pixel_interlock`]
    pub fn fragment_shader_pixel_interlock_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shader_pixel_interlock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shader_pixel_interlock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::fragment_shader_shading_rate_interlock`]
    pub fn fragment_shader_shading_rate_interlock_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shader_shading_rate_interlock as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shader_shading_rate_interlock as *mut Bool32)
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
    ///Sets the raw value of [`Self::fragment_shader_sample_interlock`]
    pub fn set_fragment_shader_sample_interlock(&mut self, value: bool) -> &mut Self {
        self.fragment_shader_sample_interlock = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_shader_pixel_interlock`]
    pub fn set_fragment_shader_pixel_interlock(&mut self, value: bool) -> &mut Self {
        self.fragment_shader_pixel_interlock = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_shader_shading_rate_interlock`]
    pub fn set_fragment_shader_shading_rate_interlock(&mut self, value: bool) -> &mut Self {
        self.fragment_shader_shading_rate_interlock = value as u8 as u32;
        self
    }
}
