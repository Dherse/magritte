//![VK_EXT_robustness2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_robustness2.html) - device extension
//!# Description
//!This extension adds stricter requirements for how out of bounds reads and
//!writes are handled.
//!Most accesses  **must**  be tightly bounds-checked, out of bounds writes  **must**  be
//!discarded, out of bound reads  **must**  return zero.
//!Rather than allowing multiple possible (0,0,0,x) vectors, the out of
//!bounds values are treated as zero, and then missing components are inserted
//!based on the format as described in [Conversion to RGBA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-conversion-to-rgba) and [vertex input attribute
//!extraction](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fxvertex-input-extraction).These additional requirements  **may**  be expensive on some implementations, and
//!should only be enabled when truly necessary.This extension also adds support for “null
//! descriptors”, where
//![`crate::Handle::null`] **can**  be used instead of a valid handle.
//!Accesses to null descriptors have well-defined behavior, and do not rely on
//!robustness.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Liam Middlebrook [liam-middlebrook](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_robustness2]
//!   @liam-middlebrook%0A<<Here describe the issue or question you have about the
//!   VK_EXT_robustness2 extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceRobustness2FeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceRobustness2PropertiesEXT`]
//!# New constants
//! - [`EXT_ROBUSTNESS_2_EXTENSION_NAME`]
//! - [`EXT_ROBUSTNESS_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT`
//!   - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT`
//!# Known issues & F.A.Q
//!0. Why do [`PhysicalDeviceRobustness2PropertiesEXT::
//! robust_uniform_buffer_access_size_alignment`] and
//! [`PhysicalDeviceRobustness2PropertiesEXT::robust_storage_buffer_access_size_alignment`] exist?
//! **RESOLVED** : Some implementations cannot efficiently tightly bounds-check all
//!buffer accesses.
//!Rather, the size of the bound range is padded to some power of two multiple,
//!up to 256 bytes for uniform buffers and up to 4 bytes for storage buffers,
//!and that padded size is bounds-checked.
//!This is sufficient to implement D3D-like behavior, because D3D only allows
//!binding whole uniform buffers or ranges that are a multiple of 256 bytes,
//!and D3D raw and structured buffers only support 32-bit accesses.
//!# Version History
//! - Revision 1, 2019-11-01 (Jeff Bolz, Liam Middlebrook)  - Initial draft
//!# Other info
//! * 2020-01-29
//! * No known IP claims.
//! * - Liam Middlebrook, NVIDIA  - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceRobustness2FeaturesEXT`]
//! - [`PhysicalDeviceRobustness2PropertiesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, DeviceSize, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ROBUSTNESS_2_SPEC_VERSION")]
pub const EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_ROBUSTNESS_2_EXTENSION_NAME")]
pub const EXT_ROBUSTNESS_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_robustness2");
///[VkPhysicalDeviceRobustness2FeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2FeaturesEXT.html) - Structure describing the out-of-bounds behavior for an implementation
///# C Specifications
///The [`PhysicalDeviceRobustness2FeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_robustness2
///typedef struct VkPhysicalDeviceRobustness2FeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           robustBufferAccess2;
///    VkBool32           robustImageAccess2;
///    VkBool32           nullDescriptor;
///} VkPhysicalDeviceRobustness2FeaturesEXT;
///```
/// # Members
/// This structure describes the following features:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`robust_buffer_access2`] indicates whether buffer accesses are tightly bounds-checked against
///   the range of the descriptor. Uniform buffers  **must**  be bounds-checked to the range of the descriptor,
///   where the range is rounded up to a multiple of [robustUniformBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustUniformBufferAccessSizeAlignment).
///   Storage buffers  **must**  be bounds-checked to the range of the descriptor, where the range is
///   rounded up to a multiple of [robustStorageBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustStorageBufferAccessSizeAlignment).
///   Out of bounds buffer loads will return zero values, and formatted loads will have (0,0,1)
///   values inserted for missing G, B, or A components based on the format.
/// - [`robust_image_access2`] indicates whether image accesses are tightly bounds-checked against
///   the dimensions of the image view. Out of bounds image loads will return zero values, with
///   (0,0,1) values [inserted for missing G, B, or A components]() based on the format.
/// - [`null_descriptor`] indicates whether descriptors  **can**  be written with a
///   [`crate::Handle::null`] resource or view, which are considered valid to access and act as if
///   the descriptor were bound to nothing.
/// If the [`PhysicalDeviceRobustness2FeaturesEXT`] structure is included in the [`p_next`] chain of
/// the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceRobustness2FeaturesEXT`] **can**  also be used in the [`p_next`] chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage
/// - If [`robust_buffer_access2`] is enabled then [`robustBufferAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess)
///   **must**  also be enabled
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT`
/// # Related
/// - [`ext_robustness2`]
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
#[doc(alias = "VkPhysicalDeviceRobustness2FeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2FeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`robust_buffer_access2`] indicates
    ///whether buffer accesses are tightly bounds-checked against the range of
    ///the descriptor.
    ///Uniform buffers  **must**  be bounds-checked to the range of the descriptor,
    ///where the range is rounded up to a multiple of
    ///[robustUniformBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustUniformBufferAccessSizeAlignment).
    ///Storage buffers  **must**  be bounds-checked to the range of the descriptor,
    ///where the range is rounded up to a multiple of
    ///[robustStorageBufferAccessSizeAlignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-robustStorageBufferAccessSizeAlignment).
    ///Out of bounds buffer loads will return zero values, and formatted loads
    ///will have (0,0,1) values inserted for missing G, B, or A
    ///components based on the format.
    pub robust_buffer_access2: Bool32,
    ///[`robust_image_access2`] indicates
    ///whether image accesses are tightly bounds-checked against the dimensions
    ///of the image view.
    ///Out of bounds image loads will return zero values, with (0,0,1)
    ///values [inserted for missing G, B, or A
    ///components]() based on the format.
    pub robust_image_access2: Bool32,
    ///[`null_descriptor`] indicates whether
    ///descriptors  **can**  be written with a [`crate::Handle::null`] resource or
    ///view, which are considered valid to access and act as if the descriptor
    ///were bound to nothing.
    pub null_descriptor: Bool32,
}
impl<'lt> Default for PhysicalDeviceRobustness2FeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_ROBUSTNESS2_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            robust_buffer_access2: 0,
            robust_image_access2: 0,
            null_descriptor: 0,
        }
    }
}
impl<'lt> PhysicalDeviceRobustness2FeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::robust_buffer_access2`]
    pub fn robust_buffer_access2_raw(&self) -> Bool32 {
        self.robust_buffer_access2
    }
    ///Gets the raw value of [`Self::robust_image_access2`]
    pub fn robust_image_access2_raw(&self) -> Bool32 {
        self.robust_image_access2
    }
    ///Gets the raw value of [`Self::null_descriptor`]
    pub fn null_descriptor_raw(&self) -> Bool32 {
        self.null_descriptor
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::robust_buffer_access2`]
    pub fn set_robust_buffer_access2_raw(&mut self, value: Bool32) -> &mut Self {
        self.robust_buffer_access2 = value;
        self
    }
    ///Sets the raw value of [`Self::robust_image_access2`]
    pub fn set_robust_image_access2_raw(&mut self, value: Bool32) -> &mut Self {
        self.robust_image_access2 = value;
        self
    }
    ///Sets the raw value of [`Self::null_descriptor`]
    pub fn set_null_descriptor_raw(&mut self, value: Bool32) -> &mut Self {
        self.null_descriptor = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::robust_buffer_access2`]
    pub fn with_robust_buffer_access2_raw(mut self, value: Bool32) -> Self {
        self.robust_buffer_access2 = value;
        self
    }
    ///Sets the raw value of [`Self::robust_image_access2`]
    pub fn with_robust_image_access2_raw(mut self, value: Bool32) -> Self {
        self.robust_image_access2 = value;
        self
    }
    ///Sets the raw value of [`Self::null_descriptor`]
    pub fn with_null_descriptor_raw(mut self, value: Bool32) -> Self {
        self.null_descriptor = value;
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
    ///Gets the value of [`Self::robust_buffer_access2`]
    pub fn robust_buffer_access2(&self) -> bool {
        unsafe { std::mem::transmute(self.robust_buffer_access2 as u8) }
    }
    ///Gets the value of [`Self::robust_image_access2`]
    pub fn robust_image_access2(&self) -> bool {
        unsafe { std::mem::transmute(self.robust_image_access2 as u8) }
    }
    ///Gets the value of [`Self::null_descriptor`]
    pub fn null_descriptor(&self) -> bool {
        unsafe { std::mem::transmute(self.null_descriptor as u8) }
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
    ///Gets a mutable reference to the value of [`Self::robust_buffer_access2`]
    pub fn robust_buffer_access2_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.robust_buffer_access2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.robust_buffer_access2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::robust_image_access2`]
    pub fn robust_image_access2_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.robust_image_access2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.robust_image_access2 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::null_descriptor`]
    pub fn null_descriptor_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.null_descriptor as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.null_descriptor as *mut Bool32)
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
    ///Sets the value of [`Self::robust_buffer_access2`]
    pub fn set_robust_buffer_access2(&mut self, value: bool) -> &mut Self {
        self.robust_buffer_access2 = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::robust_image_access2`]
    pub fn set_robust_image_access2(&mut self, value: bool) -> &mut Self {
        self.robust_image_access2 = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::null_descriptor`]
    pub fn set_null_descriptor(&mut self, value: bool) -> &mut Self {
        self.null_descriptor = value as u8 as u32;
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
    ///Sets the value of [`Self::robust_buffer_access2`]
    pub fn with_robust_buffer_access2(mut self, value: bool) -> Self {
        self.robust_buffer_access2 = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::robust_image_access2`]
    pub fn with_robust_image_access2(mut self, value: bool) -> Self {
        self.robust_image_access2 = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::null_descriptor`]
    pub fn with_null_descriptor(mut self, value: bool) -> Self {
        self.null_descriptor = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceRobustness2PropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRobustness2PropertiesEXT.html) - Structure describing robust buffer access properties supported by an implementation
///# C Specifications
///The [`PhysicalDeviceRobustness2PropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_robustness2
///typedef struct VkPhysicalDeviceRobustness2PropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkDeviceSize       robustStorageBufferAccessSizeAlignment;
///    VkDeviceSize       robustUniformBufferAccessSizeAlignment;
///} VkPhysicalDeviceRobustness2PropertiesEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`robust_storage_buffer_access_size_alignment`] is the number of bytes that the range of a storage buffer descriptor is rounded up to when used for bounds-checking when [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled. This value  **must**  be either 1 or 4.
/// - [`robust_uniform_buffer_access_size_alignment`] is the number of bytes that the range of a uniform buffer descriptor is rounded up to when used for bounds-checking when [`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled. This value  **must**  be a power of two in the range [1, 256].
/// # Description
/// If the [`PhysicalDeviceRobustness2PropertiesEXT`] structure is included in the [`p_next`] chain
/// of the
/// [`PhysicalDeviceProperties2`] structure passed to
/// [`get_physical_device_properties2`], it is filled in with each
/// corresponding implementation-dependent property.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT`
/// # Related
/// - [`ext_robustness2`]
/// - [`DeviceSize`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceRobustness2PropertiesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2PropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`robust_storage_buffer_access_size_alignment`] is the number of bytes that
    ///the range of a storage buffer descriptor is rounded up to when used for
    ///bounds-checking when
    ///[`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled.
    ///This value  **must**  be either 1 or 4.
    pub robust_storage_buffer_access_size_alignment: DeviceSize,
    ///[`robust_uniform_buffer_access_size_alignment`] is the number of bytes that
    ///the range of a uniform buffer descriptor is rounded up to when used for
    ///bounds-checking when
    ///[`robustBufferAccess2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess2) is enabled.
    ///This value  **must**  be a power of two in the range [1, 256].
    pub robust_uniform_buffer_access_size_alignment: DeviceSize,
}
impl<'lt> Default for PhysicalDeviceRobustness2PropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_ROBUSTNESS2_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            robust_storage_buffer_access_size_alignment: Default::default(),
            robust_uniform_buffer_access_size_alignment: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceRobustness2PropertiesEXT<'lt> {
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
    ///Gets the value of [`Self::robust_storage_buffer_access_size_alignment`]
    pub fn robust_storage_buffer_access_size_alignment(&self) -> DeviceSize {
        self.robust_storage_buffer_access_size_alignment
    }
    ///Gets the value of [`Self::robust_uniform_buffer_access_size_alignment`]
    pub fn robust_uniform_buffer_access_size_alignment(&self) -> DeviceSize {
        self.robust_uniform_buffer_access_size_alignment
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
    ///Gets a mutable reference to the value of
    /// [`Self::robust_storage_buffer_access_size_alignment`]
    pub fn robust_storage_buffer_access_size_alignment_mut(&mut self) -> &mut DeviceSize {
        &mut self.robust_storage_buffer_access_size_alignment
    }
    ///Gets a mutable reference to the value of
    /// [`Self::robust_uniform_buffer_access_size_alignment`]
    pub fn robust_uniform_buffer_access_size_alignment_mut(&mut self) -> &mut DeviceSize {
        &mut self.robust_uniform_buffer_access_size_alignment
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
    ///Sets the value of [`Self::robust_storage_buffer_access_size_alignment`]
    pub fn set_robust_storage_buffer_access_size_alignment(
        &mut self,
        value: crate::vulkan1_0::DeviceSize,
    ) -> &mut Self {
        self.robust_storage_buffer_access_size_alignment = value;
        self
    }
    ///Sets the value of [`Self::robust_uniform_buffer_access_size_alignment`]
    pub fn set_robust_uniform_buffer_access_size_alignment(
        &mut self,
        value: crate::vulkan1_0::DeviceSize,
    ) -> &mut Self {
        self.robust_uniform_buffer_access_size_alignment = value;
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
    ///Sets the value of [`Self::robust_storage_buffer_access_size_alignment`]
    pub fn with_robust_storage_buffer_access_size_alignment(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.robust_storage_buffer_access_size_alignment = value;
        self
    }
    ///Sets the value of [`Self::robust_uniform_buffer_access_size_alignment`]
    pub fn with_robust_uniform_buffer_access_size_alignment(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.robust_uniform_buffer_access_size_alignment = value;
        self
    }
}
