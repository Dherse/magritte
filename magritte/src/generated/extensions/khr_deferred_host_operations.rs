//![VK_KHR_deferred_host_operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_deferred_host_operations.html) - device extension
//!# Description
//!The `[`VK_KHR_deferred_host_operations`]` extension defines the
//!infrastructure and usage patterns for deferrable commands, but does not
//!specify any commands as deferrable.
//!This is left to additional dependent extensions.
//!Commands  **must**  not be deferred unless the deferral is specifically allowed
//!by another extension which depends on
//!`[`VK_KHR_deferred_host_operations`]`.
//!# Revision
//!4
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Josh Barczak [jbarczak](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_deferred_host_operations]
//!   @jbarczak%0A<<Here describe the issue or question you have about the
//!   VK_KHR_deferred_host_operations extension>>)
//!# New handles
//! - [`DeferredOperationKHR`]
//!# New functions & commands
//! - [`create_deferred_operation_khr`]
//! - [`deferred_operation_join_khr`]
//! - [`destroy_deferred_operation_khr`]
//! - [`get_deferred_operation_max_concurrency_khr`]
//! - [`get_deferred_operation_result_khr`]
//!# New constants
//! - [`KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME`]
//! - [`KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION`]
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR`
//! - Extending [`VulkanResultCodes`]:  - `VK_OPERATION_DEFERRED_KHR`  -
//!   `VK_OPERATION_NOT_DEFERRED_KHR`  - `VK_THREAD_DONE_KHR`  - `VK_THREAD_IDLE_KHR`
//!# Known issues & F.A.Q
//!0. Should this extension have a VkPhysicalDevice*FeaturesKHR structure?
//! **RESOLVED** : No.
//!This extension does not add any functionality on its own and requires a
//!dependent extension to actually enable functionality and thus there is no
//!value in adding a feature structure.
//!If necessary, any dependent extension could add a feature boolean if it
//!wanted to indicate that it is adding optional deferral support.
//!# Version History
//! - Revision 1, 2019-12-05 (Josh Barczak, Daniel Koch)  - Initial draft.
//! - Revision 2, 2020-03-06 (Daniel Koch, Tobias Hector)  - Add missing
//!   VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR enum  - fix sample code  - Clarified deferred operation
//!   parameter lifetimes (#2018,!3647)
//! - Revision 3, 2020-05-15 (Josh Barczak)  - Clarify behavior of
//!   vkGetDeferredOperationMaxConcurrencyKHR, allowing it to return 0 if the operation is complete
//!   (#2036,!3850)
//! - Revision 4, 2020-11-12 (Tobias Hector, Daniel Koch)  - Remove VkDeferredOperationInfoKHR and
//!   change return value semantics when deferred host operations are in use (#2067,3813)  - clarify
//!   return value of vkGetDeferredOperationResultKHR (#2339,!4110)
//!# Other info
//! * 2020-11-12
//! * No known IP claims.
//! * - Joshua Barczak, Intel  - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA  - Slawek Grajewski, Intel
//!   - Tobias Hector, AMD  - Yuriy O’Donnell, Epic  - Eric Werness, NVIDIA  - Baldur Karlsson,
//!   Valve  - Jesse Barker, Unity  - Contributors to VK_KHR_acceleration_structure,
//!   VK_KHR_ray_tracing_pipeline
//!# Related
//! - [`DeferredOperationKHR`]
//! - [`create_deferred_operation_khr`]
//! - [`deferred_operation_join_khr`]
//! - [`destroy_deferred_operation_khr`]
//! - [`get_deferred_operation_max_concurrency_khr`]
//! - [`get_deferred_operation_result_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    entry::Entry,
    vulkan1_0::{AllocationCallbacks, Device, Instance, PhysicalDevice, VulkanResultCodes},
    AsRaw, Handle, Unique, VulkanResult,
};
use std::{
    ffi::CStr,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION")]
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME")]
pub const KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_deferred_host_operations");
///[vkCreateDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html) - Create a deferred operation handle
///# C Specifications
///To construct the tracking object for a deferred command, call:
///```c
///// Provided by VK_KHR_deferred_host_operations
///VkResult vkCreateDeferredOperationKHR(
///    VkDevice                                    device,
///    const VkAllocationCallbacks*                pAllocator,
///    VkDeferredOperationKHR*                     pDeferredOperation);
///```
/// # Parameters
/// - [`device`] is the device which owns `operation`.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// - [`p_deferred_operation`] is a pointer to a handle in which the created
///   [`DeferredOperationKHR`] is returned.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - [`p_deferred_operation`] **must**  be a valid pointer to a [`DeferredOperationKHR`] handle
///
/// ## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`
/// # Related
/// - [`VK_KHR_deferred_host_operations`]
/// - [`AllocationCallbacks`]
/// - [`DeferredOperationKHR`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCreateDeferredOperationKHR")]
pub type FNCreateDeferredOperationKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        p_allocator: *const AllocationCallbacks<'lt>,
        p_deferred_operation: *mut DeferredOperationKHR,
    ) -> VulkanResultCodes,
>;
///[vkDestroyDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html) - Destroy a deferred operation handle
///# C Specifications
///When a deferred operation is completed, the application  **can**  destroy the
///tracking object by calling:
///```c
///// Provided by VK_KHR_deferred_host_operations
///void vkDestroyDeferredOperationKHR(
///    VkDevice                                    device,
///    VkDeferredOperationKHR                      operation,
///    const VkAllocationCallbacks*                pAllocator);
///```
/// # Parameters
/// - [`device`] is the device which owns [`operation`].
/// - [`operation`] is the completed operation to be destroyed.
/// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
///   chapter.
/// # Description
/// ## Valid Usage
/// - If [`AllocationCallbacks`] were provided when [`operation`] was created, a compatible set of
///   callbacks  **must**  be provided here
/// - If no [`AllocationCallbacks`] were provided when [`operation`] was created, [`p_allocator`]
///   **must**  be `NULL`
/// - [`operation`] **must**  be completed
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - If [`operation`] is not [`crate::Handle::null`], [`operation`] **must**  be a valid
///   [`DeferredOperationKHR`] handle
/// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
///   [`AllocationCallbacks`] structure
/// - If [`operation`] is a valid handle, it  **must**  have been created, allocated, or retrieved
///   from [`device`]
///
/// ## Host Synchronization
/// - Host access to [`operation`] **must**  be externally synchronized
/// # Related
/// - [`VK_KHR_deferred_host_operations`]
/// - [`AllocationCallbacks`]
/// - [`DeferredOperationKHR`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDestroyDeferredOperationKHR")]
pub type FNDestroyDeferredOperationKhr = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        operation: DeferredOperationKHR,
        p_allocator: *const AllocationCallbacks<'lt>,
    ),
>;
///[vkGetDeferredOperationMaxConcurrencyKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) - Query the maximum concurrency on a deferred operation
///# C Specifications
///To query the number of additional threads that can usefully be joined to a
///deferred operation, call:
///```c
///// Provided by VK_KHR_deferred_host_operations
///uint32_t vkGetDeferredOperationMaxConcurrencyKHR(
///    VkDevice                                    device,
///    VkDeferredOperationKHR                      operation);
///```
/// # Parameters
/// - [`device`] is the device which owns [`operation`].
/// - [`operation`] is the deferred operation to be queried.
/// # Description
/// The returned value is the maximum number of threads that can usefully
/// execute a deferred operation concurrently, reported for the state of the
/// deferred operation at the point this command is called.
/// This value is intended to be used to better schedule work onto available
/// threads.
/// Applications  **can**  join any number of threads to the deferred operation and
/// expect it to eventually complete, though excessive joins  **may**  return
/// `VK_THREAD_DONE_KHR` immediately, performing no useful work.If [`operation`] is complete,
/// [`get_deferred_operation_max_concurrency_khr`] returns zero.If [`operation`] is currently joined
/// to any threads, the value returned by
/// this command  **may**  immediately be out of date.If [`operation`] is pending, implementations
/// **must**  not return zero unless
/// at least one thread is currently executing [`deferred_operation_join_khr`]
/// on [`operation`].
/// If there are such threads, the implementation  **should**  return an estimate of
/// the number of additional threads which it could profitably use.Implementations  **may**  return
/// 2<sup>32</sup>-1 to indicate that the maximum
/// concurrency is unknown and cannot be easily derived.
/// Implementations  **may**  return values larger than the maximum concurrency
/// available on the host CPU.
/// In these situations, an application  **should**  clamp the return value rather
/// than oversubscribing the machine.
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
/// - [`operation`] **must**  have been created, allocated, or retrieved from [`device`]
/// # Related
/// - [`VK_KHR_deferred_host_operations`]
/// - [`DeferredOperationKHR`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
pub type FNGetDeferredOperationMaxConcurrencyKhr =
    Option<unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> u32>;
///[vkGetDeferredOperationResultKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html) - Query the result of a deferred operation
///# C Specifications
///The [`get_deferred_operation_result_khr`] function is defined as:
///```c
///// Provided by VK_KHR_deferred_host_operations
///VkResult vkGetDeferredOperationResultKHR(
///    VkDevice                                    device,
///    VkDeferredOperationKHR                      operation);
///```
/// # Parameters
/// - [`device`] is the device which owns [`operation`].
/// - [`operation`] is the operation whose deferred result is being queried.
/// # Description
/// If no command has been deferred on [`operation`],
/// [`get_deferred_operation_result_khr`] returns `VK_SUCCESS`.If the deferred operation is pending,
/// [`get_deferred_operation_result_khr`]
/// returns `VK_NOT_READY`.If the deferred operation is complete, it returns the appropriate return
/// value from the original command.
/// This value  **must**  be one of the [`VulkanResultCodes`] values which could have been
/// returned by the original command if the operation had not been deferred.
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
/// - [`operation`] **must**  have been created, allocated, or retrieved from [`device`]
///
/// ## Return Codes
/// * - `VK_SUCCESS`  - `VK_NOT_READY`
/// # Related
/// - [`VK_KHR_deferred_host_operations`]
/// - [`DeferredOperationKHR`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDeferredOperationResultKHR")]
pub type FNGetDeferredOperationResultKhr =
    Option<unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> VulkanResultCodes>;
///[vkDeferredOperationJoinKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html) - Assign a thread to a deferred operation
///# C Specifications
///To assign a thread to a deferred operation, call:
///```c
///// Provided by VK_KHR_deferred_host_operations
///VkResult vkDeferredOperationJoinKHR(
///    VkDevice                                    device,
///    VkDeferredOperationKHR                      operation);
///```
/// # Parameters
/// - [`device`] is the device which owns [`operation`].
/// - [`operation`] is the deferred operation that the calling thread should work on.
/// # Description
/// The [`deferred_operation_join_khr`] command will execute a portion of the
/// deferred operation on the calling thread.The return value will be one of the following:
/// - A return value of `VK_SUCCESS` indicates that [`operation`] is complete. The application
///   **should**  use [`get_deferred_operation_result_khr`] to retrieve the result of [`operation`].
/// - A return value of `VK_THREAD_DONE_KHR` indicates that the deferred operation is not complete,
///   but there is no work remaining to assign to threads. Future calls to
///   [`deferred_operation_join_khr`] are not necessary and will simply harm performance. This
///   situation  **may**  occur when other threads executing [`deferred_operation_join_khr`] are
///   about to complete [`operation`], and the implementation is unable to partition the workload
///   any further.
/// - A return value of `VK_THREAD_IDLE_KHR` indicates that the deferred operation is not complete,
///   and there is no work for the thread to do at the time of the call. This situation  **may**
///   occur if the operation encounters a temporary reduction in parallelism. By returning
///   `VK_THREAD_IDLE_KHR`, the implementation is signaling that it expects that more opportunities
///   for parallelism will emerge as execution progresses, and that future calls to
///   [`deferred_operation_join_khr`] **can**  be beneficial. In the meantime, the application
///   **can**  perform other work on the calling thread.
/// Implementations  **must**  guarantee forward progress by enforcing the following
/// invariants:
/// 0. If only one thread has invoked [`deferred_operation_join_khr`] on a given operation, that
/// thread  **must**  execute the operation to completion and return `VK_SUCCESS`.
/// 1. If multiple threads have concurrently invoked [`deferred_operation_join_khr`] on the same
/// operation, then at least one of them  **must**  complete the operation and return `VK_SUCCESS`.
///
/// ## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
/// - [`operation`] **must**  have been created, allocated, or retrieved from [`device`]
///
/// ## Return Codes
/// * - `VK_SUCCESS`  - `VK_THREAD_DONE_KHR`  - `VK_THREAD_IDLE_KHR`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
/// # Related
/// - [`VK_KHR_deferred_host_operations`]
/// - [`DeferredOperationKHR`]
/// - [`Device`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkDeferredOperationJoinKHR")]
pub type FNDeferredOperationJoinKhr =
    Option<unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> VulkanResultCodes>;
impl Device {
    ///[vkCreateDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html) - Create a deferred operation handle
    ///# C Specifications
    ///To construct the tracking object for a deferred command, call:
    ///```c
    ///// Provided by VK_KHR_deferred_host_operations
    ///VkResult vkCreateDeferredOperationKHR(
    ///    VkDevice                                    device,
    ///    const VkAllocationCallbacks*                pAllocator,
    ///    VkDeferredOperationKHR*                     pDeferredOperation);
    ///```
    /// # Parameters
    /// - [`device`] is the device which owns `operation`.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// - [`p_deferred_operation`] is a pointer to a handle in which the created
    ///   [`DeferredOperationKHR`] is returned.
    /// # Description
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - [`p_deferred_operation`] **must**  be a valid pointer to a [`DeferredOperationKHR`] handle
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`
    /// # Related
    /// - [`VK_KHR_deferred_host_operations`]
    /// - [`AllocationCallbacks`]
    /// - [`DeferredOperationKHR`]
    /// - [`Device`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCreateDeferredOperationKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn create_deferred_operation_khr<'lt>(
        self: &Unique<Device>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> VulkanResult<Unique<DeferredOperationKHR>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.create_deferred_operation_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.create_deferred_operation_khr())
            .unwrap_unchecked();
        let mut p_deferred_operation = MaybeUninit::<DeferredOperationKHR>::uninit();
        let _return = _function(
            self.as_raw(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
            p_deferred_operation.as_mut_ptr(),
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(
                _return,
                Unique::new(self, p_deferred_operation.assume_init(), AtomicBool::default()),
            ),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDestroyDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html) - Destroy a deferred operation handle
    ///# C Specifications
    ///When a deferred operation is completed, the application  **can**  destroy the
    ///tracking object by calling:
    ///```c
    ///// Provided by VK_KHR_deferred_host_operations
    ///void vkDestroyDeferredOperationKHR(
    ///    VkDevice                                    device,
    ///    VkDeferredOperationKHR                      operation,
    ///    const VkAllocationCallbacks*                pAllocator);
    ///```
    /// # Parameters
    /// - [`device`] is the device which owns [`operation`].
    /// - [`operation`] is the completed operation to be destroyed.
    /// - [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation)
    ///   chapter.
    /// # Description
    /// ## Valid Usage
    /// - If [`AllocationCallbacks`] were provided when [`operation`] was created, a compatible set
    ///   of callbacks  **must**  be provided here
    /// - If no [`AllocationCallbacks`] were provided when [`operation`] was created,
    ///   [`p_allocator`] **must**  be `NULL`
    /// - [`operation`] **must**  be completed
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - If [`operation`] is not [`crate::Handle::null`], [`operation`] **must**  be a valid
    ///   [`DeferredOperationKHR`] handle
    /// - If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid
    ///   [`AllocationCallbacks`] structure
    /// - If [`operation`] is a valid handle, it  **must**  have been created, allocated, or
    ///   retrieved from [`device`]
    ///
    /// ## Host Synchronization
    /// - Host access to [`operation`] **must**  be externally synchronized
    /// # Related
    /// - [`VK_KHR_deferred_host_operations`]
    /// - [`AllocationCallbacks`]
    /// - [`DeferredOperationKHR`]
    /// - [`Device`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDestroyDeferredOperationKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn destroy_deferred_operation_khr<'lt>(
        self: &Unique<Device>,
        operation: Option<DeferredOperationKHR>,
        p_allocator: Option<&AllocationCallbacks<'lt>>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.destroy_deferred_operation_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.destroy_deferred_operation_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            operation.unwrap_or_default(),
            p_allocator
                .map(|v| v as *const AllocationCallbacks<'lt>)
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl Device {
    ///[vkGetDeferredOperationMaxConcurrencyKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html) - Query the maximum concurrency on a deferred operation
    ///# C Specifications
    ///To query the number of additional threads that can usefully be joined to a
    ///deferred operation, call:
    ///```c
    ///// Provided by VK_KHR_deferred_host_operations
    ///uint32_t vkGetDeferredOperationMaxConcurrencyKHR(
    ///    VkDevice                                    device,
    ///    VkDeferredOperationKHR                      operation);
    ///```
    /// # Parameters
    /// - [`device`] is the device which owns [`operation`].
    /// - [`operation`] is the deferred operation to be queried.
    /// # Description
    /// The returned value is the maximum number of threads that can usefully
    /// execute a deferred operation concurrently, reported for the state of the
    /// deferred operation at the point this command is called.
    /// This value is intended to be used to better schedule work onto available
    /// threads.
    /// Applications  **can**  join any number of threads to the deferred operation and
    /// expect it to eventually complete, though excessive joins  **may**  return
    /// `VK_THREAD_DONE_KHR` immediately, performing no useful work.If [`operation`] is complete,
    /// [`get_deferred_operation_max_concurrency_khr`] returns zero.If [`operation`] is currently
    /// joined to any threads, the value returned by
    /// this command  **may**  immediately be out of date.If [`operation`] is pending,
    /// implementations  **must**  not return zero unless
    /// at least one thread is currently executing [`deferred_operation_join_khr`]
    /// on [`operation`].
    /// If there are such threads, the implementation  **should**  return an estimate of
    /// the number of additional threads which it could profitably use.Implementations  **may**
    /// return 2<sup>32</sup>-1 to indicate that the maximum
    /// concurrency is unknown and cannot be easily derived.
    /// Implementations  **may**  return values larger than the maximum concurrency
    /// available on the host CPU.
    /// In these situations, an application  **should**  clamp the return value rather
    /// than oversubscribing the machine.
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
    /// - [`operation`] **must**  have been created, allocated, or retrieved from [`device`]
    /// # Related
    /// - [`VK_KHR_deferred_host_operations`]
    /// - [`DeferredOperationKHR`]
    /// - [`Device`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDeferredOperationMaxConcurrencyKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_deferred_operation_max_concurrency_khr(
        self: &Unique<Device>,
        operation: DeferredOperationKHR,
    ) -> u32 {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.get_deferred_operation_max_concurrency_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.get_deferred_operation_max_concurrency_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), operation);
        _return
    }
}
impl Device {
    ///[vkGetDeferredOperationResultKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html) - Query the result of a deferred operation
    ///# C Specifications
    ///The [`get_deferred_operation_result_khr`] function is defined as:
    ///```c
    ///// Provided by VK_KHR_deferred_host_operations
    ///VkResult vkGetDeferredOperationResultKHR(
    ///    VkDevice                                    device,
    ///    VkDeferredOperationKHR                      operation);
    ///```
    /// # Parameters
    /// - [`device`] is the device which owns [`operation`].
    /// - [`operation`] is the operation whose deferred result is being queried.
    /// # Description
    /// If no command has been deferred on [`operation`],
    /// [`get_deferred_operation_result_khr`] returns `VK_SUCCESS`.If the deferred operation is
    /// pending, [`get_deferred_operation_result_khr`]
    /// returns `VK_NOT_READY`.If the deferred operation is complete, it returns the appropriate
    /// return
    /// value from the original command.
    /// This value  **must**  be one of the [`VulkanResultCodes`] values which could have been
    /// returned by the original command if the operation had not been deferred.
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
    /// - [`operation`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`  - `VK_NOT_READY`
    /// # Related
    /// - [`VK_KHR_deferred_host_operations`]
    /// - [`DeferredOperationKHR`]
    /// - [`Device`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDeferredOperationResultKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_deferred_operation_result_khr(
        self: &Unique<Device>,
        operation: DeferredOperationKHR,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.get_deferred_operation_result_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.get_deferred_operation_result_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), operation);
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::NOT_READY => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkDeferredOperationJoinKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html) - Assign a thread to a deferred operation
    ///# C Specifications
    ///To assign a thread to a deferred operation, call:
    ///```c
    ///// Provided by VK_KHR_deferred_host_operations
    ///VkResult vkDeferredOperationJoinKHR(
    ///    VkDevice                                    device,
    ///    VkDeferredOperationKHR                      operation);
    ///```
    /// # Parameters
    /// - [`device`] is the device which owns [`operation`].
    /// - [`operation`] is the deferred operation that the calling thread should work on.
    /// # Description
    /// The [`deferred_operation_join_khr`] command will execute a portion of the
    /// deferred operation on the calling thread.The return value will be one of the following:
    /// - A return value of `VK_SUCCESS` indicates that [`operation`] is complete. The application
    ///   **should**  use [`get_deferred_operation_result_khr`] to retrieve the result of
    ///   [`operation`].
    /// - A return value of `VK_THREAD_DONE_KHR` indicates that the deferred operation is not
    ///   complete, but there is no work remaining to assign to threads. Future calls to
    ///   [`deferred_operation_join_khr`] are not necessary and will simply harm performance. This
    ///   situation  **may**  occur when other threads executing [`deferred_operation_join_khr`] are
    ///   about to complete [`operation`], and the implementation is unable to partition the
    ///   workload any further.
    /// - A return value of `VK_THREAD_IDLE_KHR` indicates that the deferred operation is not
    ///   complete, and there is no work for the thread to do at the time of the call. This
    ///   situation  **may**  occur if the operation encounters a temporary reduction in
    ///   parallelism. By returning `VK_THREAD_IDLE_KHR`, the implementation is signaling that it
    ///   expects that more opportunities for parallelism will emerge as execution progresses, and
    ///   that future calls to [`deferred_operation_join_khr`] **can**  be beneficial. In the
    ///   meantime, the application  **can**  perform other work on the calling thread.
    /// Implementations  **must**  guarantee forward progress by enforcing the following
    /// invariants:
    /// 0. If only one thread has invoked [`deferred_operation_join_khr`] on a given operation, that
    /// thread  **must**  execute the operation to completion and return `VK_SUCCESS`.
    /// 1. If multiple threads have concurrently invoked [`deferred_operation_join_khr`] on the same
    /// operation, then at least one of them  **must**  complete the operation and return
    /// `VK_SUCCESS`.
    ///
    /// ## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
    /// - [`operation`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    /// ## Return Codes
    /// * - `VK_SUCCESS`  - `VK_THREAD_DONE_KHR`  - `VK_THREAD_IDLE_KHR`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    /// # Related
    /// - [`VK_KHR_deferred_host_operations`]
    /// - [`DeferredOperationKHR`]
    /// - [`Device`]
    ///
    /// # Notes and documentation
    /// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    /// This documentation is generated from the Vulkan specification and documentation.
    /// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    /// This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkDeferredOperationJoinKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn deferred_operation_join_khr(
        self: &Unique<Device>,
        operation: DeferredOperationKHR,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.deferred_operation_join_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_deferred_host_operations()
            .and_then(|vtable| vtable.deferred_operation_join_khr())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), operation);
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::THREAD_DONE_KHR | VulkanResultCodes::THREAD_IDLE_KHR => {
                VulkanResult::Success(_return, ())
            },
            e => VulkanResult::Err(e),
        }
    }
}
///[VkDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeferredOperationKHR.html) - A deferred operation
///# C Specifications
///The [`DeferredOperationKHR`] handle is defined as:
///```c
///// Provided by VK_KHR_deferred_host_operations
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDeferredOperationKHR)
///```
/// # Related
/// - [`VK_KHR_deferred_host_operations`]
/// - [`build_acceleration_structures_khr`]
/// - [`copy_acceleration_structure_khr`]
/// - [`copy_acceleration_structure_to_memory_khr`]
/// - [`copy_memory_to_acceleration_structure_khr`]
/// - [`create_deferred_operation_khr`]
/// - [`create_ray_tracing_pipelines_khr`]
/// - [`deferred_operation_join_khr`]
/// - [`destroy_deferred_operation_khr`]
/// - [`get_deferred_operation_max_concurrency_khr`]
/// - [`get_deferred_operation_result_khr`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeferredOperationKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
    pub fn is_null(&self) -> bool {
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
        Self::null()
    }
}
impl Handle for DeferredOperationKHR {
    type Parent = Unique<Device>;
    type VTable = ();
    type Metadata = AtomicBool;
    type Raw = u64;
    #[inline]
    fn as_raw(self) -> Self::Raw {
        self.0
    }
    #[inline]
    unsafe fn from_raw(this: Self::Raw) -> Self {
        Self(this)
    }
    #[inline]
    #[track_caller]
    unsafe fn destroy(self: &mut Unique<Self>) {
        if !self.metadata().load(Ordering::Acquire) {
            self.device()
                .destroy_deferred_operation_khr(Some(self.as_raw().coerce()), None);
        }
    }
    #[inline]
    unsafe fn load_vtable(&self, _: &Self::Parent, _: &Self::Metadata) -> Self::VTable {}
}
impl Unique<DeferredOperationKHR> {
    ///Gets the reference to the [`Entry`]
    #[inline]
    pub fn entry(&self) -> &Arc<Entry> {
        self.parent().parent().parent().parent()
    }
    ///Gets the reference to the [`Instance`]
    #[inline]
    pub fn instance(&self) -> &Unique<Instance> {
        self.parent().parent().parent()
    }
    ///Gets the reference to the [`PhysicalDevice`]
    #[inline]
    pub fn physical_device(&self) -> &Unique<PhysicalDevice> {
        self.parent().parent()
    }
    ///Gets the reference to the [`Device`]
    #[inline]
    pub fn device(&self) -> &Unique<Device> {
        self.parent()
    }
    ///Disables the base dropping behaviour of this handle
    #[inline]
    pub fn disable_drop(&self) {
        self.metadata().store(true, Ordering::Relaxed);
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_deferred_host_operations`
pub struct DeviceKhrDeferredHostOperationsVTable {
    ///See [`FNCreateDeferredOperationKhr`] for more information.
    pub create_deferred_operation_khr: FNCreateDeferredOperationKhr,
    ///See [`FNDestroyDeferredOperationKhr`] for more information.
    pub destroy_deferred_operation_khr: FNDestroyDeferredOperationKhr,
    ///See [`FNGetDeferredOperationMaxConcurrencyKhr`] for more information.
    pub get_deferred_operation_max_concurrency_khr: FNGetDeferredOperationMaxConcurrencyKhr,
    ///See [`FNGetDeferredOperationResultKhr`] for more information.
    pub get_deferred_operation_result_khr: FNGetDeferredOperationResultKhr,
    ///See [`FNDeferredOperationJoinKhr`] for more information.
    pub deferred_operation_join_khr: FNDeferredOperationJoinKhr,
}
impl DeviceKhrDeferredHostOperationsVTable {
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
            create_deferred_operation_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCreateDeferredOperationKHR").as_ptr()))
            },
            destroy_deferred_operation_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkDestroyDeferredOperationKHR").as_ptr(),
                ))
            },
            get_deferred_operation_max_concurrency_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeferredOperationMaxConcurrencyKHR").as_ptr(),
                ))
            },
            get_deferred_operation_result_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeferredOperationResultKHR").as_ptr(),
                ))
            },
            deferred_operation_join_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkDeferredOperationJoinKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::create_deferred_operation_khr`]. See [`FNCreateDeferredOperationKhr`] for more
    /// information.
    pub fn create_deferred_operation_khr(&self) -> FNCreateDeferredOperationKhr {
        self.create_deferred_operation_khr
    }
    ///Gets [`Self::destroy_deferred_operation_khr`]. See [`FNDestroyDeferredOperationKhr`] for
    /// more information.
    pub fn destroy_deferred_operation_khr(&self) -> FNDestroyDeferredOperationKhr {
        self.destroy_deferred_operation_khr
    }
    ///Gets [`Self::get_deferred_operation_max_concurrency_khr`]. See
    /// [`FNGetDeferredOperationMaxConcurrencyKhr`] for more information.
    pub fn get_deferred_operation_max_concurrency_khr(&self) -> FNGetDeferredOperationMaxConcurrencyKhr {
        self.get_deferred_operation_max_concurrency_khr
    }
    ///Gets [`Self::get_deferred_operation_result_khr`]. See [`FNGetDeferredOperationResultKhr`]
    /// for more information.
    pub fn get_deferred_operation_result_khr(&self) -> FNGetDeferredOperationResultKhr {
        self.get_deferred_operation_result_khr
    }
    ///Gets [`Self::deferred_operation_join_khr`]. See [`FNDeferredOperationJoinKhr`] for more
    /// information.
    pub fn deferred_operation_join_khr(&self) -> FNDeferredOperationJoinKhr {
        self.deferred_operation_join_khr
    }
}
