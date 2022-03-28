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
///For handle types defined as NT handles, the application  **must**  release handle
///ownership using the `CloseHandle` system call when the handle is no
///longer needed.
///For handle types defined as NT handles, the imported memory object holds a
///reference to its payload.Applications  **can**  import the same payload into multiple instances
/// of Vulkan,
///into the same instance from which it was exported, and multiple times into a
///given Vulkan instance.
///In all cases, each import operation  **must**  create a distinct
///[`DeviceMemory`] object.
///## Valid Usage
/// - If [`handle_type`] is not `0`, it  **must**  be supported for import, as reported by
///   [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
/// - The memory from which [`handle`] was exported, or the memory named by [`name`] **must**  have
///   been created on the same underlying physical device as `device`
/// - If [`handle_type`] is not `0`, it  **must**  be defined as an NT handle or a global share
///   handle
/// - If [`handle_type`] is not `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`, [`name`] **must**  be `NULL`
/// - If [`handle_type`] is not `0` and [`handle`] is `NULL`, [`name`] **must**  name a valid memory
///   resource of the type specified by [`handle_type`]
/// - If [`handle_type`] is not `0` and [`name`] is `NULL`, [`handle`] **must**  be a valid handle
///   of the type specified by [`handle_type`]
/// - if [`handle`] is not `NULL`, [`name`] **must**  be `NULL`
/// - If [`handle`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in
///   [external memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
/// -    If [`name`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in [external memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
/// - If [`handle_type`] is not `0`, [`handle_type`] **must**  be a valid
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
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the type of [`handle`] or [`name`].
    handle_type: ExternalMemoryHandleTypeFlagBits,
    ///[`handle`] is `NULL` or the external handle to import.
    handle: HANDLE,
    ///[`name`] is `NULL` or a null-terminated UTF-16 string naming the
    ///payload to import.
    name: LPCWSTR,
}
impl<'lt> Default for ImportMemoryWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: Default::default(),
            name: Default::default(),
        }
    }
}
impl<'lt> ImportMemoryWin32HandleInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::handle`]
    pub fn handle_raw(&self) -> &HANDLE {
        &self.handle
    }
    ///Gets the raw value of [`Self::name`]
    pub fn name_raw(&self) -> &LPCWSTR {
        &self.name
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::handle`]
    pub fn set_handle_raw(&mut self, value: HANDLE) -> &mut Self {
        self.handle = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name_raw(&mut self, value: LPCWSTR) -> &mut Self {
        self.name = value;
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
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets the value of [`Self::handle`]
    pub fn handle(&self) -> &HANDLE {
        &self.handle
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &LPCWSTR {
        &self.name
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::handle`]
    pub fn handle_mut(&mut self) -> &mut HANDLE {
        &mut self.handle
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut LPCWSTR {
        &mut self.name
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
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
    ///Sets the raw value of [`Self::handle`]
    pub fn set_handle(&mut self, value: crate::native::HANDLE) -> &mut Self {
        self.handle = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: crate::native::LPCWSTR) -> &mut Self {
        self.name = value;
        self
    }
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
/// - [`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying security
///   attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
/// - [`name`] is a null-terminated UTF-16 string to associate with the payload referenced by NT
///   handles exported from the created memory.
///# Description
///If [`ExportMemoryAllocateInfo`] is not included in the same [`p_next`]
///chain, this structure is ignored.If [`ExportMemoryAllocateInfo`] is included in the [`p_next`]
/// chain of
///[`MemoryAllocateInfo`] with a Windows `handleType`, but either
///[`ExportMemoryWin32HandleInfoKHR`] is not included in the [`p_next`]
///chain, or if it is but [`attributes`] is set to `NULL`, default security
///descriptor values will be used, and child processes created by the
///application will not inherit the handle, as described in the MSDN
///documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
///Further, if the structure is not present, the access rights used depend on
///the handle type.For handles of the following types:
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`
///The implementation  **must**  ensure the access rights allow read and write
///access to the memory.
/// * [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
///
///## Valid Usage
/// - If [`ExportMemoryAllocateInfo::handle_types`] does not include
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`, a [`ExportMemoryWin32HandleInfoKHR`]
///   structure  **must**  not be included in the [`p_next`] chain of [`MemoryAllocateInfo`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR`
/// - If [`attributes`] is not `NULL`, [`attributes`] **must**  be a valid pointer to a valid
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
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`]
    ///structure specifying security attributes of the handle.
    attributes: *const SECURITY_ATTRIBUTES,
    ///[`dw_access`] is a [`DWORD`] specifying access rights of the handle.
    dw_access: DWORD,
    ///[`name`] is a null-terminated UTF-16 string to associate with the
    ///payload referenced by NT handles exported from the created memory.
    name: LPCWSTR,
}
impl<'lt> Default for ExportMemoryWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: Default::default(),
        }
    }
}
impl<'lt> ExportMemoryWin32HandleInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::dw_access`]
    pub fn dw_access_raw(&self) -> &DWORD {
        &self.dw_access
    }
    ///Gets the raw value of [`Self::name`]
    pub fn name_raw(&self) -> &LPCWSTR {
        &self.name
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::dw_access`]
    pub fn set_dw_access_raw(&mut self, value: DWORD) -> &mut Self {
        self.dw_access = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name_raw(&mut self, value: LPCWSTR) -> &mut Self {
        self.name = value;
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
    ///Gets the value of [`Self::attributes`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn attributes(&self) -> &SECURITY_ATTRIBUTES {
        &*self.attributes
    }
    ///Gets the value of [`Self::dw_access`]
    pub fn dw_access(&self) -> &DWORD {
        &self.dw_access
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &LPCWSTR {
        &self.name
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::dw_access`]
    pub fn dw_access_mut(&mut self) -> &mut DWORD {
        &mut self.dw_access
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut LPCWSTR {
        &mut self.name
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
    ///Sets the raw value of [`Self::attributes`]
    pub fn set_attributes(&mut self, value: &'lt crate::native::SECURITY_ATTRIBUTES) -> &mut Self {
        self.attributes = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::dw_access`]
    pub fn set_dw_access(&mut self, value: crate::native::DWORD) -> &mut Self {
        self.dw_access = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: crate::native::LPCWSTR) -> &mut Self {
        self.name = value;
        self
    }
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
///   specified windows handle  **can**  be imported as.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR`
/// - [`p_next`] **must**  be `NULL`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryWin32HandlePropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the specified windows handle  **can**  be imported as.
    memory_type_bits: u32,
}
impl<'lt> Default for MemoryWin32HandlePropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            memory_type_bits: 0,
        }
    }
}
impl<'lt> MemoryWin32HandlePropertiesKHR<'lt> {
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
    ///Gets the value of [`Self::memory_type_bits`]
    pub fn memory_type_bits(&self) -> u32 {
        self.memory_type_bits
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
    ///Gets a mutable reference to the value of [`Self::memory_type_bits`]
    pub fn memory_type_bits_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::memory_type_bits`]
    pub fn set_memory_type_bits(&mut self, value: u32) -> &mut Self {
        self.memory_type_bits = value;
        self
    }
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
///properties of the defined external memory handle types.
///## Valid Usage
/// - [`handle_type`] **must**  have been included in [`ExportMemoryAllocateInfo::handle_types`]
///   when [`memory`] was created
/// - If [`handle_type`] is defined as an NT handle, [`GetMemoryWin32HandleKHR`] **must**  be called
///   no more than once for each valid unique combination of [`memory`] and [`handle_type`]
/// - [`handle_type`] **must**  be defined as an NT handle or a global share handle
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryGetWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`memory`] is the memory object from which the handle will be
    ///exported.
    memory: DeviceMemory,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl<'lt> Default for MemoryGetWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> MemoryGetWin32HandleInfoKHR<'lt> {
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
    ///Gets the value of [`Self::memory`]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::memory`]
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
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
    ///Sets the raw value of [`Self::memory`]
    pub fn set_memory(&mut self, value: crate::vulkan1_0::DeviceMemory) -> &mut Self {
        self.memory = value;
        self
    }
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
}
