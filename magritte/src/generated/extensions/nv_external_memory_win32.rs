//![VK_NV_external_memory_win32](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_win32.html) - device extension
//!# Description
//!Applications may wish to export memory to other Vulkan instances or other
//!APIs, or import memory from other Vulkan instances or other APIs to enable
//!Vulkan workloads to be split up across application module, process, or API
//!boundaries.
//!This extension enables win32 applications to export win32 handles from
//!Vulkan memory objects such that the underlying resources can be referenced
//!outside the Vulkan instance that created them, and import win32 handles
//!created in the Direct3D API to Vulkan memory objects.
//!# Revision
//!1
//!# Dependencies
//! - *Deprecated* by `[`khr_external_memory_win32`]` extension
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`nv_external_memory`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_external_memory_win32]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_NV_external_memory_win32 extension>>)
//!# New functions & commands
//! - [`get_memory_win32_handle_nv`]
//!# New structures
//! - Extending [`MemoryAllocateInfo`]:  - [`ExportMemoryWin32HandleInfoNV`]  -
//!   [`ImportMemoryWin32HandleInfoNV`]
//!# New constants
//! - [`NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME`]
//! - [`NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV`
//!# Known issues & F.A.Q
//!1) If memory objects are shared between processes and APIs, is this
//!considered aliasing according to the rules outlined in the
//![Memory Aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-memory-aliasing) section? **RESOLVED** : Yes, but strict exceptions to the rules are added to allow some
//!forms of aliasing in these cases.
//!Further, other extensions may build upon these new aliasing rules to define
//!specific support usage within Vulkan for imported native memory objects, or
//!memory objects from other APIs.2) Are new image layouts or metadata required to specify image
//! layouts and
//!layout transitions compatible with non-Vulkan APIs, or with other instances
//!of the same Vulkan driver? **RESOLVED** : No.
//!Separate instances of the same Vulkan driver running on the same GPU should
//!have identical internal layout semantics, so applictions have the tools they
//!need to ensure views of images are consistent between the two instances.
//!Other APIs will fall into two categories: Those that are Vulkan compatible
//!(a term to be defined by subsequent interopability extensions), or Vulkan
//!incompatible.
//!When sharing images with Vulkan incompatible APIs, the Vulkan image must be
//!transitioned to the `VK_IMAGE_LAYOUT_GENERAL` layout before handing it
//!off to the external API.Note this does not attempt to address cross-device transitions, nor
//!transitions to engines on the same device which are not visible within the
//!Vulkan API.
//!Both of these are beyond the scope of this extension.3) Do applications need to call
//! `CloseHandle`() on the values returned
//!from [`get_memory_win32_handle_nv`] when `handleType` is
//!`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV`? **RESOLVED** : Yes, unless it is passed
//! back in to another driver instance to
//!import the object.
//!A successful get call transfers ownership of the handle to the application,
//!while an import transfers ownership to the associated driver.
//!Destroying the memory object will not destroy the handle or the handle’s
//!reference to the underlying memory resource.
//!# Version History
//! - Revision 1, 2016-08-11 (James Jones)  - Initial draft
//!# Other info
//! * 2016-08-19
//! * No known IP claims.
//! * - James Jones, NVIDIA  - Carsten Rohde, NVIDIA
//!# Related
//! - [`ExportMemoryWin32HandleInfoNV`]
//! - [`ImportMemoryWin32HandleInfoNV`]
//! - [`get_memory_win32_handle_nv`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    native::{DWORD, HANDLE, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_external_memory_win32");
///[vkGetMemoryWin32HandleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html) - Retrieve Win32 handle to a device memory object
///# C Specifications
///To retrieve the handle corresponding to a device memory object created with
///[`ExportMemoryAllocateInfoNV::handle_types`] set to include
///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV` or
///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV`, call:
///```c
///// Provided by VK_NV_external_memory_win32
///VkResult vkGetMemoryWin32HandleNV(
///    VkDevice                                    device,
///    VkDeviceMemory                              memory,
///    VkExternalMemoryHandleTypeFlagsNV           handleType,
///    HANDLE*                                     pHandle);
///```
/// # Parameters
/// - [`device`] is the logical device that owns the memory.
/// - [`memory`] is the [`DeviceMemory`] object.
/// - [`handle_type`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] containing a single bit
///   specifying the type of handle requested.
/// - `handle` is a pointer to a Windows [`HANDLE`] in which the handle is returned.
/// # Description
/// ## Valid Usage
/// - [`handle_type`] **must**  be a flag specified in [`ExportMemoryAllocateInfoNV::handle_types`]
///   when allocating [`memory`]
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
/// - [`handle_type`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`]
///   values
/// - [`handle_type`] **must**  not be `0`
/// - [`p_handle`] **must**  be a valid pointer to a [`HANDLE`] value
/// - [`memory`] **must**  have been created, allocated, or retrieved from [`device`]
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`nv_external_memory_win32`]
/// - [`Device`]
/// - [`DeviceMemory`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetMemoryWin32HandleNV")]
pub type FNGetMemoryWin32HandleNv = Option<
    unsafe extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_handle: *mut HANDLE,
    ) -> VulkanResultCodes,
>;
///[VkImportMemoryWin32HandleInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoNV.html) - Import Win32 memory created on the same physical device
///# C Specifications
///To import memory created on the same physical device but outside of the
///current Vulkan instance, add a [`ImportMemoryWin32HandleInfoNV`]
///structure to the [`p_next`] chain of the [`MemoryAllocateInfo`]
///structure, specifying a handle to and the type of the memory.The
/// [`ImportMemoryWin32HandleInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_external_memory_win32
///typedef struct VkImportMemoryWin32HandleInfoNV {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkExternalMemoryHandleTypeFlagsNV    handleType;
///    HANDLE                               handle;
///} VkImportMemoryWin32HandleInfoNV;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is `0` or a [`ExternalMemoryHandleTypeFlagBitsNV`] value specifying the type
///   of memory handle in [`handle`].
/// - [`handle`] is a Windows [`HANDLE`] referring to the memory.
/// # Description
/// If [`handle_type`] is `0`, this structure is ignored by consumers of the
/// [`MemoryAllocateInfo`] structure it is chained from.
/// ## Valid Usage
/// - [`handle_type`] **must**  not have more than one bit set
/// - [`handle`] **must**  be a valid handle to memory, obtained as specified by [`handle_type`]
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV`
/// - [`handle_type`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`]
///   values
/// # Related
/// - [`nv_external_memory_win32`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportMemoryWin32HandleInfoNV")]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_type`] is `0` or a [`ExternalMemoryHandleTypeFlagBitsNV`]
    ///value specifying the type of memory handle in [`handle`].
    pub handle_type: ExternalMemoryHandleTypeFlagsNV,
    ///[`handle`] is a Windows [`HANDLE`] referring to the memory.
    pub handle: HANDLE,
}
impl<'lt> Default for ImportMemoryWin32HandleInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> ImportMemoryWin32HandleInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::handle`]
    pub fn handle_raw(&self) -> &HANDLE {
        &self.handle
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
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.handle_type
    }
    ///Gets the value of [`Self::handle`]
    pub fn handle(&self) -> HANDLE {
        self.handle
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagsNV {
        &mut self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::handle`]
    pub fn handle_mut(&mut self) -> &mut HANDLE {
        &mut self.handle
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
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(
        &mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> &mut Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::handle`]
    pub fn set_handle(&mut self, value: crate::native::HANDLE) -> &mut Self {
        self.handle = value;
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
    ///Sets the value of [`Self::handle_type`]
    pub fn with_handle_type(
        mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::handle`]
    pub fn with_handle(mut self, value: crate::native::HANDLE) -> Self {
        self.handle = value;
        self
    }
}
///[VkExportMemoryWin32HandleInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoNV.html) - Specify security attributes and access rights for Win32 memory handles
///# C Specifications
///When [`ExportMemoryAllocateInfoNV::handle_types`] includes
///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV`, add a
///[`ExportMemoryWin32HandleInfoNV`] structure to the [`p_next`] chain of
///the [`ExportMemoryAllocateInfoNV`] structure to specify security
///attributes and access rights for the memory object’s external handle.The
/// [`ExportMemoryWin32HandleInfoNV`] structure is defined as:
///```c
///// Provided by VK_NV_external_memory_win32
///typedef struct VkExportMemoryWin32HandleInfoNV {
///    VkStructureType               sType;
///    const void*                   pNext;
///    const SECURITY_ATTRIBUTES*    pAttributes;
///    DWORD                         dwAccess;
///} VkExportMemoryWin32HandleInfoNV;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying security
///   attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
/// # Description
/// If this structure is not present, or if [`attributes`] is set to `NULL`,
/// default security descriptor values will be used, and child processes created
/// by the application will not inherit the handle, as described in the MSDN
/// documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
/// Further, if the structure is not present, the access rights will be`DXGI_SHARED_RESOURCE_READ` |
/// `DXGI_SHARED_RESOURCE_WRITE`
/// * [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV`
/// - If [`attributes`] is not `NULL`, [`attributes`] **must**  be a valid pointer to a valid
///   [`SECURITY_ATTRIBUTES`] value
/// # Related
/// - [`nv_external_memory_win32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExportMemoryWin32HandleInfoNV")]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNV<'lt> {
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
}
impl<'lt> Default for ExportMemoryWin32HandleInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: std::ptr::null(),
            attributes: std::ptr::null(),
            dw_access: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> ExportMemoryWin32HandleInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::dw_access`]
    pub fn dw_access_raw(&self) -> &DWORD {
        &self.dw_access
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
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::dw_access`]
    pub fn dw_access_mut(&mut self) -> &mut DWORD {
        &mut self.dw_access
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
}
impl Device {
    ///[vkGetMemoryWin32HandleNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleNV.html) - Retrieve Win32 handle to a device memory object
    ///# C Specifications
    ///To retrieve the handle corresponding to a device memory object created with
    ///[`ExportMemoryAllocateInfoNV::handle_types`] set to include
    ///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV` or
    ///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV`, call:
    ///```c
    ///// Provided by VK_NV_external_memory_win32
    ///VkResult vkGetMemoryWin32HandleNV(
    ///    VkDevice                                    device,
    ///    VkDeviceMemory                              memory,
    ///    VkExternalMemoryHandleTypeFlagsNV           handleType,
    ///    HANDLE*                                     pHandle);
    ///```
    /// # Parameters
    /// - [`device`] is the logical device that owns the memory.
    /// - [`memory`] is the [`DeviceMemory`] object.
    /// - [`handle_type`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] containing a single
    ///   bit specifying the type of handle requested.
    /// - `handle` is a pointer to a Windows [`HANDLE`] in which the handle is returned.
    /// # Description
    /// ## Valid Usage
    /// - [`handle_type`] **must**  be a flag specified in
    ///   [`ExportMemoryAllocateInfoNV::handle_types`] when allocating [`memory`]
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
    /// - [`handle_type`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`]
    ///   values
    /// - [`handle_type`] **must**  not be `0`
    /// - [`p_handle`] **must**  be a valid pointer to a [`HANDLE`] value
    /// - [`memory`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`nv_external_memory_win32`]
    /// - [`Device`]
    /// - [`DeviceMemory`]
    /// - [`ExternalMemoryHandleTypeFlagsNV`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetMemoryWin32HandleNV")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_memory_win32_handle_nv(
        self: &Unique<Device>,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> VulkanResult<HANDLE> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .nv_external_memory_win32()
            .and_then(|vtable| vtable.get_memory_win32_handle_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .nv_external_memory_win32()
            .and_then(|vtable| vtable.get_memory_win32_handle_nv())
            .unwrap_unchecked();
        let mut p_handle = std::mem::zeroed();
        let _return = _function(self.as_raw(), memory, handle_type, &mut p_handle);
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_handle),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_NV_external_memory_win32`
pub struct DeviceNvExternalMemoryWin32VTable {
    ///See [`FNGetMemoryWin32HandleNv`] for more information.
    pub get_memory_win32_handle_nv: FNGetMemoryWin32HandleNv,
}
impl DeviceNvExternalMemoryWin32VTable {
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
            get_memory_win32_handle_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetMemoryWin32HandleNV").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_memory_win32_handle_nv`]. See [`FNGetMemoryWin32HandleNv`] for more
    /// information.
    pub fn get_memory_win32_handle_nv(&self) -> FNGetMemoryWin32HandleNv {
        self.get_memory_win32_handle_nv
    }
}
