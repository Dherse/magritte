use crate::{
    core::UUID_SIZE,
    vulkan1_0::{
        AccessFlags, AttachmentDescriptionFlags, AttachmentLoadOp, AttachmentStoreOp, BaseInStructure,
        BaseOutStructure, Bool32, Buffer, DependencyFlags, DeviceMemory, DeviceSize, Format, ImageAspectFlags,
        ImageCreateFlags, ImageLayout, ImageUsageFlags, ImageView, PipelineBindPoint, PipelineStageFlags,
        RenderPassCreateFlags, SampleCountFlagBits, SampleCountFlags, Semaphore, ShaderStageFlags, StructureType,
        SubpassContents, SubpassDescriptionFlags,
    },
    vulkan1_1::{PointClippingBehavior, SubgroupFeatureFlags, LUID_SIZE},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, os::raw::c_char};
///[VK_MAX_DRIVER_NAME_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_NAME_SIZE.html) - Maximum length of a physical device driver name string
///# C Specifications
///[`MAX_DRIVER_NAME_SIZE`] is the length in `char` values of an array
///containing a driver name string, as returned in
///[`PhysicalDeviceDriverProperties`]::driverName.
///```c
///#define VK_MAX_DRIVER_NAME_SIZE           256U
///```
///or the equivalent
///```c
///#define VK_MAX_DRIVER_NAME_SIZE_KHR       VK_MAX_DRIVER_NAME_SIZE
///```
///# Related
/// - [`VK_KHR_driver_properties`]
/// - [`crate::vulkan1_2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DRIVER_NAME_SIZE")]
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
///[VK_MAX_DRIVER_INFO_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_INFO_SIZE.html) - Length of a physical device driver information string
///# C Specifications
///[`MAX_DRIVER_INFO_SIZE`] is the length in `char` values of an array
///containing a driver information string, as returned in
///[`PhysicalDeviceDriverProperties`]::driverInfo.
///```c
///#define VK_MAX_DRIVER_INFO_SIZE           256U
///```
///or the equivalent
///```c
///#define VK_MAX_DRIVER_INFO_SIZE_KHR       VK_MAX_DRIVER_INFO_SIZE
///```
///# Related
/// - [`VK_KHR_driver_properties`]
/// - [`crate::vulkan1_2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DRIVER_INFO_SIZE")]
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
///[VkSemaphoreType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreType.html) - Sepcifies the type of a semaphore object
///# C Specifications
///Possible values of [`SemaphoreTypeCreateInfo::semaphore_type`],
///specifying the type of a semaphore, are:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkSemaphoreType {
///    VK_SEMAPHORE_TYPE_BINARY = 0,
///    VK_SEMAPHORE_TYPE_TIMELINE = 1,
///  // Provided by VK_KHR_timeline_semaphore
///    VK_SEMAPHORE_TYPE_BINARY_KHR = VK_SEMAPHORE_TYPE_BINARY,
///  // Provided by VK_KHR_timeline_semaphore
///    VK_SEMAPHORE_TYPE_TIMELINE_KHR = VK_SEMAPHORE_TYPE_TIMELINE,
///} VkSemaphoreType;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_timeline_semaphore
///typedef VkSemaphoreType VkSemaphoreTypeKHR;
///```
///# Description
/// - [`Binary`] specifies a *binary semaphore* type that has a boolean payload indicating whether
///   the semaphore is currently signaled or unsignaled. When created, the semaphore is in the
///   unsignaled state.
/// - [`Timeline`] specifies a *timeline semaphore* type that has a strictly increasing 64-bit
///   unsigned integer payload indicating whether the semaphore is signaled with respect to a
///   particular reference value. When created, the semaphore payload has the value given by the
///   `initialValue` field of [`SemaphoreTypeCreateInfo`].
///# Related
/// - [`VK_KHR_timeline_semaphore`]
/// - [`crate::vulkan1_2`]
/// - [`SemaphoreTypeCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSemaphoreType")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum SemaphoreType {
    ///[`Binary`] specifies a *binary semaphore* type that
    ///has a boolean payload indicating whether the semaphore is currently
    ///signaled or unsignaled.
    ///When created, the semaphore is in the unsignaled state.
    Binary = 0,
    ///[`Timeline`] specifies a *timeline semaphore* type
    ///that has a strictly increasing 64-bit unsigned integer payload
    ///indicating whether the semaphore is signaled with respect to a
    ///particular reference value.
    ///When created, the semaphore payload has the value given by the
    ///`initialValue` field of [`SemaphoreTypeCreateInfo`].
    Timeline = 1,
}
impl const Default for SemaphoreType {
    fn default() -> Self {
        Binary
    }
}
impl SemaphoreType {
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
///[VkSamplerReductionMode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionMode.html) - Specify reduction mode for texture filtering
///# C Specifications
///Reduction modes are specified by [`SamplerReductionMode`], which takes
///values:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkSamplerReductionMode {
///    VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE = 0,
///    VK_SAMPLER_REDUCTION_MODE_MIN = 1,
///    VK_SAMPLER_REDUCTION_MODE_MAX = 2,
///  // Provided by VK_EXT_sampler_filter_minmax
///    VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT = VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE,
///  // Provided by VK_EXT_sampler_filter_minmax
///    VK_SAMPLER_REDUCTION_MODE_MIN_EXT = VK_SAMPLER_REDUCTION_MODE_MIN,
///  // Provided by VK_EXT_sampler_filter_minmax
///    VK_SAMPLER_REDUCTION_MODE_MAX_EXT = VK_SAMPLER_REDUCTION_MODE_MAX,
///} VkSamplerReductionMode;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_sampler_filter_minmax
///typedef VkSamplerReductionMode VkSamplerReductionModeEXT;
///```
///# Description
/// - [`WeightedAverage`] specifies that texel values are combined by computing a weighted average of values in the footprint, using weights as specified in [the image operations chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-unnormalized-to-integer).
/// - [`Min`] specifies that texel values are combined by taking the component-wise minimum of
///   values in the footprint with non-zero weights.
/// - [`Max`] specifies that texel values are combined by taking the component-wise maximum of
///   values in the footprint with non-zero weights.
///# Related
/// - [`VK_EXT_sampler_filter_minmax`]
/// - [`crate::vulkan1_2`]
/// - [`SamplerReductionModeCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSamplerReductionMode")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum SamplerReductionMode {
    ///[`WeightedAverage`] specifies that texel
    ///values are combined by computing a weighted average of values in the
    ///footprint, using weights as specified in
    ///[the image operations chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-unnormalized-to-integer).
    WeightedAverage = 0,
    ///[`Min`] specifies that texel values are
    ///combined by taking the component-wise minimum of values in the footprint
    ///with non-zero weights.
    Min = 1,
    ///[`Max`] specifies that texel values are
    ///combined by taking the component-wise maximum of values in the footprint
    ///with non-zero weights.
    Max = 2,
}
impl const Default for SamplerReductionMode {
    fn default() -> Self {
        WeightedAverage
    }
}
impl SamplerReductionMode {
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
///[VkDriverId](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDriverId.html) - Khronos driver IDs
///# C Specifications
///Khronos driver IDs which **may** be returned in
///[`PhysicalDeviceDriverProperties::driver_id`] are:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkDriverId {
///    VK_DRIVER_ID_AMD_PROPRIETARY = 1,
///    VK_DRIVER_ID_AMD_OPEN_SOURCE = 2,
///    VK_DRIVER_ID_MESA_RADV = 3,
///    VK_DRIVER_ID_NVIDIA_PROPRIETARY = 4,
///    VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS = 5,
///    VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA = 6,
///    VK_DRIVER_ID_IMAGINATION_PROPRIETARY = 7,
///    VK_DRIVER_ID_QUALCOMM_PROPRIETARY = 8,
///    VK_DRIVER_ID_ARM_PROPRIETARY = 9,
///    VK_DRIVER_ID_GOOGLE_SWIFTSHADER = 10,
///    VK_DRIVER_ID_GGP_PROPRIETARY = 11,
///    VK_DRIVER_ID_BROADCOM_PROPRIETARY = 12,
///    VK_DRIVER_ID_MESA_LLVMPIPE = 13,
///    VK_DRIVER_ID_MOLTENVK = 14,
///    VK_DRIVER_ID_COREAVI_PROPRIETARY = 15,
///    VK_DRIVER_ID_JUICE_PROPRIETARY = 16,
///    VK_DRIVER_ID_VERISILICON_PROPRIETARY = 17,
///    VK_DRIVER_ID_MESA_TURNIP = 18,
///    VK_DRIVER_ID_MESA_V3DV = 19,
///    VK_DRIVER_ID_MESA_PANVK = 20,
///    VK_DRIVER_ID_SAMSUNG_PROPRIETARY = 21,
///    VK_DRIVER_ID_MESA_VENUS = 22,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_AMD_PROPRIETARY_KHR = VK_DRIVER_ID_AMD_PROPRIETARY,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_AMD_OPEN_SOURCE_KHR = VK_DRIVER_ID_AMD_OPEN_SOURCE,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_MESA_RADV_KHR = VK_DRIVER_ID_MESA_RADV,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_NVIDIA_PROPRIETARY_KHR = VK_DRIVER_ID_NVIDIA_PROPRIETARY,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS_KHR = VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA_KHR = VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_IMAGINATION_PROPRIETARY_KHR = VK_DRIVER_ID_IMAGINATION_PROPRIETARY,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_QUALCOMM_PROPRIETARY_KHR = VK_DRIVER_ID_QUALCOMM_PROPRIETARY,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_ARM_PROPRIETARY_KHR = VK_DRIVER_ID_ARM_PROPRIETARY,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_GOOGLE_SWIFTSHADER_KHR = VK_DRIVER_ID_GOOGLE_SWIFTSHADER,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_GGP_PROPRIETARY_KHR = VK_DRIVER_ID_GGP_PROPRIETARY,
///  // Provided by VK_KHR_driver_properties
///    VK_DRIVER_ID_BROADCOM_PROPRIETARY_KHR = VK_DRIVER_ID_BROADCOM_PROPRIETARY,
///} VkDriverId;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_driver_properties
///typedef VkDriverId VkDriverIdKHR;
///```
///# Related
/// - [`VK_KHR_driver_properties`]
/// - [`crate::vulkan1_2`]
/// - [`PhysicalDeviceDriverProperties`]
/// - [`PhysicalDeviceVulkan12Properties`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDriverId")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum DriverId {
    #[doc(hidden)]
    Empty = 0,
    ///No documentation found
    AmdProprietary = 1,
    ///No documentation found
    AmdOpenSource = 2,
    ///No documentation found
    MesaRadv = 3,
    ///No documentation found
    NvidiaProprietary = 4,
    ///No documentation found
    IntelProprietaryWindows = 5,
    ///No documentation found
    IntelOpenSourceMesa = 6,
    ///No documentation found
    ImaginationProprietary = 7,
    ///No documentation found
    QualcommProprietary = 8,
    ///No documentation found
    ArmProprietary = 9,
    ///No documentation found
    GoogleSwiftshader = 10,
    ///No documentation found
    GgpProprietary = 11,
    ///No documentation found
    BroadcomProprietary = 12,
    ///No documentation found
    MesaLlvmpipe = 13,
    ///No documentation found
    Moltenvk = 14,
    ///No documentation found
    CoreaviProprietary = 15,
    ///No documentation found
    JuiceProprietary = 16,
    ///No documentation found
    VerisiliconProprietary = 17,
    ///No documentation found
    MesaTurnip = 18,
    ///No documentation found
    MesaV3Dv = 19,
    ///No documentation found
    MesaPanvk = 20,
    ///No documentation found
    SamsungProprietary = 21,
    ///No documentation found
    MesaVenus = 22,
}
impl const Default for DriverId {
    fn default() -> Self {
        Empty
    }
}
impl DriverId {
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
///[VkShaderFloatControlsIndependence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderFloatControlsIndependence.html) - Bitmask specifying whether, and how, shader float controls can be set separately
///# C Specifications
///Values which **may** be returned in the `denormBehaviorIndependence` and
///`roundingModeIndependence` fields of
///[`PhysicalDeviceFloatControlsProperties`] are:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkShaderFloatControlsIndependence {
///    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY = 0,
///    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL = 1,
///    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE = 2,
///  // Provided by VK_KHR_shader_float_controls
///    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR =
/// VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY,
///  // Provided by VK_KHR_shader_float_controls
///    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR = VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL,
///  // Provided by VK_KHR_shader_float_controls
///    VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR = VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE,
///} VkShaderFloatControlsIndependence;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_shader_float_controls
///typedef VkShaderFloatControlsIndependence VkShaderFloatControlsIndependenceKHR;
///```
///# Description
/// - [`32BitOnly`] specifies that shader float controls for 32-bit floating point **can** be set
///   independently; other bit widths **must** be set identically to each other.
/// - [`All`] specifies that shader float controls for all bit widths **can** be set independently.
/// - [`None`] specifies that shader float controls for all bit widths **must** be set identically.
///# Related
/// - [`VK_KHR_shader_float_controls`]
/// - [`crate::vulkan1_2`]
/// - [`PhysicalDeviceFloatControlsProperties`]
/// - [`PhysicalDeviceVulkan12Properties`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderFloatControlsIndependence")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ShaderFloatControlsIndependence {
    ///[`32BitOnly`] specifies that
    ///shader float controls for 32-bit floating point **can** be set
    ///independently; other bit widths **must** be set identically to each other.
    _32BitOnly = 0,
    ///[`All`] specifies that shader
    ///float controls for all bit widths **can** be set independently.
    All = 1,
    ///[`None`] specifies that shader
    ///float controls for all bit widths **must** be set identically.
    None = 2,
}
impl const Default for ShaderFloatControlsIndependence {
    fn default() -> Self {
        _32BitOnly
    }
}
impl ShaderFloatControlsIndependence {
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
///[VkConformanceVersion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConformanceVersion.html) - Structure containing the conformance test suite version the implementation is compliant with
///# C Specifications
///The conformance test suite version an implementation is compliant with is
///described with the [`ConformanceVersion`] structure:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkConformanceVersion {
///    uint8_t    major;
///    uint8_t    minor;
///    uint8_t    subminor;
///    uint8_t    patch;
///} VkConformanceVersion;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_driver_properties
///typedef VkConformanceVersion VkConformanceVersionKHR;
///```
///# Members
/// - [`major`] is the major version number of the conformance test suite.
/// - [`minor`] is the minor version number of the conformance test suite.
/// - [`subminor`] is the subminor version number of the conformance test suite.
/// - [`patch`] is the patch version number of the conformance test suite.
///# Related
/// - [`VK_KHR_driver_properties`]
/// - [`crate::vulkan1_2`]
/// - [`PhysicalDeviceDriverProperties`]
/// - [`PhysicalDeviceVulkan12Properties`]
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
pub struct ConformanceVersion {
    ///[`major`] is the major version number of the conformance test suite.
    major: u8,
    ///[`minor`] is the minor version number of the conformance test suite.
    minor: u8,
    ///[`subminor`] is the subminor version number of the conformance test
    ///suite.
    subminor: u8,
    ///[`patch`] is the patch version number of the conformance test suite.
    patch: u8,
}
///[VkPhysicalDeviceDriverProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDriverProperties.html) - Structure containing driver identification information
///# C Specifications
///The [`PhysicalDeviceDriverProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceDriverProperties {
///    VkStructureType         sType;
///    void*                   pNext;
///    VkDriverId              driverID;
///    char                    driverName[VK_MAX_DRIVER_NAME_SIZE];
///    char                    driverInfo[VK_MAX_DRIVER_INFO_SIZE];
///    VkConformanceVersion    conformanceVersion;
///} VkPhysicalDeviceDriverProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_driver_properties
///typedef VkPhysicalDeviceDriverProperties VkPhysicalDeviceDriverPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`driver_id`] is a unique identifier for the driver of the physical device.
/// - [`driver_name`] is an array of [`MAX_DRIVER_NAME_SIZE`]`char` containing a null-terminated
///   UTF-8 string which is the name of the driver.
/// - [`driver_info`] is an array of [`MAX_DRIVER_INFO_SIZE`]`char` containing a null-terminated
///   UTF-8 string with additional information about the driver.
/// - [`conformance_version`] is the version of the Vulkan conformance test this driver is
///   conformant against (see [`ConformanceVersion`]).
///If the [`PhysicalDeviceDriverProperties`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These are properties of the driver corresponding
/// to a physical device.[`driver_id`]**must** be immutable for a given driver across instances,
///processes, driver versions, and system reboots.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`
///# Related
/// - [`VK_KHR_driver_properties`]
/// - [`crate::vulkan1_2`]
/// - [`ConformanceVersion`]
/// - [`DriverId`]
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
pub struct PhysicalDeviceDriverProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    driver_id: DriverId,
    ///No documentation found
    driver_name: [c_schar; MAX_DRIVER_NAME_SIZE],
    ///No documentation found
    driver_info: [c_schar; MAX_DRIVER_INFO_SIZE],
    ///No documentation found
    conformance_version: ConformanceVersion,
}
///[VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures.html) - Structure describing the extended types subgroups support feature for an implementation
///# C Specifications
///The [`PhysicalDeviceShaderSubgroupExtendedTypesFeatures`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderSubgroupExtendedTypes;
///} VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_shader_subgroup_extended_types
///typedef VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures
/// VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_subgroup_extended_types`] is a boolean specifying whether subgroup operations can use
///   8-bit integer, 16-bit integer, 64-bit integer, 16-bit floating-point, and vectors of these types
///   in [group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations)
///   with [subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup),
///   if the implementation supports the types.
///If the [`PhysicalDeviceShaderSubgroupExtendedTypesFeatures`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderSubgroupExtendedTypesFeatures`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES`
///# Related
/// - [`VK_KHR_shader_subgroup_extended_types`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_subgroup_extended_types`] is a boolean specifying whether
    ///subgroup operations can use 8-bit integer, 16-bit integer, 64-bit
    ///integer, 16-bit floating-point, and vectors of these types in
    ///[group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) with
    ///[subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup), if the implementation
    ///supports the types.
    shader_subgroup_extended_types: Bool32,
}
///[VkPhysicalDeviceSamplerFilterMinmaxProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerFilterMinmaxProperties.html) - Structure describing sampler filter minmax limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceSamplerFilterMinmaxProperties`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           filterMinmaxSingleComponentFormats;
///    VkBool32           filterMinmaxImageComponentMapping;
///} VkPhysicalDeviceSamplerFilterMinmaxProperties;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_sampler_filter_minmax
///typedef VkPhysicalDeviceSamplerFilterMinmaxProperties
/// VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`filter_minmax_single_component_formats`] is a boolean value indicating whether a minimum set
///   of required formats support min/max filtering.
/// - [`filter_minmax_image_component_mapping`] is a boolean value indicating whether the
///   implementation supports non-identity component mapping of the image when doing min/max
///   filtering.
///If the [`PhysicalDeviceSamplerFilterMinmaxProperties`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.If [`filter_minmax_single_component_formats`] is
/// [`TRUE`], the following
///formats **must** support the
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT` feature with
///`VK_IMAGE_TILING_OPTIMAL`, if they support
///`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`:
/// - `VK_FORMAT_R8_UNORM`
/// - `VK_FORMAT_R8_SNORM`
/// - `VK_FORMAT_R16_UNORM`
/// - `VK_FORMAT_R16_SNORM`
/// - `VK_FORMAT_R16_SFLOAT`
/// - `VK_FORMAT_R32_SFLOAT`
/// - `VK_FORMAT_D16_UNORM`
/// - `VK_FORMAT_X8_D24_UNORM_PACK32`
/// - `VK_FORMAT_D32_SFLOAT`
/// - `VK_FORMAT_D16_UNORM_S8_UINT`
/// - `VK_FORMAT_D24_UNORM_S8_UINT`
/// - `VK_FORMAT_D32_SFLOAT_S8_UINT`
///If the format is a depth/stencil format, this bit only specifies that the
///depth aspect (not the stencil aspect) of an image of this format supports
///min/max filtering, and that min/max filtering of the depth aspect is
///supported when depth compare is disabled in the sampler.If
/// [`filter_minmax_image_component_mapping`] is [`FALSE`] the component
///mapping of the image view used with min/max filtering **must** have been
///created with the `r` component set to the
///[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings).
///Only the `r` component of the sampled image value is defined and the
///other component values are undefined.
///If [`filter_minmax_image_component_mapping`] is [`TRUE`] this restriction
///does not apply and image component mapping works as normal.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES`
///# Related
/// - [`VK_EXT_sampler_filter_minmax`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceSamplerFilterMinmaxProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    filter_minmax_single_component_formats: Bool32,
    ///No documentation found
    filter_minmax_image_component_mapping: Bool32,
}
///[VkSamplerReductionModeCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionModeCreateInfo.html) - Structure specifying sampler reduction mode
///# C Specifications
///The [`SamplerReductionModeCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkSamplerReductionModeCreateInfo {
///    VkStructureType           sType;
///    const void*               pNext;
///    VkSamplerReductionMode    reductionMode;
///} VkSamplerReductionModeCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_sampler_filter_minmax
///typedef VkSamplerReductionModeCreateInfo VkSamplerReductionModeCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`reduction_mode`] is a [`SamplerReductionMode`] value controlling how texture filtering
///   combines texel values.
///# Description
///If the [`p_next`] chain of [`SamplerCreateInfo`] includes a
///[`SamplerReductionModeCreateInfo`] structure, then that structure
///includes a mode controlling how texture filtering combines texel values.If this structure is not
/// present, [`reduction_mode`] is considered to be
///`VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE`.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO`
/// - [`reduction_mode`]**must** be a valid [`SamplerReductionMode`] value
///# Related
/// - [`VK_EXT_sampler_filter_minmax`]
/// - [`crate::vulkan1_2`]
/// - [`SamplerReductionMode`]
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
pub struct SamplerReductionModeCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`reduction_mode`] is a [`SamplerReductionMode`] value controlling
    ///how texture filtering combines texel values.
    reduction_mode: SamplerReductionMode,
}
///[VkImageFormatListCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatListCreateInfo.html) - Specify that an image can: be used with a particular set of formats
///# C Specifications
///If the [`p_next`] chain of [`ImageCreateInfo`] includes a
///[`ImageFormatListCreateInfo`] structure, then that structure contains a
///list of all formats that **can** be used when creating views of this image.The
/// [`ImageFormatListCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkImageFormatListCreateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           viewFormatCount;
///    const VkFormat*    pViewFormats;
///} VkImageFormatListCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_image_format_list
///typedef VkImageFormatListCreateInfo VkImageFormatListCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`view_format_count`] is the number of entries in the [`p_view_formats`] array.
/// - [`p_view_formats`] is a pointer to an array of [`Format`] values specifying all formats which
///   **can** be used when creating views of this image.
///# Description
///If [`view_format_count`] is zero, [`p_view_formats`] is ignored and the
///image is created as if the [`ImageFormatListCreateInfo`] structure were
///not included in the [`p_next`] chain of [`ImageCreateInfo`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO`
/// - If [`view_format_count`] is not `0`, [`p_view_formats`]**must** be a valid pointer to an array
///   of [`view_format_count`] valid [`Format`] values
///# Related
/// - [`VK_KHR_image_format_list`]
/// - [`crate::vulkan1_2`]
/// - [`Format`]
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
pub struct ImageFormatListCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`view_format_count`] is the number of entries in the [`p_view_formats`]
    ///array.
    view_format_count: u32,
    ///[`p_view_formats`] is a pointer to an array of [`Format`] values
    ///specifying all formats which **can** be used when creating views of this
    ///image.
    p_view_formats: *mut Format,
}
///[VkPhysicalDeviceShaderFloat16Int8Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderFloat16Int8Features.html) - Structure describing features supported by VK_KHR_shader_float16_int8
///# C Specifications
///The [`PhysicalDeviceShaderFloat16Int8Features`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceShaderFloat16Int8Features {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderFloat16;
///    VkBool32           shaderInt8;
///} VkPhysicalDeviceShaderFloat16Int8Features;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_shader_float16_int8
///typedef VkPhysicalDeviceShaderFloat16Int8Features VkPhysicalDeviceShaderFloat16Int8FeaturesKHR;
///```
///
///```c
///// Provided by VK_KHR_shader_float16_int8
///typedef VkPhysicalDeviceShaderFloat16Int8Features VkPhysicalDeviceFloat16Int8FeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_float_16`] indicates whether 16-bit floats (halfs) are supported in shader code. This
///   also indicates whether shader modules **can** declare the `Float16` capability. However, this
///   only enables a subset of the storage classes that SPIR-V allows for the `Float16` SPIR-V
///   capability: Declaring and using 16-bit floats in the `Private`, `Workgroup` (for non-Block
///   variables), and `Function` storage classes is enabled, while declaring them in the interface
///   storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`, `Output`, and
///   `PushConstant`) is not enabled.
/// - [`shader_int_8`] indicates whether 8-bit integers (signed and unsigned) are supported in
///   shader code. This also indicates whether shader modules **can** declare the `Int8` capability.
///   However, this only enables a subset of the storage classes that SPIR-V allows for the `Int8`
///   SPIR-V capability: Declaring and using 8-bit integers in the `Private`, `Workgroup` (for
///   non-Block variables), and `Function` storage classes is enabled, while declaring them in the
///   interface storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`,
///   `Output`, and `PushConstant`) is not enabled.
///If the [`PhysicalDeviceShaderFloat16Int8Features`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderFloat16Int8Features`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES`
///# Related
/// - [`VK_KHR_shader_float16_int8`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceShaderFloat16Int8Features<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_float_16`] indicates
    ///whether 16-bit floats (halfs) are supported in shader code.
    ///This also indicates whether shader modules **can** declare the `Float16`
    ///capability.
    ///However, this only enables a subset of the storage classes that SPIR-V
    ///allows for the `Float16` SPIR-V capability: Declaring and using
    ///16-bit floats in the `Private`,
    ///`Workgroup` (for non-Block variables),
    ///and `Function` storage classes is enabled, while declaring them in
    ///the interface storage classes (e.g., `UniformConstant`, `Uniform`,
    ///`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
    ///not enabled.
    shader_float_16: Bool32,
    ///[`shader_int_8`] indicates
    ///whether 8-bit integers (signed and unsigned) are supported in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the `Int8`
    ///capability.
    ///However, this only enables a subset of the storage classes that SPIR-V
    ///allows for the `Int8` SPIR-V capability: Declaring and using 8-bit
    ///integers in the `Private`,
    ///`Workgroup` (for non-Block variables),
    ///and `Function` storage classes is enabled, while declaring them in
    ///the interface storage classes (e.g., `UniformConstant`, `Uniform`,
    ///`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
    ///not enabled.
    shader_int_8: Bool32,
}
///[VkPhysicalDeviceFloatControlsProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFloatControlsProperties.html) - Structure describing properties supported by VK_KHR_shader_float_controls
///# C Specifications
///The [`PhysicalDeviceFloatControlsProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceFloatControlsProperties {
///    VkStructureType                      sType;
///    void*                                pNext;
///    VkShaderFloatControlsIndependence    denormBehaviorIndependence;
///    VkShaderFloatControlsIndependence    roundingModeIndependence;
///    VkBool32                             shaderSignedZeroInfNanPreserveFloat16;
///    VkBool32                             shaderSignedZeroInfNanPreserveFloat32;
///    VkBool32                             shaderSignedZeroInfNanPreserveFloat64;
///    VkBool32                             shaderDenormPreserveFloat16;
///    VkBool32                             shaderDenormPreserveFloat32;
///    VkBool32                             shaderDenormPreserveFloat64;
///    VkBool32                             shaderDenormFlushToZeroFloat16;
///    VkBool32                             shaderDenormFlushToZeroFloat32;
///    VkBool32                             shaderDenormFlushToZeroFloat64;
///    VkBool32                             shaderRoundingModeRTEFloat16;
///    VkBool32                             shaderRoundingModeRTEFloat32;
///    VkBool32                             shaderRoundingModeRTEFloat64;
///    VkBool32                             shaderRoundingModeRTZFloat16;
///    VkBool32                             shaderRoundingModeRTZFloat32;
///    VkBool32                             shaderRoundingModeRTZFloat64;
///} VkPhysicalDeviceFloatControlsProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_shader_float_controls
///typedef VkPhysicalDeviceFloatControlsProperties VkPhysicalDeviceFloatControlsPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`denorm_behavior_independence`] is a [`ShaderFloatControlsIndependence`] value indicating
///   whether, and how, denorm behavior can be set independently for different bit widths.
/// - [`rounding_mode_independence`] is a [`ShaderFloatControlsIndependence`] value indicating
///   whether, and how, rounding modes can be set independently for different bit widths.
/// - [`shader_signed_zero_inf_nan_preserve_float_16`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span class="katex-html" aria-hidden="true"><span
///   class="base"><span class="strut"
///   style="height:0.66666em;vertical-align:-0.08333em;"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span>**can** be preserved in 16-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode **can**
///   be used for 16-bit floating-point types.
/// - [`shader_signed_zero_inf_nan_preserve_float_32`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span class="katex-html" aria-hidden="true"><span
///   class="base"><span style="height:0.66666em;vertical-align:-0.08333em;"
///   class="strut"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span>**can** be preserved in 32-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode **can**
///   be used for 32-bit floating-point types.
/// - [`shader_signed_zero_inf_nan_preserve_float_64`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span class="katex-html" aria-hidden="true"><span
///   class="base"><span style="height:0.66666em;vertical-align:-0.08333em;"
///   class="strut"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span>**can** be preserved in 64-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode **can**
///   be used for 64-bit floating-point types.
/// - [`shader_denorm_preserve_float_16`] is a boolean value indicating whether denormals **can** be
///   preserved in 16-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode **can** be used for 16-bit floating-point types.
/// - [`shader_denorm_preserve_float_32`] is a boolean value indicating whether denormals **can** be
///   preserved in 32-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode **can** be used for 32-bit floating-point types.
/// - [`shader_denorm_preserve_float_64`] is a boolean value indicating whether denormals **can** be
///   preserved in 64-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode **can** be used for 64-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_16`] is a boolean value indicating whether denormals
///   **can** be flushed to zero in 16-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode **can** be used for 16-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_32`] is a boolean value indicating whether denormals
///   **can** be flushed to zero in 32-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode **can** be used for 32-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_64`] is a boolean value indicating whether denormals
///   **can** be flushed to zero in 64-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode **can** be used for 64-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_16`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 16-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can** be used for 16-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_32`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 32-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can** be used for 32-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_64`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 64-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can** be used for 64-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_16`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 16-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can** be used for 16-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_32`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 32-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can** be used for 32-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_64`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 64-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can** be used for 64-bit floating-point types.
///If the [`PhysicalDeviceFloatControlsProperties`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES`
///# Related
/// - [`VK_KHR_shader_float_controls`]
/// - [`crate::vulkan1_2`]
/// - [`Bool32`]
/// - [`ShaderFloatControlsIndependence`]
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
pub struct PhysicalDeviceFloatControlsProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    denorm_behavior_independence: ShaderFloatControlsIndependence,
    ///No documentation found
    rounding_mode_independence: ShaderFloatControlsIndependence,
    ///No documentation found
    shader_signed_zero_inf_nan_preserve_float_16: Bool32,
    ///No documentation found
    shader_signed_zero_inf_nan_preserve_float_32: Bool32,
    ///No documentation found
    shader_signed_zero_inf_nan_preserve_float_64: Bool32,
    ///No documentation found
    shader_denorm_preserve_float_16: Bool32,
    ///No documentation found
    shader_denorm_preserve_float_32: Bool32,
    ///No documentation found
    shader_denorm_preserve_float_64: Bool32,
    ///No documentation found
    shader_denorm_flush_to_zero_float_16: Bool32,
    ///No documentation found
    shader_denorm_flush_to_zero_float_32: Bool32,
    ///No documentation found
    shader_denorm_flush_to_zero_float_64: Bool32,
    ///No documentation found
    shader_rounding_mode_rte_float_16: Bool32,
    ///No documentation found
    shader_rounding_mode_rte_float_32: Bool32,
    ///No documentation found
    shader_rounding_mode_rte_float_64: Bool32,
    ///No documentation found
    shader_rounding_mode_rtz_float_16: Bool32,
    ///No documentation found
    shader_rounding_mode_rtz_float_32: Bool32,
    ///No documentation found
    shader_rounding_mode_rtz_float_64: Bool32,
}
///[VkPhysicalDeviceHostQueryResetFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceHostQueryResetFeatures.html) - Structure describing whether queries can be reset from the host
///# C Specifications
///The [`PhysicalDeviceHostQueryResetFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceHostQueryResetFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           hostQueryReset;
///} VkPhysicalDeviceHostQueryResetFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_host_query_reset
///typedef VkPhysicalDeviceHostQueryResetFeatures VkPhysicalDeviceHostQueryResetFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`host_query_reset`] indicates that the implementation supports resetting queries from the
///   host with [`ResetQueryPool`].
///If the [`PhysicalDeviceHostQueryResetFeatures`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceHostQueryResetFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES`
///# Related
/// - [`VK_EXT_host_query_reset`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceHostQueryResetFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`host_query_reset`]
    ///indicates that the implementation supports resetting queries from the
    ///host with [`ResetQueryPool`].
    host_query_reset: Bool32,
}
///[VkPhysicalDeviceDescriptorIndexingFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorIndexingFeatures.html) - Structure describing descriptor indexing features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDescriptorIndexingFeatures`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceDescriptorIndexingFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderInputAttachmentArrayDynamicIndexing;
///    VkBool32           shaderUniformTexelBufferArrayDynamicIndexing;
///    VkBool32           shaderStorageTexelBufferArrayDynamicIndexing;
///    VkBool32           shaderUniformBufferArrayNonUniformIndexing;
///    VkBool32           shaderSampledImageArrayNonUniformIndexing;
///    VkBool32           shaderStorageBufferArrayNonUniformIndexing;
///    VkBool32           shaderStorageImageArrayNonUniformIndexing;
///    VkBool32           shaderInputAttachmentArrayNonUniformIndexing;
///    VkBool32           shaderUniformTexelBufferArrayNonUniformIndexing;
///    VkBool32           shaderStorageTexelBufferArrayNonUniformIndexing;
///    VkBool32           descriptorBindingUniformBufferUpdateAfterBind;
///    VkBool32           descriptorBindingSampledImageUpdateAfterBind;
///    VkBool32           descriptorBindingStorageImageUpdateAfterBind;
///    VkBool32           descriptorBindingStorageBufferUpdateAfterBind;
///    VkBool32           descriptorBindingUniformTexelBufferUpdateAfterBind;
///    VkBool32           descriptorBindingStorageTexelBufferUpdateAfterBind;
///    VkBool32           descriptorBindingUpdateUnusedWhilePending;
///    VkBool32           descriptorBindingPartiallyBound;
///    VkBool32           descriptorBindingVariableDescriptorCount;
///    VkBool32           runtimeDescriptorArray;
///} VkPhysicalDeviceDescriptorIndexingFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_descriptor_indexing
///typedef VkPhysicalDeviceDescriptorIndexingFeatures
/// VkPhysicalDeviceDescriptorIndexingFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_input_attachment_array_dynamic_indexing`] indicates whether arrays of input
///   attachments **can** be indexed by dynamically uniform integer expressions in shader code. If
///   this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`**must** be indexed only by constant integral expressions
///   when aggregated into arrays in shader code. This also indicates whether shader modules **can**
///   declare the `InputAttachmentArrayDynamicIndexing` capability.
/// - [`shader_uniform_texel_buffer_array_dynamic_indexing`] indicates whether arrays of uniform
///   texel buffers **can** be indexed by dynamically uniform integer expressions in shader code. If
///   this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`**must** be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `UniformTexelBufferArrayDynamicIndexing` capability.
/// - [`shader_storage_texel_buffer_array_dynamic_indexing`] indicates whether arrays of storage
///   texel buffers **can** be indexed by dynamically uniform integer expressions in shader code. If
///   this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`**must** be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `StorageTexelBufferArrayDynamicIndexing` capability.
/// - [`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform
///   buffers **can** be indexed by non-uniform integer expressions in shader code. If this feature
///   is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `UniformBufferArrayNonUniformIndexing` capability.
/// - [`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays of samplers or
///   sampled images **can** be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_SAMPLER`,
///   `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`**must** not
///   be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This
///   also indicates whether shader modules **can** declare the
///   `SampledImageArrayNonUniformIndexing` capability.
/// - [`shader_storage_buffer_array_non_uniform_indexing`] indicates whether arrays of storage
///   buffers **can** be indexed by non-uniform integer expressions in shader code. If this feature
///   is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `StorageBufferArrayNonUniformIndexing` capability.
/// - [`shader_storage_image_array_non_uniform_indexing`] indicates whether arrays of storage images
///   **can** be indexed by non-uniform integer expressions in shader code. If this feature is not
///   enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`**must** not be
///   indexed by non-uniform integer expressions when aggregated into arrays in shader code. This
///   also indicates whether shader modules **can** declare the
///   `StorageImageArrayNonUniformIndexing` capability.
/// - [`shader_input_attachment_array_non_uniform_indexing`] indicates whether arrays of input
///   attachments **can** be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `InputAttachmentArrayNonUniformIndexing` capability.
/// - [`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform
///   texel buffers **can** be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `UniformTexelBufferArrayNonUniformIndexing` capability.
/// - [`shader_storage_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of storage
///   texel buffers **can** be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `StorageTexelBufferArrayNonUniformIndexing` capability.
/// - [`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether the implementation
///   supports updating uniform buffer descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.
/// - [`descriptor_binding_sampled_image_update_after_bind`] indicates whether the implementation
///   supports updating sampled image descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
///   `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.
/// - [`descriptor_binding_storage_image_update_after_bind`] indicates whether the implementation
///   supports updating storage image descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
///   `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
/// - [`descriptor_binding_storage_buffer_update_after_bind`] indicates whether the implementation
///   supports updating storage buffer descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`.
/// - [`descriptor_binding_uniform_texel_buffer_update_after_bind`] indicates whether the
///   implementation supports updating uniform texel buffer descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used
///   with `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
/// - [`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates whether the
///   implementation supports updating storage texel buffer descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used
///   with `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
/// - [`descriptor_binding_update_unused_while_pending`] indicates whether the implementation
///   supports updating descriptors while the set is in use. If this feature is not enabled,
///   `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`**must** not be used.
/// - [`descriptor_binding_partially_bound`] indicates whether the implementation supports
///   statically using a descriptor set binding in which some descriptors are not valid. If this
///   feature is not enabled, `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT`**must** not be used.
/// - [`descriptor_binding_variable_descriptor_count`] indicates whether the implementation supports
///   descriptor sets with a variable-sized last binding. If this feature is not enabled,
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`**must** not be used.
/// - [`runtime_descriptor_array`] indicates whether the implementation supports the SPIR-V
///   `RuntimeDescriptorArray` capability. If this feature is not enabled, descriptors **must** not
///   be declared in runtime arrays.
///If the [`PhysicalDeviceDescriptorIndexingFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDescriptorIndexingFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES`
///# Related
/// - [`VK_EXT_descriptor_indexing`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceDescriptorIndexingFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_input_attachment_array_dynamic_indexing`] indicates whether arrays
    ///of input attachments **can** be indexed by dynamically uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`**must** be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`InputAttachmentArrayDynamicIndexing` capability.
    shader_input_attachment_array_dynamic_indexing: Bool32,
    ///[`shader_uniform_texel_buffer_array_dynamic_indexing`] indicates whether
    ///arrays of uniform texel buffers **can** be indexed by dynamically uniform
    ///integer expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`**must** be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`UniformTexelBufferArrayDynamicIndexing` capability.
    shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    ///[`shader_storage_texel_buffer_array_dynamic_indexing`] indicates whether
    ///arrays of storage texel buffers **can** be indexed by dynamically uniform
    ///integer expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`**must** be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`StorageTexelBufferArrayDynamicIndexing` capability.
    shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    ///[`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of uniform buffers **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`UniformBufferArrayNonUniformIndexing` capability.
    shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays
    ///of samplers or sampled images **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_SAMPLER`,
    ///`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
    ///`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`SampledImageArrayNonUniformIndexing` capability.
    shader_sampled_image_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of storage buffers **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`StorageBufferArrayNonUniformIndexing` capability.
    shader_storage_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_image_array_non_uniform_indexing`] indicates whether arrays
    ///of storage images **can** be indexed by non-uniform integer expressions in
    ///shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`StorageImageArrayNonUniformIndexing` capability.
    shader_storage_image_array_non_uniform_indexing: Bool32,
    ///[`shader_input_attachment_array_non_uniform_indexing`] indicates whether
    ///arrays of input attachments **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`InputAttachmentArrayNonUniformIndexing` capability.
    shader_input_attachment_array_non_uniform_indexing: Bool32,
    ///[`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of uniform texel buffers **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`UniformTexelBufferArrayNonUniformIndexing` capability.
    shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_texel_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of storage texel buffers **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`StorageTexelBufferArrayNonUniformIndexing` capability.
    shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    ///[`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether
    ///the implementation supports updating uniform buffer descriptors after a
    ///set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.
    descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_sampled_image_update_after_bind`] indicates whether the
    ///implementation supports updating sampled image descriptors after a set
    ///is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_SAMPLER`,
    ///`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
    ///`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.
    descriptor_binding_sampled_image_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_image_update_after_bind`] indicates whether the
    ///implementation supports updating storage image descriptors after a set
    ///is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
    descriptor_binding_storage_image_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_buffer_update_after_bind`] indicates whether
    ///the implementation supports updating storage buffer descriptors after a
    ///set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`.
    descriptor_binding_storage_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_uniform_texel_buffer_update_after_bind`] indicates
    ///whether the implementation supports updating uniform texel buffer
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
    descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates
    ///whether the implementation supports updating storage texel buffer
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
    descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_update_unused_while_pending`] indicates whether the
    ///implementation supports updating descriptors while the set is in use.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`**must** not be
    ///used.
    descriptor_binding_update_unused_while_pending: Bool32,
    ///[`descriptor_binding_partially_bound`] indicates whether the
    ///implementation supports statically using a descriptor set binding in
    ///which some descriptors are not valid.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT`**must** not be used.
    descriptor_binding_partially_bound: Bool32,
    ///[`descriptor_binding_variable_descriptor_count`] indicates whether the
    ///implementation supports descriptor sets with a variable-sized last
    ///binding.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`**must** not be
    ///used.
    descriptor_binding_variable_descriptor_count: Bool32,
    ///[`runtime_descriptor_array`] indicates whether the implementation
    ///supports the SPIR-V `RuntimeDescriptorArray` capability.
    ///If this feature is not enabled, descriptors **must** not be declared in
    ///runtime arrays.
    runtime_descriptor_array: Bool32,
}
///[VkPhysicalDeviceDescriptorIndexingProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorIndexingProperties.html) - Structure describing descriptor indexing properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDescriptorIndexingProperties`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceDescriptorIndexingProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxUpdateAfterBindDescriptorsInAllPools;
///    VkBool32           shaderUniformBufferArrayNonUniformIndexingNative;
///    VkBool32           shaderSampledImageArrayNonUniformIndexingNative;
///    VkBool32           shaderStorageBufferArrayNonUniformIndexingNative;
///    VkBool32           shaderStorageImageArrayNonUniformIndexingNative;
///    VkBool32           shaderInputAttachmentArrayNonUniformIndexingNative;
///    VkBool32           robustBufferAccessUpdateAfterBind;
///    VkBool32           quadDivergentImplicitLod;
///    uint32_t           maxPerStageDescriptorUpdateAfterBindSamplers;
///    uint32_t           maxPerStageDescriptorUpdateAfterBindUniformBuffers;
///    uint32_t           maxPerStageDescriptorUpdateAfterBindStorageBuffers;
///    uint32_t           maxPerStageDescriptorUpdateAfterBindSampledImages;
///    uint32_t           maxPerStageDescriptorUpdateAfterBindStorageImages;
///    uint32_t           maxPerStageDescriptorUpdateAfterBindInputAttachments;
///    uint32_t           maxPerStageUpdateAfterBindResources;
///    uint32_t           maxDescriptorSetUpdateAfterBindSamplers;
///    uint32_t           maxDescriptorSetUpdateAfterBindUniformBuffers;
///    uint32_t           maxDescriptorSetUpdateAfterBindUniformBuffersDynamic;
///    uint32_t           maxDescriptorSetUpdateAfterBindStorageBuffers;
///    uint32_t           maxDescriptorSetUpdateAfterBindStorageBuffersDynamic;
///    uint32_t           maxDescriptorSetUpdateAfterBindSampledImages;
///    uint32_t           maxDescriptorSetUpdateAfterBindStorageImages;
///    uint32_t           maxDescriptorSetUpdateAfterBindInputAttachments;
///} VkPhysicalDeviceDescriptorIndexingProperties;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_descriptor_indexing
///typedef VkPhysicalDeviceDescriptorIndexingProperties
/// VkPhysicalDeviceDescriptorIndexingPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`max_update_after_bind_descriptors_in_all_pools`] is the maximum number of descriptors
///   (summed over all descriptor types) that **can** be created across all pools that are created
///   with the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` bit set. Pool creation **may** fail
///   when this limit is exceeded, or when the space this limit represents is unable to satisfy a
///   pool creation due to fragmentation.
/// - [`shader_uniform_buffer_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether uniform buffer descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of uniform
///   buffers **may** execute multiple times in order to access all the descriptors.
/// - [`shader_sampled_image_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether sampler and image descriptors natively support nonuniform indexing. If this is
///   [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array
///   of samplers or images **may** execute multiple times in order to access all the descriptors.
/// - [`shader_storage_buffer_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether storage buffer descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of storage
///   buffers **may** execute multiple times in order to access all the descriptors.
/// - [`shader_storage_image_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether storage image descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of storage
///   images **may** execute multiple times in order to access all the descriptors.
/// - [`shader_input_attachment_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether input attachment descriptors natively support nonuniform indexing. If this is
///   [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array
///   of input attachments **may** execute multiple times in order to access all the descriptors.
/// - [`robust_buffer_access_update_after_bind`] is a boolean value indicating whether [`robustBufferAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess)**can**
///   be enabled in a device simultaneously with `descriptorBindingUniformBufferUpdateAfterBind`,
///   `descriptorBindingStorageBufferUpdateAfterBind`,
///   `descriptorBindingUniformTexelBufferUpdateAfterBind`, and/or
///   `descriptorBindingStorageTexelBufferUpdateAfterBind`. If this is [`FALSE`], then either
///   `robustBufferAccess`**must** be disabled or all of these update-after-bind features **must**
///   be disabled.
/// - [`quad_divergent_implicit_lod`] is a boolean value indicating whether implicit level of detail
///   calculations for image operations have well-defined results when the image and/or sampler objects
///   used for the instruction are not uniform within a quad. See [Derivative Image Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-derivative-image-operations).
/// - [`max_per_stage_descriptor_update_after_bind_samplers`] is similar to
///   `maxPerStageDescriptorSamplers` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_uniform_buffers`] is similar to
///   `maxPerStageDescriptorUniformBuffers` but counts descriptors from descriptor sets created with
///   or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_storage_buffers`] is similar to
///   `maxPerStageDescriptorStorageBuffers` but counts descriptors from descriptor sets created with
///   or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_sampled_images`] is similar to
///   `maxPerStageDescriptorSampledImages` but counts descriptors from descriptor sets created with
///   or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_storage_images`] is similar to
///   `maxPerStageDescriptorStorageImages` but counts descriptors from descriptor sets created with
///   or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_input_attachments`] is similar to
///   `maxPerStageDescriptorInputAttachments` but counts descriptors from descriptor sets created
///   with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_update_after_bind_resources`] is similar to `maxPerStageResources` but counts
///   descriptors from descriptor sets created with or without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_samplers`] is similar to `maxDescriptorSetSamplers` but
///   counts descriptors from descriptor sets created with or without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_uniform_buffers`] is similar to
///   `maxDescriptorSetUniformBuffers` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_uniform_buffers_dynamic`] is similar to
///   `maxDescriptorSetUniformBuffersDynamic` but counts descriptors from descriptor sets created
///   with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
///   While an application **can** allocate dynamic uniform buffer descriptors from a pool created
///   with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these
///   descriptors **must** not be present in any descriptor set layout that includes bindings
///   created with `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`.
/// - [`max_descriptor_set_update_after_bind_storage_buffers`] is similar to
///   `maxDescriptorSetStorageBuffers` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_storage_buffers_dynamic`] is similar to
///   `maxDescriptorSetStorageBuffersDynamic` but counts descriptors from descriptor sets created
///   with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
///   While an application **can** allocate dynamic storage buffer descriptors from a pool created
///   with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these
///   descriptors **must** not be present in any descriptor set layout that includes bindings
///   created with `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`.
/// - [`max_descriptor_set_update_after_bind_sampled_images`] is similar to
///   `maxDescriptorSetSampledImages` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_storage_images`] is similar to
///   `maxDescriptorSetStorageImages` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_input_attachments`] is similar to
///   `maxDescriptorSetInputAttachments` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
///If the [`PhysicalDeviceDescriptorIndexingProperties`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES`
///# Related
/// - [`VK_EXT_descriptor_indexing`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceDescriptorIndexingProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    max_update_after_bind_descriptors_in_all_pools: u32,
    ///No documentation found
    shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    shader_storage_image_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    robust_buffer_access_update_after_bind: Bool32,
    ///No documentation found
    quad_divergent_implicit_lod: Bool32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_samplers: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_storage_images: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    ///No documentation found
    max_per_stage_update_after_bind_resources: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_samplers: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_uniform_buffers: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_storage_buffers: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_sampled_images: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_storage_images: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_input_attachments: u32,
}
///[VkDescriptorSetLayoutBindingFlagsCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutBindingFlagsCreateInfo.html) - Structure specifying creation flags for descriptor set layout bindings
///# C Specifications
///If the [`p_next`] chain of a [`DescriptorSetLayoutCreateInfo`]
///structure includes a [`DescriptorSetLayoutBindingFlagsCreateInfo`]
///structure, then that structure includes an array of flags, one for each
///descriptor set layout binding.The [`DescriptorSetLayoutBindingFlagsCreateInfo`] structure is
/// defined
///as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    uint32_t                           bindingCount;
///    const VkDescriptorBindingFlags*    pBindingFlags;
///} VkDescriptorSetLayoutBindingFlagsCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_descriptor_indexing
///typedef VkDescriptorSetLayoutBindingFlagsCreateInfo
/// VkDescriptorSetLayoutBindingFlagsCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`binding_count`] is zero or the number of elements in [`p_binding_flags`].
/// - [`p_binding_flags`] is a pointer to an array of [`DescriptorBindingFlags`] bitfields, one for
///   each descriptor set layout binding.
///# Description
///If [`binding_count`] is zero or if this structure is not included in the
///[`p_next`] chain, the [`DescriptorBindingFlags`] for each descriptor
///set layout binding is considered to be zero.
///Otherwise, the descriptor set layout binding at
///[`DescriptorSetLayoutCreateInfo::p_bindings`][i] uses the flags in
///[`p_binding_flags`][i].Valid Usage
/// - If [`binding_count`] is not zero, [`binding_count`]**must** equal
///   [`DescriptorSetLayoutCreateInfo`]::[`binding_count`]
/// - If [`DescriptorSetLayoutCreateInfo::flags`] includes
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`, then all elements of
///   [`p_binding_flags`]**must** not include `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`,
///   `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`, or
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`
/// - If an element of [`p_binding_flags`] includes
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`, then all other elements of
///   [`DescriptorSetLayoutCreateInfo::p_bindings`]**must** have a smaller value of `binding`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_uniform_buffer_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`**must** not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_sampled_image_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`**must** not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_storage_image_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`**must** not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_storage_buffer_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`**must** not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_uniform_texel_buffer_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`**must** not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_storage_texel_buffer_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`**must** not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceInlineUniformBlockFeatures::
///   descriptor_binding_inline_uniform_block_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`**must** not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceAccelerationStructureFeaturesKHR::
///   descriptor_binding_acceleration_structure_update_after_bind`] is not enabled, all bindings
///   with descriptor type `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` or
///   `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV`**must** not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - All bindings with descriptor type `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`,
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, or
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`**must** not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_update_unused_while_pending`] is not enabled, all elements of
///   [`p_binding_flags`]**must** not include
///   `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_partially_bound`] is not
///   enabled, all elements of [`p_binding_flags`]**must** not include
///   `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_variable_descriptor_count`]
///   is not enabled, all elements of [`p_binding_flags`]**must** not include
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`
/// - If an element of [`p_binding_flags`] includes
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`, that element’s `descriptorType`**must**
///   not be `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` or
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO`
/// - If [`binding_count`] is not `0`, [`p_binding_flags`]**must** be a valid pointer to an array of
///   [`binding_count`] valid combinations of [`DescriptorBindingFlagBits`] values
///# Related
/// - [`VK_EXT_descriptor_indexing`]
/// - [`crate::vulkan1_2`]
/// - [`DescriptorBindingFlags`]
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
pub struct DescriptorSetLayoutBindingFlagsCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`binding_count`] is zero or the number of elements in
    ///[`p_binding_flags`].
    binding_count: u32,
    ///[`p_binding_flags`] is a pointer to an array of
    ///[`DescriptorBindingFlags`] bitfields, one for each descriptor set
    ///layout binding.
    p_binding_flags: *mut DescriptorBindingFlags,
}
///[VkDescriptorSetVariableDescriptorCountAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountAllocateInfo.html) - Structure specifying additional allocation parameters for descriptor sets
///# C Specifications
///If the [`p_next`] chain of a [`DescriptorSetAllocateInfo`] structure
///includes a [`DescriptorSetVariableDescriptorCountAllocateInfo`]
///structure, then that structure includes an array of descriptor counts for
///variable-sized descriptor bindings, one for each descriptor set being
///allocated.The [`DescriptorSetVariableDescriptorCountAllocateInfo`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           descriptorSetCount;
///    const uint32_t*    pDescriptorCounts;
///} VkDescriptorSetVariableDescriptorCountAllocateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_descriptor_indexing
///typedef VkDescriptorSetVariableDescriptorCountAllocateInfo
/// VkDescriptorSetVariableDescriptorCountAllocateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`descriptor_set_count`] is zero or the number of elements in [`p_descriptor_counts`].
/// - [`p_descriptor_counts`] is a pointer to an array of descriptor counts, with each member
///   specifying the number of descriptors in a variable-sized descriptor binding in the
///   corresponding descriptor set being allocated.
///# Description
///If [`descriptor_set_count`] is zero or this structure is not included in the
///[`p_next`] chain, then the variable lengths are considered to be zero.
///Otherwise, [`p_descriptor_counts`][i] is the number of descriptors in the
///variable-sized descriptor binding in the corresponding descriptor set
///layout.
///If the variable-sized descriptor binding in the corresponding descriptor set
///layout has a descriptor type of
///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
///[`p_descriptor_counts`][i] specifies the binding’s capacity in bytes.
///If [`DescriptorSetAllocateInfo::p_set_layouts`][i] does not include
///a variable-sized descriptor binding, then [`p_descriptor_counts`][i] is
///ignored.Valid Usage
/// - If [`descriptor_set_count`] is not zero, [`descriptor_set_count`]**must** equal
///   [`DescriptorSetAllocateInfo`]::[`descriptor_set_count`]
/// - If [`DescriptorSetAllocateInfo::p_set_layouts`][i] has a variable-sized descriptor binding,
///   then [`p_descriptor_counts`][i] **must** be less than or equal to the descriptor count
///   specified for that binding when the descriptor set layout was created
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO`
/// - If [`descriptor_set_count`] is not `0`, [`p_descriptor_counts`]**must** be a valid pointer to
///   an array of [`descriptor_set_count`]`uint32_t` values
///# Related
/// - [`VK_EXT_descriptor_indexing`]
/// - [`crate::vulkan1_2`]
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
pub struct DescriptorSetVariableDescriptorCountAllocateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`descriptor_set_count`] is zero or the number of elements in
    ///[`p_descriptor_counts`].
    descriptor_set_count: u32,
    ///[`p_descriptor_counts`] is a pointer to an array of descriptor counts,
    ///with each member specifying the number of descriptors in a
    ///variable-sized descriptor binding in the corresponding descriptor set
    ///being allocated.
    p_descriptor_counts: *mut u32,
}
///[VkDescriptorSetVariableDescriptorCountLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetVariableDescriptorCountLayoutSupport.html) - Structure returning information about whether a descriptor set layout can be supported
///# C Specifications
///If the [`p_next`] chain of a [`DescriptorSetLayoutSupport`] structure
///includes a [`DescriptorSetVariableDescriptorCountLayoutSupport`]
///structure, then that structure returns additional information about whether
///the descriptor set layout is supported.
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxVariableDescriptorCount;
///} VkDescriptorSetVariableDescriptorCountLayoutSupport;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_descriptor_indexing
///typedef VkDescriptorSetVariableDescriptorCountLayoutSupport
/// VkDescriptorSetVariableDescriptorCountLayoutSupportEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_variable_descriptor_count`] indicates the maximum number of descriptors supported in the
///   highest numbered binding of the layout, if that binding is variable-sized. If the highest
///   numbered binding of the layout has a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`max_variable_descriptor_count`] indicates the
///   maximum byte size supported for the binding, if that binding is variable-sized.
///# Description
///If the [`DescriptorSetLayoutCreateInfo`] structure specified in
///[`GetDescriptorSetLayoutSupport`]`::pCreateInfo` includes a
///variable-sized descriptor, then `supported` is determined assuming the
///requested size of the variable-sized descriptor, and
///[`max_variable_descriptor_count`] is set to the maximum size of that
///descriptor that **can** be successfully created (which is greater than or equal
///to the requested size passed in).
///If the [`DescriptorSetLayoutCreateInfo`] structure does not include a
///variable-sized descriptor, or if the
///[`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_variable_descriptor_count`]
///feature is not enabled, then [`max_variable_descriptor_count`] is set to
///zero.
///For the purposes of this command, a variable-sized descriptor binding with a
///`descriptorCount` of zero is treated as if the `descriptorCount` is
///one, and thus the binding is not ignored and the maximum descriptor count
///will be returned.
///If the layout is not supported, then the value written to
///[`max_variable_descriptor_count`] is undefined.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT`
///# Related
/// - [`VK_EXT_descriptor_indexing`]
/// - [`crate::vulkan1_2`]
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
pub struct DescriptorSetVariableDescriptorCountLayoutSupport<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_variable_descriptor_count`] indicates the maximum number of
    ///descriptors supported in the highest numbered binding of the layout, if
    ///that binding is variable-sized.
    ///If the highest numbered binding of the layout has a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
    ///[`max_variable_descriptor_count`] indicates the maximum byte size
    ///supported for the binding, if that binding is variable-sized.
    max_variable_descriptor_count: u32,
}
///[VkAttachmentDescription2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescription2.html) - Structure specifying an attachment description
///# C Specifications
///The [`AttachmentDescription2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkAttachmentDescription2 {
///    VkStructureType                 sType;
///    const void*                     pNext;
///    VkAttachmentDescriptionFlags    flags;
///    VkFormat                        format;
///    VkSampleCountFlagBits           samples;
///    VkAttachmentLoadOp              loadOp;
///    VkAttachmentStoreOp             storeOp;
///    VkAttachmentLoadOp              stencilLoadOp;
///    VkAttachmentStoreOp             stencilStoreOp;
///    VkImageLayout                   initialLayout;
///    VkImageLayout                   finalLayout;
///} VkAttachmentDescription2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_create_renderpass2
///typedef VkAttachmentDescription2 VkAttachmentDescription2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`AttachmentDescriptionFlagBits`] specifying additional properties
///   of the attachment.
/// - [`format`] is a [`Format`] value specifying the format of the image that will be used for the
///   attachment.
/// - [`samples`] is a [`SampleCountFlagBits`] value specifying the number of samples of the image.
/// - [`load_op`] is a [`AttachmentLoadOp`] value specifying how the contents of color and depth
///   components of the attachment are treated at the beginning of the subpass where it is first
///   used.
/// - [`store_op`] is a [`AttachmentStoreOp`] value specifying how the contents of color and depth
///   components of the attachment are treated at the end of the subpass where it is last used.
/// - [`stencil_load_op`] is a [`AttachmentLoadOp`] value specifying how the contents of stencil
///   components of the attachment are treated at the beginning of the subpass where it is first
///   used.
/// - [`stencil_store_op`] is a [`AttachmentStoreOp`] value specifying how the contents of stencil
///   components of the attachment are treated at the end of the last subpass where it is used.
/// - [`initial_layout`] is the layout the attachment image subresource will be in when a render
///   pass instance begins.
/// - [`final_layout`] is the layout the attachment image subresource will be transitioned to when a
///   render pass instance ends.
///# Description
///Parameters defined by this structure with the same name as those in
///[`AttachmentDescription`] have the identical effect to those parameters.If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is enabled, and [`format`] is
///a depth/stencil format, [`initial_layout`] and [`final_layout`]**can** be
///set to a layout that only specifies the layout of the depth aspect.If the [`p_next`] chain
/// includes a
///[`AttachmentDescriptionStencilLayout`] structure, then the
///`stencilInitialLayout` and `stencilFinalLayout` members specify the
///initial and final layouts of the stencil aspect of a depth/stencil format,
///and [`initial_layout`] and [`final_layout`] only apply to the depth
///aspect.
///For depth-only formats, the [`AttachmentDescriptionStencilLayout`]
///structure is ignored.
///For stencil-only formats, the initial and final layouts of the stencil
///aspect are taken from the [`AttachmentDescriptionStencilLayout`]
///structure if present, or [`initial_layout`] and [`final_layout`] if not
///present.If [`format`] is a depth/stencil format, and either [`initial_layout`] or
///[`final_layout`] does not specify a layout for the stencil aspect, then the
///application **must** specify the initial and final layouts of the stencil
///aspect by including a [`AttachmentDescriptionStencilLayout`] structure
///in the [`p_next`] chain.Valid Usage
/// - [`final_layout`]**must** not be `VK_IMAGE_LAYOUT_UNDEFINED` or
///   `VK_IMAGE_LAYOUT_PREINITIALIZED`
/// - If [`format`] is a color format, [`initial_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - If [`format`] is a depth/stencil format, [`initial_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
/// - If [`format`] is a color format, [`final_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - If [`format`] is a depth/stencil format, [`final_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is not enabled, [`initial_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is not enabled, [`final_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a color format, [`initial_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a color format, [`final_layout`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a depth/stencil format which includes both depth and stencil aspects, and
///   [`initial_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, the [`p_next`] chain **must** include a
///   [`AttachmentDescriptionStencilLayout`] structure
/// - If [`format`] is a depth/stencil format which includes both depth and stencil aspects, and
///   [`final_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, the [`p_next`] chain **must** include a
///   [`AttachmentDescriptionStencilLayout`] structure
/// - If [`format`] is a depth/stencil format which includes only the depth aspect,
///   [`initial_layout`]**must** not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a depth/stencil format which includes only the depth aspect,
///   [`final_layout`]**must** not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a depth/stencil format which includes only the stencil aspect,
///   [`initial_layout`]**must** not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`
/// - If [`format`] is a depth/stencil format which includes only the stencil aspect,
///   [`final_layout`]**must** not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is enabled and [`format`] is a depth/stencil format that includes a depth aspect and
///   the [`p_next`] chain includes a [`AttachmentDescriptionStencilLayout`] structure,
///   [`initial_layout`]**must** not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is enabled and [`format`] is a depth/stencil format that includes a depth aspect and
///   the [`p_next`] chain includes a [`AttachmentDescriptionStencilLayout`] structure,
///   [`final_layout`]**must** not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`AttachmentDescriptionStencilLayout`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`AttachmentDescriptionFlagBits`] values
/// - [`format`]**must** be a valid [`Format`] value
/// - [`samples`]**must** be a valid [`SampleCountFlagBits`] value
/// - [`load_op`]**must** be a valid [`AttachmentLoadOp`] value
/// - [`store_op`]**must** be a valid [`AttachmentStoreOp`] value
/// - [`stencil_load_op`]**must** be a valid [`AttachmentLoadOp`] value
/// - [`stencil_store_op`]**must** be a valid [`AttachmentStoreOp`] value
/// - [`initial_layout`]**must** be a valid [`ImageLayout`] value
/// - [`final_layout`]**must** be a valid [`ImageLayout`] value
///# Related
/// - [`VK_KHR_create_renderpass2`]
/// - [`crate::vulkan1_2`]
/// - [`AttachmentDescriptionFlags`]
/// - [`AttachmentLoadOp`]
/// - [`AttachmentStoreOp`]
/// - [`Format`]
/// - [`ImageLayout`]
/// - [`RenderPassCreateInfo2`]
/// - [`SampleCountFlagBits`]
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
pub struct AttachmentDescription2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`AttachmentDescriptionFlagBits`]
    ///specifying additional properties of the attachment.
    flags: AttachmentDescriptionFlags,
    ///[`format`] is a [`Format`] value specifying the format of the
    ///image that will be used for the attachment.
    format: Format,
    ///[`samples`] is a [`SampleCountFlagBits`] value specifying the
    ///number of samples of the image.
    samples: SampleCountFlagBits,
    ///[`load_op`] is a [`AttachmentLoadOp`] value specifying how the
    ///contents of color and depth components of the attachment are treated at
    ///the beginning of the subpass where it is first used.
    load_op: AttachmentLoadOp,
    ///[`store_op`] is a [`AttachmentStoreOp`] value specifying how the
    ///contents of color and depth components of the attachment are treated at
    ///the end of the subpass where it is last used.
    store_op: AttachmentStoreOp,
    ///[`stencil_load_op`] is a [`AttachmentLoadOp`] value specifying how
    ///the contents of stencil components of the attachment are treated at the
    ///beginning of the subpass where it is first used.
    stencil_load_op: AttachmentLoadOp,
    ///[`stencil_store_op`] is a [`AttachmentStoreOp`] value specifying how
    ///the contents of stencil components of the attachment are treated at the
    ///end of the last subpass where it is used.
    stencil_store_op: AttachmentStoreOp,
    ///[`initial_layout`] is the layout the attachment image subresource will
    ///be in when a render pass instance begins.
    initial_layout: ImageLayout,
    ///[`final_layout`] is the layout the attachment image subresource will be
    ///transitioned to when a render pass instance ends.
    final_layout: ImageLayout,
}
///[VkAttachmentReference2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentReference2.html) - Structure specifying an attachment reference
///# C Specifications
///The [`AttachmentReference2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkAttachmentReference2 {
///    VkStructureType       sType;
///    const void*           pNext;
///    uint32_t              attachment;
///    VkImageLayout         layout;
///    VkImageAspectFlags    aspectMask;
///} VkAttachmentReference2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_create_renderpass2
///typedef VkAttachmentReference2 VkAttachmentReference2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attachment`] is either an integer value identifying an attachment at the corresponding index
///   in [`RenderPassCreateInfo2::p_attachments`], or [`ATTACHMENT_UNUSED`] to signify that this
///   attachment is not used.
/// - [`layout`] is a [`ImageLayout`] value specifying the layout the attachment uses during the
///   subpass.
/// - [`aspect_mask`] is a mask of which aspect(s) **can** be accessed within the specified subpass
///   as an input attachment.
///# Description
///Parameters defined by this structure with the same name as those in
///[`AttachmentReference`] have the identical effect to those parameters.[`aspect_mask`] is ignored
/// when this structure is used to describe anything
///other than an input attachment reference.If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is enabled, and [`attachment`]
///has a depth/stencil format, [`layout`]**can** be set to a layout that only
///specifies the layout of the depth aspect.If [`layout`] only specifies the layout of the depth
/// aspect of the
///attachment, the layout of the stencil aspect is specified by the
///`stencilLayout` member of a [`AttachmentReferenceStencilLayout`]
///structure included in the [`p_next`] chain.
///Otherwise, [`layout`] describes the layout for all relevant image aspects.Valid Usage
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], [`layout`]**must** not be
///   `VK_IMAGE_LAYOUT_UNDEFINED`, `VK_IMAGE_LAYOUT_PREINITIALIZED`, or
///   `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is not enabled, and [`attachment`] is not [`ATTACHMENT_UNUSED`], [`layout`]**must**
///   not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`,
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is
///   a color format, [`layout`]**must** not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is
///   a depth/stencil format which includes both depth and stencil aspects, and [`layout`] is
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, the
///   [`p_next`] chain **must** include a [`AttachmentReferenceStencilLayout`] structure
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is
///   a depth/stencil format which includes only the depth aspect, [`layout`]**must** not be
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is
///   a depth/stencil format which includes only the stencil aspect, [`layout`]**must** not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`AttachmentReferenceStencilLayout`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`layout`]**must** be a valid [`ImageLayout`] value
///# Related
/// - [`VK_KHR_create_renderpass2`]
/// - [`crate::vulkan1_2`]
/// - [`FragmentShadingRateAttachmentInfoKHR`]
/// - [`ImageAspectFlags`]
/// - [`ImageLayout`]
/// - [`StructureType`]
/// - [`SubpassDescription2`]
/// - [`SubpassDescriptionDepthStencilResolve`]
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
pub struct AttachmentReference2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`attachment`] is either an integer value identifying an attachment at
    ///the corresponding index in
    ///[`RenderPassCreateInfo2`]::`pAttachments`, or
    ///[`ATTACHMENT_UNUSED`] to signify that this attachment is not used.
    attachment: u32,
    ///[`layout`] is a [`ImageLayout`] value specifying the layout the
    ///attachment uses during the subpass.
    layout: ImageLayout,
    ///[`aspect_mask`] is a mask of which aspect(s) **can** be accessed within
    ///the specified subpass as an input attachment.
    aspect_mask: ImageAspectFlags,
}
///[VkSubpassDescription2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescription2.html) - Structure specifying a subpass description
///# C Specifications
///The [`SubpassDescription2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkSubpassDescription2 {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkSubpassDescriptionFlags        flags;
///    VkPipelineBindPoint              pipelineBindPoint;
///    uint32_t                         viewMask;
///    uint32_t                         inputAttachmentCount;
///    const VkAttachmentReference2*    pInputAttachments;
///    uint32_t                         colorAttachmentCount;
///    const VkAttachmentReference2*    pColorAttachments;
///    const VkAttachmentReference2*    pResolveAttachments;
///    const VkAttachmentReference2*    pDepthStencilAttachment;
///    uint32_t                         preserveAttachmentCount;
///    const uint32_t*                  pPreserveAttachments;
///} VkSubpassDescription2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_create_renderpass2
///typedef VkSubpassDescription2 VkSubpassDescription2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`SubpassDescriptionFlagBits`] specifying usage of the subpass.
/// - [`pipeline_bind_point`] is a [`PipelineBindPoint`] value specifying the pipeline type
///   supported for this subpass.
/// - [`view_mask`] is a bitfield of view indices describing which views rendering is broadcast to
///   in this subpass, when multiview is enabled.
/// - [`input_attachment_count`] is the number of input attachments.
/// - [`p_input_attachments`] is a pointer to an array of [`AttachmentReference2`] structures
///   defining the input attachments for this subpass and their layouts.
/// - [`color_attachment_count`] is the number of color attachments.
/// - [`p_color_attachments`] is a pointer to an array of
///   [`color_attachment_count`][`AttachmentReference2`] structures defining the color attachments
///   for this subpass and their layouts.
/// - [`p_resolve_attachments`] is `NULL` or a pointer to an array of
///   [`color_attachment_count`][`AttachmentReference2`] structures defining the resolve attachments
///   for this subpass and their layouts.
/// - [`p_depth_stencil_attachment`] is a pointer to a [`AttachmentReference2`] structure specifying
///   the depth/stencil attachment for this subpass and its layout.
/// - [`preserve_attachment_count`] is the number of preserved attachments.
/// - [`p_preserve_attachments`] is a pointer to an array of [`preserve_attachment_count`] render
///   pass attachment indices identifying attachments that are not used by this subpass, but whose
///   contents **must** be preserved throughout the subpass.
///# Description
///Parameters defined by this structure with the same name as those in
///[`SubpassDescription`] have the identical effect to those parameters.[`view_mask`] has the same
/// effect for the described subpass as
///[`RenderPassMultiviewCreateInfo::p_view_masks`] has on each
///corresponding subpass.If a [`FragmentShadingRateAttachmentInfoKHR`] structure is included in
///the [`p_next`] chain, `pFragmentShadingRateAttachment` is not `NULL`,
///and its `attachment` member is not [`ATTACHMENT_UNUSED`], the
///identified attachment defines a fragment shading rate attachment for that
///subpass.Valid Usage
/// - [`pipeline_bind_point`]**must** be `VK_PIPELINE_BIND_POINT_GRAPHICS` or
///   `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`
/// - [`color_attachment_count`]**must** be less than or equal to
///   [`PhysicalDeviceLimits::max_color_attachments`]
/// - If the first use of an attachment in this render pass is as an input attachment, and the
///   attachment is not also used as a color or depth/stencil attachment in the same subpass, then
///   `loadOp`**must** not be `VK_ATTACHMENT_LOAD_OP_CLEAR`
/// - If [`p_resolve_attachments`] is not `NULL`, for each resolve attachment that does not have the
///   value [`ATTACHMENT_UNUSED`], the corresponding color attachment **must** not have the value
///   [`ATTACHMENT_UNUSED`]
/// - If [`p_resolve_attachments`] is not `NULL`, for each resolve attachment that is not
///   [`ATTACHMENT_UNUSED`], the corresponding color attachment **must** not have a sample count of
///   `VK_SAMPLE_COUNT_1_BIT`
/// - If [`p_resolve_attachments`] is not `NULL`, each resolve attachment that is not
///   [`ATTACHMENT_UNUSED`]**must** have a sample count of `VK_SAMPLE_COUNT_1_BIT`
/// - Any given element of [`p_resolve_attachments`]**must** have the same [`Format`] as its
///   corresponding color attachment
/// - All attachments in [`p_color_attachments`] that are not [`ATTACHMENT_UNUSED`]**must** have the
///   same sample count
/// -    All attachments in [`p_input_attachments`] that are not [`ATTACHMENT_UNUSED`]**must** have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain at least `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// -    All attachments in [`p_color_attachments`] that are not [`ATTACHMENT_UNUSED`]**must** have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
/// -    All attachments in [`p_resolve_attachments`] that are not [`ATTACHMENT_UNUSED`]**must** have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
/// -    If [`p_depth_stencil_attachment`] is not `NULL` and the attachment is not [`ATTACHMENT_UNUSED`] then it **must** have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// -    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`p_input_attachments`] that are not [`ATTACHMENT_UNUSED`]**must** have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features)**must** contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
/// -    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`p_color_attachments`] that are not [`ATTACHMENT_UNUSED`]**must** have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features)**must** contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
/// -    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`p_resolve_attachments`] that are not [`ATTACHMENT_UNUSED`]**must** have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features)**must** contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
/// - If the `[`VK_AMD_mixed_attachment_samples`]` extension is enabled, all attachments in
///   [`p_color_attachments`] that are not [`ATTACHMENT_UNUSED`]**must** have a sample count that is
///   smaller than or equal to the sample count of [`p_depth_stencil_attachment`] if it is not
///   [`ATTACHMENT_UNUSED`]
/// - If neither the `[`VK_AMD_mixed_attachment_samples`]` nor the
///   `[`VK_NV_framebuffer_mixed_samples`]` extensions are enabled, and if
///   [`p_depth_stencil_attachment`] is not [`ATTACHMENT_UNUSED`] and any attachments in
///   [`p_color_attachments`] are not [`ATTACHMENT_UNUSED`], they **must** have the same sample
///   count
/// - Each element of [`p_preserve_attachments`]**must** not be [`ATTACHMENT_UNUSED`]
/// - Any given element of [`p_preserve_attachments`]**must** not also be an element of any other
///   member of the subpass description
/// - If any attachment is used by more than one [`AttachmentReference2`] member, then each use
///   **must** use the same `layout`
/// - Attachments **must** follow the [image layout requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#attachment-type-imagelayout)
///   based on the type of attachment it is being used as
/// - If [`flags`] includes `VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX`, it **must**
///   also include `VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX`
/// - If the `attachment` member of any element of [`p_input_attachments`] is not
///   [`ATTACHMENT_UNUSED`], then the `aspectMask` member **must** be a valid combination of
///   [`ImageAspectFlagBits`]
/// - If the `attachment` member of any element of [`p_input_attachments`] is not
///   [`ATTACHMENT_UNUSED`], then the `aspectMask` member **must** not be `0`
/// - If the `attachment` member of any element of [`p_input_attachments`] is not
///   [`ATTACHMENT_UNUSED`], then the `aspectMask` member **must** not include
///   `VK_IMAGE_ASPECT_METADATA_BIT`
/// - If the `attachment` member of any element of [`p_input_attachments`] is not
///   [`ATTACHMENT_UNUSED`], then the `aspectMask` member **must** not include
///   `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` for any index *i*
/// - An attachment **must** not be used in both [`p_depth_stencil_attachment`] and
///   [`p_color_attachments`]
/// - If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview)
///   feature is not enabled, [`view_mask`]**must** be `0`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`FragmentShadingRateAttachmentInfoKHR`]
///   or [`SubpassDescriptionDepthStencilResolve`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`SubpassDescriptionFlagBits`] values
/// - [`pipeline_bind_point`]**must** be a valid [`PipelineBindPoint`] value
/// - If [`input_attachment_count`] is not `0`, [`p_input_attachments`]**must** be a valid pointer
///   to an array of [`input_attachment_count`] valid [`AttachmentReference2`] structures
/// - If [`color_attachment_count`] is not `0`, [`p_color_attachments`]**must** be a valid pointer
///   to an array of [`color_attachment_count`] valid [`AttachmentReference2`] structures
/// - If [`color_attachment_count`] is not `0`, and [`p_resolve_attachments`] is not `NULL`,
///   [`p_resolve_attachments`]**must** be a valid pointer to an array of [`color_attachment_count`]
///   valid [`AttachmentReference2`] structures
/// - If [`p_depth_stencil_attachment`] is not `NULL`, [`p_depth_stencil_attachment`]**must** be a
///   valid pointer to a valid [`AttachmentReference2`] structure
/// - If [`preserve_attachment_count`] is not `0`, [`p_preserve_attachments`]**must** be a valid
///   pointer to an array of [`preserve_attachment_count`]`uint32_t` values
///# Related
/// - [`VK_KHR_create_renderpass2`]
/// - [`crate::vulkan1_2`]
/// - [`AttachmentReference2`]
/// - [`PipelineBindPoint`]
/// - [`RenderPassCreateInfo2`]
/// - [`StructureType`]
/// - [`SubpassDescriptionFlags`]
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
pub struct SubpassDescription2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`SubpassDescriptionFlagBits`]
    ///specifying usage of the subpass.
    flags: SubpassDescriptionFlags,
    ///[`pipeline_bind_point`] is a [`PipelineBindPoint`] value specifying
    ///the pipeline type supported for this subpass.
    pipeline_bind_point: PipelineBindPoint,
    ///[`view_mask`] is a bitfield of view indices describing which views
    ///rendering is broadcast to in this subpass, when multiview is enabled.
    view_mask: u32,
    ///[`input_attachment_count`] is the number of input attachments.
    input_attachment_count: u32,
    ///[`p_input_attachments`] is a pointer to an array of
    ///[`AttachmentReference2`] structures defining the input attachments
    ///for this subpass and their layouts.
    p_input_attachments: *mut AttachmentReference2<'lt>,
    ///[`color_attachment_count`] is the number of color attachments.
    color_attachment_count: u32,
    ///[`p_color_attachments`] is a pointer to an array of
    ///[`color_attachment_count`][`AttachmentReference2`] structures
    ///defining the color attachments for this subpass and their layouts.
    p_color_attachments: *mut AttachmentReference2<'lt>,
    ///[`p_resolve_attachments`] is `NULL` or a pointer to an array of
    ///[`color_attachment_count`][`AttachmentReference2`] structures
    ///defining the resolve attachments for this subpass and their layouts.
    p_resolve_attachments: *mut AttachmentReference2<'lt>,
    ///[`p_depth_stencil_attachment`] is a pointer to a
    ///[`AttachmentReference2`] structure specifying the depth/stencil
    ///attachment for this subpass and its layout.
    p_depth_stencil_attachment: *mut AttachmentReference2<'lt>,
    ///[`preserve_attachment_count`] is the number of preserved attachments.
    preserve_attachment_count: u32,
    ///[`p_preserve_attachments`] is a pointer to an array of
    ///[`preserve_attachment_count`] render pass attachment indices identifying
    ///attachments that are not used by this subpass, but whose contents **must**
    ///be preserved throughout the subpass.
    p_preserve_attachments: *mut u32,
}
///[VkSubpassDependency2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDependency2.html) - Structure specifying a subpass dependency
///# C Specifications
///The [`SubpassDependency2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkSubpassDependency2 {
///    VkStructureType         sType;
///    const void*             pNext;
///    uint32_t                srcSubpass;
///    uint32_t                dstSubpass;
///    VkPipelineStageFlags    srcStageMask;
///    VkPipelineStageFlags    dstStageMask;
///    VkAccessFlags           srcAccessMask;
///    VkAccessFlags           dstAccessMask;
///    VkDependencyFlags       dependencyFlags;
///    int32_t                 viewOffset;
///} VkSubpassDependency2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_create_renderpass2
///typedef VkSubpassDependency2 VkSubpassDependency2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_subpass`] is the subpass index of the first subpass in the dependency, or
///   [`SUBPASS_EXTERNAL`].
/// - [`dst_subpass`] is the subpass index of the second subpass in the dependency, or
///   [`SUBPASS_EXTERNAL`].
/// - [`src_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [source stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
/// - [`dst_stage_mask`] is a bitmask of [`PipelineStageFlagBits`] specifying the [destination stage
///   mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks)
/// - [`src_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
/// - [`dst_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a [destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
/// - [`dependency_flags`] is a bitmask of [`DependencyFlagBits`].
/// - [`view_offset`] controls which views in the source subpass the views in the destination
///   subpass depend on.
///# Description
///Parameters defined by this structure with the same name as those in
///[`SubpassDependency`] have the identical effect to those parameters.[`view_offset`] has the same
/// effect for the described subpass dependency as
///[`RenderPassMultiviewCreateInfo::p_view_offsets`] has on each
///corresponding subpass dependency.If a [`MemoryBarrier2`] is included in the [`p_next`] chain,
///[`src_stage_mask`], [`dst_stage_mask`], [`src_access_mask`], and
///[`dst_access_mask`] parameters are ignored.
///The synchronization and access scopes instead are defined by the parameters
///of [`MemoryBarrier2`].Valid Usage
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`src_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-synchronization2)
///   feature is not enabled, [`src_stage_mask`]**must** not be `0`
///
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`dst_stage_mask`]**must** not contain
///   `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-synchronization2)
///   feature is not enabled, [`dst_stage_mask`]**must** not be `0`
/// - [`src_subpass`]**must** be less than or equal to [`dst_subpass`], unless one of them is
///   [`SUBPASS_EXTERNAL`], to avoid cyclic dependencies and ensure a valid execution order
/// - [`src_subpass`] and [`dst_subpass`]**must** not both be equal to [`SUBPASS_EXTERNAL`]
/// - If [`src_subpass`] is equal to [`dst_subpass`] and not all of the stages in [`src_stage_mask`]
///   and [`dst_stage_mask`] are [framebuffer-space stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions),
///   the [logically latest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order)
///   pipeline stage in [`src_stage_mask`]**must** be [logically earlier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order)
///   than or equal to the [logically earliest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order)
///   pipeline stage in [`dst_stage_mask`]
/// -    Any access flag included in [`src_access_mask`]**must** be supported by one of the pipeline stages in [`src_stage_mask`], as specified in the [table of supported access types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types-supported)
/// -    Any access flag included in [`dst_access_mask`]**must** be supported by one of the pipeline stages in [`dst_stage_mask`], as specified in the [table of supported access types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types-supported)
/// - If [`dependency_flags`] includes `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`src_subpass`]**must** not
///   be equal to [`SUBPASS_EXTERNAL`]
/// - If [`dependency_flags`] includes `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`dst_subpass`]**must** not
///   be equal to [`SUBPASS_EXTERNAL`]
/// -    If [`src_subpass`] equals [`dst_subpass`], and [`src_stage_mask`] and [`dst_stage_mask`] both include a [framebuffer-space stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions), then [`dependency_flags`]**must** include `VK_DEPENDENCY_BY_REGION_BIT`
/// - If [`view_offset`] is not equal to `0`, [`src_subpass`]**must** not be equal to
///   [`dst_subpass`]
/// - If [`dependency_flags`] does not include `VK_DEPENDENCY_VIEW_LOCAL_BIT`,
///   [`view_offset`]**must** be `0`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of [`MemoryBarrier2`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`src_stage_mask`]**must** be a valid combination of [`PipelineStageFlagBits`] values
/// - [`dst_stage_mask`]**must** be a valid combination of [`PipelineStageFlagBits`] values
/// - [`src_access_mask`]**must** be a valid combination of [`AccessFlagBits`] values
/// - [`dst_access_mask`]**must** be a valid combination of [`AccessFlagBits`] values
/// - [`dependency_flags`]**must** be a valid combination of [`DependencyFlagBits`] values
///# Related
/// - [`VK_KHR_create_renderpass2`]
/// - [`crate::vulkan1_2`]
/// - [`AccessFlags`]
/// - [`DependencyFlags`]
/// - [`PipelineStageFlags`]
/// - [`RenderPassCreateInfo2`]
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
pub struct SubpassDependency2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`src_subpass`] is the subpass index of the first subpass in the
    ///dependency, or [`SUBPASS_EXTERNAL`].
    src_subpass: u32,
    ///[`dst_subpass`] is the subpass index of the second subpass in the
    ///dependency, or [`SUBPASS_EXTERNAL`].
    dst_subpass: u32,
    ///[`src_stage_mask`] is a bitmask of [`PipelineStageFlagBits`]
    ///specifying the [source stage
    ///mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
    src_stage_mask: PipelineStageFlags,
    ///[`dst_stage_mask`] is a bitmask of [`PipelineStageFlagBits`]
    ///specifying the [destination
    ///stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks)
    dst_stage_mask: PipelineStageFlags,
    ///[`src_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a
    ///[source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
    src_access_mask: AccessFlags,
    ///[`dst_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a
    ///[destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
    dst_access_mask: AccessFlags,
    ///[`dependency_flags`] is a bitmask of [`DependencyFlagBits`].
    dependency_flags: DependencyFlags,
    ///[`view_offset`] controls which views in the source subpass the views in
    ///the destination subpass depend on.
    view_offset: i32,
}
///[VkRenderPassCreateInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateInfo2.html) - Structure specifying parameters of a newly created render pass
///# C Specifications
///The [`RenderPassCreateInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkRenderPassCreateInfo2 {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkRenderPassCreateFlags            flags;
///    uint32_t                           attachmentCount;
///    const VkAttachmentDescription2*    pAttachments;
///    uint32_t                           subpassCount;
///    const VkSubpassDescription2*       pSubpasses;
///    uint32_t                           dependencyCount;
///    const VkSubpassDependency2*        pDependencies;
///    uint32_t                           correlatedViewMaskCount;
///    const uint32_t*                    pCorrelatedViewMasks;
///} VkRenderPassCreateInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_create_renderpass2
///typedef VkRenderPassCreateInfo2 VkRenderPassCreateInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`attachment_count`] is the number of attachments used by this render pass.
/// - [`p_attachments`] is a pointer to an array of [`attachment_count`][`AttachmentDescription2`]
///   structures describing the attachments used by the render pass.
/// - [`subpass_count`] is the number of subpasses to create.
/// - [`p_subpasses`] is a pointer to an array of [`subpass_count`][`SubpassDescription2`]
///   structures describing each subpass.
/// - [`dependency_count`] is the number of dependencies between pairs of subpasses.
/// - [`p_dependencies`] is a pointer to an array of [`dependency_count`][`SubpassDependency2`]
///   structures describing dependencies between pairs of subpasses.
/// - [`correlated_view_mask_count`] is the number of correlation masks.
/// - [`p_correlated_view_masks`] is a pointer to an array of view masks indicating sets of views
///   that **may** be more efficient to render concurrently.
///# Description
///Parameters defined by this structure with the same name as those in
///[`RenderPassCreateInfo`] have the identical effect to those parameters;
///the child structures are variants of those used in
///[`RenderPassCreateInfo`] which add [`s_type`] and [`p_next`]
///parameters, allowing them to be extended.If the [`SubpassDescription2::view_mask`] member of any
/// element of
///[`p_subpasses`] is not zero, *multiview* functionality is considered to be
///enabled for this render pass.[`correlated_view_mask_count`] and [`p_correlated_view_masks`] have
/// the same
///effect as [`RenderPassMultiviewCreateInfo::correlation_mask_count`]
///and [`RenderPassMultiviewCreateInfo::p_correlation_masks`],
///respectively.Valid Usage
/// - If any two subpasses operate on attachments with overlapping ranges of the same
///   [`DeviceMemory`] object, and at least one subpass writes to that area of [`DeviceMemory`], a
///   subpass dependency **must** be included (either directly or via some intermediate subpasses)
///   between them
/// - If the `attachment` member of any element of `pInputAttachments`, `pColorAttachments`,
///   `pResolveAttachments` or `pDepthStencilAttachment`, or the attachment indexed by any element
///   of `pPreserveAttachments` in any given element of [`p_subpasses`] is bound to a range of a
///   [`DeviceMemory`] object that overlaps with any other attachment in any subpass (including the
///   same subpass), the [`AttachmentDescription2`] structures describing them **must** include
///   `VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT` in [`flags`]
/// - If the `attachment` member of any element of `pInputAttachments`, `pColorAttachments`,
///   `pResolveAttachments` or `pDepthStencilAttachment`, or any element of `pPreserveAttachments`
///   in any given element of [`p_subpasses`] is not [`ATTACHMENT_UNUSED`], then it **must** be less
///   than [`attachment_count`]
/// - If the pNext chain includes a [`RenderPassFragmentDensityMapCreateInfoEXT`] structure and the
///   `fragmentDensityMapAttachment` member is not [`ATTACHMENT_UNUSED`], then `attachment`**must**
///   be less than [`attachment_count`]
/// - If the [`p_subpasses`] pNext chain includes a [`SubpassDescriptionDepthStencilResolve`]
///   structure and the `pDepthStencilResolveAttachment` member is not `NULL` and does not have the
///   value [`ATTACHMENT_UNUSED`], then `attachment`**must** be less than [`attachment_count`]
/// - For any member of [`p_attachments`] with a `loadOp` equal to `VK_ATTACHMENT_LOAD_OP_CLEAR`,
///   the first use of that attachment **must** not specify a `layout` equal to
///   `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - For any member of [`p_attachments`] with a `stencilLoadOp` equal to
///   `VK_ATTACHMENT_LOAD_OP_CLEAR`, the first use of that attachment **must** not specify a
///   `layout` equal to `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`
/// -    For any element of [`p_dependencies`], if the `srcSubpass` is not [`SUBPASS_EXTERNAL`], all stage flags included in the `srcStageMask` member of that dependency **must** be a pipeline stage supported by the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-types) identified by the `pipelineBindPoint` member of the source subpass
/// -    For any element of [`p_dependencies`], if the `dstSubpass` is not [`SUBPASS_EXTERNAL`], all stage flags included in the `dstStageMask` member of that dependency **must** be a pipeline stage supported by the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-types) identified by the `pipelineBindPoint` member of the destination subpass
/// - The set of bits included in any element of [`p_correlated_view_masks`]**must** not overlap
///   with the set of bits included in any other element of [`p_correlated_view_masks`]
/// - If the [`SubpassDescription2::view_mask`] member of all elements of [`p_subpasses`] is `0`,
///   [`correlated_view_mask_count`]**must** be `0`
/// - The [`SubpassDescription2::view_mask`] member of all elements of [`p_subpasses`]**must**
///   either all be `0`, or all not be `0`
/// - If the [`SubpassDescription2::view_mask`] member of all elements of [`p_subpasses`] is `0`,
///   the `dependencyFlags` member of any element of [`p_dependencies`]**must** not include
///   `VK_DEPENDENCY_VIEW_LOCAL_BIT`
/// - For any element of [`p_dependencies`] where its `srcSubpass` member equals its `dstSubpass`
///   member, if the `viewMask` member of the corresponding element of [`p_subpasses`] includes more
///   than one bit, its `dependencyFlags` member **must** include `VK_DEPENDENCY_VIEW_LOCAL_BIT`
/// - If the `attachment` member of any element of the `pInputAttachments` member of any element of
///   [`p_subpasses`] is not [`ATTACHMENT_UNUSED`], the `aspectMask` member of that element of
///   `pInputAttachments`**must** only include aspects that are present in images of the format
///   specified by the element of [`p_attachments`] specified by `attachment`
/// - The `srcSubpass` member of each element of [`p_dependencies`]**must** be less than
///   [`subpass_count`]
/// - The `dstSubpass` member of each element of [`p_dependencies`]**must** be less than
///   [`subpass_count`]
/// - If any element of [`p_attachments`] is used as a fragment shading rate attachment in any
///   subpass, it **must** not be used as any other attachment in the render pass
/// - If [`flags`] includes `VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM`, an element of
///   [`p_subpasses`] includes an instance of [`FragmentShadingRateAttachmentInfoKHR`] in its
///   [`p_next`] chain, and the `pFragmentShadingRateAttachment` member of that structure is not
///   equal to `NULL`, the `attachment` member of `pFragmentShadingRateAttachment`**must** be
///   [`ATTACHMENT_UNUSED`]
/// -    If any element of [`p_attachments`] is used as a fragment shading rate attachment in any subpass, it **must** have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
/// - If the pipeline is being created with fragment shader state, and the
///   [`VK_QCOM_render_pass_shader_resolve`] extension is enabled, and if subpass has any input
///   attachments, and if the subpass description contains
///   `VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM`, then the sample count of the input
///   attachments **must** equal `rasterizationSamples`
/// - If the pipeline is being created with fragment shader state, and the
///   [`VK_QCOM_render_pass_shader_resolve`] extension is enabled, and if the subpass description
///   contains `VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM`, then `sampleShadingEnable`**must**
///   be false
/// - If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, and if
///   `pResolveAttachments` is not `NULL`, then each resolve attachment **must** be
///   [`ATTACHMENT_UNUSED`]
/// - If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, and if
///   `pDepthStencilResolveAttachment` is not `NULL`, then the depth/stencil resolve attachment
///   **must** be [`ATTACHMENT_UNUSED`]
/// - If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, then the subpass
///   **must** be the last subpass in a subpass dependency chain
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`RenderPassFragmentDensityMapCreateInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`RenderPassCreateFlagBits`] values
/// - If [`attachment_count`] is not `0`, [`p_attachments`]**must** be a valid pointer to an array
///   of [`attachment_count`] valid [`AttachmentDescription2`] structures
/// - [`p_subpasses`]**must** be a valid pointer to an array of [`subpass_count`] valid
///   [`SubpassDescription2`] structures
/// - If [`dependency_count`] is not `0`, [`p_dependencies`]**must** be a valid pointer to an array
///   of [`dependency_count`] valid [`SubpassDependency2`] structures
/// - If [`correlated_view_mask_count`] is not `0`, [`p_correlated_view_masks`]**must** be a valid
///   pointer to an array of [`correlated_view_mask_count`]`uint32_t` values
/// - [`subpass_count`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_create_renderpass2`]
/// - [`crate::vulkan1_2`]
/// - [`AttachmentDescription2`]
/// - [`RenderPassCreateFlags`]
/// - [`StructureType`]
/// - [`SubpassDependency2`]
/// - [`SubpassDescription2`]
/// - [`CreateRenderPass2`]
/// - [`CreateRenderPass2KHR`]
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
pub struct RenderPassCreateInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: RenderPassCreateFlags,
    ///[`attachment_count`] is the number of attachments used by this render
    ///pass.
    attachment_count: u32,
    ///[`p_attachments`] is a pointer to an array of [`attachment_count`][`AttachmentDescription2`]
    /// structures describing the attachments used by the render pass.
    p_attachments: *mut AttachmentDescription2<'lt>,
    ///[`subpass_count`] is the number of subpasses to create.
    subpass_count: u32,
    ///[`p_subpasses`] is a pointer to an array of [`subpass_count`][`SubpassDescription2`]
    /// structures describing each subpass.
    p_subpasses: *mut SubpassDescription2<'lt>,
    ///[`dependency_count`] is the number of dependencies between pairs of
    ///subpasses.
    dependency_count: u32,
    ///[`p_dependencies`] is a pointer to an array of [`dependency_count`][`SubpassDependency2`]
    /// structures describing dependencies between pairs of subpasses.
    p_dependencies: *mut SubpassDependency2<'lt>,
    ///[`correlated_view_mask_count`] is the number of correlation masks.
    correlated_view_mask_count: u32,
    ///[`p_correlated_view_masks`] is a pointer to an array of view masks
    ///indicating sets of views that **may** be more efficient to render
    ///concurrently.
    p_correlated_view_masks: *mut u32,
}
///[VkSubpassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassBeginInfo.html) - Structure specifying subpass begin information
///# C Specifications
///The [`SubpassBeginInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkSubpassBeginInfo {
///    VkStructureType      sType;
///    const void*          pNext;
///    VkSubpassContents    contents;
///} VkSubpassBeginInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_create_renderpass2
///typedef VkSubpassBeginInfo VkSubpassBeginInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`contents`] is a [`SubpassContents`] value specifying how the commands in the next subpass
///   will be provided.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`contents`]**must** be a valid [`SubpassContents`] value
///# Related
/// - [`VK_KHR_create_renderpass2`]
/// - [`crate::vulkan1_2`]
/// - [`StructureType`]
/// - [`SubpassContents`]
/// - [`CmdBeginRenderPass2`]
/// - [`CmdBeginRenderPass2KHR`]
/// - [`CmdNextSubpass2`]
/// - [`CmdNextSubpass2KHR`]
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
pub struct SubpassBeginInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`contents`] is a [`SubpassContents`] value specifying how the
    ///commands in the next subpass will be provided.
    contents: SubpassContents,
}
///[VkSubpassEndInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassEndInfo.html) - Structure specifying subpass end information
///# C Specifications
///The [`SubpassEndInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkSubpassEndInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///} VkSubpassEndInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_create_renderpass2
///typedef VkSubpassEndInfo VkSubpassEndInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SUBPASS_END_INFO`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`SubpassFragmentDensityMapOffsetEndInfoQCOM`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
///# Related
/// - [`VK_KHR_create_renderpass2`]
/// - [`crate::vulkan1_2`]
/// - [`StructureType`]
/// - [`CmdEndRenderPass2`]
/// - [`CmdEndRenderPass2KHR`]
/// - [`CmdNextSubpass2`]
/// - [`CmdNextSubpass2KHR`]
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
pub struct SubpassEndInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
}
///[VkPhysicalDeviceTimelineSemaphoreFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreFeatures.html) - Structure describing timeline semaphore features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceTimelineSemaphoreFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceTimelineSemaphoreFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           timelineSemaphore;
///} VkPhysicalDeviceTimelineSemaphoreFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_timeline_semaphore
///typedef VkPhysicalDeviceTimelineSemaphoreFeatures VkPhysicalDeviceTimelineSemaphoreFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`timeline_semaphore`] indicates whether semaphores created with a [`SemaphoreType`] of
///   `VK_SEMAPHORE_TYPE_TIMELINE` are supported.
///If the [`PhysicalDeviceTimelineSemaphoreFeatures`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceTimelineSemaphoreFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES`
///# Related
/// - [`VK_KHR_timeline_semaphore`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceTimelineSemaphoreFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`timeline_semaphore`]
    ///indicates whether semaphores created with a [`SemaphoreType`] of
    ///`VK_SEMAPHORE_TYPE_TIMELINE` are supported.
    timeline_semaphore: Bool32,
}
///[VkPhysicalDeviceTimelineSemaphoreProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTimelineSemaphoreProperties.html) - Structure describing timeline semaphore properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceTimelineSemaphoreProperties`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceTimelineSemaphoreProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    uint64_t           maxTimelineSemaphoreValueDifference;
///} VkPhysicalDeviceTimelineSemaphoreProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_timeline_semaphore
///typedef VkPhysicalDeviceTimelineSemaphoreProperties
/// VkPhysicalDeviceTimelineSemaphorePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`max_timeline_semaphore_value_difference`] indicates the maximum difference allowed by the
///   implementation between the current value of a timeline semaphore and any pending signal or
///   wait operations.
///If the [`PhysicalDeviceTimelineSemaphoreProperties`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES`
///# Related
/// - [`VK_KHR_timeline_semaphore`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceTimelineSemaphoreProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    max_timeline_semaphore_value_difference: u64,
}
///[VkSemaphoreTypeCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreTypeCreateInfo.html) - Structure specifying the type of a newly created semaphore
///# C Specifications
///The [`SemaphoreTypeCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkSemaphoreTypeCreateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkSemaphoreType    semaphoreType;
///    uint64_t           initialValue;
///} VkSemaphoreTypeCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_timeline_semaphore
///typedef VkSemaphoreTypeCreateInfo VkSemaphoreTypeCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore_type`] is a [`SemaphoreType`] value specifying the type of the semaphore.
/// - [`initial_value`] is the initial payload value if [`semaphore_type`] is
///   `VK_SEMAPHORE_TYPE_TIMELINE`.
///# Description
///To create a semaphore of a specific type, add a
///[`SemaphoreTypeCreateInfo`] structure to the
///[`SemaphoreCreateInfo`]::[`p_next`] chain.If no [`SemaphoreTypeCreateInfo`] structure is included in the
///[`p_next`] chain of [`SemaphoreCreateInfo`], then the created semaphore
///will have a default [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`.Valid Usage
/// - If the [`timelineSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-timelineSemaphore)
///   feature is not enabled, [`semaphore_type`]**must** not equal `VK_SEMAPHORE_TYPE_TIMELINE`
/// - If [`semaphore_type`] is `VK_SEMAPHORE_TYPE_BINARY`, [`initial_value`]**must** be zero
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`
/// - [`semaphore_type`]**must** be a valid [`SemaphoreType`] value
///# Related
/// - [`VK_KHR_timeline_semaphore`]
/// - [`crate::vulkan1_2`]
/// - [`SemaphoreType`]
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
pub struct SemaphoreTypeCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`semaphore_type`] is a [`SemaphoreType`] value specifying the type
    ///of the semaphore.
    semaphore_type: SemaphoreType,
    ///[`initial_value`] is the initial payload value if [`semaphore_type`]
    ///is `VK_SEMAPHORE_TYPE_TIMELINE`.
    initial_value: u64,
}
///[VkTimelineSemaphoreSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimelineSemaphoreSubmitInfo.html) - Structure specifying signal and wait values for timeline semaphores
///# C Specifications
///To specify the values to use when waiting for and signaling semaphores
///created with a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`,
///add a [`TimelineSemaphoreSubmitInfo`] structure to the [`p_next`] chain
///of the [`SubmitInfo`] structure when using [`QueueSubmit`] or the
///[`BindSparseInfo`] structure when using [`QueueBindSparse`].
///The [`TimelineSemaphoreSubmitInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkTimelineSemaphoreSubmitInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           waitSemaphoreValueCount;
///    const uint64_t*    pWaitSemaphoreValues;
///    uint32_t           signalSemaphoreValueCount;
///    const uint64_t*    pSignalSemaphoreValues;
///} VkTimelineSemaphoreSubmitInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_timeline_semaphore
///typedef VkTimelineSemaphoreSubmitInfo VkTimelineSemaphoreSubmitInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`wait_semaphore_value_count`] is the number of semaphore wait values specified in
///   [`p_wait_semaphore_values`].
/// - [`p_wait_semaphore_values`] is a pointer to an array of [`wait_semaphore_value_count`] values
///   for the corresponding semaphores in [`SubmitInfo::p_wait_semaphores`] to wait for.
/// - [`signal_semaphore_value_count`] is the number of semaphore signal values specified in
///   [`p_signal_semaphore_values`].
/// - [`p_signal_semaphore_values`] is a pointer to an array [`signal_semaphore_value_count`] values
///   for the corresponding semaphores in [`SubmitInfo::p_signal_semaphores`] to set when signaled.
///# Description
///If the semaphore in [`SubmitInfo::p_wait_semaphores`] or
///[`SubmitInfo::p_signal_semaphores`] corresponding to an entry in
///[`p_wait_semaphore_values`] or [`p_signal_semaphore_values`] respectively was
///not created with a [`SemaphoreType`] of
///`VK_SEMAPHORE_TYPE_TIMELINE`, the implementation **must** ignore the value
///in the [`p_wait_semaphore_values`] or [`p_signal_semaphore_values`] entry.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO`
/// - If [`wait_semaphore_value_count`] is not `0`, and [`p_wait_semaphore_values`] is not `NULL`,
///   [`p_wait_semaphore_values`]**must** be a valid pointer to an array of
///   [`wait_semaphore_value_count`]`uint64_t` values
/// - If [`signal_semaphore_value_count`] is not `0`, and [`p_signal_semaphore_values`] is not
///   `NULL`, [`p_signal_semaphore_values`]**must** be a valid pointer to an array of
///   [`signal_semaphore_value_count`]`uint64_t` values
///# Related
/// - [`VK_KHR_timeline_semaphore`]
/// - [`crate::vulkan1_2`]
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
pub struct TimelineSemaphoreSubmitInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`wait_semaphore_value_count`] is the number of semaphore wait values
    ///specified in [`p_wait_semaphore_values`].
    wait_semaphore_value_count: u32,
    ///[`p_wait_semaphore_values`] is a pointer to an array of
    ///[`wait_semaphore_value_count`] values for the corresponding semaphores in
    ///[`SubmitInfo`]::`pWaitSemaphores` to wait for.
    p_wait_semaphore_values: *mut u64,
    ///[`signal_semaphore_value_count`] is the number of semaphore signal values
    ///specified in [`p_signal_semaphore_values`].
    signal_semaphore_value_count: u32,
    ///[`p_signal_semaphore_values`] is a pointer to an array
    ///[`signal_semaphore_value_count`] values for the corresponding semaphores
    ///in [`SubmitInfo`]::`pSignalSemaphores` to set when signaled.
    p_signal_semaphore_values: *mut u64,
}
///[VkSemaphoreWaitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitInfo.html) - Structure containing information about the semaphore wait condition
///# C Specifications
///The [`SemaphoreWaitInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkSemaphoreWaitInfo {
///    VkStructureType         sType;
///    const void*             pNext;
///    VkSemaphoreWaitFlags    flags;
///    uint32_t                semaphoreCount;
///    const VkSemaphore*      pSemaphores;
///    const uint64_t*         pValues;
///} VkSemaphoreWaitInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_timeline_semaphore
///typedef VkSemaphoreWaitInfo VkSemaphoreWaitInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`SemaphoreWaitFlagBits`] specifying additional parameters for the
///   semaphore wait operation.
/// - [`semaphore_count`] is the number of semaphores to wait on.
/// - [`p_semaphores`] is a pointer to an array of [`semaphore_count`] semaphore handles to wait on.
/// - [`p_values`] is a pointer to an array of [`semaphore_count`] timeline semaphore values.
///# Description
///Valid Usage
/// - All of the elements of [`p_semaphores`]**must** reference a semaphore that was created with a
///   [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be a valid combination of [`SemaphoreWaitFlagBits`] values
/// - [`p_semaphores`]**must** be a valid pointer to an array of [`semaphore_count`] valid
///   [`Semaphore`] handles
/// - [`p_values`]**must** be a valid pointer to an array of [`semaphore_count`]`uint64_t` values
/// - [`semaphore_count`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_timeline_semaphore`]
/// - [`crate::vulkan1_2`]
/// - [`Semaphore`]
/// - [`SemaphoreWaitFlags`]
/// - [`StructureType`]
/// - [`WaitSemaphores`]
/// - [`WaitSemaphoresKHR`]
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
pub struct SemaphoreWaitInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`SemaphoreWaitFlagBits`] specifying
    ///additional parameters for the semaphore wait operation.
    flags: SemaphoreWaitFlags,
    ///[`semaphore_count`] is the number of semaphores to wait on.
    semaphore_count: u32,
    ///[`p_semaphores`] is a pointer to an array of [`semaphore_count`]
    ///semaphore handles to wait on.
    p_semaphores: *mut Semaphore,
    ///[`p_values`] is a pointer to an array of [`semaphore_count`] timeline
    ///semaphore values.
    p_values: *mut u64,
}
///[VkSemaphoreSignalInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreSignalInfo.html) - Structure containing information about a semaphore signal operation
///# C Specifications
///The [`SemaphoreSignalInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkSemaphoreSignalInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkSemaphore        semaphore;
///    uint64_t           value;
///} VkSemaphoreSignalInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_timeline_semaphore
///typedef VkSemaphoreSignalInfo VkSemaphoreSignalInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`semaphore`] is the handle of the semaphore to signal.
/// - [`value`] is the value to signal.
///# Description
///Valid Usage
/// - [`semaphore`]**must** have been created with a [`SemaphoreType`] of
///   `VK_SEMAPHORE_TYPE_TIMELINE`
/// - [`value`]**must** have a value greater than the current value of the semaphore
/// - [`value`]**must** be less than the value of any pending semaphore signal operations
/// -  [`value`]**must** have a value which does not differ from the current value of the semaphore or the value of any outstanding semaphore wait or signal operation on [`semaphore`] by more than [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`semaphore`]**must** be a valid [`Semaphore`] handle
///# Related
/// - [`VK_KHR_timeline_semaphore`]
/// - [`crate::vulkan1_2`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`SignalSemaphore`]
/// - [`SignalSemaphoreKHR`]
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
pub struct SemaphoreSignalInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`semaphore`] is the handle of the semaphore to signal.
    semaphore: Semaphore,
    ///[`value`] is the value to signal.
    value: u64,
}
///[VkPhysicalDevice8BitStorageFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice8BitStorageFeatures.html) - Structure describing features supported by VK_KHR_8bit_storage
///# C Specifications
///The [`PhysicalDevice8BitStorageFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDevice8BitStorageFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           storageBuffer8BitAccess;
///    VkBool32           uniformAndStorageBuffer8BitAccess;
///    VkBool32           storagePushConstant8;
///} VkPhysicalDevice8BitStorageFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_8bit_storage
///typedef VkPhysicalDevice8BitStorageFeatures VkPhysicalDevice8BitStorageFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`storage_buffer_8_bit_access`] indicates whether objects in the     `StorageBuffer`,
///   `ShaderRecordBufferKHR`,     or `PhysicalStorageBuffer`     storage class with the `Block`
///   decoration **can** have 8-bit integer     members.     If this feature is not enabled, 8-bit
///   integer members **must** not be used     in such objects.     This also indicates whether
///   shader modules **can** declare the     `StorageBuffer8BitAccess` capability.
/// - [`uniform_and_storage_buffer_8_bit_access`] indicates whether objects in the `Uniform` storage
///   class with the `Block` decoration **can** have 8-bit integer members. If this feature is not
///   enabled, 8-bit integer members **must** not be used in such objects. This also indicates
///   whether shader modules **can** declare the `UniformAndStorageBuffer8BitAccess` capability.
/// - [`storage_push_constant_8`] indicates whether objects in the `PushConstant` storage class
///   **can** have 8-bit integer members. If this feature is not enabled, 8-bit integer members
///   **must** not be used in such objects. This also indicates whether shader modules **can**
///   declare the `StoragePushConstant8` capability.
///If the [`PhysicalDevice8BitStorageFeatures`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevice8BitStorageFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES`
///# Related
/// - [`VK_KHR_8bit_storage`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDevice8BitStorageFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`storage_buffer_8_bit_access`] indicates whether objects in the
    ///    `StorageBuffer`,
    ///`ShaderRecordBufferKHR`,
    ///    or `PhysicalStorageBuffer`
    ///    storage class with the `Block` decoration **can** have 8-bit integer
    ///    members.
    ///    If this feature is not enabled, 8-bit integer members **must** not be used
    ///    in such objects.
    ///    This also indicates whether shader modules **can** declare the
    ///    `StorageBuffer8BitAccess` capability.
    storage_buffer_8_bit_access: Bool32,
    ///[`uniform_and_storage_buffer_8_bit_access`] indicates whether objects in the
    ///`Uniform` storage class with the `Block` decoration **can** have
    ///8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members **must** not be used
    ///in such objects.
    ///This also indicates whether shader modules **can** declare the
    ///`UniformAndStorageBuffer8BitAccess` capability.
    uniform_and_storage_buffer_8_bit_access: Bool32,
    ///[`storage_push_constant_8`] indicates whether objects in the
    ///`PushConstant` storage class **can** have 8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members **must** not be used
    ///in such objects.
    ///This also indicates whether shader modules **can** declare the
    ///`StoragePushConstant8` capability.
    storage_push_constant_8: Bool32,
}
///[VkPhysicalDeviceVulkanMemoryModelFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkanMemoryModelFeatures.html) - Structure describing features supported by the memory model
///# C Specifications
///The [`PhysicalDeviceVulkanMemoryModelFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceVulkanMemoryModelFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           vulkanMemoryModel;
///    VkBool32           vulkanMemoryModelDeviceScope;
///    VkBool32           vulkanMemoryModelAvailabilityVisibilityChains;
///} VkPhysicalDeviceVulkanMemoryModelFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_vulkan_memory_model
///typedef VkPhysicalDeviceVulkanMemoryModelFeatures VkPhysicalDeviceVulkanMemoryModelFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`vulkan_memory_model`] indicates whether the Vulkan Memory Model is supported, as defined in [Vulkan Memory Model](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model). This also indicates whether shader modules **can** declare the `VulkanMemoryModel` capability.
/// - [`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory Model can use
///   [`Device`] scope synchronization. This also indicates whether shader modules **can** declare
///   the `VulkanMemoryModelDeviceScope` capability.
/// - [`vulkan_memory_model_availability_visibility_chains`] indicates whether the Vulkan Memory Model can use [availability and visibility chains](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model-availability-visibility) with more than one element.
///If the [`PhysicalDeviceVulkanMemoryModelFeaturesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVulkanMemoryModelFeaturesKHR`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES`
///# Related
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceVulkanMemoryModelFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`vulkan_memory_model`]
    ///indicates whether the Vulkan Memory Model is supported, as defined in
    ///[Vulkan Memory Model](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model).
    ///This also indicates whether shader modules **can** declare the
    ///`VulkanMemoryModel` capability.
    vulkan_memory_model: Bool32,
    ///[`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory
    ///Model can use [`Device`] scope synchronization.
    ///This also indicates whether shader modules **can** declare the
    ///`VulkanMemoryModelDeviceScope` capability.
    vulkan_memory_model_device_scope: Bool32,
    ///[`vulkan_memory_model_availability_visibility_chains`] indicates whether
    ///the Vulkan Memory Model can use [availability and visibility chains](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model-availability-visibility) with more than one element.
    vulkan_memory_model_availability_visibility_chains: Bool32,
}
///[VkPhysicalDeviceShaderAtomicInt64Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderAtomicInt64Features.html) - Structure describing features supported by VK_KHR_shader_atomic_int64
///# C Specifications
///The [`PhysicalDeviceShaderAtomicInt64Features`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceShaderAtomicInt64Features {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderBufferInt64Atomics;
///    VkBool32           shaderSharedInt64Atomics;
///} VkPhysicalDeviceShaderAtomicInt64Features;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_shader_atomic_int64
///typedef VkPhysicalDeviceShaderAtomicInt64Features VkPhysicalDeviceShaderAtomicInt64FeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_buffer_int_64_atomics`] indicates whether shaders **can** perform 64-bit unsigned and
///   signed integer atomic operations on buffers.
/// - [`shader_shared_int_64_atomics`] indicates whether shaders **can** perform 64-bit unsigned and
///   signed integer atomic operations on shared memory.
///If the [`PhysicalDeviceShaderAtomicInt64Features`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderAtomicInt64Features`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES`
///# Related
/// - [`VK_KHR_shader_atomic_int64`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceShaderAtomicInt64Features<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_buffer_int_64_atomics`] indicates whether shaders **can** perform
    ///64-bit unsigned and signed integer atomic operations on buffers.
    shader_buffer_int_64_atomics: Bool32,
    ///[`shader_shared_int_64_atomics`] indicates whether shaders **can** perform
    ///64-bit unsigned and signed integer atomic operations on shared memory.
    shader_shared_int_64_atomics: Bool32,
}
///[VkPhysicalDeviceDepthStencilResolveProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthStencilResolveProperties.html) - Structure describing depth/stencil resolve properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDepthStencilResolveProperties`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceDepthStencilResolveProperties {
///    VkStructureType       sType;
///    void*                 pNext;
///    VkResolveModeFlags    supportedDepthResolveModes;
///    VkResolveModeFlags    supportedStencilResolveModes;
///    VkBool32              independentResolveNone;
///    VkBool32              independentResolve;
///} VkPhysicalDeviceDepthStencilResolveProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_depth_stencil_resolve
///typedef VkPhysicalDeviceDepthStencilResolveProperties
/// VkPhysicalDeviceDepthStencilResolvePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`supported_depth_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set
///   of supported depth resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT`**must** be included in the
///   set but implementations **may** support additional modes.
/// - [`supported_stencil_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set
///   of supported stencil resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT`**must** be included in
///   the set but implementations **may** support additional modes.
///   `VK_RESOLVE_MODE_AVERAGE_BIT`**must** not be included in the set.
/// - [`independent_resolve_none`] is [`TRUE`] if the implementation supports setting the depth and
///   stencil resolve modes to different values when one of those modes is `VK_RESOLVE_MODE_NONE`.
///   Otherwise the implementation only supports setting both modes to the same value.
/// - [`independent_resolve`] is [`TRUE`] if the implementation supports all combinations of the
///   supported depth and stencil resolve modes, including setting either depth or stencil resolve
///   mode to `VK_RESOLVE_MODE_NONE`. An implementation that supports
///   [`independent_resolve`]**must** also support [`independent_resolve_none`].
///If the [`PhysicalDeviceDepthStencilResolveProperties`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES`
///# Related
/// - [`VK_KHR_depth_stencil_resolve`]
/// - [`crate::vulkan1_2`]
/// - [`Bool32`]
/// - [`ResolveModeFlags`]
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
pub struct PhysicalDeviceDepthStencilResolveProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    supported_depth_resolve_modes: ResolveModeFlags,
    ///No documentation found
    supported_stencil_resolve_modes: ResolveModeFlags,
    ///No documentation found
    independent_resolve_none: Bool32,
    ///No documentation found
    independent_resolve: Bool32,
}
///[VkSubpassDescriptionDepthStencilResolve](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassDescriptionDepthStencilResolve.html) - Structure specifying depth/stencil resolve operations for a subpass
///# C Specifications
///If the [`p_next`] chain of [`SubpassDescription2`] includes a
///[`SubpassDescriptionDepthStencilResolve`] structure, then that structure
///describes multisample resolve operations for the depth/stencil attachment in
///a subpass.The [`SubpassDescriptionDepthStencilResolve`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkSubpassDescriptionDepthStencilResolve {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkResolveModeFlagBits            depthResolveMode;
///    VkResolveModeFlagBits            stencilResolveMode;
///    const VkAttachmentReference2*    pDepthStencilResolveAttachment;
///} VkSubpassDescriptionDepthStencilResolve;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_depth_stencil_resolve
///typedef VkSubpassDescriptionDepthStencilResolve VkSubpassDescriptionDepthStencilResolveKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`depth_resolve_mode`] is a [`ResolveModeFlagBits`] value describing the depth resolve mode.
/// - [`stencil_resolve_mode`] is a [`ResolveModeFlagBits`] value describing the stencil resolve
///   mode.
/// - [`p_depth_stencil_resolve_attachment`] is `NULL` or a pointer to a [`AttachmentReference2`]
///   structure defining the depth/stencil resolve attachment for this subpass and its layout.
///# Description
///If [`p_depth_stencil_resolve_attachment`] is `NULL`, or if its attachment
///index is [`ATTACHMENT_UNUSED`], it indicates that no depth/stencil
///resolve attachment will be used in the subpass.Valid Usage
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], `pDepthStencilAttachment`**must** not be `NULL` or have the value
///   [`ATTACHMENT_UNUSED`]
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], [`depth_resolve_mode`] and [`stencil_resolve_mode`]**must** not both be
///   `VK_RESOLVE_MODE_NONE`
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], `pDepthStencilAttachment`**must** not have a sample count of
///   `VK_SAMPLE_COUNT_1_BIT`
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], [`p_depth_stencil_resolve_attachment`]**must** have a sample count of
///   `VK_SAMPLE_COUNT_1_BIT`
/// -    If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`] then it **must** have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`] and [`Format`] of [`p_depth_stencil_resolve_attachment`] has a depth
///   component, then the [`Format`] of `pDepthStencilAttachment`**must** have a depth component
///   with the same number of bits and numerical type
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], and [`Format`] of [`p_depth_stencil_resolve_attachment`] has a stencil
///   component, then the [`Format`] of `pDepthStencilAttachment`**must** have a stencil component
///   with the same number of bits and numerical type
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`] and the [`Format`] of [`p_depth_stencil_resolve_attachment`] has a depth
///   component, then the value of [`depth_resolve_mode`]**must** be one of the bits set in
///   [`PhysicalDeviceDepthStencilResolveProperties::supported_depth_resolve_modes`] or
///   `VK_RESOLVE_MODE_NONE`
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`] and the [`Format`] of [`p_depth_stencil_resolve_attachment`] has a
///   stencil component, then the value of [`stencil_resolve_mode`]**must** be one of the bits set
///   in [`PhysicalDeviceDepthStencilResolveProperties::supported_stencil_resolve_modes`] or
///   `VK_RESOLVE_MODE_NONE`
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], the [`Format`] of [`p_depth_stencil_resolve_attachment`] has both depth
///   and stencil components, [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve`]
///   is [`FALSE`], and [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve_none`] is
///   [`FALSE`], then the values of [`depth_resolve_mode`] and [`stencil_resolve_mode`]**must** be
///   identical
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], the [`Format`] of [`p_depth_stencil_resolve_attachment`] has both depth
///   and stencil components, [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve`]
///   is [`FALSE`] and [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve_none`] is
///   [`TRUE`], then the values of [`depth_resolve_mode`] and [`stencil_resolve_mode`]**must** be
///   identical or one of them **must** be `VK_RESOLVE_MODE_NONE`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE`
/// - If [`p_depth_stencil_resolve_attachment`] is not `NULL`,
///   [`p_depth_stencil_resolve_attachment`]**must** be a valid pointer to a valid
///   [`AttachmentReference2`] structure
///# Related
/// - [`VK_KHR_depth_stencil_resolve`]
/// - [`crate::vulkan1_2`]
/// - [`AttachmentReference2`]
/// - [`ResolveModeFlagBits`]
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
pub struct SubpassDescriptionDepthStencilResolve<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`depth_resolve_mode`] is a [`ResolveModeFlagBits`] value describing
    ///the depth resolve mode.
    depth_resolve_mode: ResolveModeFlagBits,
    ///[`stencil_resolve_mode`] is a [`ResolveModeFlagBits`] value
    ///describing the stencil resolve mode.
    stencil_resolve_mode: ResolveModeFlagBits,
    ///[`p_depth_stencil_resolve_attachment`] is `NULL` or a pointer to a
    ///[`AttachmentReference2`] structure defining the depth/stencil
    ///resolve attachment for this subpass and its layout.
    p_depth_stencil_resolve_attachment: *mut AttachmentReference2<'lt>,
}
///[VkImageStencilUsageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageStencilUsageCreateInfo.html) - Specify separate usage flags for the stencil aspect of a depth-stencil image
///# C Specifications
///The [`ImageStencilUsageCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkImageStencilUsageCreateInfo {
///    VkStructureType      sType;
///    const void*          pNext;
///    VkImageUsageFlags    stencilUsage;
///} VkImageStencilUsageCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_separate_stencil_usage
///typedef VkImageStencilUsageCreateInfo VkImageStencilUsageCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stencil_usage`] is a bitmask of [`ImageUsageFlagBits`] describing the intended usage of the
///   stencil aspect of the image.
///# Description
///If the [`p_next`] chain of [`ImageCreateInfo`] includes a
///[`ImageStencilUsageCreateInfo`] structure, then that structure includes
///the usage flags specific to the stencil aspect of the image for an image
///with a depth-stencil format.This structure specifies image usages which only apply to the
/// stencil aspect
///of a depth/stencil format image.
///When this structure is included in the [`p_next`] chain of
///[`ImageCreateInfo`], the stencil aspect of the image **must** only be used
///as specified by [`stencil_usage`].
///When this structure is not included in the [`p_next`] chain of
///[`ImageCreateInfo`], the stencil aspect of an image **must** only be used
///as specified by [`ImageCreateInfo::usage`].
///Use of other aspects of an image are unaffected by this structure.This structure **can** also be
/// included in the [`p_next`] chain of
///[`PhysicalDeviceImageFormatInfo2`] to query additional capabilities
///specific to image creation parameter combinations including a separate set
///of usage flags for the stencil aspect of the image using
///[`GetPhysicalDeviceImageFormatProperties2`].
///When this structure is not included in the [`p_next`] chain of
///[`PhysicalDeviceImageFormatInfo2`] then the implicit value of
///[`stencil_usage`] matches that of
///[`PhysicalDeviceImageFormatInfo2::usage`].Valid Usage
/// - If [`stencil_usage`] includes `VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT`, it **must** not
///   include bits other than `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` or
///   `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO`
/// - [`stencil_usage`]**must** be a valid combination of [`ImageUsageFlagBits`] values
/// - [`stencil_usage`]**must** not be `0`
///# Related
/// - [`VK_EXT_separate_stencil_usage`]
/// - [`crate::vulkan1_2`]
/// - [`ImageUsageFlags`]
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
pub struct ImageStencilUsageCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`stencil_usage`] is a bitmask of [`ImageUsageFlagBits`] describing
    ///the intended usage of the stencil aspect of the image.
    stencil_usage: ImageUsageFlags,
}
///[VkPhysicalDeviceScalarBlockLayoutFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceScalarBlockLayoutFeatures.html) - Structure indicating support for scalar block layouts
///# C Specifications
///The [`PhysicalDeviceScalarBlockLayoutFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceScalarBlockLayoutFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           scalarBlockLayout;
///} VkPhysicalDeviceScalarBlockLayoutFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_scalar_block_layout
///typedef VkPhysicalDeviceScalarBlockLayoutFeatures VkPhysicalDeviceScalarBlockLayoutFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`scalar_block_layout`] indicates that the implementation supports the layout of resource blocks in shaders using [scalar alignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-alignment-requirements).
///If the [`PhysicalDeviceScalarBlockLayoutFeatures`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceScalarBlockLayoutFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES`
///# Related
/// - [`VK_EXT_scalar_block_layout`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceScalarBlockLayoutFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`scalar_block_layout`]
    ///indicates that the implementation supports the layout of resource blocks
    ///in shaders using [scalar
    ///alignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-alignment-requirements).
    scalar_block_layout: Bool32,
}
///[VkPhysicalDeviceUniformBufferStandardLayoutFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceUniformBufferStandardLayoutFeatures.html) - Structure indicating support for std430-like packing in uniform buffers
///# C Specifications
///The [`PhysicalDeviceUniformBufferStandardLayoutFeatures`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           uniformBufferStandardLayout;
///} VkPhysicalDeviceUniformBufferStandardLayoutFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_uniform_buffer_standard_layout
///typedef VkPhysicalDeviceUniformBufferStandardLayoutFeatures
/// VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`uniform_buffer_standard_layout`] indicates that the implementation supports the same layouts
///   for uniform buffers as for storage and other kinds of buffers. See [Standard Buffer Layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-resources-standard-layout).
///If the [`PhysicalDeviceUniformBufferStandardLayoutFeatures`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceUniformBufferStandardLayoutFeatures`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES`
///# Related
/// - [`VK_KHR_uniform_buffer_standard_layout`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`uniform_buffer_standard_layout`] indicates that the implementation
    ///supports the same layouts for uniform buffers as for storage and other
    ///kinds of buffers.
    ///See [Standard Buffer Layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-resources-standard-layout).
    uniform_buffer_standard_layout: Bool32,
}
///[VkPhysicalDeviceBufferDeviceAddressFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeatures.html) - Structure describing buffer address features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceBufferDeviceAddressFeatures`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceBufferDeviceAddressFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           bufferDeviceAddress;
///    VkBool32           bufferDeviceAddressCaptureReplay;
///    VkBool32           bufferDeviceAddressMultiDevice;
///} VkPhysicalDeviceBufferDeviceAddressFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_buffer_device_address
///typedef VkPhysicalDeviceBufferDeviceAddressFeatures
/// VkPhysicalDeviceBufferDeviceAddressFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`buffer_device_address`] indicates that the implementation supports accessing buffer memory
///   in shaders as storage buffers via an address queried from [`GetBufferDeviceAddress`].
/// - [`buffer_device_address_capture_replay`] indicates that the implementation supports saving and
///   reusing buffer and device addresses, e.g. for trace capture and replay.
/// - [`buffer_device_address_multi_device`] indicates that the implementation supports the
///   [`buffer_device_address`] , `rayTracingPipeline` and `rayQuery` features for logical devices
///   created with multiple physical devices. If this feature is not supported, buffer and
///   acceleration structure addresses **must** not be queried on a logical device created with more
///   than one physical device.
///See [`GetBufferDeviceAddress`] for more information.If the
/// [`PhysicalDeviceBufferDeviceAddressFeatures`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceBufferDeviceAddressFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES`
///# Related
/// - [`VK_KHR_buffer_device_address`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceBufferDeviceAddressFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`buffer_device_address`] indicates that the implementation supports
    ///accessing buffer memory in shaders as storage buffers via an address
    ///queried from [`GetBufferDeviceAddress`].
    buffer_device_address: Bool32,
    ///[`buffer_device_address_capture_replay`] indicates that the implementation
    ///supports saving and reusing buffer and device addresses, e.g. for trace
    ///capture and replay.
    buffer_device_address_capture_replay: Bool32,
    ///[`buffer_device_address_multi_device`] indicates that the implementation
    ///supports the [`buffer_device_address`]
    ///, `rayTracingPipeline` and `rayQuery` features
    ///for logical devices created with multiple physical devices.
    ///If this feature is not supported, buffer
    ///and acceleration structure
    ///addresses **must** not be queried on a logical device created with more
    ///than one physical device.
    buffer_device_address_multi_device: Bool32,
}
///[VkBufferDeviceAddressInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressInfo.html) - Structure specifying the buffer to query an address for
///# C Specifications
///The [`BufferDeviceAddressInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkBufferDeviceAddressInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBuffer           buffer;
///} VkBufferDeviceAddressInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_buffer_device_address
///typedef VkBufferDeviceAddressInfo VkBufferDeviceAddressInfoKHR;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_buffer_device_address
///typedef VkBufferDeviceAddressInfo VkBufferDeviceAddressInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer`] specifies the buffer whose address is being queried.
///# Description
///Valid Usage
/// - If [`buffer`] is non-sparse and was not created with the
///   `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT` flag, then it **must** be bound
///   completely and contiguously to a single [`DeviceMemory`] object
/// - [`buffer`]**must** have been created with `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`buffer`]**must** be a valid [`Buffer`] handle
///# Related
/// - [`crate::vulkan1_2`]
/// - [`Buffer`]
/// - [`StructureType`]
/// - [`GetBufferDeviceAddress`]
/// - [`GetBufferDeviceAddressEXT`]
/// - [`GetBufferDeviceAddressKHR`]
/// - [`GetBufferOpaqueCaptureAddress`]
/// - [`GetBufferOpaqueCaptureAddressKHR`]
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
pub struct BufferDeviceAddressInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`buffer`] specifies the buffer whose address is being queried.
    buffer: Buffer,
}
///[VkBufferOpaqueCaptureAddressCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferOpaqueCaptureAddressCreateInfo.html) - Request a specific address for a buffer
///# C Specifications
///To request a specific device address for a buffer, add a
///[`BufferOpaqueCaptureAddressCreateInfo`] structure to the [`p_next`]
///chain of the [`BufferCreateInfo`] structure.
///The [`BufferOpaqueCaptureAddressCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkBufferOpaqueCaptureAddressCreateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint64_t           opaqueCaptureAddress;
///} VkBufferOpaqueCaptureAddressCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_buffer_device_address
///typedef VkBufferOpaqueCaptureAddressCreateInfo VkBufferOpaqueCaptureAddressCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`opaque_capture_address`] is the opaque capture address requested for the buffer.
///# Description
///If [`opaque_capture_address`] is zero, no specific address is requested.If
/// [`opaque_capture_address`] is not zero, then it **should** be an address
///retrieved from [`GetBufferOpaqueCaptureAddress`] for an identically
///created buffer on the same implementation.If this structure is not present, it is as if
/// [`opaque_capture_address`] is
///zero.Apps **should** avoid creating buffers with app-provided addresses and
///implementation-provided addresses in the same process, to reduce the
///likelihood of `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS` errors.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO`
///# Related
/// - [`VK_KHR_buffer_device_address`]
/// - [`crate::vulkan1_2`]
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
pub struct BufferOpaqueCaptureAddressCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`opaque_capture_address`] is the opaque capture address requested for
    ///the buffer.
    opaque_capture_address: u64,
}
///[VkPhysicalDeviceImagelessFramebufferFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImagelessFramebufferFeatures.html) - Structure indicating support for imageless framebuffers
///# C Specifications
///The [`PhysicalDeviceImagelessFramebufferFeatures`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceImagelessFramebufferFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           imagelessFramebuffer;
///} VkPhysicalDeviceImagelessFramebufferFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_imageless_framebuffer
///typedef VkPhysicalDeviceImagelessFramebufferFeatures
/// VkPhysicalDeviceImagelessFramebufferFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`imageless_framebuffer`] indicates that the implementation supports specifying the image view
///   for attachments at render pass begin time via [`RenderPassAttachmentBeginInfo`].
///If the [`PhysicalDeviceImagelessFramebufferFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceImagelessFramebufferFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES`
///# Related
/// - [`VK_KHR_imageless_framebuffer`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceImagelessFramebufferFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`imageless_framebuffer`] indicates that the implementation supports
    ///specifying the image view for attachments at render pass begin time via
    ///[`RenderPassAttachmentBeginInfo`].
    imageless_framebuffer: Bool32,
}
///[VkFramebufferAttachmentsCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentsCreateInfo.html) - Structure specifying parameters of images that will be used with a framebuffer
///# C Specifications
///The [`FramebufferAttachmentsCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkFramebufferAttachmentsCreateInfo {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    uint32_t                                   attachmentImageInfoCount;
///    const VkFramebufferAttachmentImageInfo*    pAttachmentImageInfos;
///} VkFramebufferAttachmentsCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_imageless_framebuffer
///typedef VkFramebufferAttachmentsCreateInfo VkFramebufferAttachmentsCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attachment_image_info_count`] is the number of attachments being described.
/// - [`p_attachment_image_infos`] is a pointer to an array of [`FramebufferAttachmentImageInfo`]
///   structures, each structure describing a number of parameters of the corresponding attachment
///   in a render pass instance.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO`
/// - If [`attachment_image_info_count`] is not `0`, [`p_attachment_image_infos`]**must** be a valid
///   pointer to an array of [`attachment_image_info_count`] valid
///   [`FramebufferAttachmentImageInfo`] structures
///# Related
/// - [`crate::vulkan1_2`]
/// - [`FramebufferAttachmentImageInfo`]
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
pub struct FramebufferAttachmentsCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`attachment_image_info_count`] is the number of attachments being
    ///described.
    attachment_image_info_count: u32,
    ///[`p_attachment_image_infos`] is a pointer to an array of
    ///[`FramebufferAttachmentImageInfo`] structures, each structure
    ///describing a number of parameters of the corresponding attachment in a
    ///render pass instance.
    p_attachment_image_infos: *mut FramebufferAttachmentImageInfo<'lt>,
}
///[VkFramebufferAttachmentImageInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferAttachmentImageInfo.html) - Structure specifying parameters of an image that will be used with a framebuffer
///# C Specifications
///The [`FramebufferAttachmentImageInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkFramebufferAttachmentImageInfo {
///    VkStructureType       sType;
///    const void*           pNext;
///    VkImageCreateFlags    flags;
///    VkImageUsageFlags     usage;
///    uint32_t              width;
///    uint32_t              height;
///    uint32_t              layerCount;
///    uint32_t              viewFormatCount;
///    const VkFormat*       pViewFormats;
///} VkFramebufferAttachmentImageInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_imageless_framebuffer
///typedef VkFramebufferAttachmentImageInfo VkFramebufferAttachmentImageInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`ImageCreateFlagBits`], matching the value of
///   [`ImageCreateInfo`]::[`flags`] used to create an image that will be used with this
///   framebuffer.
/// - [`usage`] is a bitmask of [`ImageUsageFlagBits`], matching the value of
///   [`ImageCreateInfo`]::[`usage`] used to create an image used with this framebuffer.
/// - [`width`] is the width of the image view used for rendering.
/// - [`height`] is the height of the image view used for rendering.
/// - [`layer_count`] is the number of array layers of the image view used for rendering.
/// - [`view_format_count`] is the number of entries in the [`p_view_formats`] array, matching the
///   value of [`ImageFormatListCreateInfo`]::[`view_format_count`] used to create an image used
///   with this framebuffer.
/// - [`p_view_formats`] is a pointer to an array of [`Format`] values specifying all of the formats
///   which **can** be used when creating views of the image, matching the value of
///   [`ImageFormatListCreateInfo`]::[`p_view_formats`] used to create an image used with this
///   framebuffer.
///# Description
///Images that **can** be used with the framebuffer when beginning a render pass,
///as specified by [`RenderPassAttachmentBeginInfo`], **must** be created with
///parameters that are identical to those specified here.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be a valid combination of [`ImageCreateFlagBits`] values
/// - [`usage`]**must** be a valid combination of [`ImageUsageFlagBits`] values
/// - [`usage`]**must** not be `0`
/// - If [`view_format_count`] is not `0`, [`p_view_formats`]**must** be a valid pointer to an array
///   of [`view_format_count`] valid [`Format`] values
///# Related
/// - [`VK_KHR_imageless_framebuffer`]
/// - [`crate::vulkan1_2`]
/// - [`Format`]
/// - [`FramebufferAttachmentsCreateInfo`]
/// - [`ImageCreateFlags`]
/// - [`ImageUsageFlags`]
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
pub struct FramebufferAttachmentImageInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`ImageCreateFlagBits`], matching the
    ///value of [`ImageCreateInfo`]::[`flags`] used to create an image
    ///that will be used with this framebuffer.
    flags: ImageCreateFlags,
    ///[`usage`] is a bitmask of [`ImageUsageFlagBits`], matching the
    ///value of [`ImageCreateInfo`]::[`usage`] used to create an image
    ///used with this framebuffer.
    usage: ImageUsageFlags,
    ///[`width`] is the width of the image view used for rendering.
    width: u32,
    ///[`height`] is the height of the image view used for rendering.
    height: u32,
    ///[`layer_count`] is the number of array layers of the image view used
    ///for rendering.
    layer_count: u32,
    ///[`view_format_count`] is the number of entries in the [`p_view_formats`]
    ///array, matching the value of
    ///[`ImageFormatListCreateInfo`]::[`view_format_count`] used to create
    ///an image used with this framebuffer.
    view_format_count: u32,
    ///[`p_view_formats`] is a pointer to an array of [`Format`] values
    ///specifying all of the formats which **can** be used when creating views of
    ///the image, matching the value of
    ///[`ImageFormatListCreateInfo`]::[`p_view_formats`] used to create an
    ///image used with this framebuffer.
    p_view_formats: *mut Format,
}
///[VkRenderPassAttachmentBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassAttachmentBeginInfo.html) - Structure specifying images to be used as framebuffer attachments
///# C Specifications
///The [`RenderPassAttachmentBeginInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkRenderPassAttachmentBeginInfo {
///    VkStructureType       sType;
///    const void*           pNext;
///    uint32_t              attachmentCount;
///    const VkImageView*    pAttachments;
///} VkRenderPassAttachmentBeginInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_imageless_framebuffer
///typedef VkRenderPassAttachmentBeginInfo VkRenderPassAttachmentBeginInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attachment_count`] is the number of attachments.
/// - [`p_attachments`] is a pointer to an array of [`ImageView`] handles, each of which will be
///   used as the corresponding attachment in the render pass instance.
///# Description
///Valid Usage
/// - Each element of [`p_attachments`]**must** only specify a single mip level
/// - Each element of [`p_attachments`]**must** have been created with the identity swizzle
/// - Each element of [`p_attachments`]**must** have been created with
///   [`ImageViewCreateInfo::view_type`] not equal to `VK_IMAGE_VIEW_TYPE_3D`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO`
/// - If [`attachment_count`] is not `0`, [`p_attachments`]**must** be a valid pointer to an array
///   of [`attachment_count`] valid [`ImageView`] handles
///# Related
/// - [`VK_KHR_imageless_framebuffer`]
/// - [`crate::vulkan1_2`]
/// - [`ImageView`]
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
pub struct RenderPassAttachmentBeginInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`attachment_count`] is the number of attachments.
    attachment_count: u32,
    ///[`p_attachments`] is a pointer to an array of [`ImageView`]
    ///handles, each of which will be used as the corresponding attachment in
    ///the render pass instance.
    p_attachments: *mut ImageView,
}
///[VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures.html) - Structure describing whether the implementation can do depth and stencil image barriers separately
///# C Specifications
///The [`PhysicalDeviceSeparateDepthStencilLayoutsFeatures`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           separateDepthStencilLayouts;
///} VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_separate_depth_stencil_layouts
///typedef VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures
/// VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`separate_depth_stencil_layouts`] indicates whether the implementation supports a
///   [`ImageMemoryBarrier`] for a depth/stencil image with only one of `VK_IMAGE_ASPECT_DEPTH_BIT`
///   or `VK_IMAGE_ASPECT_STENCIL_BIT` set, and whether `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` can be used.
///If the [`PhysicalDeviceSeparateDepthStencilLayoutsFeatures`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceSeparateDepthStencilLayoutsFeatures`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES`
///# Related
/// - [`VK_KHR_separate_depth_stencil_layouts`]
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`separate_depth_stencil_layouts`] indicates whether the implementation
    ///supports a [`ImageMemoryBarrier`] for a depth/stencil image with
    ///only one of `VK_IMAGE_ASPECT_DEPTH_BIT` or
    ///`VK_IMAGE_ASPECT_STENCIL_BIT` set, and whether
    ///`VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
    ///`VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
    ///`VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or
    ///`VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` can be used.
    separate_depth_stencil_layouts: Bool32,
}
///[VkAttachmentReferenceStencilLayout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentReferenceStencilLayout.html) - Structure specifying an attachment description
///# C Specifications
///The [`AttachmentReferenceStencilLayout`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkAttachmentReferenceStencilLayout {
///    VkStructureType    sType;
///    void*              pNext;
///    VkImageLayout      stencilLayout;
///} VkAttachmentReferenceStencilLayout;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_separate_depth_stencil_layouts
///typedef VkAttachmentReferenceStencilLayout VkAttachmentReferenceStencilLayoutKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stencil_layout`] is a [`ImageLayout`] value specifying the layout the stencil aspect of the
///   attachment uses during the subpass.
///# Description
///Valid Usage
/// - [`stencil_layout`]**must** not be `VK_IMAGE_LAYOUT_UNDEFINED`,
///   `VK_IMAGE_LAYOUT_PREINITIALIZED`, `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT`
/// - [`stencil_layout`]**must** be a valid [`ImageLayout`] value
///# Related
/// - [`VK_KHR_separate_depth_stencil_layouts`]
/// - [`crate::vulkan1_2`]
/// - [`ImageLayout`]
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
pub struct AttachmentReferenceStencilLayout<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`stencil_layout`] is a [`ImageLayout`] value specifying the layout
    ///the stencil aspect of the attachment uses during the subpass.
    stencil_layout: ImageLayout,
}
///[VkAttachmentDescriptionStencilLayout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentDescriptionStencilLayout.html) - Structure specifying an attachment description
///# C Specifications
///The [`AttachmentDescriptionStencilLayout`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkAttachmentDescriptionStencilLayout {
///    VkStructureType    sType;
///    void*              pNext;
///    VkImageLayout      stencilInitialLayout;
///    VkImageLayout      stencilFinalLayout;
///} VkAttachmentDescriptionStencilLayout;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_separate_depth_stencil_layouts
///typedef VkAttachmentDescriptionStencilLayout VkAttachmentDescriptionStencilLayoutKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stencil_initial_layout`] is the layout the stencil aspect of the attachment image
///   subresource will be in when a render pass instance begins.
/// - [`stencil_final_layout`] is the layout the stencil aspect of the attachment image subresource
///   will be transitioned to when a render pass instance ends.
///# Description
///Valid Usage
/// - [`stencil_initial_layout`]**must** not be `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - [`stencil_final_layout`]**must** not be `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - [`stencil_final_layout`]**must** not be `VK_IMAGE_LAYOUT_UNDEFINED` or
///   `VK_IMAGE_LAYOUT_PREINITIALIZED`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT`
/// - [`stencil_initial_layout`]**must** be a valid [`ImageLayout`] value
/// - [`stencil_final_layout`]**must** be a valid [`ImageLayout`] value
///# Related
/// - [`VK_KHR_separate_depth_stencil_layouts`]
/// - [`crate::vulkan1_2`]
/// - [`ImageLayout`]
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
pub struct AttachmentDescriptionStencilLayout<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`stencil_initial_layout`] is the layout the stencil aspect of the
    ///attachment image subresource will be in when a render pass instance
    ///begins.
    stencil_initial_layout: ImageLayout,
    ///[`stencil_final_layout`] is the layout the stencil aspect of the
    ///attachment image subresource will be transitioned to when a render pass
    ///instance ends.
    stencil_final_layout: ImageLayout,
}
///[VkMemoryOpaqueCaptureAddressAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryOpaqueCaptureAddressAllocateInfo.html) - Request a specific address for a memory allocation
///# C Specifications
///To request a specific device address for a memory allocation, add a
///[`MemoryOpaqueCaptureAddressAllocateInfo`] structure to the [`p_next`]
///chain of the [`MemoryAllocateInfo`] structure.
///The [`MemoryOpaqueCaptureAddressAllocateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkMemoryOpaqueCaptureAddressAllocateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint64_t           opaqueCaptureAddress;
///} VkMemoryOpaqueCaptureAddressAllocateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_buffer_device_address
///typedef VkMemoryOpaqueCaptureAddressAllocateInfo VkMemoryOpaqueCaptureAddressAllocateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`opaque_capture_address`] is the opaque capture address requested for the memory allocation.
///# Description
///If [`opaque_capture_address`] is zero, no specific address is requested.If
/// [`opaque_capture_address`] is not zero, it **should** be an address
///retrieved from [`GetDeviceMemoryOpaqueCaptureAddress`] on an identically
///created memory allocation on the same implementation.If this structure is not present, it is as
/// if [`opaque_capture_address`] is
///zero.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO`
///# Related
/// - [`VK_KHR_buffer_device_address`]
/// - [`crate::vulkan1_2`]
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
pub struct MemoryOpaqueCaptureAddressAllocateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`opaque_capture_address`] is the opaque capture address requested for
    ///the memory allocation.
    opaque_capture_address: u64,
}
///[VkDeviceMemoryOpaqueCaptureAddressInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOpaqueCaptureAddressInfo.html) - Structure specifying the memory object to query an address for
///# C Specifications
///The [`DeviceMemoryOpaqueCaptureAddressInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkDeviceMemoryOpaqueCaptureAddressInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkDeviceMemory     memory;
///} VkDeviceMemoryOpaqueCaptureAddressInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_buffer_device_address
///typedef VkDeviceMemoryOpaqueCaptureAddressInfo VkDeviceMemoryOpaqueCaptureAddressInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory`] specifies the memory whose address is being queried.
///# Description
///Valid Usage
/// - [`memory`]**must** have been allocated with `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
///# Related
/// - [`VK_KHR_buffer_device_address`]
/// - [`crate::vulkan1_2`]
/// - [`DeviceMemory`]
/// - [`StructureType`]
/// - [`GetDeviceMemoryOpaqueCaptureAddress`]
/// - [`GetDeviceMemoryOpaqueCaptureAddressKHR`]
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
pub struct DeviceMemoryOpaqueCaptureAddressInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`memory`] specifies the memory whose address is being queried.
    memory: DeviceMemory,
}
///[VkPhysicalDeviceVulkan11Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Features.html) - Structure describing the Vulkan 1.1 features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceVulkan11Features`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceVulkan11Features {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           storageBuffer16BitAccess;
///    VkBool32           uniformAndStorageBuffer16BitAccess;
///    VkBool32           storagePushConstant16;
///    VkBool32           storageInputOutput16;
///    VkBool32           multiview;
///    VkBool32           multiviewGeometryShader;
///    VkBool32           multiviewTessellationShader;
///    VkBool32           variablePointersStorageBuffer;
///    VkBool32           variablePointers;
///    VkBool32           protectedMemory;
///    VkBool32           samplerYcbcrConversion;
///    VkBool32           shaderDrawParameters;
///} VkPhysicalDeviceVulkan11Features;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`storage_buffer_16_bit_access`] specifies whether objects in the     `StorageBuffer`,
///   `ShaderRecordBufferKHR`,     or `PhysicalStorageBuffer`     storage class with the `Block`
///   decoration **can** have 16-bit integer     and 16-bit floating-point members.     If this
///   feature is not enabled, 16-bit integer or 16-bit floating-point     members **must** not be
///   used in such objects.     This also specifies whether shader modules **can** declare the
///   `StorageBuffer16BitAccess` capability.
/// - [`uniform_and_storage_buffer_16_bit_access`] specifies whether objects in the `Uniform`
///   storage class with the `Block` decoration **can** have 16-bit integer and 16-bit
///   floating-point members. If this feature is not enabled, 16-bit integer or 16-bit
///   floating-point members **must** not be used in such objects. This also specifies whether
///   shader modules **can** declare the `UniformAndStorageBuffer16BitAccess` capability.
/// - [`storage_push_constant_16`] specifies whether objects in the `PushConstant` storage class
///   **can** have 16-bit integer and 16-bit floating-point members. If this feature is not enabled,
///   16-bit integer or floating-point members **must** not be used in such objects. This also
///   specifies whether shader modules **can** declare the `StoragePushConstant16` capability.
/// - [`storage_input_output_16`] specifies whether objects in the `Input` and `Output` storage
///   classes **can** have 16-bit integer and 16-bit floating-point members. If this feature is not
///   enabled, 16-bit integer or 16-bit floating-point members **must** not be used in such objects.
///   This also specifies whether shader modules **can** declare the `StorageInputOutput16`
///   capability.
/// - [`multiview`] specifies whether the implementation supports multiview rendering within a
///   render pass. If this feature is not enabled, the view mask of each subpass **must** always be
///   zero.
/// - [`multiview_geometry_shader`] specifies whether the implementation supports multiview
///   rendering within a render pass, with [geometry shaders](). If this feature is not enabled,
///   then a pipeline compiled against a subpass with a non-zero view mask **must** not include a
///   geometry shader.
/// - [`multiview_tessellation_shader`] specifies whether the implementation supports multiview
///   rendering within a render pass, with [tessellation shaders](). If this feature is not enabled,
///   then a pipeline compiled against a subpass with a non-zero view mask **must** not include any
///   tessellation shaders.
/// - [`variable_pointers_storage_buffer`] specifies whether the implementation supports the SPIR-V
///   `VariablePointersStorageBuffer` capability. When this feature is not enabled, shader modules
///   **must** not declare the `SPV_KHR_variable_pointers` extension or the
///   `VariablePointersStorageBuffer` capability.
/// - [`variable_pointers`] specifies whether the implementation supports the SPIR-V
///   `VariablePointers` capability. When this feature is not enabled, shader modules **must** not
///   declare the `VariablePointers` capability.
/// - [`protected_memory`] specifies whether protected memory is supported.
/// - [`sampler_ycbcr_conversion`] specifies whether the implementation supports [sampler
///   Y′C<sub>B</sub>C<sub>R</sub> conversion](). If [`sampler_ycbcr_conversion`] is [`FALSE`],
///   sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is not supported, and samplers using sampler
///   Y′C<sub>B</sub>C<sub>R</sub> conversion **must** not be used.
/// - [`shader_draw_parameters`] specifies whether the implementation supports the SPIR-V
///   `DrawParameters` capability. When this feature is not enabled, shader modules **must** not
///   declare the `SPV_KHR_shader_draw_parameters` extension or the `DrawParameters` capability.
///If the [`PhysicalDeviceVulkan11Features`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVulkan11Features`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES`
///# Related
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceVulkan11Features<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`storage_buffer_16_bit_access`] specifies whether objects in the
    ///    `StorageBuffer`,
    ///`ShaderRecordBufferKHR`,
    ///    or `PhysicalStorageBuffer`
    ///    storage class with the `Block` decoration **can** have 16-bit integer
    ///    and 16-bit floating-point members.
    ///    If this feature is not enabled, 16-bit integer or 16-bit floating-point
    ///    members **must** not be used in such objects.
    ///    This also specifies whether shader modules **can** declare the
    ///    `StorageBuffer16BitAccess` capability.
    storage_buffer_16_bit_access: Bool32,
    ///[`uniform_and_storage_buffer_16_bit_access`] specifies whether objects in
    ///the `Uniform` storage class with the `Block` decoration **can** have
    ///16-bit integer and 16-bit floating-point members.
    ///If this feature is not enabled, 16-bit integer or 16-bit floating-point
    ///members **must** not be used in such objects.
    ///This also specifies whether shader modules **can** declare the
    ///`UniformAndStorageBuffer16BitAccess` capability.
    uniform_and_storage_buffer_16_bit_access: Bool32,
    ///[`storage_push_constant_16`] specifies whether objects in the
    ///`PushConstant` storage class **can** have 16-bit integer and 16-bit
    ///floating-point members.
    ///If this feature is not enabled, 16-bit integer or floating-point members
    ///**must** not be used in such objects.
    ///This also specifies whether shader modules **can** declare the
    ///`StoragePushConstant16` capability.
    storage_push_constant_16: Bool32,
    ///[`storage_input_output_16`] specifies whether objects in the `Input`
    ///and `Output` storage classes **can** have 16-bit integer and 16-bit
    ///floating-point members.
    ///If this feature is not enabled, 16-bit integer or 16-bit floating-point
    ///members **must** not be used in such objects.
    ///This also specifies whether shader modules **can** declare the
    ///`StorageInputOutput16` capability.
    storage_input_output_16: Bool32,
    ///[`multiview`] specifies whether
    ///the implementation supports multiview rendering within a render pass.
    ///If this feature is not enabled, the view mask of each subpass **must**
    ///always be zero.
    multiview: Bool32,
    ///[`multiview_geometry_shader`]
    ///specifies whether the implementation supports multiview rendering within
    ///a render pass, with [geometry shaders]().
    ///If this feature is not enabled, then a pipeline compiled against a
    ///subpass with a non-zero view mask **must** not include a geometry shader.
    multiview_geometry_shader: Bool32,
    ///[`multiview_tessellation_shader`] specifies whether the implementation
    ///supports multiview rendering within a render pass, with
    ///[tessellation shaders]().
    ///If this feature is not enabled, then a pipeline compiled against a
    ///subpass with a non-zero view mask **must** not include any tessellation
    ///shaders.
    multiview_tessellation_shader: Bool32,
    ///[`variable_pointers_storage_buffer`] specifies whether the implementation
    ///supports the SPIR-V `VariablePointersStorageBuffer` capability.
    ///When this feature is not enabled, shader modules **must** not declare the
    ///`SPV_KHR_variable_pointers` extension or the
    ///`VariablePointersStorageBuffer` capability.
    variable_pointers_storage_buffer: Bool32,
    ///[`variable_pointers`]
    ///specifies whether the implementation supports the SPIR-V
    ///`VariablePointers` capability.
    ///When this feature is not enabled, shader modules **must** not declare the
    ///`VariablePointers` capability.
    variable_pointers: Bool32,
    ///[`protected_memory`]
    ///specifies whether protected memory is supported.
    protected_memory: Bool32,
    ///[`sampler_ycbcr_conversion`] specifies whether the implementation
    ///supports [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion]().
    ///If [`sampler_ycbcr_conversion`] is [`FALSE`], sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///conversion is not supported, and samplers using sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///conversion **must** not be used.
    sampler_ycbcr_conversion: Bool32,
    ///[`shader_draw_parameters`] specifies whether the implementation supports
    ///the SPIR-V `DrawParameters` capability.
    ///When this feature is not enabled, shader modules **must** not declare the
    ///`SPV_KHR_shader_draw_parameters` extension or the `DrawParameters`
    ///capability.
    shader_draw_parameters: Bool32,
}
///[VkPhysicalDeviceVulkan11Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan11Properties.html) - Structure specifying physical device properties for functionality promoted to Vulkan 1.1
///# C Specifications
///The [`PhysicalDeviceVulkan11Properties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceVulkan11Properties {
///    VkStructureType            sType;
///    void*                      pNext;
///    uint8_t                    deviceUUID[VK_UUID_SIZE];
///    uint8_t                    driverUUID[VK_UUID_SIZE];
///    uint8_t                    deviceLUID[VK_LUID_SIZE];
///    uint32_t                   deviceNodeMask;
///    VkBool32                   deviceLUIDValid;
///    uint32_t                   subgroupSize;
///    VkShaderStageFlags         subgroupSupportedStages;
///    VkSubgroupFeatureFlags     subgroupSupportedOperations;
///    VkBool32                   subgroupQuadOperationsInAllStages;
///    VkPointClippingBehavior    pointClippingBehavior;
///    uint32_t                   maxMultiviewViewCount;
///    uint32_t                   maxMultiviewInstanceIndex;
///    VkBool32                   protectedNoFault;
///    uint32_t                   maxPerSetDescriptors;
///    VkDeviceSize               maxMemoryAllocationSize;
///} VkPhysicalDeviceVulkan11Properties;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`device_uuid`] is an array of [`UUID_SIZE`]`uint8_t` values representing a universally unique
///   identifier for the device.
/// - [`driver_uuid`] is an array of [`UUID_SIZE`]`uint8_t` values representing a universally unique
///   identifier for the driver build in use by the device.
/// - [`device_luid`] is an array of [`LUID_SIZE`]`uint8_t` values representing a locally unique
///   identifier for the device.
/// - [`device_node_mask`] is a `uint32_t` bitfield identifying the node within a linked device
///   adapter corresponding to the device.
/// - [`device_luid_valid`] is a boolean value that will be [`TRUE`] if [`device_luid`] contains a
///   valid LUID and [`device_node_mask`] contains a valid node mask, and [`FALSE`] if they do not.
/// - [`subgroup_size`] is the default number of invocations in each subgroup. [`subgroup_size`] is
///   at least 1 if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or
///   `VK_QUEUE_COMPUTE_BIT`. [`subgroup_size`] is a power-of-two.
/// - [`subgroup_supported_stages`] is a bitfield of [`ShaderStageFlagBits`] describing the shader stages that [group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) with [subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup) are supported in. [`subgroup_supported_stages`] will have the `VK_SHADER_STAGE_COMPUTE_BIT` bit set if any of the physical device’s queues support `VK_QUEUE_COMPUTE_BIT`.
/// - [`subgroup_supported_operations`] is a bitmask of [`SubgroupFeatureFlagBits`] specifying the sets of [group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) with [subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup) supported on this device. [`subgroup_supported_operations`] will have the `VK_SUBGROUP_FEATURE_BASIC_BIT` bit set if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`.
/// - [`subgroup_quad_operations_in_all_stages`] is a boolean specifying whether [quad group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-quad-operations)
///   are available in all stages, or are restricted to fragment and compute stages.
/// - [`point_clipping_behavior`] is a [`PointClippingBehavior`] value specifying the point clipping
///   behavior supported by the implementation.
/// - [`max_multiview_view_count`] is one greater than the maximum view index that **can** be used
///   in a subpass.
/// - [`max_multiview_instance_index`] is the maximum valid value of instance index allowed to be
///   generated by a drawing command recorded within a subpass of a multiview render pass instance.
/// - [`protected_no_fault`] specifies how an implementation behaves when an application attempts to
///   write to unprotected memory in a protected queue operation, read from protected memory in an
///   unprotected queue operation, or perform a query in a protected queue operation. If this limit
///   is [`TRUE`], such writes will be discarded or have undefined values written, reads and queries
///   will return undefined values. If this limit is [`FALSE`], applications **must** not perform
///   these operations. See [[memory-protected-access-rules]]() for more information.
/// - [`max_per_set_descriptors`] is a maximum number of descriptors (summed over all descriptor
///   types) in a single descriptor set that is guaranteed to satisfy any implementation-dependent
///   constraints on the size of a descriptor set itself. Applications **can** query whether a
///   descriptor set that goes beyond this limit is supported using
///   [`GetDescriptorSetLayoutSupport`].
/// - [`max_memory_allocation_size`] is the maximum size of a memory allocation that **can** be
///   created, even if there is more space available in the heap.
///If the [`PhysicalDeviceVulkan11Properties`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These properties correspond to Vulkan 1.1
/// functionality.The members of [`PhysicalDeviceVulkan11Properties`] have the same values
///as the corresponding members of [`PhysicalDeviceIdProperties`],
///[`PhysicalDeviceSubgroupProperties`],
///[`PhysicalDevicePointClippingProperties`],
///[`PhysicalDeviceMultiviewProperties`],
///[`PhysicalDeviceProtectedMemoryProperties`], and
///[`PhysicalDeviceMaintenance3Properties`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES`
///# Related
/// - [`crate::vulkan1_2`]
/// - [`Bool32`]
/// - [`DeviceSize`]
/// - [`PointClippingBehavior`]
/// - [`ShaderStageFlags`]
/// - [`StructureType`]
/// - [`SubgroupFeatureFlags`]
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
pub struct PhysicalDeviceVulkan11Properties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    device_uuid: [u8; UUID_SIZE],
    ///No documentation found
    driver_uuid: [u8; UUID_SIZE],
    ///No documentation found
    device_luid: [u8; LUID_SIZE],
    ///No documentation found
    device_node_mask: u32,
    ///No documentation found
    device_luid_valid: Bool32,
    ///No documentation found
    subgroup_size: u32,
    ///No documentation found
    subgroup_supported_stages: ShaderStageFlags,
    ///No documentation found
    subgroup_supported_operations: SubgroupFeatureFlags,
    ///No documentation found
    subgroup_quad_operations_in_all_stages: Bool32,
    ///No documentation found
    point_clipping_behavior: PointClippingBehavior,
    ///No documentation found
    max_multiview_view_count: u32,
    ///No documentation found
    max_multiview_instance_index: u32,
    ///No documentation found
    protected_no_fault: Bool32,
    ///No documentation found
    max_per_set_descriptors: u32,
    ///No documentation found
    max_memory_allocation_size: DeviceSize,
}
///[VkPhysicalDeviceVulkan12Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Features.html) - Structure describing the Vulkan 1.2 features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceVulkan12Features`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceVulkan12Features {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           samplerMirrorClampToEdge;
///    VkBool32           drawIndirectCount;
///    VkBool32           storageBuffer8BitAccess;
///    VkBool32           uniformAndStorageBuffer8BitAccess;
///    VkBool32           storagePushConstant8;
///    VkBool32           shaderBufferInt64Atomics;
///    VkBool32           shaderSharedInt64Atomics;
///    VkBool32           shaderFloat16;
///    VkBool32           shaderInt8;
///    VkBool32           descriptorIndexing;
///    VkBool32           shaderInputAttachmentArrayDynamicIndexing;
///    VkBool32           shaderUniformTexelBufferArrayDynamicIndexing;
///    VkBool32           shaderStorageTexelBufferArrayDynamicIndexing;
///    VkBool32           shaderUniformBufferArrayNonUniformIndexing;
///    VkBool32           shaderSampledImageArrayNonUniformIndexing;
///    VkBool32           shaderStorageBufferArrayNonUniformIndexing;
///    VkBool32           shaderStorageImageArrayNonUniformIndexing;
///    VkBool32           shaderInputAttachmentArrayNonUniformIndexing;
///    VkBool32           shaderUniformTexelBufferArrayNonUniformIndexing;
///    VkBool32           shaderStorageTexelBufferArrayNonUniformIndexing;
///    VkBool32           descriptorBindingUniformBufferUpdateAfterBind;
///    VkBool32           descriptorBindingSampledImageUpdateAfterBind;
///    VkBool32           descriptorBindingStorageImageUpdateAfterBind;
///    VkBool32           descriptorBindingStorageBufferUpdateAfterBind;
///    VkBool32           descriptorBindingUniformTexelBufferUpdateAfterBind;
///    VkBool32           descriptorBindingStorageTexelBufferUpdateAfterBind;
///    VkBool32           descriptorBindingUpdateUnusedWhilePending;
///    VkBool32           descriptorBindingPartiallyBound;
///    VkBool32           descriptorBindingVariableDescriptorCount;
///    VkBool32           runtimeDescriptorArray;
///    VkBool32           samplerFilterMinmax;
///    VkBool32           scalarBlockLayout;
///    VkBool32           imagelessFramebuffer;
///    VkBool32           uniformBufferStandardLayout;
///    VkBool32           shaderSubgroupExtendedTypes;
///    VkBool32           separateDepthStencilLayouts;
///    VkBool32           hostQueryReset;
///    VkBool32           timelineSemaphore;
///    VkBool32           bufferDeviceAddress;
///    VkBool32           bufferDeviceAddressCaptureReplay;
///    VkBool32           bufferDeviceAddressMultiDevice;
///    VkBool32           vulkanMemoryModel;
///    VkBool32           vulkanMemoryModelDeviceScope;
///    VkBool32           vulkanMemoryModelAvailabilityVisibilityChains;
///    VkBool32           shaderOutputViewportIndex;
///    VkBool32           shaderOutputLayer;
///    VkBool32           subgroupBroadcastDynamicId;
///} VkPhysicalDeviceVulkan12Features;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`sampler_mirror_clamp_to_edge`] indicates whether the implementation supports the
///   `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` sampler address mode. If this feature is not
///   enabled, the `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` sampler address mode **must** not
///   be used.
/// - [`draw_indirect_count`] indicates whether the implementation supports the
///   [`CmdDrawIndirectCount`] and [`CmdDrawIndexedIndirectCount`] functions. If this feature is not
///   enabled, these functions **must** not be used.
/// - [`storage_buffer_8_bit_access`] indicates whether objects in the     `StorageBuffer`,
///   `ShaderRecordBufferKHR`,     or `PhysicalStorageBuffer`     storage class with the `Block`
///   decoration **can** have 8-bit integer     members.     If this feature is not enabled, 8-bit
///   integer members **must** not be used     in such objects.     This also indicates whether
///   shader modules **can** declare the     `StorageBuffer8BitAccess` capability.
/// - [`uniform_and_storage_buffer_8_bit_access`] indicates whether objects in the `Uniform` storage
///   class with the `Block` decoration **can** have 8-bit integer members. If this feature is not
///   enabled, 8-bit integer members **must** not be used in such objects. This also indicates
///   whether shader modules **can** declare the `UniformAndStorageBuffer8BitAccess` capability.
/// - [`storage_push_constant_8`] indicates whether objects in the `PushConstant` storage class
///   **can** have 8-bit integer members. If this feature is not enabled, 8-bit integer members
///   **must** not be used in such objects. This also indicates whether shader modules **can**
///   declare the `StoragePushConstant8` capability.
/// - [`shader_buffer_int_64_atomics`] indicates whether shaders **can** perform 64-bit unsigned and
///   signed integer atomic operations on buffers.
/// - [`shader_shared_int_64_atomics`] indicates whether shaders **can** perform 64-bit unsigned and
///   signed integer atomic operations on shared memory.
/// - [`shader_float_16`] indicates whether 16-bit floats (halfs) are supported in shader code. This
///   also indicates whether shader modules **can** declare the `Float16` capability. However, this
///   only enables a subset of the storage classes that SPIR-V allows for the `Float16` SPIR-V
///   capability: Declaring and using 16-bit floats in the `Private`, `Workgroup` (for non-Block
///   variables), and `Function` storage classes is enabled, while declaring them in the interface
///   storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`, `Output`, and
///   `PushConstant`) is not enabled.
/// - [`shader_int_8`] indicates whether 8-bit integers (signed and unsigned) are supported in
///   shader code. This also indicates whether shader modules **can** declare the `Int8` capability.
///   However, this only enables a subset of the storage classes that SPIR-V allows for the `Int8`
///   SPIR-V capability: Declaring and using 8-bit integers in the `Private`, `Workgroup` (for
///   non-Block variables), and `Function` storage classes is enabled, while declaring them in the
///   interface storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`,
///   `Output`, and `PushConstant`) is not enabled.
/// - [`descriptor_indexing`] indicates whether the implementation supports the minimum set of descriptor indexing features as described in the [Feature Requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-requirements) section. Enabling the [`descriptor_indexing`] member when [`CreateDevice`] is called does not imply the other minimum descriptor indexing features are also enabled. Those other descriptor indexing features **must** be enabled individually as needed by the application.
/// - [`shader_input_attachment_array_dynamic_indexing`] indicates whether arrays of input
///   attachments **can** be indexed by dynamically uniform integer expressions in shader code. If
///   this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`**must** be indexed only by constant integral expressions
///   when aggregated into arrays in shader code. This also indicates whether shader modules **can**
///   declare the `InputAttachmentArrayDynamicIndexing` capability.
/// - [`shader_uniform_texel_buffer_array_dynamic_indexing`] indicates whether arrays of uniform
///   texel buffers **can** be indexed by dynamically uniform integer expressions in shader code. If
///   this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`**must** be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `UniformTexelBufferArrayDynamicIndexing` capability.
/// - [`shader_storage_texel_buffer_array_dynamic_indexing`] indicates whether arrays of storage
///   texel buffers **can** be indexed by dynamically uniform integer expressions in shader code. If
///   this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`**must** be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `StorageTexelBufferArrayDynamicIndexing` capability.
/// - [`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform
///   buffers **can** be indexed by non-uniform integer expressions in shader code. If this feature
///   is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `UniformBufferArrayNonUniformIndexing` capability.
/// - [`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays of samplers or
///   sampled images **can** be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_SAMPLER`,
///   `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`**must** not
///   be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This
///   also indicates whether shader modules **can** declare the
///   `SampledImageArrayNonUniformIndexing` capability.
/// - [`shader_storage_buffer_array_non_uniform_indexing`] indicates whether arrays of storage
///   buffers **can** be indexed by non-uniform integer expressions in shader code. If this feature
///   is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `StorageBufferArrayNonUniformIndexing` capability.
/// - [`shader_storage_image_array_non_uniform_indexing`] indicates whether arrays of storage images
///   **can** be indexed by non-uniform integer expressions in shader code. If this feature is not
///   enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`**must** not be
///   indexed by non-uniform integer expressions when aggregated into arrays in shader code. This
///   also indicates whether shader modules **can** declare the
///   `StorageImageArrayNonUniformIndexing` capability.
/// - [`shader_input_attachment_array_non_uniform_indexing`] indicates whether arrays of input
///   attachments **can** be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `InputAttachmentArrayNonUniformIndexing` capability.
/// - [`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform
///   texel buffers **can** be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `UniformTexelBufferArrayNonUniformIndexing` capability.
/// - [`shader_storage_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of storage
///   texel buffers **can** be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`**must** not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules **can** declare the `StorageTexelBufferArrayNonUniformIndexing` capability.
/// - [`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether the implementation
///   supports updating uniform buffer descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.
/// - [`descriptor_binding_sampled_image_update_after_bind`] indicates whether the implementation
///   supports updating sampled image descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
///   `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.
/// - [`descriptor_binding_storage_image_update_after_bind`] indicates whether the implementation
///   supports updating storage image descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
///   `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
/// - [`descriptor_binding_storage_buffer_update_after_bind`] indicates whether the implementation
///   supports updating storage buffer descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`.
/// - [`descriptor_binding_uniform_texel_buffer_update_after_bind`] indicates whether the
///   implementation supports updating uniform texel buffer descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used
///   with `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
/// - [`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates whether the
///   implementation supports updating storage texel buffer descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used
///   with `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
/// - [`descriptor_binding_update_unused_while_pending`] indicates whether the implementation
///   supports updating descriptors while the set is in use. If this feature is not enabled,
///   `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`**must** not be used.
/// - [`descriptor_binding_partially_bound`] indicates whether the implementation supports
///   statically using a descriptor set binding in which some descriptors are not valid. If this
///   feature is not enabled, `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT`**must** not be used.
/// - [`descriptor_binding_variable_descriptor_count`] indicates whether the implementation supports
///   descriptor sets with a variable-sized last binding. If this feature is not enabled,
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`**must** not be used.
/// - [`runtime_descriptor_array`] indicates whether the implementation supports the SPIR-V
///   `RuntimeDescriptorArray` capability. If this feature is not enabled, descriptors **must** not
///   be declared in runtime arrays.
/// - [`sampler_filter_minmax`] indicates whether the implementation supports a minimum set of required formats supporting min/max filtering as defined by the [`filterMinmaxSingleComponentFormats`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-filterMinmaxSingleComponentFormats-minimum-requirements) property minimum requirements. If this feature is not enabled, then no [`SamplerCreateInfo`][`p_next`] chain can include a [`SamplerReductionModeCreateInfo`] structure.
/// - [`scalar_block_layout`] indicates that the implementation supports the layout of resource
///   blocks in shaders using [scalar alignment]().
/// - [`imageless_framebuffer`] indicates that the implementation supports specifying the image view
///   for attachments at render pass begin time via [`RenderPassAttachmentBeginInfo`].
/// - [`uniform_buffer_standard_layout`] indicates that the implementation supports the same layouts
///   for uniform buffers as for storage and other kinds of buffers. See [Standard Buffer Layout]().
/// - [`shader_subgroup_extended_types`] is a boolean specifying whether subgroup operations can use
///   8-bit integer, 16-bit integer, 64-bit integer, 16-bit floating-point, and vectors of these
///   types in [group operations]() with [subgroup scope](), if the implementation supports the
///   types.
/// - [`separate_depth_stencil_layouts`] indicates whether the implementation supports a
///   [`ImageMemoryBarrier`] for a depth/stencil image with only one of `VK_IMAGE_ASPECT_DEPTH_BIT`
///   or `VK_IMAGE_ASPECT_STENCIL_BIT` set, and whether `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` can be used.
/// - [`host_query_reset`] indicates that the implementation supports resetting queries from the
///   host with [`ResetQueryPool`].
/// - [`timeline_semaphore`] indicates whether semaphores created with a [`SemaphoreType`] of
///   `VK_SEMAPHORE_TYPE_TIMELINE` are supported.
/// - [`buffer_device_address`] indicates that the implementation supports accessing buffer memory
///   in shaders as storage buffers via an address queried from [`GetBufferDeviceAddress`].
/// - [`buffer_device_address_capture_replay`] indicates that the implementation supports saving and
///   reusing buffer and device addresses, e.g. for trace capture and replay.
/// - [`buffer_device_address_multi_device`] indicates that the implementation supports the
///   [`buffer_device_address`] , `rayTracingPipeline` and `rayQuery` features for logical devices
///   created with multiple physical devices. If this feature is not supported, buffer and
///   acceleration structure addresses **must** not be queried on a logical device created with more
///   than one physical device.
/// - [`vulkan_memory_model`] indicates whether the Vulkan Memory Model is supported, as defined in
///   [Vulkan Memory Model](). This also indicates whether shader modules **can** declare the
///   `VulkanMemoryModel` capability.
/// - [`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory Model can use
///   [`Device`] scope synchronization. This also indicates whether shader modules **can** declare
///   the `VulkanMemoryModelDeviceScope` capability.
/// - [`vulkan_memory_model_availability_visibility_chains`] indicates whether the Vulkan Memory
///   Model can use [availability and visibility chains]() with more than one element.
/// - [`shader_output_viewport_index`] indicates whether the implementation supports the
///   `ShaderViewportIndex` SPIR-V capability enabling variables decorated with the `ViewportIndex`
///   built-in to be exported from vertex or tessellation evaluation shaders. If this feature is not
///   enabled, the `ViewportIndex` built-in decoration **must** not be used on outputs in vertex or
///   tessellation evaluation shaders.
/// - [`shader_output_layer`] indicates whether the implementation supports the `ShaderLayer` SPIR-V
///   capability enabling variables decorated with the `Layer` built-in to be exported from vertex
///   or tessellation evaluation shaders. If this feature is not enabled, the `Layer` built-in
///   decoration **must** not be used on outputs in vertex or tessellation evaluation shaders.
/// - If [`subgroup_broadcast_dynamic_id`] is [`TRUE`], the “Id” operand of
///   `OpGroupNonUniformBroadcast`**can** be dynamically uniform within a subgroup, and the “Index”
///   operand of `OpGroupNonUniformQuadBroadcast`**can** be dynamically uniform within the
///   derivative group. If it is [`FALSE`], these operands **must** be constants.
///If the [`PhysicalDeviceVulkan12Features`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVulkan12Features`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES`
///# Related
/// - [`crate::vulkan1_2`]
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
pub struct PhysicalDeviceVulkan12Features<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`sampler_mirror_clamp_to_edge`]
    ///indicates whether the implementation supports the
    ///`VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` sampler address mode.
    ///If this feature is not enabled, the
    ///`VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` sampler address mode
    ///**must** not be used.
    sampler_mirror_clamp_to_edge: Bool32,
    ///[`draw_indirect_count`] indicates whether
    ///the implementation supports the [`CmdDrawIndirectCount`] and
    ///[`CmdDrawIndexedIndirectCount`] functions.
    ///If this feature is not enabled, these functions **must** not be used.
    draw_indirect_count: Bool32,
    ///[`storage_buffer_8_bit_access`] indicates whether objects in the
    ///    `StorageBuffer`,
    ///`ShaderRecordBufferKHR`,
    ///    or `PhysicalStorageBuffer`
    ///    storage class with the `Block` decoration **can** have 8-bit integer
    ///    members.
    ///    If this feature is not enabled, 8-bit integer members **must** not be used
    ///    in such objects.
    ///    This also indicates whether shader modules **can** declare the
    ///    `StorageBuffer8BitAccess` capability.
    storage_buffer_8_bit_access: Bool32,
    ///[`uniform_and_storage_buffer_8_bit_access`] indicates whether objects in the
    ///`Uniform` storage class with the `Block` decoration **can** have
    ///8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members **must** not be used
    ///in such objects.
    ///This also indicates whether shader modules **can** declare the
    ///`UniformAndStorageBuffer8BitAccess` capability.
    uniform_and_storage_buffer_8_bit_access: Bool32,
    ///[`storage_push_constant_8`] indicates whether objects in the
    ///`PushConstant` storage class **can** have 8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members **must** not be used
    ///in such objects.
    ///This also indicates whether shader modules **can** declare the
    ///`StoragePushConstant8` capability.
    storage_push_constant_8: Bool32,
    ///[`shader_buffer_int_64_atomics`] indicates whether shaders **can** perform
    ///64-bit unsigned and signed integer atomic operations on buffers.
    shader_buffer_int_64_atomics: Bool32,
    ///[`shader_shared_int_64_atomics`] indicates whether shaders **can** perform
    ///64-bit unsigned and signed integer atomic operations on shared memory.
    shader_shared_int_64_atomics: Bool32,
    ///[`shader_float_16`] indicates
    ///whether 16-bit floats (halfs) are supported in shader code.
    ///This also indicates whether shader modules **can** declare the `Float16`
    ///capability.
    ///However, this only enables a subset of the storage classes that SPIR-V
    ///allows for the `Float16` SPIR-V capability: Declaring and using
    ///16-bit floats in the `Private`,
    ///`Workgroup` (for non-Block variables),
    ///and `Function` storage classes is enabled, while declaring them in
    ///the interface storage classes (e.g., `UniformConstant`, `Uniform`,
    ///`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
    ///not enabled.
    shader_float_16: Bool32,
    ///[`shader_int_8`] indicates
    ///whether 8-bit integers (signed and unsigned) are supported in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the `Int8`
    ///capability.
    ///However, this only enables a subset of the storage classes that SPIR-V
    ///allows for the `Int8` SPIR-V capability: Declaring and using 8-bit
    ///integers in the `Private`,
    ///`Workgroup` (for non-Block variables),
    ///and `Function` storage classes is enabled, while declaring them in
    ///the interface storage classes (e.g., `UniformConstant`, `Uniform`,
    ///`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
    ///not enabled.
    shader_int_8: Bool32,
    ///[`descriptor_indexing`] indicates
    ///whether the implementation supports the minimum set of descriptor
    ///indexing features as described in the [Feature
    ///Requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-requirements) section.
    ///Enabling the [`descriptor_indexing`] member when [`CreateDevice`]
    ///is called does not imply the other minimum descriptor indexing features
    ///are also enabled.
    ///Those other descriptor indexing features **must** be enabled individually
    ///as needed by the application.
    descriptor_indexing: Bool32,
    ///[`shader_input_attachment_array_dynamic_indexing`] indicates whether arrays
    ///of input attachments **can** be indexed by dynamically uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`**must** be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`InputAttachmentArrayDynamicIndexing` capability.
    shader_input_attachment_array_dynamic_indexing: Bool32,
    ///[`shader_uniform_texel_buffer_array_dynamic_indexing`] indicates whether
    ///arrays of uniform texel buffers **can** be indexed by dynamically uniform
    ///integer expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`**must** be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`UniformTexelBufferArrayDynamicIndexing` capability.
    shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    ///[`shader_storage_texel_buffer_array_dynamic_indexing`] indicates whether
    ///arrays of storage texel buffers **can** be indexed by dynamically uniform
    ///integer expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`**must** be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`StorageTexelBufferArrayDynamicIndexing` capability.
    shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    ///[`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of uniform buffers **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`UniformBufferArrayNonUniformIndexing` capability.
    shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays
    ///of samplers or sampled images **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_SAMPLER`,
    ///`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
    ///`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`SampledImageArrayNonUniformIndexing` capability.
    shader_sampled_image_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of storage buffers **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`StorageBufferArrayNonUniformIndexing` capability.
    shader_storage_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_image_array_non_uniform_indexing`] indicates whether arrays
    ///of storage images **can** be indexed by non-uniform integer expressions in
    ///shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`StorageImageArrayNonUniformIndexing` capability.
    shader_storage_image_array_non_uniform_indexing: Bool32,
    ///[`shader_input_attachment_array_non_uniform_indexing`] indicates whether
    ///arrays of input attachments **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`InputAttachmentArrayNonUniformIndexing` capability.
    shader_input_attachment_array_non_uniform_indexing: Bool32,
    ///[`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of uniform texel buffers **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`UniformTexelBufferArrayNonUniformIndexing` capability.
    shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_texel_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of storage texel buffers **can** be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`**must** not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules **can** declare the
    ///`StorageTexelBufferArrayNonUniformIndexing` capability.
    shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    ///[`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether
    ///the implementation supports updating uniform buffer descriptors after a
    ///set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.
    descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_sampled_image_update_after_bind`] indicates whether the
    ///implementation supports updating sampled image descriptors after a set
    ///is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_SAMPLER`,
    ///`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
    ///`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.
    descriptor_binding_sampled_image_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_image_update_after_bind`] indicates whether the
    ///implementation supports updating storage image descriptors after a set
    ///is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
    descriptor_binding_storage_image_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_buffer_update_after_bind`] indicates whether
    ///the implementation supports updating storage buffer descriptors after a
    ///set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`.
    descriptor_binding_storage_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_uniform_texel_buffer_update_after_bind`] indicates
    ///whether the implementation supports updating uniform texel buffer
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
    descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates
    ///whether the implementation supports updating storage texel buffer
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`**must** not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
    descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_update_unused_while_pending`] indicates whether the
    ///implementation supports updating descriptors while the set is in use.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`**must** not be
    ///used.
    descriptor_binding_update_unused_while_pending: Bool32,
    ///[`descriptor_binding_partially_bound`] indicates whether the
    ///implementation supports statically using a descriptor set binding in
    ///which some descriptors are not valid.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT`**must** not be used.
    descriptor_binding_partially_bound: Bool32,
    ///[`descriptor_binding_variable_descriptor_count`] indicates whether the
    ///implementation supports descriptor sets with a variable-sized last
    ///binding.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`**must** not be
    ///used.
    descriptor_binding_variable_descriptor_count: Bool32,
    ///[`runtime_descriptor_array`] indicates whether the implementation
    ///supports the SPIR-V `RuntimeDescriptorArray` capability.
    ///If this feature is not enabled, descriptors **must** not be declared in
    ///runtime arrays.
    runtime_descriptor_array: Bool32,
    ///[`sampler_filter_minmax`] indicates
    ///whether the implementation supports a minimum set of required formats
    ///supporting min/max filtering as defined by the
    ///[`filterMinmaxSingleComponentFormats`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-filterMinmaxSingleComponentFormats-minimum-requirements)
    ///property minimum requirements.
    ///If this feature is not enabled, then no [`SamplerCreateInfo`][`p_next`] chain can include a
    /// [`SamplerReductionModeCreateInfo`] structure.
    sampler_filter_minmax: Bool32,
    ///[`scalar_block_layout`]
    ///indicates that the implementation supports the layout of resource blocks
    ///in shaders using [scalar
    ///alignment]().
    scalar_block_layout: Bool32,
    ///[`imageless_framebuffer`] indicates that the implementation supports
    ///specifying the image view for attachments at render pass begin time via
    ///[`RenderPassAttachmentBeginInfo`].
    imageless_framebuffer: Bool32,
    ///[`uniform_buffer_standard_layout`] indicates that the implementation
    ///supports the same layouts for uniform buffers as for storage and other
    ///kinds of buffers.
    ///See [Standard Buffer Layout]().
    uniform_buffer_standard_layout: Bool32,
    ///[`shader_subgroup_extended_types`] is a boolean specifying whether
    ///subgroup operations can use 8-bit integer, 16-bit integer, 64-bit
    ///integer, 16-bit floating-point, and vectors of these types in
    ///[group operations]() with
    ///[subgroup scope](), if the implementation
    ///supports the types.
    shader_subgroup_extended_types: Bool32,
    ///[`separate_depth_stencil_layouts`] indicates whether the implementation
    ///supports a [`ImageMemoryBarrier`] for a depth/stencil image with
    ///only one of `VK_IMAGE_ASPECT_DEPTH_BIT` or
    ///`VK_IMAGE_ASPECT_STENCIL_BIT` set, and whether
    ///`VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
    ///`VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
    ///`VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or
    ///`VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` can be used.
    separate_depth_stencil_layouts: Bool32,
    ///[`host_query_reset`]
    ///indicates that the implementation supports resetting queries from the
    ///host with [`ResetQueryPool`].
    host_query_reset: Bool32,
    ///[`timeline_semaphore`]
    ///indicates whether semaphores created with a [`SemaphoreType`] of
    ///`VK_SEMAPHORE_TYPE_TIMELINE` are supported.
    timeline_semaphore: Bool32,
    ///[`buffer_device_address`] indicates that the implementation supports
    ///accessing buffer memory in shaders as storage buffers via an address
    ///queried from [`GetBufferDeviceAddress`].
    buffer_device_address: Bool32,
    ///[`buffer_device_address_capture_replay`] indicates that the implementation
    ///supports saving and reusing buffer and device addresses, e.g. for trace
    ///capture and replay.
    buffer_device_address_capture_replay: Bool32,
    ///[`buffer_device_address_multi_device`] indicates that the implementation
    ///supports the [`buffer_device_address`]
    ///, `rayTracingPipeline` and `rayQuery` features
    ///for logical devices created with multiple physical devices.
    ///If this feature is not supported, buffer
    ///and acceleration structure
    ///addresses **must** not be queried on a logical device created with more
    ///than one physical device.
    buffer_device_address_multi_device: Bool32,
    ///[`vulkan_memory_model`]
    ///indicates whether the Vulkan Memory Model is supported, as defined in
    ///[Vulkan Memory Model]().
    ///This also indicates whether shader modules **can** declare the
    ///`VulkanMemoryModel` capability.
    vulkan_memory_model: Bool32,
    ///[`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory
    ///Model can use [`Device`] scope synchronization.
    ///This also indicates whether shader modules **can** declare the
    ///`VulkanMemoryModelDeviceScope` capability.
    vulkan_memory_model_device_scope: Bool32,
    ///[`vulkan_memory_model_availability_visibility_chains`] indicates whether
    ///the Vulkan Memory Model can use [availability and visibility chains]() with more than one
    /// element.
    vulkan_memory_model_availability_visibility_chains: Bool32,
    ///[`shader_output_viewport_index`]
    ///indicates whether the implementation supports the
    ///`ShaderViewportIndex` SPIR-V capability enabling variables decorated
    ///with the `ViewportIndex` built-in to be exported from vertex or
    ///tessellation evaluation shaders.
    ///If this feature is not enabled, the `ViewportIndex` built-in
    ///decoration **must** not be used on outputs in vertex or tessellation
    ///evaluation shaders.
    shader_output_viewport_index: Bool32,
    ///[`shader_output_layer`] indicates whether
    ///the implementation supports the `ShaderLayer` SPIR-V capability
    ///enabling variables decorated with the `Layer` built-in to be exported
    ///from vertex or tessellation evaluation shaders.
    ///If this feature is not enabled, the `Layer` built-in decoration **must**
    ///not be used on outputs in vertex or tessellation evaluation shaders.
    shader_output_layer: Bool32,
    ///If
    ///[`subgroup_broadcast_dynamic_id`] is [`TRUE`], the “Id” operand of
    ///`OpGroupNonUniformBroadcast`**can** be dynamically uniform within a
    ///subgroup, and the “Index” operand of
    ///`OpGroupNonUniformQuadBroadcast`**can** be dynamically uniform within
    ///the derivative group.
    ///If it is [`FALSE`], these operands **must** be constants.
    subgroup_broadcast_dynamic_id: Bool32,
}
///[VkPhysicalDeviceVulkan12Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVulkan12Properties.html) - Structure specifying physical device properties for functionality promoted to Vulkan 1.2
///# C Specifications
///The [`PhysicalDeviceVulkan12Properties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_2
///typedef struct VkPhysicalDeviceVulkan12Properties {
///    VkStructureType                      sType;
///    void*                                pNext;
///    VkDriverId                           driverID;
///    char                                 driverName[VK_MAX_DRIVER_NAME_SIZE];
///    char                                 driverInfo[VK_MAX_DRIVER_INFO_SIZE];
///    VkConformanceVersion                 conformanceVersion;
///    VkShaderFloatControlsIndependence    denormBehaviorIndependence;
///    VkShaderFloatControlsIndependence    roundingModeIndependence;
///    VkBool32                             shaderSignedZeroInfNanPreserveFloat16;
///    VkBool32                             shaderSignedZeroInfNanPreserveFloat32;
///    VkBool32                             shaderSignedZeroInfNanPreserveFloat64;
///    VkBool32                             shaderDenormPreserveFloat16;
///    VkBool32                             shaderDenormPreserveFloat32;
///    VkBool32                             shaderDenormPreserveFloat64;
///    VkBool32                             shaderDenormFlushToZeroFloat16;
///    VkBool32                             shaderDenormFlushToZeroFloat32;
///    VkBool32                             shaderDenormFlushToZeroFloat64;
///    VkBool32                             shaderRoundingModeRTEFloat16;
///    VkBool32                             shaderRoundingModeRTEFloat32;
///    VkBool32                             shaderRoundingModeRTEFloat64;
///    VkBool32                             shaderRoundingModeRTZFloat16;
///    VkBool32                             shaderRoundingModeRTZFloat32;
///    VkBool32                             shaderRoundingModeRTZFloat64;
///    uint32_t                             maxUpdateAfterBindDescriptorsInAllPools;
///    VkBool32                             shaderUniformBufferArrayNonUniformIndexingNative;
///    VkBool32                             shaderSampledImageArrayNonUniformIndexingNative;
///    VkBool32                             shaderStorageBufferArrayNonUniformIndexingNative;
///    VkBool32                             shaderStorageImageArrayNonUniformIndexingNative;
///    VkBool32                             shaderInputAttachmentArrayNonUniformIndexingNative;
///    VkBool32                             robustBufferAccessUpdateAfterBind;
///    VkBool32                             quadDivergentImplicitLod;
///    uint32_t                             maxPerStageDescriptorUpdateAfterBindSamplers;
///    uint32_t                             maxPerStageDescriptorUpdateAfterBindUniformBuffers;
///    uint32_t                             maxPerStageDescriptorUpdateAfterBindStorageBuffers;
///    uint32_t                             maxPerStageDescriptorUpdateAfterBindSampledImages;
///    uint32_t                             maxPerStageDescriptorUpdateAfterBindStorageImages;
///    uint32_t                             maxPerStageDescriptorUpdateAfterBindInputAttachments;
///    uint32_t                             maxPerStageUpdateAfterBindResources;
///    uint32_t                             maxDescriptorSetUpdateAfterBindSamplers;
///    uint32_t                             maxDescriptorSetUpdateAfterBindUniformBuffers;
///    uint32_t                             maxDescriptorSetUpdateAfterBindUniformBuffersDynamic;
///    uint32_t                             maxDescriptorSetUpdateAfterBindStorageBuffers;
///    uint32_t                             maxDescriptorSetUpdateAfterBindStorageBuffersDynamic;
///    uint32_t                             maxDescriptorSetUpdateAfterBindSampledImages;
///    uint32_t                             maxDescriptorSetUpdateAfterBindStorageImages;
///    uint32_t                             maxDescriptorSetUpdateAfterBindInputAttachments;
///    VkResolveModeFlags                   supportedDepthResolveModes;
///    VkResolveModeFlags                   supportedStencilResolveModes;
///    VkBool32                             independentResolveNone;
///    VkBool32                             independentResolve;
///    VkBool32                             filterMinmaxSingleComponentFormats;
///    VkBool32                             filterMinmaxImageComponentMapping;
///    uint64_t                             maxTimelineSemaphoreValueDifference;
///    VkSampleCountFlags                   framebufferIntegerColorSampleCounts;
///} VkPhysicalDeviceVulkan12Properties;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`driver_id`] is a unique identifier for the driver of the physical device.
/// - [`driver_name`] is an array of [`MAX_DRIVER_NAME_SIZE`]`char` containing a null-terminated
///   UTF-8 string which is the name of the driver.
/// - [`driver_info`] is an array of [`MAX_DRIVER_INFO_SIZE`]`char` containing a null-terminated
///   UTF-8 string with additional information about the driver.
/// - [`conformance_version`] is the version of the Vulkan conformance test this driver is
///   conformant against (see [`ConformanceVersion`]).
/// - [`denorm_behavior_independence`] is a [`ShaderFloatControlsIndependence`] value indicating
///   whether, and how, denorm behavior can be set independently for different bit widths.
/// - [`rounding_mode_independence`] is a [`ShaderFloatControlsIndependence`] value indicating
///   whether, and how, rounding modes can be set independently for different bit widths.
/// - [`shader_signed_zero_inf_nan_preserve_float_16`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span class="katex-html" aria-hidden="true"><span
///   class="base"><span style="height:0.66666em;vertical-align:-0.08333em;"
///   class="strut"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span>**can** be preserved in 16-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode **can**
///   be used for 16-bit floating-point types.
/// - [`shader_signed_zero_inf_nan_preserve_float_32`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span aria-hidden="true" class="katex-html"><span
///   class="base"><span class="strut"
///   style="height:0.66666em;vertical-align:-0.08333em;"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span>**can** be preserved in 32-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode **can**
///   be used for 32-bit floating-point types.
/// - [`shader_signed_zero_inf_nan_preserve_float_64`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span class="katex-html" aria-hidden="true"><span
///   class="base"><span class="strut"
///   style="height:0.66666em;vertical-align:-0.08333em;"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span>**can** be preserved in 64-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode **can**
///   be used for 64-bit floating-point types.
/// - [`shader_denorm_preserve_float_16`] is a boolean value indicating whether denormals **can** be
///   preserved in 16-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode **can** be used for 16-bit floating-point types.
/// - [`shader_denorm_preserve_float_32`] is a boolean value indicating whether denormals **can** be
///   preserved in 32-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode **can** be used for 32-bit floating-point types.
/// - [`shader_denorm_preserve_float_64`] is a boolean value indicating whether denormals **can** be
///   preserved in 64-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode **can** be used for 64-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_16`] is a boolean value indicating whether denormals
///   **can** be flushed to zero in 16-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode **can** be used for 16-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_32`] is a boolean value indicating whether denormals
///   **can** be flushed to zero in 32-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode **can** be used for 32-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_64`] is a boolean value indicating whether denormals
///   **can** be flushed to zero in 64-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode **can** be used for 64-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_16`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 16-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can** be used for 16-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_32`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 32-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can** be used for 32-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_64`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 64-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can** be used for 64-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_16`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 16-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can** be used for 16-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_32`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 32-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can** be used for 32-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_64`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 64-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can** be used for 64-bit floating-point types.
/// - [`max_update_after_bind_descriptors_in_all_pools`] is the maximum number of descriptors
///   (summed over all descriptor types) that **can** be created across all pools that are created
///   with the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` bit set. Pool creation **may** fail
///   when this limit is exceeded, or when the space this limit represents is unable to satisfy a
///   pool creation due to fragmentation.
/// - [`shader_uniform_buffer_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether uniform buffer descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of uniform
///   buffers **may** execute multiple times in order to access all the descriptors.
/// - [`shader_sampled_image_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether sampler and image descriptors natively support nonuniform indexing. If this is
///   [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array
///   of samplers or images **may** execute multiple times in order to access all the descriptors.
/// - [`shader_storage_buffer_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether storage buffer descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of storage
///   buffers **may** execute multiple times in order to access all the descriptors.
/// - [`shader_storage_image_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether storage image descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of storage
///   images **may** execute multiple times in order to access all the descriptors.
/// - [`shader_input_attachment_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether input attachment descriptors natively support nonuniform indexing. If this is
///   [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array
///   of input attachments **may** execute multiple times in order to access all the descriptors.
/// - [`robust_buffer_access_update_after_bind`] is a boolean value indicating whether
///   [`robustBufferAccess`]()**can** be enabled in a device simultaneously with
///   `descriptorBindingUniformBufferUpdateAfterBind`,
///   `descriptorBindingStorageBufferUpdateAfterBind`,
///   `descriptorBindingUniformTexelBufferUpdateAfterBind`, and/or
///   `descriptorBindingStorageTexelBufferUpdateAfterBind`. If this is [`FALSE`], then either
///   `robustBufferAccess`**must** be disabled or all of these update-after-bind features **must**
///   be disabled.
/// - [`quad_divergent_implicit_lod`] is a boolean value indicating whether implicit level of detail
///   calculations for image operations have well-defined results when the image and/or sampler
///   objects used for the instruction are not uniform within a quad. See [Derivative Image
///   Operations]().
/// - [`max_per_stage_descriptor_update_after_bind_samplers`] is similar to
///   `maxPerStageDescriptorSamplers` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_uniform_buffers`] is similar to
///   `maxPerStageDescriptorUniformBuffers` but counts descriptors from descriptor sets created with
///   or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_storage_buffers`] is similar to
///   `maxPerStageDescriptorStorageBuffers` but counts descriptors from descriptor sets created with
///   or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_sampled_images`] is similar to
///   `maxPerStageDescriptorSampledImages` but counts descriptors from descriptor sets created with
///   or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_storage_images`] is similar to
///   `maxPerStageDescriptorStorageImages` but counts descriptors from descriptor sets created with
///   or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_descriptor_update_after_bind_input_attachments`] is similar to
///   `maxPerStageDescriptorInputAttachments` but counts descriptors from descriptor sets created
///   with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_per_stage_update_after_bind_resources`] is similar to `maxPerStageResources` but counts
///   descriptors from descriptor sets created with or without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_samplers`] is similar to `maxDescriptorSetSamplers` but
///   counts descriptors from descriptor sets created with or without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_uniform_buffers`] is similar to
///   `maxDescriptorSetUniformBuffers` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_uniform_buffers_dynamic`] is similar to
///   `maxDescriptorSetUniformBuffersDynamic` but counts descriptors from descriptor sets created
///   with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
///   While an application **can** allocate dynamic uniform buffer descriptors from a pool created
///   with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these
///   descriptors **must** not be present in any descriptor set layout that includes bindings
///   created with `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`.
/// - [`max_descriptor_set_update_after_bind_storage_buffers`] is similar to
///   `maxDescriptorSetStorageBuffers` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_storage_buffers_dynamic`] is similar to
///   `maxDescriptorSetStorageBuffersDynamic` but counts descriptors from descriptor sets created
///   with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
///   While an application **can** allocate dynamic storage buffer descriptors from a pool created
///   with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these
///   descriptors **must** not be present in any descriptor set layout that includes bindings
///   created with `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`.
/// - [`max_descriptor_set_update_after_bind_sampled_images`] is similar to
///   `maxDescriptorSetSampledImages` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_storage_images`] is similar to
///   `maxDescriptorSetStorageImages` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_input_attachments`] is similar to
///   `maxDescriptorSetInputAttachments` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`supported_depth_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set
///   of supported depth resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT`**must** be included in the
///   set but implementations **may** support additional modes.
/// - [`supported_stencil_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set
///   of supported stencil resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT`**must** be included in
///   the set but implementations **may** support additional modes.
///   `VK_RESOLVE_MODE_AVERAGE_BIT`**must** not be included in the set.
/// - [`independent_resolve_none`] is [`TRUE`] if the implementation supports setting the depth and
///   stencil resolve modes to different values when one of those modes is `VK_RESOLVE_MODE_NONE`.
///   Otherwise the implementation only supports setting both modes to the same value.
/// - [`independent_resolve`] is [`TRUE`] if the implementation supports all combinations of the
///   supported depth and stencil resolve modes, including setting either depth or stencil resolve
///   mode to `VK_RESOLVE_MODE_NONE`. An implementation that supports
///   [`independent_resolve`]**must** also support [`independent_resolve_none`].
/// - [`filter_minmax_single_component_formats`] is a boolean value indicating whether a minimum set
///   of required formats support min/max filtering.
/// - [`filter_minmax_image_component_mapping`] is a boolean value indicating whether the
///   implementation supports non-identity component mapping of the image when doing min/max
///   filtering.
/// - [`max_timeline_semaphore_value_difference`] indicates the maximum difference allowed by the
///   implementation between the current value of a timeline semaphore and any pending signal or
///   wait operations.
/// - [`framebuffer_integer_color_sample_counts`] is a bitmask of [`SampleCountFlagBits`] indicating
///   the color sample counts that are supported for all framebuffer color attachments with integer
///   formats.
///If the [`PhysicalDeviceVulkan12Properties`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These properties correspond to Vulkan 1.2
/// functionality.The members of [`PhysicalDeviceVulkan12Properties`]**must** have the same
///values as the corresponding members of
///[`PhysicalDeviceDriverProperties`],
///[`PhysicalDeviceFloatControlsProperties`],
///[`PhysicalDeviceDescriptorIndexingProperties`],
///[`PhysicalDeviceDepthStencilResolveProperties`],
///[`PhysicalDeviceSamplerFilterMinmaxProperties`], and
///[`PhysicalDeviceTimelineSemaphoreProperties`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES`
///# Related
/// - [`crate::vulkan1_2`]
/// - [`Bool32`]
/// - [`ConformanceVersion`]
/// - [`DriverId`]
/// - [`ResolveModeFlags`]
/// - [`SampleCountFlags`]
/// - [`ShaderFloatControlsIndependence`]
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
pub struct PhysicalDeviceVulkan12Properties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    driver_id: DriverId,
    ///No documentation found
    driver_name: [c_schar; MAX_DRIVER_NAME_SIZE],
    ///No documentation found
    driver_info: [c_schar; MAX_DRIVER_INFO_SIZE],
    ///No documentation found
    conformance_version: ConformanceVersion,
    ///No documentation found
    denorm_behavior_independence: ShaderFloatControlsIndependence,
    ///No documentation found
    rounding_mode_independence: ShaderFloatControlsIndependence,
    ///No documentation found
    shader_signed_zero_inf_nan_preserve_float_16: Bool32,
    ///No documentation found
    shader_signed_zero_inf_nan_preserve_float_32: Bool32,
    ///No documentation found
    shader_signed_zero_inf_nan_preserve_float_64: Bool32,
    ///No documentation found
    shader_denorm_preserve_float_16: Bool32,
    ///No documentation found
    shader_denorm_preserve_float_32: Bool32,
    ///No documentation found
    shader_denorm_preserve_float_64: Bool32,
    ///No documentation found
    shader_denorm_flush_to_zero_float_16: Bool32,
    ///No documentation found
    shader_denorm_flush_to_zero_float_32: Bool32,
    ///No documentation found
    shader_denorm_flush_to_zero_float_64: Bool32,
    ///No documentation found
    shader_rounding_mode_rte_float_16: Bool32,
    ///No documentation found
    shader_rounding_mode_rte_float_32: Bool32,
    ///No documentation found
    shader_rounding_mode_rte_float_64: Bool32,
    ///No documentation found
    shader_rounding_mode_rtz_float_16: Bool32,
    ///No documentation found
    shader_rounding_mode_rtz_float_32: Bool32,
    ///No documentation found
    shader_rounding_mode_rtz_float_64: Bool32,
    ///No documentation found
    max_update_after_bind_descriptors_in_all_pools: u32,
    ///No documentation found
    shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    shader_storage_image_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    robust_buffer_access_update_after_bind: Bool32,
    ///No documentation found
    quad_divergent_implicit_lod: Bool32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_samplers: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_storage_images: u32,
    ///No documentation found
    max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    ///No documentation found
    max_per_stage_update_after_bind_resources: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_samplers: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_uniform_buffers: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_storage_buffers: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_sampled_images: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_storage_images: u32,
    ///No documentation found
    max_descriptor_set_update_after_bind_input_attachments: u32,
    ///No documentation found
    supported_depth_resolve_modes: ResolveModeFlags,
    ///No documentation found
    supported_stencil_resolve_modes: ResolveModeFlags,
    ///No documentation found
    independent_resolve_none: Bool32,
    ///No documentation found
    independent_resolve: Bool32,
    ///No documentation found
    filter_minmax_single_component_formats: Bool32,
    ///No documentation found
    filter_minmax_image_component_mapping: Bool32,
    ///No documentation found
    max_timeline_semaphore_value_difference: u64,
    ///No documentation found
    framebuffer_integer_color_sample_counts: SampleCountFlags,
}
