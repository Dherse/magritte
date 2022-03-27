use crate::{
    native::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Fence, StructureType},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_fence_win32");
///[VkImportFenceWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportFenceWin32HandleInfoKHR.html) - (None)
///# C Specifications
///The [`ImportFenceWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_fence_win32
///typedef struct VkImportFenceWin32HandleInfoKHR {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkFence                              fence;
///    VkFenceImportFlags                   flags;
///    VkExternalFenceHandleTypeFlagBits    handleType;
///    HANDLE                               handle;
///    LPCWSTR                              name;
///} VkImportFenceWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fence`] is the fence into which the state will be imported.
/// - [`flags`] is a bitmask of [`FenceImportFlagBits`] specifying additional parameters for the
///   fence payload import operation.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of
///   [`handle`].
/// - [`handle`] is `NULL` or the external handle to import.
/// - [`name`] is `NULL` or a null-terminated UTF-16 string naming the underlying synchronization
///   primitive to import.
///# Description
///The handle types supported by [`handle_type`] are:Valid Usage
/// - [`handle_type`]**must** be a value included in the [Handle Types Supported by [`ImportFenceWin32HandleInfoKHR`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fence-handletypes-win32)
///   table
/// - If [`handle_type`] is not `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`, [`name`]**must**
///   be `NULL`
/// - If [`handle`] is `NULL`, [`name`]**must** name a valid synchronization primitive of the type
///   specified by [`handle_type`]
/// - If [`name`] is `NULL`, [`handle`]**must** be a valid handle of the type specified by
///   [`handle_type`]
/// - If [`handle`] is not `NULL`, [`name`]**must** be `NULL`
/// -    If [`handle`] is not `NULL`, it **must** obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
/// -    If [`name`] is not `NULL`, it **must** obey any requirements listed for [`handle_type`] in [external fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`fence`]**must** be a valid [`Fence`] handle
/// - [`flags`]**must** be a valid combination of [`FenceImportFlagBits`] values
///Host Synchronization
/// - Host access to [`fence`]**must** be externally synchronized
///# Related
/// - [`VK_KHR_external_fence_win32`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`FenceImportFlags`]
/// - [`StructureType`]
/// - [`ImportFenceWin32HandleKHR`]
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
pub struct ImportFenceWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`fence`] is the fence into which the state will be imported.
    fence: Fence,
    ///[`flags`] is a bitmask of [`FenceImportFlagBits`] specifying
    ///additional parameters for the fence payload import operation.
    flags: FenceImportFlags,
    ///[`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value
    ///specifying the type of [`handle`].
    handle_type: ExternalFenceHandleTypeFlagBits,
    ///[`handle`] is `NULL` or the external handle to import.
    handle: HANDLE,
    ///[`name`] is `NULL` or a null-terminated UTF-16 string naming the
    ///underlying synchronization primitive to import.
    name: LPCWSTR,
}
impl<'lt> Default for ImportFenceWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            fence: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            handle: Default::default(),
            name: Default::default(),
        }
    }
}
impl<'lt> ImportFenceWin32HandleInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::handle`]
    pub fn handle_raw(&self) -> &HANDLE {
        &self.handle
    }
    ///Gets the raw value of [`Self::name`]
    pub fn name_raw(&self) -> &LPCWSTR {
        &self.name
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
    ///Sets the raw value of [`Self::name`]
    pub fn set_name_raw(&mut self, value: LPCWSTR) -> &mut Self {
        self.name = value;
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
    ///Gets the value of [`Self::handle`]
    pub fn handle(&self) -> &HANDLE {
        &self.handle
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &LPCWSTR {
        &self.name
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
    ///Gets a mutable reference to the value of [`Self::handle`]
    pub fn handle_mut(&mut self) -> &mut HANDLE {
        &mut self.handle
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut LPCWSTR {
        &mut self.name
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
    ///Sets the raw value of [`Self::handle`]
    pub fn set_handle(&mut self, value: crate::native::HANDLE) -> &mut Self {
        self.handle = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: crate::native::LPCWSTR) -> &mut Self {
        self.name = value;
        self
    }
}
///[VkExportFenceWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportFenceWin32HandleInfoKHR.html) - Structure specifying additional attributes of Windows handles exported from a fence
///# C Specifications
///To specify additional attributes of NT handles exported from a fence, add a
///[`ExportFenceWin32HandleInfoKHR`] structure to the [`p_next`] chain of
///the [`FenceCreateInfo`] structure.
///The [`ExportFenceWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_fence_win32
///typedef struct VkExportFenceWin32HandleInfoKHR {
///    VkStructureType               sType;
///    const void*                   pNext;
///    const SECURITY_ATTRIBUTES*    pAttributes;
///    DWORD                         dwAccess;
///    LPCWSTR                       name;
///} VkExportFenceWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attributes`] is a pointer to a Windows [`SECURITY_ATTRIBUTES`] structure specifying security
///   attributes of the handle.
/// - [`dw_access`] is a [`DWORD`] specifying access rights of the handle.
/// - [`name`] is a null-terminated UTF-16 string to associate with the underlying synchronization
///   primitive referenced by NT handles exported from the created fence.
///# Description
///If [`ExportFenceCreateInfo`] is not inluded in the same [`p_next`]
///chain, this structure is ignored.If [`ExportFenceCreateInfo`] is included in the [`p_next`]
/// chain of
///[`FenceCreateInfo`] with a Windows `handleType`, but either
///[`ExportFenceWin32HandleInfoKHR`] is not included in the [`p_next`]
///chain, or if it is but [`attributes`] is set to `NULL`, default security
///descriptor values will be used, and child processes created by the
///application will not inherit the handle, as described in the MSDN
///documentation for “Synchronization Object Security and Access Rights”<sup>1</sup>.
///Further, if the structure is not present, the access rights will be`DXGI_SHARED_RESOURCE_READ` |
/// `DXGI_SHARED_RESOURCE_WRITE`for handles of the following
/// types:`VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`
/// * [https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights](https://docs.microsoft.com/en-us/windows/win32/sync/synchronization-object-security-and-access-rights)
///Valid Usage
/// - If [`ExportFenceCreateInfo::handle_types`] does not include
///   `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT`, a [`ExportFenceWin32HandleInfoKHR`]
///   structure **must** not be included in the [`p_next`] chain of [`FenceCreateInfo`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR`
/// - If [`attributes`] is not `NULL`, [`attributes`]**must** be a valid pointer to a valid
///   [`SECURITY_ATTRIBUTES`] value
///# Related
/// - [`VK_KHR_external_fence_win32`]
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
pub struct ExportFenceWin32HandleInfoKHR<'lt> {
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
    ///[`name`] is a null-terminated UTF-16 string to associate with the
    ///underlying synchronization primitive referenced by NT handles exported
    ///from the created fence.
    name: LPCWSTR,
}
impl<'lt> Default for ExportFenceWin32HandleInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            attributes: std::ptr::null(),
            dw_access: Default::default(),
            name: Default::default(),
        }
    }
}
impl<'lt> ExportFenceWin32HandleInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::dw_access`]
    pub fn dw_access_raw(&self) -> &DWORD {
        &self.dw_access
    }
    ///Gets the raw value of [`Self::name`]
    pub fn name_raw(&self) -> &LPCWSTR {
        &self.name
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
    ///Sets the raw value of [`Self::name`]
    pub fn set_name_raw(&mut self, value: LPCWSTR) -> &mut Self {
        self.name = value;
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
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &LPCWSTR {
        &self.name
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::dw_access`]
    pub fn dw_access_mut(&mut self) -> &mut DWORD {
        &mut self.dw_access
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut LPCWSTR {
        &mut self.name
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
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: crate::native::LPCWSTR) -> &mut Self {
        self.name = value;
        self
    }
}
///[VkFenceGetWin32HandleInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceGetWin32HandleInfoKHR.html) - Structure describing a Win32 handle fence export operation
///# C Specifications
///The [`FenceGetWin32HandleInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_external_fence_win32
///typedef struct VkFenceGetWin32HandleInfoKHR {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkFence                              fence;
///    VkExternalFenceHandleTypeFlagBits    handleType;
///} VkFenceGetWin32HandleInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fence`] is the fence from which state will be exported.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying the type of handle
///   requested.
///# Description
///The properties of the handle returned depend on the value of
///[`handle_type`].
///See [`ExternalFenceHandleTypeFlagBits`] for a description of the
///properties of the defined external fence handle types.Valid Usage
/// - [`handle_type`]**must** have been included in [`ExportFenceCreateInfo::handle_types`] when the
///   [`fence`]’s current payload was created
/// - If [`handle_type`] is defined as an NT handle, [`GetFenceWin32HandleKHR`]**must** be called no
///   more than once for each valid unique combination of [`fence`] and [`handle_type`]
/// -  [`fence`]**must** not currently have its payload replaced by an imported payload as described below in [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing) unless that imported payload’s handle type was included in [`ExternalFenceProperties::export_from_imported_handle_types`] for [`handle_type`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`fence`]**must** be signaled, or have an associated [fence signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-signaling) pending execution
/// - [`handle_type`]**must** be defined as an NT handle or a global share handle
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`fence`]**must** be a valid [`Fence`] handle
/// - [`handle_type`]**must** be a valid [`ExternalFenceHandleTypeFlagBits`] value
///# Related
/// - [`VK_KHR_external_fence_win32`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`Fence`]
/// - [`StructureType`]
/// - [`GetFenceWin32HandleKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FenceGetWin32HandleInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`fence`] is the fence from which state will be exported.
    fence: Fence,
    ///[`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    handle_type: ExternalFenceHandleTypeFlagBits,
}
impl<'lt> Default for FenceGetWin32HandleInfoKHR<'lt> {
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
impl<'lt> FenceGetWin32HandleInfoKHR<'lt> {
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
