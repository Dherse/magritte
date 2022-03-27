use crate::{
    vulkan1_0::{BaseInStructure, Semaphore, StructureType},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_semaphore_fd");
///[VkImportSemaphoreFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreFdInfoKHR.html) - Structure specifying POSIX file descriptor to import to a semaphore
///# C Specifications
///The [`ImportSemaphoreFdInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_semaphore_fd
///typedef struct VkImportSemaphoreFdInfoKHR {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    VkSemaphore                              semaphore;
///    VkSemaphoreImportFlags                   flags;
///    VkExternalSemaphoreHandleTypeFlagBits    handleType;
///    int                                      fd;
///} VkImportSemaphoreFdInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore`] is the semaphore into which the payload will be imported.
/// - [`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying additional parameters for the
///   semaphore payload import operation.
/// - [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of
///   [`fd`].
/// - [`fd`] is the external handle to import.
///# Description
///The handle types supported by [`handle_type`] are:Valid Usage
/// - [`handle_type`]**must** be a value included in the [Handle Types Supported by [`ImportSemaphoreFdInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphore-handletypes-fd)
///   table
/// -  [`fd`]**must** obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
/// - If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT`, the
///   [`SemaphoreCreateInfo`]::[`flags`] field **must** match that of the semaphore from which
///   [`fd`] was exported
/// - If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT`, the
///   [`SemaphoreTypeCreateInfo::semaphore_type`] field **must** match that of the semaphore from
///   which [`fd`] was exported
/// - If [`flags`] contains `VK_SEMAPHORE_IMPORT_TEMPORARY_BIT`, the
///   [`SemaphoreTypeCreateInfo::semaphore_type`] field of the semaphore from which [`fd`] was
///   exported **must** not be `VK_SEMAPHORE_TYPE_TIMELINE`
///If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT`,
///the special value `-1` for [`fd`] is treated like a valid sync file
///descriptor referring to an object that has already signaled.
///The import operation will succeed and the [`Semaphore`] will have a
///temporarily imported payload as if a valid file descriptor had been
///provided.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`semaphore`]**must** be a valid [`Semaphore`] handle
/// - [`flags`]**must** be a valid combination of [`SemaphoreImportFlagBits`] values
/// - [`handle_type`]**must** be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///Host Synchronization
/// - Host access to [`semaphore`]**must** be externally synchronized
///# Related
/// - [`VK_KHR_external_semaphore_fd`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`SemaphoreImportFlags`]
/// - [`StructureType`]
/// - [`ImportSemaphoreFdKHR`]
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
pub struct ImportSemaphoreFdInfoKHR<'lt> {
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
    ///specifying the type of [`fd`].
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
    ///[`fd`] is the external handle to import.
    fd: i32,
}
///[VkSemaphoreGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetFdInfoKHR.html) - Structure describing a POSIX FD semaphore export operation
///# C Specifications
///The [`SemaphoreGetFdInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_semaphore_fd
///typedef struct VkSemaphoreGetFdInfoKHR {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    VkSemaphore                              semaphore;
///    VkExternalSemaphoreHandleTypeFlagBits    handleType;
///} VkSemaphoreGetFdInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore`] is the semaphore from which state will be exported.
/// - [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of
///   handle requested.
///# Description
///The properties of the file descriptor returned depend on the value of
///[`handle_type`].
///See [`ExternalSemaphoreHandleTypeFlagBits`] for a description of the
///properties of the defined external semaphore handle types.Valid Usage
/// - [`handle_type`]**must** have been included in [`ExportSemaphoreCreateInfo::handle_types`] when
///   [`semaphore`]’s current payload was created
/// -  [`semaphore`]**must** not currently have its payload replaced by an imported payload as described below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) unless that imported payload’s handle type was included in [`ExternalSemaphoreProperties::export_from_imported_handle_types`] for [`handle_type`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, as defined below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), there **must** be no queue waiting on [`semaphore`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`semaphore`]**must** be signaled, or have an associated [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) pending execution
/// - [`handle_type`]**must** be defined as a POSIX file descriptor handle
/// - If [`handle_type`] refers to a handle type with copy payload transference semantics,
///   [`semaphore`]**must** have been created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`
/// - If [`handle_type`] refers to a handle type with copy payload transference semantics,
///   [`semaphore`]**must** have an associated semaphore signal operation that has been submitted
///   for execution and any semaphore signal operations on which it depends (if any) **must** have
///   also been submitted for execution
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`semaphore`]**must** be a valid [`Semaphore`] handle
/// - [`handle_type`]**must** be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_semaphore_fd`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`GetSemaphoreFdKHR`]
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
pub struct SemaphoreGetFdInfoKHR<'lt> {
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
