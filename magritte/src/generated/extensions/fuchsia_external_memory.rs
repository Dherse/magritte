use crate::{
    native::zx_handle_t,
    vulkan1_0::{BaseInStructure, BaseOutStructure, DeviceMemory, StructureType},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_FUCHSIA_external_memory");
///[VkImportMemoryZirconHandleInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryZirconHandleInfoFUCHSIA.html) - Structure specifying import parameters for Zircon handle to external memory
///# C Specifications
///The [`ImportMemoryZirconHandleInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_external_memory
///typedef struct VkImportMemoryZirconHandleInfoFUCHSIA {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///    zx_handle_t                           handle;
///} VkImportMemoryZirconHandleInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of
///   [`handle`].
/// - [`handle`] is a [`zx_handle_t`] (Zircon) handle to the external memory.
///# Description
///Valid Usage
/// - [`handle_type`]**must** be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
/// - [`handle`] must be a valid VMO handle
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA`
/// - If [`handle_type`] is not `0`, [`handle_type`]**must** be a valid
///   [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_FUCHSIA_external_memory`]
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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImportMemoryZirconHandleInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the type of [`handle`].
    handle_type: ExternalMemoryHandleTypeFlagBits,
    ///[`handle`] is a [`zx_handle_t`] (Zircon) handle to the external
    ///memory.
    handle: zx_handle_t,
}
///[VkMemoryZirconHandlePropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryZirconHandlePropertiesFUCHSIA.html) - Structure specifying Zircon handle compatible external memory
///# C Specifications
///The [`MemoryZirconHandlePropertiesFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_external_memory
///typedef struct VkMemoryZirconHandlePropertiesFUCHSIA {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           memoryTypeBits;
///} VkMemoryZirconHandlePropertiesFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_type_bits`] a bitmask containing one bit set for every memory type which the
///   specified handle can be imported as.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_FUCHSIA_external_memory`]
/// - [`StructureType`]
/// - [`GetMemoryZirconHandlePropertiesFUCHSIA`]
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
pub struct MemoryZirconHandlePropertiesFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`memory_type_bits`] a bitmask containing one bit set for every memory
    ///type which the specified handle can be imported as.
    memory_type_bits: u32,
}
///[VkMemoryGetZirconHandleInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryGetZirconHandleInfoFUCHSIA.html) - Structure specifying export parameters for Zircon handle to device memory
///# C Specifications
///[`MemoryGetZirconHandleInfoFUCHSIA`] is defined as:
///```c
///// Provided by VK_FUCHSIA_external_memory
///typedef struct VkMemoryGetZirconHandleInfoFUCHSIA {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkDeviceMemory                        memory;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///} VkMemoryGetZirconHandleInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory`] the [`DeviceMemory`] being exported.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of the
///   handle pointed to by [`GetMemoryZirconHandleFUCHSIA`]`::pZirconHandle`.
///# Description
///Valid Usage
/// - [`handle_type`]**must** be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
/// - [`handle_type`]**must** have been included in the `handleTypes` field of the
///   [`ExportMemoryAllocateInfo`] structure when the external memory was allocated
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
/// - [`handle_type`]**must** be a valid [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_FUCHSIA_external_memory`]
/// - [`DeviceMemory`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`GetMemoryZirconHandleFUCHSIA`]
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
pub struct MemoryGetZirconHandleInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`memory`] the [`DeviceMemory`] being exported.
    memory: DeviceMemory,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the type of the handle pointed to by
    ///[`GetMemoryZirconHandleFUCHSIA`]::`pZirconHandle`.
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
