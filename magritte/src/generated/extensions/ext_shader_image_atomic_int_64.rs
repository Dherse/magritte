use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_shader_image_atomic_int64");
///[VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html) - Structure describing features supported by VK_EXT_shader_image_atomic_int64
///# C Specifications
///The [`PhysicalDeviceShaderImageAtomicInt64FeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_shader_image_atomic_int64
///typedef struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderImageInt64Atomics;
///    VkBool32           sparseImageInt64Atomics;
///} VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_image_int_64_atomics`] indicates whether shaders  **can**  support 64-bit unsigned
///   and signed integer atomic operations on images.
/// - [`sparse_image_int_64_atomics`] indicates whether 64-bit integer atomics  **can**  be used on
///   sparse images.
///If the `VkPhysicalDeviceShaderAtomicInt64FeaturesEXT` structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///`VkPhysicalDeviceShaderAtomicInt64FeaturesEXT` **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT`
///# Related
/// - [`VK_EXT_shader_image_atomic_int64`]
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
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_image_int_64_atomics`]
    ///indicates whether shaders  **can**  support 64-bit unsigned and signed
    ///integer atomic operations on images.
    shader_image_int_64_atomics: Bool32,
    ///[`sparse_image_int_64_atomics`]
    ///indicates whether 64-bit integer atomics  **can**  be used on sparse images.
    sparse_image_int_64_atomics: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shader_image_int_64_atomics: 0,
            sparse_image_int_64_atomics: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderImageAtomicInt64FeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_image_int_64_atomics`]
    pub fn shader_image_int_64_atomics_raw(&self) -> Bool32 {
        self.shader_image_int_64_atomics
    }
    ///Gets the raw value of [`Self::sparse_image_int_64_atomics`]
    pub fn sparse_image_int_64_atomics_raw(&self) -> Bool32 {
        self.sparse_image_int_64_atomics
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_image_int_64_atomics`]
    pub fn set_shader_image_int_64_atomics_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_image_int_64_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::sparse_image_int_64_atomics`]
    pub fn set_sparse_image_int_64_atomics_raw(&mut self, value: Bool32) -> &mut Self {
        self.sparse_image_int_64_atomics = value;
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
    ///Gets the value of [`Self::shader_image_int_64_atomics`]
    pub fn shader_image_int_64_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_image_int_64_atomics as u8) }
    }
    ///Gets the value of [`Self::sparse_image_int_64_atomics`]
    pub fn sparse_image_int_64_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.sparse_image_int_64_atomics as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_image_int_64_atomics`]
    pub fn shader_image_int_64_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_image_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_image_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::sparse_image_int_64_atomics`]
    pub fn sparse_image_int_64_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.sparse_image_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.sparse_image_int_64_atomics as *mut Bool32)
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
    ///Sets the raw value of [`Self::shader_image_int_64_atomics`]
    pub fn set_shader_image_int_64_atomics(&mut self, value: bool) -> &mut Self {
        self.shader_image_int_64_atomics = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::sparse_image_int_64_atomics`]
    pub fn set_sparse_image_int_64_atomics(&mut self, value: bool) -> &mut Self {
        self.sparse_image_int_64_atomics = value as u8 as u32;
        self
    }
}
