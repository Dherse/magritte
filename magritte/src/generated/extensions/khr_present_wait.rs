//![VK_KHR_present_wait](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_present_wait.html) - device extension
//!# Description
//!This device extension allows an application that uses the
//!`[`VK_KHR_swapchain`]` extension to wait for present operations to
//!complete.
//!An application can use this to monitor and control the pacing of the
//!application by managing the number of outstanding images yet to be
//!presented.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_swapchain`]`
//! - Requires `[`VK_KHR_present_id`]`
//!# Contacts
//! - Keith Packard [keithp](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_present_wait]
//!   @keithp%0A<<Here describe the issue or question you have about the VK_KHR_present_wait
//!   extension>>)
//!# New functions & commands
//! - [`WaitForPresentKHR`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDevicePresentWaitFeaturesKHR`]
//!# New constants
//! - [`KHR_PRESENT_WAIT_EXTENSION_NAME`]
//! - [`KHR_PRESENT_WAIT_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR`
//!# Known issues & F.A.Q
//!1) When does the wait finish?**RESOLVED**.
//!The wait will finish when the present is visible to the user.
//!There is no requirement for any precise timing relationship between the
//!presentation of the image to the user, but implementations **should** signal
//!the wait as close as possible to the presentation of the first pixel in the
//!new image to the user.2) Should this use fences or other existing synchronization
//! mechanism.**RESOLVED**.
//!Because display and rendering are often implemented in separate drivers,
//!this extension will provide a separate synchronization API.3) Should this extension share
//! present identification with other extensions?**RESOLVED**.
//!Yes.
//!A new extension, VK_KHR_present_id, should be created to provide a shared
//!structure for presentation identifiers.4) What happens when presentations complete out of order
//! wrt calls to
//!vkQueuePresent? This could happen if the semaphores for the presentations
//!were ready out of order.**OPTION A**: Require that when a PresentId is set that the driver
//! ensure that
//!images are always presented in the order of calls to vkQueuePresent.**OPTION B**: Finish both
//! waits when the earliest present completes.
//!This will complete the later present wait earlier than the actual
//!presentation.
//!This should be the easiest to implement as the driver need only track the
//!largest present ID completed.
//!This is also the 'natural' consequence of interpreting the existing
//!vkWaitForPresentKHR specificationn.**OPTION C**: Finish both waits when both have completed.
//!This will complete the earlier presentation later than the actual
//!presentation time.
//!This is allowed by the current specification as there is no precise timing
//!requirement for when the presentId value is updated.
//!This requires slightly more complexity in the driver as it will need to
//!track all outstanding presentId values.
//!# Version History
//! - Revision 1, 2019-02-19 (Keith Packard)
//! - Initial version
//!# Other info
//! * 2019-05-15
//! * No known IP claims.
//!*
//! - Keith Packard, Valve
//! - Ian Elliott, Google
//! - Tobias Hector, AMD
//! - Daniel Stone, Collabora
//!# Related
//! - [`PhysicalDevicePresentWaitFeaturesKHR`]
//! - [`WaitForPresentKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PRESENT_WAIT_SPEC_VERSION")]
pub const KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PRESENT_WAIT_EXTENSION_NAME")]
pub const KHR_PRESENT_WAIT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_present_wait");
