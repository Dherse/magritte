//![VK_EXT_shader_atomic_float](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_atomic_float.html) - device extension
//!# Description
//!This extension allows a shader to contain floating-point atomic operations
//!on buffer, workgroup, and image memory.
//!It also advertises the SPIR-V `AtomicFloat32AddEXT` and
//!`AtomicFloat64AddEXT` capabilities that allows atomic addition on
//!floating-points numbers.
//!The supported operations include `OpAtomicFAddEXT`,
//!`OpAtomicExchange`, `OpAtomicLoad` and `OpAtomicStore`.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Vikram Kushwaha [vkushwaha-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_atomic_float]
//!   @vkushwaha-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_shader_atomic_float extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`]
//!# New constants
//! - [`EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME`]
//! - [`EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2020-07-15 (Vikram Kushwaha)  - Internal revisions
//!# Other info
//! * 2020-07-15
//! * No known IP claims.
//! * - This extension requires [`SPV_EXT_shader_atomic_float_add`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_shader_atomic_float_add.html)
//!   - This extension provides API support for [`GL_EXT_shader_atomic_float`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_shader_atomic_float.txt)
//! * - Vikram Kushwaha, NVIDIA  - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`]
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
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION")]
pub const EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME")]
pub const EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_shader_atomic_float");
///[VkPhysicalDeviceShaderAtomicFloatFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloatFeaturesEXT.html) - Structure describing features supported by VK_EXT_shader_atomic_float
///# C Specifications
///The [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_shader_atomic_float
///typedef struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderBufferFloat32Atomics;
///    VkBool32           shaderBufferFloat32AtomicAdd;
///    VkBool32           shaderBufferFloat64Atomics;
///    VkBool32           shaderBufferFloat64AtomicAdd;
///    VkBool32           shaderSharedFloat32Atomics;
///    VkBool32           shaderSharedFloat32AtomicAdd;
///    VkBool32           shaderSharedFloat64Atomics;
///    VkBool32           shaderSharedFloat64AtomicAdd;
///    VkBool32           shaderImageFloat32Atomics;
///    VkBool32           shaderImageFloat32AtomicAdd;
///    VkBool32           sparseImageFloat32Atomics;
///    VkBool32           sparseImageFloat32AtomicAdd;
///} VkPhysicalDeviceShaderAtomicFloatFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_buffer_float32_atomics`] indicates whether shaders  **can**  perform 32-bit
///   floating-point load, store and exchange atomic operations on storage buffers.
/// - [`shader_buffer_float32_atomic_add`] indicates whether shaders  **can**  perform 32-bit
///   floating-point add atomic operations on storage buffers.
/// - [`shader_buffer_float64_atomics`] indicates whether shaders  **can**  perform 64-bit
///   floating-point load, store and exchange atomic operations on storage buffers.
/// - [`shader_buffer_float64_atomic_add`] indicates whether shaders  **can**  perform 64-bit
///   floating-point add atomic operations on storage buffers.
/// - [`shader_shared_float32_atomics`] indicates whether shaders  **can**  perform 32-bit
///   floating-point load, store and exchange atomic operations on shared memory.
/// - [`shader_shared_float32_atomic_add`] indicates whether shaders  **can**  perform 32-bit
///   floating-point add atomic operations on shared memory.
/// - [`shader_shared_float64_atomics`] indicates whether shaders  **can**  perform 64-bit
///   floating-point load, store and exchange atomic operations on shared memory.
/// - [`shader_shared_float64_atomic_add`] indicates whether shaders  **can**  perform 64-bit
///   floating-point add atomic operations on shared memory.
/// - [`shader_image_float32_atomics`] indicates whether shaders  **can**  perform 32-bit
///   floating-point load, store and exchange atomic image operations.
/// - [`shader_image_float32_atomic_add`] indicates whether shaders  **can**  perform 32-bit
///   floating-point add atomic image operations.
/// - [`sparse_image_float32_atomics`] indicates whether 32-bit floating-point load, store and
///   exchange atomic operations  **can**  be used on sparse images.
/// - [`sparse_image_float32_atomic_add`] indicates whether 32-bit floating-point add atomic
///   operations  **can**  be used on sparse images.
///If the [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderAtomicFloatFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT`
///# Related
/// - [`ext_shader_atomic_float`]
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
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloatFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_buffer_float32_atomics`]
    ///indicates whether shaders  **can**  perform 32-bit floating-point load, store
    ///and exchange atomic operations on storage buffers.
    pub shader_buffer_float32_atomics: Bool32,
    ///[`shader_buffer_float32_atomic_add`] indicates whether shaders  **can**
    ///perform 32-bit floating-point add atomic operations on storage buffers.
    pub shader_buffer_float32_atomic_add: Bool32,
    ///[`shader_buffer_float64_atomics`]
    ///indicates whether shaders  **can**  perform 64-bit floating-point load, store
    ///and exchange atomic operations on storage buffers.
    pub shader_buffer_float64_atomics: Bool32,
    ///[`shader_buffer_float64_atomic_add`] indicates whether shaders  **can**
    ///perform 64-bit floating-point add atomic operations on storage buffers.
    pub shader_buffer_float64_atomic_add: Bool32,
    ///[`shader_shared_float32_atomics`]
    ///indicates whether shaders  **can**  perform 32-bit floating-point load, store
    ///and exchange atomic operations on shared memory.
    pub shader_shared_float32_atomics: Bool32,
    ///[`shader_shared_float32_atomic_add`] indicates whether shaders  **can**
    ///perform 32-bit floating-point add atomic operations on shared memory.
    pub shader_shared_float32_atomic_add: Bool32,
    ///[`shader_shared_float64_atomics`]
    ///indicates whether shaders  **can**  perform 64-bit floating-point load, store
    ///and exchange atomic operations on shared memory.
    pub shader_shared_float64_atomics: Bool32,
    ///[`shader_shared_float64_atomic_add`] indicates whether shaders  **can**
    ///perform 64-bit floating-point add atomic operations on shared memory.
    pub shader_shared_float64_atomic_add: Bool32,
    ///[`shader_image_float32_atomics`]
    ///indicates whether shaders  **can**  perform 32-bit floating-point load, store
    ///and exchange atomic image operations.
    pub shader_image_float32_atomics: Bool32,
    ///[`shader_image_float32_atomic_add`] indicates whether shaders  **can**  perform
    ///32-bit floating-point add atomic image operations.
    pub shader_image_float32_atomic_add: Bool32,
    ///[`sparse_image_float32_atomics`]
    ///indicates whether 32-bit floating-point load, store and exchange atomic
    ///operations  **can**  be used on sparse images.
    pub sparse_image_float32_atomics: Bool32,
    ///[`sparse_image_float32_atomic_add`] indicates whether 32-bit
    ///floating-point add atomic operations  **can**  be used on sparse images.
    pub sparse_image_float32_atomic_add: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderAtomicFloatFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            shader_buffer_float32_atomics: 0,
            shader_buffer_float32_atomic_add: 0,
            shader_buffer_float64_atomics: 0,
            shader_buffer_float64_atomic_add: 0,
            shader_shared_float32_atomics: 0,
            shader_shared_float32_atomic_add: 0,
            shader_shared_float64_atomics: 0,
            shader_shared_float64_atomic_add: 0,
            shader_image_float32_atomics: 0,
            shader_image_float32_atomic_add: 0,
            sparse_image_float32_atomics: 0,
            sparse_image_float32_atomic_add: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderAtomicFloatFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::shader_buffer_float32_atomics`]
    pub fn shader_buffer_float32_atomics_raw(&self) -> Bool32 {
        self.shader_buffer_float32_atomics
    }
    ///Gets the raw value of [`Self::shader_buffer_float32_atomic_add`]
    pub fn shader_buffer_float32_atomic_add_raw(&self) -> Bool32 {
        self.shader_buffer_float32_atomic_add
    }
    ///Gets the raw value of [`Self::shader_buffer_float64_atomics`]
    pub fn shader_buffer_float64_atomics_raw(&self) -> Bool32 {
        self.shader_buffer_float64_atomics
    }
    ///Gets the raw value of [`Self::shader_buffer_float64_atomic_add`]
    pub fn shader_buffer_float64_atomic_add_raw(&self) -> Bool32 {
        self.shader_buffer_float64_atomic_add
    }
    ///Gets the raw value of [`Self::shader_shared_float32_atomics`]
    pub fn shader_shared_float32_atomics_raw(&self) -> Bool32 {
        self.shader_shared_float32_atomics
    }
    ///Gets the raw value of [`Self::shader_shared_float32_atomic_add`]
    pub fn shader_shared_float32_atomic_add_raw(&self) -> Bool32 {
        self.shader_shared_float32_atomic_add
    }
    ///Gets the raw value of [`Self::shader_shared_float64_atomics`]
    pub fn shader_shared_float64_atomics_raw(&self) -> Bool32 {
        self.shader_shared_float64_atomics
    }
    ///Gets the raw value of [`Self::shader_shared_float64_atomic_add`]
    pub fn shader_shared_float64_atomic_add_raw(&self) -> Bool32 {
        self.shader_shared_float64_atomic_add
    }
    ///Gets the raw value of [`Self::shader_image_float32_atomics`]
    pub fn shader_image_float32_atomics_raw(&self) -> Bool32 {
        self.shader_image_float32_atomics
    }
    ///Gets the raw value of [`Self::shader_image_float32_atomic_add`]
    pub fn shader_image_float32_atomic_add_raw(&self) -> Bool32 {
        self.shader_image_float32_atomic_add
    }
    ///Gets the raw value of [`Self::sparse_image_float32_atomics`]
    pub fn sparse_image_float32_atomics_raw(&self) -> Bool32 {
        self.sparse_image_float32_atomics
    }
    ///Gets the raw value of [`Self::sparse_image_float32_atomic_add`]
    pub fn sparse_image_float32_atomic_add_raw(&self) -> Bool32 {
        self.sparse_image_float32_atomic_add
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_float32_atomics`]
    pub fn set_shader_buffer_float32_atomics_raw(mut self, value: Bool32) -> Self {
        self.shader_buffer_float32_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_float32_atomic_add`]
    pub fn set_shader_buffer_float32_atomic_add_raw(mut self, value: Bool32) -> Self {
        self.shader_buffer_float32_atomic_add = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_float64_atomics`]
    pub fn set_shader_buffer_float64_atomics_raw(mut self, value: Bool32) -> Self {
        self.shader_buffer_float64_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_float64_atomic_add`]
    pub fn set_shader_buffer_float64_atomic_add_raw(mut self, value: Bool32) -> Self {
        self.shader_buffer_float64_atomic_add = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_float32_atomics`]
    pub fn set_shader_shared_float32_atomics_raw(mut self, value: Bool32) -> Self {
        self.shader_shared_float32_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_float32_atomic_add`]
    pub fn set_shader_shared_float32_atomic_add_raw(mut self, value: Bool32) -> Self {
        self.shader_shared_float32_atomic_add = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_float64_atomics`]
    pub fn set_shader_shared_float64_atomics_raw(mut self, value: Bool32) -> Self {
        self.shader_shared_float64_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_float64_atomic_add`]
    pub fn set_shader_shared_float64_atomic_add_raw(mut self, value: Bool32) -> Self {
        self.shader_shared_float64_atomic_add = value;
        self
    }
    ///Sets the raw value of [`Self::shader_image_float32_atomics`]
    pub fn set_shader_image_float32_atomics_raw(mut self, value: Bool32) -> Self {
        self.shader_image_float32_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_image_float32_atomic_add`]
    pub fn set_shader_image_float32_atomic_add_raw(mut self, value: Bool32) -> Self {
        self.shader_image_float32_atomic_add = value;
        self
    }
    ///Sets the raw value of [`Self::sparse_image_float32_atomics`]
    pub fn set_sparse_image_float32_atomics_raw(mut self, value: Bool32) -> Self {
        self.sparse_image_float32_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::sparse_image_float32_atomic_add`]
    pub fn set_sparse_image_float32_atomic_add_raw(mut self, value: Bool32) -> Self {
        self.sparse_image_float32_atomic_add = value;
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
    ///Gets the value of [`Self::shader_buffer_float32_atomics`]
    pub fn shader_buffer_float32_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_float32_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_buffer_float32_atomic_add`]
    pub fn shader_buffer_float32_atomic_add(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_float32_atomic_add as u8) }
    }
    ///Gets the value of [`Self::shader_buffer_float64_atomics`]
    pub fn shader_buffer_float64_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_float64_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_buffer_float64_atomic_add`]
    pub fn shader_buffer_float64_atomic_add(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_float64_atomic_add as u8) }
    }
    ///Gets the value of [`Self::shader_shared_float32_atomics`]
    pub fn shader_shared_float32_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_float32_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_shared_float32_atomic_add`]
    pub fn shader_shared_float32_atomic_add(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_float32_atomic_add as u8) }
    }
    ///Gets the value of [`Self::shader_shared_float64_atomics`]
    pub fn shader_shared_float64_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_float64_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_shared_float64_atomic_add`]
    pub fn shader_shared_float64_atomic_add(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_float64_atomic_add as u8) }
    }
    ///Gets the value of [`Self::shader_image_float32_atomics`]
    pub fn shader_image_float32_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_image_float32_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_image_float32_atomic_add`]
    pub fn shader_image_float32_atomic_add(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_image_float32_atomic_add as u8) }
    }
    ///Gets the value of [`Self::sparse_image_float32_atomics`]
    pub fn sparse_image_float32_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.sparse_image_float32_atomics as u8) }
    }
    ///Gets the value of [`Self::sparse_image_float32_atomic_add`]
    pub fn sparse_image_float32_atomic_add(&self) -> bool {
        unsafe { std::mem::transmute(self.sparse_image_float32_atomic_add as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_buffer_float32_atomics`]
    pub fn shader_buffer_float32_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_float32_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_float32_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_buffer_float32_atomic_add`]
    pub fn shader_buffer_float32_atomic_add_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_float32_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_float32_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_buffer_float64_atomics`]
    pub fn shader_buffer_float64_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_float64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_float64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_buffer_float64_atomic_add`]
    pub fn shader_buffer_float64_atomic_add_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_float64_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_float64_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_float32_atomics`]
    pub fn shader_shared_float32_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_float32_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_float32_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_float32_atomic_add`]
    pub fn shader_shared_float32_atomic_add_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_float32_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_float32_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_float64_atomics`]
    pub fn shader_shared_float64_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_float64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_float64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_float64_atomic_add`]
    pub fn shader_shared_float64_atomic_add_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_float64_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_float64_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_image_float32_atomics`]
    pub fn shader_image_float32_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_image_float32_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_image_float32_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_image_float32_atomic_add`]
    pub fn shader_image_float32_atomic_add_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_image_float32_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_image_float32_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::sparse_image_float32_atomics`]
    pub fn sparse_image_float32_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.sparse_image_float32_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.sparse_image_float32_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::sparse_image_float32_atomic_add`]
    pub fn sparse_image_float32_atomic_add_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.sparse_image_float32_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.sparse_image_float32_atomic_add as *mut Bool32)
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
    ///Sets the value of [`Self::shader_buffer_float32_atomics`]
    pub fn set_shader_buffer_float32_atomics(mut self, value: bool) -> Self {
        self.shader_buffer_float32_atomics = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_buffer_float32_atomic_add`]
    pub fn set_shader_buffer_float32_atomic_add(mut self, value: bool) -> Self {
        self.shader_buffer_float32_atomic_add = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_buffer_float64_atomics`]
    pub fn set_shader_buffer_float64_atomics(mut self, value: bool) -> Self {
        self.shader_buffer_float64_atomics = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_buffer_float64_atomic_add`]
    pub fn set_shader_buffer_float64_atomic_add(mut self, value: bool) -> Self {
        self.shader_buffer_float64_atomic_add = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_shared_float32_atomics`]
    pub fn set_shader_shared_float32_atomics(mut self, value: bool) -> Self {
        self.shader_shared_float32_atomics = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_shared_float32_atomic_add`]
    pub fn set_shader_shared_float32_atomic_add(mut self, value: bool) -> Self {
        self.shader_shared_float32_atomic_add = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_shared_float64_atomics`]
    pub fn set_shader_shared_float64_atomics(mut self, value: bool) -> Self {
        self.shader_shared_float64_atomics = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_shared_float64_atomic_add`]
    pub fn set_shader_shared_float64_atomic_add(mut self, value: bool) -> Self {
        self.shader_shared_float64_atomic_add = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_image_float32_atomics`]
    pub fn set_shader_image_float32_atomics(mut self, value: bool) -> Self {
        self.shader_image_float32_atomics = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_image_float32_atomic_add`]
    pub fn set_shader_image_float32_atomic_add(mut self, value: bool) -> Self {
        self.shader_image_float32_atomic_add = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::sparse_image_float32_atomics`]
    pub fn set_sparse_image_float32_atomics(mut self, value: bool) -> Self {
        self.sparse_image_float32_atomics = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::sparse_image_float32_atomic_add`]
    pub fn set_sparse_image_float32_atomic_add(mut self, value: bool) -> Self {
        self.sparse_image_float32_atomic_add = value as u8 as u32;
        self
    }
}
