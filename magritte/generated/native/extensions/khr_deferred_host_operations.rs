use crate::native::vulkan1_0::{AllocationCallbacks, Device, VulkanResultCodes};
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDeferredOperationKHR")]
#[repr(transparent)]
pub struct DeferredOperationKHR(u64);
impl DeferredOperationKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DeferredOperationKHR {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::khr_deferred_host_operations::{
    KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME, KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION,
};
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
