//![VK_KHR_external_fence_fd](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_fd.html) - device extension
//!# Description
//!An application using external memory may wish to synchronize access to that
//!memory using fences.
//!This extension enables an application to export fence payload to and import
//!fence payload from POSIX file descriptors.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_external_fence`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_fence_fd]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_external_fence_fd
//!   extension>>)
//!# New functions & commands
//! - [`get_fence_fd_khr`]
//! - [`import_fence_fd_khr`]
//!# New structures
//! - [`FenceGetFdInfoKHR`]
//! - [`ImportFenceFdInfoKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_FENCE_FD_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR`
//!# Known issues & F.A.Q
//!This extension borrows concepts, semantics, and language from
//!`[`khr_external_semaphore_fd`]`.
//!That extension’s issues apply equally to this extension.
//!# Version History
//! - Revision 1, 2017-05-08 (Jesse Hall)  - Initial revision
//!# Other info
//! * 2017-05-08
//! * No known IP claims.
//! * - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Cass Everitt, Oculus  -
//!   Contributors to `[`khr_external_semaphore_fd`]`
//!# Related
//! - [`FenceGetFdInfoKHR`]
//! - [`ImportFenceFdInfoKHR`]
//! - [`get_fence_fd_khr`]
//! - [`import_fence_fd_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, Device, Fence, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_fence_fd");
///[vkGetFenceFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html) - Get a POSIX file descriptor handle for a fence
///# C Specifications
///To export a POSIX file descriptor representing the payload of a fence, call:
///```c
///// Provided by VK_KHR_external_fence_fd
///VkResult vkGetFenceFdKHR(
///    VkDevice                                    device,
///    const VkFenceGetFdInfoKHR*                  pGetFdInfo,
///    int*                                        pFd);
///```
/// # Parameters
/// - [`device`] is the logical device that created the fence being exported.
/// - [`p_get_fd_info`] is a pointer to a [`FenceGetFdInfoKHR`] structure containing parameters of
///   the export operation.
/// - [`p_fd`] will return the file descriptor representing the fence payload.
/// # Description
/// Each call to [`get_fence_fd_khr`] **must**  create a new file descriptor and
/// transfer ownership of it to the application.
/// To avoid leaking resources, the application  **must**  release ownership of the
/// file descriptor when it is no longer needed.If `pGetFdInfo->handleType` is
/// `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT` and the fence is signaled at
/// the time [`get_fence_fd_khr`] is called, [`p_fd`] **may**  return the value
/// `-1` instead of a valid file descriptor.Where supported by the operating system, the
/// implementation  **must**  set the
/// file descriptor to be closed automatically when an `execve` system call
/// is made.Exporting a file descriptor from a fence  **may**  have side effects depending on
/// the transference of the specified handle type, as described in
/// [Importing Fence State](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing).
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_get_fd_info`] **must**  be a valid pointer to a valid [`FenceGetFdInfoKHR`] structure
/// - [`p_fd`] **must**  be a valid pointer to an `int` value
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`khr_external_fence_fd`]
/// - [`Device`]
/// - [`FenceGetFdInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetFenceFdKHR")]
pub type FNGetFenceFdKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const FenceGetFdInfoKHR<'lt>,
        p_fd: *mut i32,
    ) -> VulkanResultCodes,
>;
///[vkImportFenceFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html) - Import a fence from a POSIX file descriptor
///# C Specifications
///To import a fence payload from a POSIX file descriptor, call:
///```c
///// Provided by VK_KHR_external_fence_fd
///VkResult vkImportFenceFdKHR(
///    VkDevice                                    device,
///    const VkImportFenceFdInfoKHR*               pImportFenceFdInfo);
///```
/// # Parameters
/// - [`device`] is the logical device that created the fence.
/// - [`p_import_fence_fd_info`] is a pointer to a [`ImportFenceFdInfoKHR`] structure specifying the
///   fence and import parameters.
/// # Description
/// Importing a fence payload from a file descriptor transfers ownership of the
/// file descriptor from the application to the Vulkan implementation.
/// The application  **must**  not perform any operations on the file descriptor
/// after a successful import.Applications  **can**  import the same fence payload into multiple
/// instances of
/// Vulkan, into the same instance from which it was exported, and multiple
/// times into a given Vulkan instance.
/// ## Valid Usage
/// - `fence` **must**  not be associated with any queue command that has not yet completed
///   execution on that queue
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_import_fence_fd_info`] **must**  be a valid pointer to a valid [`ImportFenceFdInfoKHR`]
///   structure
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
/// # Related
/// - [`khr_external_fence_fd`]
/// - [`Device`]
/// - [`ImportFenceFdInfoKHR`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkImportFenceFdKHR")]
pub type FNImportFenceFdKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_import_fence_fd_info: *const ImportFenceFdInfoKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[VkImportFenceFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportFenceFdInfoKHR.html) - (None)
///# C Specifications
///The [`ImportFenceFdInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_fence_fd
///typedef struct VkImportFenceFdInfoKHR {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkFence                              fence;
///    VkFenceImportFlags                   flags;
///    VkExternalFenceHandleTypeFlagBits    handleType;
///    int                                  fd;
///} VkImportFenceFdInfoKHR;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fence`] is the fence into which the payload will be imported.
/// - [`flags`] is a bitmask of [`FenceImportFlagBits`] specifying additional parameters for the
///   fence payload import operation.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of [`fd`].
/// - [`fd`] is the external handle to import.
/// # Description
/// The handle types supported by [`handle_type`] are:
/// ## Valid Usage
/// - [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportFenceFdInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fence-handletypes-fd)
///   table
/// -  [`fd`] **must**  obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
/// If [`handle_type`] is `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT`, the
/// special value `-1` for [`fd`] is treated like a valid sync file descriptor
/// referring to an object that has already signaled.
/// The import operation will succeed and the [`Fence`] will have a
/// temporarily imported payload as if a valid file descriptor had been
/// provided.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`fence`] **must**  be a valid [`Fence`] handle
/// - [`flags`] **must**  be a valid combination of [`FenceImportFlagBits`] values
/// - [`handle_type`] **must**  be a valid [`ExternalFenceHandleTypeFlagBits`] value
///
/// ## Host Synchronization
/// - Host access to [`fence`] **must**  be externally synchronized
/// # Related
/// - [`khr_external_fence_fd`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`FenceImportFlags`]
/// - [`StructureType`]
/// - [`import_fence_fd_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportFenceFdInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImportFenceFdInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`fence`] is the fence into which the payload will be imported.
    pub fence: Fence,
    ///[`flags`] is a bitmask of [`FenceImportFlagBits`] specifying
    ///additional parameters for the fence payload import operation.
    pub flags: FenceImportFlags,
    ///[`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value
    ///specifying the type of [`fd`].
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    ///[`fd`] is the external handle to import.
    pub fd: i32,
}
impl<'lt> Default for ImportFenceFdInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_FENCE_FD_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            fd: 0,
        }
    }
}
impl<'lt> ImportFenceFdInfoKHR<'lt> {
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
    ///Gets the value of [`Self::fd`]
    pub fn fd(&self) -> i32 {
        self.fd
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
    ///Sets the value of [`Self::fence`]
    pub fn set_fence(&mut self, value: crate::vulkan1_0::Fence) -> &mut Self {
        self.fence = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::vulkan1_1::FenceImportFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlagBits) -> &mut Self {
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
    ///Sets the value of [`Self::fence`]
    pub fn with_fence(mut self, value: crate::vulkan1_0::Fence) -> Self {
        self.fence = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::vulkan1_1::FenceImportFlags) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn with_handle_type(mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::fd`]
    pub fn with_fd(mut self, value: i32) -> Self {
        self.fd = value;
        self
    }
}
///[VkFenceGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceGetFdInfoKHR.html) - Structure describing a POSIX FD fence export operation
///# C Specifications
///The [`FenceGetFdInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_fence_fd
///typedef struct VkFenceGetFdInfoKHR {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkFence                              fence;
///    VkExternalFenceHandleTypeFlagBits    handleType;
///} VkFenceGetFdInfoKHR;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fence`] is the fence from which state will be exported.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of handle
///   requested.
/// # Description
/// The properties of the file descriptor returned depend on the value of
/// [`handle_type`].
/// See [`ExternalFenceHandleTypeFlagBits`] for a description of the
/// properties of the defined external fence handle types.
/// ## Valid Usage
/// - [`handle_type`] **must**  have been included in [`ExportFenceCreateInfo::handle_types`] when
///   [`fence`]’s current payload was created
/// - If [`handle_type`] refers to a handle type with copy payload transference semantics, [`fence`]
///   **must**  be signaled, or have an associated [fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling)
///   pending execution
/// -  [`fence`] **must**  not currently have its payload replaced by an imported payload as described below in [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing) unless that imported payload’s handle type was included in [`ExternalFenceProperties::export_from_imported_handle_types`] for [`handle_type`]
/// - [`handle_type`] **must**  be defined as a POSIX file descriptor handle
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`fence`] **must**  be a valid [`Fence`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalFenceHandleTypeFlagBits`] value
/// # Related
/// - [`khr_external_fence_fd`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`StructureType`]
/// - [`get_fence_fd_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFenceGetFdInfoKHR")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct FenceGetFdInfoKHR<'lt> {
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
impl<'lt> Default for FenceGetFdInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::FENCE_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            fence: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> FenceGetFdInfoKHR<'lt> {
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
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::fence`]
    pub fn set_fence(&mut self, value: crate::vulkan1_0::Fence) -> &mut Self {
        self.fence = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlagBits) -> &mut Self {
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
    ///Sets the value of [`Self::fence`]
    pub fn with_fence(mut self, value: crate::vulkan1_0::Fence) -> Self {
        self.fence = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn with_handle_type(mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
}
impl Device {
    ///[vkGetFenceFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html) - Get a POSIX file descriptor handle for a fence
    ///# C Specifications
    ///To export a POSIX file descriptor representing the payload of a fence, call:
    ///```c
    ///// Provided by VK_KHR_external_fence_fd
    ///VkResult vkGetFenceFdKHR(
    ///    VkDevice                                    device,
    ///    const VkFenceGetFdInfoKHR*                  pGetFdInfo,
    ///    int*                                        pFd);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that created the fence being exported.
    /// - [`p_get_fd_info`] is a pointer to a [`FenceGetFdInfoKHR`] structure containing parameters
    ///   of the export operation.
    /// - [`p_fd`] will return the file descriptor representing the fence payload.
    /// # Description
    /// Each call to [`get_fence_fd_khr`] **must**  create a new file descriptor and
    /// transfer ownership of it to the application.
    /// To avoid leaking resources, the application  **must**  release ownership of the
    /// file descriptor when it is no longer needed.If `pGetFdInfo->handleType` is
    /// `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT` and the fence is signaled at
    /// the time [`get_fence_fd_khr`] is called, [`p_fd`] **may**  return the value
    /// `-1` instead of a valid file descriptor.Where supported by the operating system, the
    /// implementation  **must**  set the
    /// file descriptor to be closed automatically when an `execve` system call
    /// is made.Exporting a file descriptor from a fence  **may**  have side effects depending on
    /// the transference of the specified handle type, as described in
    /// [Importing Fence State](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing).
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_get_fd_info`] **must**  be a valid pointer to a valid [`FenceGetFdInfoKHR`] structure
    /// - [`p_fd`] **must**  be a valid pointer to an `int` value
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`khr_external_fence_fd`]
    /// - [`Device`]
    /// - [`FenceGetFdInfoKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetFenceFdKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_fence_fd_khr<'lt>(
        self: &Unique<Device>,
        p_get_fd_info: &FenceGetFdInfoKHR<'lt>,
    ) -> VulkanResult<i32> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_fence_fd()
            .and_then(|vtable| vtable.get_fence_fd_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_fence_fd()
            .and_then(|vtable| vtable.get_fence_fd_khr())
            .unwrap_unchecked();
        let mut p_fd = Default::default();
        let _return = _function(self.as_raw(), p_get_fd_info as *const FenceGetFdInfoKHR<'lt>, &mut p_fd);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_fd),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkImportFenceFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html) - Import a fence from a POSIX file descriptor
    ///# C Specifications
    ///To import a fence payload from a POSIX file descriptor, call:
    ///```c
    ///// Provided by VK_KHR_external_fence_fd
    ///VkResult vkImportFenceFdKHR(
    ///    VkDevice                                    device,
    ///    const VkImportFenceFdInfoKHR*               pImportFenceFdInfo);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that created the fence.
    /// - [`p_import_fence_fd_info`] is a pointer to a [`ImportFenceFdInfoKHR`] structure specifying
    ///   the fence and import parameters.
    /// # Description
    /// Importing a fence payload from a file descriptor transfers ownership of the
    /// file descriptor from the application to the Vulkan implementation.
    /// The application  **must**  not perform any operations on the file descriptor
    /// after a successful import.Applications  **can**  import the same fence payload into multiple
    /// instances of
    /// Vulkan, into the same instance from which it was exported, and multiple
    /// times into a given Vulkan instance.
    /// ## Valid Usage
    /// - `fence` **must**  not be associated with any queue command that has not yet completed
    ///   execution on that queue
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_import_fence_fd_info`] **must**  be a valid pointer to a valid
    ///   [`ImportFenceFdInfoKHR`] structure
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    /// # Related
    /// - [`khr_external_fence_fd`]
    /// - [`Device`]
    /// - [`ImportFenceFdInfoKHR`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkImportFenceFdKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn import_fence_fd_khr<'lt>(
        self: &Unique<Device>,
        p_import_fence_fd_info: &ImportFenceFdInfoKHR<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_fence_fd()
            .and_then(|vtable| vtable.import_fence_fd_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_fence_fd()
            .and_then(|vtable| vtable.import_fence_fd_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_import_fence_fd_info as *const ImportFenceFdInfoKHR<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_external_fence_fd`
pub struct DeviceKhrExternalFenceFdVTable {
    ///See [`FNGetFenceFdKhr`] for more information.
    pub get_fence_fd_khr: FNGetFenceFdKhr,
    ///See [`FNImportFenceFdKhr`] for more information.
    pub import_fence_fd_khr: FNImportFenceFdKhr,
}
impl DeviceKhrExternalFenceFdVTable {
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
            get_fence_fd_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetFenceFdKHR").as_ptr()))
            },
            import_fence_fd_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkImportFenceFdKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_fence_fd_khr`]. See [`FNGetFenceFdKhr`] for more information.
    pub fn get_fence_fd_khr(&self) -> FNGetFenceFdKhr {
        self.get_fence_fd_khr
    }
    ///Gets [`Self::import_fence_fd_khr`]. See [`FNImportFenceFdKhr`] for more information.
    pub fn import_fence_fd_khr(&self) -> FNImportFenceFdKhr {
        self.import_fence_fd_khr
    }
}
