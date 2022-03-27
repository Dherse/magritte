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
/// types which **may** be returned in
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
///   complete render passes. The performance query containing the performance counter **must**
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
    ///The performance query containing the performance counter **must** begin and
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
/// types which **may** be returned in
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
/// which **may** be returned in
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
///[`PhysicalDevicePerformanceQueryFeaturesKHR`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`performance_counter_query_pools`] indicates whether the implementation
    ///supports performance counter query pools.
    performance_counter_query_pools: Bool32,
    ///[`performance_counter_multiple_query_pools`] indicates whether the
    ///implementation supports using multiple performance query pools in a
    ///primary command buffer and secondary command buffers executed within it.
    performance_counter_multiple_query_pools: Bool32,
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
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`allow_command_buffer_query_copies`] is [`TRUE`] if the performance
    ///query pools are allowed to be used with [`CmdCopyQueryPoolResults`].
    allow_command_buffer_query_copies: Bool32,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceCounterKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceCounterDescriptionKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
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
/// - [`counter_index_count`] is the length of the [`p_counter_indices`] array.
/// - [`p_counter_indices`] is a pointer to an array of indices into the
///   [`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`]`::pCounters` to enable in
///   this performance query pool.
///# Description
///Valid Usage
/// - [`queue_family_index`]**must** be a valid queue family index of the device
/// - The [`performanceCounterQueryPools`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-performanceCounterQueryPools)
///   feature **must** be enabled
/// - Each element of [`p_counter_indices`]**must** be in the range of counters reported by
///   [`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`] for the queue family
///   specified in [`queue_family_index`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`
/// - [`p_counter_indices`]**must** be a valid pointer to an array of
///   [`counter_index_count`]`uint32_t` values
/// - [`counter_index_count`]**must** be greater than `0`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`queue_family_index`] is the queue family index to create this
    ///performance query pool for.
    queue_family_index: u32,
    ///[`counter_index_count`] is the length of the [`p_counter_indices`]
    ///array.
    counter_index_count: u32,
    ///[`p_counter_indices`] is a pointer to an array of indices into the
    ///[`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`]::`pCounters`
    ///to enable in this performance query pool.
    p_counter_indices: *mut u32,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AcquireProfilingLockInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: AcquireProfilingLockFlagsKHR,
    ///[`timeout`] indicates how long the function waits, in nanoseconds, if
    ///the profiling lock is not available.
    timeout: u64,
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
///structure, the batch defaults to use counter pass index 0.Valid Usage
/// - [`counter_pass_index`]**must** be less than the number of counter passes required by any
///   queries within the batch. The required number of counter passes for a performance query is
///   obtained by calling [`GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`counter_pass_index`] specifies which counter pass index is active.
    counter_pass_index: u32,
}
