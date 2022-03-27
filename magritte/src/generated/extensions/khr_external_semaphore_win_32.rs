use crate::{
    native::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Semaphore, StructureType},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_semaphore_win32");
///[VkImportSemaphoreWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreWin32HandleInfoKHR.html) - Structure specifying Windows handle to import to a semaphore
///# C Specifications
///The [`ImportSemaphoreWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_semaphore_win32
///typedef struct VkImportSemaphoreWin32HandleInfoKHR {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    VkSemaphore                              semaphore;
///    VkSemaphoreImportFlags                   flags;
///    VkExternalSemaphoreHandleTypeFlagBits    handleType;
///    HANDLE                                   handle;
///    LPCWSTR                                  name;
///} VkImportSemaphoreWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore`] is the semaphore into which the payload will be imported.
/// - [`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying additional parameters for the
///   semaphore payload import operation.
/// - [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of
///   [`handle`].
/// - [`handle`] is `NULL` or the external handle to import.
/// - [`name`] is `NULL` or a null-terminated UTF-16 string naming the underlying synchronization
///   primitive to import.
///# Description
///The handle types supported by [`handle_type`] are:Valid Usage
/// - [`handle_type`]**must** be a value included in the [Handle Types Supported by [`ImportSemaphoreWin32HandleInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphore-handletypes-win32)
///   table
/// - If [`handle_type`] is not `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`, [`name`]**must** be `NULL`
/// - If [`handle`] is `NULL`, [`name`]**must** name a valid synchronization primitive of the type
///   specified by [`handle_type`]
/// - If [`name`] is `NULL`, [`handle`]**must** be a valid handle of the type specified by
///   [`handle_type`]
/// - If [`handle`] is not `NULL`, [`name`]**must** be `NULL`
/// -    If [`handle`] is not `NULL`, it **must** obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
/// -    If [`name`] is not `NULL`, it **must** obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
/// - If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`, the
///   [`SemaphoreCreateInfo`]::[`flags`] field **must** match that of the semaphore from which
///   [`handle`] or [`name`] was exported
/// - If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`, the
///   [`SemaphoreTypeCreateInfo::semaphore_type`] field **must** match that of the semaphore from
///   which [`handle`] or [`name`] was exported
/// - If [`flags`] contains `VK_SEMAPHORE_IMPORT_TEMPORARY_BIT`, the
///   [`SemaphoreTypeCreateInfo::semaphore_type`] field of the semaphore from which [`handle`] or
///   [`name`] was exported **must** not be `VK_SEMAPHORE_TYPE_TIMELINE`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`semaphore`]**must** be a valid [`Semaphore`] handle
/// - [`flags`]**must** be a valid combination of [`SemaphoreImportFlagBits`] values
///Host Synchronization
/// - Host access to [`semaphore`]**must** be externally synchronized
///# Related
/// - [`VK_KHR_external_semaphore_win32`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`SemaphoreImportFlags`]
/// - [`StructureType`]
/// - [`ImportSemaphoreWin32HandleKHR`]
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
pub struct ImportSemaphoreWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`semaphore`] is the semaphore into which the payload will be
    ///imported.
    semaphore: Semaphore,
    ///[`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying
    ///additional parameters for the semaphore payload import operation.
    flags: SemaphoreImportFlags,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the type of [`handle`].
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
    ///[`handle`] is `NULL` or the external handle to import.
    handle: HANDLE,
    ///[`name`] is `NULL` or a null-terminated UTF-16 string naming the
    ///underlying synchronization primitive to import.
    name: LPCWSTR,
}
impl<'lt> Default for ImportSemaphoreWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: Default::default(),
            name: Default::default(),
        }
    }
}
impl<'lt> ImportSemaphoreWin32HandleInfoKHR<'lt> {
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
    ///Gets the value of [`Self::semaphore`]
    pub fn semaphore(&self) -> Semaphore {
        self.semaphore
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> SemaphoreImportFlags {
        self.flags
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalSemaphoreHandleTypeFlagBits {
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
    ///Gets a mutable reference to the value of [`Self::semaphore`]
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut SemaphoreImportFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlagBits {
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
    ///Sets the raw value of [`Self::semaphore`]
    pub fn set_semaphore(&mut self, value: crate::vulkan1_0::Semaphore) -> &mut Self {
        self.semaphore = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::vulkan1_1::SemaphoreImportFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
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
///[VkExportSemaphoreWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreWin32HandleInfoKHR.html) - Structure specifying additional attributes of Windows handles exported from a semaphore
///# C Specifications
///To specify additional attributes of NT handles exported from a semaphore,
///add a [`ExportSemaphoreWin32HandleInfoKHR`] structure to the [`p_next`]
///chain of the [`SemaphoreCreateInfo`] structure.
///The [`ExportSemaphoreWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_semaphore_win32
///typedef struct VkExportSemaphoreWin32HandleInfoKHR {
///    VkStructureType               sType;
///    const void*                   pNext;
///    const SECURITY_ATTRIBUTES*    pAttributes;
///    DWORD                         dwAccess;
///    LPCWSTR                       name;
///} VkExportSemaphoreWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying security
///   attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
/// - [`name`] is a null-terminated UTF-16 string to associate with the underlying synchronization
///   primitive referenced by NT handles exported from the created semaphore.
///# Description
///If [`ExportSemaphoreCreateInfo`] is not included in the same [`p_next`]
///chain, this structure is ignored.If [`ExportSemaphoreCreateInfo`] is included in the [`p_next`]
/// chain of
///[`SemaphoreCreateInfo`] with a Windows `handleType`, but either
///[`ExportSemaphoreWin32HandleInfoKHR`] is not included in the [`p_next`]
///chain, or if it is but [`attributes`] is set to `NULL`, default security
///descriptor values will be used, and child processes created by the
///application will not inherit the handle, as described in the MSDN
///documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
///Further, if the structure is not present, the access rights used depend on
///the handle type.For handles of the following
/// types:`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT`The implementation **must** ensure the
/// access rights allow both signal and wait
///operations on the semaphore.For handles of the following
/// types:`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`The access rights **must**
/// be:`GENERIC_ALL`
/// * [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
///Valid Usage
/// - If [`ExportSemaphoreCreateInfo::handle_types`] does not include
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`,
///   [`ExportSemaphoreWin32HandleInfoKHR`]**must** not be included in the [`p_next`] chain of
///   [`SemaphoreCreateInfo`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`
/// - If [`attributes`] is not `NULL`, [`attributes`]**must** be a valid pointer to a valid
///   [`SECURITY_ATTRIBUTES`] value
///# Related
/// - [`VK_KHR_external_semaphore_win32`]
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
pub struct ExportSemaphoreWin32HandleInfoKHR<'lt> {
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
    ///underlying synchronization primitive referenced by NT handles exported
    ///from the created semaphore.
    name: LPCWSTR,
}
impl<'lt> Default for ExportSemaphoreWin32HandleInfoKHR<'lt> {
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
impl<'lt> ExportSemaphoreWin32HandleInfoKHR<'lt> {
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
///[VkD3D12FenceSubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkD3D12FenceSubmitInfoKHR.html) - Structure specifying values for Direct3D 12 fence-backed semaphores
///# C Specifications
///To specify the values to use when waiting for and signaling semaphores whose
///[current payload](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) refers to a
///Direct3D 12 fence, add a [`D3D12FenceSubmitInfoKHR`] structure to the
///[`p_next`] chain of the [`SubmitInfo`] structure.
///The [`D3D12FenceSubmitInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_semaphore_win32
///typedef struct VkD3D12FenceSubmitInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           waitSemaphoreValuesCount;
///    const uint64_t*    pWaitSemaphoreValues;
///    uint32_t           signalSemaphoreValuesCount;
///    const uint64_t*    pSignalSemaphoreValues;
///} VkD3D12FenceSubmitInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`wait_semaphore_values_count`] is the number of semaphore wait values specified in
///   [`wait_semaphore_values`].
/// - [`wait_semaphore_values`] is a pointer to an array of [`wait_semaphore_values_count`] values
///   for the corresponding semaphores in [`SubmitInfo::wait_semaphores`] to wait for.
/// - [`signal_semaphore_values_count`] is the number of semaphore signal values specified in
///   [`signal_semaphore_values`].
/// - [`signal_semaphore_values`] is a pointer to an array of [`signal_semaphore_values_count`]
///   values for the corresponding semaphores in [`SubmitInfo::signal_semaphores`] to set when
///   signaled.
///# Description
///If the semaphore in [`SubmitInfo::wait_semaphores`] or
///[`SubmitInfo::signal_semaphores`] corresponding to an entry in
///[`wait_semaphore_values`] or [`signal_semaphore_values`] respectively does
///not currently have a [payload](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-payloads)
///referring to a Direct3D 12 fence, the implementation **must** ignore the value
///in the [`wait_semaphore_values`] or [`signal_semaphore_values`] entry.Valid Usage
/// - [`wait_semaphore_values_count`]**must** be the same value as
///   [`SubmitInfo::wait_semaphore_count`], where [`SubmitInfo`] is in the [`p_next`] chain of this
///   [`D3D12FenceSubmitInfoKHR`] structure
/// - [`signal_semaphore_values_count`]**must** be the same value as
///   [`SubmitInfo::signal_semaphore_count`], where [`SubmitInfo`] is in the [`p_next`] chain of
///   this [`D3D12FenceSubmitInfoKHR`] structure
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR`
/// - If [`wait_semaphore_values_count`] is not `0`, and [`wait_semaphore_values`] is not `NULL`,
///   [`wait_semaphore_values`]**must** be a valid pointer to an array of
///   [`wait_semaphore_values_count`]`uint64_t` values
/// - If [`signal_semaphore_values_count`] is not `0`, and [`signal_semaphore_values`] is not
///   `NULL`, [`signal_semaphore_values`]**must** be a valid pointer to an array of
///   [`signal_semaphore_values_count`]`uint64_t` values
///# Related
/// - [`VK_KHR_external_semaphore_win32`]
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
pub struct D3D12FenceSubmitInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`wait_semaphore_values_count`] is the number of semaphore wait values
    ///specified in [`wait_semaphore_values`].
    wait_semaphore_values_count: u32,
    ///[`wait_semaphore_values`] is a pointer to an array of
    ///[`wait_semaphore_values_count`] values for the corresponding semaphores
    ///in [`SubmitInfo`]::`pWaitSemaphores` to wait for.
    wait_semaphore_values: *const u64,
    ///[`signal_semaphore_values_count`] is the number of semaphore signal
    ///values specified in [`signal_semaphore_values`].
    signal_semaphore_values_count: u32,
    ///[`signal_semaphore_values`] is a pointer to an array of
    ///[`signal_semaphore_values_count`] values for the corresponding semaphores
    ///in [`SubmitInfo`]::`pSignalSemaphores` to set when signaled.
    signal_semaphore_values: *const u64,
}
impl<'lt> Default for D3D12FenceSubmitInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            wait_semaphore_values_count: 0,
            wait_semaphore_values: std::ptr::null(),
            signal_semaphore_values_count: 0,
            signal_semaphore_values: std::ptr::null(),
        }
    }
}
impl<'lt> D3D12FenceSubmitInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::wait_semaphore_values_count`]
    pub fn wait_semaphore_values_count_raw(&self) -> u32 {
        self.wait_semaphore_values_count
    }
    ///Gets the raw value of [`Self::wait_semaphore_values`]
    pub fn wait_semaphore_values_raw(&self) -> *const u64 {
        self.wait_semaphore_values
    }
    ///Gets the raw value of [`Self::signal_semaphore_values_count`]
    pub fn signal_semaphore_values_count_raw(&self) -> u32 {
        self.signal_semaphore_values_count
    }
    ///Gets the raw value of [`Self::signal_semaphore_values`]
    pub fn signal_semaphore_values_raw(&self) -> *const u64 {
        self.signal_semaphore_values
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_values_count`]
    pub fn set_wait_semaphore_values_count_raw(&mut self, value: u32) -> &mut Self {
        self.wait_semaphore_values_count = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_values`]
    pub fn set_wait_semaphore_values_raw(&mut self, value: *const u64) -> &mut Self {
        self.wait_semaphore_values = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_values_count`]
    pub fn set_signal_semaphore_values_count_raw(&mut self, value: u32) -> &mut Self {
        self.signal_semaphore_values_count = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_values`]
    pub fn set_signal_semaphore_values_raw(&mut self, value: *const u64) -> &mut Self {
        self.signal_semaphore_values = value;
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
    ///Gets the value of [`Self::wait_semaphore_values_count`]
    pub fn wait_semaphore_values_count(&self) -> u32 {
        self.wait_semaphore_values_count
    }
    ///Gets the value of [`Self::wait_semaphore_values`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn wait_semaphore_values(&self) -> &[u64] {
        std::slice::from_raw_parts(self.wait_semaphore_values, self.wait_semaphore_values_count as usize)
    }
    ///Gets the value of [`Self::signal_semaphore_values_count`]
    pub fn signal_semaphore_values_count(&self) -> u32 {
        self.signal_semaphore_values_count
    }
    ///Gets the value of [`Self::signal_semaphore_values`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn signal_semaphore_values(&self) -> &[u64] {
        std::slice::from_raw_parts(
            self.signal_semaphore_values,
            self.signal_semaphore_values_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::wait_semaphore_values_count`]
    pub fn wait_semaphore_values_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::signal_semaphore_values_count`]
    pub fn signal_semaphore_values_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::wait_semaphore_values_count`]
    pub fn set_wait_semaphore_values_count(&mut self, value: u32) -> &mut Self {
        self.wait_semaphore_values_count = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_values`]
    pub fn set_wait_semaphore_values(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.wait_semaphore_values = value.as_ptr();
        self.wait_semaphore_values_count = len_;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_values_count`]
    pub fn set_signal_semaphore_values_count(&mut self, value: u32) -> &mut Self {
        self.signal_semaphore_values_count = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_values`]
    pub fn set_signal_semaphore_values(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.signal_semaphore_values = value.as_ptr();
        self.signal_semaphore_values_count = len_;
        self
    }
}
///[VkSemaphoreGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetWin32HandleInfoKHR.html) - Structure describing a Win32 handle semaphore export operation
///# C Specifications
///The [`SemaphoreGetWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_semaphore_win32
///typedef struct VkSemaphoreGetWin32HandleInfoKHR {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    VkSemaphore                              semaphore;
///    VkExternalSemaphoreHandleTypeFlagBits    handleType;
///} VkSemaphoreGetWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore`] is the semaphore from which state will be exported.
/// - [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of
///   handle requested.
///# Description
///The properties of the handle returned depend on the value of
///[`handle_type`].
///See [`ExternalSemaphoreHandleTypeFlagBits`] for a description of the
///properties of the defined external semaphore handle types.Valid Usage
/// - [`handle_type`]**must** have been included in [`ExportSemaphoreCreateInfo::handle_types`] when
///   the [`semaphore`]’s current payload was created
/// - If [`handle_type`] is defined as an NT handle, [`GetSemaphoreWin32HandleKHR`]**must** be
///   called no more than once for each valid unique combination of [`semaphore`] and
///   [`handle_type`]
/// -  [`semaphore`]**must** not currently have its payload replaced by an imported payload as described below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) unless that imported payload’s handle type was included in [`ExternalSemaphoreProperties::export_from_imported_handle_types`] for [`handle_type`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, as defined below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), there **must** be no queue waiting on [`semaphore`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`semaphore`]**must** be signaled, or have an associated [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) pending execution
/// - [`handle_type`]**must** be defined as an NT handle or a global share handle
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`semaphore`]**must** be a valid [`Semaphore`] handle
/// - [`handle_type`]**must** be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_semaphore_win32`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`GetSemaphoreWin32HandleKHR`]
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
pub struct SemaphoreGetWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`semaphore`] is the semaphore from which state will be exported.
    semaphore: Semaphore,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl<'lt> Default for SemaphoreGetWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> SemaphoreGetWin32HandleInfoKHR<'lt> {
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
    ///Gets the value of [`Self::semaphore`]
    pub fn semaphore(&self) -> Semaphore {
        self.semaphore
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalSemaphoreHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::semaphore`]
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlagBits {
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
    ///Sets the raw value of [`Self::semaphore`]
    pub fn set_semaphore(&mut self, value: crate::vulkan1_0::Semaphore) -> &mut Self {
        self.semaphore = value;
        self
    }
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
}
