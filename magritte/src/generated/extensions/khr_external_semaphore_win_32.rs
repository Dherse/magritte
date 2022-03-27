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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImportSemaphoreWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
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
/// - [`p_attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying
///   security attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
/// - [`name`] is a null-terminated UTF-16 string to associate with the underlying synchronization
///   primitive referenced by NT handles exported from the created semaphore.
///# Description
///If [`ExportSemaphoreCreateInfo`] is not included in the same [`p_next`]
///chain, this structure is ignored.If [`ExportSemaphoreCreateInfo`] is included in the [`p_next`]
/// chain of
///[`SemaphoreCreateInfo`] with a Windows `handleType`, but either
///[`ExportSemaphoreWin32HandleInfoKHR`] is not included in the [`p_next`]
///chain, or if it is but [`p_attributes`] is set to `NULL`, default security
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
/// - If [`p_attributes`] is not `NULL`, [`p_attributes`]**must** be a valid pointer to a valid
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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKHR<'lt> {
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
    ///underlying synchronization primitive referenced by NT handles exported
    ///from the created semaphore.
    name: LPCWSTR,
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
///   [`p_wait_semaphore_values`].
/// - [`p_wait_semaphore_values`] is a pointer to an array of [`wait_semaphore_values_count`] values
///   for the corresponding semaphores in [`SubmitInfo::p_wait_semaphores`] to wait for.
/// - [`signal_semaphore_values_count`] is the number of semaphore signal values specified in
///   [`p_signal_semaphore_values`].
/// - [`p_signal_semaphore_values`] is a pointer to an array of [`signal_semaphore_values_count`]
///   values for the corresponding semaphores in [`SubmitInfo::p_signal_semaphores`] to set when
///   signaled.
///# Description
///If the semaphore in [`SubmitInfo::p_wait_semaphores`] or
///[`SubmitInfo::p_signal_semaphores`] corresponding to an entry in
///[`p_wait_semaphore_values`] or [`p_signal_semaphore_values`] respectively does
///not currently have a [payload](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-payloads)
///referring to a Direct3D 12 fence, the implementation **must** ignore the value
///in the [`p_wait_semaphore_values`] or [`p_signal_semaphore_values`] entry.Valid Usage
/// - [`wait_semaphore_values_count`]**must** be the same value as
///   [`SubmitInfo::wait_semaphore_count`], where [`SubmitInfo`] is in the [`p_next`] chain of this
///   [`D3D12FenceSubmitInfoKHR`] structure
/// - [`signal_semaphore_values_count`]**must** be the same value as
///   [`SubmitInfo::signal_semaphore_count`], where [`SubmitInfo`] is in the [`p_next`] chain of
///   this [`D3D12FenceSubmitInfoKHR`] structure
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR`
/// - If [`wait_semaphore_values_count`] is not `0`, and [`p_wait_semaphore_values`] is not `NULL`,
///   [`p_wait_semaphore_values`]**must** be a valid pointer to an array of
///   [`wait_semaphore_values_count`]`uint64_t` values
/// - If [`signal_semaphore_values_count`] is not `0`, and [`p_signal_semaphore_values`] is not
///   `NULL`, [`p_signal_semaphore_values`]**must** be a valid pointer to an array of
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct D3D12FenceSubmitInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`wait_semaphore_values_count`] is the number of semaphore wait values
    ///specified in [`p_wait_semaphore_values`].
    wait_semaphore_values_count: u32,
    ///[`p_wait_semaphore_values`] is a pointer to an array of
    ///[`wait_semaphore_values_count`] values for the corresponding semaphores
    ///in [`SubmitInfo`]::`pWaitSemaphores` to wait for.
    p_wait_semaphore_values: *mut u64,
    ///[`signal_semaphore_values_count`] is the number of semaphore signal
    ///values specified in [`p_signal_semaphore_values`].
    signal_semaphore_values_count: u32,
    ///[`p_signal_semaphore_values`] is a pointer to an array of
    ///[`signal_semaphore_values_count`] values for the corresponding semaphores
    ///in [`SubmitInfo`]::`pSignalSemaphores` to set when signaled.
    p_signal_semaphore_values: *mut u64,
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`semaphore`] is the semaphore from which state will be exported.
    semaphore: Semaphore,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
