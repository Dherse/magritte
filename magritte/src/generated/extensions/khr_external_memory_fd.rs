use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, DeviceMemory, StructureType},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_memory_fd");
///[VkImportMemoryFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryFdInfoKHR.html) - Import memory created on the same physical device from a file descriptor
///# C Specifications
///To import memory from a POSIX file descriptor handle, add a
///[`ImportMemoryFdInfoKHR`] structure to the [`p_next`] chain of the
///[`MemoryAllocateInfo`] structure.
///The [`ImportMemoryFdInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_memory_fd
///typedef struct VkImportMemoryFdInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///    int                                   fd;
///} VkImportMemoryFdInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the handle type of
///   [`fd`].
/// - [`fd`] is the external handle to import.
///# Description
///Importing memory from a file descriptor transfers ownership of the file
///descriptor from the application to the Vulkan implementation.
///The application **must** not perform any operations on the file descriptor
///after a successful import.
///The imported memory object holds a reference to its payload.Applications **can** import the same
/// payload into multiple instances of Vulkan,
///into the same instance from which it was exported, and multiple times into a
///given Vulkan instance.
///In all cases, each import operation **must** create a distinct
///[`DeviceMemory`] object.Valid Usage
/// - If [`handle_type`] is not `0`, it **must** be supported for import, as reported by
///   [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
/// - The memory from which [`fd`] was exported **must** have been created on the same underlying
///   physical device as `device`
/// - If [`handle_type`] is not `0`, it **must** be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`
///   or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT`
/// - If [`handle_type`] is not `0`, [`fd`]**must** be a valid handle of the type specified by
///   [`handle_type`]
/// -    The memory represented by [`fd`]**must** have been created from a physical device and driver that is compatible with `device` and [`handle_type`], as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
/// -  [`fd`]**must** obey any requirements listed for [`handle_type`] in [external memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR`
/// - If [`handle_type`] is not `0`, [`handle_type`]**must** be a valid
///   [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_memory_fd`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
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
pub struct ImportMemoryFdInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the handle type of [`fd`].
    handle_type: ExternalMemoryHandleTypeFlagBits,
    ///[`fd`] is the external handle to import.
    fd: i32,
}
///[VkMemoryFdPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryFdPropertiesKHR.html) - Properties of External Memory File Descriptors
///# C Specifications
///The [`MemoryFdPropertiesKHR`] structure returned is defined as:
///```c
///// Provided by VK_KHR_external_memory_fd
///typedef struct VkMemoryFdPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           memoryTypeBits;
///} VkMemoryFdPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the
///   specified file descriptor **can** be imported as.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_external_memory_fd`]
/// - [`StructureType`]
/// - [`GetMemoryFdPropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MemoryFdPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the specified file descriptor **can** be imported as.
    memory_type_bits: u32,
}
///[VkMemoryGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetFdInfoKHR.html) - Structure describing a POSIX FD semaphore export operation
///# C Specifications
///The [`MemoryGetFdInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_memory_fd
///typedef struct VkMemoryGetFdInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkDeviceMemory                        memory;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///} VkMemoryGetFdInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory`] is the memory object from which the handle will be exported.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of handle
///   requested.
///# Description
///The properties of the file descriptor exported depend on the value of
///[`handle_type`].
///See [`ExternalMemoryHandleTypeFlagBits`] for a description of the
///properties of the defined external memory handle types.Valid Usage
/// - [`handle_type`]**must** have been included in [`ExportMemoryAllocateInfo::handle_types`] when
///   [`memory`] was created
/// - [`handle_type`]**must** be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT` or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
/// - [`handle_type`]**must** be a valid [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_memory_fd`]
/// - [`DeviceMemory`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`GetMemoryFdKHR`]
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
pub struct MemoryGetFdInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`memory`] is the memory object from which the handle will be
    ///exported.
    memory: DeviceMemory,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
