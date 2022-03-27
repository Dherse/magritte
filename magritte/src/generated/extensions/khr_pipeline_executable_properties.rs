use crate::{
    core::MAX_DESCRIPTION_SIZE,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Pipeline, ShaderStageFlags, StructureType},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    os::raw::c_char,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_pipeline_executable_properties");
///[VkPipelineExecutableStatisticFormatKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticFormatKHR.html) - Enum describing a pipeline executable statistic
///# C Specifications
///The [`PipelineExecutableStatisticFormatKHR`] enum is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef enum VkPipelineExecutableStatisticFormatKHR {
///    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR = 0,
///    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR = 1,
///    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR = 2,
///    VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR = 3,
///} VkPipelineExecutableStatisticFormatKHR;
///```
///# Description
/// - [`PipelineExecutableStatisticFormatBool32Khr`] specifies that the statistic is returned as a
///   32-bit boolean value which **must** be either [`TRUE`] or [`FALSE`] and **should** be read
///   from the `b32` field of [`PipelineExecutableStatisticValueKHR`].
/// - [`PipelineExecutableStatisticFormatInt64Khr`] specifies that the statistic is returned as a
///   signed 64-bit integer and **should** be read from the `i64` field of
///   [`PipelineExecutableStatisticValueKHR`].
/// - [`PipelineExecutableStatisticFormatUint64Khr`] specifies that the statistic is returned as an
///   unsigned 64-bit integer and **should** be read from the `u64` field of
///   [`PipelineExecutableStatisticValueKHR`].
/// - [`PipelineExecutableStatisticFormatFloat64Khr`] specifies that the statistic is returned as a
///   64-bit floating-point value and **should** be read from the `f64` field of
///   [`PipelineExecutableStatisticValueKHR`].
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`PipelineExecutableStatisticKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineExecutableStatisticFormatKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum PipelineExecutableStatisticFormatKHR {
    ///[`PipelineExecutableStatisticFormatBool32Khr`] specifies that
    ///the statistic is returned as a 32-bit boolean value which **must** be
    ///either [`TRUE`] or [`FALSE`] and **should** be read from the
    ///`b32` field of [`PipelineExecutableStatisticValueKHR`].
    PipelineExecutableStatisticFormatBool32Khr = 0,
    ///[`PipelineExecutableStatisticFormatInt64Khr`] specifies that
    ///the statistic is returned as a signed 64-bit integer and **should** be read
    ///from the `i64` field of [`PipelineExecutableStatisticValueKHR`].
    PipelineExecutableStatisticFormatInt64Khr = 1,
    ///[`PipelineExecutableStatisticFormatUint64Khr`] specifies that
    ///the statistic is returned as an unsigned 64-bit integer and **should** be
    ///read from the `u64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    PipelineExecutableStatisticFormatUint64Khr = 2,
    ///[`PipelineExecutableStatisticFormatFloat64Khr`] specifies that
    ///the statistic is returned as a 64-bit floating-point value and **should**
    ///be read from the `f64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    PipelineExecutableStatisticFormatFloat64Khr = 3,
}
impl const Default for PipelineExecutableStatisticFormatKHR {
    fn default() -> Self {
        PipelineExecutableStatisticFormatBool32Khr
    }
}
impl PipelineExecutableStatisticFormatKHR {
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
///[VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR.html) - Structure describing whether pipeline executable properties are available
///# C Specifications
///The [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`] structure
///is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           pipelineExecutableInfo;
///} VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline_executable_info`] indicates that the implementation supports reporting properties
///   and statistics about the pipeline executables associated with a compiled pipeline.
///If the [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
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
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`pipeline_executable_info`]
    ///indicates that the implementation supports reporting properties and
    ///statistics about the pipeline executables associated with a compiled
    ///pipeline.
    pipeline_executable_info: Bool32,
}
///[VkPipelineInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineInfoKHR.html) - Structure describing a pipeline
///# C Specifications
///The [`PipelineInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkPipeline         pipeline;
///} VkPipelineInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline`] is a [`Pipeline`] handle.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`pipeline`]**must** be a valid [`Pipeline`] handle
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Pipeline`]
/// - [`StructureType`]
/// - [`GetPipelineExecutablePropertiesKHR`]
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
pub struct PipelineInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`pipeline`] is a [`Pipeline`] handle.
    pipeline: Pipeline,
}
///[VkPipelineExecutablePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutablePropertiesKHR.html) - Structure describing a pipeline executable
///# C Specifications
///The [`PipelineExecutablePropertiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineExecutablePropertiesKHR {
///    VkStructureType       sType;
///    void*                 pNext;
///    VkShaderStageFlags    stages;
///    char                  name[VK_MAX_DESCRIPTION_SIZE];
///    char                  description[VK_MAX_DESCRIPTION_SIZE];
///    uint32_t              subgroupSize;
///} VkPipelineExecutablePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stages`] is a bitmask of zero or more [`ShaderStageFlagBits`] indicating which shader stages
///   (if any) were principally used as inputs to compile this pipeline executable.
/// - [`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8
///   string which is a short human readable name for this pipeline executable.
/// - [`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated
///   UTF-8 string which is a human readable description for this pipeline executable.
/// - [`subgroup_size`] is the subgroup size with which this pipeline executable is dispatched.
///# Description
///Not all implementations have a 1:1 mapping between shader stages and
///pipeline executables and some implementations **may** reduce a given shader
///stage to fixed function hardware programming such that no pipeline
///executable is available.
///No guarantees are provided about the mapping between shader stages and
///pipeline executables and [`stages`]**should** be considered a best effort
///hint.
///Because the application **cannot** rely on the [`stages`] field to provide an
///exact description, [`name`] and [`description`] provide a human readable
///name and description which more accurately describes the given pipeline
///executable.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`ShaderStageFlags`]
/// - [`StructureType`]
/// - [`GetPipelineExecutablePropertiesKHR`]
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
pub struct PipelineExecutablePropertiesKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`stages`] is a bitmask of zero or more [`ShaderStageFlagBits`]
    ///indicating which shader stages (if any) were principally used as inputs
    ///to compile this pipeline executable.
    stages: ShaderStageFlags,
    ///[`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a short human
    ///readable name for this pipeline executable.
    name: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a human readable
    ///description for this pipeline executable.
    description: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`subgroup_size`] is the subgroup size with which this pipeline
    ///executable is dispatched.
    subgroup_size: u32,
}
///[VkPipelineExecutableInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInfoKHR.html) - Structure describing a pipeline executable to query for associated statistics or internal representations
///# C Specifications
///The [`PipelineExecutableInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineExecutableInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkPipeline         pipeline;
///    uint32_t           executableIndex;
///} VkPipelineExecutableInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`pipeline`] is the pipeline to query.
/// - [`executable_index`] is the index of the pipeline executable to query in the array of
///   executable properties returned by [`GetPipelineExecutablePropertiesKHR`].
///# Description
///Valid Usage
/// - [`executable_index`]**must** be less than the number of pipeline executables associated with
///   [`pipeline`] as returned in the `pExecutableCount` parameter of
///   [`GetPipelineExecutablePropertiesKHR`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`
/// - [`p_next`]**must** be `NULL`
/// - [`pipeline`]**must** be a valid [`Pipeline`] handle
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Pipeline`]
/// - [`StructureType`]
/// - [`GetPipelineExecutableInternalRepresentationsKHR`]
/// - [`GetPipelineExecutableStatisticsKHR`]
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
pub struct PipelineExecutableInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`pipeline`] is the pipeline to query.
    pipeline: Pipeline,
    ///[`executable_index`] is the index of the pipeline executable to query
    ///in the array of executable properties returned by
    ///[`GetPipelineExecutablePropertiesKHR`].
    executable_index: u32,
}
///[VkPipelineExecutableStatisticKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticKHR.html) - Structure describing a compile-time pipeline executable statistic
///# C Specifications
///The [`PipelineExecutableStatisticKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineExecutableStatisticKHR {
///    VkStructureType                           sType;
///    void*                                     pNext;
///    char                                      name[VK_MAX_DESCRIPTION_SIZE];
///    char                                      description[VK_MAX_DESCRIPTION_SIZE];
///    VkPipelineExecutableStatisticFormatKHR    format;
///    VkPipelineExecutableStatisticValueKHR     value;
///} VkPipelineExecutableStatisticKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8
///   string which is a short human readable name for this statistic.
/// - [`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated
///   UTF-8 string which is a human readable description for this statistic.
/// - [`format`] is a [`PipelineExecutableStatisticFormatKHR`] value specifying the format of the
///   data found in [`value`].
/// - [`value`] is the value of this statistic.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`PipelineExecutableStatisticFormatKHR`]
/// - [`PipelineExecutableStatisticValueKHR`]
/// - [`StructureType`]
/// - [`GetPipelineExecutableStatisticsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineExecutableStatisticKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a short human
    ///readable name for this statistic.
    name: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a human readable
    ///description for this statistic.
    description: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`format`] is a [`PipelineExecutableStatisticFormatKHR`] value
    ///specifying the format of the data found in [`value`].
    format: PipelineExecutableStatisticFormatKHR,
    ///[`value`] is the value of this statistic.
    value: PipelineExecutableStatisticValueKHR,
}
///[VkPipelineExecutableInternalRepresentationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html) - Structure describing the textual form of a pipeline executable internal representation
///# C Specifications
///The [`PipelineExecutableInternalRepresentationKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef struct VkPipelineExecutableInternalRepresentationKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    char               name[VK_MAX_DESCRIPTION_SIZE];
///    char               description[VK_MAX_DESCRIPTION_SIZE];
///    VkBool32           isText;
///    size_t             dataSize;
///    void*              pData;
///} VkPipelineExecutableInternalRepresentationKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8
///   string which is a short human readable name for this internal representation.
/// - [`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated
///   UTF-8 string which is a human readable description for this internal representation.
/// - [`is_text`] specifies whether the returned data is text or opaque data. If [`is_text`] is
///   [`TRUE`] then the data returned in [`p_data`] is text and is guaranteed to be a
///   null-terminated UTF-8 string.
/// - [`data_size`] is an integer related to the size, in bytes, of the internal representation’s
///   data, as described below.
/// - [`p_data`] is either `NULL` or a pointer to a block of data into which the implementation will
///   write the internal representation.
///# Description
///If [`p_data`] is `NULL`, then the size, in bytes, of the internal
///representation data is returned in [`data_size`].
///Otherwise, [`data_size`] must be the size of the buffer, in bytes, pointed
///to by [`p_data`] and on return [`data_size`] is overwritten with the
///number of bytes of data actually written to [`p_data`] including any
///trailing null character.
///If [`data_size`] is less than the size, in bytes, of the internal
///representation’s data, at most [`data_size`] bytes of data will be written
///to [`p_data`], and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available representation was
///returned.If [`is_text`] is [`TRUE`] and [`p_data`] is not `NULL` and
///[`data_size`] is not zero, the last byte written to [`p_data`] will be a
///null character.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Bool32`]
/// - [`StructureType`]
/// - [`GetPipelineExecutableInternalRepresentationsKHR`]
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
pub struct PipelineExecutableInternalRepresentationKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a short human
    ///readable name for this internal representation.
    name: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a human readable
    ///description for this internal representation.
    description: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`is_text`] specifies whether the returned data is text or opaque data.
    ///If [`is_text`] is [`TRUE`] then the data returned in [`p_data`]
    ///is text and is guaranteed to be a null-terminated UTF-8 string.
    is_text: Bool32,
    ///[`data_size`] is an integer related to the size, in bytes, of the
    ///internal representation’s data, as described below.
    data_size: usize,
    ///[`p_data`] is either `NULL` or a pointer to a block of data into which
    ///the implementation will write the internal representation.
    p_data: *const c_void,
}
