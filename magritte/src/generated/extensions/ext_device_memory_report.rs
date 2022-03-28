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
//! - Extending [`DeviceCreateInfo`]:  - [`DeviceDeviceMemoryReportCreateInfoEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceDeviceMemoryReportFeaturesEXT`]
//!# New enums
//! - [`DeviceMemoryReportEventTypeEXT`]
//!# New bitmasks
//! - [`DeviceMemoryReportFlagsEXT`]
//!# New constants
//! - [`EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME`]
//! - [`EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT`
//!# Known issues & F.A.Q
//!1) Should this be better expressed as an extension to VK_EXT_debug_utils and
//!its general-purpose messenger construct? **RESOLVED** : No.
//!The intended lifecycle is quite different.
//!We want to make this extension tied to the device’s lifecycle.
//!Each ICD just handles its own implementation of this extension, and this
//!extension will only be directly exposed from the ICD.
//!So we can avoid the extra implementation complexity used to accommodate the
//!flexibility of `[`VK_EXT_debug_utils`]` extension.2) Can we extend and use the existing internal
//! allocation callbacks instead
//!of adding the new callback structure in this extension? **RESOLVED** : No.
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
//! heap is used? **RESOLVED** : Yes.
//!It is important for non-UMA systems to have all the device memory
//!allocations attributed to the corresponding device memory heaps.
//!For internally-allocated device memory, `heapIndex` will always
//!correspond to an advertised heap, rather than having a magic value
//!indicating a non-advertised heap.
//!Drivers can advertise heaps that do not have any corresponding memory types
//!if they need to.4) Should we use an array of callback for the layers to intercept instead of
//!chaining multiple of the [`DeviceDeviceMemoryReportCreateInfoEXT`]
//!structures in the `pNext` of [`DeviceCreateInfo`]? **RESOLVED**  No.
//!The pointer to the [`DeviceDeviceMemoryReportCreateInfoEXT`] structure
//!itself is const and you cannot just cast it away.
//!Thus we cannot update the callback array inside the structure.
//!In addition, we cannot drop this `pNext` chain either, so making a copy
//!of this whole structure does not work either.5) Should we track bulk allocations shared among
//! multiple objects? **RESOLVED**  No.
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
//!the Vulkan commands? **RESOLVED**  No.
//!Some implementations might choose to multiplex work from multiple
//!application threads into a single backend thread and perform JIT allocations
//!as a part of that flow.
//!Since this behavior is theoretically legit, we cannot require the callbacks
//!to be always called in the same thread with the Vulkan commands, and the
//!note is to remind the applications to handle this case properly.7) Should we add an additional
//! “allocation failed” event type with things
//!like size and heap index reported? **RESOLVED**  Yes.
//!This fits in well with the callback infrastructure added in this extension,
//!and implementation touches the same code and has the same overheads as the
//!rest of the extension.
//!It could help debugging things like getting an
//!`VK_ERROR_OUT_OF_HOST_MEMORY` error when ending a command buffer.
//!Right now the allocation failure could have happened anywhere during
//!recording, and a callback would be really useful to understand where and
//!why.
//!# Version History
//! - Revision 1, 2020-08-26 (Yiwei Zhang)  - Initial version
//! - Revision 2, 2021-01-06 (Yiwei Zhang)  - Minor description update
//!# Other info
//! * 2021-01-06
//! * No known IP claims.
//! * - Yiwei Zhang, Google  - Jesse Hall, Google
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
#[non_exhaustive]
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
        Self::DeviceMemoryReportEventTypeAllocateExt
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
///[VkDeviceMemoryReportFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_device_memory_report
///typedef VkFlags VkDeviceMemoryReportFlagsEXT;
///```
///# Related
/// - [`VK_EXT_device_memory_report`]
/// - [`DeviceDeviceMemoryReportCreateInfoEXT`]
/// - [`DeviceMemoryReportCallbackDataEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DeviceMemoryReportFlagsEXT(u32);
impl const Default for DeviceMemoryReportFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DeviceMemoryReportFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DeviceMemoryReportFlagsEXT))
            .field(&self.0)
            .finish()
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
///[`PhysicalDeviceDeviceMemoryReportFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`device_memory_report`] indicates
    ///whether the implementation supports the ability to register device
    ///memory report callbacks.
    device_memory_report: Bool32,
}
impl<'lt> Default for PhysicalDeviceDeviceMemoryReportFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            device_memory_report: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDeviceMemoryReportFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::device_memory_report`]
    pub fn device_memory_report_raw(&self) -> Bool32 {
        self.device_memory_report
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_memory_report`]
    pub fn set_device_memory_report_raw(&mut self, value: Bool32) -> &mut Self {
        self.device_memory_report = value;
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
    ///Gets the value of [`Self::device_memory_report`]
    pub fn device_memory_report(&self) -> bool {
        unsafe { std::mem::transmute(self.device_memory_report as u8) }
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
    ///Gets a mutable reference to the value of [`Self::device_memory_report`]
    pub fn device_memory_report_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.device_memory_report as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.device_memory_report as *mut Bool32)
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
    ///Sets the raw value of [`Self::device_memory_report`]
    pub fn set_device_memory_report(&mut self, value: bool) -> &mut Self {
        self.device_memory_report = value as u8 as u32;
        self
    }
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
/// - [`user_data`] is user data to be passed to the callback.
///# Description
///The callback  **may**  be called from multiple threads simultaneously.The callback  **must**  be
/// called only once by the implementation when a
///[`DeviceMemoryReportEventTypeEXT`] event occurs.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT`
/// - [`flags`] **must**  be `0`
/// - [`pfn_user_callback`] **must**  be a valid [`PFNDeviceMemoryReportCallbackEXT`] value
/// - [`user_data`] **must**  be a pointer value
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceDeviceMemoryReportCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is 0 and reserved for future use.
    flags: DeviceMemoryReportFlagsEXT,
    ///[`pfn_user_callback`] is the application callback function to call.
    pfn_user_callback: PFNDeviceMemoryReportCallbackEXT<'lt>,
    ///[`user_data`] is user data to be passed to the callback.
    user_data: *mut c_void,
}
impl<'lt> Default for DeviceDeviceMemoryReportCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            pfn_user_callback: Default::default(),
            user_data: std::ptr::null_mut(),
        }
    }
}
impl<'lt> DeviceDeviceMemoryReportCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::user_data`]
    pub fn user_data_raw(&self) -> &*mut c_void {
        &self.user_data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data_raw(&mut self, value: *mut c_void) -> &mut Self {
        self.user_data = value;
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> DeviceMemoryReportFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::pfn_user_callback`]
    pub fn pfn_user_callback(&self) -> &PFNDeviceMemoryReportCallbackEXT<'lt> {
        &self.pfn_user_callback
    }
    ///Gets the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data(&self) -> &c_void {
        &*self.user_data
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DeviceMemoryReportFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::pfn_user_callback`]
    pub fn pfn_user_callback_mut(&mut self) -> &mut PFNDeviceMemoryReportCallbackEXT<'lt> {
        &mut self.pfn_user_callback
    }
    ///Gets a mutable reference to the value of [`Self::user_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn user_data_mut(&mut self) -> &mut c_void {
        &mut *self.user_data
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_device_memory_report::DeviceMemoryReportFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::pfn_user_callback`]
    pub fn set_pfn_user_callback(
        &mut self,
        value: crate::extensions::ext_device_memory_report::PFNDeviceMemoryReportCallbackEXT<'lt>,
    ) -> &mut Self {
        self.pfn_user_callback = value;
        self
    }
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.user_data = value as *mut _;
        self
    }
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
///object.If an internally-allocated device memory object or a [`DeviceMemory`] **cannot**  be
/// exported, [`memory_object_id`] **must**  be unique in the
///[`Device`].If an internally-allocated device memory object or a [`DeviceMemory`]
///supports being exported, [`memory_object_id`] **must**  be unique system wide.If an internal
/// device memory object or a [`DeviceMemory`] is backed by
///an imported external memory object, [`memory_object_id`] **must**  be unique
///system wide.
///## Implementor’s NoteIf the heap backing an internally-allocated device memory  **cannot**  be
/// used to
///back [`DeviceMemory`], implementations  **can**  advertise that heap with no
///types.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT`
/// - [`p_next`] **must**  be `NULL`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceMemoryReportCallbackDataEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
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
impl<'lt> Default for DeviceMemoryReportCallbackDataEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            type_: Default::default(),
            memory_object_id: 0,
            size: Default::default(),
            object_type: Default::default(),
            object_handle: 0,
            heap_index: 0,
        }
    }
}
impl<'lt> DeviceMemoryReportCallbackDataEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> DeviceMemoryReportFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> DeviceMemoryReportEventTypeEXT {
        self.type_
    }
    ///Gets the value of [`Self::memory_object_id`]
    pub fn memory_object_id(&self) -> u64 {
        self.memory_object_id
    }
    ///Gets the value of [`Self::size`]
    pub fn size(&self) -> DeviceSize {
        self.size
    }
    ///Gets the value of [`Self::object_type`]
    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }
    ///Gets the value of [`Self::object_handle`]
    pub fn object_handle(&self) -> u64 {
        self.object_handle
    }
    ///Gets the value of [`Self::heap_index`]
    pub fn heap_index(&self) -> u32 {
        self.heap_index
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
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DeviceMemoryReportFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type__mut(&mut self) -> &mut DeviceMemoryReportEventTypeEXT {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::memory_object_id`]
    pub fn memory_object_id_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::size`]
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
    }
    ///Gets a mutable reference to the value of [`Self::object_type`]
    pub fn object_type_mut(&mut self) -> &mut ObjectType {
        &mut self.object_type
    }
    ///Gets a mutable reference to the value of [`Self::object_handle`]
    pub fn object_handle_mut(&mut self) -> &mut u64 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::heap_index`]
    pub fn heap_index_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_device_memory_report::DeviceMemoryReportFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::type_`]
    pub fn set_type_(
        &mut self,
        value: crate::extensions::ext_device_memory_report::DeviceMemoryReportEventTypeEXT,
    ) -> &mut Self {
        self.type_ = value;
        self
    }
    ///Sets the raw value of [`Self::memory_object_id`]
    pub fn set_memory_object_id(&mut self, value: u64) -> &mut Self {
        self.memory_object_id = value;
        self
    }
    ///Sets the raw value of [`Self::size`]
    pub fn set_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.size = value;
        self
    }
    ///Sets the raw value of [`Self::object_type`]
    pub fn set_object_type(&mut self, value: crate::vulkan1_0::ObjectType) -> &mut Self {
        self.object_type = value;
        self
    }
    ///Sets the raw value of [`Self::object_handle`]
    pub fn set_object_handle(&mut self, value: u64) -> &mut Self {
        self.object_handle = value;
        self
    }
    ///Sets the raw value of [`Self::heap_index`]
    pub fn set_heap_index(&mut self, value: u32) -> &mut Self {
        self.heap_index = value;
        self
    }
}
