use crate::{
    native::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, BaseOutStructure, DeviceMemory, StructureType},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_memory_win32");
///[VkImportMemoryWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoKHR.html) - Import Win32 memory created on the same physical device
///# C Specifications
///To import memory from a Windows handle, add a
///[`ImportMemoryWin32HandleInfoKHR`] structure to the [`p_next`] chain of
///the [`MemoryAllocateInfo`] structure.The [`ImportMemoryWin32HandleInfoKHR`] structure is defined
/// as:
///```c
///// Provided by VK_KHR_external_memory_win32
///typedef struct VkImportMemoryWin32HandleInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///    HANDLE                                handle;
///    LPCWSTR                               name;
///} VkImportMemoryWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of
///   [`handle`] or [`name`].
/// - [`handle`] is `NULL` or the external handle to import.
/// - [`name`] is `NULL` or a null-terminated UTF-16 string naming the payload to import.
///# Description
///Importing memory object payloads from Windows handles does not transfer
///ownership of the handle to the Vulkan implementation.
///For handle types defined as NT handles, the application **must** release handle
///ownership using the `CloseHandle` system call when the handle is no
///longer needed.
///For handle types defined as NT handles, the imported memory object holds a
///reference to its payload.Applications **can** import the same payload into multiple instances of
/// Vulkan,
///into the same instance from which it was exported, and multiple times into a
///given Vulkan instance.
///In all cases, each import operation **must** create a distinct
///[`DeviceMemory`] object.Valid Usage
/// - If [`handle_type`] is not `0`, it **must** be supported for import, as reported by
///   [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
/// - The memory from which [`handle`] was exported, or the memory named by [`name`]**must** have
///   been created on the same underlying physical device as `device`
/// - If [`handle_type`] is not `0`, it **must** be defined as an NT handle or a global share handle
/// - If [`handle_type`] is not `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`, [`name`]**must** be `NULL`
/// - If [`handle_type`] is not `0` and [`handle`] is `NULL`, [`name`]**must** name a valid memory
///   resource of the type specified by [`handle_type`]
/// - If [`handle_type`] is not `0` and [`name`] is `NULL`, [`handle`]**must** be a valid handle of
///   the type specified by [`handle_type`]
/// - if [`handle`] is not `NULL`, [`name`]**must** be `NULL`
/// -    If [`handle`] is not `NULL`, it **must** obey any requirements listed for [`handle_type`] in [external memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
/// -    If [`name`] is not `NULL`, it **must** obey any requirements listed for [`handle_type`] in [external memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
/// - If [`handle_type`] is not `0`, [`handle_type`]**must** be a valid
///   [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_memory_win32`]
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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the type of [`handle`] or [`name`].
    handle_type: ExternalMemoryHandleTypeFlagBits,
    ///[`handle`] is `NULL` or the external handle to import.
    handle: HANDLE,
    ///[`name`] is `NULL` or a null-terminated UTF-16 string naming the
    ///payload to import.
    name: LPCWSTR,
}
///[VkExportMemoryWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoKHR.html) - Structure specifying additional attributes of Windows handles exported from a memory
///# C Specifications
///To specify additional attributes of NT handles exported from a memory
///object, add a [`ExportMemoryWin32HandleInfoKHR`] structure to the
///[`p_next`] chain of the [`MemoryAllocateInfo`] structure.
///The [`ExportMemoryWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_memory_win32
///typedef struct VkExportMemoryWin32HandleInfoKHR {
///    VkStructureType               sType;
///    const void*                   pNext;
///    const SECURITY_ATTRIBUTES*    pAttributes;
///    DWORD                         dwAccess;
///    LPCWSTR                       name;
///} VkExportMemoryWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying
///   security attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
/// - [`name`] is a null-terminated UTF-16 string to associate with the payload referenced by NT
///   handles exported from the created memory.
///# Description
///If [`ExportMemoryAllocateInfo`] is not included in the same [`p_next`]
///chain, this structure is ignored.If [`ExportMemoryAllocateInfo`] is included in the [`p_next`]
/// chain of
///[`MemoryAllocateInfo`] with a Windows `handleType`, but either
///[`ExportMemoryWin32HandleInfoKHR`] is not included in the [`p_next`]
///chain, or if it is but [`p_attributes`] is set to `NULL`, default security
///descriptor values will be used, and child processes created by the
///application will not inherit the handle, as described in the MSDN
///documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
///Further, if the structure is not present, the access rights used depend on
///the handle type.For handles of the following types:
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`
///The implementation **must** ensure the access rights allow read and write
///access to the memory.
/// * [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
///Valid Usage
/// - If [`ExportMemoryAllocateInfo::handle_types`] does not include
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`, a [`ExportMemoryWin32HandleInfoKHR`]
///   structure **must** not be included in the [`p_next`] chain of [`MemoryAllocateInfo`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
/// - If [`p_attributes`] is not `NULL`, [`p_attributes`]**must** be a valid pointer to a valid
///   [`SECURITY_ATTRIBUTES`] value
///# Related
/// - [`VK_KHR_external_memory_win32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`p_attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`]
    ///structure specifying security attributes of the handle.
    p_attributes: *mut SECURITY_ATTRIBUTES,
    ///[`dw_access`] is a [`DWORD`] specifying access rights of the handle.
    dw_access: DWORD,
    ///[`name`] is a null-terminated UTF-16 string to associate with the
    ///payload referenced by NT handles exported from the created memory.
    name: LPCWSTR,
}
///[VkMemoryWin32HandlePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html) - Properties of External Memory Windows Handles
///# C Specifications
///The [`MemoryWin32HandlePropertiesKHR`] structure returned is defined as:
///```c
///// Provided by VK_KHR_external_memory_win32
///typedef struct VkMemoryWin32HandlePropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           memoryTypeBits;
///} VkMemoryWin32HandlePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the
///   specified windows handle **can** be imported as.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_external_memory_win32`]
/// - [`StructureType`]
/// - [`GetMemoryWin32HandlePropertiesKHR`]
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
pub struct MemoryWin32HandlePropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the specified windows handle **can** be imported as.
    memory_type_bits: u32,
}
///[VkMemoryGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetWin32HandleInfoKHR.html) - Structure describing a Win32 handle semaphore export operation
///# C Specifications
///The [`MemoryGetWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_memory_win32
///typedef struct VkMemoryGetWin32HandleInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkDeviceMemory                        memory;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///} VkMemoryGetWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory`] is the memory object from which the handle will be exported.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of handle
///   requested.
///# Description
///The properties of the handle returned depend on the value of
///[`handle_type`].
///See [`ExternalMemoryHandleTypeFlagBits`] for a description of the
///properties of the defined external memory handle types.Valid Usage
/// - [`handle_type`]**must** have been included in [`ExportMemoryAllocateInfo::handle_types`] when
///   [`memory`] was created
/// - If [`handle_type`] is defined as an NT handle, [`GetMemoryWin32HandleKHR`]**must** be called
///   no more than once for each valid unique combination of [`memory`] and [`handle_type`]
/// - [`handle_type`]**must** be defined as an NT handle or a global share handle
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
/// - [`handle_type`]**must** be a valid [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_memory_win32`]
/// - [`DeviceMemory`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`GetMemoryWin32HandleKHR`]
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
pub struct MemoryGetWin32HandleInfoKHR<'lt> {
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
