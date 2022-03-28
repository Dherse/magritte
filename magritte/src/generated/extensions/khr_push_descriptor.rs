//![VK_KHR_push_descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_push_descriptor.html) - device extension
//!# Description
//!This extension allows descriptors to be written into the command buffer,
//!while the implementation is responsible for managing their memory.
//!Push descriptors may enable easier porting from older APIs and in some cases
//!can be more efficient than writing descriptors into descriptor sets.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_push_descriptor]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_push_descriptor
//!   extension>>)
//!# New functions & commands
//! - [`CmdPushDescriptorSetKHR`]
//!If [`VK_KHR_descriptor_update_template`] is supported:
//! - [`CmdPushDescriptorSetWithTemplateKHR`]
//!If [Version 1.1]() is supported:
//! - [`CmdPushDescriptorSetWithTemplateKHR`]
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePushDescriptorPropertiesKHR`]
//!# New constants
//! - [`KHR_PUSH_DESCRIPTOR_EXTENSION_NAME`]
//! - [`KHR_PUSH_DESCRIPTOR_SPEC_VERSION`]
//! - Extending [`DescriptorSetLayoutCreateFlagBits`]:  -
//!   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR`
//!If [`VK_KHR_descriptor_update_template`] is supported:
//! - Extending [`DescriptorUpdateTemplateType`]:  -
//!   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
//!If [Version 1.1]() is supported:
//! - Extending [`DescriptorUpdateTemplateType`]:  -
//!   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
//!# Version History
//! - Revision 1, 2016-10-15 (Jeff Bolz)  - Internal revisions
//! - Revision 2, 2017-09-12 (Tobias Hector)  - Added interactions with Vulkan 1.1
//!# Other info
//! * 2017-09-12
//! * No known IP claims.
//! * - Jeff Bolz, NVIDIA  - Michael Worcester, Imagination Technologies
//!# Related
//! - [`PhysicalDevicePushDescriptorPropertiesKHR`]
//! - [`CmdPushDescriptorSetKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION")]
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME")]
pub const KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_push_descriptor");
///[VkPhysicalDevicePushDescriptorPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html) - Structure describing push descriptor limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDevicePushDescriptorPropertiesKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_push_descriptor
///typedef struct VkPhysicalDevicePushDescriptorPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxPushDescriptors;
///} VkPhysicalDevicePushDescriptorPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_push_descriptors`] is the maximum number of descriptors that  **can**  be used in a
///   descriptor set created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` set.
///# Description
///If the [`PhysicalDevicePushDescriptorPropertiesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR`
///# Related
/// - [`VK_KHR_push_descriptor`]
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
pub struct PhysicalDevicePushDescriptorPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`max_push_descriptors`] is the maximum
    ///number of descriptors that  **can**  be used in a descriptor set created with
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` set.
    max_push_descriptors: u32,
}
impl<'lt> Default for PhysicalDevicePushDescriptorPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            max_push_descriptors: 0,
        }
    }
}
impl<'lt> PhysicalDevicePushDescriptorPropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::max_push_descriptors`]
    pub fn max_push_descriptors(&self) -> u32 {
        self.max_push_descriptors
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
    ///Gets a mutable reference to the value of [`Self::max_push_descriptors`]
    pub fn max_push_descriptors_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::max_push_descriptors`]
    pub fn set_max_push_descriptors(&mut self, value: u32) -> &mut Self {
        self.max_push_descriptors = value;
        self
    }
}
