//![VK_EXT_texel_buffer_alignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_texel_buffer_alignment.html) - device extension
//!# Description
//!This extension adds more expressive alignment requirements for uniform and
//!storage texel buffers.
//!Some implementations have single texel alignment requirements that cannot be
//!expressed via
//![`PhysicalDeviceLimits::min_texel_buffer_offset_alignment`].
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Deprecation State
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_texel_buffer_alignment]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_texel_buffer_alignment extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  -
//!   [`PhysicalDeviceTexelBufferAlignmentPropertiesEXT`]
//!# New constants
//! - [`EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME`]
//! - [`EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT`
//!# Version history
//! - Revision 1, 2019-06-06 (Jeff Bolz)  - Initial draft
//!# Other information
//! * 2019-06-06
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`]
//! - [`PhysicalDeviceTexelBufferAlignmentPropertiesEXT`]
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
#[doc(alias = "VK_EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION")]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME")]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_texel_buffer_alignment");
///[VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html) - Structure describing the texel buffer alignment features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_texel_buffer_alignment
///typedef struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           texelBufferAlignment;
///} VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT;
///```
/// # Members
/// This structure describes the following feature:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`texel_buffer_alignment`] indicates whether the implementation uses more specific alignment
///   requirements advertised in [`PhysicalDeviceTexelBufferAlignmentProperties`] rather than
///   [`PhysicalDeviceLimits::min_texel_buffer_offset_alignment`].
/// If the [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceTexelBufferAlignmentFeaturesEXT`] **can**  also be used in the [`p_next`] chain
/// of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT`
/// # Related
/// - [`ext_texel_buffer_alignment`]
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
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`texel_buffer_alignment`] indicates
    ///whether the implementation uses more specific alignment requirements
    ///advertised in [`PhysicalDeviceTexelBufferAlignmentProperties`]
    ///rather than
    ///[`PhysicalDeviceLimits`]::`minTexelBufferOffsetAlignment`.
    pub texel_buffer_alignment: Bool32,
}
impl<'lt> Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            texel_buffer_alignment: 0,
        }
    }
}
impl<'lt> PhysicalDeviceTexelBufferAlignmentFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::texel_buffer_alignment`]
    pub fn texel_buffer_alignment_raw(&self) -> Bool32 {
        self.texel_buffer_alignment
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::texel_buffer_alignment`]
    pub fn set_texel_buffer_alignment_raw(&mut self, value: Bool32) -> &mut Self {
        self.texel_buffer_alignment = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::texel_buffer_alignment`]
    pub fn with_texel_buffer_alignment_raw(mut self, value: Bool32) -> Self {
        self.texel_buffer_alignment = value;
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
    ///Gets the value of [`Self::texel_buffer_alignment`]
    pub fn texel_buffer_alignment(&self) -> bool {
        unsafe { std::mem::transmute(self.texel_buffer_alignment as u8) }
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
    ///Gets a mutable reference to the value of [`Self::texel_buffer_alignment`]
    pub fn texel_buffer_alignment_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.texel_buffer_alignment as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.texel_buffer_alignment as *mut Bool32)
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
    ///Sets the value of [`Self::texel_buffer_alignment`]
    pub fn set_texel_buffer_alignment(&mut self, value: bool) -> &mut Self {
        self.texel_buffer_alignment = value as u8 as u32;
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
    ///Sets the value of [`Self::texel_buffer_alignment`]
    pub fn with_texel_buffer_alignment(mut self, value: bool) -> Self {
        self.texel_buffer_alignment = value as u8 as u32;
        self
    }
}
