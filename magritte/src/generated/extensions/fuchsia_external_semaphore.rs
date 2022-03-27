use crate::{
    native::zx_handle_t,
    vulkan1_0::{BaseInStructure, Semaphore, StructureType},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
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
///The handle types supported by [`handle_type`] are:Valid Usage
/// - [`handle_type`]**must** be a value included in the [Handle Types Supported by [`ImportSemaphoreZirconHandleInfoFUCHSIA`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphore-handletypes-fuchsia)
///   table
/// -  [`zircon_handle`]**must** obey any requirements listed for [`handle_type`] in [external semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
/// - [`zircon_handle`]**must** have `ZX_RIGHTS_BASIC` and `ZX_RIGHTS_SIGNAL` rights
/// - The [`SemaphoreTypeCreateInfo::semaphore_type`] field **must** not be
///   `VK_SEMAPHORE_TYPE_TIMELINE`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
/// - [`semaphore`]**must** be a valid [`Semaphore`] handle
/// - [`flags`]**must** be a valid combination of [`SemaphoreImportFlagBits`] values
/// - [`handle_type`]**must** be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///Host Synchronization
/// - Host access to [`semaphore`]**must** be externally synchronized
///# Related
/// - [`VK_FUCHSIA_external_semaphore`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`SemaphoreImportFlags`]
/// - [`StructureType`]
/// - [`ImportSemaphoreZirconHandleFUCHSIA`]
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
pub struct ImportSemaphoreZirconHandleInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`semaphore`] is the semaphore into which the payload will be
    ///imported.
    semaphore: Semaphore,
    ///[`flags`] is a bitmask of [`SemaphoreImportFlagBits`] specifying
    ///additional parameters for the semaphore payload import operation.
    flags: SemaphoreImportFlags,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the type of [`zircon_handle`].
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
    ///[`zircon_handle`] is the external handle to import.
    zircon_handle: zx_handle_t,
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
///properties of the defined external semaphore handle types.Valid Usage
/// - [`handle_type`]**must** have been included in [`ExportSemaphoreCreateInfo::handle_types`] when
///   [`semaphore`]’s current payload was created
/// -  [`semaphore`]**must** not currently have its payload replaced by an imported payload as described below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing) unless that imported payload’s handle type was included in [`ExternalSemaphoreProperties::export_from_imported_handle_types`] for [`handle_type`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, as defined below in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), there **must** be no queue waiting on [`semaphore`]
/// -    If [`handle_type`] refers to a handle type with copy payload transference semantics, [`semaphore`]**must** be signaled, or have an associated [semaphore signal operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-signaling) pending execution
/// - [`handle_type`]**must** be defined as a Zircon event handle
/// - [`semaphore`]**must** have been created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA`
/// - [`p_next`]**must** be `NULL`
/// - [`semaphore`]**must** be a valid [`Semaphore`] handle
/// - [`handle_type`]**must** be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///# Related
/// - [`VK_FUCHSIA_external_semaphore`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`GetSemaphoreZirconHandleFUCHSIA`]
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
pub struct SemaphoreGetZirconHandleInfoFUCHSIA<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`semaphore`] is the semaphore from which state will be exported.
    semaphore: Semaphore,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the type of handle requested.
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
