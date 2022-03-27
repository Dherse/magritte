use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION")]
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME")]
pub const KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_deferred_host_operations");
///[VkDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeferredOperationKHR.html) - A deferred operation
///# C Specifications
///The [`DeferredOperationKHR`] handle is defined as:
///```c
///// Provided by VK_KHR_deferred_host_operations
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDeferredOperationKHR)
///```
///# Related
/// - [`VK_KHR_deferred_host_operations`]
/// - [`BuildAccelerationStructuresKHR`]
/// - [`CopyAccelerationStructureKHR`]
/// - [`CopyAccelerationStructureToMemoryKHR`]
/// - [`CopyMemoryToAccelerationStructureKHR`]
/// - [`CreateDeferredOperationKHR`]
/// - [`CreateRayTracingPipelinesKHR`]
/// - [`DeferredOperationJoinKHR`]
/// - [`DestroyDeferredOperationKHR`]
/// - [`GetDeferredOperationMaxConcurrencyKHR`]
/// - [`GetDeferredOperationResultKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct DeferredOperationKHR(pub u64);
impl DeferredOperationKHR {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub const fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for DeferredOperationKHR {}
impl Default for DeferredOperationKHR {
    fn default() -> Self {
        Self::default()
    }
}
impl std::fmt::Pointer for DeferredOperationKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
impl std::fmt::Debug for DeferredOperationKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
