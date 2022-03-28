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
//! - Requires `[`VK_KHR_external_fence`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_fence_fd]
//!   @critsec%0A<<Here describe the issue or question you have about the VK_KHR_external_fence_fd
//!   extension>>)
//!# New functions & commands
//! - [`GetFenceFdKHR`]
//! - [`ImportFenceFdKHR`]
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
//!`[`VK_KHR_external_semaphore_fd`]`.
//!That extension’s issues apply equally to this extension.
//!# Version History
//! - Revision 1, 2017-05-08 (Jesse Hall)  - Initial revision
//!# Other info
//! * 2017-05-08
//! * No known IP claims.
//! * - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Cass Everitt, Oculus  -
//!   Contributors to `[`VK_KHR_external_semaphore_fd`]`
//!# Related
//! - [`FenceGetFdInfoKHR`]
//! - [`ImportFenceFdInfoKHR`]
//! - [`GetFenceFdKHR`]
//! - [`ImportFenceFdKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, Fence, StructureType},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fence`] is the fence into which the payload will be imported.
/// - [`flags`] is a bitmask of [`FenceImportFlagBits`] specifying additional parameters for the
///   fence payload import operation.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of [`fd`].
/// - [`fd`] is the external handle to import.
///# Description
///The handle types supported by [`handle_type`] are:
///## Valid Usage
/// - [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportFenceFdInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fence-handletypes-fd)
///   table
/// -  [`fd`] **must**  obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
///If [`handle_type`] is `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT`, the
///special value `-1` for [`fd`] is treated like a valid sync file descriptor
///referring to an object that has already signaled.
///The import operation will succeed and the [`Fence`] will have a
///temporarily imported payload as if a valid file descriptor had been
///provided.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`fence`] **must**  be a valid [`Fence`] handle
/// - [`flags`] **must**  be a valid combination of [`FenceImportFlagBits`] values
/// - [`handle_type`] **must**  be a valid [`ExternalFenceHandleTypeFlagBits`] value
///
///## Host Synchronization
/// - Host access to [`fence`] **must**  be externally synchronized
///# Related
/// - [`VK_KHR_external_fence_fd`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`FenceImportFlags`]
/// - [`StructureType`]
/// - [`ImportFenceFdKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportFenceFdInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImportFenceFdInfoKHR<'lt> {
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
            s_type: Default::default(),
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
    ///Sets the raw value of [`Self::fence`]
    pub fn set_fence(&mut self, value: crate::vulkan1_0::Fence) -> &mut Self {
        self.fence = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::vulkan1_1::FenceImportFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
    ///Sets the raw value of [`Self::fd`]
    pub fn set_fd(&mut self, value: i32) -> &mut Self {
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fence`] is the fence from which state will be exported.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of handle
///   requested.
///# Description
///The properties of the file descriptor returned depend on the value of
///[`handle_type`].
///See [`ExternalFenceHandleTypeFlagBits`] for a description of the
///properties of the defined external fence handle types.
///## Valid Usage
/// - [`handle_type`] **must**  have been included in [`ExportFenceCreateInfo::handle_types`] when
///   [`fence`]’s current payload was created
/// - If [`handle_type`] refers to a handle type with copy payload transference semantics, [`fence`]
///   **must**  be signaled, or have an associated [fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling)
///   pending execution
/// -  [`fence`] **must**  not currently have its payload replaced by an imported payload as described below in [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing) unless that imported payload’s handle type was included in [`ExternalFenceProperties::export_from_imported_handle_types`] for [`handle_type`]
/// - [`handle_type`] **must**  be defined as a POSIX file descriptor handle
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`fence`] **must**  be a valid [`Fence`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalFenceHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_fence_fd`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`StructureType`]
/// - [`GetFenceFdKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFenceGetFdInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FenceGetFdInfoKHR<'lt> {
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
            s_type: Default::default(),
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
    ///Sets the raw value of [`Self::fence`]
    pub fn set_fence(&mut self, value: crate::vulkan1_0::Fence) -> &mut Self {
        self.fence = value;
        self
    }
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
}
