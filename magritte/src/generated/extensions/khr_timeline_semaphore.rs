//![VK_KHR_timeline_semaphore](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_timeline_semaphore.html) - device extension
//!# Description
//!This extension introduces a new type of semaphore that has an integer
//!payload identifying a point in a timeline.
//!Such timeline semaphores support the following operations:
//! - Host query - A host operation that allows querying the payload of the timeline semaphore.
//! - Host wait - A host operation that allows a blocking wait for a timeline semaphore to reach a
//!   specified value.
//! - Host signal - A host operation that allows advancing the timeline semaphore to a specified
//!   value.
//! - Device wait - A device operation that allows waiting for a timeline semaphore to reach a
//!   specified value.
//! - Device signal - A device operation that allows advancing the timeline semaphore to a specified
//!   value.
//!# Revision
//!2
//!# Dependencies
//! - *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jason Ekstrand [jekstrand](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_timeline_semaphore]
//!   @jekstrand%0A<<Here describe the issue or question you have about the
//!   VK_KHR_timeline_semaphore extension>>)
//!# New functions & commands
//! - [`get_semaphore_counter_value_khr`]
//! - [`signal_semaphore_khr`]
//! - [`wait_semaphores_khr`]
//!# New structures
//! - [`SemaphoreSignalInfoKHR`]
//! - [`SemaphoreWaitInfoKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceTimelineSemaphoreFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceTimelineSemaphorePropertiesKHR`]
//! - Extending [`SemaphoreCreateInfo`], [`PhysicalDeviceExternalSemaphoreInfo`]:  -
//!   [`SemaphoreTypeCreateInfoKHR`]
//! - Extending [`SubmitInfo`], [`BindSparseInfo`]:  - [`TimelineSemaphoreSubmitInfoKHR`]
//!# New enums
//! - [`SemaphoreTypeKHR`]
//! - [`SemaphoreWaitFlagBitsKHR`]
//!# New bitmasks
//! - [`SemaphoreWaitFlagsKHR`]
//!# New constants
//! - [`KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME`]
//! - [`KHR_TIMELINE_SEMAPHORE_SPEC_VERSION`]
//! - Extending [`SemaphoreType`]:  - `VK_SEMAPHORE_TYPE_BINARY_KHR`  -
//!   `VK_SEMAPHORE_TYPE_TIMELINE_KHR`
//! - Extending [`SemaphoreWaitFlagBits`]:  - `VK_SEMAPHORE_WAIT_ANY_BIT_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Do we need a new object type for this? **RESOLVED** : No, we just introduce a new type of
//! semaphore object, as
//![`VK_KHR_external_semaphore_win32`] already uses semaphores as the destination
//!for importing D3D12 fence objects, which are semantically close/identical to
//!the proposed synchronization primitive.2) What type of payload the new synchronization primitive
//! has? **RESOLVED** : A 64-bit unsigned integer that can only be set to strictly
//!increasing values by signal operations and is not changed by wait
//!operations.3) Does the new synchronization primitive have the same signal-before-wait
//!requirement as the existing semaphores do? **RESOLVED** : No.
//!Timeline semaphores support signaling and waiting entirely asynchronously.
//!It is the responsibility of the client to avoid deadlock.4) Does the new synchronization
//! primitive allow resetting its payload? **RESOLVED** : No, allowing the payload value to “go
//! backwards” is
//!problematic.
//!Applications looking for reset behavior should create a new instance of the
//!sychronization primitive instead.5) How do we enable host waits on the synchronization
//! primitive? **RESOLVED** : Both a non-blocking query of the current payload value of the
//!synchronization primitive, and a blocking wait operation are provided.6) How do we enable device
//! waits and signals on the synchronization
//!primitive? **RESOLVED** : Similar to [`VK_KHR_external_semaphore_win32`], this extension
//!introduces a new structure that can be chained to [`SubmitInfo`] to
//!specify the values signaled semaphores should be set to, and the values
//!waited semaphores need to reach.7) Can the new synchronization primitive be used to synchronize
//! presentation
//!and swapchain image acquisition operations? **RESOLVED** : Some implementations may have
//! problems with supporting that
//!directly, thus it is not allowed in this extension.8) Do we want to support external sharing of
//! the new synchronization
//!primitive type? **RESOLVED** : Yes.
//!Timeline semaphore specific external sharing capabilities can be queried
//!using [`get_physical_device_external_semaphore_properties`] by chaining the
//!new [`SemaphoreTypeCreateInfoKHR`] structure to its
//!`pExternalSemaphoreInfo` structure.
//!This allows having a different set of external semaphore handle types
//!supported for timeline semaphores vs binary semaphores.9) Do we need to add a host signal
//! operation for the new synchronization
//!primitive type? **RESOLVED** : Yes.
//!This helps in situations where one host thread submits a workload but
//!another host thread has the information on when the workload is ready to be
//!executed.10) How should the new synchronization primitive interact with the ordering
//!requirements of the original [`Semaphore`]? **RESOLVED** : Prior to calling any command which
//! **may**  cause a wait operation
//!on a binary semaphore, the client  **must**  ensure that the semaphore signal
//!operation that has been submitted for execution and any semaphore signal
//!operations on which it depends (if any)  **must**  have also been submitted for
//!execution.11) Should we have separate feature bits for different sub-features of
//!timeline semaphores? **RESOLVED** : No.
//!The only feature which cannot be supported universally is timeline semaphore
//!import/export.
//!For import/export, the client is already required to query available
//!external handle types via
//![`get_physical_device_external_semaphore_properties`] and provide the
//!semaphore type by adding a [`SemaphoreTypeCreateInfoKHR`] structure to
//!the `pNext` chain of [`PhysicalDeviceExternalSemaphoreInfo`] so no
//!new feature bit is required.
//!# Version History
//! - Revision 1, 2018-05-10 (Jason Ekstrand)  - Initial version
//! - Revision 2, 2019-06-12 (Jason Ekstrand)  - Added an initialValue parameter to timeline
//!   semaphore creation
//!# Other info
//! * 2019-06-12
//! * No known IP claims.
//! * - This extension interacts with `[`VK_KHR_external_semaphore_capabilities`]`  - This extension
//!   interacts with `[`VK_KHR_external_semaphore`]`  - This extension interacts with
//!   `[`VK_KHR_external_semaphore_win32`]`  - Promoted to Vulkan 1.2 Core
//! * - Jeff Bolz, NVIDIA  - Yuriy O’Donnell, Epic Games  - Jason Ekstrand, Intel  - Jesse Hall,
//!   Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Daniel Rakos, AMD  - Ray Smith, Arm
//!# Related
//! - [`PhysicalDeviceTimelineSemaphoreFeaturesKHR`]
//! - [`PhysicalDeviceTimelineSemaphorePropertiesKHR`]
//! - [`SemaphoreSignalInfoKHR`]
//! - [`SemaphoreTypeCreateInfoKHR`]
//! - [`SemaphoreTypeKHR`]
//! - [`SemaphoreWaitFlagBitsKHR`]
//! - [`SemaphoreWaitFlagsKHR`]
//! - [`SemaphoreWaitInfoKHR`]
//! - [`TimelineSemaphoreSubmitInfoKHR`]
//! - [`get_semaphore_counter_value_khr`]
//! - [`signal_semaphore_khr`]
//! - [`wait_semaphores_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::Device,
    vulkan1_2::{FNGetSemaphoreCounterValue, FNSignalSemaphore, FNWaitSemaphores},
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION")]
pub const KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_timeline_semaphore");
///The V-table of [`Device`] for functions from `VK_KHR_timeline_semaphore`
pub struct DeviceKhrTimelineSemaphoreVTable {
    ///See [`FNGetSemaphoreCounterValue`] for more information.
    pub get_semaphore_counter_value_khr: FNGetSemaphoreCounterValue,
    ///See [`FNWaitSemaphores`] for more information.
    pub wait_semaphores_khr: FNWaitSemaphores,
    ///See [`FNSignalSemaphore`] for more information.
    pub signal_semaphore_khr: FNSignalSemaphore,
}
impl DeviceKhrTimelineSemaphoreVTable {
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
            get_semaphore_counter_value_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetSemaphoreCounterValueKHR").as_ptr(),
                ))
            },
            wait_semaphores_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkWaitSemaphoresKHR").as_ptr()))
            },
            signal_semaphore_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkSignalSemaphoreKHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_semaphore_counter_value_khr`]. See [`FNGetSemaphoreCounterValue`] for more
    /// information.
    pub fn get_semaphore_counter_value_khr(&self) -> FNGetSemaphoreCounterValue {
        self.get_semaphore_counter_value_khr
    }
    ///Gets [`Self::wait_semaphores_khr`]. See [`FNWaitSemaphores`] for more information.
    pub fn wait_semaphores_khr(&self) -> FNWaitSemaphores {
        self.wait_semaphores_khr
    }
    ///Gets [`Self::signal_semaphore_khr`]. See [`FNSignalSemaphore`] for more information.
    pub fn signal_semaphore_khr(&self) -> FNSignalSemaphore {
        self.signal_semaphore_khr
    }
}
