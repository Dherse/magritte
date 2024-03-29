//![VK_KHR_external_semaphore_fd](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_fd.html) - device extension
//!# Description
//!An application using external memory may wish to synchronize access to that
//!memory using semaphores.
//!This extension enables an application to export semaphore payload to and
//!import semaphore payload from POSIX file descriptors.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_external_semaphore`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_semaphore_fd]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_semaphore_fd extension>>)
//!# New commands
//! - [`get_semaphore_fd_khr`]
//! - [`import_semaphore_fd_khr`]
//!# New structures
//! - [`ImportSemaphoreFdInfoKHR`]
//! - [`SemaphoreGetFdInfoKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR`
//!# Known issues & F.A.Q.
//!1) Does the application need to close the file descriptor returned by
//![`get_semaphore_fd_khr`]? **RESOLVED** : Yes, unless it is passed back in to a driver instance
//! to import
//!the semaphore.
//!A successful get call transfers ownership of the file descriptor to the
//!application, and a successful import transfers it back to the driver.
//!Destroying the original semaphore object will not close the file descriptor
//!or remove its reference to the underlying semaphore resource associated with
//!it.
//!# Version history
//! - Revision 1, 2016-10-21 (Jesse Hall)  - Initial revision
//!# Other information
//! * 2016-10-21
//! * No known IP claims.
//! * - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Carsten Rohde, NVIDIA
//!# Related
//! - [`ImportSemaphoreFdInfoKHR`]
//! - [`SemaphoreGetFdInfoKHR`]
//! - [`get_semaphore_fd_khr`]
//! - [`import_semaphore_fd_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, Device, Semaphore, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
    AsRaw, Unique, VulkanResult,
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
///[vkGetSemaphoreFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html) - Get a POSIX file descriptor handle for a semaphore
///# C Specifications
///To export a POSIX file descriptor representing the payload of a semaphore,
///call:
///```c
///// Provided by VK_KHR_external_semaphore_fd
///VkResult vkGetSemaphoreFdKHR(
///    VkDevice                                    device,
///    const VkSemaphoreGetFdInfoKHR*              pGetFdInfo,
///    int*                                        pFd);
///```
///# Parameters
/// - [`device`] is the logical device that created the semaphore being exported.
/// - [`p_get_fd_info`] is a pointer to a [`SemaphoreGetFdInfoKHR`] structure containing parameters
///   of the export operation.
/// - [`p_fd`] will return the file descriptor representing the semaphore payload.
///# Description
///Each call to [`get_semaphore_fd_khr`] **must**  create a new file descriptor
///and transfer ownership of it to the application.
///To avoid leaking resources, the application  **must**  release ownership of the
///file descriptor when it is no longer needed.Where supported by the operating system, the
/// implementation  **must**  set the
///file descriptor to be closed automatically when an `execve` system call
///is made.Exporting a file descriptor from a semaphore  **may**  have side effects
///depending on the transference of the specified handle type, as described in
///[Importing Semaphore State](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing).
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_get_fd_info`] **must**  be a valid pointer to a valid [`SemaphoreGetFdInfoKHR`] structure
/// - [`p_fd`] **must**  be a valid pointer to an `int` value
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
///# Related
/// - [`khr_external_semaphore_fd`]
/// - [`Device`]
/// - [`SemaphoreGetFdInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetSemaphoreFdKHR")]
pub type FNGetSemaphoreFdKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const SemaphoreGetFdInfoKHR<'lt>,
        p_fd: *mut i32,
    ) -> VulkanResultCodes,
>;
///[vkImportSemaphoreFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html) - Import a semaphore from a POSIX file descriptor
///# C Specifications
///To import a semaphore payload from a POSIX file descriptor, call:
///```c
///// Provided by VK_KHR_external_semaphore_fd
///VkResult vkImportSemaphoreFdKHR(
///    VkDevice                                    device,
///    const VkImportSemaphoreFdInfoKHR*           pImportSemaphoreFdInfo);
///```
///# Parameters
/// - [`device`] is the logical device that created the semaphore.
/// - [`p_import_semaphore_fd_info`] is a pointer to a [`ImportSemaphoreFdInfoKHR`] structure
///   specifying the semaphore and import parameters.
///# Description
///Importing a semaphore payload from a file descriptor transfers ownership of
///the file descriptor from the application to the Vulkan implementation.
///The application  **must**  not perform any operations on the file descriptor
///after a successful import.Applications  **can**  import the same semaphore payload into multiple
/// instances
///of Vulkan, into the same instance from which it was exported, and multiple
///times into a given Vulkan instance.
///## Valid Usage
/// - `semaphore` **must**  not be associated with any queue command that has not yet completed
///   execution on that queue
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_import_semaphore_fd_info`] **must**  be a valid pointer to a valid
///   [`ImportSemaphoreFdInfoKHR`] structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///# Related
/// - [`khr_external_semaphore_fd`]
/// - [`Device`]
/// - [`ImportSemaphoreFdInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkImportSemaphoreFdKHR")]
pub type FNImportSemaphoreFdKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'lt>,
    ) -> VulkanResultCodes,
>;
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
///The handle types supported by [`handle_type`] are:
///## Valid Usage
/// - [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportSemaphoreFdInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphore-handletypes-fd)
///   table
/// -  [`fd`] **must**  obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
/// - If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT`, the
///   [`SemaphoreCreateInfo`]::[`flags`] field  **must**  match that of the semaphore from which
///   [`fd`] was exported
/// - If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT`, the
///   [`SemaphoreTypeCreateInfo::semaphore_type`] field  **must**  match that of the semaphore from
///   which [`fd`] was exported
/// - If [`flags`] contains `VK_SEMAPHORE_IMPORT_TEMPORARY_BIT`, the
///   [`SemaphoreTypeCreateInfo::semaphore_type`] field of the semaphore from which [`fd`] was
///   exported  **must**  not be `VK_SEMAPHORE_TYPE_TIMELINE`
///If [`handle_type`] is `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT`,
///the special value `-1` for [`fd`] is treated like a valid sync file
///descriptor referring to an object that has already signaled.
///The import operation will succeed and the [`Semaphore`] will have a
///temporarily imported payload as if a valid file descriptor had been
///provided.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`semaphore`] **must**  be a valid [`Semaphore`] handle
/// - [`flags`] **must**  be a valid combination of [`SemaphoreImportFlagBits`] values
/// - [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///
///## Host Synchronization
/// - Host access to [`semaphore`] **must**  be externally synchronized
///# Related
/// - [`khr_external_semaphore_fd`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`SemaphoreImportFlags`]
/// - [`StructureType`]
/// - [`import_semaphore_fd_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportSemaphoreFdInfoKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImportSemaphoreFdInfoKHR<'lt> {
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
    ///specifying the type of [`fd`].
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    ///[`fd`] is the external handle to import.
    pub fd: i32,
}
impl<'lt> Default for ImportSemaphoreFdInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_SEMAPHORE_FD_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: 0,
        }
    }
}
impl<'lt> ImportSemaphoreFdInfoKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> ImportSemaphoreFdInfoKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> SemaphoreImportFlags {
        self.flags
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalSemaphoreHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets the value of [`Self::fd`]
    pub fn fd(&self) -> i32 {
        self.fd
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
    ///Gets a mutable reference to the value of [`Self::fd`]
    pub fn fd_mut(&mut self) -> &mut i32 {
        &mut self.fd
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
    ///Sets the value of [`Self::fd`]
    pub fn set_fd(&mut self, value: i32) -> &mut Self {
        self.fd = value;
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
    ///Sets the value of [`Self::fd`]
    pub fn with_fd(mut self, value: i32) -> Self {
        self.fd = value;
        self
    }
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
///properties of the defined external semaphore handle types.
///## Valid Usage
/// - [`handle_type`] **must**  have been included in [`ExportSemaphoreCreateInfo::handle_types`]
///   when [`semaphore`]’s current payload was created
/// -  [`semaphore`] **must**  not currently have its payload replaced by an imported payload as described below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) unless that imported payload’s handle type was included in [`ExternalSemaphoreProperties::export_from_imported_handle_types`] for [`handle_type`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, as defined below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), there  **must**  be no queue waiting on [`semaphore`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`semaphore`] **must**  be signaled, or have an associated [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) pending execution
/// - [`handle_type`] **must**  be defined as a POSIX file descriptor handle
/// - If [`handle_type`] refers to a handle type with copy payload transference semantics,
///   [`semaphore`] **must**  have been created with a [`SemaphoreType`] of
///   `VK_SEMAPHORE_TYPE_BINARY`
/// - If [`handle_type`] refers to a handle type with copy payload transference semantics,
///   [`semaphore`] **must**  have an associated semaphore signal operation that has been submitted
///   for execution and any semaphore signal operations on which it depends (if any)  **must**  have
///   also been submitted for execution
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`semaphore`] **must**  be a valid [`Semaphore`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///# Related
/// - [`khr_external_semaphore_fd`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`get_semaphore_fd_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSemaphoreGetFdInfoKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SemaphoreGetFdInfoKHR<'lt> {
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
impl<'lt> Default for SemaphoreGetFdInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SEMAPHORE_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> SemaphoreGetFdInfoKHR<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> SemaphoreGetFdInfoKHR<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
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
    ///[vkGetSemaphoreFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html) - Get a POSIX file descriptor handle for a semaphore
    ///# C Specifications
    ///To export a POSIX file descriptor representing the payload of a semaphore,
    ///call:
    ///```c
    ///// Provided by VK_KHR_external_semaphore_fd
    ///VkResult vkGetSemaphoreFdKHR(
    ///    VkDevice                                    device,
    ///    const VkSemaphoreGetFdInfoKHR*              pGetFdInfo,
    ///    int*                                        pFd);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that created the semaphore being exported.
    /// - [`p_get_fd_info`] is a pointer to a [`SemaphoreGetFdInfoKHR`] structure containing
    ///   parameters of the export operation.
    /// - [`p_fd`] will return the file descriptor representing the semaphore payload.
    ///# Description
    ///Each call to [`get_semaphore_fd_khr`] **must**  create a new file descriptor
    ///and transfer ownership of it to the application.
    ///To avoid leaking resources, the application  **must**  release ownership of the
    ///file descriptor when it is no longer needed.Where supported by the operating system, the
    /// implementation  **must**  set the
    ///file descriptor to be closed automatically when an `execve` system call
    ///is made.Exporting a file descriptor from a semaphore  **may**  have side effects
    ///depending on the transference of the specified handle type, as described in
    ///[Importing Semaphore State](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing).
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_get_fd_info`] **must**  be a valid pointer to a valid [`SemaphoreGetFdInfoKHR`]
    ///   structure
    /// - [`p_fd`] **must**  be a valid pointer to an `int` value
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///# Related
    /// - [`khr_external_semaphore_fd`]
    /// - [`Device`]
    /// - [`SemaphoreGetFdInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetSemaphoreFdKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_semaphore_fd_khr<'lt>(
        self: &Unique<Device>,
        p_get_fd_info: &SemaphoreGetFdInfoKHR<'lt>,
    ) -> VulkanResult<i32> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_semaphore_fd()
            .and_then(|vtable| vtable.get_semaphore_fd_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_semaphore_fd()
            .and_then(|vtable| vtable.get_semaphore_fd_khr())
            .unwrap_unchecked();
        let mut p_fd = Default::default();
        let _return = _function(
            self.as_raw(),
            p_get_fd_info as *const SemaphoreGetFdInfoKHR<'lt>,
            &mut p_fd,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_fd),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkImportSemaphoreFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html) - Import a semaphore from a POSIX file descriptor
    ///# C Specifications
    ///To import a semaphore payload from a POSIX file descriptor, call:
    ///```c
    ///// Provided by VK_KHR_external_semaphore_fd
    ///VkResult vkImportSemaphoreFdKHR(
    ///    VkDevice                                    device,
    ///    const VkImportSemaphoreFdInfoKHR*           pImportSemaphoreFdInfo);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that created the semaphore.
    /// - [`p_import_semaphore_fd_info`] is a pointer to a [`ImportSemaphoreFdInfoKHR`] structure
    ///   specifying the semaphore and import parameters.
    ///# Description
    ///Importing a semaphore payload from a file descriptor transfers ownership of
    ///the file descriptor from the application to the Vulkan implementation.
    ///The application  **must**  not perform any operations on the file descriptor
    ///after a successful import.Applications  **can**  import the same semaphore payload into
    /// multiple instances
    ///of Vulkan, into the same instance from which it was exported, and multiple
    ///times into a given Vulkan instance.
    ///## Valid Usage
    /// - `semaphore` **must**  not be associated with any queue command that has not yet completed
    ///   execution on that queue
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_import_semaphore_fd_info`] **must**  be a valid pointer to a valid
    ///   [`ImportSemaphoreFdInfoKHR`] structure
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    ///# Related
    /// - [`khr_external_semaphore_fd`]
    /// - [`Device`]
    /// - [`ImportSemaphoreFdInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkImportSemaphoreFdKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn import_semaphore_fd_khr<'lt>(
        self: &Unique<Device>,
        p_import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_semaphore_fd()
            .and_then(|vtable| vtable.import_semaphore_fd_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_semaphore_fd()
            .and_then(|vtable| vtable.import_semaphore_fd_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_import_semaphore_fd_info as *const ImportSemaphoreFdInfoKHR<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_external_semaphore_fd`
pub struct DeviceKhrExternalSemaphoreFdVTable {
    ///See [`FNGetSemaphoreFdKhr`] for more information.
    pub get_semaphore_fd_khr: FNGetSemaphoreFdKhr,
    ///See [`FNImportSemaphoreFdKhr`] for more information.
    pub import_semaphore_fd_khr: FNImportSemaphoreFdKhr,
}
impl DeviceKhrExternalSemaphoreFdVTable {
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
            get_semaphore_fd_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetSemaphoreFdKHR").as_ptr()))
            },
            import_semaphore_fd_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkImportSemaphoreFdKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_semaphore_fd_khr`]. See [`FNGetSemaphoreFdKhr`] for more information.
    pub fn get_semaphore_fd_khr(&self) -> FNGetSemaphoreFdKhr {
        self.get_semaphore_fd_khr
    }
    ///Gets [`Self::import_semaphore_fd_khr`]. See [`FNImportSemaphoreFdKhr`] for more information.
    pub fn import_semaphore_fd_khr(&self) -> FNImportSemaphoreFdKhr {
        self.import_semaphore_fd_khr
    }
}
