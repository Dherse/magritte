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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_types`] is zero, or a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more
    ///external memory handle types.
    handle_types: ExternalMemoryHandleTypeFlagsNV,
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExportMemoryAllocateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more memory
    ///handle types that **may** be exported.
    ///Multiple handle types **may** be requested for the same allocation as long
    ///as they are compatible, as reported by
    ///[`GetPhysicalDeviceExternalImageFormatPropertiesNV`].
    handle_types: ExternalMemoryHandleTypeFlagsNV,
}
