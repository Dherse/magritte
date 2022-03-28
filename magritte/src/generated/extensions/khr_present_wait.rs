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
//! - [`WaitForPresentKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PRESENT_WAIT_SPEC_VERSION")]
pub const KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PRESENT_WAIT_EXTENSION_NAME")]
pub const KHR_PRESENT_WAIT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_present_wait");
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
/// - [`present_wait`] indicates that the implementation supports [`WaitForPresentKHR`].
///If the [`PhysicalDevicePresentWaitFeaturesKHR`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePresentWaitFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR`
///# Related
/// - [`VK_KHR_present_wait`]
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
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
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
    ///implementation supports [`WaitForPresentKHR`].
    pub present_wait: Bool32,
}
impl<'lt> Default for PhysicalDevicePresentWaitFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            present_wait: 0,
        }
    }
}
impl<'lt> PhysicalDevicePresentWaitFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::present_wait`]
    pub fn present_wait_raw(&self) -> Bool32 {
        self.present_wait
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::present_wait`]
    pub fn set_present_wait_raw(&mut self, value: Bool32) -> &mut Self {
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
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::present_wait`]
    pub fn set_present_wait(&mut self, value: bool) -> &mut Self {
        self.present_wait = value as u8 as u32;
        self
    }
}
