//!# [VK_KHR_deferred_host_operations](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_deferred_host_operations.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_deferred_host_operations/VK_KHR_deferred_host_operations.md")]
use crate::{
    cstr,
    vulkan1_0::{AllocationCallbacks, Device, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeferredOperationKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_deferred_host_operations/VkDeferredOperationKHR.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDeferredOperationKHR")]
#[repr(transparent)]
pub struct DeferredOperationKHR(u64);
impl DeferredOperationKHR {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for DeferredOperationKHR {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION")]
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME")]
pub const KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_deferred_host_operations");
///# [vkCreateDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_deferred_host_operations/vkCreateDeferredOperationKHR.md")]
#[doc(alias = "vkCreateDeferredOperationKHR")]
pub type FNCreateDeferredOperationKhr = unsafe extern "system" fn(
    device: Device,
    p_allocator: *const AllocationCallbacks,
    p_deferred_operation: *mut DeferredOperationKHR,
) -> VulkanResultCodes;
///# [vkDestroyDeferredOperationKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_deferred_host_operations/vkDestroyDeferredOperationKHR.md")]
#[doc(alias = "vkDestroyDeferredOperationKHR")]
pub type FNDestroyDeferredOperationKhr =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR, p_allocator: *const AllocationCallbacks);
///# [vkGetDeferredOperationMaxConcurrencyKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_deferred_host_operations/vkGetDeferredOperationMaxConcurrencyKHR.md")]
#[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
pub type FNGetDeferredOperationMaxConcurrencyKhr =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> u32;
///# [vkGetDeferredOperationResultKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_deferred_host_operations/vkGetDeferredOperationResultKHR.md")]
#[doc(alias = "vkGetDeferredOperationResultKHR")]
pub type FNGetDeferredOperationResultKhr =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> VulkanResultCodes;
///# [vkDeferredOperationJoinKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_deferred_host_operations/vkDeferredOperationJoinKHR.md")]
#[doc(alias = "vkDeferredOperationJoinKHR")]
pub type FNDeferredOperationJoinKhr =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> VulkanResultCodes;
