//!# [VK_FUCHSIA_external_semaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_external_semaphore.html)
# ! [doc = include_str ! ("../../../../doc/extensions/fuchsia_external_semaphore/VK_FUCHSIA_external_semaphore.md")]
use crate::{
    cstr,
    opaque::zx_handle_t,
    vulkan1_0::{BaseInStructure, Device, Semaphore, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
};
use std::ffi::CStr;
///# [VkImportSemaphoreZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportSemaphoreZirconHandleInfoFUCHSIA.html)
# [doc = include_str ! ("../../../../doc/extensions/fuchsia_external_semaphore/VkImportSemaphoreZirconHandleInfoFUCHSIA.md")]
#[doc(alias = "VkImportSemaphoreZirconHandleInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportSemaphoreZirconHandleInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    semaphore: Semaphore,
    flags: SemaphoreImportFlags,
    #[doc(alias = "handleType")]
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
    #[doc(alias = "zirconHandle")]
    zircon_handle: zx_handle_t,
}
///# [VkSemaphoreGetZirconHandleInfoFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSemaphoreGetZirconHandleInfoFUCHSIA.html)
# [doc = include_str ! ("../../../../doc/extensions/fuchsia_external_semaphore/VkSemaphoreGetZirconHandleInfoFUCHSIA.md")]
#[doc(alias = "VkSemaphoreGetZirconHandleInfoFUCHSIA")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreGetZirconHandleInfoFUCHSIA {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    semaphore: Semaphore,
    #[doc(alias = "handleType")]
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static CStr = cstr!("VK_FUCHSIA_external_semaphore");
///# [vkGetSemaphoreZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html)
# [doc = include_str ! ("../../../../doc/extensions/fuchsia_external_semaphore/vkGetSemaphoreZirconHandleFUCHSIA.md")]
#[doc(alias = "vkGetSemaphoreZirconHandleFUCHSIA")]
pub type FNGetSemaphoreZirconHandleFuchsia = unsafe extern "system" fn(
    device: Device,
    p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
    p_zircon_handle: *mut zx_handle_t,
) -> VulkanResultCodes;
///# [vkImportSemaphoreZirconHandleFUCHSIA](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html)
# [doc = include_str ! ("../../../../doc/extensions/fuchsia_external_semaphore/vkImportSemaphoreZirconHandleFUCHSIA.md")]
#[doc(alias = "vkImportSemaphoreZirconHandleFUCHSIA")]
pub type FNImportSemaphoreZirconHandleFuchsia = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
) -> VulkanResultCodes;
