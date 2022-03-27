use crate::vulkan1_0::{BaseInStructure, Bool32, StructureType};
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
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const INTEL_PERFORMANCE_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_INTEL_performance_query");
///[VkPerformanceConfigurationTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html) - Type of performance configuration
///# C Specifications
///Possible values of
///[`PerformanceConfigurationAcquireInfoINTEL::type_`], specifying
///performance configuration types, are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceConfigurationTypeINTEL {
///    VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL = 0,
///} VkPerformanceConfigurationTypeINTEL;
///```
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceConfigurationAcquireInfoINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceConfigurationTypeINTEL")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum PerformanceConfigurationTypeINTEL {
    ///No documentation found
    PerformanceConfigurationTypeCommandQueueMetricsDiscoveryActivatedIntel = 0,
}
impl const Default for PerformanceConfigurationTypeINTEL {
    fn default() -> Self {
        PerformanceConfigurationTypeCommandQueueMetricsDiscoveryActivatedIntel
    }
}
impl PerformanceConfigurationTypeINTEL {
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
///[VkQueryPoolSamplingModeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolSamplingModeINTEL.html) - Enum specifying how performance queries should be captured
///# C Specifications
///Possible values of
///[`QueryPoolPerformanceQueryCreateInfoINTEL::performance_counters_sampling`]
///are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkQueryPoolSamplingModeINTEL {
///    VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL = 0,
///} VkQueryPoolSamplingModeINTEL;
///```
///# Description
/// - [`QueryPoolSamplingModeManualIntel`] is the default mode in which the application calls
///   [`CmdBeginQuery`] and [`CmdEndQuery`] to record performance data.
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`QueryPoolPerformanceQueryCreateInfoINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueryPoolSamplingModeINTEL")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum QueryPoolSamplingModeINTEL {
    ///[`QueryPoolSamplingModeManualIntel`] is the default mode in
    ///which the application calls [`CmdBeginQuery`] and
    ///[`CmdEndQuery`] to record performance data.
    QueryPoolSamplingModeManualIntel = 0,
}
impl const Default for QueryPoolSamplingModeINTEL {
    fn default() -> Self {
        QueryPoolSamplingModeManualIntel
    }
}
impl QueryPoolSamplingModeINTEL {
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
///[VkPerformanceOverrideTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideTypeINTEL.html) - Performance override type
///# C Specifications
///Possible values of [`PerformanceOverrideInfoINTEL::type_`],
///specifying performance override types, are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceOverrideTypeINTEL {
///    VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL = 0,
///    VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL = 1,
///} VkPerformanceOverrideTypeINTEL;
///```
///# Description
/// - [`PerformanceOverrideTypeNullHardwareIntel`] turns all rendering operations into noop.
/// - [`PerformanceOverrideTypeFlushGpuCachesIntel`] stalls the stream of commands until all
///   previously emitted commands have completed and all caches been flushed and invalidated.
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceOverrideInfoINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceOverrideTypeINTEL")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum PerformanceOverrideTypeINTEL {
    ///[`PerformanceOverrideTypeNullHardwareIntel`] turns all
    ///rendering operations into noop.
    PerformanceOverrideTypeNullHardwareIntel = 0,
    ///[`PerformanceOverrideTypeFlushGpuCachesIntel`] stalls the
    ///stream of commands until all previously emitted commands have completed
    ///and all caches been flushed and invalidated.
    PerformanceOverrideTypeFlushGpuCachesIntel = 1,
}
impl const Default for PerformanceOverrideTypeINTEL {
    fn default() -> Self {
        PerformanceOverrideTypeNullHardwareIntel
    }
}
impl PerformanceOverrideTypeINTEL {
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
///[VkPerformanceParameterTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceParameterTypeINTEL.html) - Parameters that can be queried
///# C Specifications
///Possible values of [`GetPerformanceParameterINTEL`]`::parameter`,
///specifying a performance query feature, are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceParameterTypeINTEL {
///    VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL = 0,
///    VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL = 1,
///} VkPerformanceParameterTypeINTEL;
///```
///# Description
/// - [`PerformanceParameterTypeHwCountersSupportedIntel`] has a boolean result which tells whether
///   hardware counters can be captured.
/// - [`PerformanceParameterTypeStreamMarkerValidBitsIntel`] has a 32 bits integer result which
///   tells how many bits can be written into the [`PerformanceValueINTEL`] value.
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`GetPerformanceParameterINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceParameterTypeINTEL")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum PerformanceParameterTypeINTEL {
    ///[`PerformanceParameterTypeHwCountersSupportedIntel`] has a
    ///boolean result which tells whether hardware counters can be captured.
    PerformanceParameterTypeHwCountersSupportedIntel = 0,
    ///[`PerformanceParameterTypeStreamMarkerValidBitsIntel`] has a
    ///32 bits integer result which tells how many bits can be written into the
    ///[`PerformanceValueINTEL`] value.
    PerformanceParameterTypeStreamMarkerValidBitsIntel = 1,
}
impl const Default for PerformanceParameterTypeINTEL {
    fn default() -> Self {
        PerformanceParameterTypeHwCountersSupportedIntel
    }
}
impl PerformanceParameterTypeINTEL {
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
///[VkPerformanceValueTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueTypeINTEL.html) - Type of the parameters that can be queried
///# C Specifications
///Possible values of [`PerformanceValueINTEL::type_`], specifying the
///type of the data returned in [`PerformanceValueINTEL::data`], are:
/// - [`PerformanceValueTypeUint32Intel`] specifies that unsigned 32-bit integer data is returned in
///   `data.value32`.
/// - [`PerformanceValueTypeUint64Intel`] specifies that unsigned 64-bit integer data is returned in
///   `data.value64`.
/// - [`PerformanceValueTypeFloatIntel`] specifies that floating-point data is returned in
///   `data.valueFloat`.
/// - [`PerformanceValueTypeBoolIntel`] specifies that [`Bool32`] data is returned in
///   `data.valueBool`.
/// - [`PerformanceValueTypeStringIntel`] specifies that a pointer to a null-terminated UTF-8 string
///   is returned in `data.valueString`. The pointer is valid for the lifetime of the `device`
///   parameter passed to [`GetPerformanceParameterINTEL`].
///
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceValueTypeINTEL {
///    VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL = 0,
///    VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL = 1,
///    VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL = 2,
///    VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL = 3,
///    VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL = 4,
///} VkPerformanceValueTypeINTEL;
///```
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceValueINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceValueTypeINTEL")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum PerformanceValueTypeINTEL {
    ///No documentation found
    PerformanceValueTypeUint32Intel = 0,
    ///No documentation found
    PerformanceValueTypeUint64Intel = 1,
    ///No documentation found
    PerformanceValueTypeFloatIntel = 2,
    ///No documentation found
    PerformanceValueTypeBoolIntel = 3,
    ///No documentation found
    PerformanceValueTypeStringIntel = 4,
}
impl const Default for PerformanceValueTypeINTEL {
    fn default() -> Self {
        PerformanceValueTypeUint32Intel
    }
}
impl PerformanceValueTypeINTEL {
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
///[VkPerformanceValueINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueINTEL.html) - Container for value and types of parameters that can be queried
///# C Specifications
///The [`PerformanceValueINTEL`] structure is defined as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceValueINTEL {
///    VkPerformanceValueTypeINTEL    type;
///    VkPerformanceValueDataINTEL    data;
///} VkPerformanceValueINTEL;
///```
///# Members
/// - [`type_`] is a [`PerformanceValueTypeINTEL`] value specifying the type of the returned data.
/// - [`data`] is a [`PerformanceValueDataINTEL`] union specifying the value of the returned data.
///# Description
///Valid Usage (Implicit)
/// - [`type_`]**must** be a valid [`PerformanceValueTypeINTEL`] value
/// - If [`type_`] is `VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL`, the `valueString` member of
///   [`data`]**must** be a null-terminated UTF-8 string
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceValueDataINTEL`]
/// - [`PerformanceValueTypeINTEL`]
/// - [`GetPerformanceParameterINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PerformanceValueINTEL<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`type_`] is a [`PerformanceValueTypeINTEL`] value specifying the
    ///type of the returned data.
    type_: PerformanceValueTypeINTEL,
    ///[`data`] is a [`PerformanceValueDataINTEL`] union specifying the
    ///value of the returned data.
    data: PerformanceValueDataINTEL<'lt>,
}
impl<'lt> Default for PerformanceValueINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            type_: Default::default(),
            data: Default::default(),
        }
    }
}
impl<'lt> PerformanceValueINTEL<'lt> {
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> PerformanceValueTypeINTEL {
        self.type_
    }
    ///Gets the value of [`Self::data`]
    pub fn data(&self) -> PerformanceValueDataINTEL<'lt> {
        self.data
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type__mut(&mut self) -> &mut PerformanceValueTypeINTEL {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::data`]
    pub fn data_mut(&mut self) -> &mut PerformanceValueDataINTEL<'lt> {
        &mut self.data
    }
    ///Sets the raw value of [`Self::type_`]
    pub fn set_type_(
        &mut self,
        value: crate::extensions::intel_performance_query::PerformanceValueTypeINTEL,
    ) -> &mut Self {
        self.type_ = value;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data(
        &mut self,
        value: crate::extensions::intel_performance_query::PerformanceValueDataINTEL<'lt>,
    ) -> &mut Self {
        self.data = value;
        self
    }
}
///[VkInitializePerformanceApiInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html) - Structure specifying parameters of initialize of the device
///# C Specifications
///The [`InitializePerformanceApiInfoINTEL`] structure is defined as :
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkInitializePerformanceApiInfoINTEL {
///    VkStructureType    sType;
///    const void*        pNext;
///    void*              pUserData;
///} VkInitializePerformanceApiInfoINTEL;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`user_data`] is a pointer for application data.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`StructureType`]
/// - [`InitializePerformanceApiINTEL`]
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
pub struct InitializePerformanceApiInfoINTEL<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`user_data`] is a pointer for application data.
    user_data: *mut c_void,
}
impl<'lt> Default for InitializePerformanceApiInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            user_data: std::ptr::null_mut(),
        }
    }
}
impl<'lt> InitializePerformanceApiInfoINTEL<'lt> {
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
    ///Sets the raw value of [`Self::user_data`]
    pub fn set_user_data(&mut self, value: &'lt mut std::ffi::c_void) -> &mut Self {
        self.user_data = value as *mut _;
        self
    }
}
///[VkQueryPoolPerformanceQueryCreateInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html) - Structure specifying parameters to create a pool of performance queries
///# C Specifications
///The [`QueryPoolPerformanceQueryCreateInfoINTEL`] structure is defined
///as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkQueryPoolSamplingModeINTEL    performanceCountersSampling;
///} VkQueryPoolPerformanceQueryCreateInfoINTEL;
///```
///
///```c
///// Provided by VK_INTEL_performance_query
///typedef VkQueryPoolPerformanceQueryCreateInfoINTEL VkQueryPoolCreateInfoINTEL;
///```
///# Members
///To create a pool for Intel performance queries, set
///[`QueryPoolCreateInfo::query_type`] to
///`VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL` and add a
///[`QueryPoolPerformanceQueryCreateInfoINTEL`] structure to the
///[`p_next`] chain of the [`QueryPoolCreateInfo`] structure.
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`performance_counters_sampling`] describe how performance queries should be captured.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL`
/// - [`performance_counters_sampling`]**must** be a valid [`QueryPoolSamplingModeINTEL`] value
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`QueryPoolSamplingModeINTEL`]
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
pub struct QueryPoolPerformanceQueryCreateInfoINTEL<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`performance_counters_sampling`] describe how performance queries
    ///should be captured.
    performance_counters_sampling: QueryPoolSamplingModeINTEL,
}
impl<'lt> Default for QueryPoolPerformanceQueryCreateInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            performance_counters_sampling: Default::default(),
        }
    }
}
impl<'lt> QueryPoolPerformanceQueryCreateInfoINTEL<'lt> {
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
    ///Gets the value of [`Self::performance_counters_sampling`]
    pub fn performance_counters_sampling(&self) -> QueryPoolSamplingModeINTEL {
        self.performance_counters_sampling
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::performance_counters_sampling`]
    pub fn performance_counters_sampling_mut(&mut self) -> &mut QueryPoolSamplingModeINTEL {
        &mut self.performance_counters_sampling
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
    ///Sets the raw value of [`Self::performance_counters_sampling`]
    pub fn set_performance_counters_sampling(
        &mut self,
        value: crate::extensions::intel_performance_query::QueryPoolSamplingModeINTEL,
    ) -> &mut Self {
        self.performance_counters_sampling = value;
        self
    }
}
///[VkPerformanceMarkerInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceMarkerInfoINTEL.html) - Structure specifying performance markers
///# C Specifications
///The [`PerformanceMarkerInfoINTEL`] structure is defined as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceMarkerInfoINTEL {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint64_t           marker;
///} VkPerformanceMarkerInfoINTEL;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`marker`] is the marker value that will be recorded into the opaque query results.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`StructureType`]
/// - [`CmdSetPerformanceMarkerINTEL`]
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
pub struct PerformanceMarkerInfoINTEL<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`marker`] is the marker value that will be recorded into the opaque
    ///query results.
    marker: u64,
}
impl<'lt> Default for PerformanceMarkerInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            marker: 0,
        }
    }
}
impl<'lt> PerformanceMarkerInfoINTEL<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::marker`]
    pub fn marker_raw(&self) -> u64 {
        self.marker
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::marker`]
    pub fn set_marker_raw(&mut self, value: u64) -> &mut Self {
        self.marker = value;
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
    ///Gets the value of [`Self::marker`]
    pub fn marker(&self) -> u64 {
        self.marker
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::marker`]
    pub fn marker_mut(&mut self) -> &mut u64 {
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
    ///Sets the raw value of [`Self::marker`]
    pub fn set_marker(&mut self, value: u64) -> &mut Self {
        self.marker = value;
        self
    }
}
///[VkPerformanceStreamMarkerInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html) - Structure specifying stream performance markers
///# C Specifications
///The [`PerformanceStreamMarkerInfoINTEL`] structure is defined as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceStreamMarkerInfoINTEL {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           marker;
///} VkPerformanceStreamMarkerInfoINTEL;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`marker`] is the marker value that will be recorded into the reports consumed by an external
///   application.
///# Description
///Valid Usage
/// - The value written by the application into [`marker`]**must** only used the valid bits as
///   reported by [`GetPerformanceParameterINTEL`] with the
///   `VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`StructureType`]
/// - [`CmdSetPerformanceStreamMarkerINTEL`]
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
pub struct PerformanceStreamMarkerInfoINTEL<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`marker`] is the marker value that will be recorded into the reports
    ///consumed by an external application.
    marker: u32,
}
impl<'lt> Default for PerformanceStreamMarkerInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            marker: 0,
        }
    }
}
impl<'lt> PerformanceStreamMarkerInfoINTEL<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::marker`]
    pub fn marker_raw(&self) -> u32 {
        self.marker
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::marker`]
    pub fn set_marker_raw(&mut self, value: u32) -> &mut Self {
        self.marker = value;
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
    ///Gets the value of [`Self::marker`]
    pub fn marker(&self) -> u32 {
        self.marker
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::marker`]
    pub fn marker_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::marker`]
    pub fn set_marker(&mut self, value: u32) -> &mut Self {
        self.marker = value;
        self
    }
}
///[VkPerformanceOverrideInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideInfoINTEL.html) - Performance override information
///# C Specifications
///The [`PerformanceOverrideInfoINTEL`] structure is defined as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceOverrideInfoINTEL {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkPerformanceOverrideTypeINTEL    type;
///    VkBool32                          enable;
///    uint64_t                          parameter;
///} VkPerformanceOverrideInfoINTEL;
///```
///# Members
/// - [`type_`] is the particular [`PerformanceOverrideTypeINTEL`] to set.
/// - [`enable`] defines whether the override is enabled.
/// - [`parameter`] is a potential required parameter for the override.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL`
/// - [`p_next`]**must** be `NULL`
/// - [`type_`]**must** be a valid [`PerformanceOverrideTypeINTEL`] value
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`Bool32`]
/// - [`PerformanceOverrideTypeINTEL`]
/// - [`StructureType`]
/// - [`CmdSetPerformanceOverrideINTEL`]
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
pub struct PerformanceOverrideInfoINTEL<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseInStructure<'lt>,
    ///[`type_`] is the particular [`PerformanceOverrideTypeINTEL`] to
    ///set.
    type_: PerformanceOverrideTypeINTEL,
    ///[`enable`] defines whether the override is enabled.
    enable: Bool32,
    ///[`parameter`] is a potential required parameter for the override.
    parameter: u64,
}
impl<'lt> Default for PerformanceOverrideInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            type_: Default::default(),
            enable: 0,
            parameter: 0,
        }
    }
}
impl<'lt> PerformanceOverrideInfoINTEL<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::enable`]
    pub fn enable_raw(&self) -> Bool32 {
        self.enable
    }
    ///Gets the raw value of [`Self::parameter`]
    pub fn parameter_raw(&self) -> u64 {
        self.parameter
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::enable`]
    pub fn set_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.enable = value;
        self
    }
    ///Sets the raw value of [`Self::parameter`]
    pub fn set_parameter_raw(&mut self, value: u64) -> &mut Self {
        self.parameter = value;
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
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> PerformanceOverrideTypeINTEL {
        self.type_
    }
    ///Gets the value of [`Self::enable`]
    pub fn enable(&self) -> bool {
        unsafe { std::mem::transmute(self.enable as u8) }
    }
    ///Gets the value of [`Self::parameter`]
    pub fn parameter(&self) -> u64 {
        self.parameter
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type__mut(&mut self) -> &mut PerformanceOverrideTypeINTEL {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::enable`]
    pub fn enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.enable as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::parameter`]
    pub fn parameter_mut(&mut self) -> &mut u64 {
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
    ///Sets the raw value of [`Self::type_`]
    pub fn set_type_(
        &mut self,
        value: crate::extensions::intel_performance_query::PerformanceOverrideTypeINTEL,
    ) -> &mut Self {
        self.type_ = value;
        self
    }
    ///Sets the raw value of [`Self::enable`]
    pub fn set_enable(&mut self, value: bool) -> &mut Self {
        self.enable = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::parameter`]
    pub fn set_parameter(&mut self, value: u64) -> &mut Self {
        self.parameter = value;
        self
    }
}
///[VkPerformanceConfigurationAcquireInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html) - Acquire a configuration to capture performance data
///# C Specifications
///The [`PerformanceConfigurationAcquireInfoINTEL`] structure is defined
///as:
///```c
///// Provided by VK_INTEL_performance_query
///typedef struct VkPerformanceConfigurationAcquireInfoINTEL {
///    VkStructureType                        sType;
///    const void*                            pNext;
///    VkPerformanceConfigurationTypeINTEL    type;
///} VkPerformanceConfigurationAcquireInfoINTEL;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`type_`] is one of the [`PerformanceConfigurationTypeINTEL`] type of performance
///   configuration that will be acquired.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL`
/// - [`p_next`]**must** be `NULL`
/// - [`type_`]**must** be a valid [`PerformanceConfigurationTypeINTEL`] value
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceConfigurationTypeINTEL`]
/// - [`StructureType`]
/// - [`AcquirePerformanceConfigurationINTEL`]
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
pub struct PerformanceConfigurationAcquireInfoINTEL<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`type_`] is one of the [`PerformanceConfigurationTypeINTEL`] type
    ///of performance configuration that will be acquired.
    type_: PerformanceConfigurationTypeINTEL,
}
impl<'lt> Default for PerformanceConfigurationAcquireInfoINTEL<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            type_: Default::default(),
        }
    }
}
impl<'lt> PerformanceConfigurationAcquireInfoINTEL<'lt> {
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
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> PerformanceConfigurationTypeINTEL {
        self.type_
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type__mut(&mut self) -> &mut PerformanceConfigurationTypeINTEL {
        &mut self.type_
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
    ///Sets the raw value of [`Self::type_`]
    pub fn set_type_(
        &mut self,
        value: crate::extensions::intel_performance_query::PerformanceConfigurationTypeINTEL,
    ) -> &mut Self {
        self.type_ = value;
        self
    }
}
