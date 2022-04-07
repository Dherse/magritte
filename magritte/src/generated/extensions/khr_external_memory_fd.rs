//![VK_KHR_external_memory_fd](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_fd.html) - device extension
//!# Description
//!An application may wish to reference device memory in multiple Vulkan
//!logical devices or instances, in multiple processes, and/or in multiple
//!APIs.
//!This extension enables an application to export POSIX file descriptor
//!handles from Vulkan memory objects and to import Vulkan memory objects from
//!POSIX file descriptor handles exported from other Vulkan memory objects or
//!from similar resources in other APIs.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_memory`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_memory_fd]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_memory_fd extension>>)
//!# New functions & commands
//! - [`get_memory_fd_khr`]
//! - [`get_memory_fd_properties_khr`]
//!# New structures
//! - [`MemoryFdPropertiesKHR`]
//! - [`MemoryGetFdInfoKHR`]
//! - Extending [`MemoryAllocateInfo`]:  - [`ImportMemoryFdInfoKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR`  - `VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Does the application need to close the file descriptor returned by
//![`get_memory_fd_khr`]? **RESOLVED** : Yes, unless it is passed back in to a driver instance to
//! import
//!the memory.
//!A successful get call transfers ownership of the file descriptor to the
//!application, and a successful import transfers it back to the driver.
//!Destroying the original memory object will not close the file descriptor or
//!remove its reference to the underlying memory resource associated with it.2) Do drivers ever
//! need to expose multiple file descriptors per memory
//!object? **RESOLVED** : No.
//!This would indicate there are actually multiple memory objects, rather than
//!a single memory object.3) How should the valid size and memory type for POSIX file descriptor
//!memory handles created outside of Vulkan be specified? **RESOLVED** : The valid memory types are
//! queried directly from the external
//!handle.
//!The size will be specified by future extensions that introduce such external
//!memory handle types.
//!# Version History
//! - Revision 1, 2016-10-21 (James Jones)  - Initial revision
//!# Other info
//! * 2016-10-21
//! * No known IP claims.
//! * - James Jones, NVIDIA  - Jeff Juliano, NVIDIA
//!# Related
//! - [`ImportMemoryFdInfoKHR`]
//! - [`MemoryFdPropertiesKHR`]
//! - [`MemoryGetFdInfoKHR`]
//! - [`get_memory_fd_khr`]
//! - [`get_memory_fd_properties_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_memory_fd");
///[vkGetMemoryFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html) - Get a POSIX file descriptor for a memory object
///# C Specifications
///To export a POSIX file descriptor referencing the payload of a Vulkan device
///memory object, call:
///```c
///// Provided by VK_KHR_external_memory_fd
///VkResult vkGetMemoryFdKHR(
///    VkDevice                                    device,
///    const VkMemoryGetFdInfoKHR*                 pGetFdInfo,
///    int*                                        pFd);
///```
///# Parameters
/// - [`device`] is the logical device that created the device memory being exported.
/// - [`p_get_fd_info`] is a pointer to a [`MemoryGetFdInfoKHR`] structure containing parameters of
///   the export operation.
/// - [`p_fd`] will return a file descriptor referencing the payload of the device memory object.
///# Description
///Each call to [`get_memory_fd_khr`] **must**  create a new file descriptor
///holding a reference to the memory object’s payload and transfer ownership of
///the file descriptor to the application.
///To avoid leaking resources, the application  **must**  release ownership of the
///file descriptor using the `close` system call when it is no longer
///needed, or by importing a Vulkan memory object from it.
///Where supported by the operating system, the implementation  **must**  set the
///file descriptor to be closed automatically when an `execve` system call
///is made.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_get_fd_info`] **must**  be a valid pointer to a valid [`MemoryGetFdInfoKHR`] structure
/// - [`p_fd`] **must**  be a valid pointer to an `int` value
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
///# Related
/// - [`VK_KHR_external_memory_fd`]
/// - [`Device`]
/// - [`MemoryGetFdInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetMemoryFdKHR")]
pub type FNGetMemoryFdKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const MemoryGetFdInfoKHR<'lt>,
        p_fd: *mut i32,
    ) -> VulkanResultCodes,
>;
///[vkGetMemoryFdPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) - Get Properties of External Memory File Descriptors
///# C Specifications
///POSIX file descriptor memory handles compatible with Vulkan  **may**  also be
///created by non-Vulkan APIs using methods beyond the scope of this
///specification.
///To determine the correct parameters to use when importing such handles,
///call:
///```c
///// Provided by VK_KHR_external_memory_fd
///VkResult vkGetMemoryFdPropertiesKHR(
///    VkDevice                                    device,
///    VkExternalMemoryHandleTypeFlagBits          handleType,
///    int                                         fd,
///    VkMemoryFdPropertiesKHR*                    pMemoryFdProperties);
///```
///# Parameters
/// - [`device`] is the logical device that will be importing [`fd`].
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of the
///   handle [`fd`].
/// - [`fd`] is the handle which will be imported.
/// - [`p_memory_fd_properties`] is a pointer to a [`MemoryFdPropertiesKHR`] structure in which the
///   properties of the handle [`fd`] are returned.
///# Description
///## Valid Usage
/// - [`fd`] **must**  be an external memory handle created outside of the Vulkan API
/// - [`handle_type`] **must**  not be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
/// - [`p_memory_fd_properties`] **must**  be a valid pointer to a [`MemoryFdPropertiesKHR`]
///   structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///# Related
/// - [`VK_KHR_external_memory_fd`]
/// - [`Device`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`MemoryFdPropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetMemoryFdPropertiesKHR")]
pub type FNGetMemoryFdPropertiesKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: i32,
        p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'lt>,
    ) -> VulkanResultCodes,
>;
///[VkImportMemoryFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryFdInfoKHR.html) - Import memory created on the same physical device from a file descriptor
///# C Specifications
///To import memory from a POSIX file descriptor handle, add a
///[`ImportMemoryFdInfoKHR`] structure to the [`p_next`] chain of the
///[`MemoryAllocateInfo`] structure.
///The [`ImportMemoryFdInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_memory_fd
///typedef struct VkImportMemoryFdInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///    int                                   fd;
///} VkImportMemoryFdInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the handle type of
///   [`fd`].
/// - [`fd`] is the external handle to import.
///# Description
///Importing memory from a file descriptor transfers ownership of the file
///descriptor from the application to the Vulkan implementation.
///The application  **must**  not perform any operations on the file descriptor
///after a successful import.
///The imported memory object holds a reference to its payload.Applications  **can**  import the
/// same payload into multiple instances of Vulkan,
///into the same instance from which it was exported, and multiple times into a
///given Vulkan instance.
///In all cases, each import operation  **must**  create a distinct
///[`DeviceMemory`] object.
///## Valid Usage
/// - If [`handle_type`] is not `0`, it  **must**  be supported for import, as reported by
///   [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
/// - The memory from which [`fd`] was exported  **must**  have been created on the same underlying
///   physical device as `device`
/// - If [`handle_type`] is not `0`, it  **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`
///   or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT`
/// - If [`handle_type`] is not `0`, [`fd`] **must**  be a valid handle of the type specified by
///   [`handle_type`]
/// - The memory represented by [`fd`] **must**  have been created from a physical device and driver
///   that is compatible with `device` and [`handle_type`], as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
/// -  [`fd`] **must**  obey any requirements listed for [`handle_type`] in [external memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR`
/// - If [`handle_type`] is not `0`, [`handle_type`] **must**  be a valid
///   [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_memory_fd`]
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
#[doc(alias = "VkImportMemoryFdInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImportMemoryFdInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the handle type of [`fd`].
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    ///[`fd`] is the external handle to import.
    pub fd: i32,
}
impl<'lt> Default for ImportMemoryFdInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_MEMORY_FD_INFO_KHR,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            fd: 0,
        }
    }
}
impl<'lt> ImportMemoryFdInfoKHR<'lt> {
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
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
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
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::fd`]
    pub fn fd_mut(&mut self) -> &mut i32 {
        &mut self.fd
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
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::fd`]
    pub fn set_fd(mut self, value: i32) -> Self {
        self.fd = value;
        self
    }
}
///[VkMemoryFdPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryFdPropertiesKHR.html) - Properties of External Memory File Descriptors
///# C Specifications
///The [`MemoryFdPropertiesKHR`] structure returned is defined as:
///```c
///// Provided by VK_KHR_external_memory_fd
///typedef struct VkMemoryFdPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           memoryTypeBits;
///} VkMemoryFdPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the
///   specified file descriptor  **can**  be imported as.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_external_memory_fd`]
/// - [`StructureType`]
/// - [`get_memory_fd_properties_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryFdPropertiesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct MemoryFdPropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the specified file descriptor  **can**  be imported as.
    pub memory_type_bits: u32,
}
impl<'lt> Default for MemoryFdPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MEMORY_FD_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            memory_type_bits: 0,
        }
    }
}
impl<'lt> MemoryFdPropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
        &mut self.memory_type_bits
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::memory_type_bits`]
    pub fn set_memory_type_bits(mut self, value: u32) -> Self {
        self.memory_type_bits = value;
        self
    }
}
///[VkMemoryGetFdInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetFdInfoKHR.html) - Structure describing a POSIX FD semaphore export operation
///# C Specifications
///The [`MemoryGetFdInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_memory_fd
///typedef struct VkMemoryGetFdInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkDeviceMemory                        memory;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///} VkMemoryGetFdInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory`] is the memory object from which the handle will be exported.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of handle
///   requested.
///# Description
///The properties of the file descriptor exported depend on the value of
///[`handle_type`].
///See [`ExternalMemoryHandleTypeFlagBits`] for a description of the
///properties of the defined external memory handle types.
///## Valid Usage
/// - [`handle_type`] **must**  have been included in [`ExportMemoryAllocateInfo::handle_types`]
///   when [`memory`] was created
/// - [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT` or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_memory_fd`]
/// - [`DeviceMemory`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`get_memory_fd_khr`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryGetFdInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct MemoryGetFdInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`memory`] is the memory object from which the handle will be
    ///exported.
    pub memory: DeviceMemory,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl<'lt> Default for MemoryGetFdInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MEMORY_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> MemoryGetFdInfoKHR<'lt> {
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
    ///Sets the value of [`Self::memory`]
    pub fn set_memory(mut self, value: crate::vulkan1_0::DeviceMemory) -> Self {
        self.memory = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
}
impl Device {
    ///[vkGetMemoryFdKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html) - Get a POSIX file descriptor for a memory object
    ///# C Specifications
    ///To export a POSIX file descriptor referencing the payload of a Vulkan device
    ///memory object, call:
    ///```c
    ///// Provided by VK_KHR_external_memory_fd
    ///VkResult vkGetMemoryFdKHR(
    ///    VkDevice                                    device,
    ///    const VkMemoryGetFdInfoKHR*                 pGetFdInfo,
    ///    int*                                        pFd);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that created the device memory being exported.
    /// - [`p_get_fd_info`] is a pointer to a [`MemoryGetFdInfoKHR`] structure containing parameters
    ///   of the export operation.
    /// - [`p_fd`] will return a file descriptor referencing the payload of the device memory
    ///   object.
    ///# Description
    ///Each call to [`get_memory_fd_khr`] **must**  create a new file descriptor
    ///holding a reference to the memory object’s payload and transfer ownership of
    ///the file descriptor to the application.
    ///To avoid leaking resources, the application  **must**  release ownership of the
    ///file descriptor using the `close` system call when it is no longer
    ///needed, or by importing a Vulkan memory object from it.
    ///Where supported by the operating system, the implementation  **must**  set the
    ///file descriptor to be closed automatically when an `execve` system call
    ///is made.
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_get_fd_info`] **must**  be a valid pointer to a valid [`MemoryGetFdInfoKHR`] structure
    /// - [`p_fd`] **must**  be a valid pointer to an `int` value
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///# Related
    /// - [`VK_KHR_external_memory_fd`]
    /// - [`Device`]
    /// - [`MemoryGetFdInfoKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetMemoryFdKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_memory_fd_khr<'lt>(
        self: &Unique<Device>,
        p_get_fd_info: &MemoryGetFdInfoKHR<'lt>,
    ) -> VulkanResult<i32> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_memory_fd()
            .and_then(|vtable| vtable.get_memory_fd_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_memory_fd()
            .and_then(|vtable| vtable.get_memory_fd_khr())
            .unwrap_unchecked();
        let mut p_fd = Default::default();
        let _return = _function(
            self.as_raw(),
            p_get_fd_info as *const MemoryGetFdInfoKHR<'lt>,
            &mut p_fd,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_fd),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetMemoryFdPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) - Get Properties of External Memory File Descriptors
    ///# C Specifications
    ///POSIX file descriptor memory handles compatible with Vulkan  **may**  also be
    ///created by non-Vulkan APIs using methods beyond the scope of this
    ///specification.
    ///To determine the correct parameters to use when importing such handles,
    ///call:
    ///```c
    ///// Provided by VK_KHR_external_memory_fd
    ///VkResult vkGetMemoryFdPropertiesKHR(
    ///    VkDevice                                    device,
    ///    VkExternalMemoryHandleTypeFlagBits          handleType,
    ///    int                                         fd,
    ///    VkMemoryFdPropertiesKHR*                    pMemoryFdProperties);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that will be importing [`fd`].
    /// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of the
    ///   handle [`fd`].
    /// - [`fd`] is the handle which will be imported.
    /// - [`p_memory_fd_properties`] is a pointer to a [`MemoryFdPropertiesKHR`] structure in which
    ///   the properties of the handle [`fd`] are returned.
    ///# Description
    ///## Valid Usage
    /// - [`fd`] **must**  be an external memory handle created outside of the Vulkan API
    /// - [`handle_type`] **must**  not be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
    /// - [`p_memory_fd_properties`] **must**  be a valid pointer to a [`MemoryFdPropertiesKHR`]
    ///   structure
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    ///# Related
    /// - [`VK_KHR_external_memory_fd`]
    /// - [`Device`]
    /// - [`ExternalMemoryHandleTypeFlagBits`]
    /// - [`MemoryFdPropertiesKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetMemoryFdPropertiesKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_memory_fd_properties_khr<'lt>(
        self: &Unique<Device>,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: Option<i32>,
        p_memory_fd_properties: Option<MemoryFdPropertiesKHR<'lt>>,
    ) -> VulkanResult<MemoryFdPropertiesKHR<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_external_memory_fd()
            .and_then(|vtable| vtable.get_memory_fd_properties_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_external_memory_fd()
            .and_then(|vtable| vtable.get_memory_fd_properties_khr())
            .unwrap_unchecked();
        let mut p_memory_fd_properties = p_memory_fd_properties.unwrap_or_default();
        let _return = _function(
            self.as_raw(),
            handle_type,
            fd.unwrap_or_default() as _,
            &mut p_memory_fd_properties,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, {
                p_memory_fd_properties.p_next = std::ptr::null_mut();
                p_memory_fd_properties
            }),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_external_memory_fd`
pub struct DeviceKhrExternalMemoryFdVTable {
    ///See [`FNGetMemoryFdKhr`] for more information.
    pub get_memory_fd_khr: FNGetMemoryFdKhr,
    ///See [`FNGetMemoryFdPropertiesKhr`] for more information.
    pub get_memory_fd_properties_khr: FNGetMemoryFdPropertiesKhr,
}
impl DeviceKhrExternalMemoryFdVTable {
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
            get_memory_fd_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetMemoryFdKHR").as_ptr()))
            },
            get_memory_fd_properties_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetMemoryFdPropertiesKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_memory_fd_khr`]. See [`FNGetMemoryFdKhr`] for more information.
    pub fn get_memory_fd_khr(&self) -> FNGetMemoryFdKhr {
        self.get_memory_fd_khr
    }
    ///Gets [`Self::get_memory_fd_properties_khr`]. See [`FNGetMemoryFdPropertiesKhr`] for more
    /// information.
    pub fn get_memory_fd_properties_khr(&self) -> FNGetMemoryFdPropertiesKhr {
        self.get_memory_fd_properties_khr
    }
}
