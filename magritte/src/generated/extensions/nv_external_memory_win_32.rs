use crate::{
    extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    native::{DWORD, HANDLE, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, StructureType},
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is `0` or a [`ExternalMemoryHandleTypeFlagBitsNV`] value specifying the type
///   of memory handle in [`handle`].
/// - [`handle`] is a Windows [`HANDLE`] referring to the memory.
///# Description
///If [`handle_type`] is `0`, this structure is ignored by consumers of the
///[`MemoryAllocateInfo`] structure it is chained from.
///## Valid Usage
/// - [`handle_type`] **must**  not have more than one bit set
/// - [`handle`] **must**  be a valid handle to memory, obtained as specified by [`handle_type`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV`
/// - [`handle_type`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBitsNV`]
///   values
///# Related
/// - [`VK_NV_external_memory_win32`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`handle_type`] is `0` or a [`ExternalMemoryHandleTypeFlagBitsNV`]
    ///value specifying the type of memory handle in [`handle`].
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    ///[`handle`] is a Windows [`HANDLE`] referring to the memory.
    handle: HANDLE,
}
impl<'lt> Default for ImportMemoryWin32HandleInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            handle: Default::default(),
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
    pub fn handle(&self) -> &HANDLE {
        &self.handle
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
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(
        &mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> &mut Self {
        self.handle_type = value;
        self
    }
    ///Sets the raw value of [`Self::handle`]
    pub fn set_handle(&mut self, value: crate::native::HANDLE) -> &mut Self {
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
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying security
///   attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
///# Description
///If this structure is not present, or if [`attributes`] is set to `NULL`,
///default security descriptor values will be used, and child processes created
///by the application will not inherit the handle, as described in the MSDN
///documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
///Further, if the structure is not present, the access rights will be`DXGI_SHARED_RESOURCE_READ` |
/// `DXGI_SHARED_RESOURCE_WRITE`
/// * [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV`
/// - If [`attributes`] is not `NULL`, [`attributes`] **must**  be a valid pointer to a valid
///   [`SECURITY_ATTRIBUTES`] value
///# Related
/// - [`VK_NV_external_memory_win32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`]
    ///structure specifying security attributes of the handle.
    attributes: *const SECURITY_ATTRIBUTES,
    ///[`dw_access`] is a [`DWORD`] specifying access rights of the handle.
    dw_access: DWORD,
}
impl<'lt> Default for ExportMemoryWin32HandleInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            attributes: std::ptr::null(),
            dw_access: Default::default(),
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
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn attributes(&self) -> &SECURITY_ATTRIBUTES {
        &*self.attributes
    }
    ///Gets the value of [`Self::dw_access`]
    pub fn dw_access(&self) -> &DWORD {
        &self.dw_access
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::dw_access`]
    pub fn dw_access_mut(&mut self) -> &mut DWORD {
        &mut self.dw_access
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
    ///Sets the raw value of [`Self::attributes`]
    pub fn set_attributes(&mut self, value: &'lt crate::native::SECURITY_ATTRIBUTES) -> &mut Self {
        self.attributes = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::dw_access`]
    pub fn set_dw_access(&mut self, value: crate::native::DWORD) -> &mut Self {
        self.dw_access = value;
        self
    }
}
