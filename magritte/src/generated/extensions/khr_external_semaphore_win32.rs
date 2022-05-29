//![VK_KHR_external_semaphore_win32](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_win32.html) - device extension
//!# Description
//!An application using external memory may wish to synchronize access to that
//!memory using semaphores.
//!This extension enables an application to export semaphore payload to and
//!import semaphore payload from Windows handles.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_external_semaphore`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_semaphore_win32]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_semaphore_win32 extension>>)
//!# New functions & commands
//! - [`get_semaphore_win32_handle_khr`]
//! - [`import_semaphore_win32_handle_khr`]
//!# New structures
//! - [`ImportSemaphoreWin32HandleInfoKHR`]
//! - [`SemaphoreGetWin32HandleInfoKHR`]
//! - Extending [`SemaphoreCreateInfo`]:  - [`ExportSemaphoreWin32HandleInfoKHR`]
//! - Extending [`SubmitInfo`]:  - [`D3d12FenceSubmitInfoKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Do applications need to call `CloseHandle`() on the values returned
//!from [`get_semaphore_win32_handle_khr`] when `handleType` is
//!`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`? **RESOLVED** : Yes, unless it is
//! passed back in to another driver instance to
//!import the object.
//!A successful get call transfers ownership of the handle to the application.
//!Destroying the semaphore object will not destroy the handle or the handle’s
//!reference to the underlying semaphore resource.2) Should the language regarding KMT/Windows 7
//! handles be moved to a
//!separate extension so that it can be deprecated over time? **RESOLVED** : No.
//!Support for them can be deprecated by drivers if they choose, by no longer
//!returning them in the supported handle types of the instance level queries.3) Should
//! applications be allowed to specify additional object attributes
//!for shared handles? **RESOLVED** : Yes.
//!Applications will be allowed to provide similar attributes to those they
//!would to any other handle creation API.4) How do applications communicate the desired fence
//! values to use with
//!`D3D12_FENCE`-based Vulkan semaphores? **RESOLVED** : There are a couple of options.
//!The values for the signaled and reset states could be communicated up front
//!when creating the object and remain static for the life of the Vulkan
//!semaphore, or they could be specified using auxiliary structures when
//!submitting semaphore signal and wait operations, similar to what is done
//!with the keyed mutex extensions.
//!The latter is more flexible and consistent with the keyed mutex usage, but
//!the former is a much simpler API.Since Vulkan tends to favor flexibility and consistency over
//! simplicity, a
//!new structure specifying D3D12 fence acquire and release values is added to
//!the [`queue_submit`] function.
//!# Version History
//! - Revision 1, 2016-10-21 (James Jones)  - Initial revision
//!# Other info
//! * 2016-10-21
//! * No known IP claims.
//! * - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Carsten Rohde, NVIDIA
//!# Related
//! - [`D3d12FenceSubmitInfoKHR`]
//! - [`ExportSemaphoreWin32HandleInfoKHR`]
//! - [`ImportSemaphoreWin32HandleInfoKHR`]
//! - [`SemaphoreGetWin32HandleInfoKHR`]
//! - [`get_semaphore_win32_handle_khr`]
//! - [`import_semaphore_win32_handle_khr`]
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
    vulkan1_0::{BaseInStructure, Device, Semaphore, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
    AsRaw, Unique, VulkanResult,
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
///[vkGetSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) - Get a Windows HANDLE for a semaphore
///# C Specifications
///To export a Windows handle representing the payload of a semaphore, call:
///```c
///// Provided by VK_KHR_external_semaphore_win32
///VkResult vkGetSemaphoreWin32HandleKHR(
///    VkDevice                                    device,
///    const VkSemaphoreGetWin32HandleInfoKHR*     pGetWin32HandleInfo,
///    HANDLE*                                     pHandle);
///```
/// # Parameters
/// - [`device`] is the logical device that created the semaphore being exported.
/// - [`p_get_win32_handle_info`] is a pointer to a [`SemaphoreGetWin32HandleInfoKHR`] structure
///   containing parameters of the export operation.
/// - [`p_handle`] will return the Windows handle representing the semaphore state.
/// # Description
/// For handle types defined as NT handles, the handles returned by
/// [`get_semaphore_win32_handle_khr`] are owned by the application.
/// To avoid leaking resources, the application  **must**  release ownership of them
/// using the `CloseHandle` system call when they are no longer needed.Exporting a Windows handle
/// from a semaphore  **may**  have side effects depending
/// on the transference of the specified handle type, as described in
/// [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing).
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_get_win32_handle_info`] **must**  be a valid pointer to a valid
///   [`SemaphoreGetWin32HandleInfoKHR`] structure
/// - [`p_handle`] **must**  be a valid pointer to a [`HANDLE`] value
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`khr_external_semaphore_win32`]
/// - [`Device`]
/// - [`SemaphoreGetWin32HandleInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
pub type FNGetSemaphoreWin32HandleKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR<'lt>,
        p_handle: *mut HANDLE,
    ) -> VulkanResultCodes,
>;
///[vkImportSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) - Import a semaphore from a Windows HANDLE
///# C Specifications
///To import a semaphore payload from a Windows handle, call:
///```c
///// Provided by VK_KHR_external_semaphore_win32
///VkResult vkImportSemaphoreWin32HandleKHR(
///    VkDevice                                    device,
///    const VkImportSemaphoreWin32HandleInfoKHR*  pImportSemaphoreWin32HandleInfo);
///```
/// # Parameters
/// - [`device`] is the logical device that created the semaphore.
/// - [`p_import_semaphore_win32_handle_info`] is a pointer to a
///   [`ImportSemaphoreWin32HandleInfoKHR`] structure specifying the semaphore and import
///   parameters.
/// # Description
/// Importing a semaphore payload from Windows handles does not transfer
/// ownership of the handle to the Vulkan implementation.
/// For handle types defined as NT handles, the application  **must**  release
/// ownership using the `CloseHandle` system call when the handle is no
/// longer needed.Applications  **can**  import the same semaphore payload into multiple instances
/// of Vulkan, into the same instance from which it was exported, and multiple
/// times into a given Vulkan instance.
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_import_semaphore_win32_handle_info`] **must**  be a valid pointer to a valid
///   [`ImportSemaphoreWin32HandleInfoKHR`] structure
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
/// # Related
/// - [`khr_external_semaphore_win32`]
/// - [`Device`]
/// - [`ImportSemaphoreWin32HandleInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
pub type FNImportSemaphoreWin32HandleKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR<'lt>,
    ) -> VulkanResultCodes,
>;
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
/// # Members
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
/// # Description
/// The handle types supported by [`handle_type`] are:
/// ## Valid Usage
/// - [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportSemaphoreWin32HandleInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphore-handletypes-win32)
///   table
/// - If [`handle_type`] is not `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`, [`name`] **must**  be `NULL`
/// - If [`handle`] is `NULL`, [`name`] **must**  name a valid synchronization primitive of the type
///   specified by [`handle_type`]
/// - If [`name`] is `NULL`, [`handle`] **must**  be a valid handle of the type specified by
///   [`handle_type`]
/// - If [`handle`] is not `NULL`, [`name`] **must**  be `NULL`
/// - If [`handle`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in
///   [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
/// -    If [`name`] is not `NULL`, it  **must**  obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
/// - If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`, the
///   [`SemaphoreCreateInfo`]::[`flags`] field  **must**  match that of the semaphore from which
///   [`handle`] or [`name`] was exported
/// - If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`, the
///   [`SemaphoreTypeCreateInfo::semaphore_type`] field  **must**  match that of the semaphore from
///   which [`handle`] or [`name`] was exported
/// - If [`flags`] contains `VK_SEMAPHORE_IMPORT_TEMPORARY_BIT`, the
///   [`SemaphoreTypeCreateInfo::semaphore_type`] field of the semaphore from which [`handle`] or
///   [`name`] was exported  **must**  not be `VK_SEMAPHORE_TYPE_TIMELINE`
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`semaphore`] **must**  be a valid [`Semaphore`] handle
/// - [`flags`] **must**  be a valid combination of [`SemaphoreImportFlagBits`] values
///
/// ## Host Synchronization
/// - Host access to [`semaphore`] **must**  be externally synchronized
/// # Related
/// - [`khr_external_semaphore_win32`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`SemaphoreImportFlags`]
/// - [`StructureType`]
/// - [`import_semaphore_win32_handle_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportSemaphoreWin32HandleInfoKHR")]
#[repr(C)]
pub struct ImportSemaphoreWin32HandleInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`semaphore`] is the semaphore into which the payload will be
    ///imported.
    pub semaphore: Semaphore,
    ///[`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying
    ///additional parameters for the semaphore payload import operation.
    pub flags: SemaphoreImportFlags,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the type of [`handle`].
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    ///[`handle`] is `NULL` or the external handle to import.
    pub handle: HANDLE,
    ///[`name`] is `NULL` or a null-terminated UTF-16 string naming the
    ///underlying synchronization primitive to import.
    pub name: LPCWSTR,
}
impl<'lt> Default for ImportSemaphoreWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
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
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::handle`]
    pub fn with_handle_raw(mut self, value: HANDLE) -> Self {
        self.handle = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn with_name_raw(mut self, value: LPCWSTR) -> Self {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::semaphore`]
    pub fn set_semaphore(&mut self, value: crate::vulkan1_0::Semaphore) -> &mut Self {
        self.semaphore = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::vulkan1_1::SemaphoreImportFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::handle`]
    pub fn set_handle(&mut self, value: crate::native::HANDLE) -> &mut Self {
        self.handle = value;
        self
    }
    ///Sets the value of [`Self::name`]
    pub fn set_name(&mut self, value: crate::native::LPCWSTR) -> &mut Self {
        self.name = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::semaphore`]
    pub fn with_semaphore(mut self, value: crate::vulkan1_0::Semaphore) -> Self {
        self.semaphore = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::vulkan1_1::SemaphoreImportFlags) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn with_handle_type(mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::handle`]
    pub fn with_handle(mut self, value: crate::native::HANDLE) -> Self {
        self.handle = value;
        self
    }
    ///Sets the value of [`Self::name`]
    pub fn with_name(mut self, value: crate::native::LPCWSTR) -> Self {
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying security
///   attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
/// - [`name`] is a null-terminated UTF-16 string to associate with the underlying synchronization
///   primitive referenced by NT handles exported from the created semaphore.
/// # Description
/// If [`ExportSemaphoreCreateInfo`] is not included in the same [`p_next`]
/// chain, this structure is ignored.If [`ExportSemaphoreCreateInfo`] is included in the [`p_next`]
/// chain of
/// [`SemaphoreCreateInfo`] with a Windows `handleType`, but either
/// [`ExportSemaphoreWin32HandleInfoKHR`] is not included in the [`p_next`]
/// chain, or if it is but [`attributes`] is set to `NULL`, default security
/// descriptor values will be used, and child processes created by the
/// application will not inherit the handle, as described in the MSDN
/// documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
/// Further, if the structure is not present, the access rights used depend on
/// the handle type.For handles of the following
/// types:`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT`The implementation  **must**  ensure
/// the access rights allow both signal and wait
/// operations on the semaphore.For handles of the following
/// types:`VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`The access rights  **must**
/// be:`GENERIC_ALL`
/// * [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
///
/// ## Valid Usage
/// - If [`ExportSemaphoreCreateInfo::handle_types`] does not include
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT` or
///   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT`, [`ExportSemaphoreWin32HandleInfoKHR`]
///   **must**  not be included in the [`p_next`] chain of [`SemaphoreCreateInfo`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR`
/// - If [`attributes`] is not `NULL`, [`attributes`] **must**  be a valid pointer to a valid
///   [`SECURITY_ATTRIBUTES`] value
/// # Related
/// - [`khr_external_semaphore_win32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExportSemaphoreWin32HandleInfoKHR")]
#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKHR<'lt> {
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
    ///from the created semaphore.
    pub name: LPCWSTR,
}
impl<'lt> Default for ExportSemaphoreWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: std::ptr::null(),
            attributes: std::ptr::null(),
            dw_access: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
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
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::dw_access`]
    pub fn with_dw_access_raw(mut self, value: DWORD) -> Self {
        self.dw_access = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn with_name_raw(mut self, value: LPCWSTR) -> Self {
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
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::attributes`]
    pub fn set_attributes(&mut self, value: *const crate::native::SECURITY_ATTRIBUTES) -> &mut Self {
        self.attributes = value;
        self
    }
    ///Sets the value of [`Self::dw_access`]
    pub fn set_dw_access(&mut self, value: crate::native::DWORD) -> &mut Self {
        self.dw_access = value;
        self
    }
    ///Sets the value of [`Self::name`]
    pub fn set_name(&mut self, value: crate::native::LPCWSTR) -> &mut Self {
        self.name = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::attributes`]
    pub fn with_attributes(mut self, value: *const crate::native::SECURITY_ATTRIBUTES) -> Self {
        self.attributes = value;
        self
    }
    ///Sets the value of [`Self::dw_access`]
    pub fn with_dw_access(mut self, value: crate::native::DWORD) -> Self {
        self.dw_access = value;
        self
    }
    ///Sets the value of [`Self::name`]
    pub fn with_name(mut self, value: crate::native::LPCWSTR) -> Self {
        self.name = value;
        self
    }
}
///[VkD3D12FenceSubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkD3D12FenceSubmitInfoKHR.html) - Structure specifying values for Direct3D 12 fence-backed semaphores
///# C Specifications
///To specify the values to use when waiting for and signaling semaphores whose
///[current payload](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) refers to a
///Direct3D 12 fence, add a [`D3d12FenceSubmitInfoKHR`] structure to the
///[`p_next`] chain of the [`SubmitInfo`] structure.
///The [`D3d12FenceSubmitInfoKHR`] structure is defined as:
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
/// # Members
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
/// # Description
/// If the semaphore in [`SubmitInfo::wait_semaphores`] or
/// [`SubmitInfo::signal_semaphores`] corresponding to an entry in
/// [`wait_semaphore_values`] or [`signal_semaphore_values`] respectively does
/// not currently have a [payload](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-payloads)
/// referring to a Direct3D 12 fence, the implementation  **must**  ignore the value
/// in the [`wait_semaphore_values`] or [`signal_semaphore_values`] entry.
/// ## Valid Usage
/// - [`wait_semaphore_values_count`] **must**  be the same value as
///   [`SubmitInfo::wait_semaphore_count`], where [`SubmitInfo`] is in the [`p_next`] chain of this
///   [`D3d12FenceSubmitInfoKHR`] structure
/// - [`signal_semaphore_values_count`] **must**  be the same value as
///   [`SubmitInfo::signal_semaphore_count`], where [`SubmitInfo`] is in the [`p_next`] chain of
///   this [`D3d12FenceSubmitInfoKHR`] structure
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR`
/// - If [`wait_semaphore_values_count`] is not `0`, and [`wait_semaphore_values`] is not `NULL`,
///   [`wait_semaphore_values`] **must**  be a valid pointer to an array of
///   [`wait_semaphore_values_count`]`uint64_t` values
/// - If [`signal_semaphore_values_count`] is not `0`, and [`signal_semaphore_values`] is not
///   `NULL`, [`signal_semaphore_values`] **must**  be a valid pointer to an array of
///   [`signal_semaphore_values_count`]`uint64_t` values
/// # Related
/// - [`khr_external_semaphore_win32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkD3D12FenceSubmitInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct D3d12FenceSubmitInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`wait_semaphore_values_count`] is the number of semaphore wait values
    ///specified in [`wait_semaphore_values`].
    pub wait_semaphore_values_count: u32,
    ///[`wait_semaphore_values`] is a pointer to an array of
    ///[`wait_semaphore_values_count`] values for the corresponding semaphores
    ///in [`SubmitInfo`]::`pWaitSemaphores` to wait for.
    pub wait_semaphore_values: *const u64,
    ///[`signal_semaphore_values_count`] is the number of semaphore signal
    ///values specified in [`signal_semaphore_values`].
    pub signal_semaphore_values_count: u32,
    ///[`signal_semaphore_values`] is a pointer to an array of
    ///[`signal_semaphore_values_count`] values for the corresponding semaphores
    ///in [`SubmitInfo`]::`pSignalSemaphores` to set when signaled.
    pub signal_semaphore_values: *const u64,
}
impl<'lt> Default for D3d12FenceSubmitInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::D3D12_FENCE_SUBMIT_INFO_KHR,
            p_next: std::ptr::null(),
            wait_semaphore_values_count: 0,
            wait_semaphore_values: std::ptr::null(),
            signal_semaphore_values_count: 0,
            signal_semaphore_values: std::ptr::null(),
        }
    }
}
impl<'lt> D3d12FenceSubmitInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::wait_semaphore_values`]
    pub fn wait_semaphore_values_raw(&self) -> *const u64 {
        self.wait_semaphore_values
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
    ///Sets the raw value of [`Self::wait_semaphore_values`]
    pub fn set_wait_semaphore_values_raw(&mut self, value: *const u64) -> &mut Self {
        self.wait_semaphore_values = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_values`]
    pub fn set_signal_semaphore_values_raw(&mut self, value: *const u64) -> &mut Self {
        self.signal_semaphore_values = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_values`]
    pub fn with_wait_semaphore_values_raw(mut self, value: *const u64) -> Self {
        self.wait_semaphore_values = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_values`]
    pub fn with_signal_semaphore_values_raw(mut self, value: *const u64) -> Self {
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
        &mut self.wait_semaphore_values_count
    }
    ///Gets a mutable reference to the value of [`Self::signal_semaphore_values_count`]
    pub fn signal_semaphore_values_count_mut(&mut self) -> &mut u32 {
        &mut self.signal_semaphore_values_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::wait_semaphore_values_count`]
    pub fn set_wait_semaphore_values_count(&mut self, value: u32) -> &mut Self {
        self.wait_semaphore_values_count = value;
        self
    }
    ///Sets the value of [`Self::wait_semaphore_values`]
    pub fn set_wait_semaphore_values(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.wait_semaphore_values = value.as_ptr();
        self.wait_semaphore_values_count = len_;
        self
    }
    ///Sets the value of [`Self::signal_semaphore_values_count`]
    pub fn set_signal_semaphore_values_count(&mut self, value: u32) -> &mut Self {
        self.signal_semaphore_values_count = value;
        self
    }
    ///Sets the value of [`Self::signal_semaphore_values`]
    pub fn set_signal_semaphore_values(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.signal_semaphore_values = value.as_ptr();
        self.signal_semaphore_values_count = len_;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::wait_semaphore_values_count`]
    pub fn with_wait_semaphore_values_count(mut self, value: u32) -> Self {
        self.wait_semaphore_values_count = value;
        self
    }
    ///Sets the value of [`Self::wait_semaphore_values`]
    pub fn with_wait_semaphore_values(mut self, value: &'lt [u64]) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.wait_semaphore_values = value.as_ptr();
        self.wait_semaphore_values_count = len_;
        self
    }
    ///Sets the value of [`Self::signal_semaphore_values_count`]
    pub fn with_signal_semaphore_values_count(mut self, value: u32) -> Self {
        self.signal_semaphore_values_count = value;
        self
    }
    ///Sets the value of [`Self::signal_semaphore_values`]
    pub fn with_signal_semaphore_values(mut self, value: &'lt [u64]) -> Self {
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore`] is the semaphore from which state will be exported.
/// - [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of
///   handle requested.
/// # Description
/// The properties of the handle returned depend on the value of
/// [`handle_type`].
/// See [`ExternalSemaphoreHandleTypeFlagBits`] for a description of the
/// properties of the defined external semaphore handle types.
/// ## Valid Usage
/// - [`handle_type`] **must**  have been included in [`ExportSemaphoreCreateInfo::handle_types`]
///   when the [`semaphore`]’s current payload was created
/// - If [`handle_type`] is defined as an NT handle, [`get_semaphore_win32_handle_khr`] **must**  be
///   called no more than once for each valid unique combination of [`semaphore`] and
///   [`handle_type`]
/// -  [`semaphore`] **must**  not currently have its payload replaced by an imported payload as described below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) unless that imported payload’s handle type was included in [`ExternalSemaphoreProperties::export_from_imported_handle_types`] for [`handle_type`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, as defined below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), there  **must**  be no queue waiting on [`semaphore`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`semaphore`] **must**  be signaled, or have an associated [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) pending execution
/// - [`handle_type`] **must**  be defined as an NT handle or a global share handle
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`semaphore`] **must**  be a valid [`Semaphore`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
/// # Related
/// - [`khr_external_semaphore_win32`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`get_semaphore_win32_handle_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSemaphoreGetWin32HandleInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`semaphore`] is the semaphore from which state will be exported.
    pub semaphore: Semaphore,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl<'lt> Default for SemaphoreGetWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
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
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::semaphore`]
    pub fn set_semaphore(&mut self, value: crate::vulkan1_0::Semaphore) -> &mut Self {
        self.semaphore = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::semaphore`]
    pub fn with_semaphore(mut self, value: crate::vulkan1_0::Semaphore) -> Self {
        self.semaphore = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn with_handle_type(mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
}
impl Device {
    ///[vkGetSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html) - Get a Windows HANDLE for a semaphore
    ///# C Specifications
    ///To export a Windows handle representing the payload of a semaphore, call:
    ///```c
    ///// Provided by VK_KHR_external_semaphore_win32
    ///VkResult vkGetSemaphoreWin32HandleKHR(
    ///    VkDevice                                    device,
    ///    const VkSemaphoreGetWin32HandleInfoKHR*     pGetWin32HandleInfo,
    ///    HANDLE*                                     pHandle);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that created the semaphore being exported.
    /// - [`p_get_win32_handle_info`] is a pointer to a [`SemaphoreGetWin32HandleInfoKHR`] structure
    ///   containing parameters of the export operation.
    /// - [`p_handle`] will return the Windows handle representing the semaphore state.
    /// # Description
    /// For handle types defined as NT handles, the handles returned by
    /// [`get_semaphore_win32_handle_khr`] are owned by the application.
    /// To avoid leaking resources, the application  **must**  release ownership of them
    /// using the `CloseHandle` system call when they are no longer needed.Exporting a Windows
    /// handle from a semaphore  **may**  have side effects depending
    /// on the transference of the specified handle type, as described in
    /// [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing).
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_get_win32_handle_info`] **must**  be a valid pointer to a valid
    ///   [`SemaphoreGetWin32HandleInfoKHR`] structure
    /// - [`p_handle`] **must**  be a valid pointer to a [`HANDLE`] value
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`khr_external_semaphore_win32`]
    /// - [`Device`]
    /// - [`SemaphoreGetWin32HandleInfoKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_semaphore_win32_handle_khr<'lt>(
        self: &Unique<Device>,
        p_get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR<'lt>,
    ) -> VulkanResult<HANDLE> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_semaphore_win32()
            .and_then(|vtable| vtable.get_semaphore_win32_handle_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_semaphore_win32()
            .and_then(|vtable| vtable.get_semaphore_win32_handle_khr())
            .unwrap_unchecked();
        let mut p_handle = std::mem::zeroed();
        let _return = _function(
            self.as_raw(),
            p_get_win32_handle_info as *const SemaphoreGetWin32HandleInfoKHR<'lt>,
            &mut p_handle,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_handle),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkImportSemaphoreWin32HandleKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html) - Import a semaphore from a Windows HANDLE
    ///# C Specifications
    ///To import a semaphore payload from a Windows handle, call:
    ///```c
    ///// Provided by VK_KHR_external_semaphore_win32
    ///VkResult vkImportSemaphoreWin32HandleKHR(
    ///    VkDevice                                    device,
    ///    const VkImportSemaphoreWin32HandleInfoKHR*  pImportSemaphoreWin32HandleInfo);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that created the semaphore.
    /// - [`p_import_semaphore_win32_handle_info`] is a pointer to a
    ///   [`ImportSemaphoreWin32HandleInfoKHR`] structure specifying the semaphore and import
    ///   parameters.
    /// # Description
    /// Importing a semaphore payload from Windows handles does not transfer
    /// ownership of the handle to the Vulkan implementation.
    /// For handle types defined as NT handles, the application  **must**  release
    /// ownership using the `CloseHandle` system call when the handle is no
    /// longer needed.Applications  **can**  import the same semaphore payload into multiple
    /// instances
    /// of Vulkan, into the same instance from which it was exported, and multiple
    /// times into a given Vulkan instance.
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_import_semaphore_win32_handle_info`] **must**  be a valid pointer to a valid
    ///   [`ImportSemaphoreWin32HandleInfoKHR`] structure
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    /// # Related
    /// - [`khr_external_semaphore_win32`]
    /// - [`Device`]
    /// - [`ImportSemaphoreWin32HandleInfoKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn import_semaphore_win32_handle_khr<'lt>(
        self: &Unique<Device>,
        p_import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_semaphore_win32()
            .and_then(|vtable| vtable.import_semaphore_win32_handle_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_semaphore_win32()
            .and_then(|vtable| vtable.import_semaphore_win32_handle_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_import_semaphore_win32_handle_info as *const ImportSemaphoreWin32HandleInfoKHR<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_external_semaphore_win32`
pub struct DeviceKhrExternalSemaphoreWin32VTable {
    ///See [`FNGetSemaphoreWin32HandleKhr`] for more information.
    pub get_semaphore_win32_handle_khr: FNGetSemaphoreWin32HandleKhr,
    ///See [`FNImportSemaphoreWin32HandleKhr`] for more information.
    pub import_semaphore_win32_handle_khr: FNImportSemaphoreWin32HandleKhr,
}
impl DeviceKhrExternalSemaphoreWin32VTable {
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
            get_semaphore_win32_handle_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetSemaphoreWin32HandleKHR").as_ptr()))
            },
            import_semaphore_win32_handle_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkImportSemaphoreWin32HandleKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_semaphore_win32_handle_khr`]. See [`FNGetSemaphoreWin32HandleKhr`] for more
    /// information.
    pub fn get_semaphore_win32_handle_khr(&self) -> FNGetSemaphoreWin32HandleKhr {
        self.get_semaphore_win32_handle_khr
    }
    ///Gets [`Self::import_semaphore_win32_handle_khr`]. See [`FNImportSemaphoreWin32HandleKhr`]
    /// for more information.
    pub fn import_semaphore_win32_handle_khr(&self) -> FNImportSemaphoreWin32HandleKhr {
        self.import_semaphore_win32_handle_khr
    }
}
