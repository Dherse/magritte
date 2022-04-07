//![VK_KHR_external_fence_win32](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_win32.html) - device extension
//!# Description
//!An application using external memory may wish to synchronize access to that
//!memory using fences.
//!This extension enables an application to export fence payload to and import
//!fence payload from Windows handles.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_fence`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_fence_win32]
//!   @critsec%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_fence_win32 extension>>)
//!# New functions & commands
//! - [`get_fence_win32_handle_khr`]
//! - [`import_fence_win32_handle_khr`]
//!# New structures
//! - [`FenceGetWin32HandleInfoKHR`]
//! - [`ImportFenceWin32HandleInfoKHR`]
//! - Extending [`FenceCreateInfo`]:  - [`ExportFenceWin32HandleInfoKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR`
//!# Known issues & F.A.Q
//!This extension borrows concepts, semantics, and language from
//!`[`VK_KHR_external_semaphore_win32`]`.
//!That extension’s issues apply equally to this extension.1) Should D3D12 fence handle types be
//! supported, like they are for
//!semaphores? **RESOLVED** : No.
//!Doing so would require extending the fence signal and wait operations to
//!provide values to signal / wait for, like [`D3D12FenceSubmitInfoKHR`]
//!does.
//!A D3D12 fence can be signaled by importing it into a [`Semaphore`]
//!instead of a [`Fence`], and applications can check status or wait on the
//!D3D12 fence using non-Vulkan APIs.
//!The convenience of being able to do these operations on [`Fence`]
//!objects does not justify the extra API complexity.
//!# Version History
//! - Revision 1, 2017-05-08 (Jesse Hall)  - Initial revision
//!# Other info
//! * 2017-05-08
//! * No known IP claims.
//! * - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Cass Everitt, Oculus  -
//!   Contributors to `[`VK_KHR_external_semaphore_win32`]`
//!# Related
//! - [`ExportFenceWin32HandleInfoKHR`]
//! - [`FenceGetWin32HandleInfoKHR`]
//! - [`ImportFenceWin32HandleInfoKHR`]
//! - [`get_fence_win32_handle_khr`]
//! - [`import_fence_win32_handle_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Device, Fence, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
    AsRaw, Unique, VulkanResult,
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
///[vkGetFenceWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html) - Get a Windows HANDLE for a fence
///# C Specifications
///To export a Windows handle representing the state of a fence, call:
///```c
///// Provided by VK_KHR_external_fence_win32
///VkResult vkGetFenceWin32HandleKHR(
///    VkDevice                                    device,
///    const VkFenceGetWin32HandleInfoKHR*         pGetWin32HandleInfo,
///    HANDLE*                                     pHandle);
///```
/// # Parameters
/// - [`device`] is the logical device that created the fence being exported.
/// - [`p_get_win_32_handle_info`] is a pointer to a [`FenceGetWin32HandleInfoKHR`] structure
///   containing parameters of the export operation.
/// - [`p_handle`] will return the Windows handle representing the fence state.
/// # Description
/// For handle types defined as NT handles, the handles returned by
/// [`get_fence_win32_handle_khr`] are owned by the application.
/// To avoid leaking resources, the application  **must**  release ownership of them
/// using the `CloseHandle` system call when they are no longer needed.Exporting a Windows handle
/// from a fence  **may**  have side effects depending on
/// the transference of the specified handle type, as described in
/// [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing).
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_get_win_32_handle_info`] **must**  be a valid pointer to a valid
///   [`FenceGetWin32HandleInfoKHR`] structure
/// - [`p_handle`] **must**  be a valid pointer to a [`HANDLE`] value
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_KHR_external_fence_win32`]
/// - [`Device`]
/// - [`FenceGetWin32HandleInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetFenceWin32HandleKHR")]
pub type FNGetFenceWin32HandleKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_get_win_32_handle_info: *const FenceGetWin32HandleInfoKHR<'lt>,
        p_handle: *mut HANDLE,
    ) -> VulkanResultCodes,
>;
///[vkImportFenceWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html) - Import a fence from a Windows HANDLE
///# C Specifications
///To import a fence payload from a Windows handle, call:
///```c
///// Provided by VK_KHR_external_fence_win32
///VkResult vkImportFenceWin32HandleKHR(
///    VkDevice                                    device,
///    const VkImportFenceWin32HandleInfoKHR*      pImportFenceWin32HandleInfo);
///```
/// # Parameters
/// - [`device`] is the logical device that created the fence.
/// - [`p_import_fence_win_32_handle_info`] is a pointer to a [`ImportFenceWin32HandleInfoKHR`]
///   structure specifying the fence and import parameters.
/// # Description
/// Importing a fence payload from Windows handles does not transfer ownership
/// of the handle to the Vulkan implementation.
/// For handle types defined as NT handles, the application  **must**  release
/// ownership using the `CloseHandle` system call when the handle is no
/// longer needed.Applications  **can**  import the same fence payload into multiple instances of
/// Vulkan, into the same instance from which it was exported, and multiple
/// times into a given Vulkan instance.
/// ## Valid Usage
/// - `fence` **must**  not be associated with any queue command that has not yet completed
///   execution on that queue
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_import_fence_win_32_handle_info`] **must**  be a valid pointer to a valid
///   [`ImportFenceWin32HandleInfoKHR`] structure
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
/// # Related
/// - [`VK_KHR_external_fence_win32`]
/// - [`Device`]
/// - [`ImportFenceWin32HandleInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkImportFenceWin32HandleKHR")]
pub type FNImportFenceWin32HandleKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_import_fence_win_32_handle_info: *const ImportFenceWin32HandleInfoKHR<'lt>,
    ) -> VulkanResultCodes,
>;
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
/// # Members
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
/// # Description
/// The handle types supported by [`handle_type`] are:
/// ## Valid Usage
/// - [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportFenceWin32HandleInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fence-handletypes-win32)
///   table
/// - If [`handle_type`] is not `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`, [`name`] **must**
///   be `NULL`
/// - If [`handle`] is `NULL`, [`name`] **must**  name a valid synchronization primitive of the type
///   specified by [`handle_type`]
/// - If [`name`] is `NULL`, [`handle`] **must**  be a valid handle of the type specified by
///   [`handle_type`]
/// - If [`handle`] is not `NULL`, [`name`] **must**  be `NULL`
/// - If [`handle`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in
///   [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
/// -    If [`name`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`fence`] **must**  be a valid [`Fence`] handle
/// - [`flags`] **must**  be a valid combination of [`FenceImportFlagBits`] values
///
/// ## Host Synchronization
/// - Host access to [`fence`] **must**  be externally synchronized
/// # Related
/// - [`VK_KHR_external_fence_win32`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`FenceImportFlags`]
/// - [`StructureType`]
/// - [`import_fence_win32_handle_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportFenceWin32HandleInfoKHR")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImportFenceWin32HandleInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`fence`] is the fence into which the state will be imported.
    pub fence: Fence,
    ///[`flags`] is a bitmask of [`FenceImportFlagBits`] specifying
    ///additional parameters for the fence payload import operation.
    pub flags: FenceImportFlags,
    ///[`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value
    ///specifying the type of [`handle`].
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    ///[`handle`] is `NULL` or the external handle to import.
    pub handle: HANDLE,
    ///[`name`] is `NULL` or a null-terminated UTF-16 string naming the
    ///underlying synchronization primitive to import.
    pub name: LPCWSTR,
}
impl<'lt> Default for ImportFenceWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> ImportFenceWin32HandleInfoKHR<'lt> {
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::handle`]
    pub fn set_handle_raw(mut self, value: HANDLE) -> Self {
        self.handle = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name_raw(mut self, value: LPCWSTR) -> Self {
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
    ///Gets the value of [`Self::fence`]
    pub fn fence(&self) -> Fence {
        self.fence
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> FenceImportFlags {
        self.flags
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalFenceHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets the value of [`Self::handle`]
    pub fn handle(&self) -> HANDLE {
        self.handle
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> LPCWSTR {
        self.name
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::fence`]
    pub fn fence_mut(&mut self) -> &mut Fence {
        &mut self.fence
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut FenceImportFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalFenceHandleTypeFlagBits {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::fence`]
    pub fn set_fence(mut self, value: crate::vulkan1_0::Fence) -> Self {
        self.fence = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(mut self, value: crate::vulkan1_1::FenceImportFlags) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::handle`]
    pub fn set_handle(mut self, value: crate::native::HANDLE) -> Self {
        self.handle = value;
        self
    }
    ///Sets the value of [`Self::name`]
    pub fn set_name(mut self, value: crate::native::LPCWSTR) -> Self {
        self.name = value;
        self
    }
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying security
///   attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
/// - [`name`] is a null-terminated UTF-16 string to associate with the underlying synchronization
///   primitive referenced by NT handles exported from the created fence.
/// # Description
/// If [`ExportFenceCreateInfo`] is not inluded in the same [`p_next`]
/// chain, this structure is ignored.If [`ExportFenceCreateInfo`] is included in the [`p_next`]
/// chain of
/// [`FenceCreateInfo`] with a Windows `handleType`, but either
/// [`ExportFenceWin32HandleInfoKHR`] is not included in the [`p_next`]
/// chain, or if it is but [`attributes`] is set to `NULL`, default security
/// descriptor values will be used, and child processes created by the
/// application will not inherit the handle, as described in the MSDN
/// documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
/// Further, if the structure is not present, the access rights will be`DXGI_SHARED_RESOURCE_READ` |
/// `DXGI_SHARED_RESOURCE_WRITE`for handles of the following
/// types:`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`
/// * [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
///
/// ## Valid Usage
/// - If [`ExportFenceCreateInfo::handle_types`] does not include
///   `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`, a [`ExportFenceWin32HandleInfoKHR`]
///   structure  **must**  not be included in the [`p_next`] chain of [`FenceCreateInfo`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR`
/// - If [`attributes`] is not `NULL`, [`attributes`] **must**  be a valid pointer to a valid
///   [`SECURITY_ATTRIBUTES`] value
/// # Related
/// - [`VK_KHR_external_fence_win32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExportFenceWin32HandleInfoKHR")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExportFenceWin32HandleInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`]
    ///structure specifying security attributes of the handle.
    pub attributes: *const SECURITY_ATTRIBUTES,
    ///[`dw_access`] is a [`DWORD`] specifying access rights of the handle.
    pub dw_access: DWORD,
    ///[`name`] is a null-terminated UTF-16 string to associate with the
    ///underlying synchronization primitive referenced by NT handles exported
    ///from the created fence.
    pub name: LPCWSTR,
}
impl<'lt> Default for ExportFenceWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            attributes: std::ptr::null(),
            dw_access: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> ExportFenceWin32HandleInfoKHR<'lt> {
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
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::dw_access`]
    pub fn set_dw_access_raw(mut self, value: DWORD) -> Self {
        self.dw_access = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name_raw(mut self, value: LPCWSTR) -> Self {
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
    pub fn attributes(&self) -> *const SECURITY_ATTRIBUTES {
        self.attributes
    }
    ///Gets the value of [`Self::dw_access`]
    pub fn dw_access(&self) -> DWORD {
        self.dw_access
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> LPCWSTR {
        self.name
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::attributes`]
    pub fn set_attributes(mut self, value: *const crate::native::SECURITY_ATTRIBUTES) -> Self {
        self.attributes = value;
        self
    }
    ///Sets the value of [`Self::dw_access`]
    pub fn set_dw_access(mut self, value: crate::native::DWORD) -> Self {
        self.dw_access = value;
        self
    }
    ///Sets the value of [`Self::name`]
    pub fn set_name(mut self, value: crate::native::LPCWSTR) -> Self {
        self.name = value;
        self
    }
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fence`] is the fence from which state will be exported.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of handle
///   requested.
/// # Description
/// The properties of the handle returned depend on the value of
/// [`handle_type`].
/// See [`ExternalFenceHandleTypeFlagBits`] for a description of the
/// properties of the defined external fence handle types.
/// ## Valid Usage
/// - [`handle_type`] **must**  have been included in [`ExportFenceCreateInfo::handle_types`] when
///   the [`fence`]’s current payload was created
/// - If [`handle_type`] is defined as an NT handle, [`get_fence_win32_handle_khr`] **must**  be
///   called no more than once for each valid unique combination of [`fence`] and [`handle_type`]
/// -  [`fence`] **must**  not currently have its payload replaced by an imported payload as described below in [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing) unless that imported payload’s handle type was included in [`ExternalFenceProperties::export_from_imported_handle_types`] for [`handle_type`]
/// - If [`handle_type`] refers to a handle type with copy payload transference semantics, [`fence`]
///   **must**  be signaled, or have an associated [fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling)
///   pending execution
/// - [`handle_type`] **must**  be defined as an NT handle or a global share handle
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`fence`] **must**  be a valid [`Fence`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalFenceHandleTypeFlagBits`] value
/// # Related
/// - [`VK_KHR_external_fence_win32`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`StructureType`]
/// - [`get_fence_win32_handle_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFenceGetWin32HandleInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FenceGetWin32HandleInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`fence`] is the fence from which state will be exported.
    pub fence: Fence,
    ///[`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
impl<'lt> Default for FenceGetWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> FenceGetWin32HandleInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::fence`]
    pub fn fence(&self) -> Fence {
        self.fence
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalFenceHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::fence`]
    pub fn fence_mut(&mut self) -> &mut Fence {
        &mut self.fence
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalFenceHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::fence`]
    pub fn set_fence(mut self, value: crate::vulkan1_0::Fence) -> Self {
        self.fence = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
}
impl Device {
    ///[vkGetFenceWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html) - Get a Windows HANDLE for a fence
    ///# C Specifications
    ///To export a Windows handle representing the state of a fence, call:
    ///```c
    ///// Provided by VK_KHR_external_fence_win32
    ///VkResult vkGetFenceWin32HandleKHR(
    ///    VkDevice                                    device,
    ///    const VkFenceGetWin32HandleInfoKHR*         pGetWin32HandleInfo,
    ///    HANDLE*                                     pHandle);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that created the fence being exported.
    /// - [`p_get_win_32_handle_info`] is a pointer to a [`FenceGetWin32HandleInfoKHR`] structure
    ///   containing parameters of the export operation.
    /// - [`p_handle`] will return the Windows handle representing the fence state.
    /// # Description
    /// For handle types defined as NT handles, the handles returned by
    /// [`get_fence_win32_handle_khr`] are owned by the application.
    /// To avoid leaking resources, the application  **must**  release ownership of them
    /// using the `CloseHandle` system call when they are no longer needed.Exporting a Windows
    /// handle from a fence  **may**  have side effects depending on
    /// the transference of the specified handle type, as described in
    /// [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing).
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_get_win_32_handle_info`] **must**  be a valid pointer to a valid
    ///   [`FenceGetWin32HandleInfoKHR`] structure
    /// - [`p_handle`] **must**  be a valid pointer to a [`HANDLE`] value
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_KHR_external_fence_win32`]
    /// - [`Device`]
    /// - [`FenceGetWin32HandleInfoKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetFenceWin32HandleKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_fence_win32_handle_khr<'lt>(
        self: &Unique<Device>,
        p_get_win_32_handle_info: &FenceGetWin32HandleInfoKHR<'lt>,
    ) -> VulkanResult<HANDLE> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_fence_win_32()
            .and_then(|vtable| vtable.get_fence_win32_handle_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_fence_win_32()
            .and_then(|vtable| vtable.get_fence_win32_handle_khr())
            .unwrap_unchecked();
        let mut p_handle = std::mem::zeroed();
        let _return = _function(
            self.as_raw(),
            p_get_win_32_handle_info as *const FenceGetWin32HandleInfoKHR<'lt>,
            &mut p_handle,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_handle),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkImportFenceWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html) - Import a fence from a Windows HANDLE
    ///# C Specifications
    ///To import a fence payload from a Windows handle, call:
    ///```c
    ///// Provided by VK_KHR_external_fence_win32
    ///VkResult vkImportFenceWin32HandleKHR(
    ///    VkDevice                                    device,
    ///    const VkImportFenceWin32HandleInfoKHR*      pImportFenceWin32HandleInfo);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that created the fence.
    /// - [`p_import_fence_win_32_handle_info`] is a pointer to a [`ImportFenceWin32HandleInfoKHR`]
    ///   structure specifying the fence and import parameters.
    /// # Description
    /// Importing a fence payload from Windows handles does not transfer ownership
    /// of the handle to the Vulkan implementation.
    /// For handle types defined as NT handles, the application  **must**  release
    /// ownership using the `CloseHandle` system call when the handle is no
    /// longer needed.Applications  **can**  import the same fence payload into multiple instances
    /// of
    /// Vulkan, into the same instance from which it was exported, and multiple
    /// times into a given Vulkan instance.
    /// ## Valid Usage
    /// - `fence` **must**  not be associated with any queue command that has not yet completed
    ///   execution on that queue
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_import_fence_win_32_handle_info`] **must**  be a valid pointer to a valid
    ///   [`ImportFenceWin32HandleInfoKHR`] structure
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    /// # Related
    /// - [`VK_KHR_external_fence_win32`]
    /// - [`Device`]
    /// - [`ImportFenceWin32HandleInfoKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkImportFenceWin32HandleKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn import_fence_win32_handle_khr<'lt>(
        self: &Unique<Device>,
        p_import_fence_win_32_handle_info: &ImportFenceWin32HandleInfoKHR<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_fence_win_32()
            .and_then(|vtable| vtable.import_fence_win32_handle_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_fence_win_32()
            .and_then(|vtable| vtable.import_fence_win32_handle_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_import_fence_win_32_handle_info as *const ImportFenceWin32HandleInfoKHR<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_external_fence_win32`
pub struct DeviceKhrExternalFenceWin32VTable {
    ///See [`FNGetFenceWin32HandleKhr`] for more information.
    pub get_fence_win32_handle_khr: FNGetFenceWin32HandleKhr,
    ///See [`FNImportFenceWin32HandleKhr`] for more information.
    pub import_fence_win32_handle_khr: FNImportFenceWin32HandleKhr,
}
impl DeviceKhrExternalFenceWin32VTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_fence_win32_handle_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetFenceWin32HandleKHR").as_ptr()))
            },
            import_fence_win32_handle_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkImportFenceWin32HandleKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_fence_win32_handle_khr`]. See [`FNGetFenceWin32HandleKhr`] for more
    /// information.
    pub fn get_fence_win32_handle_khr(&self) -> FNGetFenceWin32HandleKhr {
        self.get_fence_win32_handle_khr
    }
    ///Gets [`Self::import_fence_win32_handle_khr`]. See [`FNImportFenceWin32HandleKhr`] for more
    /// information.
    pub fn import_fence_win32_handle_khr(&self) -> FNImportFenceWin32HandleKhr {
        self.import_fence_win32_handle_khr
    }
}
