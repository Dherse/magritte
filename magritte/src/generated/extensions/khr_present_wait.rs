//![VK_KHR_present_wait](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_present_wait.html) - device extension
//!# Description
//!This device extension allows an application that uses the
//!`[`khr_swapchain`]` extension to wait for present operations to
//!complete.
//!An application can use this to monitor and control the pacing of the
//!application by managing the number of outstanding images yet to be
//!presented.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_swapchain`]`
//! - Requires `[`khr_present_id`]`
//!# Contacts
//! - Keith Packard [keithp](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_present_wait]
//!   @keithp%0A<<Here describe the issue or question you have about the VK_KHR_present_wait
//!   extension>>)
//!# New functions & commands
//! - [`wait_for_present_khr`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePresentWaitFeaturesKHR`]
//!# New constants
//! - [`KHR_PRESENT_WAIT_EXTENSION_NAME`]
//! - [`KHR_PRESENT_WAIT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR`
//!# Known issues & F.A.Q
//!1) When does the wait finish? **RESOLVED** .
//!The wait will finish when the present is visible to the user.
//!There is no requirement for any precise timing relationship between the
//!presentation of the image to the user, but implementations  **should**  signal
//!the wait as close as possible to the presentation of the first pixel in the
//!new image to the user.2) Should this use fences or other existing synchronization mechanism.
//! **RESOLVED** .
//!Because display and rendering are often implemented in separate drivers,
//!this extension will provide a separate synchronization API.3) Should this extension share
//! present identification with other extensions? **RESOLVED** .
//!Yes.
//!A new extension, VK_KHR_present_id, should be created to provide a shared
//!structure for presentation identifiers.4) What happens when presentations complete out of order
//! wrt calls to
//!vkQueuePresent? This could happen if the semaphores for the presentations
//!were ready out of order. **OPTION A** : Require that when a PresentId is set that the driver
//! ensure that
//!images are always presented in the order of calls to vkQueuePresent. **OPTION B** : Finish both
//! waits when the earliest present completes.
//!This will complete the later present wait earlier than the actual
//!presentation.
//!This should be the easiest to implement as the driver need only track the
//!largest present ID completed.
//!This is also the 'natural' consequence of interpreting the existing
//!vkWaitForPresentKHR specificationn. **OPTION C** : Finish both waits when both have completed.
//!This will complete the earlier presentation later than the actual
//!presentation time.
//!This is allowed by the current specification as there is no precise timing
//!requirement for when the presentId value is updated.
//!This requires slightly more complexity in the driver as it will need to
//!track all outstanding presentId values.
//!# Version History
//! - Revision 1, 2019-02-19 (Keith Packard)  - Initial version
//!# Other info
//! * 2019-05-15
//! * No known IP claims.
//! * - Keith Packard, Valve  - Ian Elliott, Google  - Tobias Hector, AMD  - Daniel Stone, Collabora
//!# Related
//! - [`PhysicalDevicePresentWaitFeaturesKHR`]
//! - [`wait_for_present_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseOutStructure, Bool32, Device, StructureType, VulkanResultCodes},
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PRESENT_WAIT_SPEC_VERSION")]
pub const KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PRESENT_WAIT_EXTENSION_NAME")]
pub const KHR_PRESENT_WAIT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_present_wait");
///[vkWaitForPresentKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html) - Wait for presentation
///# C Specifications
///When the [`presentWait`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentWait) feature is enabled, an
///application  **can**  wait for an image to be presented to the user by first
///specifying a presentId for the target presentation by adding a
///[`PresentIdKHR`] structure to the `pNext` chain of the
///[`PresentInfoKHR`] structure and then waiting for that presentation to
///complete by calling:
///```c
///// Provided by VK_KHR_present_wait
///VkResult vkWaitForPresentKHR(
///    VkDevice                                    device,
///    VkSwapchainKHR                              swapchain,
///    uint64_t                                    presentId,
///    uint64_t                                    timeout);
///```
///# Parameters
/// - [`device`] is the device associated with [`swapchain`].
/// - [`swapchain`] is the non-retired swapchain on which an image was queued for presentation.
/// - [`present_id`] is the presentation presentId to wait for.
/// - [`timeout`] is the timeout period in units of nanoseconds. [`timeout`] is adjusted to the
///   closest value allowed by the implementation-dependent timeout accuracy, which  **may**  be
///   substantially longer than one nanosecond, and  **may**  be longer than the requested period.
///# Description
///[`wait_for_present_khr`] waits for the presentId associated with
///[`swapchain`] to be increased in value so that it is at least equal to
///[`present_id`].For `VK_PRESENT_MODE_MAILBOX_KHR` (or other present mode where images
///may be replaced in the presentation queue) any wait of this type associated
///with such an image  **must**  be signaled no later than a wait associated with
///the replacing image would be signaled.When the presentation has completed, the presentId
/// associated with the
///related `pSwapchains` entry will be increased in value so that it is at
///least equal to the value provided in the [`PresentIdKHR`] structure.There is no requirement for
/// any precise timing relationship between the
///presentation of the image to the user and the update of the presentId value,
///but implementations  **should**  make this as close as possible to the
///presentation of the first pixel in the new image to the user.The call to
/// [`wait_for_present_khr`] will block until either the presentId
///associated with [`swapchain`] is greater than or equal to [`present_id`],
///or [`timeout`] nanoseconds passes.
///When the swapchain becomes OUT_OF_DATE, the call will either return
///`VK_SUCCESS` (if the image was delivered to the presentation engine and
///may have been presented to the user) or will return early with status
///`VK_ERROR_OUT_OF_DATE_KHR` (if the image was not presented to the user).As an exception to the
/// normal rules for objects which are externally
///synchronized, the [`swapchain`] passed to [`wait_for_present_khr`] **may**
///be simultaneously used by other threads in calls to functions other than
///[`destroy_swapchain_khr`].
///Access to the swapchain data associated with this extension  **must**  be atomic
///within the implementation.
///## Valid Usage
/// - [`swapchain`] **must**  not be in the retired state
/// - The [`presentWait`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentWait)
///   feature  **must**  be enabled
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
/// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
///   from the same [`Instance`]
///
///## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_TIMEOUT`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`
///# Related
/// - [`khr_present_wait`]
/// - [`Device`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkWaitForPresentKHR")]
pub type FNWaitForPresentKhr = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> VulkanResultCodes,
>;
///[VkPhysicalDevicePresentWaitFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePresentWaitFeaturesKHR.html) - Structure indicating support for present wait
///# C Specifications
///The [`PhysicalDevicePresentWaitFeaturesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_present_wait
///typedef struct VkPhysicalDevicePresentWaitFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           presentWait;
///} VkPhysicalDevicePresentWaitFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`present_wait`] indicates that the implementation supports [`wait_for_present_khr`].
///If the [`PhysicalDevicePresentWaitFeaturesKHR`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePresentWaitFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR`
///# Related
/// - [`khr_present_wait`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDevicePresentWaitFeaturesKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDevicePresentWaitFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`present_wait`] indicates that the
    ///implementation supports [`wait_for_present_khr`].
    pub present_wait: Bool32,
}
impl<'lt> Default for PhysicalDevicePresentWaitFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR,
            p_next: std::ptr::null_mut(),
            present_wait: 0,
        }
    }
}
impl<'lt> PhysicalDevicePresentWaitFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::present_wait`]
    pub fn present_wait_raw(&self) -> Bool32 {
        self.present_wait
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::present_wait`]
    pub fn set_present_wait_raw(mut self, value: Bool32) -> Self {
        self.present_wait = value;
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::present_wait`]
    pub fn present_wait(&self) -> bool {
        unsafe { std::mem::transmute(self.present_wait as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::present_wait`]
    pub fn present_wait_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.present_wait as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.present_wait as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::present_wait`]
    pub fn set_present_wait(mut self, value: bool) -> Self {
        self.present_wait = value as u8 as u32;
        self
    }
}
impl Device {
    ///[vkWaitForPresentKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html) - Wait for presentation
    ///# C Specifications
    ///When the [`presentWait`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentWait) feature is enabled, an
    ///application  **can**  wait for an image to be presented to the user by first
    ///specifying a presentId for the target presentation by adding a
    ///[`PresentIdKHR`] structure to the `pNext` chain of the
    ///[`PresentInfoKHR`] structure and then waiting for that presentation to
    ///complete by calling:
    ///```c
    ///// Provided by VK_KHR_present_wait
    ///VkResult vkWaitForPresentKHR(
    ///    VkDevice                                    device,
    ///    VkSwapchainKHR                              swapchain,
    ///    uint64_t                                    presentId,
    ///    uint64_t                                    timeout);
    ///```
    ///# Parameters
    /// - [`device`] is the device associated with [`swapchain`].
    /// - [`swapchain`] is the non-retired swapchain on which an image was queued for presentation.
    /// - [`present_id`] is the presentation presentId to wait for.
    /// - [`timeout`] is the timeout period in units of nanoseconds. [`timeout`] is adjusted to the
    ///   closest value allowed by the implementation-dependent timeout accuracy, which  **may**  be
    ///   substantially longer than one nanosecond, and  **may**  be longer than the requested
    ///   period.
    ///# Description
    ///[`wait_for_present_khr`] waits for the presentId associated with
    ///[`swapchain`] to be increased in value so that it is at least equal to
    ///[`present_id`].For `VK_PRESENT_MODE_MAILBOX_KHR` (or other present mode where images
    ///may be replaced in the presentation queue) any wait of this type associated
    ///with such an image  **must**  be signaled no later than a wait associated with
    ///the replacing image would be signaled.When the presentation has completed, the presentId
    /// associated with the
    ///related `pSwapchains` entry will be increased in value so that it is at
    ///least equal to the value provided in the [`PresentIdKHR`] structure.There is no requirement
    /// for any precise timing relationship between the
    ///presentation of the image to the user and the update of the presentId value,
    ///but implementations  **should**  make this as close as possible to the
    ///presentation of the first pixel in the new image to the user.The call to
    /// [`wait_for_present_khr`] will block until either the presentId
    ///associated with [`swapchain`] is greater than or equal to [`present_id`],
    ///or [`timeout`] nanoseconds passes.
    ///When the swapchain becomes OUT_OF_DATE, the call will either return
    ///`VK_SUCCESS` (if the image was delivered to the presentation engine and
    ///may have been presented to the user) or will return early with status
    ///`VK_ERROR_OUT_OF_DATE_KHR` (if the image was not presented to the user).As an exception to
    /// the normal rules for objects which are externally
    ///synchronized, the [`swapchain`] passed to [`wait_for_present_khr`] **may**
    ///be simultaneously used by other threads in calls to functions other than
    ///[`destroy_swapchain_khr`].
    ///Access to the swapchain data associated with this extension  **must**  be atomic
    ///within the implementation.
    ///## Valid Usage
    /// - [`swapchain`] **must**  not be in the retired state
    /// - The [`presentWait`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentWait)
    ///   feature  **must**  be enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
    /// - Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved
    ///   from the same [`Instance`]
    ///
    ///## Host Synchronization
    /// - Host access to [`swapchain`] **must**  be externally synchronized
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_TIMEOUT`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_DEVICE_LOST`
    ///# Related
    /// - [`khr_present_wait`]
    /// - [`Device`]
    /// - [`SwapchainKHR`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkWaitForPresentKHR")]
    #[track_caller]
    #[inline]
    pub unsafe fn wait_for_present_khr(
        self: &Unique<Device>,
        swapchain: SwapchainKHR,
        present_id: Option<u64>,
        timeout: Option<u64>,
    ) -> VulkanResult<()> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .khr_present_wait()
            .and_then(|vtable| vtable.wait_for_present_khr())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .khr_present_wait()
            .and_then(|vtable| vtable.wait_for_present_khr())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            swapchain,
            present_id.unwrap_or_default() as _,
            timeout.unwrap_or_default() as _,
        );
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::TIMEOUT => VulkanResult::Success(_return, ()),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_present_wait`
pub struct DeviceKhrPresentWaitVTable {
    ///See [`FNWaitForPresentKhr`] for more information.
    pub wait_for_present_khr: FNWaitForPresentKhr,
}
impl DeviceKhrPresentWaitVTable {
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
            wait_for_present_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkWaitForPresentKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::wait_for_present_khr`]. See [`FNWaitForPresentKhr`] for more information.
    pub fn wait_for_present_khr(&self) -> FNWaitForPresentKhr {
        self.wait_for_present_khr
    }
}
