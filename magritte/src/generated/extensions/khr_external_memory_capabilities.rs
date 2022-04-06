//![VK_KHR_external_memory_capabilities](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_capabilities.html) - instance extension
//!# Description
//!An application may wish to reference device memory in multiple Vulkan
//!logical devices or instances, in multiple processes, and/or in multiple
//!APIs.
//!This extension provides a set of capability queries and handle definitions
//!that allow an application to determine what types of “external” memory
//!handles an implementation supports for a given set of use cases.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_memory_capabilities]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_memory_capabilities extension>>)
//!# New functions & commands
//! - [`get_physical_device_external_buffer_properties_khr`]
//!# New structures
//! - [`ExternalBufferPropertiesKHR`]
//! - [`ExternalMemoryPropertiesKHR`]
//! - [`PhysicalDeviceExternalBufferInfoKHR`]
//! - Extending [`ImageFormatProperties2`]:  - [`ExternalImageFormatPropertiesKHR`]
//! - Extending [`PhysicalDeviceImageFormatInfo2`]:  - [`PhysicalDeviceExternalImageFormatInfoKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceIdPropertiesKHR`]
//!# New enums
//! - [`ExternalMemoryFeatureFlagBitsKHR`]
//! - [`ExternalMemoryHandleTypeFlagBitsKHR`]
//!# New bitmasks
//! - [`ExternalMemoryFeatureFlagsKHR`]
//! - [`ExternalMemoryHandleTypeFlagsKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION`]
//! - [`LUID_SIZE_KHR`]
//! - Extending [`ExternalMemoryFeatureFlagBits`]:  -
//!   `VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR`  -
//!   `VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR`  -
//!   `VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR`
//! - Extending [`ExternalMemoryHandleTypeFlagBits`]:  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR`  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR`  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR`  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR`  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR`  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`  -
//!   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR`
//!# Known issues & F.A.Q
//!1) Why do so many external memory capabilities need to be queried on a
//!per-memory-handle-type basis? **PROPOSED RESOLUTION** : This is because some handle types are
//! based on
//!OS-native objects that have far more limited capabilities than the very
//!generic Vulkan memory objects.
//!Not all memory handle types can name memory objects that support 3D images,
//!for example.
//!Some handle types cannot even support the deferred image and memory binding
//!behavior of Vulkan and require specifying the image when allocating or
//!importing the memory object.2) Do the [`ExternalImageFormatPropertiesKHR`] and
//![`ExternalBufferPropertiesKHR`] structs need to include a list of memory
//!type bits that support the given handle type? **PROPOSED RESOLUTION** : No.
//!The memory types that do not support the handle types will simply be
//!filtered out of the results returned by [`get_image_memory_requirements`]
//!and [`get_buffer_memory_requirements`] when a set of handle types was
//!specified at image or buffer creation time.3) Should the non-opaque handle types be moved to
//! their own extension? **PROPOSED RESOLUTION** : Perhaps.
//!However, defining the handle type bits does very little and does not require
//!any platform-specific types on its own, and it is easier to maintain the
//!bitfield values in a single extension for now.
//!Presumably more handle types could be added by separate extensions though,
//!and it would be midly weird to have some platform-specific ones defined in
//!the core spec and some in extensions4) Do we need a `D3D11_TILEPOOL` type? **PROPOSED
//! RESOLUTION** : No.
//!This is technically possible, but the synchronization is awkward.
//!D3D11 surfaces must be synchronized using shared mutexes, and these
//!synchronization primitives are shared by the entire memory object, so D3D11
//!shared allocations divided among multiple buffer and image bindings may be
//!difficult to synchronize.5) Should the Windows 7-compatible handle types be named “KMT” handles
//! or
//!“GLOBAL_SHARE” handles? **PROPOSED RESOLUTION** : KMT, simply because it is more concise.6) How
//! do applications identify compatible devices and drivers across
//!instance, process, and API boundaries when sharing memory? **PROPOSED RESOLUTION** : New device
//! properties are exposed that allow
//!applications to correctly correlate devices and drivers.
//!A device and driver UUID that must both match to ensure sharing
//!compatibility between two Vulkan instances, or a Vulkan instance and an
//!extensible external API are added.
//!To allow correlating with Direct3D devices, a device LUID is added that
//!corresponds to a DXGI adapter LUID.
//!A driver ID is not needed for Direct3D because mismatched driver component
//!versions are not currently supported on the Windows OS.
//!Should support for such configurations be introduced at the OS level,
//!further Vulkan extensions would be needed to correlate userspace component
//!builds.
//!# Version History
//! - Revision 1, 2016-10-17 (James Jones)  - Initial version
//!# Other info
//! * 2016-10-17
//! * No known IP claims.
//! * - Interacts with `[`VK_KHR_dedicated_allocation`]`.  - Interacts with
//!   `[`VK_NV_dedicated_allocation`]`.  - Promoted to Vulkan 1.1 Core
//! * - Ian Elliot, Google  - Jesse Hall, Google  - James Jones, NVIDIA
//!# Related
//! - [`LUID_SIZE_KHR`]
//! - [`ExternalBufferPropertiesKHR`]
//! - [`ExternalImageFormatPropertiesKHR`]
//! - [`ExternalMemoryFeatureFlagBitsKHR`]
//! - [`ExternalMemoryFeatureFlagsKHR`]
//! - [`ExternalMemoryHandleTypeFlagBitsKHR`]
//! - [`ExternalMemoryHandleTypeFlagsKHR`]
//! - [`ExternalMemoryPropertiesKHR`]
//! - [`PhysicalDeviceExternalBufferInfoKHR`]
//! - [`PhysicalDeviceExternalImageFormatInfoKHR`]
//! - [`PhysicalDeviceIdPropertiesKHR`]
//! - [`get_physical_device_external_buffer_properties_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{vulkan1_0::Instance, vulkan1_1::FNGetPhysicalDeviceExternalBufferProperties};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_external_memory_capabilities");
///The V-table of [`Instance`] for functions from `VK_KHR_external_memory_capabilities`
pub struct InstanceKhrExternalMemoryCapabilitiesVTable {
    ///See [`FNGetPhysicalDeviceExternalBufferProperties`] for more information.
    pub get_physical_device_external_buffer_properties_khr: FNGetPhysicalDeviceExternalBufferProperties,
}
impl InstanceKhrExternalMemoryCapabilitiesVTable {
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
            get_physical_device_external_buffer_properties_khr: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceExternalBufferPropertiesKHR").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_external_buffer_properties_khr`]. See
    /// [`FNGetPhysicalDeviceExternalBufferProperties`] for more information.
    pub fn get_physical_device_external_buffer_properties_khr(&self) -> FNGetPhysicalDeviceExternalBufferProperties {
        self.get_physical_device_external_buffer_properties_khr
    }
}
