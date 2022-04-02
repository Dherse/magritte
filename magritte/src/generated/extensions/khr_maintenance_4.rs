//![VK_KHR_maintenance4](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance4.html) - device extension
//!# Description
//![`VK_KHR_maintenance4`] adds a collection of minor features, none of which
//!would warrant an entire extension of their own.The new features are as follows:
//! - Allow the application to destroy their [`PipelineLayout`] object immediately after it was used
//!   to create another object. It is no longer necessary to keep its handle valid while the created
//!   object is in use.
//! - Add a new [`maxBufferSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxBufferSize)
//!   implementation-defined limit for the maximum size [`Buffer`] that  **can**  be created.
//! - Add support for the SPIR-V 1.2 `LocalSizeId` execution mode, which can be used as an
//!   alternative to `LocalSize` to specify the local workgroup size with specialization constants.
//! - Add a guarantee that images created with identical creation parameters will always have the
//!   same alignment requirements.
//! - Add new [`GetDeviceBufferMemoryRequirementsKHR`], [`GetDeviceImageMemoryRequirementsKHR`], and
//!   [`GetDeviceImageSparseMemoryRequirementsKHR`] to allow the application to query the image
//!   memory requirements without having to create an image object and query it.
//! - Relax the requirement that push constants must be initialized before they are dynamically
//!   accessed.
//! - Relax the interface matching rules to allow a larger output vector to match with a smaller
//!   input vector, with additional values being discarded.
//! - Add a guarantee for buffer memory requirement that the size memory requirement is never
//!   greater than the result of aligning create size with the alignment memory requirement.
//!# Revision
//!2
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.1
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_maintenance4]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_KHR_maintenance4
//!   extension>>)
//!# New functions & commands
//! - [`GetDeviceBufferMemoryRequirementsKHR`]
//! - [`GetDeviceImageMemoryRequirementsKHR`]
//! - [`GetDeviceImageSparseMemoryRequirementsKHR`]
//!# New structures
//! - [`DeviceBufferMemoryRequirementsKHR`]
//! - [`DeviceImageMemoryRequirementsKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceMaintenance4FeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceMaintenance4PropertiesKHR`]
//!# New constants
//! - [`KHR_MAINTENANCE_4_EXTENSION_NAME`]
//! - [`KHR_MAINTENANCE_4_SPEC_VERSION`]
//! - Extending [`ImageAspectFlagBits`]:  - `VK_IMAGE_ASPECT_NONE_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR`
//!# Known issues & F.A.Q
//!None.
//!# Version History
//! - Revision 1, 2021-08-18 (Piers Daniell)  - Internal revisions
//! - Revision 2, 2021-10-25 (Yiwei Zhang)  - More guarantees on buffer memory requirements
//!# Other info
//! * 2021-10-25
//! * - Promoted to Vulkan 1.3 Core  - Requires SPIR-V 1.2 for `LocalSizeId`
//! * - Lionel Duc, NVIDIA  - Jason Ekstrand, Intel  - Spencer Fricke, Samsung  - Tobias Hector, AMD
//!   - Lionel Landwerlin, Intel  - Graeme Leese, Broadcom  - Tom Olson, Arm  - Stu Smith, AMD  -
//!   Yiwei Zhang, Google
//!# Related
//! - [`DeviceBufferMemoryRequirementsKHR`]
//! - [`DeviceImageMemoryRequirementsKHR`]
//! - [`PhysicalDeviceMaintenance4FeaturesKHR`]
//! - [`PhysicalDeviceMaintenance4PropertiesKHR`]
//! - [`GetDeviceBufferMemoryRequirementsKHR`]
//! - [`GetDeviceImageMemoryRequirementsKHR`]
//! - [`GetDeviceImageSparseMemoryRequirementsKHR`]
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
    vulkan1_3::{
        FNGetDeviceBufferMemoryRequirements, FNGetDeviceImageMemoryRequirements,
        FNGetDeviceImageSparseMemoryRequirements,
    },
};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_MAINTENANCE_4_SPEC_VERSION")]
pub const KHR_MAINTENANCE_4_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_MAINTENANCE_4_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_4_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_maintenance4");
///The V-table of [`Device`] for functions from VK_KHR_maintenance4
pub struct DeviceKhrMaintenance4VTable {
    ///See [`FNGetDeviceBufferMemoryRequirements`] for more information.
    pub get_device_buffer_memory_requirements: FNGetDeviceBufferMemoryRequirements,
    ///See [`FNGetDeviceImageMemoryRequirements`] for more information.
    pub get_device_image_memory_requirements: FNGetDeviceImageMemoryRequirements,
    ///See [`FNGetDeviceImageSparseMemoryRequirements`] for more information.
    pub get_device_image_sparse_memory_requirements: FNGetDeviceImageSparseMemoryRequirements,
}
impl DeviceKhrMaintenance4VTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Device) -> Self
    where
        F: Fn(Device, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            get_device_buffer_memory_requirements: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeviceBufferMemoryRequirementsKHR"),
                ))
            },
            get_device_image_memory_requirements: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetDeviceImageMemoryRequirementsKHR")))
            },
            get_device_image_sparse_memory_requirements: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeviceImageSparseMemoryRequirementsKHR"),
                ))
            },
        }
    }
    ///Gets [`Self::get_device_buffer_memory_requirements`]. See
    /// [`FNGetDeviceBufferMemoryRequirements`] for more information.
    pub fn get_device_buffer_memory_requirements(&self) -> FNGetDeviceBufferMemoryRequirements {
        self.get_device_buffer_memory_requirements
    }
    ///Gets [`Self::get_device_image_memory_requirements`]. See
    /// [`FNGetDeviceImageMemoryRequirements`] for more information.
    pub fn get_device_image_memory_requirements(&self) -> FNGetDeviceImageMemoryRequirements {
        self.get_device_image_memory_requirements
    }
    ///Gets [`Self::get_device_image_sparse_memory_requirements`]. See
    /// [`FNGetDeviceImageSparseMemoryRequirements`] for more information.
    pub fn get_device_image_sparse_memory_requirements(&self) -> FNGetDeviceImageSparseMemoryRequirements {
        self.get_device_image_sparse_memory_requirements
    }
}
