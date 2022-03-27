use crate::{
    extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_external_memory");
///[VkExternalMemoryImageCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfoNV.html) - Specify that an image may be backed by external memory
///# C Specifications
///If the [`p_next`] chain includes a [`ExternalMemoryImageCreateInfoNV`]
///structure, then that structure defines a set of external memory handle types
///that **may** be used as backing store for the image.The [`ExternalMemoryImageCreateInfoNV`]
/// structure is defined as:
///```c
///// Provided by VK_NV_external_memory
///typedef struct VkExternalMemoryImageCreateInfoNV {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkExternalMemoryHandleTypeFlagsNV    handleTypes;
///} VkExternalMemoryImageCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_types`] is zero, or a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] specifying
///   one or more external memory handle types.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV`
/// - [`handle_types`]**must** be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`]
///   values
///# Related
/// - [`VK_NV_external_memory`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`handle_types`] is zero, or a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more
    ///external memory handle types.
    handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl<'lt> Default for ExternalMemoryImageCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl<'lt> ExternalMemoryImageCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::handle_types`]
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.handle_types
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_types`]
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagsNV {
        &mut self.handle_types
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::handle_types`]
    pub fn set_handle_types(
        &mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> &mut Self {
        self.handle_types = value;
        self
    }
}
///[VkExportMemoryAllocateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfoNV.html) - Specify memory handle types that may be exported
///# C Specifications
///The [`ExportMemoryAllocateInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_external_memory
///typedef struct VkExportMemoryAllocateInfoNV {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkExternalMemoryHandleTypeFlagsNV    handleTypes;
///} VkExportMemoryAllocateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more
///   memory handle types that **may** be exported. Multiple handle types **may** be requested for
///   the same allocation as long as they are compatible, as reported by
///   [`GetPhysicalDeviceExternalImageFormatPropertiesNV`].
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV`
/// - [`handle_types`]**must** be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`]
///   values
///# Related
/// - [`VK_NV_external_memory`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExportMemoryAllocateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more memory
    ///handle types that **may** be exported.
    ///Multiple handle types **may** be requested for the same allocation as long
    ///as they are compatible, as reported by
    ///[`GetPhysicalDeviceExternalImageFormatPropertiesNV`].
    handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl<'lt> Default for ExportMemoryAllocateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl<'lt> ExportMemoryAllocateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::handle_types`]
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.handle_types
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_types`]
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagsNV {
        &mut self.handle_types
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::handle_types`]
    pub fn set_handle_types(
        &mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> &mut Self {
        self.handle_types = value;
        self
    }
}
