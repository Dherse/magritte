//![VK_EXT_memory_priority](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_memory_priority.html) - device extension
//!# Description
//!This extension adds a `priority` value specified at memory allocation
//!time.
//!On some systems with both device-local and non-device-local memory heaps,
//!the implementation may transparently move memory from one heap to another
//!when a heap becomes full (for example, when the total memory used across all
//!processes exceeds the size of the heap).
//!In such a case, this priority value may be used to determine which
//!allocations are more likely to remain in device-local memory.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_memory_priority]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the VK_EXT_memory_priority
//!   extension>>)
//!# New structures
//! - Extending [`MemoryAllocateInfo`]:  - [`MemoryPriorityAllocateInfoEXT`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceMemoryPriorityFeaturesEXT`]
//!# New constants
//! - [`EXT_MEMORY_PRIORITY_EXTENSION_NAME`]
//! - [`EXT_MEMORY_PRIORITY_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2018-10-08 (Jeff Bolz)  - Initial revision
//!# Other info
//! * 2018-10-08
//! * - Jeff Bolz, NVIDIA  - Jeff Juliano, NVIDIA
//!# Related
//! - [`MemoryPriorityAllocateInfoEXT`]
//! - [`PhysicalDeviceMemoryPriorityFeaturesEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_SPEC_VERSION")]
pub const EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME")]
pub const EXT_MEMORY_PRIORITY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_memory_priority");
///[VkPhysicalDeviceMemoryPriorityFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html) - Structure describing memory priority features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceMemoryPriorityFeaturesEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_memory_priority
///typedef struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           memoryPriority;
///} VkPhysicalDeviceMemoryPriorityFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_priority`] indicates that the implementation supports memory priorities specified at
///   memory allocation time via [`MemoryPriorityAllocateInfoEXT`].
///If the [`PhysicalDeviceMemoryPriorityFeaturesEXT`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMemoryPriorityFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT`
///# Related
/// - [`ext_memory_priority`]
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
#[doc(alias = "VkPhysicalDeviceMemoryPriorityFeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`memory_priority`] indicates that the
    ///implementation supports memory priorities specified at memory allocation
    ///time via [`MemoryPriorityAllocateInfoEXT`].
    pub memory_priority: Bool32,
}
impl<'lt> Default for PhysicalDeviceMemoryPriorityFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            memory_priority: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMemoryPriorityFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::memory_priority`]
    pub fn memory_priority_raw(&self) -> Bool32 {
        self.memory_priority
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::memory_priority`]
    pub fn set_memory_priority_raw(&mut self, value: Bool32) -> &mut Self {
        self.memory_priority = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::memory_priority`]
    pub fn with_memory_priority_raw(mut self, value: Bool32) -> Self {
        self.memory_priority = value;
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
    ///Gets the value of [`Self::memory_priority`]
    pub fn memory_priority(&self) -> bool {
        unsafe { std::mem::transmute(self.memory_priority as u8) }
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
    ///Gets a mutable reference to the value of [`Self::memory_priority`]
    pub fn memory_priority_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.memory_priority as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.memory_priority as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::memory_priority`]
    pub fn set_memory_priority(&mut self, value: bool) -> &mut Self {
        self.memory_priority = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::memory_priority`]
    pub fn with_memory_priority(mut self, value: bool) -> Self {
        self.memory_priority = value as u8 as u32;
        self
    }
}
///[VkMemoryPriorityAllocateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html) - Specify a memory allocation priority
///# C Specifications
///If the [`p_next`] chain includes a [`MemoryPriorityAllocateInfoEXT`]
///structure, then that structure includes a priority for the memory.The
/// [`MemoryPriorityAllocateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_memory_priority
///typedef struct VkMemoryPriorityAllocateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    float              priority;
///} VkMemoryPriorityAllocateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`priority`] is a floating-point value between `0` and `1`, indicating the priority of the
///   allocation relative to other memory allocations. Larger values are higher priority. The
///   granularity of the priorities is implementation-dependent.
///# Description
///Memory allocations with higher priority  **may**  be more likely to stay in
///device-local memory when the system is under memory pressure.If this structure is not included,
/// it is as if the [`priority`] value were
///`0.5`.
///## Valid Usage
/// - [`priority`] **must**  be between `0` and `1`, inclusive
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT`
///# Related
/// - [`ext_memory_priority`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryPriorityAllocateInfoEXT")]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`priority`] is a floating-point value between `0` and `1`, indicating
    ///the priority of the allocation relative to other memory allocations.
    ///Larger values are higher priority.
    ///The granularity of the priorities is implementation-dependent.
    pub priority: f32,
}
impl<'lt> Default for MemoryPriorityAllocateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MEMORY_PRIORITY_ALLOCATE_INFO_EXT,
            p_next: std::ptr::null(),
            priority: 0.0,
        }
    }
}
impl<'lt> MemoryPriorityAllocateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::priority`]
    pub fn priority(&self) -> f32 {
        self.priority
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::priority`]
    pub fn priority_mut(&mut self) -> &mut f32 {
        &mut self.priority
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::priority`]
    pub fn set_priority(&mut self, value: f32) -> &mut Self {
        self.priority = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::priority`]
    pub fn with_priority(mut self, value: f32) -> Self {
        self.priority = value;
        self
    }
}
