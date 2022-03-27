use crate::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, DeviceSize, StructureType},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the handle type.
/// - [`p_host_pointer`] is the host pointer to import from.
///# Description
///Importing memory from a host pointer shares ownership of the memory between
///the host and the Vulkan implementation.
///The application **can** continue to access the memory through the host pointer
///but it is the application’s responsibility to synchronize device and
///non-device access to the payload as defined in
///[Host Access to Device Memory Objects](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device-hostaccess).Applications **can** import the same payload into multiple instances of Vulkan
///and multiple times into a given Vulkan instance.
///However, implementations **may** fail to import the same payload multiple times
///into a given physical device due to platform constraints.Importing memory from a particular host
/// pointer **may** not be possible due to
///additional platform-specific restrictions beyond the scope of this
///specification in which case the implementation **must** fail the memory import
///operation with the error code `VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR`.Whether device memory
/// objects imported from a host pointer hold a reference
///to their payload is undefined.
///As such, the application **must** ensure that the imported memory range remains
///valid and accessible for the lifetime of the imported memory object.Valid Usage
/// - If [`handle_type`] is not `0`, it **must** be supported for import, as reported in
///   [`ExternalMemoryProperties`]
/// - If [`handle_type`] is not `0`, it **must** be
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT` or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`
/// - [`p_host_pointer`]**must** be a pointer aligned to an integer multiple of
///   [`PhysicalDeviceExternalMemoryHostPropertiesEXT::min_imported_host_pointer_alignment`]
/// - If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`,
///   [`p_host_pointer`]**must** be a pointer to `allocationSize` number of bytes of host memory,
///   where `allocationSize` is the member of the [`MemoryAllocateInfo`] structure this structure is
///   chained to
/// - If [`handle_type`] is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`,
///   [`p_host_pointer`]**must** be a pointer to `allocationSize` number of bytes of host mapped
///   foreign memory, where `allocationSize` is the member of the [`MemoryAllocateInfo`] structure
///   this structure is chained to
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT`
/// - [`handle_type`]**must** be a valid [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_EXT_external_memory_host`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImportMemoryHostPointerInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the handle type.
    handle_type: ExternalMemoryHandleTypeFlagBits,
    ///[`p_host_pointer`] is the host pointer to import from.
    p_host_pointer: *const c_void,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the
///   specified host pointer **can** be imported as.
///# Description
///The value returned by [`memory_type_bits`]**must** only include bits that
///identify memory types which are host visible.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_EXT_external_memory_host`]
/// - [`StructureType`]
/// - [`GetMemoryHostPointerPropertiesEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MemoryHostPointerPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`memory_type_bits`] is a bitmask containing one bit set for every
    ///memory type which the specified host pointer **can** be imported as.
    memory_type_bits: u32,
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_imported_host_pointer_alignment`] is the minimum **required** alignment, in bytes, for
///   the base address and size of host pointers that **can** be imported to a Vulkan memory object.
///   The value **must** be a power of two.
///# Description
///If the [`PhysicalDeviceExternalMemoryHostPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_external_memory_host`]
/// - [`DeviceSize`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`min_imported_host_pointer_alignment`] is the minimum **required**
    ///alignment, in bytes, for the base address and size of host pointers that
    ///**can** be imported to a Vulkan memory object.
    ///The value **must** be a power of two.
    min_imported_host_pointer_alignment: DeviceSize,
}
