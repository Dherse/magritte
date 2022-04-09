//![VK_EXT_external_memory_host](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_external_memory_host.html) - device extension
//!# Description
//!This extension enables an application to import host allocations and host
//!mapped foreign device memory to Vulkan memory objects.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_memory`]`
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_external_memory_host]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the
//!   VK_EXT_external_memory_host extension>>)
//!# New functions & commands
//! - [`get_memory_host_pointer_properties_ext`]
//!# New structures
//! - [`MemoryHostPointerPropertiesEXT`]
//! - Extending [`MemoryAllocateInfo`]:  - [`ImportMemoryHostPointerInfoEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceExternalMemoryHostPropertiesEXT`]
//!# New constants
//! - [`EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME`]
//! - [`EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION`]
//! - Extending [`ExternalMemoryHandleTypeFlagBits`]:  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT`
//!# Known issues & F.A.Q
//!1) What memory type has to be used to import host pointers? **RESOLVED** : Depends on the
//! implementation.
//!Applications have to use the new [`get_memory_host_pointer_properties_ext`]
//!command to query the supported memory types for a particular host pointer.
//!The reported memory types may include memory types that come from a memory
//!heap that is otherwise not usable for regular memory object allocation and
//!thus such a heap’s size may be zero.2) Can the application still access the contents of the host
//! allocation
//!after importing? **RESOLVED** : Yes.
//!However, usual synchronization requirements apply.3) Can the application free the host
//! allocation? **RESOLVED** : No, it violates valid usage conditions.
//!Using the memory object imported from a host allocation that is already
//!freed thus results in undefined behavior.4) Is [`map_memory`] expected to return the same host
//! address which was
//!specified when importing it to the memory object? **RESOLVED** : No.
//!Implementations are allowed to return the same address but it is not
//!required.
//!Some implementations might return a different virtual mapping of the
//!allocation, although the same physical pages will be used.5) Is there any limitation on the
//! alignment of the host pointer and/or size? **RESOLVED** : Yes.
//!Both the address and the size have to be an integer multiple of
//!`minImportedHostPointerAlignment`.
//!In addition, some platforms and foreign devices may have additional
//!restrictions.6) Can the same host allocation be imported multiple times into a given
//!physical device? **RESOLVED** : No, at least not guaranteed by this extension.
//!Some platforms do not allow locking the same physical pages for device
//!access multiple times, so attempting to do it may result in undefined
//!behavior.7) Does this extension support exporting the new handle type? **RESOLVED** : No.8)
//! Should we include the possibility to import host mapped foreign device
//!memory using this API? **RESOLVED** : Yes, through a separate handle type.
//!Implementations are still allowed to support only one of the handle types
//!introduced by this extension by not returning import support for a
//!particular handle type as returned in [`ExternalMemoryPropertiesKHR`].
//!# Version History
//! - Revision 1, 2017-11-10 (Daniel Rakos)  - Internal revisions
//!# Other info
//! * 2017-11-10
//! * No known IP claims.
//! * - Jaakko Konttinen, AMD  - David Mao, AMD  - Daniel Rakos, AMD  - Tobias Hector, Imagination
//!   Technologies  - Jason Ekstrand, Intel  - James Jones, NVIDIA
//!# Related
//! - [`ImportMemoryHostPointerInfoEXT`]
//! - [`MemoryHostPointerPropertiesEXT`]
//! - [`PhysicalDeviceExternalMemoryHostPropertiesEXT`]
//! - [`get_memory_host_pointer_properties_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceSize, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
    AsRaw, Unique, VulkanResult,
};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION")]
pub const EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME")]
pub const EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_external_memory_host");
///[vkGetMemoryHostPointerPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) - Get properties of external memory host pointer
///# C Specifications
///To determine the correct parameters to use when importing host pointers,
///call:
///```c
///// Provided by VK_EXT_external_memory_host
///VkResult vkGetMemoryHostPointerPropertiesEXT(
///    VkDevice                                    device,
///    VkExternalMemoryHandleTypeFlagBits          handleType,
///    const void*                                 pHostPointer,
///    VkMemoryHostPointerPropertiesEXT*           pMemoryHostPointerProperties);
///```
/// # Parameters
/// - [`device`] is the logical device that will be importing [`p_host_pointer`].
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of the
///   handle [`p_host_pointer`].
/// - [`p_host_pointer`] is the host pointer to import from.
/// - [`p_memory_host_pointer_properties`] is a pointer to a [`MemoryHostPointerPropertiesEXT`]
///   structure in which the host pointer properties are returned.
/// # Description
/// ## Valid Usage
/// - [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT` or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`
/// - [`p_host_pointer`] **must**  be a pointer aligned to an integer multiple of
///   [`PhysicalDeviceExternalMemoryHostPropertiesEXT::min_imported_host_pointer_alignment`]
/// - If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`,
///   [`p_host_pointer`] **must**  be a pointer to host memory
/// - If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`,
///   [`p_host_pointer`] **must**  be a pointer to host mapped foreign memory
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
/// - [`p_memory_host_pointer_properties`] **must**  be a valid pointer to a
///   [`MemoryHostPointerPropertiesEXT`] structure
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
/// # Related
/// - [`VK_EXT_external_memory_host`]
/// - [`Device`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`MemoryHostPointerPropertiesEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
pub type FNGetMemoryHostPointerPropertiesExt = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<'lt>,
    ) -> VulkanResultCodes,
>;
///[VkImportMemoryHostPointerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryHostPointerInfoEXT.html) - Import memory from a host pointer
///# C Specifications
///To import memory from a host pointer, add a
///[`ImportMemoryHostPointerInfoEXT`] structure to the [`p_next`] chain of
///the [`MemoryAllocateInfo`] structure.
///The [`ImportMemoryHostPointerInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_external_memory_host
///typedef struct VkImportMemoryHostPointerInfoEXT {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///    void*                                 pHostPointer;
///} VkImportMemoryHostPointerInfoEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the handle type.
/// - [`host_pointer`] is the host pointer to import from.
/// # Description
/// Importing memory from a host pointer shares ownership of the memory between
/// the host and the Vulkan implementation.
/// The application  **can**  continue to access the memory through the host pointer
/// but it is the application’s responsibility to synchronize device and
/// non-device access to the payload as defined in
/// [Host Access to Device Memory Objects](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device-hostaccess).Applications  **can**  import the same payload into multiple instances of Vulkan
/// and multiple times into a given Vulkan instance.
/// However, implementations  **may**  fail to import the same payload multiple times
/// into a given physical device due to platform constraints.Importing memory from a particular host
/// pointer  **may**  not be possible due to
/// additional platform-specific restrictions beyond the scope of this
/// specification in which case the implementation  **must**  fail the memory import
/// operation with the error code `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`.Whether device memory
/// objects imported from a host pointer hold a reference
/// to their payload is undefined.
/// As such, the application  **must**  ensure that the imported memory range remains
/// valid and accessible for the lifetime of the imported memory object.
/// ## Valid Usage
/// - If [`handle_type`] is not `0`, it  **must**  be supported for import, as reported in
///   [`ExternalMemoryProperties`]
/// - If [`handle_type`] is not `0`, it  **must**  be
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT` or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`
/// - [`host_pointer`] **must**  be a pointer aligned to an integer multiple of
///   [`PhysicalDeviceExternalMemoryHostPropertiesEXT::min_imported_host_pointer_alignment`]
/// - If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`,
///   [`host_pointer`] **must**  be a pointer to `allocationSize` number of bytes of host memory,
///   where `allocationSize` is the member of the [`MemoryAllocateInfo`] structure this structure is
///   chained to
/// - If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`,
///   [`host_pointer`] **must**  be a pointer to `allocationSize` number of bytes of host mapped
///   foreign memory, where `allocationSize` is the member of the [`MemoryAllocateInfo`] structure
///   this structure is chained to
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT`
/// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
/// # Related
/// - [`VK_EXT_external_memory_host`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportMemoryHostPointerInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the handle type.
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    ///[`host_pointer`] is the host pointer to import from.
    pub host_pointer: *mut c_void,
}
impl<'lt> Default for ImportMemoryHostPointerInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            host_pointer: std::ptr::null_mut(),
        }
    }
}
impl<'lt> ImportMemoryHostPointerInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::host_pointer`]
    pub fn host_pointer_raw(&self) -> *mut c_void {
        self.host_pointer
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::host_pointer`]
    pub fn set_host_pointer_raw(mut self, value: *mut c_void) -> Self {
        self.host_pointer = value;
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
    ///Gets the value of [`Self::host_pointer`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn host_pointer(&self) -> &c_void {
        &*self.host_pointer
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::host_pointer`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn host_pointer_mut(&mut self) -> &mut c_void {
        &mut *self.host_pointer
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
    ///Sets the value of [`Self::host_pointer`]
    pub fn set_host_pointer(mut self, value: &'lt mut std::ffi::c_void) -> Self {
        self.host_pointer = value as *mut _;
        self
    }
}
///[VkMemoryHostPointerPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryHostPointerPropertiesEXT.html) - Properties of external memory host pointer
///# C Specifications
///The [`MemoryHostPointerPropertiesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_external_memory_host
///typedef struct VkMemoryHostPointerPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           memoryTypeBits;
///} VkMemoryHostPointerPropertiesEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the
///   specified host pointer  **can**  be imported as.
/// # Description
/// The value returned by [`memory_type_bits`] **must**  only include bits that
/// identify memory types which are host visible.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT`
/// - [`p_next`] **must**  be `NULL`
/// # Related
/// - [`VK_EXT_external_memory_host`]
/// - [`StructureType`]
/// - [`get_memory_host_pointer_properties_ext`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryHostPointerPropertiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the specified host pointer  **can**  be imported as.
    pub memory_type_bits: u32,
}
impl<'lt> Default for MemoryHostPointerPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            memory_type_bits: 0,
        }
    }
}
impl<'lt> MemoryHostPointerPropertiesEXT<'lt> {
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
///[VkPhysicalDeviceExternalMemoryHostPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalMemoryHostPropertiesEXT.html) - Structure describing external memory host pointer limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceExternalMemoryHostPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_external_memory_host
///typedef struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkDeviceSize       minImportedHostPointerAlignment;
///} VkPhysicalDeviceExternalMemoryHostPropertiesEXT;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_imported_host_pointer_alignment`] is the minimum  **required**  alignment, in bytes, for
///   the base address and size of host pointers that  **can**  be imported to a Vulkan memory
///   object. The value  **must**  be a power of two.
/// # Description
/// If the [`PhysicalDeviceExternalMemoryHostPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
/// [`PhysicalDeviceProperties2`] structure passed to
/// [`get_physical_device_properties2`], it is filled in with each
/// corresponding implementation-dependent property.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT`
/// # Related
/// - [`VK_EXT_external_memory_host`]
/// - [`DeviceSize`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceExternalMemoryHostPropertiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`min_imported_host_pointer_alignment`] is the minimum  **required**
    ///alignment, in bytes, for the base address and size of host pointers that
    /// **can**  be imported to a Vulkan memory object.
    ///The value  **must**  be a power of two.
    pub min_imported_host_pointer_alignment: DeviceSize,
}
impl<'lt> Default for PhysicalDeviceExternalMemoryHostPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            min_imported_host_pointer_alignment: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceExternalMemoryHostPropertiesEXT<'lt> {
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
    ///Gets the value of [`Self::min_imported_host_pointer_alignment`]
    pub fn min_imported_host_pointer_alignment(&self) -> DeviceSize {
        self.min_imported_host_pointer_alignment
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
    ///Gets a mutable reference to the value of [`Self::min_imported_host_pointer_alignment`]
    pub fn min_imported_host_pointer_alignment_mut(&mut self) -> &mut DeviceSize {
        &mut self.min_imported_host_pointer_alignment
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
    ///Sets the value of [`Self::min_imported_host_pointer_alignment`]
    pub fn set_min_imported_host_pointer_alignment(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.min_imported_host_pointer_alignment = value;
        self
    }
}
impl Device {
    ///[vkGetMemoryHostPointerPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryHostPointerPropertiesEXT.html) - Get properties of external memory host pointer
    ///# C Specifications
    ///To determine the correct parameters to use when importing host pointers,
    ///call:
    ///```c
    ///// Provided by VK_EXT_external_memory_host
    ///VkResult vkGetMemoryHostPointerPropertiesEXT(
    ///    VkDevice                                    device,
    ///    VkExternalMemoryHandleTypeFlagBits          handleType,
    ///    const void*                                 pHostPointer,
    ///    VkMemoryHostPointerPropertiesEXT*           pMemoryHostPointerProperties);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that will be importing [`p_host_pointer`].
    /// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of the
    ///   handle [`p_host_pointer`].
    /// - [`p_host_pointer`] is the host pointer to import from.
    /// - [`p_memory_host_pointer_properties`] is a pointer to a [`MemoryHostPointerPropertiesEXT`]
    ///   structure in which the host pointer properties are returned.
    /// # Description
    /// ## Valid Usage
    /// - [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT` or
    ///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`
    /// - [`p_host_pointer`] **must**  be a pointer aligned to an integer multiple of
    ///   [`PhysicalDeviceExternalMemoryHostPropertiesEXT::min_imported_host_pointer_alignment`]
    /// - If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`,
    ///   [`p_host_pointer`] **must**  be a pointer to host memory
    /// - If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`,
    ///   [`p_host_pointer`] **must**  be a pointer to host mapped foreign memory
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
    /// - [`p_memory_host_pointer_properties`] **must**  be a valid pointer to a
    ///   [`MemoryHostPointerPropertiesEXT`] structure
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    /// # Related
    /// - [`VK_EXT_external_memory_host`]
    /// - [`Device`]
    /// - [`ExternalMemoryHandleTypeFlagBits`]
    /// - [`MemoryHostPointerPropertiesEXT`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetMemoryHostPointerPropertiesEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_memory_host_pointer_properties_ext<'lt>(
        self: &Unique<Device>,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: Option<MemoryHostPointerPropertiesEXT<'lt>>,
    ) -> VulkanResult<MemoryHostPointerPropertiesEXT<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_external_memory_host()
            .and_then(|vtable| vtable.get_memory_host_pointer_properties_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_external_memory_host()
            .and_then(|vtable| vtable.get_memory_host_pointer_properties_ext())
            .unwrap_unchecked();
        let mut p_memory_host_pointer_properties = p_memory_host_pointer_properties.unwrap_or_default();
        let _return = _function(
            self.as_raw(),
            handle_type,
            p_host_pointer,
            &mut p_memory_host_pointer_properties,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, {
                p_memory_host_pointer_properties.p_next = std::ptr::null_mut();
                p_memory_host_pointer_properties
            }),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_external_memory_host`
pub struct DeviceExtExternalMemoryHostVTable {
    ///See [`FNGetMemoryHostPointerPropertiesExt`] for more information.
    pub get_memory_host_pointer_properties_ext: FNGetMemoryHostPointerPropertiesExt,
}
impl DeviceExtExternalMemoryHostVTable {
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
            get_memory_host_pointer_properties_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetMemoryHostPointerPropertiesEXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_memory_host_pointer_properties_ext`]. See
    /// [`FNGetMemoryHostPointerPropertiesExt`] for more information.
    pub fn get_memory_host_pointer_properties_ext(&self) -> FNGetMemoryHostPointerPropertiesExt {
        self.get_memory_host_pointer_properties_ext
    }
}
