//![VK_EXT_pageable_device_local_memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_pageable_device_local_memory.html) - device extension
//!# Description
//!Vulkan is frequently implemented on multi-user and multi-process operating
//!systems where the device-local memory can be shared by more than one
//!process.
//!On such systems the size of the device-local memory available to the
//!application may not be the full size of the memory heap at all times.
//!In order for these operating systems to support multiple applications the
//!device-local memory is virtualized and paging is used to move memory between
//!device-local and host-local memory heaps, transparent to the application.The current Vulkan
//! specification does not expose this behavior well and may
//!cause applications to make suboptimal memory choices when allocating memory.
//!For example, in a system with multiple applications running, the application
//!may think that device-local memory is full and revert to making
//!performance-sensitive allocations from host-local memory.
//!In reality the memory heap might not have been full, it just appeared to be
//!at the time memory consumption was queried, and a device-local allocation
//!would have succeeded.
//!A well designed operating system that implements pageable device-local
//!memory will try to have all memory allocations for the foreground
//!application paged into device-local memory, and paged out for other
//!applications as needed when not in use.When this extension is exposed by the Vulkan
//! implementation it indicates to
//!the application that the operating system implements pageable device-local
//!memory and the application should adjust its memory allocation strategy
//!accordingly.
//!The extension also exposes a new [`set_device_memory_priority_ext`] function
//!to allow the application to dynamically adjust the priority of existing
//!memory allocations based on its current needs.
//!This will help the operating system page out lower priority memory
//!allocations before higher priority allocations when needed.
//!It will also help the operating system decide which memory allocations to
//!page back into device-local memory first.To take best advantage of pageable device-local memory
//! the application must
//!create the Vulkan device with the
//![`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT::pageable_device_local_memory`]
//!feature enabled.
//!When enabled the Vulkan implementation will allow device-local memory
//!allocations to be paged in and out by the operating system, and  **may**  not
//!return VK_ERROR_OUT_OF_DEVICE_MEMORY even if device-local memory appears to
//!be full, but will instead page this, or other allocations, out to make room.
//!The Vulkan implementation will also ensure that host-local memory
//!allocations will never be promoted to device-local memory by the operating
//!system, or consume device-local memory.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_EXT_memory_priority`]`
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_pageable_device_local_memory]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_pageable_device_local_memory extension>>)
//!# New functions & commands
//! - [`set_device_memory_priority_ext`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`]
//!# New constants
//! - [`EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME`]
//! - [`EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2021-08-24 (Piers Daniell)  - Initial revision
//!# Other info
//! * 2021-08-24
//! * - Hans-Kristian Arntzen, Valve  - Axel Gneiting, id Software  - Billy Khan, id Software  -
//!   Daniel Koch, NVIDIA  - Chris Lentini, NVIDIA  - Joshua Schnarr, NVIDIA  - Stu Smith, AMD
//!# Related
//! - [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`]
//! - [`set_device_memory_priority_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseOutStructure, Bool32, Device, DeviceMemory, StructureType},
    AsRaw, Unique,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_pageable_device_local_memory");
///[vkSetDeviceMemoryPriorityEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html) - Change a memory allocation priority
///# C Specifications
///To modify the priority of an existing memory allocation, call:
///```c
///// Provided by VK_EXT_pageable_device_local_memory
///void vkSetDeviceMemoryPriorityEXT(
///    VkDevice                                    device,
///    VkDeviceMemory                              memory,
///    float                                       priority);
///```
///# Parameters
/// - [`device`] is the logical device that owns the memory.
/// - [`memory`] is the [`DeviceMemory`] object to which the new priority will be applied.
/// - [`priority`] is a floating-point value between `0` and `1`, indicating the priority of the
///   allocation relative to other memory allocations. Larger values are higher priority. The
///   granularity of the priorities is implementation-dependent.
///# Description
///Memory allocations with higher priority  **may**  be more likely to stay in
///device-local memory when the system is under memory pressure.
///## Valid Usage
/// - [`priority`] **must**  be between `0` and `1`, inclusive
///
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
/// - [`memory`] **must**  have been created, allocated, or retrieved from [`device`]
///# Related
/// - [`VK_EXT_pageable_device_local_memory`]
/// - [`Device`]
/// - [`DeviceMemory`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
pub type FNSetDeviceMemoryPriorityExt =
    Option<unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32)>;
///[VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html) - Structure describing whether the implementation supports pageable device-local memory
///# C Specifications
///The [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_pageable_device_local_memory
///typedef struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           pageableDeviceLocalMemory;
///} VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pageable_device_local_memory`] indicates that the implementation supports pageable
///   device-local memory and  **may**  transparently move device-local memory allocations to
///   host-local memory to better share device-local memory with other applications.
///If the [`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT`
///# Related
/// - [`VK_EXT_pageable_device_local_memory`]
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
#[doc(alias = "VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`pageable_device_local_memory`]
    ///indicates that the implementation supports pageable device-local memory
    ///and  **may**  transparently move device-local memory allocations to
    ///host-local memory to better share device-local memory with other
    ///applications.
    pub pageable_device_local_memory: Bool32,
}
impl<'lt> Default for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            pageable_device_local_memory: 0,
        }
    }
}
impl<'lt> PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::pageable_device_local_memory`]
    pub fn pageable_device_local_memory_raw(&self) -> Bool32 {
        self.pageable_device_local_memory
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::pageable_device_local_memory`]
    pub fn set_pageable_device_local_memory_raw(mut self, value: Bool32) -> Self {
        self.pageable_device_local_memory = value;
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
    ///Gets the value of [`Self::pageable_device_local_memory`]
    pub fn pageable_device_local_memory(&self) -> bool {
        unsafe { std::mem::transmute(self.pageable_device_local_memory as u8) }
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
    ///Gets a mutable reference to the value of [`Self::pageable_device_local_memory`]
    pub fn pageable_device_local_memory_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.pageable_device_local_memory as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.pageable_device_local_memory as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::pageable_device_local_memory`]
    pub fn set_pageable_device_local_memory(mut self, value: bool) -> Self {
        self.pageable_device_local_memory = value as u8 as u32;
        self
    }
}
impl Device {
    ///[vkSetDeviceMemoryPriorityEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html) - Change a memory allocation priority
    ///# C Specifications
    ///To modify the priority of an existing memory allocation, call:
    ///```c
    ///// Provided by VK_EXT_pageable_device_local_memory
    ///void vkSetDeviceMemoryPriorityEXT(
    ///    VkDevice                                    device,
    ///    VkDeviceMemory                              memory,
    ///    float                                       priority);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device that owns the memory.
    /// - [`memory`] is the [`DeviceMemory`] object to which the new priority will be applied.
    /// - [`priority`] is a floating-point value between `0` and `1`, indicating the priority of the
    ///   allocation relative to other memory allocations. Larger values are higher priority. The
    ///   granularity of the priorities is implementation-dependent.
    ///# Description
    ///Memory allocations with higher priority  **may**  be more likely to stay in
    ///device-local memory when the system is under memory pressure.
    ///## Valid Usage
    /// - [`priority`] **must**  be between `0` and `1`, inclusive
    ///
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
    /// - [`memory`] **must**  have been created, allocated, or retrieved from [`device`]
    ///# Related
    /// - [`VK_EXT_pageable_device_local_memory`]
    /// - [`Device`]
    /// - [`DeviceMemory`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn set_device_memory_priority_ext(
        self: &Unique<Device>,
        memory: DeviceMemory,
        priority: Option<f32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_pageable_device_local_memory()
            .and_then(|vtable| vtable.set_device_memory_priority_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_pageable_device_local_memory()
            .and_then(|vtable| vtable.set_device_memory_priority_ext())
            .unwrap_unchecked();
        let _return = _function(self.as_raw(), memory, priority.unwrap_or_default() as _);
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_pageable_device_local_memory`
pub struct DeviceExtPageableDeviceLocalMemoryVTable {
    ///See [`FNSetDeviceMemoryPriorityExt`] for more information.
    pub set_device_memory_priority_ext: FNSetDeviceMemoryPriorityExt,
}
impl DeviceExtPageableDeviceLocalMemoryVTable {
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
            set_device_memory_priority_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkSetDeviceMemoryPriorityEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::set_device_memory_priority_ext`]. See [`FNSetDeviceMemoryPriorityExt`] for more
    /// information.
    pub fn set_device_memory_priority_ext(&self) -> FNSetDeviceMemoryPriorityExt {
        self.set_device_memory_priority_ext
    }
}
