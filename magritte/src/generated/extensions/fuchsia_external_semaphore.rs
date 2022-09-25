//![VK_FUCHSIA_external_semaphore](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_external_semaphore.html) - device extension
//!# Description
//!An application using external memory may wish to synchronize access to that
//!memory using semaphores.
//!This extension enables an application to export semaphore payload to and
//!import semaphore payload from Zircon event handles.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_external_semaphore_capabilities`]`
//! - Requires `[`khr_external_semaphore`]`
# ! [doc = concat ! ("# " , "Contacts")]
//! - John Rosasco [rosasco](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_FUCHSIA_external_semaphore]
//!   @rosasco%0A<<Here describe the issue or question you have about the
//!   VK_FUCHSIA_external_semaphore extension>>)
# ! [doc = concat ! ("# " , "New commands")]
//! - [`get_semaphore_zircon_handle_fuchsia`]
//! - [`import_semaphore_zircon_handle_fuchsia`]
# ! [doc = concat ! ("# " , "New structures")]
//! - [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
//! - [`SemaphoreGetZirconHandleInfoFUCHSIA`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME`]
//! - [`FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION`]
//! - Extending [`ExternalSemaphoreHandleTypeFlagBits`]:  -
//!   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA`  -
//!   `VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
# ! [doc = concat ! ("# " , "Known issues & F.A.Q.")]
//!1) Does the application need to close the Zircon event handle returned by
//![`get_semaphore_zircon_handle_fuchsia`]? **RESOLVED** : Yes, unless it is passed back in to a
//! driver instance to import
//!the semaphore.
//!A successful get call transfers ownership of the Zircon event handle to the
//!application, and a successful import transfers it back to the driver.
//!Destroying the original semaphore object will not close the Zircon event
//!handle nor remove its reference to the underlying semaphore resource
//!associated with it.
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2021-03-08 (John Rosasco)  - Initial revision
//!# Other info
//! * 2021-03-08
//! * No known IP claims.
//! * - Craig Stout, Google  - John Bauman, Google  - John Rosasco, Google
//!# Related
//! - [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
//! - [`SemaphoreGetZirconHandleInfoFUCHSIA`]
//! - [`get_semaphore_zircon_handle_fuchsia`]
//! - [`import_semaphore_zircon_handle_fuchsia`]
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
    vulkan1_0::{BaseInStructure, Device, Semaphore, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_FUCHSIA_external_semaphore");
///[vkGetSemaphoreZirconHandleFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html) - Get a Zircon event handle for a semaphore
///# C Specifications
///To export a Zircon event handle representing the payload of a semaphore,
///call:
///```c
///// Provided by VK_FUCHSIA_external_semaphore
///VkResult vkGetSemaphoreZirconHandleFUCHSIA(
///    VkDevice                                    device,
///    const VkSemaphoreGetZirconHandleInfoFUCHSIA* pGetZirconHandleInfo,
///    zx_handle_t*                                pZirconHandle);
///```
///# Parameters
/// - [`device`] is the logical device that created the semaphore being exported.
/// - [`p_get_zircon_handle_info`] is a pointer to a [`SemaphoreGetZirconHandleInfoFUCHSIA`]
///   structure containing parameters of the export operation.
/// - [`p_zircon_handle`] will return the Zircon event handle representing the semaphore payload.
///# Description
///Each call to [`get_semaphore_zircon_handle_fuchsia`] **must**  create a Zircon
///event handle and transfer ownership of it to the application.
///To avoid leaking resources, the application  **must**  release ownership of the
///Zircon event handle when it is no longer needed.Exporting a Zircon event handle from a semaphore
/// **may**  have side effects
///depending on the transference of the specified handle type, as described in
///[Importing Semaphore State](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing).
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_get_zircon_handle_info`] **must**  be a valid pointer to a valid
///   [`SemaphoreGetZirconHandleInfoFUCHSIA`] structure
/// - [`p_zircon_handle`] **must**  be a valid pointer to a [`zx_handle_t`] value
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
///# Related
/// - [`fuchsia_external_semaphore`]
/// - [`Device`]
/// - [`SemaphoreGetZirconHandleInfoFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
pub type FNGetSemaphoreZirconHandleFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA<'lt>,
        p_zircon_handle: *mut zx_handle_t,
    ) -> VulkanResultCodes,
>;
///[vkImportSemaphoreZirconHandleFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html) - Import a semaphore from a Zircon event handle
///# C Specifications
///To import a semaphore payload from a Zircon event handle, call:
///```c
///// Provided by VK_FUCHSIA_external_semaphore
///VkResult vkImportSemaphoreZirconHandleFUCHSIA(
///    VkDevice                                    device,
///    const VkImportSemaphoreZirconHandleInfoFUCHSIA* pImportSemaphoreZirconHandleInfo);
///```
///# Parameters
/// - [`device`] is the logical device that created the semaphore.
/// - [`p_import_semaphore_zircon_handle_info`] is a pointer to a
///   [`ImportSemaphoreZirconHandleInfoFUCHSIA`] structure specifying the semaphore and import
///   parameters.
///# Description
///Importing a semaphore payload from a Zircon event handle transfers ownership
///of the handle from the application to the Vulkan implementation.
///The application  **must**  not perform any operations on the handle after a
///successful import.Applications  **can**  import the same semaphore payload into multiple
/// instances
///of Vulkan, into the same instance from which it was exported, and multiple
///times into a given Vulkan instance.
///## Valid Usage
/// - `semaphore` **must**  not be associated with any queue command that has not yet completed
///   execution on that queue
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_import_semaphore_zircon_handle_info`] **must**  be a valid pointer to a valid
///   [`ImportSemaphoreZirconHandleInfoFUCHSIA`] structure
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
///# Related
/// - [`fuchsia_external_semaphore`]
/// - [`Device`]
/// - [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
pub type FNImportSemaphoreZirconHandleFuchsia = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA<'lt>,
    ) -> VulkanResultCodes,
>;
///[VkImportSemaphoreZirconHandleInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreZirconHandleInfoFUCHSIA.html) - Structure specifying Zircon event handle to import to a semaphore
///# C Specifications
///The [`ImportSemaphoreZirconHandleInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_external_semaphore
///typedef struct VkImportSemaphoreZirconHandleInfoFUCHSIA {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    VkSemaphore                              semaphore;
///    VkSemaphoreImportFlags                   flags;
///    VkExternalSemaphoreHandleTypeFlagBits    handleType;
///    zx_handle_t                              zirconHandle;
///} VkImportSemaphoreZirconHandleInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore`] is the semaphore into which the payload will be imported.
/// - [`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying additional parameters for the
///   semaphore payload import operation.
/// - [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of
///   [`zircon_handle`].
/// - [`zircon_handle`] is the external handle to import.
///# Description
///The handle types supported by [`handle_type`] are:
///## Valid Usage
/// - [`handle_type`] **must**  be a value included in the [Handle Types Supported by [`ImportSemaphoreZirconHandleInfoFUCHSIA`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphore-handletypes-fuchsia)
///   table
/// -  [`zircon_handle`] **must**  obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
/// - [`zircon_handle`] **must**  have `ZX_RIGHTS_BASIC` and `ZX_RIGHTS_SIGNAL` rights
/// - The [`SemaphoreTypeCreateInfo::semaphore_type`] field  **must**  not be
///   `VK_SEMAPHORE_TYPE_TIMELINE`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
/// - [`semaphore`] **must**  be a valid [`Semaphore`] handle
/// - [`flags`] **must**  be a valid combination of [`SemaphoreImportFlagBits`] values
/// - [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///
///## Host Synchronization
/// - Host access to [`semaphore`] **must**  be externally synchronized
///# Related
/// - [`fuchsia_external_semaphore`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`SemaphoreImportFlags`]
/// - [`StructureType`]
/// - [`import_semaphore_zircon_handle_fuchsia`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImportSemaphoreZirconHandleInfoFUCHSIA")]
#[repr(C)]
pub struct ImportSemaphoreZirconHandleInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`semaphore`] is the semaphore into which the payload will be
    ///imported.
    pub semaphore: Semaphore,
    ///[`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying
    ///additional parameters for the semaphore payload import operation.
    pub flags: SemaphoreImportFlags,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the type of [`zircon_handle`].
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    ///[`zircon_handle`] is the external handle to import.
    pub zircon_handle: zx_handle_t,
}
impl<'lt> Default for ImportSemaphoreZirconHandleInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            flags: Default::default(),
            handle_type: Default::default(),
            zircon_handle: unsafe { std::mem::zeroed() },
        }
    }
}
impl<'lt> ImportSemaphoreZirconHandleInfoFUCHSIA<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::zircon_handle`]
    pub fn zircon_handle_raw(&self) -> &zx_handle_t {
        &self.zircon_handle
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::zircon_handle`]
    pub fn set_zircon_handle_raw(&mut self, value: zx_handle_t) -> &mut Self {
        self.zircon_handle = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::zircon_handle`]
    pub fn with_zircon_handle_raw(mut self, value: zx_handle_t) -> Self {
        self.zircon_handle = value;
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
    ///Gets the value of [`Self::semaphore`]
    pub fn semaphore(&self) -> Semaphore {
        self.semaphore
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> SemaphoreImportFlags {
        self.flags
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalSemaphoreHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets the value of [`Self::zircon_handle`]
    pub fn zircon_handle(&self) -> zx_handle_t {
        self.zircon_handle
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::semaphore`]
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut SemaphoreImportFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlagBits {
        &mut self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::zircon_handle`]
    pub fn zircon_handle_mut(&mut self) -> &mut zx_handle_t {
        &mut self.zircon_handle
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
    ///Sets the value of [`Self::semaphore`]
    pub fn set_semaphore(&mut self, value: crate::vulkan1_0::Semaphore) -> &mut Self {
        self.semaphore = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::vulkan1_1::SemaphoreImportFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::zircon_handle`]
    pub fn set_zircon_handle(&mut self, value: crate::native::zx_handle_t) -> &mut Self {
        self.zircon_handle = value;
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
    ///Sets the value of [`Self::semaphore`]
    pub fn with_semaphore(mut self, value: crate::vulkan1_0::Semaphore) -> Self {
        self.semaphore = value;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(mut self, value: crate::vulkan1_1::SemaphoreImportFlags) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn with_handle_type(mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
    ///Sets the value of [`Self::zircon_handle`]
    pub fn with_zircon_handle(mut self, value: crate::native::zx_handle_t) -> Self {
        self.zircon_handle = value;
        self
    }
}
///[VkSemaphoreGetZirconHandleInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetZirconHandleInfoFUCHSIA.html) - Structure describing a Zircon event handle semaphore export operation
///# C Specifications
///The [`SemaphoreGetZirconHandleInfoFUCHSIA`] structure is defined as:
///```c
///// Provided by VK_FUCHSIA_external_semaphore
///typedef struct VkSemaphoreGetZirconHandleInfoFUCHSIA {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    VkSemaphore                              semaphore;
///    VkExternalSemaphoreHandleTypeFlagBits    handleType;
///} VkSemaphoreGetZirconHandleInfoFUCHSIA;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore`] is the semaphore from which state will be exported.
/// - [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the type of
///   handle requested.
///# Description
///The properties of the Zircon event handle returned depend on the value of
///[`handle_type`].
///See [`ExternalSemaphoreHandleTypeFlagBits`] for a description of the
///properties of the defined external semaphore handle types.
///## Valid Usage
/// - [`handle_type`] **must**  have been included in [`ExportSemaphoreCreateInfo::handle_types`]
///   when [`semaphore`]’s current payload was created
/// -  [`semaphore`] **must**  not currently have its payload replaced by an imported payload as described below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) unless that imported payload’s handle type was included in [`ExternalSemaphoreProperties::export_from_imported_handle_types`] for [`handle_type`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, as defined below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), there  **must**  be no queue waiting on [`semaphore`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`semaphore`] **must**  be signaled, or have an associated [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) pending execution
/// - [`handle_type`] **must**  be defined as a Zircon event handle
/// - [`semaphore`] **must**  have been created with a [`SemaphoreType`] of
///   `VK_SEMAPHORE_TYPE_BINARY`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
/// - [`p_next`] **must**  be `NULL`
/// - [`semaphore`] **must**  be a valid [`Semaphore`] handle
/// - [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///# Related
/// - [`fuchsia_external_semaphore`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`get_semaphore_zircon_handle_fuchsia`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSemaphoreGetZirconHandleInfoFUCHSIA")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SemaphoreGetZirconHandleInfoFUCHSIA<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`semaphore`] is the semaphore from which state will be exported.
    pub semaphore: Semaphore,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl<'lt> Default for SemaphoreGetZirconHandleInfoFUCHSIA<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> SemaphoreGetZirconHandleInfoFUCHSIA<'lt> {
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
    ///Gets the value of [`Self::semaphore`]
    pub fn semaphore(&self) -> Semaphore {
        self.semaphore
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalSemaphoreHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::semaphore`]
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlagBits {
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
    ///Sets the value of [`Self::semaphore`]
    pub fn set_semaphore(&mut self, value: crate::vulkan1_0::Semaphore) -> &mut Self {
        self.semaphore = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
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
    ///Sets the value of [`Self::semaphore`]
    pub fn with_semaphore(mut self, value: crate::vulkan1_0::Semaphore) -> Self {
        self.semaphore = value;
        self
    }
    ///Sets the value of [`Self::handle_type`]
    pub fn with_handle_type(mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> Self {
        self.handle_type = value;
        self
    }
}
impl Device {
    ///[vkGetSemaphoreZirconHandleFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html) - Get a Zircon event handle for a semaphore
    ///# C Specifications
    ///To export a Zircon event handle representing the payload of a semaphore,
    ///call:
    ///```c
    ///// Provided by VK_FUCHSIA_external_semaphore
    ///VkResult vkGetSemaphoreZirconHandleFUCHSIA(
    ///    VkDevice                                    device,
    ///    const VkSemaphoreGetZirconHandleInfoFUCHSIA* pGetZirconHandleInfo,
    ///    zx_handle_t*                                pZirconHandle);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that created the semaphore being exported.
    /// - [`p_get_zircon_handle_info`] is a pointer to a [`SemaphoreGetZirconHandleInfoFUCHSIA`]
    ///   structure containing parameters of the export operation.
    /// - [`p_zircon_handle`] will return the Zircon event handle representing the semaphore
    ///   payload.
    ///# Description
    ///Each call to [`get_semaphore_zircon_handle_fuchsia`] **must**  create a Zircon
    ///event handle and transfer ownership of it to the application.
    ///To avoid leaking resources, the application  **must**  release ownership of the
    ///Zircon event handle when it is no longer needed.Exporting a Zircon event handle from a
    /// semaphore  **may**  have side effects
    ///depending on the transference of the specified handle type, as described in
    ///[Importing Semaphore State](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing).
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_get_zircon_handle_info`] **must**  be a valid pointer to a valid
    ///   [`SemaphoreGetZirconHandleInfoFUCHSIA`] structure
    /// - [`p_zircon_handle`] **must**  be a valid pointer to a [`zx_handle_t`] value
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`
    ///# Related
    /// - [`fuchsia_external_semaphore`]
    /// - [`Device`]
    /// - [`SemaphoreGetZirconHandleInfoFUCHSIA`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_semaphore_zircon_handle_fuchsia<'lt>(
        self: &Unique<Device>,
        p_get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA<'lt>,
    ) -> VulkanResult<zx_handle_t> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_external_semaphore()
            .and_then(|vtable| vtable.get_semaphore_zircon_handle_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_external_semaphore()
            .and_then(|vtable| vtable.get_semaphore_zircon_handle_fuchsia())
            .unwrap_unchecked();
        let mut p_zircon_handle = std::mem::zeroed();
        let _return = _function(
            self.as_raw(),
            p_get_zircon_handle_info as *const SemaphoreGetZirconHandleInfoFUCHSIA<'lt>,
            &mut p_zircon_handle,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, p_zircon_handle),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkImportSemaphoreZirconHandleFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html) - Import a semaphore from a Zircon event handle
    ///# C Specifications
    ///To import a semaphore payload from a Zircon event handle, call:
    ///```c
    ///// Provided by VK_FUCHSIA_external_semaphore
    ///VkResult vkImportSemaphoreZirconHandleFUCHSIA(
    ///    VkDevice                                    device,
    ///    const VkImportSemaphoreZirconHandleInfoFUCHSIA* pImportSemaphoreZirconHandleInfo);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that created the semaphore.
    /// - [`p_import_semaphore_zircon_handle_info`] is a pointer to a
    ///   [`ImportSemaphoreZirconHandleInfoFUCHSIA`] structure specifying the semaphore and import
    ///   parameters.
    ///# Description
    ///Importing a semaphore payload from a Zircon event handle transfers ownership
    ///of the handle from the application to the Vulkan implementation.
    ///The application  **must**  not perform any operations on the handle after a
    ///successful import.Applications  **can**  import the same semaphore payload into multiple
    /// instances
    ///of Vulkan, into the same instance from which it was exported, and multiple
    ///times into a given Vulkan instance.
    ///## Valid Usage
    /// - `semaphore` **must**  not be associated with any queue command that has not yet completed
    ///   execution on that queue
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_import_semaphore_zircon_handle_info`] **must**  be a valid pointer to a valid
    ///   [`ImportSemaphoreZirconHandleInfoFUCHSIA`] structure
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`
    ///# Related
    /// - [`fuchsia_external_semaphore`]
    /// - [`Device`]
    /// - [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
    #[track_caller]
    #[inline]
    pub unsafe fn import_semaphore_zircon_handle_fuchsia<'lt>(
        self: &Unique<Device>,
        p_import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA<'lt>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .fuchsia_external_semaphore()
            .and_then(|vtable| vtable.import_semaphore_zircon_handle_fuchsia())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .fuchsia_external_semaphore()
            .and_then(|vtable| vtable.import_semaphore_zircon_handle_fuchsia())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_import_semaphore_zircon_handle_info as *const ImportSemaphoreZirconHandleInfoFUCHSIA<'lt>,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_FUCHSIA_external_semaphore`
pub struct DeviceFuchsiaExternalSemaphoreVTable {
    ///See [`FNGetSemaphoreZirconHandleFuchsia`] for more information.
    pub get_semaphore_zircon_handle_fuchsia: FNGetSemaphoreZirconHandleFuchsia,
    ///See [`FNImportSemaphoreZirconHandleFuchsia`] for more information.
    pub import_semaphore_zircon_handle_fuchsia: FNImportSemaphoreZirconHandleFuchsia,
}
impl DeviceFuchsiaExternalSemaphoreVTable {
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
            get_semaphore_zircon_handle_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetSemaphoreZirconHandleFUCHSIA").as_ptr(),
                ))
            },
            import_semaphore_zircon_handle_fuchsia: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkImportSemaphoreZirconHandleFUCHSIA").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_semaphore_zircon_handle_fuchsia`]. See
    /// [`FNGetSemaphoreZirconHandleFuchsia`] for more information.
    pub fn get_semaphore_zircon_handle_fuchsia(&self) -> FNGetSemaphoreZirconHandleFuchsia {
        self.get_semaphore_zircon_handle_fuchsia
    }
    ///Gets [`Self::import_semaphore_zircon_handle_fuchsia`]. See
    /// [`FNImportSemaphoreZirconHandleFuchsia`] for more information.
    pub fn import_semaphore_zircon_handle_fuchsia(&self) -> FNImportSemaphoreZirconHandleFuchsia {
        self.import_semaphore_zircon_handle_fuchsia
    }
}
