//![VK_EXT_device_memory_report](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_device_memory_report.html) - device extension
//!# Description
//!This device extension allows registration of device memory event callbacks
//!upon device creation, so that applications or middleware can obtain detailed
//!information about memory usage and how memory is associated with Vulkan
//!objects.
//!This extension exposes the actual underlying device memory usage, including
//!allocations that are not normally visible to the application, such as memory
//!consumed by [`CreateGraphicsPipelines`].
//!It is intended primarily for use by debug tooling rather than for production
//!applications.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Yiwei Zhang [zhangyiwei](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_device_memory_report]
//!   @zhangyiwei%0A<<Here describe the issue or question you have about the
//!   VK_EXT_device_memory_report extension>>)
//!# New structures
//! - [`DeviceMemoryReportCallbackDataEXT`]
//! - Extending [`DeviceCreateInfo`]:
//! - [`DeviceDeviceMemoryReportCreateInfoEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceDeviceMemoryReportFeaturesEXT`]
//!# New enums
//! - [`DeviceMemoryReportEventTypeEXT`]
//!# New bitmasks
//! - [`DeviceMemoryReportFlagsEXT`]
//!# New constants
//! - [`EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME`]
//! - [`EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT`
//! - `VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT`
//!# Known issues & F.A.Q
//!1) Should this be better expressed as an extension to VK_EXT_debug_utils and
//!its general-purpose messenger construct?**RESOLVED**: No.
//!The intended lifecycle is quite different.
//!We want to make this extension tied to the device’s lifecycle.
//!Each ICD just handles its own implementation of this extension, and this
//!extension will only be directly exposed from the ICD.
//!So we can avoid the extra implementation complexity used to accommodate the
//!flexibility of `[`VK_EXT_debug_utils`]` extension.2) Can we extend and use the existing internal
//! allocation callbacks instead
//!of adding the new callback structure in this extension?**RESOLVED**: No.
//!Our memory reporting layer that combines this information with other memory
//!information it collects directly (e.g. bindings of resources to
//![`DeviceMemory`]) would have to intercept all entry points that take a
//![`AllocationCallbacks`] parameter and inject its own
//!`pfnInternalAllocation` and `pfnInternalFree`.
//!That may be doable for the extensions we know about, but not for ones we do
//!not.
//!The proposal would work fine in the face of most unknown extensions.
//!But even for ones we know about, since apps can provide a different set of
//!callbacks and userdata and those can be retained by the driver and used
//!later (esp.
//!for pool object, but not just those), we would have to dynamically allocate
//!the interception trampoline every time.
//!That is getting to be an unreasonably large amount of complexity and
//!(possibly) overhead.We are interested in both alloc/free and import/unimport.
//!The latter is fairly important for tracking (and avoiding double-counting)
//!of swapchain images (still true with “native swapchains” based on external
//!memory) and media/camera interop.
//!Though we might be able to handle this with additional
//![`InternalAllocationType`] values, for import/export we do want to be
//!able to tie this to the external resource, which is one thing that the
//!`memoryObjectId` is for.The internal alloc/free callbacks are not extensible except via new
//![`InternalAllocationType`] values.
//!The [`DeviceMemoryReportCallbackDataEXT`] in this extension is
//!extensible.
//!That was deliberate: there is a real possibility we will want to get extra
//!information in the future.
//!As one example, currently this reports only physical allocations, but we
//!believe there are interesting cases for tracking how populated that VA
//!region is.The callbacks are clearly specified as only callable within the context of a
//!call from the app into Vulkan.
//!We believe there are some cases where drivers can allocate device memory
//!asynchronously.
//!This was one of the sticky issues that derailed the internal device memory
//!allocation reporting design (which is essentially what this extension is
//!trying to do) leading up to 1.0.[`AllocationCallbacks`] is described in a section called “Host
//! memory”
//!and the intro to it is very explicitly about host memory.
//!The other callbacks are all inherently about host memory.
//!But this extension is very focused on device memory.3) Should the callback be reporting which
//! heap is used?**RESOLVED**: Yes.
//!It is important for non-UMA systems to have all the device memory
//!allocations attributed to the corresponding device memory heaps.
//!For internally-allocated device memory, `heapIndex` will always
//!correspond to an advertised heap, rather than having a magic value
//!indicating a non-advertised heap.
//!Drivers can advertise heaps that do not have any corresponding memory types
//!if they need to.4) Should we use an array of callback for the layers to intercept instead of
//!chaining multiple of the [`DeviceDeviceMemoryReportCreateInfoEXT`]
//!structures in the `pNext` of [`DeviceCreateInfo`]?**RESOLVED** No.
//!The pointer to the [`DeviceDeviceMemoryReportCreateInfoEXT`] structure
//!itself is const and you cannot just cast it away.
//!Thus we cannot update the callback array inside the structure.
//!In addition, we cannot drop this `pNext` chain either, so making a copy
//!of this whole structure does not work either.5) Should we track bulk allocations shared among
//! multiple objects?**RESOLVED** No.
//!Take the shader heap as an example.
//!Some implementations will let multiple [`Pipeline`] objects share the
//!same shader heap.
//!We are not asking the implementation to report `VK_OBJECT_TYPE_PIPELINE`
//!along with a [`crate::utils::Handle::null`] for this bulk allocation.
//!Instead, this bulk allocation is considered as a layer below what this
//!extension is interested in.
//!Later, when the actual [`Pipeline`] objects are created by suballocating
//!from the bulk allocation, we ask the implementation to report the valid
//!handles of the [`Pipeline`] objects along with the actual suballocated
//!sizes and different `memoryObjectId`.6) Can we require the callbacks to be always called in the
//! same thread with
//!the Vulkan commands?**RESOLVED** No.
//!Some implementations might choose to multiplex work from multiple
//!application threads into a single backend thread and perform JIT allocations
//!as a part of that flow.
//!Since this behavior is theoretically legit, we cannot require the callbacks
//!to be always called in the same thread with the Vulkan commands, and the
//!note is to remind the applications to handle this case properly.7) Should we add an additional
//! “allocation failed” event type with things
//!like size and heap index reported?**RESOLVED** Yes.
//!This fits in well with the callback infrastructure added in this extension,
//!and implementation touches the same code and has the same overheads as the
//!rest of the extension.
//!It could help debugging things like getting an
//!`VK_ERROR_OUT_OF_HOST_MEMORY` error when ending a command buffer.
//!Right now the allocation failure could have happened anywhere during
//!recording, and a callback would be really useful to understand where and
//!why.
//!# Version History
//! - Revision 1, 2020-08-26 (Yiwei Zhang)
//! - Initial version
//! - Revision 2, 2021-01-06 (Yiwei Zhang)
//! - Minor description update
//!# Other info
//! * 2021-01-06
//! * No known IP claims.
//!*
//! - Yiwei Zhang, Google
//! - Jesse Hall, Google
//!# Related
//! - [`PFNDeviceMemoryReportCallbackEXT`]
//! - [`DeviceDeviceMemoryReportCreateInfoEXT`]
//! - [`DeviceMemoryReportCallbackDataEXT`]
//! - [`DeviceMemoryReportEventTypeEXT`]
//! - [`DeviceMemoryReportFlagsEXT`]
//! - [`PhysicalDeviceDeviceMemoryReportFeaturesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION")]
pub const EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME")]
pub const EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_device_memory_report");
///[VkDeviceMemoryReportEventTypeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportEventTypeEXT.html) - Events that can occur on a device memory object
///# C Specifications
///Possible values of [`DeviceMemoryReportCallbackDataEXT::type_`],
///specifying event types which cause the device driver to call the callback,
///are:
///```c
///// Provided by VK_EXT_device_memory_report
///typedef enum VkDeviceMemoryReportEventTypeEXT {
///    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT = 0,
///    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT = 1,
///    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT = 2,
///    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT = 3,
///    VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT = 4,
///} VkDeviceMemoryReportEventTypeEXT;
///```
///# Description
/// - [`DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE`] specifies this
///event corresponds to the allocation of an internal device memory object
///or a [`DeviceMemory`].
/// - [`DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE`] specifies this event
///corresponds to the deallocation of an internally-allocated device memory
///object or a [`DeviceMemory`].
/// - [`DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT`] specifies this event
///corresponds to the import of an external memory object.
/// - [`DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT`] specifies this
///event is the release of an imported external memory object.
/// - [`DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED`] specifies
///this event corresponds to the failed allocation of an internal device
///memory object or a [`DeviceMemory`].
///# Related
/// - [`VK_EXT_device_memory_report`]
/// - [`DeviceMemoryReportCallbackDataEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceMemoryReportEventTypeEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceMemoryReportEventTypeEXT(i32);
impl const Default for DeviceMemoryReportEventTypeEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DeviceMemoryReportEventTypeEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("DeviceMemoryReportEventTypeEXT")
            .field(match *self {
                Self::DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE => &"DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE",
                Self::DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE => &"DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE",
                Self::DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT => &"DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT",
                Self::DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT => &"DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT",
                Self::DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED => {
                    &"DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED"
                },
                other => unreachable!("invalid value for `DeviceMemoryReportEventTypeEXT`: {:?}", other),
            })
            .finish()
    }
}
impl DeviceMemoryReportEventTypeEXT {
    ///[`DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE`] specifies this
    ///event corresponds to the allocation of an internal device memory object
    ///or a [`DeviceMemory`].
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE: Self = Self(0);
    ///[`DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE`] specifies this event
    ///corresponds to the deallocation of an internally-allocated device memory
    ///object or a [`DeviceMemory`].
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE: Self = Self(1);
    ///[`DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT`] specifies this event
    ///corresponds to the import of an external memory object.
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT: Self = Self(2);
    ///[`DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT`] specifies this
    ///event is the release of an imported external memory object.
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT: Self = Self(3);
    ///[`DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED`] specifies
    ///this event corresponds to the failed allocation of an internal device
    ///memory object or a [`DeviceMemory`].
    pub const DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED: Self = Self(4);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
