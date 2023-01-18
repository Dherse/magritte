use crate::{
    cstr,
    vulkan1_0::{AllocationCallbacks, Device, VulkanResultCodes},
};
use std::ffi::CStr;
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
#[doc(alias = "vkCreateDeferredOperationKHR")]
pub type FNCreateDeferredOperationKhr = unsafe extern "system" fn(
    device: Device,
    p_allocator: *const AllocationCallbacks,
    p_deferred_operation: *mut DeferredOperationKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyDeferredOperationKHR")]
pub type FNDestroyDeferredOperationKhr =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
pub type FNGetDeferredOperationMaxConcurrencyKhr =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> u32;
#[doc(alias = "vkGetDeferredOperationResultKHR")]
pub type FNGetDeferredOperationResultKhr =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> VulkanResultCodes;
#[doc(alias = "vkDeferredOperationJoinKHR")]
pub type FNDeferredOperationJoinKhr =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> VulkanResultCodes;
