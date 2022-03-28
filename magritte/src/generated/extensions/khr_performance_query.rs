use crate::{
    core::{MAX_DESCRIPTION_SIZE, UUID_SIZE},
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData, os::raw::c_char};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const KHR_PERFORMANCE_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_performance_query");
///[VkPerformanceCounterScopeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterScopeKHR.html) - Supported counter scope types
///# C Specifications
///Performance counters have an associated scope.
///This scope describes the granularity of a performance counter.The performance counter scope
/// types which  **may**  be returned in
///[`PerformanceCounterKHR::scope`] are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterScopeKHR {
///    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR = 0,
///    VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR = 1,
///    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR = 2,
///    VK_QUERY_SCOPE_COMMAND_BUFFER_KHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR,
///    VK_QUERY_SCOPE_RENDER_PASS_KHR = VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR,
///    VK_QUERY_SCOPE_COMMAND_KHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR,
///} VkPerformanceCounterScopeKHR;
///```
///# Description
/// - [`PerformanceCounterScopeCommandBufferKhr`] - the performance counter scope is a single
///   complete command buffer.
/// - [`PerformanceCounterScopeRenderPassKhr`] - the performance counter scope is zero or more
///   complete render passes. The performance query containing the performance counter  **must**
///   begin and end outside a render pass instance.
/// - [`PerformanceCounterScopeCommandKhr`] - the performance counter scope is zero or more
///   commands.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterScopeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum PerformanceCounterScopeKHR {
    ///[`PerformanceCounterScopeCommandBufferKhr`] - the performance
    ///counter scope is a single complete command buffer.
    PerformanceCounterScopeCommandBufferKhr = 0,
    ///[`PerformanceCounterScopeRenderPassKhr`] - the performance
    ///counter scope is zero or more complete render passes.
    ///The performance query containing the performance counter  **must**  begin and
    ///end outside a render pass instance.
    PerformanceCounterScopeRenderPassKhr = 1,
    ///[`PerformanceCounterScopeCommandKhr`] - the performance counter
    ///scope is zero or more commands.
    PerformanceCounterScopeCommandKhr = 2,
}
impl const Default for PerformanceCounterScopeKHR {
    fn default() -> Self {
        PerformanceCounterScopeCommandBufferKhr
    }
}
impl PerformanceCounterScopeKHR {
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
///[VkPerformanceCounterUnitKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterUnitKHR.html) - Supported counter unit types
///# C Specifications
///Performance counters have an associated unit.
///This unit describes how to interpret the performance counter result.The performance counter unit
/// types which  **may**  be returned in
///[`PerformanceCounterKHR::unit`] are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterUnitKHR {
///    VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR = 0,
///    VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR = 1,
///    VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR = 2,
///    VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR = 3,
///    VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR = 4,
///    VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR = 5,
///    VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR = 6,
///    VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR = 7,
///    VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR = 8,
///    VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR = 9,
///    VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR = 10,
///} VkPerformanceCounterUnitKHR;
///```
///# Description
/// - [`PerformanceCounterUnitGenericKhr`] - the performance counter unit is a generic data point.
/// - [`PerformanceCounterUnitPercentageKhr`] - the performance counter unit is a percentage (%).
/// - [`PerformanceCounterUnitNanosecondsKhr`] - the performance counter unit is a value of
///   nanoseconds (ns).
/// - [`PerformanceCounterUnitBytesKhr`] - the performance counter unit is a value of bytes.
/// - [`PerformanceCounterUnitBytesPerSecondKhr`] - the performance counter unit is a value of
///   bytes/s.
/// - [`PerformanceCounterUnitKelvinKhr`] - the performance counter unit is a temperature reported
///   in Kelvin.
/// - [`PerformanceCounterUnitWattsKhr`] - the performance counter unit is a value of watts (W).
/// - [`PerformanceCounterUnitVoltsKhr`] - the performance counter unit is a value of volts (V).
/// - [`PerformanceCounterUnitAmpsKhr`] - the performance counter unit is a value of amps (A).
/// - [`PerformanceCounterUnitHertzKhr`] - the performance counter unit is a value of hertz (Hz).
/// - [`PerformanceCounterUnitCyclesKhr`] - the performance counter unit is a value of cycles.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterUnitKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum PerformanceCounterUnitKHR {
    ///[`PerformanceCounterUnitGenericKhr`] - the performance counter
    ///unit is a generic data point.
    PerformanceCounterUnitGenericKhr = 0,
    ///[`PerformanceCounterUnitPercentageKhr`] - the performance
    ///counter unit is a percentage (%).
    PerformanceCounterUnitPercentageKhr = 1,
    ///[`PerformanceCounterUnitNanosecondsKhr`] - the performance
    ///counter unit is a value of nanoseconds (ns).
    PerformanceCounterUnitNanosecondsKhr = 2,
    ///[`PerformanceCounterUnitBytesKhr`] - the performance counter
    ///unit is a value of bytes.
    PerformanceCounterUnitBytesKhr = 3,
    ///[`PerformanceCounterUnitBytesPerSecondKhr`] - the performance
    ///counter unit is a value of bytes/s.
    PerformanceCounterUnitBytesPerSecondKhr = 4,
    ///[`PerformanceCounterUnitKelvinKhr`] - the performance counter
    ///unit is a temperature reported in Kelvin.
    PerformanceCounterUnitKelvinKhr = 5,
    ///[`PerformanceCounterUnitWattsKhr`] - the performance counter
    ///unit is a value of watts (W).
    PerformanceCounterUnitWattsKhr = 6,
    ///[`PerformanceCounterUnitVoltsKhr`] - the performance counter
    ///unit is a value of volts (V).
    PerformanceCounterUnitVoltsKhr = 7,
    ///[`PerformanceCounterUnitAmpsKhr`] - the performance counter
    ///unit is a value of amps (A).
    PerformanceCounterUnitAmpsKhr = 8,
    ///[`PerformanceCounterUnitHertzKhr`] - the performance counter
    ///unit is a value of hertz (Hz).
    PerformanceCounterUnitHertzKhr = 9,
    ///[`PerformanceCounterUnitCyclesKhr`] - the performance counter
    ///unit is a value of cycles.
    PerformanceCounterUnitCyclesKhr = 10,
}
impl const Default for PerformanceCounterUnitKHR {
    fn default() -> Self {
        PerformanceCounterUnitGenericKhr
    }
}
impl PerformanceCounterUnitKHR {
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
///[VkPerformanceCounterStorageKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterStorageKHR.html) - Supported counter storage types
///# C Specifications
///Performance counters have an associated storage.
///This storage describes the payload of a counter result.The performance counter storage types
/// which  **may**  be returned in
///[`PerformanceCounterKHR::storage`] are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterStorageKHR {
///    VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR = 0,
///    VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR = 1,
///    VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR = 2,
///    VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR = 3,
///    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR = 4,
///    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR = 5,
///} VkPerformanceCounterStorageKHR;
///```
///# Description
/// - [`PerformanceCounterStorageInt32Khr`] - the performance counter storage is a 32-bit signed
///   integer.
/// - [`PerformanceCounterStorageInt64Khr`] - the performance counter storage is a 64-bit signed
///   integer.
/// - [`PerformanceCounterStorageUint32Khr`] - the performance counter storage is a 32-bit unsigned
///   integer.
/// - [`PerformanceCounterStorageUint64Khr`] - the performance counter storage is a 64-bit unsigned
///   integer.
/// - [`PerformanceCounterStorageFloat32Khr`] - the performance counter storage is a 32-bit
///   floating-point.
/// - [`PerformanceCounterStorageFloat64Khr`] - the performance counter storage is a 64-bit
///   floating-point.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterStorageKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum PerformanceCounterStorageKHR {
    ///[`PerformanceCounterStorageInt32Khr`] - the performance counter
    ///storage is a 32-bit signed integer.
    PerformanceCounterStorageInt32Khr = 0,
    ///[`PerformanceCounterStorageInt64Khr`] - the performance counter
    ///storage is a 64-bit signed integer.
    PerformanceCounterStorageInt64Khr = 1,
    ///[`PerformanceCounterStorageUint32Khr`] - the performance
    ///counter storage is a 32-bit unsigned integer.
    PerformanceCounterStorageUint32Khr = 2,
    ///[`PerformanceCounterStorageUint64Khr`] - the performance
    ///counter storage is a 64-bit unsigned integer.
    PerformanceCounterStorageUint64Khr = 3,
    ///[`PerformanceCounterStorageFloat32Khr`] - the performance
    ///counter storage is a 32-bit floating-point.
    PerformanceCounterStorageFloat32Khr = 4,
    ///[`PerformanceCounterStorageFloat64Khr`] - the performance
    ///counter storage is a 64-bit floating-point.
    PerformanceCounterStorageFloat64Khr = 5,
}
impl const Default for PerformanceCounterStorageKHR {
    fn default() -> Self {
        PerformanceCounterStorageInt32Khr
    }
}
impl PerformanceCounterStorageKHR {
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
///[VkPhysicalDevicePerformanceQueryFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html) - Structure describing performance query support for an implementation
///# C Specifications
///The [`PhysicalDevicePerformanceQueryFeaturesKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPhysicalDevicePerformanceQueryFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           performanceCounterQueryPools;
///    VkBool32           performanceCounterMultipleQueryPools;
///} VkPhysicalDevicePerformanceQueryFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`performance_counter_query_pools`] indicates whether the implementation supports performance
///   counter query pools.
/// - [`performance_counter_multiple_query_pools`] indicates whether the implementation supports
///   using multiple performance query pools in a primary command buffer and secondary command
///   buffers executed within it.
///If the [`PhysicalDevicePerformanceQueryFeaturesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePerformanceQueryFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR`
///# Related
/// - [`VK_KHR_performance_query`]
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
pub struct PhysicalDevicePerformanceQueryFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`performance_counter_query_pools`] indicates whether the implementation
    ///supports performance counter query pools.
    performance_counter_query_pools: Bool32,
    ///[`performance_counter_multiple_query_pools`] indicates whether the
    ///implementation supports using multiple performance query pools in a
    ///primary command buffer and secondary command buffers executed within it.
    performance_counter_multiple_query_pools: Bool32,
}
impl<'lt> Default for PhysicalDevicePerformanceQueryFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            performance_counter_query_pools: 0,
            performance_counter_multiple_query_pools: 0,
        }
    }
}
impl<'lt> PhysicalDevicePerformanceQueryFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::performance_counter_query_pools`]
    pub fn performance_counter_query_pools_raw(&self) -> Bool32 {
        self.performance_counter_query_pools
    }
    ///Gets the raw value of [`Self::performance_counter_multiple_query_pools`]
    pub fn performance_counter_multiple_query_pools_raw(&self) -> Bool32 {
        self.performance_counter_multiple_query_pools
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::performance_counter_query_pools`]
    pub fn set_performance_counter_query_pools_raw(&mut self, value: Bool32) -> &mut Self {
        self.performance_counter_query_pools = value;
        self
    }
    ///Sets the raw value of [`Self::performance_counter_multiple_query_pools`]
    pub fn set_performance_counter_multiple_query_pools_raw(&mut self, value: Bool32) -> &mut Self {
        self.performance_counter_multiple_query_pools = value;
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
    ///Gets the value of [`Self::performance_counter_query_pools`]
    pub fn performance_counter_query_pools(&self) -> bool {
        unsafe { std::mem::transmute(self.performance_counter_query_pools as u8) }
    }
    ///Gets the value of [`Self::performance_counter_multiple_query_pools`]
    pub fn performance_counter_multiple_query_pools(&self) -> bool {
        unsafe { std::mem::transmute(self.performance_counter_multiple_query_pools as u8) }
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
    ///Gets a mutable reference to the value of [`Self::performance_counter_query_pools`]
    pub fn performance_counter_query_pools_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.performance_counter_query_pools as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.performance_counter_query_pools as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::performance_counter_multiple_query_pools`]
    pub fn performance_counter_multiple_query_pools_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.performance_counter_multiple_query_pools as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.performance_counter_multiple_query_pools as *mut Bool32)
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
    ///Sets the raw value of [`Self::performance_counter_query_pools`]
    pub fn set_performance_counter_query_pools(&mut self, value: bool) -> &mut Self {
        self.performance_counter_query_pools = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::performance_counter_multiple_query_pools`]
    pub fn set_performance_counter_multiple_query_pools(&mut self, value: bool) -> &mut Self {
        self.performance_counter_multiple_query_pools = value as u8 as u32;
        self
    }
}
///[VkPhysicalDevicePerformanceQueryPropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html) - Structure describing performance query properties for an implementation
///# C Specifications
///The [`PhysicalDevicePerformanceQueryPropertiesKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPhysicalDevicePerformanceQueryPropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           allowCommandBufferQueryCopies;
///} VkPhysicalDevicePerformanceQueryPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`allow_command_buffer_query_copies`] is [`TRUE`] if the performance query pools are allowed
///   to be used with [`CmdCopyQueryPoolResults`].
///# Description
///If the [`PhysicalDevicePerformanceQueryPropertiesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR`
///# Related
/// - [`VK_KHR_performance_query`]
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
pub struct PhysicalDevicePerformanceQueryPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`allow_command_buffer_query_copies`] is [`TRUE`] if the performance
    ///query pools are allowed to be used with [`CmdCopyQueryPoolResults`].
    allow_command_buffer_query_copies: Bool32,
}
impl<'lt> Default for PhysicalDevicePerformanceQueryPropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            allow_command_buffer_query_copies: 0,
        }
    }
}
impl<'lt> PhysicalDevicePerformanceQueryPropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::allow_command_buffer_query_copies`]
    pub fn allow_command_buffer_query_copies_raw(&self) -> Bool32 {
        self.allow_command_buffer_query_copies
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::allow_command_buffer_query_copies`]
    pub fn set_allow_command_buffer_query_copies_raw(&mut self, value: Bool32) -> &mut Self {
        self.allow_command_buffer_query_copies = value;
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
    ///Gets the value of [`Self::allow_command_buffer_query_copies`]
    pub fn allow_command_buffer_query_copies(&self) -> bool {
        unsafe { std::mem::transmute(self.allow_command_buffer_query_copies as u8) }
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
    ///Gets a mutable reference to the value of [`Self::allow_command_buffer_query_copies`]
    pub fn allow_command_buffer_query_copies_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.allow_command_buffer_query_copies as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.allow_command_buffer_query_copies as *mut Bool32)
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
    ///Sets the raw value of [`Self::allow_command_buffer_query_copies`]
    pub fn set_allow_command_buffer_query_copies(&mut self, value: bool) -> &mut Self {
        self.allow_command_buffer_query_copies = value as u8 as u32;
        self
    }
}
///[VkPerformanceCounterKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterKHR.html) - Structure providing information about a counter
///# C Specifications
///The [`PerformanceCounterKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPerformanceCounterKHR {
///    VkStructureType                   sType;
///    void*                             pNext;
///    VkPerformanceCounterUnitKHR       unit;
///    VkPerformanceCounterScopeKHR      scope;
///    VkPerformanceCounterStorageKHR    storage;
///    uint8_t                           uuid[VK_UUID_SIZE];
///} VkPerformanceCounterKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`unit`] is a [`PerformanceCounterUnitKHR`] specifying the unit that the counter data will
///   record.
/// - [`scope`] is a [`PerformanceCounterScopeKHR`] specifying the scope that the counter belongs
///   to.
/// - [`storage`] is a [`PerformanceCounterStorageKHR`] specifying the storage type that the
///   counter’s data uses.
/// - [`uuid`] is an array of size [`UUID_SIZE`], containing 8-bit values that represent a
///   universally unique identifier for the counter of the physical device.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterScopeKHR`]
/// - [`PerformanceCounterStorageKHR`]
/// - [`PerformanceCounterUnitKHR`]
/// - [`StructureType`]
/// - [`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`]
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
pub struct PerformanceCounterKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`unit`] is a [`PerformanceCounterUnitKHR`] specifying the unit
    ///that the counter data will record.
    unit: PerformanceCounterUnitKHR,
    ///[`scope`] is a [`PerformanceCounterScopeKHR`] specifying the scope
    ///that the counter belongs to.
    scope: PerformanceCounterScopeKHR,
    ///[`storage`] is a [`PerformanceCounterStorageKHR`] specifying the
    ///storage type that the counter’s data uses.
    storage: PerformanceCounterStorageKHR,
    ///[`uuid`] is an array of size [`UUID_SIZE`], containing 8-bit
    ///values that represent a universally unique identifier for the counter of
    ///the physical device.
    uuid: [u8; UUID_SIZE],
}
impl<'lt> Default for PerformanceCounterKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            unit: Default::default(),
            scope: Default::default(),
            storage: Default::default(),
            uuid: [0; UUID_SIZE],
        }
    }
}
impl<'lt> PerformanceCounterKHR<'lt> {
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
    ///Gets the value of [`Self::unit`]
    pub fn unit(&self) -> PerformanceCounterUnitKHR {
        self.unit
    }
    ///Gets the value of [`Self::scope`]
    pub fn scope(&self) -> PerformanceCounterScopeKHR {
        self.scope
    }
    ///Gets the value of [`Self::storage`]
    pub fn storage(&self) -> PerformanceCounterStorageKHR {
        self.storage
    }
    ///Gets the value of [`Self::uuid`]
    pub fn uuid(&self) -> &[u8; UUID_SIZE] {
        &getter
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
    ///Gets a mutable reference to the value of [`Self::unit`]
    pub fn unit_mut(&mut self) -> &mut PerformanceCounterUnitKHR {
        &mut self.unit
    }
    ///Gets a mutable reference to the value of [`Self::scope`]
    pub fn scope_mut(&mut self) -> &mut PerformanceCounterScopeKHR {
        &mut self.scope
    }
    ///Gets a mutable reference to the value of [`Self::storage`]
    pub fn storage_mut(&mut self) -> &mut PerformanceCounterStorageKHR {
        &mut self.storage
    }
    ///Gets a mutable reference to the value of [`Self::uuid`]
    pub fn uuid_mut(&mut self) -> &mut [u8; UUID_SIZE] {
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
    ///Sets the raw value of [`Self::unit`]
    pub fn set_unit(
        &mut self,
        value: crate::extensions::khr_performance_query::PerformanceCounterUnitKHR,
    ) -> &mut Self {
        self.unit = value;
        self
    }
    ///Sets the raw value of [`Self::scope`]
    pub fn set_scope(
        &mut self,
        value: crate::extensions::khr_performance_query::PerformanceCounterScopeKHR,
    ) -> &mut Self {
        self.scope = value;
        self
    }
    ///Sets the raw value of [`Self::storage`]
    pub fn set_storage(
        &mut self,
        value: crate::extensions::khr_performance_query::PerformanceCounterStorageKHR,
    ) -> &mut Self {
        self.storage = value;
        self
    }
    ///Sets the raw value of [`Self::uuid`]
    pub fn set_uuid(&mut self, value: [u8; crate::core::UUID_SIZE]) -> &mut Self {
        self.uuid = value;
        self
    }
}
///[VkPerformanceCounterDescriptionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionKHR.html) - Structure providing more detailed information about a counter
///# C Specifications
///The [`PerformanceCounterDescriptionKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPerformanceCounterDescriptionKHR {
///    VkStructureType                            sType;
///    void*                                      pNext;
///    VkPerformanceCounterDescriptionFlagsKHR    flags;
///    char                                       name[VK_MAX_DESCRIPTION_SIZE];
///    char                                       category[VK_MAX_DESCRIPTION_SIZE];
///    char                                       description[VK_MAX_DESCRIPTION_SIZE];
///} VkPerformanceCounterDescriptionKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`PerformanceCounterDescriptionFlagBitsKHR`] indicating the usage
///   behavior for the counter.
/// - [`name`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing a null-terminated UTF-8
///   string specifying the name of the counter.
/// - [`category`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing a null-terminated UTF-8
///   string specifying the category of the counter.
/// - [`description`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing a null-terminated
///   UTF-8 string specifying the description of the counter.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterDescriptionFlagsKHR`]
/// - [`StructureType`]
/// - [`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`]
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
pub struct PerformanceCounterDescriptionKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`flags`] is a bitmask of
    ///[`PerformanceCounterDescriptionFlagBitsKHR`] indicating the usage
    ///behavior for the counter.
    flags: PerformanceCounterDescriptionFlagsKHR,
    ///[`name`] is an array of size [`MAX_DESCRIPTION_SIZE`], containing
    ///a null-terminated UTF-8 string specifying the name of the counter.
    name: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`category`] is an array of size [`MAX_DESCRIPTION_SIZE`],
    ///containing a null-terminated UTF-8 string specifying the category of the
    ///counter.
    category: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`description`] is an array of size [`MAX_DESCRIPTION_SIZE`],
    ///containing a null-terminated UTF-8 string specifying the description of
    ///the counter.
    description: [c_schar; MAX_DESCRIPTION_SIZE],
}
impl<'lt> Default for PerformanceCounterDescriptionKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            flags: Default::default(),
            name: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
            category: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
            description: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
        }
    }
}
impl<'lt> PerformanceCounterDescriptionKHR<'lt> {
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
    pub fn flags(&self) -> PerformanceCounterDescriptionFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::category`]
    pub fn category(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::description`]
    pub fn description(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
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
    pub fn flags_mut(&mut self) -> &mut PerformanceCounterDescriptionFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::category`]
    pub fn category_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::description`]
    pub fn description_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
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
        value: crate::extensions::khr_performance_query::PerformanceCounterDescriptionFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.name = value;
        self
    }
    ///Sets the raw value of [`Self::category`]
    pub fn set_category(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.category = value;
        self
    }
    ///Sets the raw value of [`Self::description`]
    pub fn set_description(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.description = value;
        self
    }
}
///[VkQueryPoolPerformanceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html) - Structure specifying parameters of a newly created performance query pool
///# C Specifications
///The [`QueryPoolPerformanceCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkQueryPoolPerformanceCreateInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           queueFamilyIndex;
///    uint32_t           counterIndexCount;
///    const uint32_t*    pCounterIndices;
///} VkQueryPoolPerformanceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`queue_family_index`] is the queue family index to create this performance query pool for.
/// - [`counter_index_count`] is the length of the [`counter_indices`] array.
/// - [`counter_indices`] is a pointer to an array of indices into the
///   [`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`]`::pCounters` to enable in
///   this performance query pool.
///# Description
///## Valid Usage
/// - [`queue_family_index`] **must**  be a valid queue family index of the device
/// - The [`performanceCounterQueryPools`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-performanceCounterQueryPools)
///   feature  **must**  be enabled
/// - Each element of [`counter_indices`] **must**  be in the range of counters reported by
///   [`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`] for the queue family
///   specified in [`queue_family_index`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`
/// - [`counter_indices`] **must**  be a valid pointer to an array of
///   [`counter_index_count`]`uint32_t` values
/// - [`counter_index_count`] **must**  be greater than `0`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`queue_family_index`] is the queue family index to create this
    ///performance query pool for.
    queue_family_index: u32,
    ///[`counter_index_count`] is the length of the [`counter_indices`]
    ///array.
    counter_index_count: u32,
    ///[`counter_indices`] is a pointer to an array of indices into the
    ///[`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`]::`pCounters`
    ///to enable in this performance query pool.
    counter_indices: *const u32,
}
impl<'lt> Default for QueryPoolPerformanceCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            queue_family_index: 0,
            counter_index_count: 0,
            counter_indices: std::ptr::null(),
        }
    }
}
impl<'lt> QueryPoolPerformanceCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::counter_indices`]
    pub fn counter_indices_raw(&self) -> *const u32 {
        self.counter_indices
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::counter_indices`]
    pub fn set_counter_indices_raw(&mut self, value: *const u32) -> &mut Self {
        self.counter_indices = value;
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
    ///Gets the value of [`Self::queue_family_index`]
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    ///Gets the value of [`Self::counter_index_count`]
    pub fn counter_index_count(&self) -> u32 {
        self.counter_index_count
    }
    ///Gets the value of [`Self::counter_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn counter_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.counter_indices, self.counter_index_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::queue_family_index`]
    pub fn queue_family_index_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::counter_index_count`]
    pub fn counter_index_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::queue_family_index`]
    pub fn set_queue_family_index(&mut self, value: u32) -> &mut Self {
        self.queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::counter_index_count`]
    pub fn set_counter_index_count(&mut self, value: u32) -> &mut Self {
        self.counter_index_count = value;
        self
    }
    ///Sets the raw value of [`Self::counter_indices`]
    pub fn set_counter_indices(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.counter_indices = value.as_ptr();
        self.counter_index_count = len_;
        self
    }
}
///[VkAcquireProfilingLockInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockInfoKHR.html) - Structure specifying parameters to acquire the profiling lock
///# C Specifications
///The [`AcquireProfilingLockInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkAcquireProfilingLockInfoKHR {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkAcquireProfilingLockFlagsKHR    flags;
///    uint64_t                          timeout;
///} VkAcquireProfilingLockInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`timeout`] indicates how long the function waits, in nanoseconds, if the profiling lock is
///   not available.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
///If [`timeout`] is 0, [`AcquireProfilingLockKHR`] will not block while
///attempting to acquire the profling lock.
///If [`timeout`] is `UINT64_MAX`, the function will not return until the
///profiling lock was acquired.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`AcquireProfilingLockFlagsKHR`]
/// - [`StructureType`]
/// - [`AcquireProfilingLockKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AcquireProfilingLockInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: AcquireProfilingLockFlagsKHR,
    ///[`timeout`] indicates how long the function waits, in nanoseconds, if
    ///the profiling lock is not available.
    timeout: u64,
}
impl<'lt> Default for AcquireProfilingLockInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            timeout: 0,
        }
    }
}
impl<'lt> AcquireProfilingLockInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> AcquireProfilingLockFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::timeout`]
    pub fn timeout(&self) -> u64 {
        self.timeout
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut AcquireProfilingLockFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::timeout`]
    pub fn timeout_mut(&mut self) -> &mut u64 {
        &mut getter
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
        value: crate::extensions::khr_performance_query::AcquireProfilingLockFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::timeout`]
    pub fn set_timeout(&mut self, value: u64) -> &mut Self {
        self.timeout = value;
        self
    }
}
///[VkPerformanceQuerySubmitInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html) - Structure indicating which counter pass index is active for performance queries
///# C Specifications
///The [`PerformanceQuerySubmitInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_performance_query
///typedef struct VkPerformanceQuerySubmitInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           counterPassIndex;
///} VkPerformanceQuerySubmitInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`counter_pass_index`] specifies which counter pass index is active.
///# Description
///If the [`SubmitInfo`]::[`p_next`] chain does not include this
///structure, the batch defaults to use counter pass index 0.
///## Valid Usage
/// - [`counter_pass_index`] **must**  be less than the number of counter passes required by any
///   queries within the batch. The required number of counter passes for a performance query is
///   obtained by calling [`GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`counter_pass_index`] specifies which counter pass index is active.
    counter_pass_index: u32,
}
impl<'lt> Default for PerformanceQuerySubmitInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            counter_pass_index: 0,
        }
    }
}
impl<'lt> PerformanceQuerySubmitInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Gets the value of [`Self::counter_pass_index`]
    pub fn counter_pass_index(&self) -> u32 {
        self.counter_pass_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::counter_pass_index`]
    pub fn counter_pass_index_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::counter_pass_index`]
    pub fn set_counter_pass_index(&mut self, value: u32) -> &mut Self {
        self.counter_pass_index = value;
        self
    }
}
