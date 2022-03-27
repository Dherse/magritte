use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DeviceSize, ObjectType, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
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
/// - [`DeviceMemoryReportEventTypeAllocateExt`] specifies this event corresponds to the allocation
///   of an internal device memory object or a [`DeviceMemory`].
/// - [`DeviceMemoryReportEventTypeFreeExt`] specifies this event corresponds to the deallocation of
///   an internally-allocated device memory object or a [`DeviceMemory`].
/// - [`DeviceMemoryReportEventTypeImportExt`] specifies this event corresponds to the import of an
///   external memory object.
/// - [`DeviceMemoryReportEventTypeUnimportExt`] specifies this event is the release of an imported
///   external memory object.
/// - [`DeviceMemoryReportEventTypeAllocationFailedExt`] specifies this event corresponds to the
///   failed allocation of an internal device memory object or a [`DeviceMemory`].
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum DeviceMemoryReportEventTypeEXT {
    ///[`DeviceMemoryReportEventTypeAllocateExt`] specifies this
    ///event corresponds to the allocation of an internal device memory object
    ///or a [`DeviceMemory`].
    DeviceMemoryReportEventTypeAllocateExt = 0,
    ///[`DeviceMemoryReportEventTypeFreeExt`] specifies this event
    ///corresponds to the deallocation of an internally-allocated device memory
    ///object or a [`DeviceMemory`].
    DeviceMemoryReportEventTypeFreeExt = 1,
    ///[`DeviceMemoryReportEventTypeImportExt`] specifies this event
    ///corresponds to the import of an external memory object.
    DeviceMemoryReportEventTypeImportExt = 2,
    ///[`DeviceMemoryReportEventTypeUnimportExt`] specifies this
    ///event is the release of an imported external memory object.
    DeviceMemoryReportEventTypeUnimportExt = 3,
    ///[`DeviceMemoryReportEventTypeAllocationFailedExt`] specifies
    ///this event corresponds to the failed allocation of an internal device
    ///memory object or a [`DeviceMemory`].
    DeviceMemoryReportEventTypeAllocationFailedExt = 4,
}
impl const Default for DeviceMemoryReportEventTypeEXT {
    fn default() -> Self {
        DeviceMemoryReportEventTypeAllocateExt
    }
}
impl DeviceMemoryReportEventTypeEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkPhysicalDeviceDeviceMemoryReportFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html) - Structure describing whether device memory report callback can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDeviceMemoryReportFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_device_memory_report
///typedef struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           deviceMemoryReport;
///} VkPhysicalDeviceDeviceMemoryReportFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_memory_report`] indicates whether the implementation supports the ability to register
///   device memory report callbacks.
///If the [`PhysicalDeviceDeviceMemoryReportFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDeviceMemoryReportFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT`
///# Related
/// - [`VK_EXT_device_memory_report`]
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`device_memory_report`] indicates
    ///whether the implementation supports the ability to register device
    ///memory report callbacks.
    device_memory_report: Bool32,
}
///[VkDeviceDeviceMemoryReportCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html) - Register device memory report callbacks for a Vulkan device
///# C Specifications
///To register callbacks for underlying device memory events of type
///[`DeviceMemoryReportEventTypeEXT`], add one or multiple
///[`DeviceDeviceMemoryReportCreateInfoEXT`] structures to the [`p_next`]
///chain of the [`DeviceCreateInfo`] structure.
///```c
///// Provided by VK_EXT_device_memory_report
///typedef struct VkDeviceDeviceMemoryReportCreateInfoEXT {
///    VkStructureType                        sType;
///    const void*                            pNext;
///    VkDeviceMemoryReportFlagsEXT           flags;
///    PFN_vkDeviceMemoryReportCallbackEXT    pfnUserCallback;
///    void*                                  pUserData;
///} VkDeviceDeviceMemoryReportCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is 0 and reserved for future use.
/// - [`pfn_user_callback`] is the application callback function to call.
/// - [`p_user_data`] is user data to be passed to the callback.
///# Description
///The callback **may** be called from multiple threads simultaneously.The callback **must** be
/// called only once by the implementation when a
///[`DeviceMemoryReportEventTypeEXT`] event occurs.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
/// - [`pfn_user_callback`]**must** be a valid [`PFNDeviceMemoryReportCallbackEXT`] value
/// - [`p_user_data`]**must** be a pointer value
///# Related
/// - [`PFNDeviceMemoryReportCallbackEXT`]
/// - [`VK_EXT_device_memory_report`]
/// - [`DeviceMemoryReportFlagsEXT`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceDeviceMemoryReportCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is 0 and reserved for future use.
    flags: DeviceMemoryReportFlagsEXT,
    ///[`pfn_user_callback`] is the application callback function to call.
    pfn_user_callback: PFNDeviceMemoryReportCallbackEXT<'lt>,
    ///[`p_user_data`] is user data to be passed to the callback.
    p_user_data: *const c_void,
}
///[VkDeviceMemoryReportCallbackDataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportCallbackDataEXT.html) - Structure specifying parameters returned to the callback
///# C Specifications
///The definition of [`DeviceMemoryReportCallbackDataEXT`] is:
///```c
///// Provided by VK_EXT_device_memory_report
///typedef struct VkDeviceMemoryReportCallbackDataEXT {
///    VkStructureType                     sType;
///    void*                               pNext;
///    VkDeviceMemoryReportFlagsEXT        flags;
///    VkDeviceMemoryReportEventTypeEXT    type;
///    uint64_t                            memoryObjectId;
///    VkDeviceSize                        size;
///    VkObjectType                        objectType;
///    uint64_t                            objectHandle;
///    uint32_t                            heapIndex;
///} VkDeviceMemoryReportCallbackDataEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is 0 and reserved for future use.
/// - [`type_`] is a [`DeviceMemoryReportEventTypeEXT`] type specifying the type of event reported
///   in this [`DeviceMemoryReportCallbackDataEXT`] structure.
/// - [`memory_object_id`] is the unique id for the underlying memory object as described below.
/// - [`size`] is the size of the memory object in bytes. If [`type_`] is
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`,
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT` or
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`, [`size`] is a valid [`DeviceSize`]
///   value. Otherwise, [`size`] is undefined.
/// - [`object_type`] is a [`ObjectType`] value specifying the type of the object associated with
///   this device memory report event. If [`type_`] is
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`,
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT`,
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT`,
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT` or
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`, [`object_type`] is a valid
///   [`ObjectType`] enum. Otherwise, [`object_type`] is undefined.
/// - [`object_handle`] is the object this device memory report event is attributed to. If [`type_`]
///   is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`, `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT`,
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT` or `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT`,
///   [`object_handle`] is a valid Vulkan handle of the type associated with [`object_type`] as defined
///   in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types)
///   table. Otherwise, [`object_handle`] is undefined.
/// - [`heap_index`] describes which memory heap this device memory allocation is made from. If
///   [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT` or
///   `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`, [`heap_index`] corresponds to one
///   of the valid heaps from the [`PhysicalDeviceMemoryProperties`] structure. Otherwise,
///   [`heap_index`] is undefined.
///# Description
///[`memory_object_id`] is used to avoid double-counting on the same memory
///object.If an internally-allocated device memory object or a [`DeviceMemory`]**cannot** be
/// exported, [`memory_object_id`]**must** be unique in the
///[`Device`].If an internally-allocated device memory object or a [`DeviceMemory`]
///supports being exported, [`memory_object_id`]**must** be unique system wide.If an internal
/// device memory object or a [`DeviceMemory`] is backed by
///an imported external memory object, [`memory_object_id`]**must** be unique
///system wide.Implementor’s NoteIf the heap backing an internally-allocated device memory
/// **cannot** be used to
///back [`DeviceMemory`], implementations **can** advertise that heap with no
///types.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_EXT_device_memory_report`]
/// - [`DeviceMemoryReportEventTypeEXT`]
/// - [`DeviceMemoryReportFlagsEXT`]
/// - [`DeviceSize`]
/// - [`ObjectType`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceMemoryReportCallbackDataEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`flags`] is 0 and reserved for future use.
    flags: DeviceMemoryReportFlagsEXT,
    ///[`type_`] is a [`DeviceMemoryReportEventTypeEXT`] type specifying
    ///the type of event reported in this
    ///[`DeviceMemoryReportCallbackDataEXT`] structure.
    type_: DeviceMemoryReportEventTypeEXT,
    ///[`memory_object_id`] is the unique id for the underlying memory object
    ///as described below.
    memory_object_id: u64,
    ///[`size`] is the size of the memory object in bytes.
    ///If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`,
    ///`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT` or
    ///`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`,
    ///[`size`] is a valid [`DeviceSize`] value.
    ///Otherwise, [`size`] is undefined.
    size: DeviceSize,
    ///[`object_type`] is a [`ObjectType`] value specifying the type of
    ///the object associated with this device memory report event.
    ///If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`,
    ///`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT`,
    ///`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT`,
    ///`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT` or
    ///`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`,
    ///[`object_type`] is a valid [`ObjectType`] enum.
    ///Otherwise, [`object_type`] is undefined.
    object_type: ObjectType,
    ///[`object_handle`] is the object this device memory report event is
    ///attributed to.
    ///If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`,
    ///`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT`,
    ///`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT` or
    ///`VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT`,
    ///[`object_handle`] is a valid Vulkan handle of the type associated with
    ///[`object_type`] as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types) table.
    ///Otherwise, [`object_handle`] is undefined.
    object_handle: u64,
    ///[`heap_index`] describes which memory heap this device memory
    ///allocation is made from.
    ///If [`type_`] is `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT`
    ///or `VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT`,
    ///[`heap_index`] corresponds to one of the valid heaps from the
    ///[`PhysicalDeviceMemoryProperties`] structure.
    ///Otherwise, [`heap_index`] is undefined.
    heap_index: u32,
}
