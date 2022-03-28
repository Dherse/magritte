//![VK_KHR_pipeline_executable_properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_executable_properties.html) - device extension
//!# Description
//!When a pipeline is created, its state and shaders are compiled into zero or
//!more device-specific executables, which are used when executing commands
//!against that pipeline.
//!This extension adds a mechanism to query properties and statistics about the
//!different executables produced by the pipeline compilation process.
//!This is intended to be used by debugging and performance tools to allow them
//!to provide more detailed information to the user.
//!Certain compile-time shader statistics provided through this extension may
//!be useful to developers for debugging or performance analysis.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jason Ekstrand [jekstrand](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_pipeline_executable_properties]
//!   @jekstrand%0A<<Here describe the issue or question you have about the
//!   VK_KHR_pipeline_executable_properties extension>>)
//!# New functions & commands
//! - [`GetPipelineExecutableInternalRepresentationsKHR`]
//! - [`GetPipelineExecutablePropertiesKHR`]
//! - [`GetPipelineExecutableStatisticsKHR`]
//!# New structures
//! - [`PipelineExecutableInfoKHR`]
//! - [`PipelineExecutableInternalRepresentationKHR`]
//! - [`PipelineExecutablePropertiesKHR`]
//! - [`PipelineExecutableStatisticKHR`]
//! - [`PipelineInfoKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]
//!# New enums
//! - [`PipelineExecutableStatisticFormatKHR`]
//!# New constants
//! - [`KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME`]
//! - [`KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION`]
//! - Extending [`PipelineCreateFlagBits`]:  -
//!   `VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR`  -
//!   `VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`  - `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) What should we call the pieces of the pipeline which are produced by the
//!compilation process and about which you can query properties and statistics? **RESOLVED** : Call
//! them “executables”.
//!The name “binary” was used in early drafts of the extension but it was
//!determined that “pipeline binary” could have a fairly broad meaning (such
//!as a binary serialized form of an entire pipeline) and was too big of a
//!namespace for the very specific needs of this extension.
//!# Version History
//! - Revision 1, 2019-05-28 (Jason Ekstrand)  - Initial draft
//!# Other info
//! * 2019-05-28
//! * No known IP claims.
//! * - Jason Ekstrand, Intel  - Ian Romanick, Intel  - Kenneth Graunke, Intel  - Baldur Karlsson,
//!   Valve  - Jesse Hall, Google  - Jeff Bolz, Nvidia  - Piers Daniel, Nvidia  - Tobias Hector, AMD
//!   - Jan-Harald Fredriksen, ARM  - Tom Olson, ARM  - Daniel Koch, Nvidia  - Spencer Fricke,
//!   Samsung
//!# Related
//! - [`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`]
//! - [`PipelineExecutableInfoKHR`]
//! - [`PipelineExecutableInternalRepresentationKHR`]
//! - [`PipelineExecutablePropertiesKHR`]
//! - [`PipelineExecutableStatisticFormatKHR`]
//! - [`PipelineExecutableStatisticKHR`]
//! - [`PipelineExecutableStatisticValueKHR`]
//! - [`PipelineInfoKHR`]
//! - [`GetPipelineExecutableInternalRepresentationsKHR`]
//! - [`GetPipelineExecutablePropertiesKHR`]
//! - [`GetPipelineExecutableStatisticsKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
///   32-bit boolean value which  **must**  be either [`TRUE`] or [`FALSE`] and  **should**  be read
///   from the `b32` field of [`PipelineExecutableStatisticValueKHR`].
/// - [`PipelineExecutableStatisticFormatInt64Khr`] specifies that the statistic is returned as a
///   signed 64-bit integer and  **should**  be read from the `i64` field of
///   [`PipelineExecutableStatisticValueKHR`].
/// - [`PipelineExecutableStatisticFormatUint64Khr`] specifies that the statistic is returned as an
///   unsigned 64-bit integer and  **should**  be read from the `u64` field of
///   [`PipelineExecutableStatisticValueKHR`].
/// - [`PipelineExecutableStatisticFormatFloat64Khr`] specifies that the statistic is returned as a
///   64-bit floating-point value and  **should**  be read from the `f64` field of
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
#[doc(alias = "VkPipelineExecutableStatisticFormatKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum PipelineExecutableStatisticFormatKHR {
    ///[`PipelineExecutableStatisticFormatBool32Khr`] specifies that
    ///the statistic is returned as a 32-bit boolean value which  **must**  be
    ///either [`TRUE`] or [`FALSE`] and  **should**  be read from the
    ///`b32` field of [`PipelineExecutableStatisticValueKHR`].
    PipelineExecutableStatisticFormatBool32Khr = 0,
    ///[`PipelineExecutableStatisticFormatInt64Khr`] specifies that
    ///the statistic is returned as a signed 64-bit integer and  **should**  be read
    ///from the `i64` field of [`PipelineExecutableStatisticValueKHR`].
    PipelineExecutableStatisticFormatInt64Khr = 1,
    ///[`PipelineExecutableStatisticFormatUint64Khr`] specifies that
    ///the statistic is returned as an unsigned 64-bit integer and  **should**  be
    ///read from the `u64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    PipelineExecutableStatisticFormatUint64Khr = 2,
    ///[`PipelineExecutableStatisticFormatFloat64Khr`] specifies that
    ///the statistic is returned as a 64-bit floating-point value and  **should**
    ///be read from the `f64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    PipelineExecutableStatisticFormatFloat64Khr = 3,
}
impl const Default for PipelineExecutableStatisticFormatKHR {
    fn default() -> Self {
        Self::PipelineExecutableStatisticFormatBool32Khr
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
///[`PhysicalDevicePipelineExecutablePropertiesFeaturesKHR`] **can**  also be used in the
/// [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`pipeline_executable_info`]
    ///indicates that the implementation supports reporting properties and
    ///statistics about the pipeline executables associated with a compiled
    ///pipeline.
    pub pipeline_executable_info: Bool32,
}
impl<'lt> Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            pipeline_executable_info: 0,
        }
    }
}
impl<'lt> PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::pipeline_executable_info`]
    pub fn pipeline_executable_info_raw(&self) -> Bool32 {
        self.pipeline_executable_info
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_executable_info`]
    pub fn set_pipeline_executable_info_raw(&mut self, value: Bool32) -> &mut Self {
        self.pipeline_executable_info = value;
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
    ///Gets the value of [`Self::pipeline_executable_info`]
    pub fn pipeline_executable_info(&self) -> bool {
        unsafe { std::mem::transmute(self.pipeline_executable_info as u8) }
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
    ///Gets a mutable reference to the value of [`Self::pipeline_executable_info`]
    pub fn pipeline_executable_info_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.pipeline_executable_info as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.pipeline_executable_info as *mut Bool32)
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
    ///Sets the raw value of [`Self::pipeline_executable_info`]
    pub fn set_pipeline_executable_info(&mut self, value: bool) -> &mut Self {
        self.pipeline_executable_info = value as u8 as u32;
        self
    }
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
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
#[doc(alias = "VkPipelineInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineInfoKHR<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`pipeline`] is a [`Pipeline`] handle.
    pub pipeline: Pipeline,
}
impl<'lt> Default for PipelineInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            pipeline: Default::default(),
        }
    }
}
impl<'lt> PipelineInfoKHR<'lt> {
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
    ///Gets the value of [`Self::pipeline`]
    pub fn pipeline(&self) -> Pipeline {
        self.pipeline
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::pipeline`]
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
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
    ///Sets the raw value of [`Self::pipeline`]
    pub fn set_pipeline(&mut self, value: crate::vulkan1_0::Pipeline) -> &mut Self {
        self.pipeline = value;
        self
    }
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
///pipeline executables and some implementations  **may**  reduce a given shader
///stage to fixed function hardware programming such that no pipeline
///executable is available.
///No guarantees are provided about the mapping between shader stages and
///pipeline executables and [`stages`] **should**  be considered a best effort
///hint.
///Because the application  **cannot**  rely on the [`stages`] field to provide an
///exact description, [`name`] and [`description`] provide a human readable
///name and description which more accurately describes the given pipeline
///executable.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR`
/// - [`p_next`] **must**  be `NULL`
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
#[doc(alias = "VkPipelineExecutablePropertiesKHR")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineExecutablePropertiesKHR<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`stages`] is a bitmask of zero or more [`ShaderStageFlagBits`]
    ///indicating which shader stages (if any) were principally used as inputs
    ///to compile this pipeline executable.
    pub stages: ShaderStageFlags,
    ///[`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a short human
    ///readable name for this pipeline executable.
    pub name: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a human readable
    ///description for this pipeline executable.
    pub description: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`subgroup_size`] is the subgroup size with which this pipeline
    ///executable is dispatched.
    pub subgroup_size: u32,
}
impl<'lt> Default for PipelineExecutablePropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            stages: Default::default(),
            name: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
            description: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
            subgroup_size: 0,
        }
    }
}
impl<'lt> PipelineExecutablePropertiesKHR<'lt> {
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
    ///Gets the value of [`Self::stages`]
    pub fn stages(&self) -> ShaderStageFlags {
        self.stages
    }
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::description`]
    pub fn description(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::subgroup_size`]
    pub fn subgroup_size(&self) -> u32 {
        self.subgroup_size
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
    ///Gets a mutable reference to the value of [`Self::stages`]
    pub fn stages_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.stages
    }
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::description`]
    pub fn description_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::subgroup_size`]
    pub fn subgroup_size_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::stages`]
    pub fn set_stages(&mut self, value: crate::vulkan1_0::ShaderStageFlags) -> &mut Self {
        self.stages = value;
        self
    }
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.name = value;
        self
    }
    ///Sets the raw value of [`Self::description`]
    pub fn set_description(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.description = value;
        self
    }
    ///Sets the raw value of [`Self::subgroup_size`]
    pub fn set_subgroup_size(&mut self, value: u32) -> &mut Self {
        self.subgroup_size = value;
        self
    }
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
///## Valid Usage
/// - [`executable_index`] **must**  be less than the number of pipeline executables associated with
///   [`pipeline`] as returned in the `pExecutableCount` parameter of
///   [`GetPipelineExecutablePropertiesKHR`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`pipeline`] **must**  be a valid [`Pipeline`] handle
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
#[doc(alias = "VkPipelineExecutableInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineExecutableInfoKHR<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`pipeline`] is the pipeline to query.
    pub pipeline: Pipeline,
    ///[`executable_index`] is the index of the pipeline executable to query
    ///in the array of executable properties returned by
    ///[`GetPipelineExecutablePropertiesKHR`].
    pub executable_index: u32,
}
impl<'lt> Default for PipelineExecutableInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            pipeline: Default::default(),
            executable_index: 0,
        }
    }
}
impl<'lt> PipelineExecutableInfoKHR<'lt> {
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
    ///Gets the value of [`Self::pipeline`]
    pub fn pipeline(&self) -> Pipeline {
        self.pipeline
    }
    ///Gets the value of [`Self::executable_index`]
    pub fn executable_index(&self) -> u32 {
        self.executable_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::pipeline`]
    pub fn pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }
    ///Gets a mutable reference to the value of [`Self::executable_index`]
    pub fn executable_index_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::pipeline`]
    pub fn set_pipeline(&mut self, value: crate::vulkan1_0::Pipeline) -> &mut Self {
        self.pipeline = value;
        self
    }
    ///Sets the raw value of [`Self::executable_index`]
    pub fn set_executable_index(&mut self, value: u32) -> &mut Self {
        self.executable_index = value;
        self
    }
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR`
/// - [`p_next`] **must**  be `NULL`
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
#[doc(alias = "VkPipelineExecutableStatisticKHR")]
#[derive(Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineExecutableStatisticKHR<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a short human
    ///readable name for this statistic.
    pub name: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a human readable
    ///description for this statistic.
    pub description: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`format`] is a [`PipelineExecutableStatisticFormatKHR`] value
    ///specifying the format of the data found in [`value`].
    pub format: PipelineExecutableStatisticFormatKHR,
    ///[`value`] is the value of this statistic.
    pub value: PipelineExecutableStatisticValueKHR,
}
impl<'lt> Default for PipelineExecutableStatisticKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            name: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
            description: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
            format: Default::default(),
            value: Default::default(),
        }
    }
}
impl<'lt> PipelineExecutableStatisticKHR<'lt> {
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
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::description`]
    pub fn description(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> PipelineExecutableStatisticFormatKHR {
        self.format
    }
    ///Gets the value of [`Self::value`]
    pub fn value(&self) -> PipelineExecutableStatisticValueKHR {
        self.value
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
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::description`]
    pub fn description_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut PipelineExecutableStatisticFormatKHR {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::value`]
    pub fn value_mut(&mut self) -> &mut PipelineExecutableStatisticValueKHR {
        &mut self.value
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
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.name = value;
        self
    }
    ///Sets the raw value of [`Self::description`]
    pub fn set_description(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.description = value;
        self
    }
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(
        &mut self,
        value: crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticFormatKHR,
    ) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the raw value of [`Self::value`]
    pub fn set_value(
        &mut self,
        value: crate::extensions::khr_pipeline_executable_properties::PipelineExecutableStatisticValueKHR,
    ) -> &mut Self {
        self.value = value;
        self
    }
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
///   [`TRUE`] then the data returned in [`data`] is text and is guaranteed to be a null-terminated
///   UTF-8 string.
/// - [`data_size`] is an integer related to the size, in bytes, of the internal representation’s
///   data, as described below.
/// - [`data`] is either `NULL` or a pointer to a block of data into which the implementation will
///   write the internal representation.
///# Description
///If [`data`] is `NULL`, then the size, in bytes, of the internal
///representation data is returned in [`data_size`].
///Otherwise, [`data_size`] must be the size of the buffer, in bytes, pointed
///to by [`data`] and on return [`data_size`] is overwritten with the
///number of bytes of data actually written to [`data`] including any
///trailing null character.
///If [`data_size`] is less than the size, in bytes, of the internal
///representation’s data, at most [`data_size`] bytes of data will be written
///to [`data`], and `VK_INCOMPLETE` will be returned instead of
///`VK_SUCCESS`, to indicate that not all the available representation was
///returned.If [`is_text`] is [`TRUE`] and [`data`] is not `NULL` and
///[`data_size`] is not zero, the last byte written to [`data`] will be a
///null character.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`
/// - [`p_next`] **must**  be `NULL`
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
#[doc(alias = "VkPipelineExecutableInternalRepresentationKHR")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a short human
    ///readable name for this internal representation.
    pub name: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char`
    ///containing a null-terminated UTF-8 string which is a human readable
    ///description for this internal representation.
    pub description: [c_schar; MAX_DESCRIPTION_SIZE],
    ///[`is_text`] specifies whether the returned data is text or opaque data.
    ///If [`is_text`] is [`TRUE`] then the data returned in [`data`]
    ///is text and is guaranteed to be a null-terminated UTF-8 string.
    pub is_text: Bool32,
    ///[`data_size`] is an integer related to the size, in bytes, of the
    ///internal representation’s data, as described below.
    pub data_size: usize,
    ///[`data`] is either `NULL` or a pointer to a block of data into which
    ///the implementation will write the internal representation.
    pub data: *mut c_void,
}
impl<'lt> Default for PipelineExecutableInternalRepresentationKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            name: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
            description: [b'\0' as i8; MAX_DESCRIPTION_SIZE],
            is_text: 0,
            data_size: 0,
            data: std::ptr::null_mut(),
        }
    }
}
impl<'lt> PipelineExecutableInternalRepresentationKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::is_text`]
    pub fn is_text_raw(&self) -> Bool32 {
        self.is_text
    }
    ///Gets the raw value of [`Self::data`]
    pub fn data_raw(&self) -> &*mut c_void {
        &self.data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::is_text`]
    pub fn set_is_text_raw(&mut self, value: Bool32) -> &mut Self {
        self.is_text = value;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data_raw(&mut self, value: *mut c_void) -> &mut Self {
        self.data = value;
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
    ///Gets the value of [`Self::name`]
    pub fn name(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::description`]
    pub fn description(&self) -> &[c_schar; MAX_DESCRIPTION_SIZE] {
        &getter
    }
    ///Gets the value of [`Self::is_text`]
    pub fn is_text(&self) -> bool {
        unsafe { std::mem::transmute(self.is_text as u8) }
    }
    ///Gets the value of [`Self::data_size`]
    pub fn data_size(&self) -> usize {
        self.data_size
    }
    ///Gets the value of [`Self::data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn data(&self) -> &[c_void] {
        std::slice::from_raw_parts(self.data, self.data_size as usize)
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
    ///Gets a mutable reference to the value of [`Self::name`]
    pub fn name_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::description`]
    pub fn description_mut(&mut self) -> &mut [c_schar; MAX_DESCRIPTION_SIZE] {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::is_text`]
    pub fn is_text_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.is_text as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.is_text as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::data_size`]
    pub fn data_size_mut(&mut self) -> &mut usize {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn data_mut(&mut self) -> &mut [c_void] {
        std::slice::from_raw_parts_mut(self.data, self.data_size as usize)
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
    ///Sets the raw value of [`Self::name`]
    pub fn set_name(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.name = value;
        self
    }
    ///Sets the raw value of [`Self::description`]
    pub fn set_description(&mut self, value: [std::os::raw::c_char; crate::core::MAX_DESCRIPTION_SIZE]) -> &mut Self {
        self.description = value;
        self
    }
    ///Sets the raw value of [`Self::is_text`]
    pub fn set_is_text(&mut self, value: bool) -> &mut Self {
        self.is_text = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::data_size`]
    pub fn set_data_size(&mut self, value: usize) -> &mut Self {
        self.data_size = value;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data(&mut self, value: &'lt mut [std::ffi::c_void]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.data = value.as_mut_ptr();
        self.data_size = len_;
        self
    }
}
///[VkPipelineExecutableStatisticValueKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableStatisticValueKHR.html) - A union describing a pipeline executable statistic
///# C Specifications
///The [`PipelineExecutableStatisticValueKHR`] union is defined as:
///```c
///// Provided by VK_KHR_pipeline_executable_properties
///typedef union VkPipelineExecutableStatisticValueKHR {
///    VkBool32    b32;
///    int64_t     i64;
///    uint64_t    u64;
///    double      f64;
///} VkPipelineExecutableStatisticValueKHR;
///```
///# Members
/// - [`b_32`] is the 32-bit boolean value if the [`PipelineExecutableStatisticFormatKHR`] is
///   `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR`.
/// - [`i_64`] is the signed 64-bit integer value if the [`PipelineExecutableStatisticFormatKHR`] is
///   `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR`.
/// - [`u_64`] is the unsigned 64-bit integer value if the [`PipelineExecutableStatisticFormatKHR`]
///   is `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR`.
/// - [`f_64`] is the 64-bit floating-point value if the [`PipelineExecutableStatisticFormatKHR`] is
///   `VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR`.
///# Related
/// - [`VK_KHR_pipeline_executable_properties`]
/// - [`Bool32`]
/// - [`PipelineExecutableStatisticKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineExecutableStatisticValueKHR")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub union PipelineExecutableStatisticValueKHR {
    ///[`b_32`] is the 32-bit boolean value if the
    ///[`PipelineExecutableStatisticFormatKHR`] is
    ///`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR`.
    pub b_32: Bool32,
    ///[`i_64`] is the signed 64-bit integer value if the
    ///[`PipelineExecutableStatisticFormatKHR`] is
    ///`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR`.
    pub i_64: i64,
    ///[`u_64`] is the unsigned 64-bit integer value if the
    ///[`PipelineExecutableStatisticFormatKHR`] is
    ///`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR`.
    pub u_64: u64,
    ///[`f_64`] is the 64-bit floating-point value if the
    ///[`PipelineExecutableStatisticFormatKHR`] is
    ///`VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR`.
    pub f_64: f64,
}
impl Default for PipelineExecutableStatisticValueKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
