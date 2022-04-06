//![VK_FUCHSIA_external_memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_external_memory.html) - device extension
//!# Description
//!Vulkan apps may wish to export or import device memory handles to or from
//!other logical devices, instances or APIs.This memory sharing can eliminate copies of memory
//! buffers when different
//!subsystems need to interoperate on them.
//!Sharing memory buffers may also facilitate a better distribution of
//!processing workload for more complex memory manipulation pipelines.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_external_memory_capabilities`]`
//! - Requires `[`VK_KHR_external_memory`]`
//!# Contacts
//! - John Rosasco [rosasco](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_FUCHSIA_external_memory]
//!   @rosasco%0A<<Here describe the issue or question you have about the VK_FUCHSIA_external_memory
//!   extension>>)
//!# New functions & commands
//! - [`get_memory_zircon_handle_fuchsia`]
//! - [`get_memory_zircon_handle_properties_fuchsia`]
//!# New structures
//! - [`MemoryGetZirconHandleInfoFUCHSIA`]
//! - [`MemoryZirconHandlePropertiesFUCHSIA`]
//! - Extending [`MemoryAllocateInfo`]:  - [`ImportMemoryZirconHandleInfoFUCHSIA`]
//!# New constants
//! - [`FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME`]
//! - [`FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION`]
//! - Extending [`ExternalMemoryHandleTypeFlagBits`]:  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA`
//!   - `VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA`
//!# Known issues & F.A.Q
//!See `[`VK_KHR_external_memory`]` issues list for further information.
//!# Version History
//! - Revision 1, 2021-03-01 (John Rosasco)  - Initial draft
//!# Other info
//! * 2021-03-01
//! * No known IP claims.
//! * - Craig Stout, Google  - John Bauman, Google  - John Rosasco, Google
//!# Related
//! - [`ImportMemoryZirconHandleInfoFUCHSIA`]
//! - [`MemoryGetZirconHandleInfoFUCHSIA`]
//! - [`MemoryZirconHandlePropertiesFUCHSIA`]
//! - [`get_memory_zircon_handle_fuchsia`]
//! - [`get_memory_zircon_handle_properties_fuchsia`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    native::zx_handle_t,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
    AsRaw, Unique, VulkanResult,
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
///[vkGetMemoryZirconHandleFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html) - Get a Zircon handle for an external memory object
///# C Specifications
///To export device memory as a Zircon handle that can be used by another
///instance, device, or process, the handle to the [`DeviceMemory`] must be
///retrieved using [`get_memory_zircon_handle_fuchsia`]:
///```c
///// Provided by VK_FUCHSIA_external_memory
///VkResult vkGetMemoryZirconHandleFUCHSIA(
///    VkDevice                                    device,
///    const VkMemoryGetZirconHandleInfoFUCHSIA*   pGetZirconHandleInfo,
///    zx_handle_t*                                pZirconHandle);
///```
///# Parameters
/// - [`device`] is the [`Device`].
/// - [`p_get_zircon_handle_info`] is a pointer to a [`MemoryGetZirconHandleInfoFUCHSIA`] structure.
/// - [`p_zircon_handle`] is a pointer to a [`zx_handle_t`] which holds the resulting Zircon handle.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_get_zircon_handle_info`] **must**  be a valid pointer to a valid
///   [`MemoryGetZirconHandleInfoFUCHSIA`] structure
/// - [`p_zircon_handle`] **must**  be a valid pointer to a [`zx_handle_t`] value
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
///# Related
/// - [`VK_FUCHSIA_external_memory`]
/// - [`Device`]
/// - [`MemoryGetZirconHandleInfoFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
pub type FNGetMemoryZirconHandleFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA<'lt>,
        p_zircon_handle: *mut zx_handle_t,
    ) -> VulkanResultCodes,
>;
///[vkGetMemoryZirconHandlePropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html) - Get a Zircon handle properties for an external memory object
///# C Specifications
///To obtain the memoryTypeIndex for the [`MemoryAllocateInfo`] structure,
///call [`get_memory_zircon_handle_properties_fuchsia`]:
///```c
///// Provided by VK_FUCHSIA_external_memory
///VkResult vkGetMemoryZirconHandlePropertiesFUCHSIA(
///    VkDevice                                    device,
///    VkExternalMemoryHandleTypeFlagBits          handleType,
///    zx_handle_t                                 zirconHandle,
///    VkMemoryZirconHandlePropertiesFUCHSIA*      pMemoryZirconHandleProperties);
///```
///# Parameters
/// - [`device`] is the [`Device`].
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of
///   [`zircon_handle`]
/// - [`zircon_handle`] is a [`zx_handle_t`] (Zircon) handle to the external resource.
/// - [`p_memory_zircon_handle_properties`] is a pointer to a
///   [`MemoryZirconHandlePropertiesFUCHSIA`] structure in which the result will be stored.
///# Description
///## Valid Usage
/// - [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
/// - [`zircon_handle`] must reference a valid VMO
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
/// - [`p_memory_zircon_handle_properties`] **must**  be a valid pointer to a
///   [`MemoryZirconHandlePropertiesFUCHSIA`] structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///# Related
/// - [`VK_FUCHSIA_external_memory`]
/// - [`Device`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`MemoryZirconHandlePropertiesFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
pub type FNGetMemoryZirconHandlePropertiesFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        zircon_handle: zx_handle_t,
        p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA<'lt>,
    ) -> VulkanResultCodes,
>;
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
///## Valid Usage
/// - [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
/// - [`handle`] must be a valid VMO handle
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA`
/// - If [`handle_type`] is not `0`, [`handle_type`] **must**  be a valid
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
#[doc(alias = "VkImportMemoryZirconHandleInfoFUCHSIA")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImportMemoryZirconHandleInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the type of [`handle`].
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    ///[`handle`] is a [`zx_handle_t`] (Zircon) handle to the external
    ///memory.
    pub handle: zx_handle_t,
}
impl<'lt> Default for ImportMemoryZirconHandleInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> ImportMemoryZirconHandleInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::handle`]
    pub fn handle_raw(&self) -> &zx_handle_t {
        &self.handle
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::handle`]
    pub fn set_handle_raw(mut self, value: zx_handle_t) -> Self {
        self.handle = value;
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
    ///Gets the value of [`Self::handle`]
    pub fn handle(&self) -> zx_handle_t {
        self.handle
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::handle`]
    pub fn handle_mut(&mut self) -> &mut zx_handle_t {
        &mut self.handle
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
    ///Sets the value of [`Self::handle`]
    pub fn set_handle(mut self, value: crate::native::zx_handle_t) -> Self {
        self.handle = value;
        self
    }
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_FUCHSIA_external_memory`]
/// - [`StructureType`]
/// - [`get_memory_zircon_handle_properties_fuchsia`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryZirconHandlePropertiesFUCHSIA")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryZirconHandlePropertiesFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`memory_type_bits`] a bitmask containing one bit set for every memory
    ///type which the specified handle can be imported as.
    pub memory_type_bits: u32,
}
impl<'lt> Default for MemoryZirconHandlePropertiesFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA,
            p_next: std::ptr::null_mut(),
            memory_type_bits: 0,
        }
    }
}
impl<'lt> MemoryZirconHandlePropertiesFUCHSIA<'lt> {
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
///   handle pointed to by [`get_memory_zircon_handle_fuchsia`]`::pZirconHandle`.
///# Description
///## Valid Usage
/// - [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
/// - [`handle_type`] **must**  have been included in the `handleTypes` field of the
///   [`ExportMemoryAllocateInfo`] structure when the external memory was allocated
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`VK_FUCHSIA_external_memory`]
/// - [`DeviceMemory`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`get_memory_zircon_handle_fuchsia`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryGetZirconHandleInfoFUCHSIA")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryGetZirconHandleInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`memory`] the [`DeviceMemory`] being exported.
    pub memory: DeviceMemory,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the type of the handle pointed to by
    ///[`get_memory_zircon_handle_fuchsia`]::`pZirconHandle`.
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl<'lt> Default for MemoryGetZirconHandleInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> MemoryGetZirconHandleInfoFUCHSIA<'lt> {
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
    ///[vkGetMemoryZirconHandleFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandleFUCHSIA.html) - Get a Zircon handle for an external memory object
    ///# C Specifications
    ///To export device memory as a Zircon handle that can be used by another
    ///instance, device, or process, the handle to the [`DeviceMemory`] must be
    ///retrieved using [`get_memory_zircon_handle_fuchsia`]:
    ///```c
    ///// Provided by VK_FUCHSIA_external_memory
    ///VkResult vkGetMemoryZirconHandleFUCHSIA(
    ///    VkDevice                                    device,
    ///    const VkMemoryGetZirconHandleInfoFUCHSIA*   pGetZirconHandleInfo,
    ///    zx_handle_t*                                pZirconHandle);
    ///```
    ///# Parameters
    /// - [`device`] is the [`Device`].
    /// - [`p_get_zircon_handle_info`] is a pointer to a [`MemoryGetZirconHandleInfoFUCHSIA`]
    ///   structure.
    /// - [`p_zircon_handle`] is a pointer to a [`zx_handle_t`] which holds the resulting Zircon
    ///   handle.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_get_zircon_handle_info`] **must**  be a valid pointer to a valid
    ///   [`MemoryGetZirconHandleInfoFUCHSIA`] structure
    /// - [`p_zircon_handle`] **must**  be a valid pointer to a [`zx_handle_t`] value
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///# Related
    /// - [`VK_FUCHSIA_external_memory`]
    /// - [`Device`]
    /// - [`MemoryGetZirconHandleInfoFUCHSIA`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetMemoryZirconHandleFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_memory_zircon_handle_fuchsia<'a: 'this, 'b: 'a + 'this, 'this, 'lt>(
        self: &'this Unique<'a, 'b, Device>,
        p_get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA<'lt>,
    ) -> VulkanResult<zx_handle_t> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_external_memory()
            .and_then(|vtable| vtable.get_memory_zircon_handle_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_external_memory()
            .and_then(|vtable| vtable.get_memory_zircon_handle_fuchsia())
            .unwrap_unchecked();
        let mut p_zircon_handle = std::mem::zeroed();
        let _return = _function(
            self.as_raw(),
            p_get_zircon_handle_info as *const MemoryGetZirconHandleInfoFUCHSIA<'lt>,
            &mut p_zircon_handle,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_zircon_handle),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetMemoryZirconHandlePropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html) - Get a Zircon handle properties for an external memory object
    ///# C Specifications
    ///To obtain the memoryTypeIndex for the [`MemoryAllocateInfo`] structure,
    ///call [`get_memory_zircon_handle_properties_fuchsia`]:
    ///```c
    ///// Provided by VK_FUCHSIA_external_memory
    ///VkResult vkGetMemoryZirconHandlePropertiesFUCHSIA(
    ///    VkDevice                                    device,
    ///    VkExternalMemoryHandleTypeFlagBits          handleType,
    ///    zx_handle_t                                 zirconHandle,
    ///    VkMemoryZirconHandlePropertiesFUCHSIA*      pMemoryZirconHandleProperties);
    ///```
    ///# Parameters
    /// - [`device`] is the [`Device`].
    /// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of
    ///   [`zircon_handle`]
    /// - [`zircon_handle`] is a [`zx_handle_t`] (Zircon) handle to the external resource.
    /// - [`p_memory_zircon_handle_properties`] is a pointer to a
    ///   [`MemoryZirconHandlePropertiesFUCHSIA`] structure in which the result will be stored.
    ///# Description
    ///## Valid Usage
    /// - [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
    /// - [`zircon_handle`] must reference a valid VMO
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
    /// - [`p_memory_zircon_handle_properties`] **must**  be a valid pointer to a
    ///   [`MemoryZirconHandlePropertiesFUCHSIA`] structure
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    ///# Related
    /// - [`VK_FUCHSIA_external_memory`]
    /// - [`Device`]
    /// - [`ExternalMemoryHandleTypeFlagBits`]
    /// - [`MemoryZirconHandlePropertiesFUCHSIA`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetMemoryZirconHandlePropertiesFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia<'a: 'this, 'b: 'a + 'this, 'this, 'lt>(
        self: &'this Unique<'a, 'b, Device>,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        zircon_handle: zx_handle_t,
        p_memory_zircon_handle_properties: Option<MemoryZirconHandlePropertiesFUCHSIA<'lt>>,
    ) -> VulkanResult<MemoryZirconHandlePropertiesFUCHSIA<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_external_memory()
            .and_then(|vtable| vtable.get_memory_zircon_handle_properties_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_external_memory()
            .and_then(|vtable| vtable.get_memory_zircon_handle_properties_fuchsia())
            .unwrap_unchecked();
        let mut p_memory_zircon_handle_properties = p_memory_zircon_handle_properties.unwrap_or_default();
        let _return = _function(
            self.as_raw(),
            handle_type,
            zircon_handle,
            &mut p_memory_zircon_handle_properties,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, {
                p_memory_zircon_handle_properties.p_next = std::ptr::null_mut();
                p_memory_zircon_handle_properties
            }),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_FUCHSIA_external_memory`
pub struct DeviceFuchsiaExternalMemoryVTable {
    ///See [`FNGetMemoryZirconHandleFuchsia`] for more information.
    pub get_memory_zircon_handle_fuchsia: FNGetMemoryZirconHandleFuchsia,
    ///See [`FNGetMemoryZirconHandlePropertiesFuchsia`] for more information.
    pub get_memory_zircon_handle_properties_fuchsia: FNGetMemoryZirconHandlePropertiesFuchsia,
}
impl DeviceFuchsiaExternalMemoryVTable {
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
            get_memory_zircon_handle_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetMemoryZirconHandleFUCHSIA").as_ptr(),
                ))
            },
            get_memory_zircon_handle_properties_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetMemoryZirconHandlePropertiesFUCHSIA").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_memory_zircon_handle_fuchsia`]. See [`FNGetMemoryZirconHandleFuchsia`] for
    /// more information.
    pub fn get_memory_zircon_handle_fuchsia(&self) -> FNGetMemoryZirconHandleFuchsia {
        self.get_memory_zircon_handle_fuchsia
    }
    ///Gets [`Self::get_memory_zircon_handle_properties_fuchsia`]. See
    /// [`FNGetMemoryZirconHandlePropertiesFuchsia`] for more information.
    pub fn get_memory_zircon_handle_properties_fuchsia(&self) -> FNGetMemoryZirconHandlePropertiesFuchsia {
        self.get_memory_zircon_handle_properties_fuchsia
    }
}
