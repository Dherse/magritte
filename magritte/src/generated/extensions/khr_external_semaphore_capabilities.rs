//![VK_KHR_external_semaphore_capabilities](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_capabilities.html) - instance extension
//!# Description
//!An application may wish to reference device semaphores in multiple Vulkan
//!logical devices or instances, in multiple processes, and/or in multiple
//!APIs.
//!This extension provides a set of capability queries and handle definitions
//!that allow an application to determine what types of “external” semaphore
//!handles an implementation supports for a given set of use cases.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_semaphore_capabilities]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_semaphore_capabilities extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceExternalSemaphorePropertiesKHR`]
//!# New structures
//! - [`ExternalSemaphorePropertiesKHR`]
//! - [`PhysicalDeviceExternalSemaphoreInfoKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceIdPropertiesKHR`]
//!# New enums
//! - [`ExternalSemaphoreFeatureFlagBitsKHR`]
//! - [`ExternalSemaphoreHandleTypeFlagBitsKHR`]
//!# New bitmasks
//! - [`ExternalSemaphoreFeatureFlagsKHR`]
//! - [`ExternalSemaphoreHandleTypeFlagsKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION`]
//! - [`LUID_SIZE_KHR`]
//! - Extending [`ExternalSemaphoreFeatureFlagBits`]:  -
//!   `VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR`  -
//!   `VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR`
//! - Extending [`ExternalSemaphoreHandleTypeFlagBits`]:  -
//!   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR`  -
//!   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR`  -
//!   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`  -
//!   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR`  -
//!   `VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR`
//!# Version History
//! - Revision 1, 2016-10-20 (James Jones)  - Initial revision
//!# Other info
//! * 2016-10-20
//! * No known IP claims.
//! * - Promoted to Vulkan 1.1 Core
//! * - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA
//!# Related
//! - [`LUID_SIZE_KHR`]
//! - [`ExternalSemaphoreFeatureFlagBitsKHR`]
//! - [`ExternalSemaphoreFeatureFlagsKHR`]
//! - [`ExternalSemaphoreHandleTypeFlagBitsKHR`]
//! - [`ExternalSemaphoreHandleTypeFlagsKHR`]
//! - [`ExternalSemaphorePropertiesKHR`]
//! - [`PhysicalDeviceExternalSemaphoreInfoKHR`]
//! - [`PhysicalDeviceIdPropertiesKHR`]
//! - [`GetPhysicalDeviceExternalSemaphorePropertiesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{vulkan1_0::Instance, vulkan1_1::FNGetPhysicalDeviceExternalSemaphoreProperties};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_external_semaphore_capabilities");
///The V-table of [`Instance`] for functions from `VK_KHR_external_semaphore_capabilities`
pub struct InstanceKhrExternalSemaphoreCapabilitiesVTable {
    ///See [`FNGetPhysicalDeviceExternalSemaphoreProperties`] for more information.
    pub get_physical_device_external_semaphore_properties: FNGetPhysicalDeviceExternalSemaphoreProperties,
}
impl InstanceKhrExternalSemaphoreCapabilitiesVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Instance,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Instance,
    ) -> Self {
        Self {
            get_physical_device_external_semaphore_properties: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceExternalSemaphorePropertiesKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_external_semaphore_properties`]. See
    /// [`FNGetPhysicalDeviceExternalSemaphoreProperties`] for more information.
    pub fn get_physical_device_external_semaphore_properties(&self) -> FNGetPhysicalDeviceExternalSemaphoreProperties {
        self.get_physical_device_external_semaphore_properties
    }
}
