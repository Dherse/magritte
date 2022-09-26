//![VK_KHR_external_fence_capabilities](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_capabilities.html) - instance extension
//!# Description
//!An application may wish to reference device fences in multiple Vulkan
//!logical devices or instances, in multiple processes, and/or in multiple
//!APIs.
//!This extension provides a set of capability queries and handle definitions
//!that allow an application to determine what types of “external” fence
//!handles an implementation supports for a given set of use cases.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Deprecation State
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_fence_capabilities]
//!   @critsec%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_fence_capabilities extension>>)
//!# New commands
//! - [`get_physical_device_external_fence_properties_khr`]
//!# New structures
//! - [`ExternalFencePropertiesKHR`]
//! - [`PhysicalDeviceExternalFenceInfoKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceIdPropertiesKHR`]
//!# New enums
//! - [`ExternalFenceFeatureFlagBitsKHR`]
//! - [`ExternalFenceHandleTypeFlagBitsKHR`]
//!# New bitmasks
//! - [`ExternalFenceFeatureFlagsKHR`]
//! - [`ExternalFenceHandleTypeFlagsKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION`]
//! - [`LUID_SIZE_KHR`]
//! - Extending [`ExternalFenceFeatureFlagBits`]:  - `VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR`
//!   - `VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR`
//! - Extending [`ExternalFenceHandleTypeFlagBits`]:  -
//!   `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR`  -
//!   `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`  -
//!   `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR`  -
//!   `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR`
//!# Version history
//! - Revision 1, 2017-05-08 (Jesse Hall)  - Initial version
//!# Other information
//! * 2017-05-08
//! * No known IP claims.
//! * - Promoted to Vulkan 1.1 Core
//! * - Jesse Hall, Google  - James Jones, NVIDIA  - Jeff Juliano, NVIDIA  - Cass Everitt, Oculus  -
//!   Contributors to `[`khr_external_semaphore_capabilities`]`
//!# Related
//! - [`LUID_SIZE_KHR`]
//! - [`ExternalFenceFeatureFlagBitsKHR`]
//! - [`ExternalFenceFeatureFlagsKHR`]
//! - [`ExternalFenceHandleTypeFlagBitsKHR`]
//! - [`ExternalFenceHandleTypeFlagsKHR`]
//! - [`ExternalFencePropertiesKHR`]
//! - [`PhysicalDeviceExternalFenceInfoKHR`]
//! - [`PhysicalDeviceIdPropertiesKHR`]
//! - [`get_physical_device_external_fence_properties_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{vulkan1_0::Instance, vulkan1_1::FNGetPhysicalDeviceExternalFenceProperties};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_external_fence_capabilities");
///The V-table of [`Instance`] for functions from `VK_KHR_external_fence_capabilities`
pub struct InstanceKhrExternalFenceCapabilitiesVTable {
    ///See [`FNGetPhysicalDeviceExternalFenceProperties`] for more information.
    pub get_physical_device_external_fence_properties_khr: FNGetPhysicalDeviceExternalFenceProperties,
}
impl InstanceKhrExternalFenceCapabilitiesVTable {
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
            get_physical_device_external_fence_properties_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceExternalFencePropertiesKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_external_fence_properties_khr`]. See
    /// [`FNGetPhysicalDeviceExternalFenceProperties`] for more information.
    pub fn get_physical_device_external_fence_properties_khr(&self) -> FNGetPhysicalDeviceExternalFenceProperties {
        self.get_physical_device_external_fence_properties_khr
    }
}
