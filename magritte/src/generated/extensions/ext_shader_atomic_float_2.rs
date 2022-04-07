//![VK_EXT_shader_atomic_float2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_atomic_float2.html) - device extension
//!# Description
//!This extension allows a shader to perform 16-bit floating-point atomic
//!operations on buffer and workgroup memory as well as floating-point atomic
//!minimum and maximum operations on buffer, workgroup, and image memory.
//!It advertises the SPIR-V `AtomicFloat16AddEXT` capability which allows
//!atomic add operations on 16-bit floating-point numbers and the SPIR-V
//!`AtomicFloat16MinMaxEXT`, `AtomicFloat32MinMaxEXT` and
//!`AtomicFloat64MinMaxEXT` capabilities which allow atomic minimum and
//!maximum operations on floating-point numbers.
//!The supported operations include `OpAtomicFAddEXT`, `OpAtomicFMinEXT`
//!and `OpAtomicFMaxEXT`.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_shader_atomic_float`]`
//!# Contacts
//! - Jason Ekstrand [jekstrand](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_atomic_float2]
//!   @jekstrand%0A<<Here describe the issue or question you have about the
//!   VK_EXT_shader_atomic_float2 extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShaderAtomicFloat2FeaturesEXT`]
//!# New constants
//! - [`EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME`]
//! - [`EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT`
//!# Known issues & F.A.Q
//!1) Should this extension add support for 16-bit image atomics? **RESOLVED** : No.
//!While Vulkan supports creating storage images with
//!`VK_FORMAT_R16_SFLOAT` and doing load and store on them, the data in the
//!shader has a 32-bit representation.
//!Vulkan currently has no facility for even basic reading or writing such
//!images using 16-bit float values in the shader.
//!Adding such functionality would be required before 16-bit image atomics
//!would make sense and is outside the scope of this extension.
//!# Version History
//! - Revision 1, 2020-08-14 (Jason Ekstrand)  - Internal revisions
//!# Other info
//! * 2020-08-14
//! * No known IP claims.
//! * - This extension requires the VK_EXT_shader_atomic_float extension.  - This extension requires
//!   [`SPV_EXT_shader_atomic_float_min_max`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_shader_atomic_float_min_max.html)
//!   and [`SPV_EXT_shader_atomic_float16_add`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_shader_atomic_float16_add.html)
//!   - This extension provides API support for [`GLSL_EXT_shader_atomic_float2`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_shader_atomic_float2.txt)
//! * - Jason Ekstrand, Intel
//!# Related
//! - [`PhysicalDeviceShaderAtomicFloat2FeaturesEXT`]
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
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION")]
pub const EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME")]
pub const EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_shader_atomic_float2");
///[VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT.html) - Structure describing features supported by VK_EXT_shader_atomic_float2
///# C Specifications
///The [`PhysicalDeviceShaderAtomicFloat2FeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_shader_atomic_float2
///typedef struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderBufferFloat16Atomics;
///    VkBool32           shaderBufferFloat16AtomicAdd;
///    VkBool32           shaderBufferFloat16AtomicMinMax;
///    VkBool32           shaderBufferFloat32AtomicMinMax;
///    VkBool32           shaderBufferFloat64AtomicMinMax;
///    VkBool32           shaderSharedFloat16Atomics;
///    VkBool32           shaderSharedFloat16AtomicAdd;
///    VkBool32           shaderSharedFloat16AtomicMinMax;
///    VkBool32           shaderSharedFloat32AtomicMinMax;
///    VkBool32           shaderSharedFloat64AtomicMinMax;
///    VkBool32           shaderImageFloat32AtomicMinMax;
///    VkBool32           sparseImageFloat32AtomicMinMax;
///} VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_buffer_float_16_atomics`] indicates whether shaders  **can**  perform 16-bit
///   floating-point load, store, and exchange atomic operations on storage buffers.
/// - [`shader_buffer_float_16_atomic_add`] indicates whether shaders  **can**  perform 16-bit
///   floating-point add atomic operations on storage buffers.
/// - [`shader_buffer_float_16_atomic_min_max`] indicates whether shaders  **can**  perform 16-bit
///   floating-point min and max atomic operations on storage buffers.
/// - [`shader_buffer_float_32_atomic_min_max`] indicates whether shaders  **can**  perform 32-bit
///   floating-point min and max atomic operations on storage buffers.
/// - [`shader_buffer_float_64_atomic_min_max`] indicates whether shaders  **can**  perform 64-bit
///   floating-point min and max atomic operations on storage buffers.
/// - [`shader_shared_float_16_atomics`] indicates whether shaders  **can**  perform 16-bit
///   floating-point load, store and exchange atomic operations on shared memory.
/// - [`shader_shared_float_16_atomic_add`] indicates whether shaders  **can**  perform 16-bit
///   floating-point add atomic operations on shared memory.
/// - [`shader_shared_float_16_atomic_min_max`] indicates whether shaders  **can**  perform 16-bit
///   floating-point min and max atomic operations on shared memory.
/// - [`shader_shared_float_32_atomic_min_max`] indicates whether shaders  **can**  perform 32-bit
///   floating-point min and max atomic operations on shared memory.
/// - [`shader_shared_float_64_atomic_min_max`] indicates whether shaders  **can**  perform 64-bit
///   floating-point min and max atomic operations on shared memory.
/// - [`shader_image_float_32_atomic_min_max`] indicates whether shaders  **can**  perform 32-bit
///   floating-point min and max atomic image operations.
/// - [`sparse_image_float_32_atomic_min_max`] indicates whether 32-bit floating-point min and max
///   atomic operations  **can**  be used on sparse images.
///If the [`PhysicalDeviceShaderAtomicFloat2FeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderAtomicFloat2FeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT`
///# Related
/// - [`VK_EXT_shader_atomic_float2`]
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
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_buffer_float_16_atomics`]
    ///indicates whether shaders  **can**  perform 16-bit floating-point load,
    ///store, and exchange atomic operations on storage buffers.
    pub shader_buffer_float_16_atomics: Bool32,
    ///[`shader_buffer_float_16_atomic_add`] indicates whether shaders  **can**
    ///perform 16-bit floating-point add atomic operations on storage buffers.
    pub shader_buffer_float_16_atomic_add: Bool32,
    ///[`shader_buffer_float_16_atomic_min_max`] indicates whether shaders  **can**
    ///perform 16-bit floating-point min and max atomic operations on storage
    ///buffers.
    pub shader_buffer_float_16_atomic_min_max: Bool32,
    ///[`shader_buffer_float_32_atomic_min_max`] indicates whether shaders  **can**
    ///perform 32-bit floating-point min and max atomic operations on storage
    ///buffers.
    pub shader_buffer_float_32_atomic_min_max: Bool32,
    ///[`shader_buffer_float_64_atomic_min_max`] indicates whether shaders  **can**
    ///perform 64-bit floating-point min and max atomic operations on storage
    ///buffers.
    pub shader_buffer_float_64_atomic_min_max: Bool32,
    ///[`shader_shared_float_16_atomics`]
    ///indicates whether shaders  **can**  perform 16-bit floating-point load, store
    ///and exchange atomic operations on shared memory.
    pub shader_shared_float_16_atomics: Bool32,
    ///[`shader_shared_float_16_atomic_add`] indicates whether shaders  **can**
    ///perform 16-bit floating-point add atomic operations on shared memory.
    pub shader_shared_float_16_atomic_add: Bool32,
    ///[`shader_shared_float_16_atomic_min_max`] indicates whether shaders  **can**
    ///perform 16-bit floating-point min and max atomic operations on shared
    ///memory.
    pub shader_shared_float_16_atomic_min_max: Bool32,
    ///[`shader_shared_float_32_atomic_min_max`] indicates whether shaders  **can**
    ///perform 32-bit floating-point min and max atomic operations on shared
    ///memory.
    pub shader_shared_float_32_atomic_min_max: Bool32,
    ///[`shader_shared_float_64_atomic_min_max`] indicates whether shaders  **can**
    ///perform 64-bit floating-point min and max atomic operations on shared
    ///memory.
    pub shader_shared_float_64_atomic_min_max: Bool32,
    ///[`shader_image_float_32_atomic_min_max`] indicates whether shaders  **can**
    ///perform 32-bit floating-point min and max atomic image operations.
    pub shader_image_float_32_atomic_min_max: Bool32,
    ///[`sparse_image_float_32_atomic_min_max`] indicates whether 32-bit
    ///floating-point min and max atomic operations  **can**  be used on sparse
    ///images.
    pub sparse_image_float_32_atomic_min_max: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT2_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            shader_buffer_float_16_atomics: 0,
            shader_buffer_float_16_atomic_add: 0,
            shader_buffer_float_16_atomic_min_max: 0,
            shader_buffer_float_32_atomic_min_max: 0,
            shader_buffer_float_64_atomic_min_max: 0,
            shader_shared_float_16_atomics: 0,
            shader_shared_float_16_atomic_add: 0,
            shader_shared_float_16_atomic_min_max: 0,
            shader_shared_float_32_atomic_min_max: 0,
            shader_shared_float_64_atomic_min_max: 0,
            shader_image_float_32_atomic_min_max: 0,
            sparse_image_float_32_atomic_min_max: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderAtomicFloat2FeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::shader_buffer_float_16_atomics`]
    pub fn shader_buffer_float_16_atomics_raw(&self) -> Bool32 {
        self.shader_buffer_float_16_atomics
    }
    ///Gets the raw value of [`Self::shader_buffer_float_16_atomic_add`]
    pub fn shader_buffer_float_16_atomic_add_raw(&self) -> Bool32 {
        self.shader_buffer_float_16_atomic_add
    }
    ///Gets the raw value of [`Self::shader_buffer_float_16_atomic_min_max`]
    pub fn shader_buffer_float_16_atomic_min_max_raw(&self) -> Bool32 {
        self.shader_buffer_float_16_atomic_min_max
    }
    ///Gets the raw value of [`Self::shader_buffer_float_32_atomic_min_max`]
    pub fn shader_buffer_float_32_atomic_min_max_raw(&self) -> Bool32 {
        self.shader_buffer_float_32_atomic_min_max
    }
    ///Gets the raw value of [`Self::shader_buffer_float_64_atomic_min_max`]
    pub fn shader_buffer_float_64_atomic_min_max_raw(&self) -> Bool32 {
        self.shader_buffer_float_64_atomic_min_max
    }
    ///Gets the raw value of [`Self::shader_shared_float_16_atomics`]
    pub fn shader_shared_float_16_atomics_raw(&self) -> Bool32 {
        self.shader_shared_float_16_atomics
    }
    ///Gets the raw value of [`Self::shader_shared_float_16_atomic_add`]
    pub fn shader_shared_float_16_atomic_add_raw(&self) -> Bool32 {
        self.shader_shared_float_16_atomic_add
    }
    ///Gets the raw value of [`Self::shader_shared_float_16_atomic_min_max`]
    pub fn shader_shared_float_16_atomic_min_max_raw(&self) -> Bool32 {
        self.shader_shared_float_16_atomic_min_max
    }
    ///Gets the raw value of [`Self::shader_shared_float_32_atomic_min_max`]
    pub fn shader_shared_float_32_atomic_min_max_raw(&self) -> Bool32 {
        self.shader_shared_float_32_atomic_min_max
    }
    ///Gets the raw value of [`Self::shader_shared_float_64_atomic_min_max`]
    pub fn shader_shared_float_64_atomic_min_max_raw(&self) -> Bool32 {
        self.shader_shared_float_64_atomic_min_max
    }
    ///Gets the raw value of [`Self::shader_image_float_32_atomic_min_max`]
    pub fn shader_image_float_32_atomic_min_max_raw(&self) -> Bool32 {
        self.shader_image_float_32_atomic_min_max
    }
    ///Gets the raw value of [`Self::sparse_image_float_32_atomic_min_max`]
    pub fn sparse_image_float_32_atomic_min_max_raw(&self) -> Bool32 {
        self.sparse_image_float_32_atomic_min_max
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_float_16_atomics`]
    pub fn set_shader_buffer_float_16_atomics_raw(mut self, value: Bool32) -> Self {
        self.shader_buffer_float_16_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_float_16_atomic_add`]
    pub fn set_shader_buffer_float_16_atomic_add_raw(mut self, value: Bool32) -> Self {
        self.shader_buffer_float_16_atomic_add = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_float_16_atomic_min_max`]
    pub fn set_shader_buffer_float_16_atomic_min_max_raw(mut self, value: Bool32) -> Self {
        self.shader_buffer_float_16_atomic_min_max = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_float_32_atomic_min_max`]
    pub fn set_shader_buffer_float_32_atomic_min_max_raw(mut self, value: Bool32) -> Self {
        self.shader_buffer_float_32_atomic_min_max = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_float_64_atomic_min_max`]
    pub fn set_shader_buffer_float_64_atomic_min_max_raw(mut self, value: Bool32) -> Self {
        self.shader_buffer_float_64_atomic_min_max = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_float_16_atomics`]
    pub fn set_shader_shared_float_16_atomics_raw(mut self, value: Bool32) -> Self {
        self.shader_shared_float_16_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_float_16_atomic_add`]
    pub fn set_shader_shared_float_16_atomic_add_raw(mut self, value: Bool32) -> Self {
        self.shader_shared_float_16_atomic_add = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_float_16_atomic_min_max`]
    pub fn set_shader_shared_float_16_atomic_min_max_raw(mut self, value: Bool32) -> Self {
        self.shader_shared_float_16_atomic_min_max = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_float_32_atomic_min_max`]
    pub fn set_shader_shared_float_32_atomic_min_max_raw(mut self, value: Bool32) -> Self {
        self.shader_shared_float_32_atomic_min_max = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_float_64_atomic_min_max`]
    pub fn set_shader_shared_float_64_atomic_min_max_raw(mut self, value: Bool32) -> Self {
        self.shader_shared_float_64_atomic_min_max = value;
        self
    }
    ///Sets the raw value of [`Self::shader_image_float_32_atomic_min_max`]
    pub fn set_shader_image_float_32_atomic_min_max_raw(mut self, value: Bool32) -> Self {
        self.shader_image_float_32_atomic_min_max = value;
        self
    }
    ///Sets the raw value of [`Self::sparse_image_float_32_atomic_min_max`]
    pub fn set_sparse_image_float_32_atomic_min_max_raw(mut self, value: Bool32) -> Self {
        self.sparse_image_float_32_atomic_min_max = value;
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
    ///Gets the value of [`Self::shader_buffer_float_16_atomics`]
    pub fn shader_buffer_float_16_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_float_16_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_buffer_float_16_atomic_add`]
    pub fn shader_buffer_float_16_atomic_add(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_float_16_atomic_add as u8) }
    }
    ///Gets the value of [`Self::shader_buffer_float_16_atomic_min_max`]
    pub fn shader_buffer_float_16_atomic_min_max(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_float_16_atomic_min_max as u8) }
    }
    ///Gets the value of [`Self::shader_buffer_float_32_atomic_min_max`]
    pub fn shader_buffer_float_32_atomic_min_max(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_float_32_atomic_min_max as u8) }
    }
    ///Gets the value of [`Self::shader_buffer_float_64_atomic_min_max`]
    pub fn shader_buffer_float_64_atomic_min_max(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_float_64_atomic_min_max as u8) }
    }
    ///Gets the value of [`Self::shader_shared_float_16_atomics`]
    pub fn shader_shared_float_16_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_float_16_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_shared_float_16_atomic_add`]
    pub fn shader_shared_float_16_atomic_add(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_float_16_atomic_add as u8) }
    }
    ///Gets the value of [`Self::shader_shared_float_16_atomic_min_max`]
    pub fn shader_shared_float_16_atomic_min_max(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_float_16_atomic_min_max as u8) }
    }
    ///Gets the value of [`Self::shader_shared_float_32_atomic_min_max`]
    pub fn shader_shared_float_32_atomic_min_max(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_float_32_atomic_min_max as u8) }
    }
    ///Gets the value of [`Self::shader_shared_float_64_atomic_min_max`]
    pub fn shader_shared_float_64_atomic_min_max(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_float_64_atomic_min_max as u8) }
    }
    ///Gets the value of [`Self::shader_image_float_32_atomic_min_max`]
    pub fn shader_image_float_32_atomic_min_max(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_image_float_32_atomic_min_max as u8) }
    }
    ///Gets the value of [`Self::sparse_image_float_32_atomic_min_max`]
    pub fn sparse_image_float_32_atomic_min_max(&self) -> bool {
        unsafe { std::mem::transmute(self.sparse_image_float_32_atomic_min_max as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_buffer_float_16_atomics`]
    pub fn shader_buffer_float_16_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_float_16_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_float_16_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_buffer_float_16_atomic_add`]
    pub fn shader_buffer_float_16_atomic_add_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_float_16_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_float_16_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_buffer_float_16_atomic_min_max`]
    pub fn shader_buffer_float_16_atomic_min_max_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_float_16_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_float_16_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_buffer_float_32_atomic_min_max`]
    pub fn shader_buffer_float_32_atomic_min_max_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_float_32_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_float_32_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_buffer_float_64_atomic_min_max`]
    pub fn shader_buffer_float_64_atomic_min_max_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_float_64_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_float_64_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_float_16_atomics`]
    pub fn shader_shared_float_16_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_float_16_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_float_16_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_float_16_atomic_add`]
    pub fn shader_shared_float_16_atomic_add_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_float_16_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_float_16_atomic_add as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_float_16_atomic_min_max`]
    pub fn shader_shared_float_16_atomic_min_max_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_float_16_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_float_16_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_float_32_atomic_min_max`]
    pub fn shader_shared_float_32_atomic_min_max_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_float_32_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_float_32_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_float_64_atomic_min_max`]
    pub fn shader_shared_float_64_atomic_min_max_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_float_64_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_float_64_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_image_float_32_atomic_min_max`]
    pub fn shader_image_float_32_atomic_min_max_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_image_float_32_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_image_float_32_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::sparse_image_float_32_atomic_min_max`]
    pub fn sparse_image_float_32_atomic_min_max_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.sparse_image_float_32_atomic_min_max as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.sparse_image_float_32_atomic_min_max as *mut Bool32)
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
    ///Sets the value of [`Self::shader_buffer_float_16_atomics`]
    pub fn set_shader_buffer_float_16_atomics(mut self, value: bool) -> Self {
        self.shader_buffer_float_16_atomics = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_buffer_float_16_atomic_add`]
    pub fn set_shader_buffer_float_16_atomic_add(mut self, value: bool) -> Self {
        self.shader_buffer_float_16_atomic_add = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_buffer_float_16_atomic_min_max`]
    pub fn set_shader_buffer_float_16_atomic_min_max(mut self, value: bool) -> Self {
        self.shader_buffer_float_16_atomic_min_max = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_buffer_float_32_atomic_min_max`]
    pub fn set_shader_buffer_float_32_atomic_min_max(mut self, value: bool) -> Self {
        self.shader_buffer_float_32_atomic_min_max = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_buffer_float_64_atomic_min_max`]
    pub fn set_shader_buffer_float_64_atomic_min_max(mut self, value: bool) -> Self {
        self.shader_buffer_float_64_atomic_min_max = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_shared_float_16_atomics`]
    pub fn set_shader_shared_float_16_atomics(mut self, value: bool) -> Self {
        self.shader_shared_float_16_atomics = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_shared_float_16_atomic_add`]
    pub fn set_shader_shared_float_16_atomic_add(mut self, value: bool) -> Self {
        self.shader_shared_float_16_atomic_add = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_shared_float_16_atomic_min_max`]
    pub fn set_shader_shared_float_16_atomic_min_max(mut self, value: bool) -> Self {
        self.shader_shared_float_16_atomic_min_max = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_shared_float_32_atomic_min_max`]
    pub fn set_shader_shared_float_32_atomic_min_max(mut self, value: bool) -> Self {
        self.shader_shared_float_32_atomic_min_max = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_shared_float_64_atomic_min_max`]
    pub fn set_shader_shared_float_64_atomic_min_max(mut self, value: bool) -> Self {
        self.shader_shared_float_64_atomic_min_max = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::shader_image_float_32_atomic_min_max`]
    pub fn set_shader_image_float_32_atomic_min_max(mut self, value: bool) -> Self {
        self.shader_image_float_32_atomic_min_max = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::sparse_image_float_32_atomic_min_max`]
    pub fn set_sparse_image_float_32_atomic_min_max(mut self, value: bool) -> Self {
        self.sparse_image_float_32_atomic_min_max = value as u8 as u32;
        self
    }
}
