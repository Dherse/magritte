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
