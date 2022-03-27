use crate::{
    native::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Fence, StructureType},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_fence_win32");
///[VkImportFenceWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportFenceWin32HandleInfoKHR.html) - (None)
///# C Specifications
///The [`ImportFenceWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_fence_win32
///typedef struct VkImportFenceWin32HandleInfoKHR {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkFence                              fence;
///    VkFenceImportFlags                   flags;
///    VkExternalFenceHandleTypeFlagBits    handleType;
///    HANDLE                               handle;
///    LPCWSTR                              name;
///} VkImportFenceWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fence`] is the fence into which the state will be imported.
/// - [`flags`] is a bitmask of [`FenceImportFlagBits`] specifying additional parameters for the
///   fence payload import operation.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of
///   [`handle`].
/// - [`handle`] is `NULL` or the external handle to import.
/// - [`name`] is `NULL` or a null-terminated UTF-16 string naming the underlying synchronization
///   primitive to import.
///# Description
///The handle types supported by [`handle_type`] are:Valid Usage
/// - [`handle_type`]**must** be a value included in the [Handle Types Supported by [`ImportFenceWin32HandleInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fence-handletypes-win32)
///   table
/// - If [`handle_type`] is not `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`, [`name`]**must**
///   be `NULL`
/// - If [`handle`] is `NULL`, [`name`]**must** name a valid synchronization primitive of the type
///   specified by [`handle_type`]
/// - If [`name`] is `NULL`, [`handle`]**must** be a valid handle of the type specified by
///   [`handle_type`]
/// - If [`handle`] is not `NULL`, [`name`]**must** be `NULL`
/// -    If [`handle`] is not `NULL`, it **must** obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
/// -    If [`name`] is not `NULL`, it **must** obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`fence`]**must** be a valid [`Fence`] handle
/// - [`flags`]**must** be a valid combination of [`FenceImportFlagBits`] values
///Host Synchronization
/// - Host access to [`fence`]**must** be externally synchronized
///# Related
/// - [`VK_KHR_external_fence_win32`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`FenceImportFlags`]
/// - [`StructureType`]
/// - [`ImportFenceWin32HandleKHR`]
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
pub struct ImportFenceWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`fence`] is the fence into which the state will be imported.
    fence: Fence,
    ///[`flags`] is a bitmask of [`FenceImportFlagBits`] specifying
    ///additional parameters for the fence payload import operation.
    flags: FenceImportFlags,
    ///[`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value
    ///specifying the type of [`handle`].
    handle_type: ExternalFenceHandleTypeFlagBits,
    ///[`handle`] is `NULL` or the external handle to import.
    handle: HANDLE,
    ///[`name`] is `NULL` or a null-terminated UTF-16 string naming the
    ///underlying synchronization primitive to import.
    name: LPCWSTR,
}
///[VkExportFenceWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html) - Structure specifying additional attributes of Windows handles exported from a fence
///# C Specifications
///To specify additional attributes of NT handles exported from a fence, add a
///[`ExportFenceWin32HandleInfoKHR`] structure to the [`p_next`] chain of
///the [`FenceCreateInfo`] structure.
///The [`ExportFenceWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_fence_win32
///typedef struct VkExportFenceWin32HandleInfoKHR {
///    VkStructureType               sType;
///    const void*                   pNext;
///    const SECURITY_ATTRIBUTES*    pAttributes;
///    DWORD                         dwAccess;
///    LPCWSTR                       name;
///} VkExportFenceWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`p_attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying
///   security attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
/// - [`name`] is a null-terminated UTF-16 string to associate with the underlying synchronization
///   primitive referenced by NT handles exported from the created fence.
///# Description
///If [`ExportFenceCreateInfo`] is not inluded in the same [`p_next`]
///chain, this structure is ignored.If [`ExportFenceCreateInfo`] is included in the [`p_next`]
/// chain of
///[`FenceCreateInfo`] with a Windows `handleType`, but either
///[`ExportFenceWin32HandleInfoKHR`] is not included in the [`p_next`]
///chain, or if it is but [`p_attributes`] is set to `NULL`, default security
///descriptor values will be used, and child processes created by the
///application will not inherit the handle, as described in the MSDN
///documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
///Further, if the structure is not present, the access rights will be`DXGI_SHARED_RESOURCE_READ` |
/// `DXGI_SHARED_RESOURCE_WRITE`for handles of the following
/// types:`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`
/// * [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
///Valid Usage
/// - If [`ExportFenceCreateInfo::handle_types`] does not include
///   `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`, a [`ExportFenceWin32HandleInfoKHR`]
///   structure **must** not be included in the [`p_next`] chain of [`FenceCreateInfo`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR`
/// - If [`p_attributes`] is not `NULL`, [`p_attributes`]**must** be a valid pointer to a valid
///   [`SECURITY_ATTRIBUTES`] value
///# Related
/// - [`VK_KHR_external_fence_win32`]
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
pub struct ExportFenceWin32HandleInfoKHR<'lt> {
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
    ///from the created fence.
    name: LPCWSTR,
}
///[VkFenceGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html) - Structure describing a Win32 handle fence export operation
///# C Specifications
///The [`FenceGetWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_fence_win32
///typedef struct VkFenceGetWin32HandleInfoKHR {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkFence                              fence;
///    VkExternalFenceHandleTypeFlagBits    handleType;
///} VkFenceGetWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fence`] is the fence from which state will be exported.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of handle
///   requested.
///# Description
///The properties of the handle returned depend on the value of
///[`handle_type`].
///See [`ExternalFenceHandleTypeFlagBits`] for a description of the
///properties of the defined external fence handle types.Valid Usage
/// - [`handle_type`]**must** have been included in [`ExportFenceCreateInfo::handle_types`] when the
///   [`fence`]’s current payload was created
/// - If [`handle_type`] is defined as an NT handle, [`GetFenceWin32HandleKHR`]**must** be called no
///   more than once for each valid unique combination of [`fence`] and [`handle_type`]
/// -  [`fence`]**must** not currently have its payload replaced by an imported payload as described below in [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing) unless that imported payload’s handle type was included in [`ExternalFenceProperties::export_from_imported_handle_types`] for [`handle_type`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`fence`]**must** be signaled, or have an associated [fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling) pending execution
/// - [`handle_type`]**must** be defined as an NT handle or a global share handle
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`fence`]**must** be a valid [`Fence`] handle
/// - [`handle_type`]**must** be a valid [`ExternalFenceHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_fence_win32`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`StructureType`]
/// - [`GetFenceWin32HandleKHR`]
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
pub struct FenceGetWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`fence`] is the fence from which state will be exported.
    fence: Fence,
    ///[`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    handle_type: ExternalFenceHandleTypeFlagBits,
}
