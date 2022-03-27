use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_fragment_shader_barycentric");
///[VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html) - Structure describing barycentric support in fragment shaders that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_fragment_shader_barycentric
///typedef struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fragmentShaderBarycentric;
///} VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_shader_barycentric`] indicates that the implementation supports the `BaryCoordNV`
///   and `BaryCoordNoPerspNV` SPIR-V fragment shader built-ins and supports the `PerVertexNV`
///   SPIR-V decoration on fragment shader input variables.
///See [Barycentric Interpolation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-barycentric) for more
///information.If the [`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`] structure is included
/// in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentShaderBarycentricFeaturesNV`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV`
///# Related
/// - [`VK_NV_fragment_shader_barycentric`]
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`fragment_shader_barycentric`]
    ///indicates that the implementation supports the `BaryCoordNV` and
    ///`BaryCoordNoPerspNV` SPIR-V fragment shader built-ins and supports
    ///the `PerVertexNV` SPIR-V decoration on fragment shader input
    ///variables.
    fragment_shader_barycentric: Bool32,
}
impl<'lt> Default for PhysicalDeviceFragmentShaderBarycentricFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            fragment_shader_barycentric: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentShaderBarycentricFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::fragment_shader_barycentric`]
    pub fn fragment_shader_barycentric_raw(&self) -> Bool32 {
        self.fragment_shader_barycentric
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_shader_barycentric`]
    pub fn set_fragment_shader_barycentric_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_shader_barycentric = value;
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
    ///Gets the value of [`Self::fragment_shader_barycentric`]
    pub fn fragment_shader_barycentric(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_shader_barycentric as u8) }
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
    ///Gets a mutable reference to the value of [`Self::fragment_shader_barycentric`]
    pub fn fragment_shader_barycentric_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_shader_barycentric as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_shader_barycentric as *mut Bool32)
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
    ///Sets the raw value of [`Self::fragment_shader_barycentric`]
    pub fn set_fragment_shader_barycentric(&mut self, value: bool) -> &mut Self {
        self.fragment_shader_barycentric = value as u8 as u32;
        self
    }
}
