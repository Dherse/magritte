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
use std::{
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
    os::raw::c_char,
};
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
#[non_exhaustive]
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
        Self::Binary
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
        *self as i32
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
#[non_exhaustive]
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
        Self::WeightedAverage
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
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkDriverId](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDriverId.html) - Khronos driver IDs
///# C Specifications
///Khronos driver IDs which  **may**  be returned in
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
#[non_exhaustive]
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
        Self::Empty
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
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkShaderFloatControlsIndependence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderFloatControlsIndependence.html) - Bitmask specifying whether, and how, shader float controls can be set separately
///# C Specifications
///Values which  **may**  be returned in the `denormBehaviorIndependence` and
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
/// - [`32BitOnly`] specifies that shader float controls for 32-bit floating point  **can**  be set
///   independently; other bit widths  **must**  be set identically to each other.
/// - [`All`] specifies that shader float controls for all bit widths  **can**  be set
///   independently.
/// - [`None`] specifies that shader float controls for all bit widths  **must**  be set
///   identically.
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
#[non_exhaustive]
#[repr(i32)]
pub enum ShaderFloatControlsIndependence {
    ///[`32BitOnly`] specifies that
    ///shader float controls for 32-bit floating point  **can**  be set
    ///independently; other bit widths  **must**  be set identically to each other.
    _32BitOnly = 0,
    ///[`All`] specifies that shader
    ///float controls for all bit widths  **can**  be set independently.
    All = 1,
    ///[`None`] specifies that shader
    ///float controls for all bit widths  **must**  be set identically.
    None = 2,
}
impl const Default for ShaderFloatControlsIndependence {
    fn default() -> Self {
        Self::_32BitOnly
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
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkSemaphoreWaitFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html) - Bitmask specifying additional parameters of a semaphore wait operation
///# C Specifications
///Bits which  **can**  be set in [`SemaphoreWaitInfo::flags`], specifying
///additional parameters of a semaphore wait operation, are:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkSemaphoreWaitFlagBits {
///    VK_SEMAPHORE_WAIT_ANY_BIT = 0x00000001,
///  // Provided by VK_KHR_timeline_semaphore
///    VK_SEMAPHORE_WAIT_ANY_BIT_KHR = VK_SEMAPHORE_WAIT_ANY_BIT,
///} VkSemaphoreWaitFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_timeline_semaphore
///typedef VkSemaphoreWaitFlagBits VkSemaphoreWaitFlagBitsKHR;
///```
///# Description
/// - [`SemaphoreWaitAny`] specifies that the semaphore wait condition is that at least one of the
///   semaphores in [`SemaphoreWaitInfo::semaphores`] has reached the value specified by the
///   corresponding element of [`SemaphoreWaitInfo::values`]. If [`SemaphoreWaitAny`] is not set,
///   the semaphore wait condition is that all of the semaphores in
///   [`SemaphoreWaitInfo::semaphores`] have reached the value specified by the corresponding
///   element of [`SemaphoreWaitInfo::values`].
///# Related
/// - [`VK_KHR_timeline_semaphore`]
/// - [`crate::vulkan1_2`]
/// - [`SemaphoreWaitFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSemaphoreWaitFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum SemaphoreWaitFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`SemaphoreWaitAny`] specifies that the semaphore wait
    ///condition is that at least one of the semaphores in
    ///[`SemaphoreWaitInfo`]::`pSemaphores` has reached the value
    ///specified by the corresponding element of
    ///[`SemaphoreWaitInfo`]::`pValues`.
    ///If [`SemaphoreWaitAny`] is not set, the semaphore wait
    ///condition is that all of the semaphores in
    ///[`SemaphoreWaitInfo`]::`pSemaphores` have reached the value
    ///specified by the corresponding element of
    ///[`SemaphoreWaitInfo`]::`pValues`.
    SemaphoreWaitAny = 1,
}
impl const Default for SemaphoreWaitFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl SemaphoreWaitFlagBits {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkDescriptorBindingFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html) - Bitmask specifying descriptor set layout binding properties
///# C Specifications
///Bits which  **can**  be set in each element of
///[`DescriptorSetLayoutBindingFlagsCreateInfo::binding_flags`],
///specifying options for the corresponding descriptor set layout binding, are:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkDescriptorBindingFlagBits {
///    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT = 0x00000001,
///    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT = 0x00000002,
///    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT = 0x00000004,
///    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT = 0x00000008,
///  // Provided by VK_EXT_descriptor_indexing
///    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT =
/// VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT,
///  // Provided by VK_EXT_descriptor_indexing
///    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT =
/// VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT,
///  // Provided by VK_EXT_descriptor_indexing
///    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT = VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT,
///  // Provided by VK_EXT_descriptor_indexing
///    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT =
/// VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT,
///} VkDescriptorBindingFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_descriptor_indexing
///typedef VkDescriptorBindingFlagBits VkDescriptorBindingFlagBitsEXT;
///```
///# Description
/// - [`DescriptorBindingUpdateAfterBind`] indicates that if descriptors in this binding are updated
///   between when the descriptor set is bound in a command buffer and when that command buffer is
///   submitted to a queue, then the submission will use the most recently set descriptors for this
///   binding and the updates do not invalidate the command buffer. Descriptor bindings created with
///   this flag are also partially exempt from the external synchronization requirement in
///   [`UpdateDescriptorSetWithTemplateKHR`] and [`UpdateDescriptorSets`]. Multiple descriptors with
///   this flag set  **can**  be updated concurrently in different threads, though the same
///   descriptor  **must**  not be updated concurrently by two threads. Descriptors with this flag
///   set  **can**  be updated concurrently with the set being bound to a command buffer in another
///   thread, but not concurrently with the set being reset or freed.
/// - [`DescriptorBindingPartiallyBound`] indicates that descriptors in this binding that are not
///   *dynamically used* need not contain valid descriptors at the time the descriptors are
///   consumed. A descriptor is dynamically used if any shader invocation executes an instruction
///   that performs any memory access using the descriptor.
/// - [`DescriptorBindingUpdateUnusedWhilePending`] indicates that descriptors in this binding
///   **can**  be updated after a command buffer has bound this descriptor set, or while a command
///   buffer that uses this descriptor set is pending execution, as long as the descriptors that are
///   updated are not used by those command buffers. If [`DescriptorBindingPartiallyBound`] is also
///   set, then descriptors  **can**  be updated as long as they are not dynamically used by any
///   shader invocations. If [`DescriptorBindingPartiallyBound`] is not set, then descriptors
///   **can**  be updated as long as they are not statically used by any shader invocations.
/// - [`DescriptorBindingVariableDescriptorCount`] indicates that     this is a *variable-sized descriptor binding* whose size will be     specified when a descriptor set is allocated using this layout.     The value of `descriptorCount` is treated as an upper bound on the     size of the binding.     This  **must**  only be used for the last binding in the descriptor set     layout (i.e. the binding with the largest value of `binding`).     For the purposes of counting against limits such as     `maxDescriptorSet`* and `maxPerStageDescriptor`*, the full value     of `descriptorCount` is     counted, except for descriptor bindings with a descriptor type of     `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.     In this case, `descriptorCount` specifies the upper bound on the     byte size of the binding; thus it counts against the [`maxInlineUniformBlockSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformBlockSize) and [`maxInlineUniformTotalSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformTotalSize) limits instead.
///# Related
/// - [`VK_EXT_descriptor_indexing`]
/// - [`crate::vulkan1_2`]
/// - [`DescriptorBindingFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDescriptorBindingFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum DescriptorBindingFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`DescriptorBindingUpdateAfterBind`] indicates that if
    ///descriptors in this binding are updated between when the descriptor set
    ///is bound in a command buffer and when that command buffer is submitted
    ///to a queue, then the submission will use the most recently set
    ///descriptors for this binding and the updates do not invalidate the
    ///command buffer.
    ///Descriptor bindings created with this flag are also partially exempt
    ///from the external synchronization requirement in
    ///[`UpdateDescriptorSetWithTemplateKHR`] and
    ///[`UpdateDescriptorSets`].
    ///Multiple descriptors with this flag set  **can**  be updated concurrently in
    ///different threads, though the same descriptor  **must**  not be updated
    ///concurrently by two threads.
    ///Descriptors with this flag set  **can**  be updated concurrently with the set
    ///being bound to a command buffer in another thread, but not concurrently
    ///with the set being reset or freed.
    DescriptorBindingUpdateAfterBind = 1,
    ///[`DescriptorBindingUpdateUnusedWhilePending`] indicates
    ///that descriptors in this binding  **can**  be updated after a command buffer
    ///has bound this descriptor set, or while a command buffer that uses this
    ///descriptor set is pending execution, as long as the descriptors that are
    ///updated are not used by those command buffers.
    ///If [`DescriptorBindingPartiallyBound`] is also set, then
    ///descriptors  **can**  be updated as long as they are not dynamically used by
    ///any shader invocations.
    ///If [`DescriptorBindingPartiallyBound`] is not set, then
    ///descriptors  **can**  be updated as long as they are not statically used by
    ///any shader invocations.
    DescriptorBindingUpdateUnusedWhilePending = 2,
    ///[`DescriptorBindingPartiallyBound`] indicates that
    ///descriptors in this binding that are not *dynamically used* need not
    ///contain valid descriptors at the time the descriptors are consumed.
    ///A descriptor is dynamically used if any shader invocation executes an
    ///instruction that performs any memory access using the descriptor.
    DescriptorBindingPartiallyBound = 4,
    ///[`DescriptorBindingVariableDescriptorCount`] indicates that
    ///    this is a *variable-sized descriptor binding* whose size will be
    ///    specified when a descriptor set is allocated using this layout.
    ///    The value of `descriptorCount` is treated as an upper bound on the
    ///    size of the binding.
    ///    This  **must**  only be used for the last binding in the descriptor set
    ///    layout (i.e. the binding with the largest value of `binding`).
    ///    For the purposes of counting against limits such as
    ///    `maxDescriptorSet`* and `maxPerStageDescriptor`*, the full value
    ///    of `descriptorCount` is
    ///    counted, except for descriptor bindings with a descriptor type of
    ///    `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
    ///    In this case, `descriptorCount` specifies the upper bound on the
    ///    byte size of the binding; thus it counts against the
    ///[`maxInlineUniformBlockSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformBlockSize) and [`maxInlineUniformTotalSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformTotalSize) limits
    ///instead.
    DescriptorBindingVariableDescriptorCount = 8,
}
impl const Default for DescriptorBindingFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl DescriptorBindingFlagBits {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkResolveModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html) - Bitmask indicating supported depth and stencil resolve modes
///# C Specifications
///Possible values of
///[`SubpassDescriptionDepthStencilResolve::depth_resolve_mode`] and
///`stencilResolveMode`, specifying the depth and stencil resolve modes,
///are:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkResolveModeFlagBits {
///    VK_RESOLVE_MODE_NONE = 0,
///    VK_RESOLVE_MODE_SAMPLE_ZERO_BIT = 0x00000001,
///    VK_RESOLVE_MODE_AVERAGE_BIT = 0x00000002,
///    VK_RESOLVE_MODE_MIN_BIT = 0x00000004,
///    VK_RESOLVE_MODE_MAX_BIT = 0x00000008,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_NONE_KHR = VK_RESOLVE_MODE_NONE,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR = VK_RESOLVE_MODE_SAMPLE_ZERO_BIT,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_AVERAGE_BIT_KHR = VK_RESOLVE_MODE_AVERAGE_BIT,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_MIN_BIT_KHR = VK_RESOLVE_MODE_MIN_BIT,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_MAX_BIT_KHR = VK_RESOLVE_MODE_MAX_BIT,
///} VkResolveModeFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_depth_stencil_resolve
///typedef VkResolveModeFlagBits VkResolveModeFlagBitsKHR;
///```
///# Description
/// - [`ResolveModeNone`] indicates that no resolve operation is done.
/// - [`ResolveModeSampleZero`] indicates that result of the resolve operation is equal to the value
///   of sample 0.
/// - [`ResolveModeAverage`] indicates that result of the resolve operation is the average of the
///   sample values.
/// - [`ResolveModeMin`] indicates that result of the resolve operation is the minimum of the sample
///   values.
/// - [`ResolveModeMax`] indicates that result of the resolve operation is the maximum of the sample
///   values.
///# Related
/// - [`VK_KHR_depth_stencil_resolve`]
/// - [`crate::vulkan1_2`]
/// - [`RenderingAttachmentInfo`]
/// - [`ResolveModeFlags`]
/// - [`SubpassDescriptionDepthStencilResolve`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkResolveModeFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum ResolveModeFlagBits {
    ///[`ResolveModeNone`] indicates that no resolve operation is done.
    ResolveModeNone = 0,
    ///[`ResolveModeSampleZero`] indicates that result of the
    ///resolve operation is equal to the value of sample 0.
    ResolveModeSampleZero = 1,
    ///[`ResolveModeAverage`] indicates that result of the resolve
    ///operation is the average of the sample values.
    ResolveModeAverage = 2,
    ///[`ResolveModeMin`] indicates that result of the resolve
    ///operation is the minimum of the sample values.
    ResolveModeMin = 4,
    ///[`ResolveModeMax`] indicates that result of the resolve
    ///operation is the maximum of the sample values.
    ResolveModeMax = 8,
}
impl const Default for ResolveModeFlagBits {
    fn default() -> Self {
        Self::ResolveModeNone
    }
}
impl ResolveModeFlagBits {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkSemaphoreWaitFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreWaitFlagBits.html) - Bitmask specifying additional parameters of a semaphore wait operation
///# C Specifications
///Bits which  **can**  be set in [`SemaphoreWaitInfo::flags`], specifying
///additional parameters of a semaphore wait operation, are:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkSemaphoreWaitFlagBits {
///    VK_SEMAPHORE_WAIT_ANY_BIT = 0x00000001,
///  // Provided by VK_KHR_timeline_semaphore
///    VK_SEMAPHORE_WAIT_ANY_BIT_KHR = VK_SEMAPHORE_WAIT_ANY_BIT,
///} VkSemaphoreWaitFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_timeline_semaphore
///typedef VkSemaphoreWaitFlagBits VkSemaphoreWaitFlagBitsKHR;
///```
///# Description
/// - [`SemaphoreWaitAny`] specifies that the semaphore wait condition is that at least one of the
///   semaphores in [`SemaphoreWaitInfo::semaphores`] has reached the value specified by the
///   corresponding element of [`SemaphoreWaitInfo::values`]. If [`SemaphoreWaitAny`] is not set,
///   the semaphore wait condition is that all of the semaphores in
///   [`SemaphoreWaitInfo::semaphores`] have reached the value specified by the corresponding
///   element of [`SemaphoreWaitInfo::values`].
///# Related
/// - [`VK_KHR_timeline_semaphore`]
/// - [`crate::vulkan1_2`]
/// - [`SemaphoreWaitFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSemaphoreWaitFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct SemaphoreWaitFlags(u32);
impl const Default for SemaphoreWaitFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn from(from: SemaphoreWaitFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl SemaphoreWaitFlags {
    ///[`SemaphoreWaitAny`] specifies that the semaphore wait
    ///condition is that at least one of the semaphores in
    ///[`SemaphoreWaitInfo`]::`pSemaphores` has reached the value
    ///specified by the corresponding element of
    ///[`SemaphoreWaitInfo`]::`pValues`.
    ///If [`SemaphoreWaitAny`] is not set, the semaphore wait
    ///condition is that all of the semaphores in
    ///[`SemaphoreWaitInfo`]::`pSemaphores` have reached the value
    ///specified by the corresponding element of
    ///[`SemaphoreWaitInfo`]::`pValues`.
    pub const SEMAPHORE_WAIT_ANY: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::SEMAPHORE_WAIT_ANY;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for SemaphoreWaitFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for SemaphoreWaitFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for SemaphoreWaitFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for SemaphoreWaitFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SemaphoreWaitFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SemaphoreWaitFlags> for SemaphoreWaitFlags {
    fn extend<T: IntoIterator<Item = SemaphoreWaitFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn extend<T: IntoIterator<Item = SemaphoreWaitFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<SemaphoreWaitFlagBits>>::from(i));
        }
    }
}
impl FromIterator<SemaphoreWaitFlags> for SemaphoreWaitFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreWaitFlags>>(iterator: T) -> SemaphoreWaitFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreWaitFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreWaitFlagBits>>(iterator: T) -> SemaphoreWaitFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreWaitFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SemaphoreWaitFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SemaphoreWaitFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SemaphoreWaitFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SemaphoreWaitFlags::SEMAPHORE_WAIT_ANY) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SEMAPHORE_WAIT_ANY))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SemaphoreWaitFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDescriptorBindingFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorBindingFlagBits.html) - Bitmask specifying descriptor set layout binding properties
///# C Specifications
///Bits which  **can**  be set in each element of
///[`DescriptorSetLayoutBindingFlagsCreateInfo::binding_flags`],
///specifying options for the corresponding descriptor set layout binding, are:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkDescriptorBindingFlagBits {
///    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT = 0x00000001,
///    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT = 0x00000002,
///    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT = 0x00000004,
///    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT = 0x00000008,
///  // Provided by VK_EXT_descriptor_indexing
///    VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT =
/// VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT,
///  // Provided by VK_EXT_descriptor_indexing
///    VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT =
/// VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT,
///  // Provided by VK_EXT_descriptor_indexing
///    VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT = VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT,
///  // Provided by VK_EXT_descriptor_indexing
///    VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT =
/// VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT,
///} VkDescriptorBindingFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_EXT_descriptor_indexing
///typedef VkDescriptorBindingFlagBits VkDescriptorBindingFlagBitsEXT;
///```
///# Description
/// - [`DescriptorBindingUpdateAfterBind`] indicates that if descriptors in this binding are updated
///   between when the descriptor set is bound in a command buffer and when that command buffer is
///   submitted to a queue, then the submission will use the most recently set descriptors for this
///   binding and the updates do not invalidate the command buffer. Descriptor bindings created with
///   this flag are also partially exempt from the external synchronization requirement in
///   [`UpdateDescriptorSetWithTemplateKHR`] and [`UpdateDescriptorSets`]. Multiple descriptors with
///   this flag set  **can**  be updated concurrently in different threads, though the same
///   descriptor  **must**  not be updated concurrently by two threads. Descriptors with this flag
///   set  **can**  be updated concurrently with the set being bound to a command buffer in another
///   thread, but not concurrently with the set being reset or freed.
/// - [`DescriptorBindingPartiallyBound`] indicates that descriptors in this binding that are not
///   *dynamically used* need not contain valid descriptors at the time the descriptors are
///   consumed. A descriptor is dynamically used if any shader invocation executes an instruction
///   that performs any memory access using the descriptor.
/// - [`DescriptorBindingUpdateUnusedWhilePending`] indicates that descriptors in this binding
///   **can**  be updated after a command buffer has bound this descriptor set, or while a command
///   buffer that uses this descriptor set is pending execution, as long as the descriptors that are
///   updated are not used by those command buffers. If [`DescriptorBindingPartiallyBound`] is also
///   set, then descriptors  **can**  be updated as long as they are not dynamically used by any
///   shader invocations. If [`DescriptorBindingPartiallyBound`] is not set, then descriptors
///   **can**  be updated as long as they are not statically used by any shader invocations.
/// - [`DescriptorBindingVariableDescriptorCount`] indicates that     this is a *variable-sized descriptor binding* whose size will be     specified when a descriptor set is allocated using this layout.     The value of `descriptorCount` is treated as an upper bound on the     size of the binding.     This  **must**  only be used for the last binding in the descriptor set     layout (i.e. the binding with the largest value of `binding`).     For the purposes of counting against limits such as     `maxDescriptorSet`* and `maxPerStageDescriptor`*, the full value     of `descriptorCount` is     counted, except for descriptor bindings with a descriptor type of     `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.     In this case, `descriptorCount` specifies the upper bound on the     byte size of the binding; thus it counts against the [`maxInlineUniformBlockSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformBlockSize) and [`maxInlineUniformTotalSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformTotalSize) limits instead.
///# Related
/// - [`VK_EXT_descriptor_indexing`]
/// - [`crate::vulkan1_2`]
/// - [`DescriptorBindingFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDescriptorBindingFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DescriptorBindingFlags(u32);
impl const Default for DescriptorBindingFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn from(from: DescriptorBindingFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl DescriptorBindingFlags {
    ///[`DescriptorBindingUpdateAfterBind`] indicates that if
    ///descriptors in this binding are updated between when the descriptor set
    ///is bound in a command buffer and when that command buffer is submitted
    ///to a queue, then the submission will use the most recently set
    ///descriptors for this binding and the updates do not invalidate the
    ///command buffer.
    ///Descriptor bindings created with this flag are also partially exempt
    ///from the external synchronization requirement in
    ///[`UpdateDescriptorSetWithTemplateKHR`] and
    ///[`UpdateDescriptorSets`].
    ///Multiple descriptors with this flag set  **can**  be updated concurrently in
    ///different threads, though the same descriptor  **must**  not be updated
    ///concurrently by two threads.
    ///Descriptors with this flag set  **can**  be updated concurrently with the set
    ///being bound to a command buffer in another thread, but not concurrently
    ///with the set being reset or freed.
    pub const DESCRIPTOR_BINDING_UPDATE_AFTER_BIND: Self = Self(1);
    ///[`DescriptorBindingUpdateUnusedWhilePending`] indicates
    ///that descriptors in this binding  **can**  be updated after a command buffer
    ///has bound this descriptor set, or while a command buffer that uses this
    ///descriptor set is pending execution, as long as the descriptors that are
    ///updated are not used by those command buffers.
    ///If [`DescriptorBindingPartiallyBound`] is also set, then
    ///descriptors  **can**  be updated as long as they are not dynamically used by
    ///any shader invocations.
    ///If [`DescriptorBindingPartiallyBound`] is not set, then
    ///descriptors  **can**  be updated as long as they are not statically used by
    ///any shader invocations.
    pub const DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING: Self = Self(2);
    ///[`DescriptorBindingPartiallyBound`] indicates that
    ///descriptors in this binding that are not *dynamically used* need not
    ///contain valid descriptors at the time the descriptors are consumed.
    ///A descriptor is dynamically used if any shader invocation executes an
    ///instruction that performs any memory access using the descriptor.
    pub const DESCRIPTOR_BINDING_PARTIALLY_BOUND: Self = Self(4);
    ///[`DescriptorBindingVariableDescriptorCount`] indicates that
    ///    this is a *variable-sized descriptor binding* whose size will be
    ///    specified when a descriptor set is allocated using this layout.
    ///    The value of `descriptorCount` is treated as an upper bound on the
    ///    size of the binding.
    ///    This  **must**  only be used for the last binding in the descriptor set
    ///    layout (i.e. the binding with the largest value of `binding`).
    ///    For the purposes of counting against limits such as
    ///    `maxDescriptorSet`* and `maxPerStageDescriptor`*, the full value
    ///    of `descriptorCount` is
    ///    counted, except for descriptor bindings with a descriptor type of
    ///    `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.
    ///    In this case, `descriptorCount` specifies the upper bound on the
    ///    byte size of the binding; thus it counts against the
    ///[`maxInlineUniformBlockSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformBlockSize) and [`maxInlineUniformTotalSize`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInlineUniformTotalSize) limits
    ///instead.
    pub const DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::DESCRIPTOR_BINDING_UPDATE_AFTER_BIND;
        }
        {
            all |= Self::DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING;
        }
        {
            all |= Self::DESCRIPTOR_BINDING_PARTIALLY_BOUND;
        }
        {
            all |= Self::DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DescriptorBindingFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DescriptorBindingFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DescriptorBindingFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DescriptorBindingFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DescriptorBindingFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DescriptorBindingFlags> for DescriptorBindingFlags {
    fn extend<T: IntoIterator<Item = DescriptorBindingFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn extend<T: IntoIterator<Item = DescriptorBindingFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<DescriptorBindingFlagBits>>::from(i));
        }
    }
}
impl FromIterator<DescriptorBindingFlags> for DescriptorBindingFlags {
    fn from_iter<T: IntoIterator<Item = DescriptorBindingFlags>>(iterator: T) -> DescriptorBindingFlags {
        let mut out = Self::empty();
        <Self as Extend<DescriptorBindingFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn from_iter<T: IntoIterator<Item = DescriptorBindingFlagBits>>(iterator: T) -> DescriptorBindingFlags {
        let mut out = Self::empty();
        <Self as Extend<DescriptorBindingFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DescriptorBindingFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DescriptorBindingFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DescriptorBindingFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(DescriptorBindingFlags::DESCRIPTOR_BINDING_UPDATE_AFTER_BIND)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DESCRIPTOR_BINDING_UPDATE_AFTER_BIND))?;
                    }
                    if self
                        .0
                        .contains(DescriptorBindingFlags::DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING))?;
                    }
                    if self
                        .0
                        .contains(DescriptorBindingFlags::DESCRIPTOR_BINDING_PARTIALLY_BOUND)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DESCRIPTOR_BINDING_PARTIALLY_BOUND))?;
                    }
                    if self
                        .0
                        .contains(DescriptorBindingFlags::DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DescriptorBindingFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkResolveModeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkResolveModeFlagBits.html) - Bitmask indicating supported depth and stencil resolve modes
///# C Specifications
///Possible values of
///[`SubpassDescriptionDepthStencilResolve::depth_resolve_mode`] and
///`stencilResolveMode`, specifying the depth and stencil resolve modes,
///are:
///```c
///// Provided by VK_VERSION_1_2
///typedef enum VkResolveModeFlagBits {
///    VK_RESOLVE_MODE_NONE = 0,
///    VK_RESOLVE_MODE_SAMPLE_ZERO_BIT = 0x00000001,
///    VK_RESOLVE_MODE_AVERAGE_BIT = 0x00000002,
///    VK_RESOLVE_MODE_MIN_BIT = 0x00000004,
///    VK_RESOLVE_MODE_MAX_BIT = 0x00000008,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_NONE_KHR = VK_RESOLVE_MODE_NONE,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR = VK_RESOLVE_MODE_SAMPLE_ZERO_BIT,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_AVERAGE_BIT_KHR = VK_RESOLVE_MODE_AVERAGE_BIT,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_MIN_BIT_KHR = VK_RESOLVE_MODE_MIN_BIT,
///  // Provided by VK_KHR_depth_stencil_resolve
///    VK_RESOLVE_MODE_MAX_BIT_KHR = VK_RESOLVE_MODE_MAX_BIT,
///} VkResolveModeFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_depth_stencil_resolve
///typedef VkResolveModeFlagBits VkResolveModeFlagBitsKHR;
///```
///# Description
/// - [`ResolveModeNone`] indicates that no resolve operation is done.
/// - [`ResolveModeSampleZero`] indicates that result of the resolve operation is equal to the value
///   of sample 0.
/// - [`ResolveModeAverage`] indicates that result of the resolve operation is the average of the
///   sample values.
/// - [`ResolveModeMin`] indicates that result of the resolve operation is the minimum of the sample
///   values.
/// - [`ResolveModeMax`] indicates that result of the resolve operation is the maximum of the sample
///   values.
///# Related
/// - [`VK_KHR_depth_stencil_resolve`]
/// - [`crate::vulkan1_2`]
/// - [`RenderingAttachmentInfo`]
/// - [`ResolveModeFlags`]
/// - [`SubpassDescriptionDepthStencilResolve`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkResolveModeFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ResolveModeFlags(u32);
impl const Default for ResolveModeFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ResolveModeFlagBits> for ResolveModeFlags {
    fn from(from: ResolveModeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl ResolveModeFlags {
    ///[`ResolveModeNone`] indicates that no resolve operation is done.
    pub const RESOLVE_MODE_NONE: Self = Self(0);
    ///[`ResolveModeSampleZero`] indicates that result of the
    ///resolve operation is equal to the value of sample 0.
    pub const RESOLVE_MODE_SAMPLE_ZERO: Self = Self(1);
    ///[`ResolveModeAverage`] indicates that result of the resolve
    ///operation is the average of the sample values.
    pub const RESOLVE_MODE_AVERAGE: Self = Self(2);
    ///[`ResolveModeMin`] indicates that result of the resolve
    ///operation is the minimum of the sample values.
    pub const RESOLVE_MODE_MIN: Self = Self(4);
    ///[`ResolveModeMax`] indicates that result of the resolve
    ///operation is the maximum of the sample values.
    pub const RESOLVE_MODE_MAX: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        {
            all |= Self::RESOLVE_MODE_NONE;
        }
        {
            all |= Self::RESOLVE_MODE_SAMPLE_ZERO;
        }
        {
            all |= Self::RESOLVE_MODE_AVERAGE;
        }
        {
            all |= Self::RESOLVE_MODE_MIN;
        }
        {
            all |= Self::RESOLVE_MODE_MAX;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ResolveModeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ResolveModeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ResolveModeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ResolveModeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ResolveModeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ResolveModeFlags> for ResolveModeFlags {
    fn extend<T: IntoIterator<Item = ResolveModeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ResolveModeFlagBits> for ResolveModeFlags {
    fn extend<T: IntoIterator<Item = ResolveModeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ResolveModeFlagBits>>::from(i));
        }
    }
}
impl FromIterator<ResolveModeFlags> for ResolveModeFlags {
    fn from_iter<T: IntoIterator<Item = ResolveModeFlags>>(iterator: T) -> ResolveModeFlags {
        let mut out = Self::empty();
        <Self as Extend<ResolveModeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ResolveModeFlagBits> for ResolveModeFlags {
    fn from_iter<T: IntoIterator<Item = ResolveModeFlagBits>>(iterator: T) -> ResolveModeFlags {
        let mut out = Self::empty();
        <Self as Extend<ResolveModeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ResolveModeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ResolveModeFlags);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ResolveModeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ResolveModeFlags::RESOLVE_MODE_NONE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESOLVE_MODE_NONE))?;
                    }
                    if self.0.contains(ResolveModeFlags::RESOLVE_MODE_SAMPLE_ZERO) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESOLVE_MODE_SAMPLE_ZERO))?;
                    }
                    if self.0.contains(ResolveModeFlags::RESOLVE_MODE_AVERAGE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESOLVE_MODE_AVERAGE))?;
                    }
                    if self.0.contains(ResolveModeFlags::RESOLVE_MODE_MIN) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESOLVE_MODE_MIN))?;
                    }
                    if self.0.contains(ResolveModeFlags::RESOLVE_MODE_MAX) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESOLVE_MODE_MAX))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ResolveModeFlags))
            .field(&Flags(*self))
            .finish()
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
#[doc(alias = "VkConformanceVersion")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ConformanceVersion {
    ///[`major`] is the major version number of the conformance test suite.
    pub major: u8,
    ///[`minor`] is the minor version number of the conformance test suite.
    pub minor: u8,
    ///[`subminor`] is the subminor version number of the conformance test
    ///suite.
    pub subminor: u8,
    ///[`patch`] is the patch version number of the conformance test suite.
    pub patch: u8,
}
impl Default for ConformanceVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 0,
            subminor: 0,
            patch: 0,
        }
    }
}
impl ConformanceVersion {
    ///Gets the value of [`Self::major`]
    pub fn major(&self) -> u8 {
        self.major
    }
    ///Gets the value of [`Self::minor`]
    pub fn minor(&self) -> u8 {
        self.minor
    }
    ///Gets the value of [`Self::subminor`]
    pub fn subminor(&self) -> u8 {
        self.subminor
    }
    ///Gets the value of [`Self::patch`]
    pub fn patch(&self) -> u8 {
        self.patch
    }
    ///Gets a mutable reference to the value of [`Self::major`]
    pub fn major_mut(&mut self) -> &mut u8 {
        &mut self.major
    }
    ///Gets a mutable reference to the value of [`Self::minor`]
    pub fn minor_mut(&mut self) -> &mut u8 {
        &mut self.minor
    }
    ///Gets a mutable reference to the value of [`Self::subminor`]
    pub fn subminor_mut(&mut self) -> &mut u8 {
        &mut self.subminor
    }
    ///Gets a mutable reference to the value of [`Self::patch`]
    pub fn patch_mut(&mut self) -> &mut u8 {
        &mut self.patch
    }
    ///Sets the raw value of [`Self::major`]
    pub fn set_major(&mut self, value: u8) -> &mut Self {
        self.major = value;
        self
    }
    ///Sets the raw value of [`Self::minor`]
    pub fn set_minor(&mut self, value: u8) -> &mut Self {
        self.minor = value;
        self
    }
    ///Sets the raw value of [`Self::subminor`]
    pub fn set_subminor(&mut self, value: u8) -> &mut Self {
        self.subminor = value;
        self
    }
    ///Sets the raw value of [`Self::patch`]
    pub fn set_patch(&mut self, value: u8) -> &mut Self {
        self.patch = value;
        self
    }
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
/// to a physical device.[`driver_id`] **must**  be immutable for a given driver across instances,
///processes, driver versions, and system reboots.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES`
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
#[doc(alias = "VkPhysicalDeviceDriverProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceDriverProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub driver_id: DriverId,
    ///No documentation found
    pub driver_name: [c_char; MAX_DRIVER_NAME_SIZE as usize],
    ///No documentation found
    pub driver_info: [c_char; MAX_DRIVER_INFO_SIZE as usize],
    ///No documentation found
    pub conformance_version: ConformanceVersion,
}
impl<'lt> Default for PhysicalDeviceDriverProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceDriverProperties,
            p_next: std::ptr::null_mut(),
            driver_id: Default::default(),
            driver_name: [b'\0' as i8; MAX_DRIVER_NAME_SIZE as usize],
            driver_info: [b'\0' as i8; MAX_DRIVER_INFO_SIZE as usize],
            conformance_version: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceDriverProperties<'lt> {
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
    ///Gets the value of [`Self::driver_id`]
    pub fn driver_id(&self) -> DriverId {
        self.driver_id
    }
    ///Gets the value of [`Self::driver_name`]
    pub fn driver_name(&self) -> &[c_char; MAX_DRIVER_NAME_SIZE as usize] {
        &self.driver_name
    }
    ///Gets the value of [`Self::driver_info`]
    pub fn driver_info(&self) -> &[c_char; MAX_DRIVER_INFO_SIZE as usize] {
        &self.driver_info
    }
    ///Gets the value of [`Self::conformance_version`]
    pub fn conformance_version(&self) -> ConformanceVersion {
        self.conformance_version
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
    ///Gets a mutable reference to the value of [`Self::driver_id`]
    pub fn driver_id_mut(&mut self) -> &mut DriverId {
        &mut self.driver_id
    }
    ///Gets a mutable reference to the value of [`Self::driver_name`]
    pub fn driver_name_mut(&mut self) -> &mut [c_char; MAX_DRIVER_NAME_SIZE as usize] {
        &mut self.driver_name
    }
    ///Gets a mutable reference to the value of [`Self::driver_info`]
    pub fn driver_info_mut(&mut self) -> &mut [c_char; MAX_DRIVER_INFO_SIZE as usize] {
        &mut self.driver_info
    }
    ///Gets a mutable reference to the value of [`Self::conformance_version`]
    pub fn conformance_version_mut(&mut self) -> &mut ConformanceVersion {
        &mut self.conformance_version
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
    ///Sets the raw value of [`Self::driver_id`]
    pub fn set_driver_id(&mut self, value: crate::vulkan1_2::DriverId) -> &mut Self {
        self.driver_id = value;
        self
    }
    ///Sets the raw value of [`Self::driver_name`]
    pub fn set_driver_name(
        &mut self,
        value: [std::os::raw::c_char; crate::vulkan1_2::MAX_DRIVER_NAME_SIZE as usize],
    ) -> &mut Self {
        self.driver_name = value;
        self
    }
    ///Sets the raw value of [`Self::driver_info`]
    pub fn set_driver_info(
        &mut self,
        value: [std::os::raw::c_char; crate::vulkan1_2::MAX_DRIVER_INFO_SIZE as usize],
    ) -> &mut Self {
        self.driver_info = value;
        self
    }
    ///Sets the raw value of [`Self::conformance_version`]
    pub fn set_conformance_version(&mut self, value: crate::vulkan1_2::ConformanceVersion) -> &mut Self {
        self.conformance_version = value;
        self
    }
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
///[`PhysicalDeviceShaderSubgroupExtendedTypesFeatures`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_subgroup_extended_types`] is a boolean specifying whether
    ///subgroup operations can use 8-bit integer, 16-bit integer, 64-bit
    ///integer, 16-bit floating-point, and vectors of these types in
    ///[group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) with
    ///[subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup), if the implementation
    ///supports the types.
    pub shader_subgroup_extended_types: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceShaderSubgroupExtendedTypesFeatures,
            p_next: std::ptr::null_mut(),
            shader_subgroup_extended_types: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_subgroup_extended_types`]
    pub fn shader_subgroup_extended_types_raw(&self) -> Bool32 {
        self.shader_subgroup_extended_types
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_subgroup_extended_types`]
    pub fn set_shader_subgroup_extended_types_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_subgroup_extended_types = value;
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
    ///Gets the value of [`Self::shader_subgroup_extended_types`]
    pub fn shader_subgroup_extended_types(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_subgroup_extended_types as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_subgroup_extended_types`]
    pub fn shader_subgroup_extended_types_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_subgroup_extended_types as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_subgroup_extended_types as *mut Bool32)
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
    ///Sets the raw value of [`Self::shader_subgroup_extended_types`]
    pub fn set_shader_subgroup_extended_types(&mut self, value: bool) -> &mut Self {
        self.shader_subgroup_extended_types = value as u8 as u32;
        self
    }
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
///formats  **must**  support the
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
///mapping of the image view used with min/max filtering  **must**  have been
///created with the `r` component set to the
///[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings).
///Only the `r` component of the sampled image value is defined and the
///other component values are undefined.
///If [`filter_minmax_image_component_mapping`] is [`TRUE`] this restriction
///does not apply and image component mapping works as normal.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES`
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
#[doc(alias = "VkPhysicalDeviceSamplerFilterMinmaxProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub filter_minmax_single_component_formats: Bool32,
    ///No documentation found
    pub filter_minmax_image_component_mapping: Bool32,
}
impl<'lt> Default for PhysicalDeviceSamplerFilterMinmaxProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceSamplerFilterMinmaxProperties,
            p_next: std::ptr::null_mut(),
            filter_minmax_single_component_formats: 0,
            filter_minmax_image_component_mapping: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSamplerFilterMinmaxProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::filter_minmax_single_component_formats`]
    pub fn filter_minmax_single_component_formats_raw(&self) -> Bool32 {
        self.filter_minmax_single_component_formats
    }
    ///Gets the raw value of [`Self::filter_minmax_image_component_mapping`]
    pub fn filter_minmax_image_component_mapping_raw(&self) -> Bool32 {
        self.filter_minmax_image_component_mapping
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::filter_minmax_single_component_formats`]
    pub fn set_filter_minmax_single_component_formats_raw(&mut self, value: Bool32) -> &mut Self {
        self.filter_minmax_single_component_formats = value;
        self
    }
    ///Sets the raw value of [`Self::filter_minmax_image_component_mapping`]
    pub fn set_filter_minmax_image_component_mapping_raw(&mut self, value: Bool32) -> &mut Self {
        self.filter_minmax_image_component_mapping = value;
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
    ///Gets the value of [`Self::filter_minmax_single_component_formats`]
    pub fn filter_minmax_single_component_formats(&self) -> bool {
        unsafe { std::mem::transmute(self.filter_minmax_single_component_formats as u8) }
    }
    ///Gets the value of [`Self::filter_minmax_image_component_mapping`]
    pub fn filter_minmax_image_component_mapping(&self) -> bool {
        unsafe { std::mem::transmute(self.filter_minmax_image_component_mapping as u8) }
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
    ///Gets a mutable reference to the value of [`Self::filter_minmax_single_component_formats`]
    pub fn filter_minmax_single_component_formats_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.filter_minmax_single_component_formats as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.filter_minmax_single_component_formats as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::filter_minmax_image_component_mapping`]
    pub fn filter_minmax_image_component_mapping_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.filter_minmax_image_component_mapping as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.filter_minmax_image_component_mapping as *mut Bool32)
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
    ///Sets the raw value of [`Self::filter_minmax_single_component_formats`]
    pub fn set_filter_minmax_single_component_formats(&mut self, value: bool) -> &mut Self {
        self.filter_minmax_single_component_formats = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::filter_minmax_image_component_mapping`]
    pub fn set_filter_minmax_image_component_mapping(&mut self, value: bool) -> &mut Self {
        self.filter_minmax_image_component_mapping = value as u8 as u32;
        self
    }
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
///`VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO`
/// - [`reduction_mode`] **must**  be a valid [`SamplerReductionMode`] value
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
#[doc(alias = "VkSamplerReductionModeCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SamplerReductionModeCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`reduction_mode`] is a [`SamplerReductionMode`] value controlling
    ///how texture filtering combines texel values.
    pub reduction_mode: SamplerReductionMode,
}
impl<'lt> Default for SamplerReductionModeCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SamplerReductionModeCreateInfo,
            p_next: std::ptr::null(),
            reduction_mode: Default::default(),
        }
    }
}
impl<'lt> SamplerReductionModeCreateInfo<'lt> {
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
    ///Gets the value of [`Self::reduction_mode`]
    pub fn reduction_mode(&self) -> SamplerReductionMode {
        self.reduction_mode
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::reduction_mode`]
    pub fn reduction_mode_mut(&mut self) -> &mut SamplerReductionMode {
        &mut self.reduction_mode
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
    ///Sets the raw value of [`Self::reduction_mode`]
    pub fn set_reduction_mode(&mut self, value: crate::vulkan1_2::SamplerReductionMode) -> &mut Self {
        self.reduction_mode = value;
        self
    }
}
///[VkImageFormatListCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatListCreateInfo.html) - Specify that an image can: be used with a particular set of formats
///# C Specifications
///If the [`p_next`] chain of [`ImageCreateInfo`] includes a
///[`ImageFormatListCreateInfo`] structure, then that structure contains a
///list of all formats that  **can**  be used when creating views of this image.The
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
/// - [`view_format_count`] is the number of entries in the [`view_formats`] array.
/// - [`view_formats`] is a pointer to an array of [`Format`] values specifying all formats which
///   **can**  be used when creating views of this image.
///# Description
///If [`view_format_count`] is zero, [`view_formats`] is ignored and the
///image is created as if the [`ImageFormatListCreateInfo`] structure were
///not included in the [`p_next`] chain of [`ImageCreateInfo`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO`
/// - If [`view_format_count`] is not `0`, [`view_formats`] **must**  be a valid pointer to an array
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
#[doc(alias = "VkImageFormatListCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageFormatListCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`view_format_count`] is the number of entries in the [`view_formats`]
    ///array.
    pub view_format_count: u32,
    ///[`view_formats`] is a pointer to an array of [`Format`] values
    ///specifying all formats which  **can**  be used when creating views of this
    ///image.
    pub view_formats: *const Format,
}
impl<'lt> Default for ImageFormatListCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ImageFormatListCreateInfo,
            p_next: std::ptr::null(),
            view_format_count: 0,
            view_formats: std::ptr::null(),
        }
    }
}
impl<'lt> ImageFormatListCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::view_formats`]
    pub fn view_formats_raw(&self) -> *const Format {
        self.view_formats
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::view_formats`]
    pub fn set_view_formats_raw(&mut self, value: *const Format) -> &mut Self {
        self.view_formats = value;
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
    ///Gets the value of [`Self::view_format_count`]
    pub fn view_format_count(&self) -> u32 {
        self.view_format_count
    }
    ///Gets the value of [`Self::view_formats`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn view_formats(&self) -> &[Format] {
        std::slice::from_raw_parts(self.view_formats, self.view_format_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::view_format_count`]
    pub fn view_format_count_mut(&mut self) -> &mut u32 {
        &mut self.view_format_count
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
    ///Sets the raw value of [`Self::view_format_count`]
    pub fn set_view_format_count(&mut self, value: u32) -> &mut Self {
        self.view_format_count = value;
        self
    }
    ///Sets the raw value of [`Self::view_formats`]
    pub fn set_view_formats(&mut self, value: &'lt [crate::vulkan1_0::Format]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.view_formats = value.as_ptr();
        self.view_format_count = len_;
        self
    }
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
///   also indicates whether shader modules  **can**  declare the `Float16` capability. However,
///   this only enables a subset of the storage classes that SPIR-V allows for the `Float16` SPIR-V
///   capability: Declaring and using 16-bit floats in the `Private`, `Workgroup` (for non-Block
///   variables), and `Function` storage classes is enabled, while declaring them in the interface
///   storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`, `Output`, and
///   `PushConstant`) is not enabled.
/// - [`shader_int_8`] indicates whether 8-bit integers (signed and unsigned) are supported in
///   shader code. This also indicates whether shader modules  **can**  declare the `Int8`
///   capability. However, this only enables a subset of the storage classes that SPIR-V allows for
///   the `Int8` SPIR-V capability: Declaring and using 8-bit integers in the `Private`, `Workgroup`
///   (for non-Block variables), and `Function` storage classes is enabled, while declaring them in
///   the interface storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`,
///   `Output`, and `PushConstant`) is not enabled.
///If the [`PhysicalDeviceShaderFloat16Int8Features`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderFloat16Int8Features`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceShaderFloat16Int8Features")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8Features<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_float_16`] indicates
    ///whether 16-bit floats (halfs) are supported in shader code.
    ///This also indicates whether shader modules  **can**  declare the `Float16`
    ///capability.
    ///However, this only enables a subset of the storage classes that SPIR-V
    ///allows for the `Float16` SPIR-V capability: Declaring and using
    ///16-bit floats in the `Private`,
    ///`Workgroup` (for non-Block variables),
    ///and `Function` storage classes is enabled, while declaring them in
    ///the interface storage classes (e.g., `UniformConstant`, `Uniform`,
    ///`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
    ///not enabled.
    pub shader_float_16: Bool32,
    ///[`shader_int_8`] indicates
    ///whether 8-bit integers (signed and unsigned) are supported in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the `Int8`
    ///capability.
    ///However, this only enables a subset of the storage classes that SPIR-V
    ///allows for the `Int8` SPIR-V capability: Declaring and using 8-bit
    ///integers in the `Private`,
    ///`Workgroup` (for non-Block variables),
    ///and `Function` storage classes is enabled, while declaring them in
    ///the interface storage classes (e.g., `UniformConstant`, `Uniform`,
    ///`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
    ///not enabled.
    pub shader_int_8: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderFloat16Int8Features<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceShaderFloat16Int8Features,
            p_next: std::ptr::null_mut(),
            shader_float_16: 0,
            shader_int_8: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderFloat16Int8Features<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_float_16`]
    pub fn shader_float_16_raw(&self) -> Bool32 {
        self.shader_float_16
    }
    ///Gets the raw value of [`Self::shader_int_8`]
    pub fn shader_int_8_raw(&self) -> Bool32 {
        self.shader_int_8
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_float_16`]
    pub fn set_shader_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_int_8`]
    pub fn set_shader_int_8_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_int_8 = value;
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
    ///Gets the value of [`Self::shader_float_16`]
    pub fn shader_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_int_8`]
    pub fn shader_int_8(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_int_8 as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_float_16`]
    pub fn shader_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_int_8`]
    pub fn shader_int_8_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_int_8 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_int_8 as *mut Bool32)
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
    ///Sets the raw value of [`Self::shader_float_16`]
    pub fn set_shader_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_int_8`]
    pub fn set_shader_int_8(&mut self, value: bool) -> &mut Self {
        self.shader_int_8 = value as u8 as u32;
        self
    }
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
///   class="base"><span style="height:0.66666em;vertical-align:-0.08333em;"
///   class="strut"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span> **can**  be preserved in 16-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**
///   be used for 16-bit floating-point types.
/// - [`shader_signed_zero_inf_nan_preserve_float_32`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span class="katex-html" aria-hidden="true"><span
///   class="base"><span class="strut"
///   style="height:0.66666em;vertical-align:-0.08333em;"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span> **can**  be preserved in 32-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**
///   be used for 32-bit floating-point types.
/// - [`shader_signed_zero_inf_nan_preserve_float_64`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span aria-hidden="true" class="katex-html"><span
///   class="base"><span style="height:0.66666em;vertical-align:-0.08333em;"
///   class="strut"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span> **can**  be preserved in 64-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**
///   be used for 64-bit floating-point types.
/// - [`shader_denorm_preserve_float_16`] is a boolean value indicating whether denormals  **can**
///   be preserved in 16-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode  **can**  be used for 16-bit floating-point types.
/// - [`shader_denorm_preserve_float_32`] is a boolean value indicating whether denormals  **can**
///   be preserved in 32-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode  **can**  be used for 32-bit floating-point types.
/// - [`shader_denorm_preserve_float_64`] is a boolean value indicating whether denormals  **can**
///   be preserved in 64-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode  **can**  be used for 64-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_16`] is a boolean value indicating whether denormals
///   **can**  be flushed to zero in 16-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode  **can**  be used for 16-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_32`] is a boolean value indicating whether denormals
///   **can**  be flushed to zero in 32-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode  **can**  be used for 32-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_64`] is a boolean value indicating whether denormals
///   **can**  be flushed to zero in 64-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode  **can**  be used for 64-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_16`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 16-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can**  be used for 16-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_32`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 32-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can**  be used for 32-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_64`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 64-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can**  be used for 64-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_16`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 16-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can**  be used for 16-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_32`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 32-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can**  be used for 32-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_64`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 64-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can**  be used for 64-bit floating-point types.
///If the [`PhysicalDeviceFloatControlsProperties`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES`
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
#[doc(alias = "VkPhysicalDeviceFloatControlsProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFloatControlsProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    ///No documentation found
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    ///No documentation found
    pub shader_signed_zero_inf_nan_preserve_float_16: Bool32,
    ///No documentation found
    pub shader_signed_zero_inf_nan_preserve_float_32: Bool32,
    ///No documentation found
    pub shader_signed_zero_inf_nan_preserve_float_64: Bool32,
    ///No documentation found
    pub shader_denorm_preserve_float_16: Bool32,
    ///No documentation found
    pub shader_denorm_preserve_float_32: Bool32,
    ///No documentation found
    pub shader_denorm_preserve_float_64: Bool32,
    ///No documentation found
    pub shader_denorm_flush_to_zero_float_16: Bool32,
    ///No documentation found
    pub shader_denorm_flush_to_zero_float_32: Bool32,
    ///No documentation found
    pub shader_denorm_flush_to_zero_float_64: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rte_float_16: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rte_float_32: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rte_float_64: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rtz_float_16: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rtz_float_32: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rtz_float_64: Bool32,
}
impl<'lt> Default for PhysicalDeviceFloatControlsProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceFloatControlsProperties,
            p_next: std::ptr::null_mut(),
            denorm_behavior_independence: Default::default(),
            rounding_mode_independence: Default::default(),
            shader_signed_zero_inf_nan_preserve_float_16: 0,
            shader_signed_zero_inf_nan_preserve_float_32: 0,
            shader_signed_zero_inf_nan_preserve_float_64: 0,
            shader_denorm_preserve_float_16: 0,
            shader_denorm_preserve_float_32: 0,
            shader_denorm_preserve_float_64: 0,
            shader_denorm_flush_to_zero_float_16: 0,
            shader_denorm_flush_to_zero_float_32: 0,
            shader_denorm_flush_to_zero_float_64: 0,
            shader_rounding_mode_rte_float_16: 0,
            shader_rounding_mode_rte_float_32: 0,
            shader_rounding_mode_rte_float_64: 0,
            shader_rounding_mode_rtz_float_16: 0,
            shader_rounding_mode_rtz_float_32: 0,
            shader_rounding_mode_rtz_float_64: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFloatControlsProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn shader_signed_zero_inf_nan_preserve_float_16_raw(&self) -> Bool32 {
        self.shader_signed_zero_inf_nan_preserve_float_16
    }
    ///Gets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn shader_signed_zero_inf_nan_preserve_float_32_raw(&self) -> Bool32 {
        self.shader_signed_zero_inf_nan_preserve_float_32
    }
    ///Gets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn shader_signed_zero_inf_nan_preserve_float_64_raw(&self) -> Bool32 {
        self.shader_signed_zero_inf_nan_preserve_float_64
    }
    ///Gets the raw value of [`Self::shader_denorm_preserve_float_16`]
    pub fn shader_denorm_preserve_float_16_raw(&self) -> Bool32 {
        self.shader_denorm_preserve_float_16
    }
    ///Gets the raw value of [`Self::shader_denorm_preserve_float_32`]
    pub fn shader_denorm_preserve_float_32_raw(&self) -> Bool32 {
        self.shader_denorm_preserve_float_32
    }
    ///Gets the raw value of [`Self::shader_denorm_preserve_float_64`]
    pub fn shader_denorm_preserve_float_64_raw(&self) -> Bool32 {
        self.shader_denorm_preserve_float_64
    }
    ///Gets the raw value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn shader_denorm_flush_to_zero_float_16_raw(&self) -> Bool32 {
        self.shader_denorm_flush_to_zero_float_16
    }
    ///Gets the raw value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn shader_denorm_flush_to_zero_float_32_raw(&self) -> Bool32 {
        self.shader_denorm_flush_to_zero_float_32
    }
    ///Gets the raw value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn shader_denorm_flush_to_zero_float_64_raw(&self) -> Bool32 {
        self.shader_denorm_flush_to_zero_float_64
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn shader_rounding_mode_rte_float_16_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rte_float_16
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn shader_rounding_mode_rte_float_32_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rte_float_32
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn shader_rounding_mode_rte_float_64_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rte_float_64
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn shader_rounding_mode_rtz_float_16_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rtz_float_16
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn shader_rounding_mode_rtz_float_32_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rtz_float_32
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn shader_rounding_mode_rtz_float_64_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rtz_float_64
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_64 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_16`]
    pub fn set_shader_denorm_preserve_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_preserve_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_32`]
    pub fn set_shader_denorm_preserve_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_preserve_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_64`]
    pub fn set_shader_denorm_preserve_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_preserve_float_64 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn set_shader_denorm_flush_to_zero_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn set_shader_denorm_flush_to_zero_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn set_shader_denorm_flush_to_zero_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_64 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn set_shader_rounding_mode_rte_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rte_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn set_shader_rounding_mode_rte_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rte_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn set_shader_rounding_mode_rte_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rte_float_64 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn set_shader_rounding_mode_rtz_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rtz_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn set_shader_rounding_mode_rtz_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rtz_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn set_shader_rounding_mode_rtz_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rtz_float_64 = value;
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
    ///Gets the value of [`Self::denorm_behavior_independence`]
    pub fn denorm_behavior_independence(&self) -> ShaderFloatControlsIndependence {
        self.denorm_behavior_independence
    }
    ///Gets the value of [`Self::rounding_mode_independence`]
    pub fn rounding_mode_independence(&self) -> ShaderFloatControlsIndependence {
        self.rounding_mode_independence
    }
    ///Gets the value of [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn shader_signed_zero_inf_nan_preserve_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_signed_zero_inf_nan_preserve_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn shader_signed_zero_inf_nan_preserve_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_signed_zero_inf_nan_preserve_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn shader_signed_zero_inf_nan_preserve_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_signed_zero_inf_nan_preserve_float_64 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_preserve_float_16`]
    pub fn shader_denorm_preserve_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_preserve_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_preserve_float_32`]
    pub fn shader_denorm_preserve_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_preserve_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_preserve_float_64`]
    pub fn shader_denorm_preserve_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_preserve_float_64 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn shader_denorm_flush_to_zero_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_flush_to_zero_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn shader_denorm_flush_to_zero_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_flush_to_zero_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn shader_denorm_flush_to_zero_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_flush_to_zero_float_64 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn shader_rounding_mode_rte_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rte_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn shader_rounding_mode_rte_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rte_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn shader_rounding_mode_rte_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rte_float_64 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn shader_rounding_mode_rtz_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rtz_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn shader_rounding_mode_rtz_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rtz_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn shader_rounding_mode_rtz_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rtz_float_64 as u8) }
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
    ///Gets a mutable reference to the value of [`Self::denorm_behavior_independence`]
    pub fn denorm_behavior_independence_mut(&mut self) -> &mut ShaderFloatControlsIndependence {
        &mut self.denorm_behavior_independence
    }
    ///Gets a mutable reference to the value of [`Self::rounding_mode_independence`]
    pub fn rounding_mode_independence_mut(&mut self) -> &mut ShaderFloatControlsIndependence {
        &mut self.rounding_mode_independence
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn shader_signed_zero_inf_nan_preserve_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn shader_signed_zero_inf_nan_preserve_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn shader_signed_zero_inf_nan_preserve_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_preserve_float_16`]
    pub fn shader_denorm_preserve_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_preserve_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_preserve_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_preserve_float_32`]
    pub fn shader_denorm_preserve_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_preserve_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_preserve_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_preserve_float_64`]
    pub fn shader_denorm_preserve_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_preserve_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_preserve_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn shader_denorm_flush_to_zero_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_flush_to_zero_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_flush_to_zero_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn shader_denorm_flush_to_zero_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_flush_to_zero_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_flush_to_zero_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn shader_denorm_flush_to_zero_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_flush_to_zero_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_flush_to_zero_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn shader_rounding_mode_rte_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rte_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rte_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn shader_rounding_mode_rte_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rte_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rte_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn shader_rounding_mode_rte_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rte_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rte_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn shader_rounding_mode_rtz_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rtz_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rtz_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn shader_rounding_mode_rtz_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rtz_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rtz_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn shader_rounding_mode_rtz_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rtz_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rtz_float_64 as *mut Bool32)
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
    ///Sets the raw value of [`Self::denorm_behavior_independence`]
    pub fn set_denorm_behavior_independence(
        &mut self,
        value: crate::vulkan1_2::ShaderFloatControlsIndependence,
    ) -> &mut Self {
        self.denorm_behavior_independence = value;
        self
    }
    ///Sets the raw value of [`Self::rounding_mode_independence`]
    pub fn set_rounding_mode_independence(
        &mut self,
        value: crate::vulkan1_2::ShaderFloatControlsIndependence,
    ) -> &mut Self {
        self.rounding_mode_independence = value;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_64 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_16`]
    pub fn set_shader_denorm_preserve_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_preserve_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_32`]
    pub fn set_shader_denorm_preserve_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_preserve_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_64`]
    pub fn set_shader_denorm_preserve_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_preserve_float_64 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn set_shader_denorm_flush_to_zero_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn set_shader_denorm_flush_to_zero_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn set_shader_denorm_flush_to_zero_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_64 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn set_shader_rounding_mode_rte_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rte_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn set_shader_rounding_mode_rte_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rte_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn set_shader_rounding_mode_rte_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rte_float_64 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn set_shader_rounding_mode_rtz_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rtz_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn set_shader_rounding_mode_rtz_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rtz_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn set_shader_rounding_mode_rtz_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rtz_float_64 = value as u8 as u32;
        self
    }
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
///[`PhysicalDeviceHostQueryResetFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceHostQueryResetFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`host_query_reset`]
    ///indicates that the implementation supports resetting queries from the
    ///host with [`ResetQueryPool`].
    pub host_query_reset: Bool32,
}
impl<'lt> Default for PhysicalDeviceHostQueryResetFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceHostQueryResetFeatures,
            p_next: std::ptr::null_mut(),
            host_query_reset: 0,
        }
    }
}
impl<'lt> PhysicalDeviceHostQueryResetFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::host_query_reset`]
    pub fn host_query_reset_raw(&self) -> Bool32 {
        self.host_query_reset
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::host_query_reset`]
    pub fn set_host_query_reset_raw(&mut self, value: Bool32) -> &mut Self {
        self.host_query_reset = value;
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
    ///Gets the value of [`Self::host_query_reset`]
    pub fn host_query_reset(&self) -> bool {
        unsafe { std::mem::transmute(self.host_query_reset as u8) }
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
    ///Gets a mutable reference to the value of [`Self::host_query_reset`]
    pub fn host_query_reset_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.host_query_reset as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.host_query_reset as *mut Bool32)
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
    ///Sets the raw value of [`Self::host_query_reset`]
    pub fn set_host_query_reset(&mut self, value: bool) -> &mut Self {
        self.host_query_reset = value as u8 as u32;
        self
    }
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
///   attachments  **can**  be indexed by dynamically uniform integer expressions in shader code. If
///   this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `InputAttachmentArrayDynamicIndexing` capability.
/// - [`shader_uniform_texel_buffer_array_dynamic_indexing`] indicates whether arrays of uniform
///   texel buffers  **can**  be indexed by dynamically uniform integer expressions in shader code.
///   If this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `UniformTexelBufferArrayDynamicIndexing` capability.
/// - [`shader_storage_texel_buffer_array_dynamic_indexing`] indicates whether arrays of storage
///   texel buffers  **can**  be indexed by dynamically uniform integer expressions in shader code.
///   If this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `StorageTexelBufferArrayDynamicIndexing` capability.
/// - [`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform
///   buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` **must**
///   not be indexed by non-uniform integer expressions when aggregated into arrays in shader code.
///   This also indicates whether shader modules  **can**  declare the
///   `UniformBufferArrayNonUniformIndexing` capability.
/// - [`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays of samplers or
///   sampled images  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_SAMPLER`,
///   `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` **must**
///   not be indexed by non-uniform integer expressions when aggregated into arrays in shader code.
///   This also indicates whether shader modules  **can**  declare the
///   `SampledImageArrayNonUniformIndexing` capability.
/// - [`shader_storage_buffer_array_non_uniform_indexing`] indicates whether arrays of storage
///   buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` **must**
///   not be indexed by non-uniform integer expressions when aggregated into arrays in shader code.
///   This also indicates whether shader modules  **can**  declare the
///   `StorageBufferArrayNonUniformIndexing` capability.
/// - [`shader_storage_image_array_non_uniform_indexing`] indicates whether arrays of storage images
///   **can**  be indexed by non-uniform integer expressions in shader code. If this feature is not
///   enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` **must**  not
///   be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This
///   also indicates whether shader modules  **can**  declare the
///   `StorageImageArrayNonUniformIndexing` capability.
/// - [`shader_input_attachment_array_non_uniform_indexing`] indicates whether arrays of input
///   attachments  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `InputAttachmentArrayNonUniformIndexing` capability.
/// - [`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform
///   texel buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `UniformTexelBufferArrayNonUniformIndexing` capability.
/// - [`shader_storage_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of storage
///   texel buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `StorageTexelBufferArrayNonUniformIndexing` capability.
/// - [`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether the implementation
///   supports updating uniform buffer descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.
/// - [`descriptor_binding_sampled_image_update_after_bind`] indicates whether the implementation
///   supports updating sampled image descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
///   `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.
/// - [`descriptor_binding_storage_image_update_after_bind`] indicates whether the implementation
///   supports updating storage image descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
///   `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
/// - [`descriptor_binding_storage_buffer_update_after_bind`] indicates whether the implementation
///   supports updating storage buffer descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`.
/// - [`descriptor_binding_uniform_texel_buffer_update_after_bind`] indicates whether the
///   implementation supports updating uniform texel buffer descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be
///   used with `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
/// - [`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates whether the
///   implementation supports updating storage texel buffer descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be
///   used with `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
/// - [`descriptor_binding_update_unused_while_pending`] indicates whether the implementation
///   supports updating descriptors while the set is in use. If this feature is not enabled,
///   `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT` **must**  not be used.
/// - [`descriptor_binding_partially_bound`] indicates whether the implementation supports
///   statically using a descriptor set binding in which some descriptors are not valid. If this
///   feature is not enabled, `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT` **must**  not be used.
/// - [`descriptor_binding_variable_descriptor_count`] indicates whether the implementation supports
///   descriptor sets with a variable-sized last binding. If this feature is not enabled,
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT` **must**  not be used.
/// - [`runtime_descriptor_array`] indicates whether the implementation supports the SPIR-V
///   `RuntimeDescriptorArray` capability. If this feature is not enabled, descriptors  **must**
///   not be declared in runtime arrays.
///If the [`PhysicalDeviceDescriptorIndexingFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceDescriptorIndexingFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_input_attachment_array_dynamic_indexing`] indicates whether arrays
    ///of input attachments  **can**  be indexed by dynamically uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`InputAttachmentArrayDynamicIndexing` capability.
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    ///[`shader_uniform_texel_buffer_array_dynamic_indexing`] indicates whether
    ///arrays of uniform texel buffers  **can**  be indexed by dynamically uniform
    ///integer expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`UniformTexelBufferArrayDynamicIndexing` capability.
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    ///[`shader_storage_texel_buffer_array_dynamic_indexing`] indicates whether
    ///arrays of storage texel buffers  **can**  be indexed by dynamically uniform
    ///integer expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StorageTexelBufferArrayDynamicIndexing` capability.
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    ///[`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of uniform buffers  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`UniformBufferArrayNonUniformIndexing` capability.
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays
    ///of samplers or sampled images  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_SAMPLER`,
    ///`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
    ///`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`SampledImageArrayNonUniformIndexing` capability.
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of storage buffers  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StorageBufferArrayNonUniformIndexing` capability.
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_image_array_non_uniform_indexing`] indicates whether arrays
    ///of storage images  **can**  be indexed by non-uniform integer expressions in
    ///shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StorageImageArrayNonUniformIndexing` capability.
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    ///[`shader_input_attachment_array_non_uniform_indexing`] indicates whether
    ///arrays of input attachments  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`InputAttachmentArrayNonUniformIndexing` capability.
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    ///[`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of uniform texel buffers  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`UniformTexelBufferArrayNonUniformIndexing` capability.
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_texel_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of storage texel buffers  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StorageTexelBufferArrayNonUniformIndexing` capability.
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    ///[`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether
    ///the implementation supports updating uniform buffer descriptors after a
    ///set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_sampled_image_update_after_bind`] indicates whether the
    ///implementation supports updating sampled image descriptors after a set
    ///is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_SAMPLER`,
    ///`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
    ///`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_image_update_after_bind`] indicates whether the
    ///implementation supports updating storage image descriptors after a set
    ///is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_buffer_update_after_bind`] indicates whether
    ///the implementation supports updating storage buffer descriptors after a
    ///set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`.
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_uniform_texel_buffer_update_after_bind`] indicates
    ///whether the implementation supports updating uniform texel buffer
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates
    ///whether the implementation supports updating storage texel buffer
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_update_unused_while_pending`] indicates whether the
    ///implementation supports updating descriptors while the set is in use.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT` **must**  not be
    ///used.
    pub descriptor_binding_update_unused_while_pending: Bool32,
    ///[`descriptor_binding_partially_bound`] indicates whether the
    ///implementation supports statically using a descriptor set binding in
    ///which some descriptors are not valid.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT` **must**  not be used.
    pub descriptor_binding_partially_bound: Bool32,
    ///[`descriptor_binding_variable_descriptor_count`] indicates whether the
    ///implementation supports descriptor sets with a variable-sized last
    ///binding.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT` **must**  not be
    ///used.
    pub descriptor_binding_variable_descriptor_count: Bool32,
    ///[`runtime_descriptor_array`] indicates whether the implementation
    ///supports the SPIR-V `RuntimeDescriptorArray` capability.
    ///If this feature is not enabled, descriptors  **must**  not be declared in
    ///runtime arrays.
    pub runtime_descriptor_array: Bool32,
}
impl<'lt> Default for PhysicalDeviceDescriptorIndexingFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceDescriptorIndexingFeatures,
            p_next: std::ptr::null_mut(),
            shader_input_attachment_array_dynamic_indexing: 0,
            shader_uniform_texel_buffer_array_dynamic_indexing: 0,
            shader_storage_texel_buffer_array_dynamic_indexing: 0,
            shader_uniform_buffer_array_non_uniform_indexing: 0,
            shader_sampled_image_array_non_uniform_indexing: 0,
            shader_storage_buffer_array_non_uniform_indexing: 0,
            shader_storage_image_array_non_uniform_indexing: 0,
            shader_input_attachment_array_non_uniform_indexing: 0,
            shader_uniform_texel_buffer_array_non_uniform_indexing: 0,
            shader_storage_texel_buffer_array_non_uniform_indexing: 0,
            descriptor_binding_uniform_buffer_update_after_bind: 0,
            descriptor_binding_sampled_image_update_after_bind: 0,
            descriptor_binding_storage_image_update_after_bind: 0,
            descriptor_binding_storage_buffer_update_after_bind: 0,
            descriptor_binding_uniform_texel_buffer_update_after_bind: 0,
            descriptor_binding_storage_texel_buffer_update_after_bind: 0,
            descriptor_binding_update_unused_while_pending: 0,
            descriptor_binding_partially_bound: 0,
            descriptor_binding_variable_descriptor_count: 0,
            runtime_descriptor_array: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDescriptorIndexingFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn shader_input_attachment_array_dynamic_indexing_raw(&self) -> Bool32 {
        self.shader_input_attachment_array_dynamic_indexing
    }
    ///Gets the raw value of [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing_raw(&self) -> Bool32 {
        self.shader_uniform_texel_buffer_array_dynamic_indexing
    }
    ///Gets the raw value of [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn shader_storage_texel_buffer_array_dynamic_indexing_raw(&self) -> Bool32 {
        self.shader_storage_texel_buffer_array_dynamic_indexing
    }
    ///Gets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_uniform_buffer_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn shader_sampled_image_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_sampled_image_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_storage_buffer_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn shader_storage_image_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_storage_image_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn shader_input_attachment_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_input_attachment_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_storage_texel_buffer_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_buffer_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_uniform_buffer_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn descriptor_binding_sampled_image_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_sampled_image_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn descriptor_binding_storage_image_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_storage_image_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_buffer_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_storage_buffer_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_storage_texel_buffer_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn descriptor_binding_update_unused_while_pending_raw(&self) -> Bool32 {
        self.descriptor_binding_update_unused_while_pending
    }
    ///Gets the raw value of [`Self::descriptor_binding_partially_bound`]
    pub fn descriptor_binding_partially_bound_raw(&self) -> Bool32 {
        self.descriptor_binding_partially_bound
    }
    ///Gets the raw value of [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn descriptor_binding_variable_descriptor_count_raw(&self) -> Bool32 {
        self.descriptor_binding_variable_descriptor_count
    }
    ///Gets the raw value of [`Self::runtime_descriptor_array`]
    pub fn runtime_descriptor_array_raw(&self) -> Bool32 {
        self.runtime_descriptor_array
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn set_shader_input_attachment_array_dynamic_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_input_attachment_array_dynamic_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn set_shader_uniform_texel_buffer_array_dynamic_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_uniform_texel_buffer_array_dynamic_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn set_shader_storage_texel_buffer_array_dynamic_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_texel_buffer_array_dynamic_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn set_shader_sampled_image_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn set_shader_storage_buffer_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn set_shader_storage_image_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn set_shader_input_attachment_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn set_shader_uniform_texel_buffer_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn set_shader_storage_texel_buffer_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_texel_buffer_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn set_descriptor_binding_uniform_buffer_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_uniform_buffer_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn set_descriptor_binding_sampled_image_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_sampled_image_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn set_descriptor_binding_storage_image_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_storage_image_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn set_descriptor_binding_storage_buffer_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_storage_buffer_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn set_descriptor_binding_uniform_texel_buffer_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn set_descriptor_binding_storage_texel_buffer_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_storage_texel_buffer_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn set_descriptor_binding_update_unused_while_pending_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_update_unused_while_pending = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_partially_bound`]
    pub fn set_descriptor_binding_partially_bound_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_partially_bound = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn set_descriptor_binding_variable_descriptor_count_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_variable_descriptor_count = value;
        self
    }
    ///Sets the raw value of [`Self::runtime_descriptor_array`]
    pub fn set_runtime_descriptor_array_raw(&mut self, value: Bool32) -> &mut Self {
        self.runtime_descriptor_array = value;
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
    ///Gets the value of [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn shader_input_attachment_array_dynamic_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_input_attachment_array_dynamic_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_uniform_texel_buffer_array_dynamic_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_texel_buffer_array_dynamic_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_uniform_buffer_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn shader_sampled_image_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_sampled_image_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_buffer_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_buffer_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn shader_storage_image_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_image_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn shader_input_attachment_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_input_attachment_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_uniform_texel_buffer_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_texel_buffer_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_buffer_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_uniform_buffer_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn descriptor_binding_sampled_image_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_sampled_image_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn descriptor_binding_storage_image_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_storage_image_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_buffer_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_storage_buffer_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_uniform_texel_buffer_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_storage_texel_buffer_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn descriptor_binding_update_unused_while_pending(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_update_unused_while_pending as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_partially_bound`]
    pub fn descriptor_binding_partially_bound(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_partially_bound as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn descriptor_binding_variable_descriptor_count(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_variable_descriptor_count as u8) }
    }
    ///Gets the value of [`Self::runtime_descriptor_array`]
    pub fn runtime_descriptor_array(&self) -> bool {
        unsafe { std::mem::transmute(self.runtime_descriptor_array as u8) }
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
    ///Gets a mutable reference to the value of
    /// [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn shader_input_attachment_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_input_attachment_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_input_attachment_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_uniform_texel_buffer_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_uniform_texel_buffer_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn shader_storage_texel_buffer_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_texel_buffer_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_texel_buffer_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_uniform_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_uniform_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn shader_sampled_image_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_sampled_image_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_sampled_image_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn shader_storage_image_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_image_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_image_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn shader_input_attachment_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_input_attachment_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_input_attachment_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_uniform_texel_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_uniform_texel_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_texel_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_texel_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_uniform_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_uniform_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn descriptor_binding_sampled_image_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_sampled_image_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_sampled_image_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn descriptor_binding_storage_image_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_storage_image_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_storage_image_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_storage_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_storage_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_uniform_texel_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_uniform_texel_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_storage_texel_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_storage_texel_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn descriptor_binding_update_unused_while_pending_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_update_unused_while_pending as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_update_unused_while_pending as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_binding_partially_bound`]
    pub fn descriptor_binding_partially_bound_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_partially_bound as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_partially_bound as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn descriptor_binding_variable_descriptor_count_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_variable_descriptor_count as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_variable_descriptor_count as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::runtime_descriptor_array`]
    pub fn runtime_descriptor_array_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.runtime_descriptor_array as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.runtime_descriptor_array as *mut Bool32)
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
    ///Sets the raw value of [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn set_shader_input_attachment_array_dynamic_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_input_attachment_array_dynamic_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn set_shader_uniform_texel_buffer_array_dynamic_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_uniform_texel_buffer_array_dynamic_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn set_shader_storage_texel_buffer_array_dynamic_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_storage_texel_buffer_array_dynamic_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn set_shader_sampled_image_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn set_shader_storage_buffer_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn set_shader_storage_image_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn set_shader_input_attachment_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn set_shader_uniform_texel_buffer_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn set_shader_storage_texel_buffer_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_storage_texel_buffer_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn set_descriptor_binding_uniform_buffer_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_uniform_buffer_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn set_descriptor_binding_sampled_image_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_sampled_image_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn set_descriptor_binding_storage_image_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_storage_image_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn set_descriptor_binding_storage_buffer_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_storage_buffer_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn set_descriptor_binding_uniform_texel_buffer_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn set_descriptor_binding_storage_texel_buffer_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_storage_texel_buffer_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn set_descriptor_binding_update_unused_while_pending(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_update_unused_while_pending = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_partially_bound`]
    pub fn set_descriptor_binding_partially_bound(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_partially_bound = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn set_descriptor_binding_variable_descriptor_count(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_variable_descriptor_count = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::runtime_descriptor_array`]
    pub fn set_runtime_descriptor_array(&mut self, value: bool) -> &mut Self {
        self.runtime_descriptor_array = value as u8 as u32;
        self
    }
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
///   (summed over all descriptor types) that  **can**  be created across all pools that are created
///   with the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` bit set. Pool creation  **may**
///   fail when this limit is exceeded, or when the space this limit represents is unable to satisfy
///   a pool creation due to fragmentation.
/// - [`shader_uniform_buffer_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether uniform buffer descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of uniform
///   buffers  **may**  execute multiple times in order to access all the descriptors.
/// - [`shader_sampled_image_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether sampler and image descriptors natively support nonuniform indexing. If this is
///   [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array
///   of samplers or images  **may**  execute multiple times in order to access all the descriptors.
/// - [`shader_storage_buffer_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether storage buffer descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of storage
///   buffers  **may**  execute multiple times in order to access all the descriptors.
/// - [`shader_storage_image_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether storage image descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of storage
///   images  **may**  execute multiple times in order to access all the descriptors.
/// - [`shader_input_attachment_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether input attachment descriptors natively support nonuniform indexing. If this is
///   [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array
///   of input attachments  **may**  execute multiple times in order to access all the descriptors.
/// - [`robust_buffer_access_update_after_bind`] is a boolean value indicating whether [`robustBufferAccess`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-robustBufferAccess)
///   **can**  be enabled in a device simultaneously with
///   `descriptorBindingUniformBufferUpdateAfterBind`,
///   `descriptorBindingStorageBufferUpdateAfterBind`,
///   `descriptorBindingUniformTexelBufferUpdateAfterBind`, and/or
///   `descriptorBindingStorageTexelBufferUpdateAfterBind`. If this is [`FALSE`], then either
///   `robustBufferAccess` **must**  be disabled or all of these update-after-bind features
///   **must**  be disabled.
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
///   While an application  **can**  allocate dynamic uniform buffer descriptors from a pool created
///   with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these
///   descriptors  **must**  not be present in any descriptor set layout that includes bindings
///   created with `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`.
/// - [`max_descriptor_set_update_after_bind_storage_buffers`] is similar to
///   `maxDescriptorSetStorageBuffers` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_storage_buffers_dynamic`] is similar to
///   `maxDescriptorSetStorageBuffersDynamic` but counts descriptors from descriptor sets created
///   with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
///   While an application  **can**  allocate dynamic storage buffer descriptors from a pool created
///   with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these
///   descriptors  **must**  not be present in any descriptor set layout that includes bindings
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
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES`
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
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    ///No documentation found
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub robust_buffer_access_update_after_bind: Bool32,
    ///No documentation found
    pub quad_divergent_implicit_lod: Bool32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    ///No documentation found
    pub max_per_stage_update_after_bind_resources: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_samplers: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}
impl<'lt> Default for PhysicalDeviceDescriptorIndexingProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceDescriptorIndexingProperties,
            p_next: std::ptr::null_mut(),
            max_update_after_bind_descriptors_in_all_pools: 0,
            shader_uniform_buffer_array_non_uniform_indexing_native: 0,
            shader_sampled_image_array_non_uniform_indexing_native: 0,
            shader_storage_buffer_array_non_uniform_indexing_native: 0,
            shader_storage_image_array_non_uniform_indexing_native: 0,
            shader_input_attachment_array_non_uniform_indexing_native: 0,
            robust_buffer_access_update_after_bind: 0,
            quad_divergent_implicit_lod: 0,
            max_per_stage_descriptor_update_after_bind_samplers: 0,
            max_per_stage_descriptor_update_after_bind_uniform_buffers: 0,
            max_per_stage_descriptor_update_after_bind_storage_buffers: 0,
            max_per_stage_descriptor_update_after_bind_sampled_images: 0,
            max_per_stage_descriptor_update_after_bind_storage_images: 0,
            max_per_stage_descriptor_update_after_bind_input_attachments: 0,
            max_per_stage_update_after_bind_resources: 0,
            max_descriptor_set_update_after_bind_samplers: 0,
            max_descriptor_set_update_after_bind_uniform_buffers: 0,
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: 0,
            max_descriptor_set_update_after_bind_storage_buffers: 0,
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: 0,
            max_descriptor_set_update_after_bind_sampled_images: 0,
            max_descriptor_set_update_after_bind_storage_images: 0,
            max_descriptor_set_update_after_bind_input_attachments: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDescriptorIndexingProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_uniform_buffer_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn shader_sampled_image_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_sampled_image_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_storage_buffer_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn shader_storage_image_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_storage_image_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn shader_input_attachment_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_input_attachment_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn robust_buffer_access_update_after_bind_raw(&self) -> Bool32 {
        self.robust_buffer_access_update_after_bind
    }
    ///Gets the raw value of [`Self::quad_divergent_implicit_lod`]
    pub fn quad_divergent_implicit_lod_raw(&self) -> Bool32 {
        self.quad_divergent_implicit_lod
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn set_shader_sampled_image_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn set_shader_storage_buffer_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn set_shader_storage_image_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn set_shader_input_attachment_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn set_robust_buffer_access_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.robust_buffer_access_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::quad_divergent_implicit_lod`]
    pub fn set_quad_divergent_implicit_lod_raw(&mut self, value: Bool32) -> &mut Self {
        self.quad_divergent_implicit_lod = value;
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
    ///Gets the value of [`Self::max_update_after_bind_descriptors_in_all_pools`]
    pub fn max_update_after_bind_descriptors_in_all_pools(&self) -> u32 {
        self.max_update_after_bind_descriptors_in_all_pools
    }
    ///Gets the value of [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_uniform_buffer_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn shader_sampled_image_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_sampled_image_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_buffer_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn shader_storage_image_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_image_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn shader_input_attachment_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_input_attachment_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn robust_buffer_access_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.robust_buffer_access_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::quad_divergent_implicit_lod`]
    pub fn quad_divergent_implicit_lod(&self) -> bool {
        unsafe { std::mem::transmute(self.quad_divergent_implicit_lod as u8) }
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_samplers`]
    pub fn max_per_stage_descriptor_update_after_bind_samplers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_samplers
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_uniform_buffers`]
    pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_uniform_buffers
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_storage_buffers`]
    pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_storage_buffers
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_sampled_images`]
    pub fn max_per_stage_descriptor_update_after_bind_sampled_images(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_sampled_images
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_storage_images`]
    pub fn max_per_stage_descriptor_update_after_bind_storage_images(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_storage_images
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_input_attachments`]
    pub fn max_per_stage_descriptor_update_after_bind_input_attachments(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_input_attachments
    }
    ///Gets the value of [`Self::max_per_stage_update_after_bind_resources`]
    pub fn max_per_stage_update_after_bind_resources(&self) -> u32 {
        self.max_per_stage_update_after_bind_resources
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_samplers`]
    pub fn max_descriptor_set_update_after_bind_samplers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_samplers
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_uniform_buffers`]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_uniform_buffers
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_uniform_buffers_dynamic`]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_storage_buffers`]
    pub fn max_descriptor_set_update_after_bind_storage_buffers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_buffers
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_storage_buffers_dynamic`]
    pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_buffers_dynamic
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_sampled_images`]
    pub fn max_descriptor_set_update_after_bind_sampled_images(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_sampled_images
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_storage_images`]
    pub fn max_descriptor_set_update_after_bind_storage_images(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_images
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_input_attachments`]
    pub fn max_descriptor_set_update_after_bind_input_attachments(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_input_attachments
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
    ///Gets a mutable reference to the value of
    /// [`Self::max_update_after_bind_descriptors_in_all_pools`]
    pub fn max_update_after_bind_descriptors_in_all_pools_mut(&mut self) -> &mut u32 {
        &mut self.max_update_after_bind_descriptors_in_all_pools
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_uniform_buffer_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_uniform_buffer_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn shader_sampled_image_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_sampled_image_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_sampled_image_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_buffer_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_buffer_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn shader_storage_image_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_image_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_image_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn shader_input_attachment_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_input_attachment_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_input_attachment_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn robust_buffer_access_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.robust_buffer_access_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.robust_buffer_access_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::quad_divergent_implicit_lod`]
    pub fn quad_divergent_implicit_lod_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.quad_divergent_implicit_lod as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.quad_divergent_implicit_lod as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_samplers`]
    pub fn max_per_stage_descriptor_update_after_bind_samplers_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_samplers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_uniform_buffers`]
    pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_uniform_buffers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_storage_buffers`]
    pub fn max_per_stage_descriptor_update_after_bind_storage_buffers_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_storage_buffers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_sampled_images`]
    pub fn max_per_stage_descriptor_update_after_bind_sampled_images_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_sampled_images
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_storage_images`]
    pub fn max_per_stage_descriptor_update_after_bind_storage_images_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_storage_images
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_input_attachments`]
    pub fn max_per_stage_descriptor_update_after_bind_input_attachments_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_input_attachments
    }
    ///Gets a mutable reference to the value of [`Self::max_per_stage_update_after_bind_resources`]
    pub fn max_per_stage_update_after_bind_resources_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_update_after_bind_resources
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_samplers`]
    pub fn max_descriptor_set_update_after_bind_samplers_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_samplers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_uniform_buffers`]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_uniform_buffers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_uniform_buffers_dynamic`]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_storage_buffers`]
    pub fn max_descriptor_set_update_after_bind_storage_buffers_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_storage_buffers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_storage_buffers_dynamic`]
    pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_storage_buffers_dynamic
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_sampled_images`]
    pub fn max_descriptor_set_update_after_bind_sampled_images_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_sampled_images
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_storage_images`]
    pub fn max_descriptor_set_update_after_bind_storage_images_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_storage_images
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_input_attachments`]
    pub fn max_descriptor_set_update_after_bind_input_attachments_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_input_attachments
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
    ///Sets the raw value of [`Self::max_update_after_bind_descriptors_in_all_pools`]
    pub fn set_max_update_after_bind_descriptors_in_all_pools(&mut self, value: u32) -> &mut Self {
        self.max_update_after_bind_descriptors_in_all_pools = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn set_shader_sampled_image_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn set_shader_storage_buffer_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn set_shader_storage_image_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn set_shader_input_attachment_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn set_robust_buffer_access_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.robust_buffer_access_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::quad_divergent_implicit_lod`]
    pub fn set_quad_divergent_implicit_lod(&mut self, value: bool) -> &mut Self {
        self.quad_divergent_implicit_lod = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_samplers`]
    pub fn set_max_per_stage_descriptor_update_after_bind_samplers(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_samplers = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_uniform_buffers`]
    pub fn set_max_per_stage_descriptor_update_after_bind_uniform_buffers(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_uniform_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_storage_buffers`]
    pub fn set_max_per_stage_descriptor_update_after_bind_storage_buffers(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_storage_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_sampled_images`]
    pub fn set_max_per_stage_descriptor_update_after_bind_sampled_images(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_sampled_images = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_storage_images`]
    pub fn set_max_per_stage_descriptor_update_after_bind_storage_images(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_storage_images = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_input_attachments`]
    pub fn set_max_per_stage_descriptor_update_after_bind_input_attachments(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_input_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_update_after_bind_resources`]
    pub fn set_max_per_stage_update_after_bind_resources(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_update_after_bind_resources = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_samplers`]
    pub fn set_max_descriptor_set_update_after_bind_samplers(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_samplers = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_uniform_buffers`]
    pub fn set_max_descriptor_set_update_after_bind_uniform_buffers(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_uniform_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_uniform_buffers_dynamic`]
    pub fn set_max_descriptor_set_update_after_bind_uniform_buffers_dynamic(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_storage_buffers`]
    pub fn set_max_descriptor_set_update_after_bind_storage_buffers(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_storage_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_storage_buffers_dynamic`]
    pub fn set_max_descriptor_set_update_after_bind_storage_buffers_dynamic(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_storage_buffers_dynamic = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_sampled_images`]
    pub fn set_max_descriptor_set_update_after_bind_sampled_images(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_sampled_images = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_storage_images`]
    pub fn set_max_descriptor_set_update_after_bind_storage_images(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_storage_images = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_input_attachments`]
    pub fn set_max_descriptor_set_update_after_bind_input_attachments(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_input_attachments = value;
        self
    }
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
/// - [`binding_count`] is zero or the number of elements in [`binding_flags`].
/// - [`binding_flags`] is a pointer to an array of [`DescriptorBindingFlags`] bitfields, one for
///   each descriptor set layout binding.
///# Description
///If [`binding_count`] is zero or if this structure is not included in the
///[`p_next`] chain, the [`DescriptorBindingFlags`] for each descriptor
///set layout binding is considered to be zero.
///Otherwise, the descriptor set layout binding at
///[`DescriptorSetLayoutCreateInfo::bindings`][i] uses the flags in
///[`binding_flags`][i].
///## Valid Usage
/// - If [`binding_count`] is not zero, [`binding_count`] **must**  equal
///   [`DescriptorSetLayoutCreateInfo`]::[`binding_count`]
/// - If [`DescriptorSetLayoutCreateInfo::flags`] includes
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`, then all elements of
///   [`binding_flags`] **must**  not include `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`,
///   `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`, or
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`
/// - If an element of [`binding_flags`] includes
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`, then all other elements of
///   [`DescriptorSetLayoutCreateInfo::bindings`] **must**  have a smaller value of `binding`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_uniform_buffer_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` **must**  not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_sampled_image_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` **must**  not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_storage_image_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` **must**  not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_storage_buffer_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` **must**  not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_uniform_texel_buffer_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_storage_texel_buffer_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceInlineUniformBlockFeatures::
///   descriptor_binding_inline_uniform_block_update_after_bind`] is not enabled, all bindings with
///   descriptor type `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` **must**  not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceAccelerationStructureFeaturesKHR::
///   descriptor_binding_acceleration_structure_update_after_bind`] is not enabled, all bindings
///   with descriptor type `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` or
///   `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV` **must**  not use
///   `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - All bindings with descriptor type `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`,
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`, or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
///   **must**  not use `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::
///   descriptor_binding_update_unused_while_pending`] is not enabled, all elements of
///   [`binding_flags`] **must**  not include
///   `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_partially_bound`] is not
///   enabled, all elements of [`binding_flags`] **must**  not include
///   `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT`
/// - If [`PhysicalDeviceDescriptorIndexingFeatures::descriptor_binding_variable_descriptor_count`]
///   is not enabled, all elements of [`binding_flags`] **must**  not include
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`
/// - If an element of [`binding_flags`] includes
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT`, that element’s `descriptorType`
///   **must**  not be `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` or
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO`
/// - If [`binding_count`] is not `0`, [`binding_flags`] **must**  be a valid pointer to an array of
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
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`binding_count`] is zero or the number of elements in
    ///[`binding_flags`].
    pub binding_count: u32,
    ///[`binding_flags`] is a pointer to an array of
    ///[`DescriptorBindingFlags`] bitfields, one for each descriptor set
    ///layout binding.
    pub binding_flags: *const DescriptorBindingFlags,
}
impl<'lt> Default for DescriptorSetLayoutBindingFlagsCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DescriptorSetLayoutBindingFlagsCreateInfo,
            p_next: std::ptr::null(),
            binding_count: 0,
            binding_flags: std::ptr::null(),
        }
    }
}
impl<'lt> DescriptorSetLayoutBindingFlagsCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::binding_flags`]
    pub fn binding_flags_raw(&self) -> *const DescriptorBindingFlags {
        self.binding_flags
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::binding_flags`]
    pub fn set_binding_flags_raw(&mut self, value: *const DescriptorBindingFlags) -> &mut Self {
        self.binding_flags = value;
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
    ///Gets the value of [`Self::binding_count`]
    pub fn binding_count(&self) -> u32 {
        self.binding_count
    }
    ///Gets the value of [`Self::binding_flags`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn binding_flags(&self) -> &[DescriptorBindingFlags] {
        std::slice::from_raw_parts(self.binding_flags, self.binding_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::binding_count`]
    pub fn binding_count_mut(&mut self) -> &mut u32 {
        &mut self.binding_count
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
    ///Sets the raw value of [`Self::binding_count`]
    pub fn set_binding_count(&mut self, value: u32) -> &mut Self {
        self.binding_count = value;
        self
    }
    ///Sets the raw value of [`Self::binding_flags`]
    pub fn set_binding_flags(&mut self, value: &'lt [crate::vulkan1_2::DescriptorBindingFlags]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.binding_flags = value.as_ptr();
        self.binding_count = len_;
        self
    }
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
/// - [`descriptor_set_count`] is zero or the number of elements in [`descriptor_counts`].
/// - [`descriptor_counts`] is a pointer to an array of descriptor counts, with each member
///   specifying the number of descriptors in a variable-sized descriptor binding in the
///   corresponding descriptor set being allocated.
///# Description
///If [`descriptor_set_count`] is zero or this structure is not included in the
///[`p_next`] chain, then the variable lengths are considered to be zero.
///Otherwise, [`descriptor_counts`][i] is the number of descriptors in the
///variable-sized descriptor binding in the corresponding descriptor set
///layout.
///If the variable-sized descriptor binding in the corresponding descriptor set
///layout has a descriptor type of
///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
///[`descriptor_counts`][i] specifies the binding’s capacity in bytes.
///If [`DescriptorSetAllocateInfo::set_layouts`][i] does not include
///a variable-sized descriptor binding, then [`descriptor_counts`][i] is
///ignored.
///## Valid Usage
/// - If [`descriptor_set_count`] is not zero, [`descriptor_set_count`] **must**  equal
///   [`DescriptorSetAllocateInfo`]::[`descriptor_set_count`]
/// - If [`DescriptorSetAllocateInfo::set_layouts`][i] has a variable-sized descriptor binding, then
///   [`descriptor_counts`][i]  **must**  be less than or equal to the descriptor count specified
///   for that binding when the descriptor set layout was created
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO`
/// - If [`descriptor_set_count`] is not `0`, [`descriptor_counts`] **must**  be a valid pointer to
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
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`descriptor_set_count`] is zero or the number of elements in
    ///[`descriptor_counts`].
    pub descriptor_set_count: u32,
    ///[`descriptor_counts`] is a pointer to an array of descriptor counts,
    ///with each member specifying the number of descriptors in a
    ///variable-sized descriptor binding in the corresponding descriptor set
    ///being allocated.
    pub descriptor_counts: *const u32,
}
impl<'lt> Default for DescriptorSetVariableDescriptorCountAllocateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DescriptorSetVariableDescriptorCountAllocateInfo,
            p_next: std::ptr::null(),
            descriptor_set_count: 0,
            descriptor_counts: std::ptr::null(),
        }
    }
}
impl<'lt> DescriptorSetVariableDescriptorCountAllocateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::descriptor_counts`]
    pub fn descriptor_counts_raw(&self) -> *const u32 {
        self.descriptor_counts
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_counts`]
    pub fn set_descriptor_counts_raw(&mut self, value: *const u32) -> &mut Self {
        self.descriptor_counts = value;
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
    ///Gets the value of [`Self::descriptor_set_count`]
    pub fn descriptor_set_count(&self) -> u32 {
        self.descriptor_set_count
    }
    ///Gets the value of [`Self::descriptor_counts`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn descriptor_counts(&self) -> &[u32] {
        std::slice::from_raw_parts(self.descriptor_counts, self.descriptor_set_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_set_count`]
    pub fn descriptor_set_count_mut(&mut self) -> &mut u32 {
        &mut self.descriptor_set_count
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
    ///Sets the raw value of [`Self::descriptor_set_count`]
    pub fn set_descriptor_set_count(&mut self, value: u32) -> &mut Self {
        self.descriptor_set_count = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_counts`]
    pub fn set_descriptor_counts(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.descriptor_counts = value.as_ptr();
        self.descriptor_set_count = len_;
        self
    }
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
///descriptor that  **can**  be successfully created (which is greater than or equal
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
///[`max_variable_descriptor_count`] is undefined.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupport")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_variable_descriptor_count`] indicates the maximum number of
    ///descriptors supported in the highest numbered binding of the layout, if
    ///that binding is variable-sized.
    ///If the highest numbered binding of the layout has a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
    ///[`max_variable_descriptor_count`] indicates the maximum byte size
    ///supported for the binding, if that binding is variable-sized.
    pub max_variable_descriptor_count: u32,
}
impl<'lt> Default for DescriptorSetVariableDescriptorCountLayoutSupport<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DescriptorSetVariableDescriptorCountLayoutSupport,
            p_next: std::ptr::null_mut(),
            max_variable_descriptor_count: 0,
        }
    }
}
impl<'lt> DescriptorSetVariableDescriptorCountLayoutSupport<'lt> {
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
    ///Gets the value of [`Self::max_variable_descriptor_count`]
    pub fn max_variable_descriptor_count(&self) -> u32 {
        self.max_variable_descriptor_count
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
    ///Gets a mutable reference to the value of [`Self::max_variable_descriptor_count`]
    pub fn max_variable_descriptor_count_mut(&mut self) -> &mut u32 {
        &mut self.max_variable_descriptor_count
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
    ///Sets the raw value of [`Self::max_variable_descriptor_count`]
    pub fn set_max_variable_descriptor_count(&mut self, value: u32) -> &mut Self {
        self.max_variable_descriptor_count = value;
        self
    }
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
///a depth/stencil format, [`initial_layout`] and [`final_layout`] **can**  be
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
///application  **must**  specify the initial and final layouts of the stencil
///aspect by including a [`AttachmentDescriptionStencilLayout`] structure
///in the [`p_next`] chain.
///## Valid Usage
/// - [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED` or
///   `VK_IMAGE_LAYOUT_PREINITIALIZED`
/// - If [`format`] is a color format, [`initial_layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - If [`format`] is a depth/stencil format, [`initial_layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
/// - If [`format`] is a color format, [`final_layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - If [`format`] is a depth/stencil format, [`final_layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is not enabled, [`initial_layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is not enabled, [`final_layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a color format, [`initial_layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a color format, [`final_layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a depth/stencil format which includes both depth and stencil aspects, and
///   [`initial_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, the [`p_next`] chain  **must**  include a
///   [`AttachmentDescriptionStencilLayout`] structure
/// - If [`format`] is a depth/stencil format which includes both depth and stencil aspects, and
///   [`final_layout`] is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, the [`p_next`] chain  **must**  include a
///   [`AttachmentDescriptionStencilLayout`] structure
/// - If [`format`] is a depth/stencil format which includes only the depth aspect,
///   [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a depth/stencil format which includes only the depth aspect, [`final_layout`]
///   **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`format`] is a depth/stencil format which includes only the stencil aspect,
///   [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`
/// - If [`format`] is a depth/stencil format which includes only the stencil aspect,
///   [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is enabled and [`format`] is a depth/stencil format that includes a depth aspect and
///   the [`p_next`] chain includes a [`AttachmentDescriptionStencilLayout`] structure,
///   [`initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is enabled and [`format`] is a depth/stencil format that includes a depth aspect and
///   the [`p_next`] chain includes a [`AttachmentDescriptionStencilLayout`] structure,
///   [`final_layout`] **must**  not be `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`AttachmentDescriptionStencilLayout`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`AttachmentDescriptionFlagBits`] values
/// - [`format`] **must**  be a valid [`Format`] value
/// - [`samples`] **must**  be a valid [`SampleCountFlagBits`] value
/// - [`load_op`] **must**  be a valid [`AttachmentLoadOp`] value
/// - [`store_op`] **must**  be a valid [`AttachmentStoreOp`] value
/// - [`stencil_load_op`] **must**  be a valid [`AttachmentLoadOp`] value
/// - [`stencil_store_op`] **must**  be a valid [`AttachmentStoreOp`] value
/// - [`initial_layout`] **must**  be a valid [`ImageLayout`] value
/// - [`final_layout`] **must**  be a valid [`ImageLayout`] value
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
#[doc(alias = "VkAttachmentDescription2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AttachmentDescription2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`AttachmentDescriptionFlagBits`]
    ///specifying additional properties of the attachment.
    pub flags: AttachmentDescriptionFlags,
    ///[`format`] is a [`Format`] value specifying the format of the
    ///image that will be used for the attachment.
    pub format: Format,
    ///[`samples`] is a [`SampleCountFlagBits`] value specifying the
    ///number of samples of the image.
    pub samples: SampleCountFlagBits,
    ///[`load_op`] is a [`AttachmentLoadOp`] value specifying how the
    ///contents of color and depth components of the attachment are treated at
    ///the beginning of the subpass where it is first used.
    pub load_op: AttachmentLoadOp,
    ///[`store_op`] is a [`AttachmentStoreOp`] value specifying how the
    ///contents of color and depth components of the attachment are treated at
    ///the end of the subpass where it is last used.
    pub store_op: AttachmentStoreOp,
    ///[`stencil_load_op`] is a [`AttachmentLoadOp`] value specifying how
    ///the contents of stencil components of the attachment are treated at the
    ///beginning of the subpass where it is first used.
    pub stencil_load_op: AttachmentLoadOp,
    ///[`stencil_store_op`] is a [`AttachmentStoreOp`] value specifying how
    ///the contents of stencil components of the attachment are treated at the
    ///end of the last subpass where it is used.
    pub stencil_store_op: AttachmentStoreOp,
    ///[`initial_layout`] is the layout the attachment image subresource will
    ///be in when a render pass instance begins.
    pub initial_layout: ImageLayout,
    ///[`final_layout`] is the layout the attachment image subresource will be
    ///transitioned to when a render pass instance ends.
    pub final_layout: ImageLayout,
}
impl<'lt> Default for AttachmentDescription2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::AttachmentDescription2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            format: Default::default(),
            samples: Default::default(),
            load_op: Default::default(),
            store_op: Default::default(),
            stencil_load_op: Default::default(),
            stencil_store_op: Default::default(),
            initial_layout: Default::default(),
            final_layout: Default::default(),
        }
    }
}
impl<'lt> AttachmentDescription2<'lt> {
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
    pub fn flags(&self) -> AttachmentDescriptionFlags {
        self.flags
    }
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets the value of [`Self::samples`]
    pub fn samples(&self) -> SampleCountFlagBits {
        self.samples
    }
    ///Gets the value of [`Self::load_op`]
    pub fn load_op(&self) -> AttachmentLoadOp {
        self.load_op
    }
    ///Gets the value of [`Self::store_op`]
    pub fn store_op(&self) -> AttachmentStoreOp {
        self.store_op
    }
    ///Gets the value of [`Self::stencil_load_op`]
    pub fn stencil_load_op(&self) -> AttachmentLoadOp {
        self.stencil_load_op
    }
    ///Gets the value of [`Self::stencil_store_op`]
    pub fn stencil_store_op(&self) -> AttachmentStoreOp {
        self.stencil_store_op
    }
    ///Gets the value of [`Self::initial_layout`]
    pub fn initial_layout(&self) -> ImageLayout {
        self.initial_layout
    }
    ///Gets the value of [`Self::final_layout`]
    pub fn final_layout(&self) -> ImageLayout {
        self.final_layout
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut AttachmentDescriptionFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::samples`]
    pub fn samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.samples
    }
    ///Gets a mutable reference to the value of [`Self::load_op`]
    pub fn load_op_mut(&mut self) -> &mut AttachmentLoadOp {
        &mut self.load_op
    }
    ///Gets a mutable reference to the value of [`Self::store_op`]
    pub fn store_op_mut(&mut self) -> &mut AttachmentStoreOp {
        &mut self.store_op
    }
    ///Gets a mutable reference to the value of [`Self::stencil_load_op`]
    pub fn stencil_load_op_mut(&mut self) -> &mut AttachmentLoadOp {
        &mut self.stencil_load_op
    }
    ///Gets a mutable reference to the value of [`Self::stencil_store_op`]
    pub fn stencil_store_op_mut(&mut self) -> &mut AttachmentStoreOp {
        &mut self.stencil_store_op
    }
    ///Gets a mutable reference to the value of [`Self::initial_layout`]
    pub fn initial_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.initial_layout
    }
    ///Gets a mutable reference to the value of [`Self::final_layout`]
    pub fn final_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.final_layout
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
    pub fn set_flags(&mut self, value: crate::vulkan1_0::AttachmentDescriptionFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the raw value of [`Self::samples`]
    pub fn set_samples(&mut self, value: crate::vulkan1_0::SampleCountFlagBits) -> &mut Self {
        self.samples = value;
        self
    }
    ///Sets the raw value of [`Self::load_op`]
    pub fn set_load_op(&mut self, value: crate::vulkan1_0::AttachmentLoadOp) -> &mut Self {
        self.load_op = value;
        self
    }
    ///Sets the raw value of [`Self::store_op`]
    pub fn set_store_op(&mut self, value: crate::vulkan1_0::AttachmentStoreOp) -> &mut Self {
        self.store_op = value;
        self
    }
    ///Sets the raw value of [`Self::stencil_load_op`]
    pub fn set_stencil_load_op(&mut self, value: crate::vulkan1_0::AttachmentLoadOp) -> &mut Self {
        self.stencil_load_op = value;
        self
    }
    ///Sets the raw value of [`Self::stencil_store_op`]
    pub fn set_stencil_store_op(&mut self, value: crate::vulkan1_0::AttachmentStoreOp) -> &mut Self {
        self.stencil_store_op = value;
        self
    }
    ///Sets the raw value of [`Self::initial_layout`]
    pub fn set_initial_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.initial_layout = value;
        self
    }
    ///Sets the raw value of [`Self::final_layout`]
    pub fn set_final_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.final_layout = value;
        self
    }
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
///   in [`RenderPassCreateInfo2::attachments`], or [`ATTACHMENT_UNUSED`] to signify that this
///   attachment is not used.
/// - [`layout`] is a [`ImageLayout`] value specifying the layout the attachment uses during the
///   subpass.
/// - [`aspect_mask`] is a mask of which aspect(s)  **can**  be accessed within the specified
///   subpass as an input attachment.
///# Description
///Parameters defined by this structure with the same name as those in
///[`AttachmentReference`] have the identical effect to those parameters.[`aspect_mask`] is ignored
/// when this structure is used to describe anything
///other than an input attachment reference.If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts) feature is enabled, and [`attachment`]
///has a depth/stencil format, [`layout`] **can**  be set to a layout that only
///specifies the layout of the depth aspect.If [`layout`] only specifies the layout of the depth
/// aspect of the
///attachment, the layout of the stencil aspect is specified by the
///`stencilLayout` member of a [`AttachmentReferenceStencilLayout`]
///structure included in the [`p_next`] chain.
///Otherwise, [`layout`] describes the layout for all relevant image aspects.
///## Valid Usage
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], [`layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_UNDEFINED`, `VK_IMAGE_LAYOUT_PREINITIALIZED`, or
///   `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
/// - If the [`separateDepthStencilLayouts`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-separateDepthStencilLayouts)
///   feature is not enabled, and [`attachment`] is not [`ATTACHMENT_UNUSED`], [`layout`] **must**
///   not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`,
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is
///   a color format, [`layout`] **must**  not be `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or
///   `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is
///   a depth/stencil format which includes both depth and stencil aspects, and [`layout`] is
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, the
///   [`p_next`] chain  **must**  include a [`AttachmentReferenceStencilLayout`] structure
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is
///   a depth/stencil format which includes only the depth aspect, [`layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`
/// - If [`attachment`] is not [`ATTACHMENT_UNUSED`], and the format of the referenced attachment is
///   a depth/stencil format which includes only the stencil aspect, [`layout`] **must**  not be
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL` or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`AttachmentReferenceStencilLayout`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`layout`] **must**  be a valid [`ImageLayout`] value
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
#[doc(alias = "VkAttachmentReference2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AttachmentReference2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`attachment`] is either an integer value identifying an attachment at
    ///the corresponding index in
    ///[`RenderPassCreateInfo2`]::`pAttachments`, or
    ///[`ATTACHMENT_UNUSED`] to signify that this attachment is not used.
    pub attachment: u32,
    ///[`layout`] is a [`ImageLayout`] value specifying the layout the
    ///attachment uses during the subpass.
    pub layout: ImageLayout,
    ///[`aspect_mask`] is a mask of which aspect(s)  **can**  be accessed within
    ///the specified subpass as an input attachment.
    pub aspect_mask: ImageAspectFlags,
}
impl<'lt> Default for AttachmentReference2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::AttachmentReference2,
            p_next: std::ptr::null(),
            attachment: 0,
            layout: Default::default(),
            aspect_mask: Default::default(),
        }
    }
}
impl<'lt> AttachmentReference2<'lt> {
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
    ///Gets the value of [`Self::attachment`]
    pub fn attachment(&self) -> u32 {
        self.attachment
    }
    ///Gets the value of [`Self::layout`]
    pub fn layout(&self) -> ImageLayout {
        self.layout
    }
    ///Gets the value of [`Self::aspect_mask`]
    pub fn aspect_mask(&self) -> ImageAspectFlags {
        self.aspect_mask
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::attachment`]
    pub fn attachment_mut(&mut self) -> &mut u32 {
        &mut self.attachment
    }
    ///Gets a mutable reference to the value of [`Self::layout`]
    pub fn layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.layout
    }
    ///Gets a mutable reference to the value of [`Self::aspect_mask`]
    pub fn aspect_mask_mut(&mut self) -> &mut ImageAspectFlags {
        &mut self.aspect_mask
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
    ///Sets the raw value of [`Self::attachment`]
    pub fn set_attachment(&mut self, value: u32) -> &mut Self {
        self.attachment = value;
        self
    }
    ///Sets the raw value of [`Self::layout`]
    pub fn set_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.layout = value;
        self
    }
    ///Sets the raw value of [`Self::aspect_mask`]
    pub fn set_aspect_mask(&mut self, value: crate::vulkan1_0::ImageAspectFlags) -> &mut Self {
        self.aspect_mask = value;
        self
    }
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
/// - [`input_attachments`] is a pointer to an array of [`AttachmentReference2`] structures defining
///   the input attachments for this subpass and their layouts.
/// - [`color_attachment_count`] is the number of color attachments.
/// - [`color_attachments`] is a pointer to an array of
///   [`color_attachment_count`][`AttachmentReference2`] structures defining the color attachments
///   for this subpass and their layouts.
/// - [`resolve_attachments`] is `NULL` or a pointer to an array of
///   [`color_attachment_count`][`AttachmentReference2`] structures defining the resolve attachments
///   for this subpass and their layouts.
/// - [`depth_stencil_attachment`] is a pointer to a [`AttachmentReference2`] structure specifying
///   the depth/stencil attachment for this subpass and its layout.
/// - [`preserve_attachment_count`] is the number of preserved attachments.
/// - [`preserve_attachments`] is a pointer to an array of [`preserve_attachment_count`] render pass
///   attachment indices identifying attachments that are not used by this subpass, but whose
///   contents  **must**  be preserved throughout the subpass.
///# Description
///Parameters defined by this structure with the same name as those in
///[`SubpassDescription`] have the identical effect to those parameters.[`view_mask`] has the same
/// effect for the described subpass as
///[`RenderPassMultiviewCreateInfo::view_masks`] has on each
///corresponding subpass.If a [`FragmentShadingRateAttachmentInfoKHR`] structure is included in
///the [`p_next`] chain, `pFragmentShadingRateAttachment` is not `NULL`,
///and its `attachment` member is not [`ATTACHMENT_UNUSED`], the
///identified attachment defines a fragment shading rate attachment for that
///subpass.
///## Valid Usage
/// - [`pipeline_bind_point`] **must**  be `VK_PIPELINE_BIND_POINT_GRAPHICS` or
///   `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`
/// - [`color_attachment_count`] **must**  be less than or equal to
///   [`PhysicalDeviceLimits::max_color_attachments`]
/// - If the first use of an attachment in this render pass is as an input attachment, and the
///   attachment is not also used as a color or depth/stencil attachment in the same subpass, then
///   `loadOp` **must**  not be `VK_ATTACHMENT_LOAD_OP_CLEAR`
/// - If [`resolve_attachments`] is not `NULL`, for each resolve attachment that does not have the
///   value [`ATTACHMENT_UNUSED`], the corresponding color attachment  **must**  not have the value
///   [`ATTACHMENT_UNUSED`]
/// - If [`resolve_attachments`] is not `NULL`, for each resolve attachment that is not
///   [`ATTACHMENT_UNUSED`], the corresponding color attachment  **must**  not have a sample count
///   of `VK_SAMPLE_COUNT_1_BIT`
/// - If [`resolve_attachments`] is not `NULL`, each resolve attachment that is not
///   [`ATTACHMENT_UNUSED`] **must**  have a sample count of `VK_SAMPLE_COUNT_1_BIT`
/// - Any given element of [`resolve_attachments`] **must**  have the same [`Format`] as its
///   corresponding color attachment
/// - All attachments in [`color_attachments`] that are not [`ATTACHMENT_UNUSED`] **must**  have the
///   same sample count
/// -    All attachments in [`input_attachments`] that are not [`ATTACHMENT_UNUSED`] **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain at least `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT` or `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// -    All attachments in [`color_attachments`] that are not [`ATTACHMENT_UNUSED`] **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
/// -    All attachments in [`resolve_attachments`] that are not [`ATTACHMENT_UNUSED`] **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT`
/// -    If [`depth_stencil_attachment`] is not `NULL` and the attachment is not [`ATTACHMENT_UNUSED`] then it  **must**  have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// -    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`input_attachments`] that are not [`ATTACHMENT_UNUSED`] **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
/// -    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`color_attachments`] that are not [`ATTACHMENT_UNUSED`] **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
/// -    If the [`linearColorAttachment`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-linearColorAttachment) feature is enabled and the image is created with `VK_IMAGE_TILING_LINEAR`, all attachments in [`resolve_attachments`] that are not [`ATTACHMENT_UNUSED`] **must**  have image formats whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) **must**  contain `VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV`
/// - If the `[`VK_AMD_mixed_attachment_samples`]` extension is enabled, all attachments in
///   [`color_attachments`] that are not [`ATTACHMENT_UNUSED`] **must**  have a sample count that is
///   smaller than or equal to the sample count of [`depth_stencil_attachment`] if it is not
///   [`ATTACHMENT_UNUSED`]
/// - If neither the `[`VK_AMD_mixed_attachment_samples`]` nor the
///   `[`VK_NV_framebuffer_mixed_samples`]` extensions are enabled, and if
///   [`depth_stencil_attachment`] is not [`ATTACHMENT_UNUSED`] and any attachments in
///   [`color_attachments`] are not [`ATTACHMENT_UNUSED`], they  **must**  have the same sample
///   count
/// - Each element of [`preserve_attachments`] **must**  not be [`ATTACHMENT_UNUSED`]
/// - Any given element of [`preserve_attachments`] **must**  not also be an element of any other
///   member of the subpass description
/// - If any attachment is used by more than one [`AttachmentReference2`] member, then each use
///   **must**  use the same `layout`
/// - Attachments  **must**  follow the [image layout requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#attachment-type-imagelayout)
///   based on the type of attachment it is being used as
/// - If [`flags`] includes `VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX`, it  **must**
///   also include `VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX`
/// - If the `attachment` member of any element of [`input_attachments`] is not
///   [`ATTACHMENT_UNUSED`], then the `aspectMask` member  **must**  be a valid combination of
///   [`ImageAspectFlagBits`]
/// - If the `attachment` member of any element of [`input_attachments`] is not
///   [`ATTACHMENT_UNUSED`], then the `aspectMask` member  **must**  not be `0`
/// - If the `attachment` member of any element of [`input_attachments`] is not
///   [`ATTACHMENT_UNUSED`], then the `aspectMask` member  **must**  not include
///   `VK_IMAGE_ASPECT_METADATA_BIT`
/// - If the `attachment` member of any element of [`input_attachments`] is not
///   [`ATTACHMENT_UNUSED`], then the `aspectMask` member  **must**  not include
///   `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` for any index *i*
/// - An attachment  **must**  not be used in both [`depth_stencil_attachment`] and
///   [`color_attachments`]
/// - If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview)
///   feature is not enabled, [`view_mask`] **must**  be `0`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`FragmentShadingRateAttachmentInfoKHR`]
///   or [`SubpassDescriptionDepthStencilResolve`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`SubpassDescriptionFlagBits`] values
/// - [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
/// - If [`input_attachment_count`] is not `0`, [`input_attachments`] **must**  be a valid pointer
///   to an array of [`input_attachment_count`] valid [`AttachmentReference2`] structures
/// - If [`color_attachment_count`] is not `0`, [`color_attachments`] **must**  be a valid pointer
///   to an array of [`color_attachment_count`] valid [`AttachmentReference2`] structures
/// - If [`color_attachment_count`] is not `0`, and [`resolve_attachments`] is not `NULL`,
///   [`resolve_attachments`] **must**  be a valid pointer to an array of [`color_attachment_count`]
///   valid [`AttachmentReference2`] structures
/// - If [`depth_stencil_attachment`] is not `NULL`, [`depth_stencil_attachment`] **must**  be a
///   valid pointer to a valid [`AttachmentReference2`] structure
/// - If [`preserve_attachment_count`] is not `0`, [`preserve_attachments`] **must**  be a valid
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
#[doc(alias = "VkSubpassDescription2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SubpassDescription2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`SubpassDescriptionFlagBits`]
    ///specifying usage of the subpass.
    pub flags: SubpassDescriptionFlags,
    ///[`pipeline_bind_point`] is a [`PipelineBindPoint`] value specifying
    ///the pipeline type supported for this subpass.
    pub pipeline_bind_point: PipelineBindPoint,
    ///[`view_mask`] is a bitfield of view indices describing which views
    ///rendering is broadcast to in this subpass, when multiview is enabled.
    pub view_mask: u32,
    ///[`input_attachment_count`] is the number of input attachments.
    pub input_attachment_count: u32,
    ///[`input_attachments`] is a pointer to an array of
    ///[`AttachmentReference2`] structures defining the input attachments
    ///for this subpass and their layouts.
    pub input_attachments: *const AttachmentReference2<'lt>,
    ///[`color_attachment_count`] is the number of color attachments.
    pub color_attachment_count: u32,
    ///[`color_attachments`] is a pointer to an array of
    ///[`color_attachment_count`][`AttachmentReference2`] structures
    ///defining the color attachments for this subpass and their layouts.
    pub color_attachments: *const AttachmentReference2<'lt>,
    ///[`resolve_attachments`] is `NULL` or a pointer to an array of
    ///[`color_attachment_count`][`AttachmentReference2`] structures
    ///defining the resolve attachments for this subpass and their layouts.
    pub resolve_attachments: *const AttachmentReference2<'lt>,
    ///[`depth_stencil_attachment`] is a pointer to a
    ///[`AttachmentReference2`] structure specifying the depth/stencil
    ///attachment for this subpass and its layout.
    pub depth_stencil_attachment: *const AttachmentReference2<'lt>,
    ///[`preserve_attachment_count`] is the number of preserved attachments.
    pub preserve_attachment_count: u32,
    ///[`preserve_attachments`] is a pointer to an array of
    ///[`preserve_attachment_count`] render pass attachment indices identifying
    ///attachments that are not used by this subpass, but whose contents  **must**
    ///be preserved throughout the subpass.
    pub preserve_attachments: *const u32,
}
impl<'lt> Default for SubpassDescription2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SubpassDescription2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            pipeline_bind_point: Default::default(),
            view_mask: 0,
            input_attachment_count: 0,
            input_attachments: std::ptr::null(),
            color_attachment_count: 0,
            color_attachments: std::ptr::null(),
            resolve_attachments: std::ptr::null(),
            depth_stencil_attachment: std::ptr::null(),
            preserve_attachment_count: 0,
            preserve_attachments: std::ptr::null(),
        }
    }
}
impl<'lt> SubpassDescription2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::input_attachments`]
    pub fn input_attachments_raw(&self) -> *const AttachmentReference2<'lt> {
        self.input_attachments
    }
    ///Gets the raw value of [`Self::color_attachments`]
    pub fn color_attachments_raw(&self) -> *const AttachmentReference2<'lt> {
        self.color_attachments
    }
    ///Gets the raw value of [`Self::resolve_attachments`]
    pub fn resolve_attachments_raw(&self) -> *const AttachmentReference2<'lt> {
        self.resolve_attachments
    }
    ///Gets the raw value of [`Self::depth_stencil_attachment`]
    pub fn depth_stencil_attachment_raw(&self) -> *const AttachmentReference2<'lt> {
        self.depth_stencil_attachment
    }
    ///Gets the raw value of [`Self::preserve_attachments`]
    pub fn preserve_attachments_raw(&self) -> *const u32 {
        self.preserve_attachments
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::input_attachments`]
    pub fn set_input_attachments_raw(&mut self, value: *const AttachmentReference2<'lt>) -> &mut Self {
        self.input_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachments`]
    pub fn set_color_attachments_raw(&mut self, value: *const AttachmentReference2<'lt>) -> &mut Self {
        self.color_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::resolve_attachments`]
    pub fn set_resolve_attachments_raw(&mut self, value: *const AttachmentReference2<'lt>) -> &mut Self {
        self.resolve_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::depth_stencil_attachment`]
    pub fn set_depth_stencil_attachment_raw(&mut self, value: *const AttachmentReference2<'lt>) -> &mut Self {
        self.depth_stencil_attachment = value;
        self
    }
    ///Sets the raw value of [`Self::preserve_attachments`]
    pub fn set_preserve_attachments_raw(&mut self, value: *const u32) -> &mut Self {
        self.preserve_attachments = value;
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
    pub fn flags(&self) -> SubpassDescriptionFlags {
        self.flags
    }
    ///Gets the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Gets the value of [`Self::view_mask`]
    pub fn view_mask(&self) -> u32 {
        self.view_mask
    }
    ///Gets the value of [`Self::input_attachment_count`]
    pub fn input_attachment_count(&self) -> u32 {
        self.input_attachment_count
    }
    ///Gets the value of [`Self::input_attachments`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn input_attachments(&self) -> &[AttachmentReference2<'lt>] {
        std::slice::from_raw_parts(self.input_attachments, self.input_attachment_count as usize)
    }
    ///Gets the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count(&self) -> u32 {
        self.color_attachment_count
    }
    ///Gets the value of [`Self::color_attachments`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn color_attachments(&self) -> &[AttachmentReference2<'lt>] {
        std::slice::from_raw_parts(self.color_attachments, self.color_attachment_count as usize)
    }
    ///Gets the value of [`Self::resolve_attachments`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn resolve_attachments(&self) -> &[AttachmentReference2<'lt>] {
        std::slice::from_raw_parts(self.resolve_attachments, self.color_attachment_count as usize)
    }
    ///Gets the value of [`Self::depth_stencil_attachment`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn depth_stencil_attachment(&self) -> &AttachmentReference2<'lt> {
        &*self.depth_stencil_attachment
    }
    ///Gets the value of [`Self::preserve_attachment_count`]
    pub fn preserve_attachment_count(&self) -> u32 {
        self.preserve_attachment_count
    }
    ///Gets the value of [`Self::preserve_attachments`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn preserve_attachments(&self) -> &[u32] {
        std::slice::from_raw_parts(self.preserve_attachments, self.preserve_attachment_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut SubpassDescriptionFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Gets a mutable reference to the value of [`Self::view_mask`]
    pub fn view_mask_mut(&mut self) -> &mut u32 {
        &mut self.view_mask
    }
    ///Gets a mutable reference to the value of [`Self::input_attachment_count`]
    pub fn input_attachment_count_mut(&mut self) -> &mut u32 {
        &mut self.input_attachment_count
    }
    ///Gets a mutable reference to the value of [`Self::color_attachment_count`]
    pub fn color_attachment_count_mut(&mut self) -> &mut u32 {
        &mut self.color_attachment_count
    }
    ///Gets a mutable reference to the value of [`Self::preserve_attachment_count`]
    pub fn preserve_attachment_count_mut(&mut self) -> &mut u32 {
        &mut self.preserve_attachment_count
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
    pub fn set_flags(&mut self, value: crate::vulkan1_0::SubpassDescriptionFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_bind_point`]
    pub fn set_pipeline_bind_point(&mut self, value: crate::vulkan1_0::PipelineBindPoint) -> &mut Self {
        self.pipeline_bind_point = value;
        self
    }
    ///Sets the raw value of [`Self::view_mask`]
    pub fn set_view_mask(&mut self, value: u32) -> &mut Self {
        self.view_mask = value;
        self
    }
    ///Sets the raw value of [`Self::input_attachment_count`]
    pub fn set_input_attachment_count(&mut self, value: u32) -> &mut Self {
        self.input_attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::input_attachments`]
    pub fn set_input_attachments(&mut self, value: &'lt [crate::vulkan1_2::AttachmentReference2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.input_attachments = value.as_ptr();
        self.input_attachment_count = len_;
        self
    }
    ///Sets the raw value of [`Self::color_attachment_count`]
    pub fn set_color_attachment_count(&mut self, value: u32) -> &mut Self {
        self.color_attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::color_attachments`]
    pub fn set_color_attachments(&mut self, value: &'lt [crate::vulkan1_2::AttachmentReference2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.color_attachments = value.as_ptr();
        self.color_attachment_count = len_;
        self
    }
    ///Sets the raw value of [`Self::resolve_attachments`]
    pub fn set_resolve_attachments(&mut self, value: &'lt [crate::vulkan1_2::AttachmentReference2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.resolve_attachments = value.as_ptr();
        self.color_attachment_count = len_;
        self
    }
    ///Sets the raw value of [`Self::depth_stencil_attachment`]
    pub fn set_depth_stencil_attachment(
        &mut self,
        value: &'lt crate::vulkan1_2::AttachmentReference2<'lt>,
    ) -> &mut Self {
        self.depth_stencil_attachment = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::preserve_attachment_count`]
    pub fn set_preserve_attachment_count(&mut self, value: u32) -> &mut Self {
        self.preserve_attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::preserve_attachments`]
    pub fn set_preserve_attachments(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.preserve_attachments = value.as_ptr();
        self.preserve_attachment_count = len_;
        self
    }
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
///[`RenderPassMultiviewCreateInfo::view_offsets`] has on each
///corresponding subpass dependency.If a [`MemoryBarrier2`] is included in the [`p_next`] chain,
///[`src_stage_mask`], [`dst_stage_mask`], [`src_access_mask`], and
///[`dst_access_mask`] parameters are ignored.
///The synchronization and access scopes instead are defined by the parameters
///of [`MemoryBarrier2`].
///## Valid Usage
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`src_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`src_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`src_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`src_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`src_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`src_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`src_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`src_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-synchronization2)
///   feature is not enabled, [`src_stage_mask`] **must**  not be `0`
///
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`dst_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`dst_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`dst_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`dst_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`dst_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`dst_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`dst_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`dst_stage_mask`] **must**  not contain
///   `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-synchronization2)
///   feature is not enabled, [`dst_stage_mask`] **must**  not be `0`
/// - [`src_subpass`] **must**  be less than or equal to [`dst_subpass`], unless one of them is
///   [`SUBPASS_EXTERNAL`], to avoid cyclic dependencies and ensure a valid execution order
/// - [`src_subpass`] and [`dst_subpass`] **must**  not both be equal to [`SUBPASS_EXTERNAL`]
/// - If [`src_subpass`] is equal to [`dst_subpass`] and not all of the stages in [`src_stage_mask`]
///   and [`dst_stage_mask`] are [framebuffer-space stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions),
///   the [logically latest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order)
///   pipeline stage in [`src_stage_mask`] **must**  be [logically earlier](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order)
///   than or equal to the [logically earliest](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-order)
///   pipeline stage in [`dst_stage_mask`]
/// -    Any access flag included in [`src_access_mask`] **must**  be supported by one of the pipeline stages in [`src_stage_mask`], as specified in the [table of supported access types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types-supported)
/// -    Any access flag included in [`dst_access_mask`] **must**  be supported by one of the pipeline stages in [`dst_stage_mask`], as specified in the [table of supported access types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types-supported)
/// - If [`dependency_flags`] includes `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`src_subpass`] **must**  not
///   be equal to [`SUBPASS_EXTERNAL`]
/// - If [`dependency_flags`] includes `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`dst_subpass`] **must**  not
///   be equal to [`SUBPASS_EXTERNAL`]
/// -    If [`src_subpass`] equals [`dst_subpass`], and [`src_stage_mask`] and [`dst_stage_mask`] both include a [framebuffer-space stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-framebuffer-regions), then [`dependency_flags`] **must**  include `VK_DEPENDENCY_BY_REGION_BIT`
/// - If [`view_offset`] is not equal to `0`, [`src_subpass`] **must**  not be equal to
///   [`dst_subpass`]
/// - If [`dependency_flags`] does not include `VK_DEPENDENCY_VIEW_LOCAL_BIT`, [`view_offset`]
///   **must**  be `0`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`MemoryBarrier2`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`src_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
/// - [`dst_stage_mask`] **must**  be a valid combination of [`PipelineStageFlagBits`] values
/// - [`src_access_mask`] **must**  be a valid combination of [`AccessFlagBits`] values
/// - [`dst_access_mask`] **must**  be a valid combination of [`AccessFlagBits`] values
/// - [`dependency_flags`] **must**  be a valid combination of [`DependencyFlagBits`] values
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
#[doc(alias = "VkSubpassDependency2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SubpassDependency2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`src_subpass`] is the subpass index of the first subpass in the
    ///dependency, or [`SUBPASS_EXTERNAL`].
    pub src_subpass: u32,
    ///[`dst_subpass`] is the subpass index of the second subpass in the
    ///dependency, or [`SUBPASS_EXTERNAL`].
    pub dst_subpass: u32,
    ///[`src_stage_mask`] is a bitmask of [`PipelineStageFlagBits`]
    ///specifying the [source stage
    ///mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks).
    pub src_stage_mask: PipelineStageFlags,
    ///[`dst_stage_mask`] is a bitmask of [`PipelineStageFlagBits`]
    ///specifying the [destination
    ///stage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-masks)
    pub dst_stage_mask: PipelineStageFlags,
    ///[`src_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a
    ///[source access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
    pub src_access_mask: AccessFlags,
    ///[`dst_access_mask`] is a bitmask of [`AccessFlagBits`] specifying a
    ///[destination access mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-masks).
    pub dst_access_mask: AccessFlags,
    ///[`dependency_flags`] is a bitmask of [`DependencyFlagBits`].
    pub dependency_flags: DependencyFlags,
    ///[`view_offset`] controls which views in the source subpass the views in
    ///the destination subpass depend on.
    pub view_offset: i32,
}
impl<'lt> Default for SubpassDependency2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SubpassDependency2,
            p_next: std::ptr::null(),
            src_subpass: 0,
            dst_subpass: 0,
            src_stage_mask: Default::default(),
            dst_stage_mask: Default::default(),
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            dependency_flags: Default::default(),
            view_offset: 0,
        }
    }
}
impl<'lt> SubpassDependency2<'lt> {
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
    ///Gets the value of [`Self::src_subpass`]
    pub fn src_subpass(&self) -> u32 {
        self.src_subpass
    }
    ///Gets the value of [`Self::dst_subpass`]
    pub fn dst_subpass(&self) -> u32 {
        self.dst_subpass
    }
    ///Gets the value of [`Self::src_stage_mask`]
    pub fn src_stage_mask(&self) -> PipelineStageFlags {
        self.src_stage_mask
    }
    ///Gets the value of [`Self::dst_stage_mask`]
    pub fn dst_stage_mask(&self) -> PipelineStageFlags {
        self.dst_stage_mask
    }
    ///Gets the value of [`Self::src_access_mask`]
    pub fn src_access_mask(&self) -> AccessFlags {
        self.src_access_mask
    }
    ///Gets the value of [`Self::dst_access_mask`]
    pub fn dst_access_mask(&self) -> AccessFlags {
        self.dst_access_mask
    }
    ///Gets the value of [`Self::dependency_flags`]
    pub fn dependency_flags(&self) -> DependencyFlags {
        self.dependency_flags
    }
    ///Gets the value of [`Self::view_offset`]
    pub fn view_offset(&self) -> i32 {
        self.view_offset
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_subpass`]
    pub fn src_subpass_mut(&mut self) -> &mut u32 {
        &mut self.src_subpass
    }
    ///Gets a mutable reference to the value of [`Self::dst_subpass`]
    pub fn dst_subpass_mut(&mut self) -> &mut u32 {
        &mut self.dst_subpass
    }
    ///Gets a mutable reference to the value of [`Self::src_stage_mask`]
    pub fn src_stage_mask_mut(&mut self) -> &mut PipelineStageFlags {
        &mut self.src_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::dst_stage_mask`]
    pub fn dst_stage_mask_mut(&mut self) -> &mut PipelineStageFlags {
        &mut self.dst_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::src_access_mask`]
    pub fn src_access_mask_mut(&mut self) -> &mut AccessFlags {
        &mut self.src_access_mask
    }
    ///Gets a mutable reference to the value of [`Self::dst_access_mask`]
    pub fn dst_access_mask_mut(&mut self) -> &mut AccessFlags {
        &mut self.dst_access_mask
    }
    ///Gets a mutable reference to the value of [`Self::dependency_flags`]
    pub fn dependency_flags_mut(&mut self) -> &mut DependencyFlags {
        &mut self.dependency_flags
    }
    ///Gets a mutable reference to the value of [`Self::view_offset`]
    pub fn view_offset_mut(&mut self) -> &mut i32 {
        &mut self.view_offset
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
    ///Sets the raw value of [`Self::src_subpass`]
    pub fn set_src_subpass(&mut self, value: u32) -> &mut Self {
        self.src_subpass = value;
        self
    }
    ///Sets the raw value of [`Self::dst_subpass`]
    pub fn set_dst_subpass(&mut self, value: u32) -> &mut Self {
        self.dst_subpass = value;
        self
    }
    ///Sets the raw value of [`Self::src_stage_mask`]
    pub fn set_src_stage_mask(&mut self, value: crate::vulkan1_0::PipelineStageFlags) -> &mut Self {
        self.src_stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::dst_stage_mask`]
    pub fn set_dst_stage_mask(&mut self, value: crate::vulkan1_0::PipelineStageFlags) -> &mut Self {
        self.dst_stage_mask = value;
        self
    }
    ///Sets the raw value of [`Self::src_access_mask`]
    pub fn set_src_access_mask(&mut self, value: crate::vulkan1_0::AccessFlags) -> &mut Self {
        self.src_access_mask = value;
        self
    }
    ///Sets the raw value of [`Self::dst_access_mask`]
    pub fn set_dst_access_mask(&mut self, value: crate::vulkan1_0::AccessFlags) -> &mut Self {
        self.dst_access_mask = value;
        self
    }
    ///Sets the raw value of [`Self::dependency_flags`]
    pub fn set_dependency_flags(&mut self, value: crate::vulkan1_0::DependencyFlags) -> &mut Self {
        self.dependency_flags = value;
        self
    }
    ///Sets the raw value of [`Self::view_offset`]
    pub fn set_view_offset(&mut self, value: i32) -> &mut Self {
        self.view_offset = value;
        self
    }
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
/// - [`attachments`] is a pointer to an array of [`attachment_count`][`AttachmentDescription2`]
///   structures describing the attachments used by the render pass.
/// - [`subpass_count`] is the number of subpasses to create.
/// - [`subpasses`] is a pointer to an array of [`subpass_count`][`SubpassDescription2`] structures
///   describing each subpass.
/// - [`dependency_count`] is the number of dependencies between pairs of subpasses.
/// - [`dependencies`] is a pointer to an array of [`dependency_count`][`SubpassDependency2`]
///   structures describing dependencies between pairs of subpasses.
/// - [`correlated_view_mask_count`] is the number of correlation masks.
/// - [`correlated_view_masks`] is a pointer to an array of view masks indicating sets of views that
///   **may**  be more efficient to render concurrently.
///# Description
///Parameters defined by this structure with the same name as those in
///[`RenderPassCreateInfo`] have the identical effect to those parameters;
///the child structures are variants of those used in
///[`RenderPassCreateInfo`] which add [`s_type`] and [`p_next`]
///parameters, allowing them to be extended.If the [`SubpassDescription2::view_mask`] member of any
/// element of
///[`subpasses`] is not zero, *multiview* functionality is considered to be
///enabled for this render pass.[`correlated_view_mask_count`] and [`correlated_view_masks`] have
/// the same
///effect as [`RenderPassMultiviewCreateInfo::correlation_mask_count`]
///and [`RenderPassMultiviewCreateInfo::correlation_masks`],
///respectively.
///## Valid Usage
/// - If any two subpasses operate on attachments with overlapping ranges of the same
///   [`DeviceMemory`] object, and at least one subpass writes to that area of [`DeviceMemory`], a
///   subpass dependency  **must**  be included (either directly or via some intermediate subpasses)
///   between them
/// - If the `attachment` member of any element of `pInputAttachments`, `pColorAttachments`,
///   `pResolveAttachments` or `pDepthStencilAttachment`, or the attachment indexed by any element
///   of `pPreserveAttachments` in any given element of [`subpasses`] is bound to a range of a
///   [`DeviceMemory`] object that overlaps with any other attachment in any subpass (including the
///   same subpass), the [`AttachmentDescription2`] structures describing them  **must**  include
///   `VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT` in [`flags`]
/// - If the `attachment` member of any element of `pInputAttachments`, `pColorAttachments`,
///   `pResolveAttachments` or `pDepthStencilAttachment`, or any element of `pPreserveAttachments`
///   in any given element of [`subpasses`] is not [`ATTACHMENT_UNUSED`], then it  **must**  be less
///   than [`attachment_count`]
/// - If the pNext chain includes a [`RenderPassFragmentDensityMapCreateInfoEXT`] structure and the
///   `fragmentDensityMapAttachment` member is not [`ATTACHMENT_UNUSED`], then `attachment` **must**
///   be less than [`attachment_count`]
/// - If the [`subpasses`] pNext chain includes a [`SubpassDescriptionDepthStencilResolve`]
///   structure and the `pDepthStencilResolveAttachment` member is not `NULL` and does not have the
///   value [`ATTACHMENT_UNUSED`], then `attachment` **must**  be less than [`attachment_count`]
/// - For any member of [`attachments`] with a `loadOp` equal to `VK_ATTACHMENT_LOAD_OP_CLEAR`, the
///   first use of that attachment  **must**  not specify a `layout` equal to
///   `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   or `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - For any member of [`attachments`] with a `stencilLoadOp` equal to
///   `VK_ATTACHMENT_LOAD_OP_CLEAR`, the first use of that attachment  **must**  not specify a
///   `layout` equal to `VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`
/// -    For any element of [`dependencies`], if the `srcSubpass` is not [`SUBPASS_EXTERNAL`], all stage flags included in the `srcStageMask` member of that dependency  **must**  be a pipeline stage supported by the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-types) identified by the `pipelineBindPoint` member of the source subpass
/// -    For any element of [`dependencies`], if the `dstSubpass` is not [`SUBPASS_EXTERNAL`], all stage flags included in the `dstStageMask` member of that dependency  **must**  be a pipeline stage supported by the [pipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages-types) identified by the `pipelineBindPoint` member of the destination subpass
/// - The set of bits included in any element of [`correlated_view_masks`] **must**  not overlap
///   with the set of bits included in any other element of [`correlated_view_masks`]
/// - If the [`SubpassDescription2::view_mask`] member of all elements of [`subpasses`] is `0`,
///   [`correlated_view_mask_count`] **must**  be `0`
/// - The [`SubpassDescription2::view_mask`] member of all elements of [`subpasses`] **must**
///   either all be `0`, or all not be `0`
/// - If the [`SubpassDescription2::view_mask`] member of all elements of [`subpasses`] is `0`, the
///   `dependencyFlags` member of any element of [`dependencies`] **must**  not include
///   `VK_DEPENDENCY_VIEW_LOCAL_BIT`
/// - For any element of [`dependencies`] where its `srcSubpass` member equals its `dstSubpass`
///   member, if the `viewMask` member of the corresponding element of [`subpasses`] includes more
///   than one bit, its `dependencyFlags` member  **must**  include `VK_DEPENDENCY_VIEW_LOCAL_BIT`
/// - If the `attachment` member of any element of the `pInputAttachments` member of any element of
///   [`subpasses`] is not [`ATTACHMENT_UNUSED`], the `aspectMask` member of that element of
///   `pInputAttachments` **must**  only include aspects that are present in images of the format
///   specified by the element of [`attachments`] specified by `attachment`
/// - The `srcSubpass` member of each element of [`dependencies`] **must**  be less than
///   [`subpass_count`]
/// - The `dstSubpass` member of each element of [`dependencies`] **must**  be less than
///   [`subpass_count`]
/// - If any element of [`attachments`] is used as a fragment shading rate attachment in any
///   subpass, it  **must**  not be used as any other attachment in the render pass
/// - If [`flags`] includes `VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM`, an element of [`subpasses`]
///   includes an instance of [`FragmentShadingRateAttachmentInfoKHR`] in its [`p_next`] chain, and
///   the `pFragmentShadingRateAttachment` member of that structure is not equal to `NULL`, the
///   `attachment` member of `pFragmentShadingRateAttachment` **must**  be [`ATTACHMENT_UNUSED`]
/// -    If any element of [`attachments`] is used as a fragment shading rate attachment in any subpass, it  **must**  have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
/// - If the pipeline is being created with fragment shader state, and the
///   [`VK_QCOM_render_pass_shader_resolve`] extension is enabled, and if subpass has any input
///   attachments, and if the subpass description contains
///   `VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM`, then the sample count of the input
///   attachments  **must**  equal `rasterizationSamples`
/// - If the pipeline is being created with fragment shader state, and the
///   [`VK_QCOM_render_pass_shader_resolve`] extension is enabled, and if the subpass description
///   contains `VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM`, then `sampleShadingEnable`
///   **must**  be false
/// - If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, and if
///   `pResolveAttachments` is not `NULL`, then each resolve attachment  **must**  be
///   [`ATTACHMENT_UNUSED`]
/// - If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, and if
///   `pDepthStencilResolveAttachment` is not `NULL`, then the depth/stencil resolve attachment
///   **must**  be [`ATTACHMENT_UNUSED`]
/// - If [`flags`] includes `VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM`, then the subpass
///   **must**  be the last subpass in a subpass dependency chain
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`RenderPassFragmentDensityMapCreateInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`flags`] **must**  be a valid combination of [`RenderPassCreateFlagBits`] values
/// - If [`attachment_count`] is not `0`, [`attachments`] **must**  be a valid pointer to an array
///   of [`attachment_count`] valid [`AttachmentDescription2`] structures
/// - [`subpasses`] **must**  be a valid pointer to an array of [`subpass_count`] valid
///   [`SubpassDescription2`] structures
/// - If [`dependency_count`] is not `0`, [`dependencies`] **must**  be a valid pointer to an array
///   of [`dependency_count`] valid [`SubpassDependency2`] structures
/// - If [`correlated_view_mask_count`] is not `0`, [`correlated_view_masks`] **must**  be a valid
///   pointer to an array of [`correlated_view_mask_count`]`uint32_t` values
/// - [`subpass_count`] **must**  be greater than `0`
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
#[doc(alias = "VkRenderPassCreateInfo2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct RenderPassCreateInfo2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: RenderPassCreateFlags,
    ///[`attachment_count`] is the number of attachments used by this render
    ///pass.
    pub attachment_count: u32,
    ///[`attachments`] is a pointer to an array of [`attachment_count`][`AttachmentDescription2`]
    /// structures describing the attachments used by the render pass.
    pub attachments: *const AttachmentDescription2<'lt>,
    ///[`subpass_count`] is the number of subpasses to create.
    pub subpass_count: u32,
    ///[`subpasses`] is a pointer to an array of [`subpass_count`][`SubpassDescription2`]
    /// structures describing each subpass.
    pub subpasses: *const SubpassDescription2<'lt>,
    ///[`dependency_count`] is the number of dependencies between pairs of
    ///subpasses.
    pub dependency_count: u32,
    ///[`dependencies`] is a pointer to an array of [`dependency_count`][`SubpassDependency2`]
    /// structures describing dependencies between pairs of subpasses.
    pub dependencies: *const SubpassDependency2<'lt>,
    ///[`correlated_view_mask_count`] is the number of correlation masks.
    pub correlated_view_mask_count: u32,
    ///[`correlated_view_masks`] is a pointer to an array of view masks
    ///indicating sets of views that  **may**  be more efficient to render
    ///concurrently.
    pub correlated_view_masks: *const u32,
}
impl<'lt> Default for RenderPassCreateInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RenderPassCreateInfo2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            attachment_count: 0,
            attachments: std::ptr::null(),
            subpass_count: 0,
            subpasses: std::ptr::null(),
            dependency_count: 0,
            dependencies: std::ptr::null(),
            correlated_view_mask_count: 0,
            correlated_view_masks: std::ptr::null(),
        }
    }
}
impl<'lt> RenderPassCreateInfo2<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::attachments`]
    pub fn attachments_raw(&self) -> *const AttachmentDescription2<'lt> {
        self.attachments
    }
    ///Gets the raw value of [`Self::subpasses`]
    pub fn subpasses_raw(&self) -> *const SubpassDescription2<'lt> {
        self.subpasses
    }
    ///Gets the raw value of [`Self::dependencies`]
    pub fn dependencies_raw(&self) -> *const SubpassDependency2<'lt> {
        self.dependencies
    }
    ///Gets the raw value of [`Self::correlated_view_masks`]
    pub fn correlated_view_masks_raw(&self) -> *const u32 {
        self.correlated_view_masks
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::attachments`]
    pub fn set_attachments_raw(&mut self, value: *const AttachmentDescription2<'lt>) -> &mut Self {
        self.attachments = value;
        self
    }
    ///Sets the raw value of [`Self::subpasses`]
    pub fn set_subpasses_raw(&mut self, value: *const SubpassDescription2<'lt>) -> &mut Self {
        self.subpasses = value;
        self
    }
    ///Sets the raw value of [`Self::dependencies`]
    pub fn set_dependencies_raw(&mut self, value: *const SubpassDependency2<'lt>) -> &mut Self {
        self.dependencies = value;
        self
    }
    ///Sets the raw value of [`Self::correlated_view_masks`]
    pub fn set_correlated_view_masks_raw(&mut self, value: *const u32) -> &mut Self {
        self.correlated_view_masks = value;
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
    pub fn flags(&self) -> RenderPassCreateFlags {
        self.flags
    }
    ///Gets the value of [`Self::attachment_count`]
    pub fn attachment_count(&self) -> u32 {
        self.attachment_count
    }
    ///Gets the value of [`Self::attachments`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn attachments(&self) -> &[AttachmentDescription2<'lt>] {
        std::slice::from_raw_parts(self.attachments, self.attachment_count as usize)
    }
    ///Gets the value of [`Self::subpass_count`]
    pub fn subpass_count(&self) -> u32 {
        self.subpass_count
    }
    ///Gets the value of [`Self::subpasses`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn subpasses(&self) -> &[SubpassDescription2<'lt>] {
        std::slice::from_raw_parts(self.subpasses, self.subpass_count as usize)
    }
    ///Gets the value of [`Self::dependency_count`]
    pub fn dependency_count(&self) -> u32 {
        self.dependency_count
    }
    ///Gets the value of [`Self::dependencies`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn dependencies(&self) -> &[SubpassDependency2<'lt>] {
        std::slice::from_raw_parts(self.dependencies, self.dependency_count as usize)
    }
    ///Gets the value of [`Self::correlated_view_mask_count`]
    pub fn correlated_view_mask_count(&self) -> u32 {
        self.correlated_view_mask_count
    }
    ///Gets the value of [`Self::correlated_view_masks`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn correlated_view_masks(&self) -> &[u32] {
        std::slice::from_raw_parts(self.correlated_view_masks, self.correlated_view_mask_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut RenderPassCreateFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::attachment_count`]
    pub fn attachment_count_mut(&mut self) -> &mut u32 {
        &mut self.attachment_count
    }
    ///Gets a mutable reference to the value of [`Self::subpass_count`]
    pub fn subpass_count_mut(&mut self) -> &mut u32 {
        &mut self.subpass_count
    }
    ///Gets a mutable reference to the value of [`Self::dependency_count`]
    pub fn dependency_count_mut(&mut self) -> &mut u32 {
        &mut self.dependency_count
    }
    ///Gets a mutable reference to the value of [`Self::correlated_view_mask_count`]
    pub fn correlated_view_mask_count_mut(&mut self) -> &mut u32 {
        &mut self.correlated_view_mask_count
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
    pub fn set_flags(&mut self, value: crate::vulkan1_0::RenderPassCreateFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::attachment_count`]
    pub fn set_attachment_count(&mut self, value: u32) -> &mut Self {
        self.attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::attachments`]
    pub fn set_attachments(&mut self, value: &'lt [crate::vulkan1_2::AttachmentDescription2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.attachments = value.as_ptr();
        self.attachment_count = len_;
        self
    }
    ///Sets the raw value of [`Self::subpass_count`]
    pub fn set_subpass_count(&mut self, value: u32) -> &mut Self {
        self.subpass_count = value;
        self
    }
    ///Sets the raw value of [`Self::subpasses`]
    pub fn set_subpasses(&mut self, value: &'lt [crate::vulkan1_2::SubpassDescription2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.subpasses = value.as_ptr();
        self.subpass_count = len_;
        self
    }
    ///Sets the raw value of [`Self::dependency_count`]
    pub fn set_dependency_count(&mut self, value: u32) -> &mut Self {
        self.dependency_count = value;
        self
    }
    ///Sets the raw value of [`Self::dependencies`]
    pub fn set_dependencies(&mut self, value: &'lt [crate::vulkan1_2::SubpassDependency2<'lt>]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.dependencies = value.as_ptr();
        self.dependency_count = len_;
        self
    }
    ///Sets the raw value of [`Self::correlated_view_mask_count`]
    pub fn set_correlated_view_mask_count(&mut self, value: u32) -> &mut Self {
        self.correlated_view_mask_count = value;
        self
    }
    ///Sets the raw value of [`Self::correlated_view_masks`]
    pub fn set_correlated_view_masks(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.correlated_view_masks = value.as_ptr();
        self.correlated_view_mask_count = len_;
        self
    }
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO`
/// - [`p_next`] **must**  be `NULL`
/// - [`contents`] **must**  be a valid [`SubpassContents`] value
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
#[doc(alias = "VkSubpassBeginInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SubpassBeginInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`contents`] is a [`SubpassContents`] value specifying how the
    ///commands in the next subpass will be provided.
    pub contents: SubpassContents,
}
impl<'lt> Default for SubpassBeginInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SubpassBeginInfo,
            p_next: std::ptr::null(),
            contents: Default::default(),
        }
    }
}
impl<'lt> SubpassBeginInfo<'lt> {
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
    ///Gets the value of [`Self::contents`]
    pub fn contents(&self) -> SubpassContents {
        self.contents
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::contents`]
    pub fn contents_mut(&mut self) -> &mut SubpassContents {
        &mut self.contents
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
    ///Sets the raw value of [`Self::contents`]
    pub fn set_contents(&mut self, value: crate::vulkan1_0::SubpassContents) -> &mut Self {
        self.contents = value;
        self
    }
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_END_INFO`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`SubpassFragmentDensityMapOffsetEndInfoQCOM`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
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
#[doc(alias = "VkSubpassEndInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SubpassEndInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
}
impl<'lt> Default for SubpassEndInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SubpassEndInfo,
            p_next: std::ptr::null(),
        }
    }
}
impl<'lt> SubpassEndInfo<'lt> {
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
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
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
///[`PhysicalDeviceTimelineSemaphoreFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`timeline_semaphore`]
    ///indicates whether semaphores created with a [`SemaphoreType`] of
    ///`VK_SEMAPHORE_TYPE_TIMELINE` are supported.
    pub timeline_semaphore: Bool32,
}
impl<'lt> Default for PhysicalDeviceTimelineSemaphoreFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceTimelineSemaphoreFeatures,
            p_next: std::ptr::null_mut(),
            timeline_semaphore: 0,
        }
    }
}
impl<'lt> PhysicalDeviceTimelineSemaphoreFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::timeline_semaphore`]
    pub fn timeline_semaphore_raw(&self) -> Bool32 {
        self.timeline_semaphore
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::timeline_semaphore`]
    pub fn set_timeline_semaphore_raw(&mut self, value: Bool32) -> &mut Self {
        self.timeline_semaphore = value;
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
    ///Gets the value of [`Self::timeline_semaphore`]
    pub fn timeline_semaphore(&self) -> bool {
        unsafe { std::mem::transmute(self.timeline_semaphore as u8) }
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
    ///Gets a mutable reference to the value of [`Self::timeline_semaphore`]
    pub fn timeline_semaphore_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.timeline_semaphore as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.timeline_semaphore as *mut Bool32)
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
    ///Sets the raw value of [`Self::timeline_semaphore`]
    pub fn set_timeline_semaphore(&mut self, value: bool) -> &mut Self {
        self.timeline_semaphore = value as u8 as u32;
        self
    }
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
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES`
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
#[doc(alias = "VkPhysicalDeviceTimelineSemaphoreProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub max_timeline_semaphore_value_difference: u64,
}
impl<'lt> Default for PhysicalDeviceTimelineSemaphoreProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceTimelineSemaphoreProperties,
            p_next: std::ptr::null_mut(),
            max_timeline_semaphore_value_difference: 0,
        }
    }
}
impl<'lt> PhysicalDeviceTimelineSemaphoreProperties<'lt> {
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
    ///Gets the value of [`Self::max_timeline_semaphore_value_difference`]
    pub fn max_timeline_semaphore_value_difference(&self) -> u64 {
        self.max_timeline_semaphore_value_difference
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
    ///Gets a mutable reference to the value of [`Self::max_timeline_semaphore_value_difference`]
    pub fn max_timeline_semaphore_value_difference_mut(&mut self) -> &mut u64 {
        &mut self.max_timeline_semaphore_value_difference
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
    ///Sets the raw value of [`Self::max_timeline_semaphore_value_difference`]
    pub fn set_max_timeline_semaphore_value_difference(&mut self, value: u64) -> &mut Self {
        self.max_timeline_semaphore_value_difference = value;
        self
    }
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
///will have a default [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`.
///## Valid Usage
/// - If the [`timelineSemaphore`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-timelineSemaphore)
///   feature is not enabled, [`semaphore_type`] **must**  not equal `VK_SEMAPHORE_TYPE_TIMELINE`
/// - If [`semaphore_type`] is `VK_SEMAPHORE_TYPE_BINARY`, [`initial_value`] **must**  be zero
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO`
/// - [`semaphore_type`] **must**  be a valid [`SemaphoreType`] value
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
#[doc(alias = "VkSemaphoreTypeCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SemaphoreTypeCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`semaphore_type`] is a [`SemaphoreType`] value specifying the type
    ///of the semaphore.
    pub semaphore_type: SemaphoreType,
    ///[`initial_value`] is the initial payload value if [`semaphore_type`]
    ///is `VK_SEMAPHORE_TYPE_TIMELINE`.
    pub initial_value: u64,
}
impl<'lt> Default for SemaphoreTypeCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SemaphoreTypeCreateInfo,
            p_next: std::ptr::null(),
            semaphore_type: Default::default(),
            initial_value: 0,
        }
    }
}
impl<'lt> SemaphoreTypeCreateInfo<'lt> {
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
    ///Gets the value of [`Self::semaphore_type`]
    pub fn semaphore_type(&self) -> SemaphoreType {
        self.semaphore_type
    }
    ///Gets the value of [`Self::initial_value`]
    pub fn initial_value(&self) -> u64 {
        self.initial_value
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::semaphore_type`]
    pub fn semaphore_type_mut(&mut self) -> &mut SemaphoreType {
        &mut self.semaphore_type
    }
    ///Gets a mutable reference to the value of [`Self::initial_value`]
    pub fn initial_value_mut(&mut self) -> &mut u64 {
        &mut self.initial_value
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
    ///Sets the raw value of [`Self::semaphore_type`]
    pub fn set_semaphore_type(&mut self, value: crate::vulkan1_2::SemaphoreType) -> &mut Self {
        self.semaphore_type = value;
        self
    }
    ///Sets the raw value of [`Self::initial_value`]
    pub fn set_initial_value(&mut self, value: u64) -> &mut Self {
        self.initial_value = value;
        self
    }
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
///   [`wait_semaphore_values`].
/// - [`wait_semaphore_values`] is a pointer to an array of [`wait_semaphore_value_count`] values
///   for the corresponding semaphores in [`SubmitInfo::wait_semaphores`] to wait for.
/// - [`signal_semaphore_value_count`] is the number of semaphore signal values specified in
///   [`signal_semaphore_values`].
/// - [`signal_semaphore_values`] is a pointer to an array [`signal_semaphore_value_count`] values
///   for the corresponding semaphores in [`SubmitInfo::signal_semaphores`] to set when signaled.
///# Description
///If the semaphore in [`SubmitInfo::wait_semaphores`] or
///[`SubmitInfo::signal_semaphores`] corresponding to an entry in
///[`wait_semaphore_values`] or [`signal_semaphore_values`] respectively was
///not created with a [`SemaphoreType`] of
///`VK_SEMAPHORE_TYPE_TIMELINE`, the implementation  **must**  ignore the value
///in the [`wait_semaphore_values`] or [`signal_semaphore_values`] entry.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO`
/// - If [`wait_semaphore_value_count`] is not `0`, and [`wait_semaphore_values`] is not `NULL`,
///   [`wait_semaphore_values`] **must**  be a valid pointer to an array of
///   [`wait_semaphore_value_count`]`uint64_t` values
/// - If [`signal_semaphore_value_count`] is not `0`, and [`signal_semaphore_values`] is not `NULL`,
///   [`signal_semaphore_values`] **must**  be a valid pointer to an array of
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
#[doc(alias = "VkTimelineSemaphoreSubmitInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct TimelineSemaphoreSubmitInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`wait_semaphore_value_count`] is the number of semaphore wait values
    ///specified in [`wait_semaphore_values`].
    pub wait_semaphore_value_count: u32,
    ///[`wait_semaphore_values`] is a pointer to an array of
    ///[`wait_semaphore_value_count`] values for the corresponding semaphores in
    ///[`SubmitInfo`]::`pWaitSemaphores` to wait for.
    pub wait_semaphore_values: *const u64,
    ///[`signal_semaphore_value_count`] is the number of semaphore signal values
    ///specified in [`signal_semaphore_values`].
    pub signal_semaphore_value_count: u32,
    ///[`signal_semaphore_values`] is a pointer to an array
    ///[`signal_semaphore_value_count`] values for the corresponding semaphores
    ///in [`SubmitInfo`]::`pSignalSemaphores` to set when signaled.
    pub signal_semaphore_values: *const u64,
}
impl<'lt> Default for TimelineSemaphoreSubmitInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::TimelineSemaphoreSubmitInfo,
            p_next: std::ptr::null(),
            wait_semaphore_value_count: 0,
            wait_semaphore_values: std::ptr::null(),
            signal_semaphore_value_count: 0,
            signal_semaphore_values: std::ptr::null(),
        }
    }
}
impl<'lt> TimelineSemaphoreSubmitInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::wait_semaphore_values`]
    pub fn wait_semaphore_values_raw(&self) -> *const u64 {
        self.wait_semaphore_values
    }
    ///Gets the raw value of [`Self::signal_semaphore_values`]
    pub fn signal_semaphore_values_raw(&self) -> *const u64 {
        self.signal_semaphore_values
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_values`]
    pub fn set_wait_semaphore_values_raw(&mut self, value: *const u64) -> &mut Self {
        self.wait_semaphore_values = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_values`]
    pub fn set_signal_semaphore_values_raw(&mut self, value: *const u64) -> &mut Self {
        self.signal_semaphore_values = value;
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
    ///Gets the value of [`Self::wait_semaphore_value_count`]
    pub fn wait_semaphore_value_count(&self) -> u32 {
        self.wait_semaphore_value_count
    }
    ///Gets the value of [`Self::wait_semaphore_values`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn wait_semaphore_values(&self) -> &[u64] {
        std::slice::from_raw_parts(self.wait_semaphore_values, self.wait_semaphore_value_count as usize)
    }
    ///Gets the value of [`Self::signal_semaphore_value_count`]
    pub fn signal_semaphore_value_count(&self) -> u32 {
        self.signal_semaphore_value_count
    }
    ///Gets the value of [`Self::signal_semaphore_values`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn signal_semaphore_values(&self) -> &[u64] {
        std::slice::from_raw_parts(self.signal_semaphore_values, self.signal_semaphore_value_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::wait_semaphore_value_count`]
    pub fn wait_semaphore_value_count_mut(&mut self) -> &mut u32 {
        &mut self.wait_semaphore_value_count
    }
    ///Gets a mutable reference to the value of [`Self::signal_semaphore_value_count`]
    pub fn signal_semaphore_value_count_mut(&mut self) -> &mut u32 {
        &mut self.signal_semaphore_value_count
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
    ///Sets the raw value of [`Self::wait_semaphore_value_count`]
    pub fn set_wait_semaphore_value_count(&mut self, value: u32) -> &mut Self {
        self.wait_semaphore_value_count = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_values`]
    pub fn set_wait_semaphore_values(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.wait_semaphore_values = value.as_ptr();
        self.wait_semaphore_value_count = len_;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_value_count`]
    pub fn set_signal_semaphore_value_count(&mut self, value: u32) -> &mut Self {
        self.signal_semaphore_value_count = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_values`]
    pub fn set_signal_semaphore_values(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.signal_semaphore_values = value.as_ptr();
        self.signal_semaphore_value_count = len_;
        self
    }
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
/// - [`semaphores`] is a pointer to an array of [`semaphore_count`] semaphore handles to wait on.
/// - [`values`] is a pointer to an array of [`semaphore_count`] timeline semaphore values.
///# Description
///## Valid Usage
/// - All of the elements of [`semaphores`] **must**  reference a semaphore that was created with a
///   [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_TIMELINE`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be a valid combination of [`SemaphoreWaitFlagBits`] values
/// - [`semaphores`] **must**  be a valid pointer to an array of [`semaphore_count`] valid
///   [`Semaphore`] handles
/// - [`values`] **must**  be a valid pointer to an array of [`semaphore_count`]`uint64_t` values
/// - [`semaphore_count`] **must**  be greater than `0`
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
#[doc(alias = "VkSemaphoreWaitInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SemaphoreWaitInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`SemaphoreWaitFlagBits`] specifying
    ///additional parameters for the semaphore wait operation.
    pub flags: SemaphoreWaitFlags,
    ///[`semaphore_count`] is the number of semaphores to wait on.
    pub semaphore_count: u32,
    ///[`semaphores`] is a pointer to an array of [`semaphore_count`]
    ///semaphore handles to wait on.
    pub semaphores: *const Semaphore,
    ///[`values`] is a pointer to an array of [`semaphore_count`] timeline
    ///semaphore values.
    pub values: *const u64,
}
impl<'lt> Default for SemaphoreWaitInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SemaphoreWaitInfo,
            p_next: std::ptr::null(),
            flags: Default::default(),
            semaphore_count: 0,
            semaphores: std::ptr::null(),
            values: std::ptr::null(),
        }
    }
}
impl<'lt> SemaphoreWaitInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::semaphores`]
    pub fn semaphores_raw(&self) -> *const Semaphore {
        self.semaphores
    }
    ///Gets the raw value of [`Self::values`]
    pub fn values_raw(&self) -> *const u64 {
        self.values
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::semaphores`]
    pub fn set_semaphores_raw(&mut self, value: *const Semaphore) -> &mut Self {
        self.semaphores = value;
        self
    }
    ///Sets the raw value of [`Self::values`]
    pub fn set_values_raw(&mut self, value: *const u64) -> &mut Self {
        self.values = value;
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
    pub fn flags(&self) -> SemaphoreWaitFlags {
        self.flags
    }
    ///Gets the value of [`Self::semaphore_count`]
    pub fn semaphore_count(&self) -> u32 {
        self.semaphore_count
    }
    ///Gets the value of [`Self::semaphores`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn semaphores(&self) -> &[Semaphore] {
        std::slice::from_raw_parts(self.semaphores, self.semaphore_count as usize)
    }
    ///Gets the value of [`Self::values`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn values(&self) -> &[u64] {
        std::slice::from_raw_parts(self.values, self.semaphore_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut SemaphoreWaitFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::semaphore_count`]
    pub fn semaphore_count_mut(&mut self) -> &mut u32 {
        &mut self.semaphore_count
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
    pub fn set_flags(&mut self, value: crate::vulkan1_2::SemaphoreWaitFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::semaphore_count`]
    pub fn set_semaphore_count(&mut self, value: u32) -> &mut Self {
        self.semaphore_count = value;
        self
    }
    ///Sets the raw value of [`Self::semaphores`]
    pub fn set_semaphores(&mut self, value: &'lt [crate::vulkan1_0::Semaphore]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.semaphores = value.as_ptr();
        self.semaphore_count = len_;
        self
    }
    ///Sets the raw value of [`Self::values`]
    pub fn set_values(&mut self, value: &'lt [u64]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.values = value.as_ptr();
        self.semaphore_count = len_;
        self
    }
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
///## Valid Usage
/// - [`semaphore`] **must**  have been created with a [`SemaphoreType`] of
///   `VK_SEMAPHORE_TYPE_TIMELINE`
/// - [`value`] **must**  have a value greater than the current value of the semaphore
/// - [`value`] **must**  be less than the value of any pending semaphore signal operations
/// - [`value`] **must**  have a value which does not differ from the current value of the semaphore
///   or the value of any outstanding semaphore wait or signal operation on [`semaphore`] by more than
///   [`maxTimelineSemaphoreValueDifference`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxTimelineSemaphoreValueDifference)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO`
/// - [`p_next`] **must**  be `NULL`
/// - [`semaphore`] **must**  be a valid [`Semaphore`] handle
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
#[doc(alias = "VkSemaphoreSignalInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SemaphoreSignalInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`semaphore`] is the handle of the semaphore to signal.
    pub semaphore: Semaphore,
    ///[`value`] is the value to signal.
    pub value: u64,
}
impl<'lt> Default for SemaphoreSignalInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SemaphoreSignalInfo,
            p_next: std::ptr::null(),
            semaphore: Default::default(),
            value: 0,
        }
    }
}
impl<'lt> SemaphoreSignalInfo<'lt> {
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
    ///Gets the value of [`Self::semaphore`]
    pub fn semaphore(&self) -> Semaphore {
        self.semaphore
    }
    ///Gets the value of [`Self::value`]
    pub fn value(&self) -> u64 {
        self.value
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::semaphore`]
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Gets a mutable reference to the value of [`Self::value`]
    pub fn value_mut(&mut self) -> &mut u64 {
        &mut self.value
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
    ///Sets the raw value of [`Self::semaphore`]
    pub fn set_semaphore(&mut self, value: crate::vulkan1_0::Semaphore) -> &mut Self {
        self.semaphore = value;
        self
    }
    ///Sets the raw value of [`Self::value`]
    pub fn set_value(&mut self, value: u64) -> &mut Self {
        self.value = value;
        self
    }
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
///   decoration  **can**  have 8-bit integer     members.     If this feature is not enabled, 8-bit
///   integer members  **must**  not be used     in such objects.     This also indicates whether
///   shader modules  **can**  declare the     `StorageBuffer8BitAccess` capability.
/// - [`uniform_and_storage_buffer_8_bit_access`] indicates whether objects in the `Uniform` storage
///   class with the `Block` decoration  **can**  have 8-bit integer members. If this feature is not
///   enabled, 8-bit integer members  **must**  not be used in such objects. This also indicates
///   whether shader modules  **can**  declare the `UniformAndStorageBuffer8BitAccess` capability.
/// - [`storage_push_constant_8`] indicates whether objects in the `PushConstant` storage class
///   **can**  have 8-bit integer members. If this feature is not enabled, 8-bit integer members
///   **must**  not be used in such objects. This also indicates whether shader modules  **can**
///   declare the `StoragePushConstant8` capability.
///If the [`PhysicalDevice8BitStorageFeatures`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevice8BitStorageFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES`
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
#[doc(alias = "VkPhysicalDevice8BitStorageFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDevice8BitStorageFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`storage_buffer_8_bit_access`] indicates whether objects in the
    ///    `StorageBuffer`,
    ///`ShaderRecordBufferKHR`,
    ///    or `PhysicalStorageBuffer`
    ///    storage class with the `Block` decoration  **can**  have 8-bit integer
    ///    members.
    ///    If this feature is not enabled, 8-bit integer members  **must**  not be used
    ///    in such objects.
    ///    This also indicates whether shader modules  **can**  declare the
    ///    `StorageBuffer8BitAccess` capability.
    pub storage_buffer_8_bit_access: Bool32,
    ///[`uniform_and_storage_buffer_8_bit_access`] indicates whether objects in the
    ///`Uniform` storage class with the `Block` decoration  **can**  have
    ///8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members  **must**  not be used
    ///in such objects.
    ///This also indicates whether shader modules  **can**  declare the
    ///`UniformAndStorageBuffer8BitAccess` capability.
    pub uniform_and_storage_buffer_8_bit_access: Bool32,
    ///[`storage_push_constant_8`] indicates whether objects in the
    ///`PushConstant` storage class  **can**  have 8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members  **must**  not be used
    ///in such objects.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StoragePushConstant8` capability.
    pub storage_push_constant_8: Bool32,
}
impl<'lt> Default for PhysicalDevice8BitStorageFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDevice8BitStorageFeatures,
            p_next: std::ptr::null_mut(),
            storage_buffer_8_bit_access: 0,
            uniform_and_storage_buffer_8_bit_access: 0,
            storage_push_constant_8: 0,
        }
    }
}
impl<'lt> PhysicalDevice8BitStorageFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::storage_buffer_8_bit_access`]
    pub fn storage_buffer_8_bit_access_raw(&self) -> Bool32 {
        self.storage_buffer_8_bit_access
    }
    ///Gets the raw value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn uniform_and_storage_buffer_8_bit_access_raw(&self) -> Bool32 {
        self.uniform_and_storage_buffer_8_bit_access
    }
    ///Gets the raw value of [`Self::storage_push_constant_8`]
    pub fn storage_push_constant_8_raw(&self) -> Bool32 {
        self.storage_push_constant_8
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::storage_buffer_8_bit_access`]
    pub fn set_storage_buffer_8_bit_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.storage_buffer_8_bit_access = value;
        self
    }
    ///Sets the raw value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn set_uniform_and_storage_buffer_8_bit_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.uniform_and_storage_buffer_8_bit_access = value;
        self
    }
    ///Sets the raw value of [`Self::storage_push_constant_8`]
    pub fn set_storage_push_constant_8_raw(&mut self, value: Bool32) -> &mut Self {
        self.storage_push_constant_8 = value;
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
    ///Gets the value of [`Self::storage_buffer_8_bit_access`]
    pub fn storage_buffer_8_bit_access(&self) -> bool {
        unsafe { std::mem::transmute(self.storage_buffer_8_bit_access as u8) }
    }
    ///Gets the value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn uniform_and_storage_buffer_8_bit_access(&self) -> bool {
        unsafe { std::mem::transmute(self.uniform_and_storage_buffer_8_bit_access as u8) }
    }
    ///Gets the value of [`Self::storage_push_constant_8`]
    pub fn storage_push_constant_8(&self) -> bool {
        unsafe { std::mem::transmute(self.storage_push_constant_8 as u8) }
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
    ///Gets a mutable reference to the value of [`Self::storage_buffer_8_bit_access`]
    pub fn storage_buffer_8_bit_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.storage_buffer_8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.storage_buffer_8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn uniform_and_storage_buffer_8_bit_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.uniform_and_storage_buffer_8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.uniform_and_storage_buffer_8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::storage_push_constant_8`]
    pub fn storage_push_constant_8_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.storage_push_constant_8 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.storage_push_constant_8 as *mut Bool32)
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
    ///Sets the raw value of [`Self::storage_buffer_8_bit_access`]
    pub fn set_storage_buffer_8_bit_access(&mut self, value: bool) -> &mut Self {
        self.storage_buffer_8_bit_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn set_uniform_and_storage_buffer_8_bit_access(&mut self, value: bool) -> &mut Self {
        self.uniform_and_storage_buffer_8_bit_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::storage_push_constant_8`]
    pub fn set_storage_push_constant_8(&mut self, value: bool) -> &mut Self {
        self.storage_push_constant_8 = value as u8 as u32;
        self
    }
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
/// - [`vulkan_memory_model`] indicates whether the Vulkan Memory Model is supported, as defined in [Vulkan Memory Model](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model). This also indicates whether shader modules  **can**  declare the `VulkanMemoryModel` capability.
/// - [`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory Model can use
///   [`Device`] scope synchronization. This also indicates whether shader modules  **can**  declare
///   the `VulkanMemoryModelDeviceScope` capability.
/// - [`vulkan_memory_model_availability_visibility_chains`] indicates whether the Vulkan Memory Model can use [availability and visibility chains](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model-availability-visibility) with more than one element.
///If the [`PhysicalDeviceVulkanMemoryModelFeaturesKHR`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVulkanMemoryModelFeaturesKHR`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceVulkanMemoryModelFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`vulkan_memory_model`]
    ///indicates whether the Vulkan Memory Model is supported, as defined in
    ///[Vulkan Memory Model](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model).
    ///This also indicates whether shader modules  **can**  declare the
    ///`VulkanMemoryModel` capability.
    pub vulkan_memory_model: Bool32,
    ///[`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory
    ///Model can use [`Device`] scope synchronization.
    ///This also indicates whether shader modules  **can**  declare the
    ///`VulkanMemoryModelDeviceScope` capability.
    pub vulkan_memory_model_device_scope: Bool32,
    ///[`vulkan_memory_model_availability_visibility_chains`] indicates whether
    ///the Vulkan Memory Model can use [availability and visibility chains](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-model-availability-visibility) with more than one element.
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
}
impl<'lt> Default for PhysicalDeviceVulkanMemoryModelFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceVulkanMemoryModelFeatures,
            p_next: std::ptr::null_mut(),
            vulkan_memory_model: 0,
            vulkan_memory_model_device_scope: 0,
            vulkan_memory_model_availability_visibility_chains: 0,
        }
    }
}
impl<'lt> PhysicalDeviceVulkanMemoryModelFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::vulkan_memory_model`]
    pub fn vulkan_memory_model_raw(&self) -> Bool32 {
        self.vulkan_memory_model
    }
    ///Gets the raw value of [`Self::vulkan_memory_model_device_scope`]
    pub fn vulkan_memory_model_device_scope_raw(&self) -> Bool32 {
        self.vulkan_memory_model_device_scope
    }
    ///Gets the raw value of [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn vulkan_memory_model_availability_visibility_chains_raw(&self) -> Bool32 {
        self.vulkan_memory_model_availability_visibility_chains
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model`]
    pub fn set_vulkan_memory_model_raw(&mut self, value: Bool32) -> &mut Self {
        self.vulkan_memory_model = value;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model_device_scope`]
    pub fn set_vulkan_memory_model_device_scope_raw(&mut self, value: Bool32) -> &mut Self {
        self.vulkan_memory_model_device_scope = value;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn set_vulkan_memory_model_availability_visibility_chains_raw(&mut self, value: Bool32) -> &mut Self {
        self.vulkan_memory_model_availability_visibility_chains = value;
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
    ///Gets the value of [`Self::vulkan_memory_model`]
    pub fn vulkan_memory_model(&self) -> bool {
        unsafe { std::mem::transmute(self.vulkan_memory_model as u8) }
    }
    ///Gets the value of [`Self::vulkan_memory_model_device_scope`]
    pub fn vulkan_memory_model_device_scope(&self) -> bool {
        unsafe { std::mem::transmute(self.vulkan_memory_model_device_scope as u8) }
    }
    ///Gets the value of [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn vulkan_memory_model_availability_visibility_chains(&self) -> bool {
        unsafe { std::mem::transmute(self.vulkan_memory_model_availability_visibility_chains as u8) }
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
    ///Gets a mutable reference to the value of [`Self::vulkan_memory_model`]
    pub fn vulkan_memory_model_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vulkan_memory_model as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vulkan_memory_model as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::vulkan_memory_model_device_scope`]
    pub fn vulkan_memory_model_device_scope_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vulkan_memory_model_device_scope as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vulkan_memory_model_device_scope as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn vulkan_memory_model_availability_visibility_chains_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vulkan_memory_model_availability_visibility_chains as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vulkan_memory_model_availability_visibility_chains as *mut Bool32)
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
    ///Sets the raw value of [`Self::vulkan_memory_model`]
    pub fn set_vulkan_memory_model(&mut self, value: bool) -> &mut Self {
        self.vulkan_memory_model = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model_device_scope`]
    pub fn set_vulkan_memory_model_device_scope(&mut self, value: bool) -> &mut Self {
        self.vulkan_memory_model_device_scope = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn set_vulkan_memory_model_availability_visibility_chains(&mut self, value: bool) -> &mut Self {
        self.vulkan_memory_model_availability_visibility_chains = value as u8 as u32;
        self
    }
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
/// - [`shader_buffer_int_64_atomics`] indicates whether shaders  **can**  perform 64-bit unsigned
///   and signed integer atomic operations on buffers.
/// - [`shader_shared_int_64_atomics`] indicates whether shaders  **can**  perform 64-bit unsigned
///   and signed integer atomic operations on shared memory.
///If the [`PhysicalDeviceShaderAtomicInt64Features`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderAtomicInt64Features`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceShaderAtomicInt64Features")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64Features<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_buffer_int_64_atomics`] indicates whether shaders  **can**  perform
    ///64-bit unsigned and signed integer atomic operations on buffers.
    pub shader_buffer_int_64_atomics: Bool32,
    ///[`shader_shared_int_64_atomics`] indicates whether shaders  **can**  perform
    ///64-bit unsigned and signed integer atomic operations on shared memory.
    pub shader_shared_int_64_atomics: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderAtomicInt64Features<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceShaderAtomicInt64Features,
            p_next: std::ptr::null_mut(),
            shader_buffer_int_64_atomics: 0,
            shader_shared_int_64_atomics: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderAtomicInt64Features<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_buffer_int_64_atomics`]
    pub fn shader_buffer_int_64_atomics_raw(&self) -> Bool32 {
        self.shader_buffer_int_64_atomics
    }
    ///Gets the raw value of [`Self::shader_shared_int_64_atomics`]
    pub fn shader_shared_int_64_atomics_raw(&self) -> Bool32 {
        self.shader_shared_int_64_atomics
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_int_64_atomics`]
    pub fn set_shader_buffer_int_64_atomics_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_buffer_int_64_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_int_64_atomics`]
    pub fn set_shader_shared_int_64_atomics_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_shared_int_64_atomics = value;
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
    ///Gets the value of [`Self::shader_buffer_int_64_atomics`]
    pub fn shader_buffer_int_64_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_int_64_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_shared_int_64_atomics`]
    pub fn shader_shared_int_64_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_int_64_atomics as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shader_buffer_int_64_atomics`]
    pub fn shader_buffer_int_64_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_int_64_atomics`]
    pub fn shader_shared_int_64_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_int_64_atomics as *mut Bool32)
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
    ///Sets the raw value of [`Self::shader_buffer_int_64_atomics`]
    pub fn set_shader_buffer_int_64_atomics(&mut self, value: bool) -> &mut Self {
        self.shader_buffer_int_64_atomics = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_int_64_atomics`]
    pub fn set_shader_shared_int_64_atomics(&mut self, value: bool) -> &mut Self {
        self.shader_shared_int_64_atomics = value as u8 as u32;
        self
    }
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
///   of supported depth resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT` **must**  be included in
///   the set but implementations  **may**  support additional modes.
/// - [`supported_stencil_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set
///   of supported stencil resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT` **must**  be included in
///   the set but implementations  **may**  support additional modes. `VK_RESOLVE_MODE_AVERAGE_BIT`
///   **must**  not be included in the set.
/// - [`independent_resolve_none`] is [`TRUE`] if the implementation supports setting the depth and
///   stencil resolve modes to different values when one of those modes is `VK_RESOLVE_MODE_NONE`.
///   Otherwise the implementation only supports setting both modes to the same value.
/// - [`independent_resolve`] is [`TRUE`] if the implementation supports all combinations of the
///   supported depth and stencil resolve modes, including setting either depth or stencil resolve
///   mode to `VK_RESOLVE_MODE_NONE`. An implementation that supports [`independent_resolve`]
///   **must**  also support [`independent_resolve_none`].
///If the [`PhysicalDeviceDepthStencilResolveProperties`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES`
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
#[doc(alias = "VkPhysicalDeviceDepthStencilResolveProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolveProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub supported_depth_resolve_modes: ResolveModeFlags,
    ///No documentation found
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    ///No documentation found
    pub independent_resolve_none: Bool32,
    ///No documentation found
    pub independent_resolve: Bool32,
}
impl<'lt> Default for PhysicalDeviceDepthStencilResolveProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceDepthStencilResolveProperties,
            p_next: std::ptr::null_mut(),
            supported_depth_resolve_modes: Default::default(),
            supported_stencil_resolve_modes: Default::default(),
            independent_resolve_none: 0,
            independent_resolve: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDepthStencilResolveProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::independent_resolve_none`]
    pub fn independent_resolve_none_raw(&self) -> Bool32 {
        self.independent_resolve_none
    }
    ///Gets the raw value of [`Self::independent_resolve`]
    pub fn independent_resolve_raw(&self) -> Bool32 {
        self.independent_resolve
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::independent_resolve_none`]
    pub fn set_independent_resolve_none_raw(&mut self, value: Bool32) -> &mut Self {
        self.independent_resolve_none = value;
        self
    }
    ///Sets the raw value of [`Self::independent_resolve`]
    pub fn set_independent_resolve_raw(&mut self, value: Bool32) -> &mut Self {
        self.independent_resolve = value;
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
    ///Gets the value of [`Self::supported_depth_resolve_modes`]
    pub fn supported_depth_resolve_modes(&self) -> ResolveModeFlags {
        self.supported_depth_resolve_modes
    }
    ///Gets the value of [`Self::supported_stencil_resolve_modes`]
    pub fn supported_stencil_resolve_modes(&self) -> ResolveModeFlags {
        self.supported_stencil_resolve_modes
    }
    ///Gets the value of [`Self::independent_resolve_none`]
    pub fn independent_resolve_none(&self) -> bool {
        unsafe { std::mem::transmute(self.independent_resolve_none as u8) }
    }
    ///Gets the value of [`Self::independent_resolve`]
    pub fn independent_resolve(&self) -> bool {
        unsafe { std::mem::transmute(self.independent_resolve as u8) }
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
    ///Gets a mutable reference to the value of [`Self::supported_depth_resolve_modes`]
    pub fn supported_depth_resolve_modes_mut(&mut self) -> &mut ResolveModeFlags {
        &mut self.supported_depth_resolve_modes
    }
    ///Gets a mutable reference to the value of [`Self::supported_stencil_resolve_modes`]
    pub fn supported_stencil_resolve_modes_mut(&mut self) -> &mut ResolveModeFlags {
        &mut self.supported_stencil_resolve_modes
    }
    ///Gets a mutable reference to the value of [`Self::independent_resolve_none`]
    pub fn independent_resolve_none_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.independent_resolve_none as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.independent_resolve_none as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::independent_resolve`]
    pub fn independent_resolve_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.independent_resolve as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.independent_resolve as *mut Bool32)
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
    ///Sets the raw value of [`Self::supported_depth_resolve_modes`]
    pub fn set_supported_depth_resolve_modes(&mut self, value: crate::vulkan1_2::ResolveModeFlags) -> &mut Self {
        self.supported_depth_resolve_modes = value;
        self
    }
    ///Sets the raw value of [`Self::supported_stencil_resolve_modes`]
    pub fn set_supported_stencil_resolve_modes(&mut self, value: crate::vulkan1_2::ResolveModeFlags) -> &mut Self {
        self.supported_stencil_resolve_modes = value;
        self
    }
    ///Sets the raw value of [`Self::independent_resolve_none`]
    pub fn set_independent_resolve_none(&mut self, value: bool) -> &mut Self {
        self.independent_resolve_none = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::independent_resolve`]
    pub fn set_independent_resolve(&mut self, value: bool) -> &mut Self {
        self.independent_resolve = value as u8 as u32;
        self
    }
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
/// - [`depth_stencil_resolve_attachment`] is `NULL` or a pointer to a [`AttachmentReference2`]
///   structure defining the depth/stencil resolve attachment for this subpass and its layout.
///# Description
///If [`depth_stencil_resolve_attachment`] is `NULL`, or if its attachment
///index is [`ATTACHMENT_UNUSED`], it indicates that no depth/stencil
///resolve attachment will be used in the subpass.
///## Valid Usage
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], `pDepthStencilAttachment` **must**  not be `NULL` or have the value
///   [`ATTACHMENT_UNUSED`]
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], [`depth_resolve_mode`] and [`stencil_resolve_mode`] **must**  not both
///   be `VK_RESOLVE_MODE_NONE`
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], `pDepthStencilAttachment` **must**  not have a sample count of
///   `VK_SAMPLE_COUNT_1_BIT`
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], [`depth_stencil_resolve_attachment`] **must**  have a sample count of
///   `VK_SAMPLE_COUNT_1_BIT`
/// -    If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value [`ATTACHMENT_UNUSED`] then it  **must**  have an image format whose [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) contain `VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT`
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`] and [`Format`] of [`depth_stencil_resolve_attachment`] has a depth
///   component, then the [`Format`] of `pDepthStencilAttachment` **must**  have a depth component
///   with the same number of bits and numerical type
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], and [`Format`] of [`depth_stencil_resolve_attachment`] has a stencil
///   component, then the [`Format`] of `pDepthStencilAttachment` **must**  have a stencil component
///   with the same number of bits and numerical type
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`] and the [`Format`] of [`depth_stencil_resolve_attachment`] has a depth
///   component, then the value of [`depth_resolve_mode`] **must**  be one of the bits set in
///   [`PhysicalDeviceDepthStencilResolveProperties::supported_depth_resolve_modes`] or
///   `VK_RESOLVE_MODE_NONE`
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`] and the [`Format`] of [`depth_stencil_resolve_attachment`] has a stencil
///   component, then the value of [`stencil_resolve_mode`] **must**  be one of the bits set in
///   [`PhysicalDeviceDepthStencilResolveProperties::supported_stencil_resolve_modes`] or
///   `VK_RESOLVE_MODE_NONE`
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], the [`Format`] of [`depth_stencil_resolve_attachment`] has both depth
///   and stencil components, [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve`]
///   is [`FALSE`], and [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve_none`] is
///   [`FALSE`], then the values of [`depth_resolve_mode`] and [`stencil_resolve_mode`] **must**  be
///   identical
/// - If [`depth_stencil_resolve_attachment`] is not `NULL` and does not have the value
///   [`ATTACHMENT_UNUSED`], the [`Format`] of [`depth_stencil_resolve_attachment`] has both depth
///   and stencil components, [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve`]
///   is [`FALSE`] and [`PhysicalDeviceDepthStencilResolveProperties::independent_resolve_none`] is
///   [`TRUE`], then the values of [`depth_resolve_mode`] and [`stencil_resolve_mode`] **must**  be
///   identical or one of them  **must**  be `VK_RESOLVE_MODE_NONE`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE`
/// - If [`depth_stencil_resolve_attachment`] is not `NULL`, [`depth_stencil_resolve_attachment`]
///   **must**  be a valid pointer to a valid [`AttachmentReference2`] structure
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
#[doc(alias = "VkSubpassDescriptionDepthStencilResolve")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SubpassDescriptionDepthStencilResolve<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`depth_resolve_mode`] is a [`ResolveModeFlagBits`] value describing
    ///the depth resolve mode.
    pub depth_resolve_mode: ResolveModeFlagBits,
    ///[`stencil_resolve_mode`] is a [`ResolveModeFlagBits`] value
    ///describing the stencil resolve mode.
    pub stencil_resolve_mode: ResolveModeFlagBits,
    ///[`depth_stencil_resolve_attachment`] is `NULL` or a pointer to a
    ///[`AttachmentReference2`] structure defining the depth/stencil
    ///resolve attachment for this subpass and its layout.
    pub depth_stencil_resolve_attachment: *const AttachmentReference2<'lt>,
}
impl<'lt> Default for SubpassDescriptionDepthStencilResolve<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SubpassDescriptionDepthStencilResolve,
            p_next: std::ptr::null(),
            depth_resolve_mode: Default::default(),
            stencil_resolve_mode: Default::default(),
            depth_stencil_resolve_attachment: std::ptr::null(),
        }
    }
}
impl<'lt> SubpassDescriptionDepthStencilResolve<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::depth_stencil_resolve_attachment`]
    pub fn depth_stencil_resolve_attachment_raw(&self) -> *const AttachmentReference2<'lt> {
        self.depth_stencil_resolve_attachment
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::depth_stencil_resolve_attachment`]
    pub fn set_depth_stencil_resolve_attachment_raw(&mut self, value: *const AttachmentReference2<'lt>) -> &mut Self {
        self.depth_stencil_resolve_attachment = value;
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
    ///Gets the value of [`Self::depth_resolve_mode`]
    pub fn depth_resolve_mode(&self) -> ResolveModeFlagBits {
        self.depth_resolve_mode
    }
    ///Gets the value of [`Self::stencil_resolve_mode`]
    pub fn stencil_resolve_mode(&self) -> ResolveModeFlagBits {
        self.stencil_resolve_mode
    }
    ///Gets the value of [`Self::depth_stencil_resolve_attachment`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn depth_stencil_resolve_attachment(&self) -> &AttachmentReference2<'lt> {
        &*self.depth_stencil_resolve_attachment
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::depth_resolve_mode`]
    pub fn depth_resolve_mode_mut(&mut self) -> &mut ResolveModeFlagBits {
        &mut self.depth_resolve_mode
    }
    ///Gets a mutable reference to the value of [`Self::stencil_resolve_mode`]
    pub fn stencil_resolve_mode_mut(&mut self) -> &mut ResolveModeFlagBits {
        &mut self.stencil_resolve_mode
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
    ///Sets the raw value of [`Self::depth_resolve_mode`]
    pub fn set_depth_resolve_mode(&mut self, value: crate::vulkan1_2::ResolveModeFlagBits) -> &mut Self {
        self.depth_resolve_mode = value;
        self
    }
    ///Sets the raw value of [`Self::stencil_resolve_mode`]
    pub fn set_stencil_resolve_mode(&mut self, value: crate::vulkan1_2::ResolveModeFlagBits) -> &mut Self {
        self.stencil_resolve_mode = value;
        self
    }
    ///Sets the raw value of [`Self::depth_stencil_resolve_attachment`]
    pub fn set_depth_stencil_resolve_attachment(
        &mut self,
        value: &'lt crate::vulkan1_2::AttachmentReference2<'lt>,
    ) -> &mut Self {
        self.depth_stencil_resolve_attachment = value as *const _;
        self
    }
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
///[`ImageCreateInfo`], the stencil aspect of the image  **must**  only be used
///as specified by [`stencil_usage`].
///When this structure is not included in the [`p_next`] chain of
///[`ImageCreateInfo`], the stencil aspect of an image  **must**  only be used
///as specified by [`ImageCreateInfo::usage`].
///Use of other aspects of an image are unaffected by this structure.This structure  **can**  also
/// be included in the [`p_next`] chain of
///[`PhysicalDeviceImageFormatInfo2`] to query additional capabilities
///specific to image creation parameter combinations including a separate set
///of usage flags for the stencil aspect of the image using
///[`GetPhysicalDeviceImageFormatProperties2`].
///When this structure is not included in the [`p_next`] chain of
///[`PhysicalDeviceImageFormatInfo2`] then the implicit value of
///[`stencil_usage`] matches that of
///[`PhysicalDeviceImageFormatInfo2::usage`].
///## Valid Usage
/// - If [`stencil_usage`] includes `VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT`, it  **must**  not
///   include bits other than `VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT` or
///   `VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO`
/// - [`stencil_usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
/// - [`stencil_usage`] **must**  not be `0`
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
#[doc(alias = "VkImageStencilUsageCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageStencilUsageCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`stencil_usage`] is a bitmask of [`ImageUsageFlagBits`] describing
    ///the intended usage of the stencil aspect of the image.
    pub stencil_usage: ImageUsageFlags,
}
impl<'lt> Default for ImageStencilUsageCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ImageStencilUsageCreateInfo,
            p_next: std::ptr::null(),
            stencil_usage: Default::default(),
        }
    }
}
impl<'lt> ImageStencilUsageCreateInfo<'lt> {
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
    ///Gets the value of [`Self::stencil_usage`]
    pub fn stencil_usage(&self) -> ImageUsageFlags {
        self.stencil_usage
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::stencil_usage`]
    pub fn stencil_usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.stencil_usage
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
    ///Sets the raw value of [`Self::stencil_usage`]
    pub fn set_stencil_usage(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.stencil_usage = value;
        self
    }
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
///[`PhysicalDeviceScalarBlockLayoutFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceScalarBlockLayoutFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`scalar_block_layout`]
    ///indicates that the implementation supports the layout of resource blocks
    ///in shaders using [scalar
    ///alignment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-alignment-requirements).
    pub scalar_block_layout: Bool32,
}
impl<'lt> Default for PhysicalDeviceScalarBlockLayoutFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceScalarBlockLayoutFeatures,
            p_next: std::ptr::null_mut(),
            scalar_block_layout: 0,
        }
    }
}
impl<'lt> PhysicalDeviceScalarBlockLayoutFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::scalar_block_layout`]
    pub fn scalar_block_layout_raw(&self) -> Bool32 {
        self.scalar_block_layout
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::scalar_block_layout`]
    pub fn set_scalar_block_layout_raw(&mut self, value: Bool32) -> &mut Self {
        self.scalar_block_layout = value;
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
    ///Gets the value of [`Self::scalar_block_layout`]
    pub fn scalar_block_layout(&self) -> bool {
        unsafe { std::mem::transmute(self.scalar_block_layout as u8) }
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
    ///Gets a mutable reference to the value of [`Self::scalar_block_layout`]
    pub fn scalar_block_layout_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.scalar_block_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.scalar_block_layout as *mut Bool32)
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
    ///Sets the raw value of [`Self::scalar_block_layout`]
    pub fn set_scalar_block_layout(&mut self, value: bool) -> &mut Self {
        self.scalar_block_layout = value as u8 as u32;
        self
    }
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
///[`PhysicalDeviceUniformBufferStandardLayoutFeatures`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkPhysicalDeviceUniformBufferStandardLayoutFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`uniform_buffer_standard_layout`] indicates that the implementation
    ///supports the same layouts for uniform buffers as for storage and other
    ///kinds of buffers.
    ///See [Standard Buffer Layout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-resources-standard-layout).
    pub uniform_buffer_standard_layout: Bool32,
}
impl<'lt> Default for PhysicalDeviceUniformBufferStandardLayoutFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceUniformBufferStandardLayoutFeatures,
            p_next: std::ptr::null_mut(),
            uniform_buffer_standard_layout: 0,
        }
    }
}
impl<'lt> PhysicalDeviceUniformBufferStandardLayoutFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::uniform_buffer_standard_layout`]
    pub fn uniform_buffer_standard_layout_raw(&self) -> Bool32 {
        self.uniform_buffer_standard_layout
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::uniform_buffer_standard_layout`]
    pub fn set_uniform_buffer_standard_layout_raw(&mut self, value: Bool32) -> &mut Self {
        self.uniform_buffer_standard_layout = value;
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
    ///Gets the value of [`Self::uniform_buffer_standard_layout`]
    pub fn uniform_buffer_standard_layout(&self) -> bool {
        unsafe { std::mem::transmute(self.uniform_buffer_standard_layout as u8) }
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
    ///Gets a mutable reference to the value of [`Self::uniform_buffer_standard_layout`]
    pub fn uniform_buffer_standard_layout_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.uniform_buffer_standard_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.uniform_buffer_standard_layout as *mut Bool32)
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
    ///Sets the raw value of [`Self::uniform_buffer_standard_layout`]
    pub fn set_uniform_buffer_standard_layout(&mut self, value: bool) -> &mut Self {
        self.uniform_buffer_standard_layout = value as u8 as u32;
        self
    }
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
///   acceleration structure addresses  **must**  not be queried on a logical device created with
///   more than one physical device.
///See [`GetBufferDeviceAddress`] for more information.If the
/// [`PhysicalDeviceBufferDeviceAddressFeatures`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceBufferDeviceAddressFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`buffer_device_address`] indicates that the implementation supports
    ///accessing buffer memory in shaders as storage buffers via an address
    ///queried from [`GetBufferDeviceAddress`].
    pub buffer_device_address: Bool32,
    ///[`buffer_device_address_capture_replay`] indicates that the implementation
    ///supports saving and reusing buffer and device addresses, e.g. for trace
    ///capture and replay.
    pub buffer_device_address_capture_replay: Bool32,
    ///[`buffer_device_address_multi_device`] indicates that the implementation
    ///supports the [`buffer_device_address`]
    ///, `rayTracingPipeline` and `rayQuery` features
    ///for logical devices created with multiple physical devices.
    ///If this feature is not supported, buffer
    ///and acceleration structure
    ///addresses  **must**  not be queried on a logical device created with more
    ///than one physical device.
    pub buffer_device_address_multi_device: Bool32,
}
impl<'lt> Default for PhysicalDeviceBufferDeviceAddressFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceBufferDeviceAddressFeatures,
            p_next: std::ptr::null_mut(),
            buffer_device_address: 0,
            buffer_device_address_capture_replay: 0,
            buffer_device_address_multi_device: 0,
        }
    }
}
impl<'lt> PhysicalDeviceBufferDeviceAddressFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::buffer_device_address`]
    pub fn buffer_device_address_raw(&self) -> Bool32 {
        self.buffer_device_address
    }
    ///Gets the raw value of [`Self::buffer_device_address_capture_replay`]
    pub fn buffer_device_address_capture_replay_raw(&self) -> Bool32 {
        self.buffer_device_address_capture_replay
    }
    ///Gets the raw value of [`Self::buffer_device_address_multi_device`]
    pub fn buffer_device_address_multi_device_raw(&self) -> Bool32 {
        self.buffer_device_address_multi_device
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address`]
    pub fn set_buffer_device_address_raw(&mut self, value: Bool32) -> &mut Self {
        self.buffer_device_address = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_capture_replay`]
    pub fn set_buffer_device_address_capture_replay_raw(&mut self, value: Bool32) -> &mut Self {
        self.buffer_device_address_capture_replay = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_multi_device`]
    pub fn set_buffer_device_address_multi_device_raw(&mut self, value: Bool32) -> &mut Self {
        self.buffer_device_address_multi_device = value;
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
    ///Gets the value of [`Self::buffer_device_address`]
    pub fn buffer_device_address(&self) -> bool {
        unsafe { std::mem::transmute(self.buffer_device_address as u8) }
    }
    ///Gets the value of [`Self::buffer_device_address_capture_replay`]
    pub fn buffer_device_address_capture_replay(&self) -> bool {
        unsafe { std::mem::transmute(self.buffer_device_address_capture_replay as u8) }
    }
    ///Gets the value of [`Self::buffer_device_address_multi_device`]
    pub fn buffer_device_address_multi_device(&self) -> bool {
        unsafe { std::mem::transmute(self.buffer_device_address_multi_device as u8) }
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
    ///Gets a mutable reference to the value of [`Self::buffer_device_address`]
    pub fn buffer_device_address_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.buffer_device_address as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.buffer_device_address as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::buffer_device_address_capture_replay`]
    pub fn buffer_device_address_capture_replay_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.buffer_device_address_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.buffer_device_address_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::buffer_device_address_multi_device`]
    pub fn buffer_device_address_multi_device_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.buffer_device_address_multi_device as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.buffer_device_address_multi_device as *mut Bool32)
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
    ///Sets the raw value of [`Self::buffer_device_address`]
    pub fn set_buffer_device_address(&mut self, value: bool) -> &mut Self {
        self.buffer_device_address = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_capture_replay`]
    pub fn set_buffer_device_address_capture_replay(&mut self, value: bool) -> &mut Self {
        self.buffer_device_address_capture_replay = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_multi_device`]
    pub fn set_buffer_device_address_multi_device(&mut self, value: bool) -> &mut Self {
        self.buffer_device_address_multi_device = value as u8 as u32;
        self
    }
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
///## Valid Usage
/// - If [`buffer`] is non-sparse and was not created with the
///   `VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT` flag, then it  **must**  be bound
///   completely and contiguously to a single [`DeviceMemory`] object
/// - [`buffer`] **must**  have been created with `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO`
/// - [`p_next`] **must**  be `NULL`
/// - [`buffer`] **must**  be a valid [`Buffer`] handle
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
#[doc(alias = "VkBufferDeviceAddressInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BufferDeviceAddressInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`buffer`] specifies the buffer whose address is being queried.
    pub buffer: Buffer,
}
impl<'lt> Default for BufferDeviceAddressInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BufferDeviceAddressInfo,
            p_next: std::ptr::null(),
            buffer: Default::default(),
        }
    }
}
impl<'lt> BufferDeviceAddressInfo<'lt> {
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
    ///Gets the value of [`Self::buffer`]
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::buffer`]
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
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
    ///Sets the raw value of [`Self::buffer`]
    pub fn set_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.buffer = value;
        self
    }
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
/// [`opaque_capture_address`] is not zero, then it  **should**  be an address
///retrieved from [`GetBufferOpaqueCaptureAddress`] for an identically
///created buffer on the same implementation.If this structure is not present, it is as if
/// [`opaque_capture_address`] is
///zero.Apps  **should**  avoid creating buffers with app-provided addresses and
///implementation-provided addresses in the same process, to reduce the
///likelihood of `VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS` errors.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO`
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
#[doc(alias = "VkBufferOpaqueCaptureAddressCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BufferOpaqueCaptureAddressCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`opaque_capture_address`] is the opaque capture address requested for
    ///the buffer.
    pub opaque_capture_address: u64,
}
impl<'lt> Default for BufferOpaqueCaptureAddressCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BufferOpaqueCaptureAddressCreateInfo,
            p_next: std::ptr::null(),
            opaque_capture_address: 0,
        }
    }
}
impl<'lt> BufferOpaqueCaptureAddressCreateInfo<'lt> {
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
    ///Gets the value of [`Self::opaque_capture_address`]
    pub fn opaque_capture_address(&self) -> u64 {
        self.opaque_capture_address
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::opaque_capture_address`]
    pub fn opaque_capture_address_mut(&mut self) -> &mut u64 {
        &mut self.opaque_capture_address
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
    ///Sets the raw value of [`Self::opaque_capture_address`]
    pub fn set_opaque_capture_address(&mut self, value: u64) -> &mut Self {
        self.opaque_capture_address = value;
        self
    }
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
///[`PhysicalDeviceImagelessFramebufferFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`imageless_framebuffer`] indicates that the implementation supports
    ///specifying the image view for attachments at render pass begin time via
    ///[`RenderPassAttachmentBeginInfo`].
    pub imageless_framebuffer: Bool32,
}
impl<'lt> Default for PhysicalDeviceImagelessFramebufferFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceImagelessFramebufferFeatures,
            p_next: std::ptr::null_mut(),
            imageless_framebuffer: 0,
        }
    }
}
impl<'lt> PhysicalDeviceImagelessFramebufferFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::imageless_framebuffer`]
    pub fn imageless_framebuffer_raw(&self) -> Bool32 {
        self.imageless_framebuffer
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::imageless_framebuffer`]
    pub fn set_imageless_framebuffer_raw(&mut self, value: Bool32) -> &mut Self {
        self.imageless_framebuffer = value;
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
    ///Gets the value of [`Self::imageless_framebuffer`]
    pub fn imageless_framebuffer(&self) -> bool {
        unsafe { std::mem::transmute(self.imageless_framebuffer as u8) }
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
    ///Gets a mutable reference to the value of [`Self::imageless_framebuffer`]
    pub fn imageless_framebuffer_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.imageless_framebuffer as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.imageless_framebuffer as *mut Bool32)
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
    ///Sets the raw value of [`Self::imageless_framebuffer`]
    pub fn set_imageless_framebuffer(&mut self, value: bool) -> &mut Self {
        self.imageless_framebuffer = value as u8 as u32;
        self
    }
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
/// - [`attachment_image_infos`] is a pointer to an array of [`FramebufferAttachmentImageInfo`]
///   structures, each structure describing a number of parameters of the corresponding attachment
///   in a render pass instance.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO`
/// - If [`attachment_image_info_count`] is not `0`, [`attachment_image_infos`] **must**  be a valid
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
#[doc(alias = "VkFramebufferAttachmentsCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FramebufferAttachmentsCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`attachment_image_info_count`] is the number of attachments being
    ///described.
    pub attachment_image_info_count: u32,
    ///[`attachment_image_infos`] is a pointer to an array of
    ///[`FramebufferAttachmentImageInfo`] structures, each structure
    ///describing a number of parameters of the corresponding attachment in a
    ///render pass instance.
    pub attachment_image_infos: *const FramebufferAttachmentImageInfo<'lt>,
}
impl<'lt> Default for FramebufferAttachmentsCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::FramebufferAttachmentsCreateInfo,
            p_next: std::ptr::null(),
            attachment_image_info_count: 0,
            attachment_image_infos: std::ptr::null(),
        }
    }
}
impl<'lt> FramebufferAttachmentsCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::attachment_image_infos`]
    pub fn attachment_image_infos_raw(&self) -> *const FramebufferAttachmentImageInfo<'lt> {
        self.attachment_image_infos
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::attachment_image_infos`]
    pub fn set_attachment_image_infos_raw(&mut self, value: *const FramebufferAttachmentImageInfo<'lt>) -> &mut Self {
        self.attachment_image_infos = value;
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
    ///Gets the value of [`Self::attachment_image_info_count`]
    pub fn attachment_image_info_count(&self) -> u32 {
        self.attachment_image_info_count
    }
    ///Gets the value of [`Self::attachment_image_infos`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn attachment_image_infos(&self) -> &[FramebufferAttachmentImageInfo<'lt>] {
        std::slice::from_raw_parts(self.attachment_image_infos, self.attachment_image_info_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::attachment_image_info_count`]
    pub fn attachment_image_info_count_mut(&mut self) -> &mut u32 {
        &mut self.attachment_image_info_count
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
    ///Sets the raw value of [`Self::attachment_image_info_count`]
    pub fn set_attachment_image_info_count(&mut self, value: u32) -> &mut Self {
        self.attachment_image_info_count = value;
        self
    }
    ///Sets the raw value of [`Self::attachment_image_infos`]
    pub fn set_attachment_image_infos(
        &mut self,
        value: &'lt [crate::vulkan1_2::FramebufferAttachmentImageInfo<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.attachment_image_infos = value.as_ptr();
        self.attachment_image_info_count = len_;
        self
    }
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
/// - [`view_format_count`] is the number of entries in the [`view_formats`] array, matching the
///   value of [`ImageFormatListCreateInfo`]::[`view_format_count`] used to create an image used
///   with this framebuffer.
/// - [`view_formats`] is a pointer to an array of [`Format`] values specifying all of the formats
///   which  **can**  be used when creating views of the image, matching the value of
///   [`ImageFormatListCreateInfo`]::[`view_formats`] used to create an image used with this
///   framebuffer.
///# Description
///Images that  **can**  be used with the framebuffer when beginning a render pass,
///as specified by [`RenderPassAttachmentBeginInfo`],  **must**  be created with
///parameters that are identical to those specified here.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be a valid combination of [`ImageCreateFlagBits`] values
/// - [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
/// - [`usage`] **must**  not be `0`
/// - If [`view_format_count`] is not `0`, [`view_formats`] **must**  be a valid pointer to an array
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
#[doc(alias = "VkFramebufferAttachmentImageInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FramebufferAttachmentImageInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`ImageCreateFlagBits`], matching the
    ///value of [`ImageCreateInfo`]::[`flags`] used to create an image
    ///that will be used with this framebuffer.
    pub flags: ImageCreateFlags,
    ///[`usage`] is a bitmask of [`ImageUsageFlagBits`], matching the
    ///value of [`ImageCreateInfo`]::[`usage`] used to create an image
    ///used with this framebuffer.
    pub usage: ImageUsageFlags,
    ///[`width`] is the width of the image view used for rendering.
    pub width: u32,
    ///[`height`] is the height of the image view used for rendering.
    pub height: u32,
    ///[`layer_count`] is the number of array layers of the image view used
    ///for rendering.
    pub layer_count: u32,
    ///[`view_format_count`] is the number of entries in the [`view_formats`]
    ///array, matching the value of
    ///[`ImageFormatListCreateInfo`]::[`view_format_count`] used to create
    ///an image used with this framebuffer.
    pub view_format_count: u32,
    ///[`view_formats`] is a pointer to an array of [`Format`] values
    ///specifying all of the formats which  **can**  be used when creating views of
    ///the image, matching the value of
    ///[`ImageFormatListCreateInfo`]::[`view_formats`] used to create an
    ///image used with this framebuffer.
    pub view_formats: *const Format,
}
impl<'lt> Default for FramebufferAttachmentImageInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::FramebufferAttachmentImageInfo,
            p_next: std::ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            width: 0,
            height: 0,
            layer_count: 0,
            view_format_count: 0,
            view_formats: std::ptr::null(),
        }
    }
}
impl<'lt> FramebufferAttachmentImageInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::view_formats`]
    pub fn view_formats_raw(&self) -> *const Format {
        self.view_formats
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::view_formats`]
    pub fn set_view_formats_raw(&mut self, value: *const Format) -> &mut Self {
        self.view_formats = value;
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
    pub fn flags(&self) -> ImageCreateFlags {
        self.flags
    }
    ///Gets the value of [`Self::usage`]
    pub fn usage(&self) -> ImageUsageFlags {
        self.usage
    }
    ///Gets the value of [`Self::width`]
    pub fn width(&self) -> u32 {
        self.width
    }
    ///Gets the value of [`Self::height`]
    pub fn height(&self) -> u32 {
        self.height
    }
    ///Gets the value of [`Self::layer_count`]
    pub fn layer_count(&self) -> u32 {
        self.layer_count
    }
    ///Gets the value of [`Self::view_format_count`]
    pub fn view_format_count(&self) -> u32 {
        self.view_format_count
    }
    ///Gets the value of [`Self::view_formats`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn view_formats(&self) -> &[Format] {
        std::slice::from_raw_parts(self.view_formats, self.view_format_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ImageCreateFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::usage`]
    pub fn usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.usage
    }
    ///Gets a mutable reference to the value of [`Self::width`]
    pub fn width_mut(&mut self) -> &mut u32 {
        &mut self.width
    }
    ///Gets a mutable reference to the value of [`Self::height`]
    pub fn height_mut(&mut self) -> &mut u32 {
        &mut self.height
    }
    ///Gets a mutable reference to the value of [`Self::layer_count`]
    pub fn layer_count_mut(&mut self) -> &mut u32 {
        &mut self.layer_count
    }
    ///Gets a mutable reference to the value of [`Self::view_format_count`]
    pub fn view_format_count_mut(&mut self) -> &mut u32 {
        &mut self.view_format_count
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
    pub fn set_flags(&mut self, value: crate::vulkan1_0::ImageCreateFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::usage`]
    pub fn set_usage(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.usage = value;
        self
    }
    ///Sets the raw value of [`Self::width`]
    pub fn set_width(&mut self, value: u32) -> &mut Self {
        self.width = value;
        self
    }
    ///Sets the raw value of [`Self::height`]
    pub fn set_height(&mut self, value: u32) -> &mut Self {
        self.height = value;
        self
    }
    ///Sets the raw value of [`Self::layer_count`]
    pub fn set_layer_count(&mut self, value: u32) -> &mut Self {
        self.layer_count = value;
        self
    }
    ///Sets the raw value of [`Self::view_format_count`]
    pub fn set_view_format_count(&mut self, value: u32) -> &mut Self {
        self.view_format_count = value;
        self
    }
    ///Sets the raw value of [`Self::view_formats`]
    pub fn set_view_formats(&mut self, value: &'lt [crate::vulkan1_0::Format]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.view_formats = value.as_ptr();
        self.view_format_count = len_;
        self
    }
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
/// - [`attachments`] is a pointer to an array of [`ImageView`] handles, each of which will be used
///   as the corresponding attachment in the render pass instance.
///# Description
///## Valid Usage
/// - Each element of [`attachments`] **must**  only specify a single mip level
/// - Each element of [`attachments`] **must**  have been created with the identity swizzle
/// - Each element of [`attachments`] **must**  have been created with
///   [`ImageViewCreateInfo::view_type`] not equal to `VK_IMAGE_VIEW_TYPE_3D`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO`
/// - If [`attachment_count`] is not `0`, [`attachments`] **must**  be a valid pointer to an array
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
#[doc(alias = "VkRenderPassAttachmentBeginInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct RenderPassAttachmentBeginInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`attachment_count`] is the number of attachments.
    pub attachment_count: u32,
    ///[`attachments`] is a pointer to an array of [`ImageView`]
    ///handles, each of which will be used as the corresponding attachment in
    ///the render pass instance.
    pub attachments: *const ImageView,
}
impl<'lt> Default for RenderPassAttachmentBeginInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RenderPassAttachmentBeginInfo,
            p_next: std::ptr::null(),
            attachment_count: 0,
            attachments: std::ptr::null(),
        }
    }
}
impl<'lt> RenderPassAttachmentBeginInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::attachments`]
    pub fn attachments_raw(&self) -> *const ImageView {
        self.attachments
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::attachments`]
    pub fn set_attachments_raw(&mut self, value: *const ImageView) -> &mut Self {
        self.attachments = value;
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
    ///Gets the value of [`Self::attachment_count`]
    pub fn attachment_count(&self) -> u32 {
        self.attachment_count
    }
    ///Gets the value of [`Self::attachments`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn attachments(&self) -> &[ImageView] {
        std::slice::from_raw_parts(self.attachments, self.attachment_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::attachment_count`]
    pub fn attachment_count_mut(&mut self) -> &mut u32 {
        &mut self.attachment_count
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
    ///Sets the raw value of [`Self::attachment_count`]
    pub fn set_attachment_count(&mut self, value: u32) -> &mut Self {
        self.attachment_count = value;
        self
    }
    ///Sets the raw value of [`Self::attachments`]
    pub fn set_attachments(&mut self, value: &'lt [crate::vulkan1_0::ImageView]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.attachments = value.as_ptr();
        self.attachment_count = len_;
        self
    }
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
///[`PhysicalDeviceSeparateDepthStencilLayoutsFeatures`] **can**  also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`separate_depth_stencil_layouts`] indicates whether the implementation
    ///supports a [`ImageMemoryBarrier`] for a depth/stencil image with
    ///only one of `VK_IMAGE_ASPECT_DEPTH_BIT` or
    ///`VK_IMAGE_ASPECT_STENCIL_BIT` set, and whether
    ///`VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
    ///`VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
    ///`VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or
    ///`VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` can be used.
    pub separate_depth_stencil_layouts: Bool32,
}
impl<'lt> Default for PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
            p_next: std::ptr::null_mut(),
            separate_depth_stencil_layouts: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::separate_depth_stencil_layouts`]
    pub fn separate_depth_stencil_layouts_raw(&self) -> Bool32 {
        self.separate_depth_stencil_layouts
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::separate_depth_stencil_layouts`]
    pub fn set_separate_depth_stencil_layouts_raw(&mut self, value: Bool32) -> &mut Self {
        self.separate_depth_stencil_layouts = value;
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
    ///Gets the value of [`Self::separate_depth_stencil_layouts`]
    pub fn separate_depth_stencil_layouts(&self) -> bool {
        unsafe { std::mem::transmute(self.separate_depth_stencil_layouts as u8) }
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
    ///Gets a mutable reference to the value of [`Self::separate_depth_stencil_layouts`]
    pub fn separate_depth_stencil_layouts_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.separate_depth_stencil_layouts as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.separate_depth_stencil_layouts as *mut Bool32)
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
    ///Sets the raw value of [`Self::separate_depth_stencil_layouts`]
    pub fn set_separate_depth_stencil_layouts(&mut self, value: bool) -> &mut Self {
        self.separate_depth_stencil_layouts = value as u8 as u32;
        self
    }
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
///## Valid Usage
/// - [`stencil_layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED`,
///   `VK_IMAGE_LAYOUT_PREINITIALIZED`, `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT`
/// - [`stencil_layout`] **must**  be a valid [`ImageLayout`] value
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
#[doc(alias = "VkAttachmentReferenceStencilLayout")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AttachmentReferenceStencilLayout<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`stencil_layout`] is a [`ImageLayout`] value specifying the layout
    ///the stencil aspect of the attachment uses during the subpass.
    pub stencil_layout: ImageLayout,
}
impl<'lt> Default for AttachmentReferenceStencilLayout<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::AttachmentReferenceStencilLayout,
            p_next: std::ptr::null_mut(),
            stencil_layout: Default::default(),
        }
    }
}
impl<'lt> AttachmentReferenceStencilLayout<'lt> {
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
    ///Gets the value of [`Self::stencil_layout`]
    pub fn stencil_layout(&self) -> ImageLayout {
        self.stencil_layout
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
    ///Gets a mutable reference to the value of [`Self::stencil_layout`]
    pub fn stencil_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.stencil_layout
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
    ///Sets the raw value of [`Self::stencil_layout`]
    pub fn set_stencil_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.stencil_layout = value;
        self
    }
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
///## Valid Usage
/// - [`stencil_initial_layout`] **must**  not be `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - [`stencil_final_layout`] **must**  not be `VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`, `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`,
///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, or
///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`
/// - [`stencil_final_layout`] **must**  not be `VK_IMAGE_LAYOUT_UNDEFINED` or
///   `VK_IMAGE_LAYOUT_PREINITIALIZED`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT`
/// - [`stencil_initial_layout`] **must**  be a valid [`ImageLayout`] value
/// - [`stencil_final_layout`] **must**  be a valid [`ImageLayout`] value
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
#[doc(alias = "VkAttachmentDescriptionStencilLayout")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AttachmentDescriptionStencilLayout<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`stencil_initial_layout`] is the layout the stencil aspect of the
    ///attachment image subresource will be in when a render pass instance
    ///begins.
    pub stencil_initial_layout: ImageLayout,
    ///[`stencil_final_layout`] is the layout the stencil aspect of the
    ///attachment image subresource will be transitioned to when a render pass
    ///instance ends.
    pub stencil_final_layout: ImageLayout,
}
impl<'lt> Default for AttachmentDescriptionStencilLayout<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::AttachmentDescriptionStencilLayout,
            p_next: std::ptr::null_mut(),
            stencil_initial_layout: Default::default(),
            stencil_final_layout: Default::default(),
        }
    }
}
impl<'lt> AttachmentDescriptionStencilLayout<'lt> {
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
    ///Gets the value of [`Self::stencil_initial_layout`]
    pub fn stencil_initial_layout(&self) -> ImageLayout {
        self.stencil_initial_layout
    }
    ///Gets the value of [`Self::stencil_final_layout`]
    pub fn stencil_final_layout(&self) -> ImageLayout {
        self.stencil_final_layout
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
    ///Gets a mutable reference to the value of [`Self::stencil_initial_layout`]
    pub fn stencil_initial_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.stencil_initial_layout
    }
    ///Gets a mutable reference to the value of [`Self::stencil_final_layout`]
    pub fn stencil_final_layout_mut(&mut self) -> &mut ImageLayout {
        &mut self.stencil_final_layout
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
    ///Sets the raw value of [`Self::stencil_initial_layout`]
    pub fn set_stencil_initial_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.stencil_initial_layout = value;
        self
    }
    ///Sets the raw value of [`Self::stencil_final_layout`]
    pub fn set_stencil_final_layout(&mut self, value: crate::vulkan1_0::ImageLayout) -> &mut Self {
        self.stencil_final_layout = value;
        self
    }
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
/// [`opaque_capture_address`] is not zero, it  **should**  be an address
///retrieved from [`GetDeviceMemoryOpaqueCaptureAddress`] on an identically
///created memory allocation on the same implementation.If this structure is not present, it is as
/// if [`opaque_capture_address`] is
///zero.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO`
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
#[doc(alias = "VkMemoryOpaqueCaptureAddressAllocateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`opaque_capture_address`] is the opaque capture address requested for
    ///the memory allocation.
    pub opaque_capture_address: u64,
}
impl<'lt> Default for MemoryOpaqueCaptureAddressAllocateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MemoryOpaqueCaptureAddressAllocateInfo,
            p_next: std::ptr::null(),
            opaque_capture_address: 0,
        }
    }
}
impl<'lt> MemoryOpaqueCaptureAddressAllocateInfo<'lt> {
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
    ///Gets the value of [`Self::opaque_capture_address`]
    pub fn opaque_capture_address(&self) -> u64 {
        self.opaque_capture_address
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::opaque_capture_address`]
    pub fn opaque_capture_address_mut(&mut self) -> &mut u64 {
        &mut self.opaque_capture_address
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
    ///Sets the raw value of [`Self::opaque_capture_address`]
    pub fn set_opaque_capture_address(&mut self, value: u64) -> &mut Self {
        self.opaque_capture_address = value;
        self
    }
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
///## Valid Usage
/// - [`memory`] **must**  have been allocated with `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO`
/// - [`p_next`] **must**  be `NULL`
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
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
#[doc(alias = "VkDeviceMemoryOpaqueCaptureAddressInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`memory`] specifies the memory whose address is being queried.
    pub memory: DeviceMemory,
}
impl<'lt> Default for DeviceMemoryOpaqueCaptureAddressInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DeviceMemoryOpaqueCaptureAddressInfo,
            p_next: std::ptr::null(),
            memory: Default::default(),
        }
    }
}
impl<'lt> DeviceMemoryOpaqueCaptureAddressInfo<'lt> {
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
    ///Gets the value of [`Self::memory`]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::memory`]
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
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
    ///Sets the raw value of [`Self::memory`]
    pub fn set_memory(&mut self, value: crate::vulkan1_0::DeviceMemory) -> &mut Self {
        self.memory = value;
        self
    }
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
///   decoration  **can**  have 16-bit integer     and 16-bit floating-point members.     If this
///   feature is not enabled, 16-bit integer or 16-bit floating-point     members  **must**  not be
///   used in such objects.     This also specifies whether shader modules  **can**  declare the
///   `StorageBuffer16BitAccess` capability.
/// - [`uniform_and_storage_buffer_16_bit_access`] specifies whether objects in the `Uniform`
///   storage class with the `Block` decoration  **can**  have 16-bit integer and 16-bit
///   floating-point members. If this feature is not enabled, 16-bit integer or 16-bit
///   floating-point members  **must**  not be used in such objects. This also specifies whether
///   shader modules  **can**  declare the `UniformAndStorageBuffer16BitAccess` capability.
/// - [`storage_push_constant_16`] specifies whether objects in the `PushConstant` storage class
///   **can**  have 16-bit integer and 16-bit floating-point members. If this feature is not
///   enabled, 16-bit integer or floating-point members  **must**  not be used in such objects. This
///   also specifies whether shader modules  **can**  declare the `StoragePushConstant16`
///   capability.
/// - [`storage_input_output_16`] specifies whether objects in the `Input` and `Output` storage
///   classes  **can**  have 16-bit integer and 16-bit floating-point members. If this feature is
///   not enabled, 16-bit integer or 16-bit floating-point members  **must**  not be used in such
///   objects. This also specifies whether shader modules  **can**  declare the
///   `StorageInputOutput16` capability.
/// - [`multiview`] specifies whether the implementation supports multiview rendering within a
///   render pass. If this feature is not enabled, the view mask of each subpass  **must**  always
///   be zero.
/// - [`multiview_geometry_shader`] specifies whether the implementation supports multiview
///   rendering within a render pass, with [geometry shaders](). If this feature is not enabled,
///   then a pipeline compiled against a subpass with a non-zero view mask  **must**  not include a
///   geometry shader.
/// - [`multiview_tessellation_shader`] specifies whether the implementation supports multiview
///   rendering within a render pass, with [tessellation shaders](). If this feature is not enabled,
///   then a pipeline compiled against a subpass with a non-zero view mask  **must**  not include
///   any tessellation shaders.
/// - [`variable_pointers_storage_buffer`] specifies whether the implementation supports the SPIR-V
///   `VariablePointersStorageBuffer` capability. When this feature is not enabled, shader modules
///   **must**  not declare the `SPV_KHR_variable_pointers` extension or the
///   `VariablePointersStorageBuffer` capability.
/// - [`variable_pointers`] specifies whether the implementation supports the SPIR-V
///   `VariablePointers` capability. When this feature is not enabled, shader modules  **must**  not
///   declare the `VariablePointers` capability.
/// - [`protected_memory`] specifies whether protected memory is supported.
/// - [`sampler_ycbcr_conversion`] specifies whether the implementation supports [sampler
///   Y′C<sub>B</sub>C<sub>R</sub> conversion](). If [`sampler_ycbcr_conversion`] is [`FALSE`],
///   sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is not supported, and samplers using sampler
///   Y′C<sub>B</sub>C<sub>R</sub> conversion  **must**  not be used.
/// - [`shader_draw_parameters`] specifies whether the implementation supports the SPIR-V
///   `DrawParameters` capability. When this feature is not enabled, shader modules  **must**  not
///   declare the `SPV_KHR_shader_draw_parameters` extension or the `DrawParameters` capability.
///If the [`PhysicalDeviceVulkan11Features`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVulkan11Features`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceVulkan11Features")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Features<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`storage_buffer_16_bit_access`] specifies whether objects in the
    ///    `StorageBuffer`,
    ///`ShaderRecordBufferKHR`,
    ///    or `PhysicalStorageBuffer`
    ///    storage class with the `Block` decoration  **can**  have 16-bit integer
    ///    and 16-bit floating-point members.
    ///    If this feature is not enabled, 16-bit integer or 16-bit floating-point
    ///    members  **must**  not be used in such objects.
    ///    This also specifies whether shader modules  **can**  declare the
    ///    `StorageBuffer16BitAccess` capability.
    pub storage_buffer_16_bit_access: Bool32,
    ///[`uniform_and_storage_buffer_16_bit_access`] specifies whether objects in
    ///the `Uniform` storage class with the `Block` decoration  **can**  have
    ///16-bit integer and 16-bit floating-point members.
    ///If this feature is not enabled, 16-bit integer or 16-bit floating-point
    ///members  **must**  not be used in such objects.
    ///This also specifies whether shader modules  **can**  declare the
    ///`UniformAndStorageBuffer16BitAccess` capability.
    pub uniform_and_storage_buffer_16_bit_access: Bool32,
    ///[`storage_push_constant_16`] specifies whether objects in the
    ///`PushConstant` storage class  **can**  have 16-bit integer and 16-bit
    ///floating-point members.
    ///If this feature is not enabled, 16-bit integer or floating-point members
    /// **must**  not be used in such objects.
    ///This also specifies whether shader modules  **can**  declare the
    ///`StoragePushConstant16` capability.
    pub storage_push_constant_16: Bool32,
    ///[`storage_input_output_16`] specifies whether objects in the `Input`
    ///and `Output` storage classes  **can**  have 16-bit integer and 16-bit
    ///floating-point members.
    ///If this feature is not enabled, 16-bit integer or 16-bit floating-point
    ///members  **must**  not be used in such objects.
    ///This also specifies whether shader modules  **can**  declare the
    ///`StorageInputOutput16` capability.
    pub storage_input_output_16: Bool32,
    ///[`multiview`] specifies whether
    ///the implementation supports multiview rendering within a render pass.
    ///If this feature is not enabled, the view mask of each subpass  **must**
    ///always be zero.
    pub multiview: Bool32,
    ///[`multiview_geometry_shader`]
    ///specifies whether the implementation supports multiview rendering within
    ///a render pass, with [geometry shaders]().
    ///If this feature is not enabled, then a pipeline compiled against a
    ///subpass with a non-zero view mask  **must**  not include a geometry shader.
    pub multiview_geometry_shader: Bool32,
    ///[`multiview_tessellation_shader`] specifies whether the implementation
    ///supports multiview rendering within a render pass, with
    ///[tessellation shaders]().
    ///If this feature is not enabled, then a pipeline compiled against a
    ///subpass with a non-zero view mask  **must**  not include any tessellation
    ///shaders.
    pub multiview_tessellation_shader: Bool32,
    ///[`variable_pointers_storage_buffer`] specifies whether the implementation
    ///supports the SPIR-V `VariablePointersStorageBuffer` capability.
    ///When this feature is not enabled, shader modules  **must**  not declare the
    ///`SPV_KHR_variable_pointers` extension or the
    ///`VariablePointersStorageBuffer` capability.
    pub variable_pointers_storage_buffer: Bool32,
    ///[`variable_pointers`]
    ///specifies whether the implementation supports the SPIR-V
    ///`VariablePointers` capability.
    ///When this feature is not enabled, shader modules  **must**  not declare the
    ///`VariablePointers` capability.
    pub variable_pointers: Bool32,
    ///[`protected_memory`]
    ///specifies whether protected memory is supported.
    pub protected_memory: Bool32,
    ///[`sampler_ycbcr_conversion`] specifies whether the implementation
    ///supports [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion]().
    ///If [`sampler_ycbcr_conversion`] is [`FALSE`], sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///conversion is not supported, and samplers using sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///conversion  **must**  not be used.
    pub sampler_ycbcr_conversion: Bool32,
    ///[`shader_draw_parameters`] specifies whether the implementation supports
    ///the SPIR-V `DrawParameters` capability.
    ///When this feature is not enabled, shader modules  **must**  not declare the
    ///`SPV_KHR_shader_draw_parameters` extension or the `DrawParameters`
    ///capability.
    pub shader_draw_parameters: Bool32,
}
impl<'lt> Default for PhysicalDeviceVulkan11Features<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceVulkan11Features,
            p_next: std::ptr::null_mut(),
            storage_buffer_16_bit_access: 0,
            uniform_and_storage_buffer_16_bit_access: 0,
            storage_push_constant_16: 0,
            storage_input_output_16: 0,
            multiview: 0,
            multiview_geometry_shader: 0,
            multiview_tessellation_shader: 0,
            variable_pointers_storage_buffer: 0,
            variable_pointers: 0,
            protected_memory: 0,
            sampler_ycbcr_conversion: 0,
            shader_draw_parameters: 0,
        }
    }
}
impl<'lt> PhysicalDeviceVulkan11Features<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::storage_buffer_16_bit_access`]
    pub fn storage_buffer_16_bit_access_raw(&self) -> Bool32 {
        self.storage_buffer_16_bit_access
    }
    ///Gets the raw value of [`Self::uniform_and_storage_buffer_16_bit_access`]
    pub fn uniform_and_storage_buffer_16_bit_access_raw(&self) -> Bool32 {
        self.uniform_and_storage_buffer_16_bit_access
    }
    ///Gets the raw value of [`Self::storage_push_constant_16`]
    pub fn storage_push_constant_16_raw(&self) -> Bool32 {
        self.storage_push_constant_16
    }
    ///Gets the raw value of [`Self::storage_input_output_16`]
    pub fn storage_input_output_16_raw(&self) -> Bool32 {
        self.storage_input_output_16
    }
    ///Gets the raw value of [`Self::multiview`]
    pub fn multiview_raw(&self) -> Bool32 {
        self.multiview
    }
    ///Gets the raw value of [`Self::multiview_geometry_shader`]
    pub fn multiview_geometry_shader_raw(&self) -> Bool32 {
        self.multiview_geometry_shader
    }
    ///Gets the raw value of [`Self::multiview_tessellation_shader`]
    pub fn multiview_tessellation_shader_raw(&self) -> Bool32 {
        self.multiview_tessellation_shader
    }
    ///Gets the raw value of [`Self::variable_pointers_storage_buffer`]
    pub fn variable_pointers_storage_buffer_raw(&self) -> Bool32 {
        self.variable_pointers_storage_buffer
    }
    ///Gets the raw value of [`Self::variable_pointers`]
    pub fn variable_pointers_raw(&self) -> Bool32 {
        self.variable_pointers
    }
    ///Gets the raw value of [`Self::protected_memory`]
    pub fn protected_memory_raw(&self) -> Bool32 {
        self.protected_memory
    }
    ///Gets the raw value of [`Self::sampler_ycbcr_conversion`]
    pub fn sampler_ycbcr_conversion_raw(&self) -> Bool32 {
        self.sampler_ycbcr_conversion
    }
    ///Gets the raw value of [`Self::shader_draw_parameters`]
    pub fn shader_draw_parameters_raw(&self) -> Bool32 {
        self.shader_draw_parameters
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::storage_buffer_16_bit_access`]
    pub fn set_storage_buffer_16_bit_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.storage_buffer_16_bit_access = value;
        self
    }
    ///Sets the raw value of [`Self::uniform_and_storage_buffer_16_bit_access`]
    pub fn set_uniform_and_storage_buffer_16_bit_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.uniform_and_storage_buffer_16_bit_access = value;
        self
    }
    ///Sets the raw value of [`Self::storage_push_constant_16`]
    pub fn set_storage_push_constant_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.storage_push_constant_16 = value;
        self
    }
    ///Sets the raw value of [`Self::storage_input_output_16`]
    pub fn set_storage_input_output_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.storage_input_output_16 = value;
        self
    }
    ///Sets the raw value of [`Self::multiview`]
    pub fn set_multiview_raw(&mut self, value: Bool32) -> &mut Self {
        self.multiview = value;
        self
    }
    ///Sets the raw value of [`Self::multiview_geometry_shader`]
    pub fn set_multiview_geometry_shader_raw(&mut self, value: Bool32) -> &mut Self {
        self.multiview_geometry_shader = value;
        self
    }
    ///Sets the raw value of [`Self::multiview_tessellation_shader`]
    pub fn set_multiview_tessellation_shader_raw(&mut self, value: Bool32) -> &mut Self {
        self.multiview_tessellation_shader = value;
        self
    }
    ///Sets the raw value of [`Self::variable_pointers_storage_buffer`]
    pub fn set_variable_pointers_storage_buffer_raw(&mut self, value: Bool32) -> &mut Self {
        self.variable_pointers_storage_buffer = value;
        self
    }
    ///Sets the raw value of [`Self::variable_pointers`]
    pub fn set_variable_pointers_raw(&mut self, value: Bool32) -> &mut Self {
        self.variable_pointers = value;
        self
    }
    ///Sets the raw value of [`Self::protected_memory`]
    pub fn set_protected_memory_raw(&mut self, value: Bool32) -> &mut Self {
        self.protected_memory = value;
        self
    }
    ///Sets the raw value of [`Self::sampler_ycbcr_conversion`]
    pub fn set_sampler_ycbcr_conversion_raw(&mut self, value: Bool32) -> &mut Self {
        self.sampler_ycbcr_conversion = value;
        self
    }
    ///Sets the raw value of [`Self::shader_draw_parameters`]
    pub fn set_shader_draw_parameters_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_draw_parameters = value;
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
    ///Gets the value of [`Self::storage_buffer_16_bit_access`]
    pub fn storage_buffer_16_bit_access(&self) -> bool {
        unsafe { std::mem::transmute(self.storage_buffer_16_bit_access as u8) }
    }
    ///Gets the value of [`Self::uniform_and_storage_buffer_16_bit_access`]
    pub fn uniform_and_storage_buffer_16_bit_access(&self) -> bool {
        unsafe { std::mem::transmute(self.uniform_and_storage_buffer_16_bit_access as u8) }
    }
    ///Gets the value of [`Self::storage_push_constant_16`]
    pub fn storage_push_constant_16(&self) -> bool {
        unsafe { std::mem::transmute(self.storage_push_constant_16 as u8) }
    }
    ///Gets the value of [`Self::storage_input_output_16`]
    pub fn storage_input_output_16(&self) -> bool {
        unsafe { std::mem::transmute(self.storage_input_output_16 as u8) }
    }
    ///Gets the value of [`Self::multiview`]
    pub fn multiview(&self) -> bool {
        unsafe { std::mem::transmute(self.multiview as u8) }
    }
    ///Gets the value of [`Self::multiview_geometry_shader`]
    pub fn multiview_geometry_shader(&self) -> bool {
        unsafe { std::mem::transmute(self.multiview_geometry_shader as u8) }
    }
    ///Gets the value of [`Self::multiview_tessellation_shader`]
    pub fn multiview_tessellation_shader(&self) -> bool {
        unsafe { std::mem::transmute(self.multiview_tessellation_shader as u8) }
    }
    ///Gets the value of [`Self::variable_pointers_storage_buffer`]
    pub fn variable_pointers_storage_buffer(&self) -> bool {
        unsafe { std::mem::transmute(self.variable_pointers_storage_buffer as u8) }
    }
    ///Gets the value of [`Self::variable_pointers`]
    pub fn variable_pointers(&self) -> bool {
        unsafe { std::mem::transmute(self.variable_pointers as u8) }
    }
    ///Gets the value of [`Self::protected_memory`]
    pub fn protected_memory(&self) -> bool {
        unsafe { std::mem::transmute(self.protected_memory as u8) }
    }
    ///Gets the value of [`Self::sampler_ycbcr_conversion`]
    pub fn sampler_ycbcr_conversion(&self) -> bool {
        unsafe { std::mem::transmute(self.sampler_ycbcr_conversion as u8) }
    }
    ///Gets the value of [`Self::shader_draw_parameters`]
    pub fn shader_draw_parameters(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_draw_parameters as u8) }
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
    ///Gets a mutable reference to the value of [`Self::storage_buffer_16_bit_access`]
    pub fn storage_buffer_16_bit_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.storage_buffer_16_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.storage_buffer_16_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::uniform_and_storage_buffer_16_bit_access`]
    pub fn uniform_and_storage_buffer_16_bit_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.uniform_and_storage_buffer_16_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.uniform_and_storage_buffer_16_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::storage_push_constant_16`]
    pub fn storage_push_constant_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.storage_push_constant_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.storage_push_constant_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::storage_input_output_16`]
    pub fn storage_input_output_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.storage_input_output_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.storage_input_output_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::multiview`]
    pub fn multiview_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.multiview as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.multiview as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::multiview_geometry_shader`]
    pub fn multiview_geometry_shader_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.multiview_geometry_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.multiview_geometry_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::multiview_tessellation_shader`]
    pub fn multiview_tessellation_shader_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.multiview_tessellation_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.multiview_tessellation_shader as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::variable_pointers_storage_buffer`]
    pub fn variable_pointers_storage_buffer_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.variable_pointers_storage_buffer as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.variable_pointers_storage_buffer as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::variable_pointers`]
    pub fn variable_pointers_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.variable_pointers as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.variable_pointers as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::protected_memory`]
    pub fn protected_memory_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.protected_memory as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.protected_memory as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::sampler_ycbcr_conversion`]
    pub fn sampler_ycbcr_conversion_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.sampler_ycbcr_conversion as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.sampler_ycbcr_conversion as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_draw_parameters`]
    pub fn shader_draw_parameters_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_draw_parameters as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_draw_parameters as *mut Bool32)
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
    ///Sets the raw value of [`Self::storage_buffer_16_bit_access`]
    pub fn set_storage_buffer_16_bit_access(&mut self, value: bool) -> &mut Self {
        self.storage_buffer_16_bit_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::uniform_and_storage_buffer_16_bit_access`]
    pub fn set_uniform_and_storage_buffer_16_bit_access(&mut self, value: bool) -> &mut Self {
        self.uniform_and_storage_buffer_16_bit_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::storage_push_constant_16`]
    pub fn set_storage_push_constant_16(&mut self, value: bool) -> &mut Self {
        self.storage_push_constant_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::storage_input_output_16`]
    pub fn set_storage_input_output_16(&mut self, value: bool) -> &mut Self {
        self.storage_input_output_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::multiview`]
    pub fn set_multiview(&mut self, value: bool) -> &mut Self {
        self.multiview = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::multiview_geometry_shader`]
    pub fn set_multiview_geometry_shader(&mut self, value: bool) -> &mut Self {
        self.multiview_geometry_shader = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::multiview_tessellation_shader`]
    pub fn set_multiview_tessellation_shader(&mut self, value: bool) -> &mut Self {
        self.multiview_tessellation_shader = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::variable_pointers_storage_buffer`]
    pub fn set_variable_pointers_storage_buffer(&mut self, value: bool) -> &mut Self {
        self.variable_pointers_storage_buffer = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::variable_pointers`]
    pub fn set_variable_pointers(&mut self, value: bool) -> &mut Self {
        self.variable_pointers = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::protected_memory`]
    pub fn set_protected_memory(&mut self, value: bool) -> &mut Self {
        self.protected_memory = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::sampler_ycbcr_conversion`]
    pub fn set_sampler_ycbcr_conversion(&mut self, value: bool) -> &mut Self {
        self.sampler_ycbcr_conversion = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_draw_parameters`]
    pub fn set_shader_draw_parameters(&mut self, value: bool) -> &mut Self {
        self.shader_draw_parameters = value as u8 as u32;
        self
    }
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
/// - [`max_multiview_view_count`] is one greater than the maximum view index that  **can**  be used
///   in a subpass.
/// - [`max_multiview_instance_index`] is the maximum valid value of instance index allowed to be
///   generated by a drawing command recorded within a subpass of a multiview render pass instance.
/// - [`protected_no_fault`] specifies how an implementation behaves when an application attempts to
///   write to unprotected memory in a protected queue operation, read from protected memory in an
///   unprotected queue operation, or perform a query in a protected queue operation. If this limit
///   is [`TRUE`], such writes will be discarded or have undefined values written, reads and queries
///   will return undefined values. If this limit is [`FALSE`], applications  **must**  not perform
///   these operations. See [[memory-protected-access-rules]]() for more information.
/// - [`max_per_set_descriptors`] is a maximum number of descriptors (summed over all descriptor
///   types) in a single descriptor set that is guaranteed to satisfy any implementation-dependent
///   constraints on the size of a descriptor set itself. Applications  **can**  query whether a
///   descriptor set that goes beyond this limit is supported using
///   [`GetDescriptorSetLayoutSupport`].
/// - [`max_memory_allocation_size`] is the maximum size of a memory allocation that  **can**  be
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
///[`PhysicalDeviceMaintenance3Properties`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES`
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
#[doc(alias = "VkPhysicalDeviceVulkan11Properties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Properties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub device_uuid: [u8; UUID_SIZE as usize],
    ///No documentation found
    pub driver_uuid: [u8; UUID_SIZE as usize],
    ///No documentation found
    pub device_luid: [u8; LUID_SIZE as usize],
    ///No documentation found
    pub device_node_mask: u32,
    ///No documentation found
    pub device_luid_valid: Bool32,
    ///No documentation found
    pub subgroup_size: u32,
    ///No documentation found
    pub subgroup_supported_stages: ShaderStageFlags,
    ///No documentation found
    pub subgroup_supported_operations: SubgroupFeatureFlags,
    ///No documentation found
    pub subgroup_quad_operations_in_all_stages: Bool32,
    ///No documentation found
    pub point_clipping_behavior: PointClippingBehavior,
    ///No documentation found
    pub max_multiview_view_count: u32,
    ///No documentation found
    pub max_multiview_instance_index: u32,
    ///No documentation found
    pub protected_no_fault: Bool32,
    ///No documentation found
    pub max_per_set_descriptors: u32,
    ///No documentation found
    pub max_memory_allocation_size: DeviceSize,
}
impl<'lt> Default for PhysicalDeviceVulkan11Properties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceVulkan11Properties,
            p_next: std::ptr::null_mut(),
            device_uuid: [0; UUID_SIZE as usize],
            driver_uuid: [0; UUID_SIZE as usize],
            device_luid: [0; LUID_SIZE as usize],
            device_node_mask: 0,
            device_luid_valid: 0,
            subgroup_size: 0,
            subgroup_supported_stages: Default::default(),
            subgroup_supported_operations: Default::default(),
            subgroup_quad_operations_in_all_stages: 0,
            point_clipping_behavior: Default::default(),
            max_multiview_view_count: 0,
            max_multiview_instance_index: 0,
            protected_no_fault: 0,
            max_per_set_descriptors: 0,
            max_memory_allocation_size: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceVulkan11Properties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::device_luid_valid`]
    pub fn device_luid_valid_raw(&self) -> Bool32 {
        self.device_luid_valid
    }
    ///Gets the raw value of [`Self::subgroup_quad_operations_in_all_stages`]
    pub fn subgroup_quad_operations_in_all_stages_raw(&self) -> Bool32 {
        self.subgroup_quad_operations_in_all_stages
    }
    ///Gets the raw value of [`Self::protected_no_fault`]
    pub fn protected_no_fault_raw(&self) -> Bool32 {
        self.protected_no_fault
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_luid_valid`]
    pub fn set_device_luid_valid_raw(&mut self, value: Bool32) -> &mut Self {
        self.device_luid_valid = value;
        self
    }
    ///Sets the raw value of [`Self::subgroup_quad_operations_in_all_stages`]
    pub fn set_subgroup_quad_operations_in_all_stages_raw(&mut self, value: Bool32) -> &mut Self {
        self.subgroup_quad_operations_in_all_stages = value;
        self
    }
    ///Sets the raw value of [`Self::protected_no_fault`]
    pub fn set_protected_no_fault_raw(&mut self, value: Bool32) -> &mut Self {
        self.protected_no_fault = value;
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
    ///Gets the value of [`Self::device_uuid`]
    pub fn device_uuid(&self) -> &[u8; UUID_SIZE as usize] {
        &self.device_uuid
    }
    ///Gets the value of [`Self::driver_uuid`]
    pub fn driver_uuid(&self) -> &[u8; UUID_SIZE as usize] {
        &self.driver_uuid
    }
    ///Gets the value of [`Self::device_luid`]
    pub fn device_luid(&self) -> &[u8; LUID_SIZE as usize] {
        &self.device_luid
    }
    ///Gets the value of [`Self::device_node_mask`]
    pub fn device_node_mask(&self) -> u32 {
        self.device_node_mask
    }
    ///Gets the value of [`Self::device_luid_valid`]
    pub fn device_luid_valid(&self) -> bool {
        unsafe { std::mem::transmute(self.device_luid_valid as u8) }
    }
    ///Gets the value of [`Self::subgroup_size`]
    pub fn subgroup_size(&self) -> u32 {
        self.subgroup_size
    }
    ///Gets the value of [`Self::subgroup_supported_stages`]
    pub fn subgroup_supported_stages(&self) -> ShaderStageFlags {
        self.subgroup_supported_stages
    }
    ///Gets the value of [`Self::subgroup_supported_operations`]
    pub fn subgroup_supported_operations(&self) -> SubgroupFeatureFlags {
        self.subgroup_supported_operations
    }
    ///Gets the value of [`Self::subgroup_quad_operations_in_all_stages`]
    pub fn subgroup_quad_operations_in_all_stages(&self) -> bool {
        unsafe { std::mem::transmute(self.subgroup_quad_operations_in_all_stages as u8) }
    }
    ///Gets the value of [`Self::point_clipping_behavior`]
    pub fn point_clipping_behavior(&self) -> PointClippingBehavior {
        self.point_clipping_behavior
    }
    ///Gets the value of [`Self::max_multiview_view_count`]
    pub fn max_multiview_view_count(&self) -> u32 {
        self.max_multiview_view_count
    }
    ///Gets the value of [`Self::max_multiview_instance_index`]
    pub fn max_multiview_instance_index(&self) -> u32 {
        self.max_multiview_instance_index
    }
    ///Gets the value of [`Self::protected_no_fault`]
    pub fn protected_no_fault(&self) -> bool {
        unsafe { std::mem::transmute(self.protected_no_fault as u8) }
    }
    ///Gets the value of [`Self::max_per_set_descriptors`]
    pub fn max_per_set_descriptors(&self) -> u32 {
        self.max_per_set_descriptors
    }
    ///Gets the value of [`Self::max_memory_allocation_size`]
    pub fn max_memory_allocation_size(&self) -> DeviceSize {
        self.max_memory_allocation_size
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
    ///Gets a mutable reference to the value of [`Self::device_uuid`]
    pub fn device_uuid_mut(&mut self) -> &mut [u8; UUID_SIZE as usize] {
        &mut self.device_uuid
    }
    ///Gets a mutable reference to the value of [`Self::driver_uuid`]
    pub fn driver_uuid_mut(&mut self) -> &mut [u8; UUID_SIZE as usize] {
        &mut self.driver_uuid
    }
    ///Gets a mutable reference to the value of [`Self::device_luid`]
    pub fn device_luid_mut(&mut self) -> &mut [u8; LUID_SIZE as usize] {
        &mut self.device_luid
    }
    ///Gets a mutable reference to the value of [`Self::device_node_mask`]
    pub fn device_node_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_node_mask
    }
    ///Gets a mutable reference to the value of [`Self::device_luid_valid`]
    pub fn device_luid_valid_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.device_luid_valid as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.device_luid_valid as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::subgroup_size`]
    pub fn subgroup_size_mut(&mut self) -> &mut u32 {
        &mut self.subgroup_size
    }
    ///Gets a mutable reference to the value of [`Self::subgroup_supported_stages`]
    pub fn subgroup_supported_stages_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.subgroup_supported_stages
    }
    ///Gets a mutable reference to the value of [`Self::subgroup_supported_operations`]
    pub fn subgroup_supported_operations_mut(&mut self) -> &mut SubgroupFeatureFlags {
        &mut self.subgroup_supported_operations
    }
    ///Gets a mutable reference to the value of [`Self::subgroup_quad_operations_in_all_stages`]
    pub fn subgroup_quad_operations_in_all_stages_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.subgroup_quad_operations_in_all_stages as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.subgroup_quad_operations_in_all_stages as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::point_clipping_behavior`]
    pub fn point_clipping_behavior_mut(&mut self) -> &mut PointClippingBehavior {
        &mut self.point_clipping_behavior
    }
    ///Gets a mutable reference to the value of [`Self::max_multiview_view_count`]
    pub fn max_multiview_view_count_mut(&mut self) -> &mut u32 {
        &mut self.max_multiview_view_count
    }
    ///Gets a mutable reference to the value of [`Self::max_multiview_instance_index`]
    pub fn max_multiview_instance_index_mut(&mut self) -> &mut u32 {
        &mut self.max_multiview_instance_index
    }
    ///Gets a mutable reference to the value of [`Self::protected_no_fault`]
    pub fn protected_no_fault_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.protected_no_fault as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.protected_no_fault as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::max_per_set_descriptors`]
    pub fn max_per_set_descriptors_mut(&mut self) -> &mut u32 {
        &mut self.max_per_set_descriptors
    }
    ///Gets a mutable reference to the value of [`Self::max_memory_allocation_size`]
    pub fn max_memory_allocation_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.max_memory_allocation_size
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
    ///Sets the raw value of [`Self::device_uuid`]
    pub fn set_device_uuid(&mut self, value: [u8; crate::core::UUID_SIZE as usize]) -> &mut Self {
        self.device_uuid = value;
        self
    }
    ///Sets the raw value of [`Self::driver_uuid`]
    pub fn set_driver_uuid(&mut self, value: [u8; crate::core::UUID_SIZE as usize]) -> &mut Self {
        self.driver_uuid = value;
        self
    }
    ///Sets the raw value of [`Self::device_luid`]
    pub fn set_device_luid(&mut self, value: [u8; crate::vulkan1_1::LUID_SIZE as usize]) -> &mut Self {
        self.device_luid = value;
        self
    }
    ///Sets the raw value of [`Self::device_node_mask`]
    pub fn set_device_node_mask(&mut self, value: u32) -> &mut Self {
        self.device_node_mask = value;
        self
    }
    ///Sets the raw value of [`Self::device_luid_valid`]
    pub fn set_device_luid_valid(&mut self, value: bool) -> &mut Self {
        self.device_luid_valid = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::subgroup_size`]
    pub fn set_subgroup_size(&mut self, value: u32) -> &mut Self {
        self.subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::subgroup_supported_stages`]
    pub fn set_subgroup_supported_stages(&mut self, value: crate::vulkan1_0::ShaderStageFlags) -> &mut Self {
        self.subgroup_supported_stages = value;
        self
    }
    ///Sets the raw value of [`Self::subgroup_supported_operations`]
    pub fn set_subgroup_supported_operations(&mut self, value: crate::vulkan1_1::SubgroupFeatureFlags) -> &mut Self {
        self.subgroup_supported_operations = value;
        self
    }
    ///Sets the raw value of [`Self::subgroup_quad_operations_in_all_stages`]
    pub fn set_subgroup_quad_operations_in_all_stages(&mut self, value: bool) -> &mut Self {
        self.subgroup_quad_operations_in_all_stages = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::point_clipping_behavior`]
    pub fn set_point_clipping_behavior(&mut self, value: crate::vulkan1_1::PointClippingBehavior) -> &mut Self {
        self.point_clipping_behavior = value;
        self
    }
    ///Sets the raw value of [`Self::max_multiview_view_count`]
    pub fn set_max_multiview_view_count(&mut self, value: u32) -> &mut Self {
        self.max_multiview_view_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_multiview_instance_index`]
    pub fn set_max_multiview_instance_index(&mut self, value: u32) -> &mut Self {
        self.max_multiview_instance_index = value;
        self
    }
    ///Sets the raw value of [`Self::protected_no_fault`]
    pub fn set_protected_no_fault(&mut self, value: bool) -> &mut Self {
        self.protected_no_fault = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_per_set_descriptors`]
    pub fn set_max_per_set_descriptors(&mut self, value: u32) -> &mut Self {
        self.max_per_set_descriptors = value;
        self
    }
    ///Sets the raw value of [`Self::max_memory_allocation_size`]
    pub fn set_max_memory_allocation_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.max_memory_allocation_size = value;
        self
    }
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
///   enabled, the `VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` sampler address mode  **must**
///   not be used.
/// - [`draw_indirect_count`] indicates whether the implementation supports the
///   [`CmdDrawIndirectCount`] and [`CmdDrawIndexedIndirectCount`] functions. If this feature is not
///   enabled, these functions  **must**  not be used.
/// - [`storage_buffer_8_bit_access`] indicates whether objects in the     `StorageBuffer`,
///   `ShaderRecordBufferKHR`,     or `PhysicalStorageBuffer`     storage class with the `Block`
///   decoration  **can**  have 8-bit integer     members.     If this feature is not enabled, 8-bit
///   integer members  **must**  not be used     in such objects.     This also indicates whether
///   shader modules  **can**  declare the     `StorageBuffer8BitAccess` capability.
/// - [`uniform_and_storage_buffer_8_bit_access`] indicates whether objects in the `Uniform` storage
///   class with the `Block` decoration  **can**  have 8-bit integer members. If this feature is not
///   enabled, 8-bit integer members  **must**  not be used in such objects. This also indicates
///   whether shader modules  **can**  declare the `UniformAndStorageBuffer8BitAccess` capability.
/// - [`storage_push_constant_8`] indicates whether objects in the `PushConstant` storage class
///   **can**  have 8-bit integer members. If this feature is not enabled, 8-bit integer members
///   **must**  not be used in such objects. This also indicates whether shader modules  **can**
///   declare the `StoragePushConstant8` capability.
/// - [`shader_buffer_int_64_atomics`] indicates whether shaders  **can**  perform 64-bit unsigned
///   and signed integer atomic operations on buffers.
/// - [`shader_shared_int_64_atomics`] indicates whether shaders  **can**  perform 64-bit unsigned
///   and signed integer atomic operations on shared memory.
/// - [`shader_float_16`] indicates whether 16-bit floats (halfs) are supported in shader code. This
///   also indicates whether shader modules  **can**  declare the `Float16` capability. However,
///   this only enables a subset of the storage classes that SPIR-V allows for the `Float16` SPIR-V
///   capability: Declaring and using 16-bit floats in the `Private`, `Workgroup` (for non-Block
///   variables), and `Function` storage classes is enabled, while declaring them in the interface
///   storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`, `Output`, and
///   `PushConstant`) is not enabled.
/// - [`shader_int_8`] indicates whether 8-bit integers (signed and unsigned) are supported in
///   shader code. This also indicates whether shader modules  **can**  declare the `Int8`
///   capability. However, this only enables a subset of the storage classes that SPIR-V allows for
///   the `Int8` SPIR-V capability: Declaring and using 8-bit integers in the `Private`, `Workgroup`
///   (for non-Block variables), and `Function` storage classes is enabled, while declaring them in
///   the interface storage classes (e.g., `UniformConstant`, `Uniform`, `StorageBuffer`, `Input`,
///   `Output`, and `PushConstant`) is not enabled.
/// - [`descriptor_indexing`] indicates whether the implementation supports the minimum set of descriptor indexing features as described in the [Feature Requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-requirements) section. Enabling the [`descriptor_indexing`] member when [`CreateDevice`] is called does not imply the other minimum descriptor indexing features are also enabled. Those other descriptor indexing features  **must**  be enabled individually as needed by the application.
/// - [`shader_input_attachment_array_dynamic_indexing`] indicates whether arrays of input
///   attachments  **can**  be indexed by dynamically uniform integer expressions in shader code. If
///   this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `InputAttachmentArrayDynamicIndexing` capability.
/// - [`shader_uniform_texel_buffer_array_dynamic_indexing`] indicates whether arrays of uniform
///   texel buffers  **can**  be indexed by dynamically uniform integer expressions in shader code.
///   If this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `UniformTexelBufferArrayDynamicIndexing` capability.
/// - [`shader_storage_texel_buffer_array_dynamic_indexing`] indicates whether arrays of storage
///   texel buffers  **can**  be indexed by dynamically uniform integer expressions in shader code.
///   If this feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  be indexed only by constant integral
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `StorageTexelBufferArrayDynamicIndexing` capability.
/// - [`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform
///   buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` **must**
///   not be indexed by non-uniform integer expressions when aggregated into arrays in shader code.
///   This also indicates whether shader modules  **can**  declare the
///   `UniformBufferArrayNonUniformIndexing` capability.
/// - [`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays of samplers or
///   sampled images  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_SAMPLER`,
///   `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` **must**
///   not be indexed by non-uniform integer expressions when aggregated into arrays in shader code.
///   This also indicates whether shader modules  **can**  declare the
///   `SampledImageArrayNonUniformIndexing` capability.
/// - [`shader_storage_buffer_array_non_uniform_indexing`] indicates whether arrays of storage
///   buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` **must**
///   not be indexed by non-uniform integer expressions when aggregated into arrays in shader code.
///   This also indicates whether shader modules  **can**  declare the
///   `StorageBufferArrayNonUniformIndexing` capability.
/// - [`shader_storage_image_array_non_uniform_indexing`] indicates whether arrays of storage images
///   **can**  be indexed by non-uniform integer expressions in shader code. If this feature is not
///   enabled, resources with a descriptor type of `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` **must**  not
///   be indexed by non-uniform integer expressions when aggregated into arrays in shader code. This
///   also indicates whether shader modules  **can**  declare the
///   `StorageImageArrayNonUniformIndexing` capability.
/// - [`shader_input_attachment_array_non_uniform_indexing`] indicates whether arrays of input
///   attachments  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `InputAttachmentArrayNonUniformIndexing` capability.
/// - [`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of uniform
///   texel buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `UniformTexelBufferArrayNonUniformIndexing` capability.
/// - [`shader_storage_texel_buffer_array_non_uniform_indexing`] indicates whether arrays of storage
///   texel buffers  **can**  be indexed by non-uniform integer expressions in shader code. If this
///   feature is not enabled, resources with a descriptor type of
///   `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  not be indexed by non-uniform integer
///   expressions when aggregated into arrays in shader code. This also indicates whether shader
///   modules  **can**  declare the `StorageTexelBufferArrayNonUniformIndexing` capability.
/// - [`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether the implementation
///   supports updating uniform buffer descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
///   `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.
/// - [`descriptor_binding_sampled_image_update_after_bind`] indicates whether the implementation
///   supports updating sampled image descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
///   `VK_DESCRIPTOR_TYPE_SAMPLER`, `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
///   `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.
/// - [`descriptor_binding_storage_image_update_after_bind`] indicates whether the implementation
///   supports updating storage image descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
///   `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
/// - [`descriptor_binding_storage_buffer_update_after_bind`] indicates whether the implementation
///   supports updating storage buffer descriptors after a set is bound. If this feature is not
///   enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
///   `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`.
/// - [`descriptor_binding_uniform_texel_buffer_update_after_bind`] indicates whether the
///   implementation supports updating uniform texel buffer descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be
///   used with `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
/// - [`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates whether the
///   implementation supports updating storage texel buffer descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be
///   used with `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
/// - [`descriptor_binding_update_unused_while_pending`] indicates whether the implementation
///   supports updating descriptors while the set is in use. If this feature is not enabled,
///   `VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT` **must**  not be used.
/// - [`descriptor_binding_partially_bound`] indicates whether the implementation supports
///   statically using a descriptor set binding in which some descriptors are not valid. If this
///   feature is not enabled, `VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT` **must**  not be used.
/// - [`descriptor_binding_variable_descriptor_count`] indicates whether the implementation supports
///   descriptor sets with a variable-sized last binding. If this feature is not enabled,
///   `VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT` **must**  not be used.
/// - [`runtime_descriptor_array`] indicates whether the implementation supports the SPIR-V
///   `RuntimeDescriptorArray` capability. If this feature is not enabled, descriptors  **must**
///   not be declared in runtime arrays.
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
///   acceleration structure addresses  **must**  not be queried on a logical device created with
///   more than one physical device.
/// - [`vulkan_memory_model`] indicates whether the Vulkan Memory Model is supported, as defined in
///   [Vulkan Memory Model](). This also indicates whether shader modules  **can**  declare the
///   `VulkanMemoryModel` capability.
/// - [`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory Model can use
///   [`Device`] scope synchronization. This also indicates whether shader modules  **can**  declare
///   the `VulkanMemoryModelDeviceScope` capability.
/// - [`vulkan_memory_model_availability_visibility_chains`] indicates whether the Vulkan Memory
///   Model can use [availability and visibility chains]() with more than one element.
/// - [`shader_output_viewport_index`] indicates whether the implementation supports the
///   `ShaderViewportIndex` SPIR-V capability enabling variables decorated with the `ViewportIndex`
///   built-in to be exported from vertex or tessellation evaluation shaders. If this feature is not
///   enabled, the `ViewportIndex` built-in decoration  **must**  not be used on outputs in vertex
///   or tessellation evaluation shaders.
/// - [`shader_output_layer`] indicates whether the implementation supports the `ShaderLayer` SPIR-V
///   capability enabling variables decorated with the `Layer` built-in to be exported from vertex
///   or tessellation evaluation shaders. If this feature is not enabled, the `Layer` built-in
///   decoration  **must**  not be used on outputs in vertex or tessellation evaluation shaders.
/// - If [`subgroup_broadcast_dynamic_id`] is [`TRUE`], the “Id” operand of
///   `OpGroupNonUniformBroadcast` **can**  be dynamically uniform within a subgroup, and the
///   “Index” operand of `OpGroupNonUniformQuadBroadcast` **can**  be dynamically uniform within the
///   derivative group. If it is [`FALSE`], these operands  **must**  be constants.
///If the [`PhysicalDeviceVulkan12Features`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVulkan12Features`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES`
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
#[doc(alias = "VkPhysicalDeviceVulkan12Features")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Features<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`sampler_mirror_clamp_to_edge`]
    ///indicates whether the implementation supports the
    ///`VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` sampler address mode.
    ///If this feature is not enabled, the
    ///`VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE` sampler address mode
    /// **must**  not be used.
    pub sampler_mirror_clamp_to_edge: Bool32,
    ///[`draw_indirect_count`] indicates whether
    ///the implementation supports the [`CmdDrawIndirectCount`] and
    ///[`CmdDrawIndexedIndirectCount`] functions.
    ///If this feature is not enabled, these functions  **must**  not be used.
    pub draw_indirect_count: Bool32,
    ///[`storage_buffer_8_bit_access`] indicates whether objects in the
    ///    `StorageBuffer`,
    ///`ShaderRecordBufferKHR`,
    ///    or `PhysicalStorageBuffer`
    ///    storage class with the `Block` decoration  **can**  have 8-bit integer
    ///    members.
    ///    If this feature is not enabled, 8-bit integer members  **must**  not be used
    ///    in such objects.
    ///    This also indicates whether shader modules  **can**  declare the
    ///    `StorageBuffer8BitAccess` capability.
    pub storage_buffer_8_bit_access: Bool32,
    ///[`uniform_and_storage_buffer_8_bit_access`] indicates whether objects in the
    ///`Uniform` storage class with the `Block` decoration  **can**  have
    ///8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members  **must**  not be used
    ///in such objects.
    ///This also indicates whether shader modules  **can**  declare the
    ///`UniformAndStorageBuffer8BitAccess` capability.
    pub uniform_and_storage_buffer_8_bit_access: Bool32,
    ///[`storage_push_constant_8`] indicates whether objects in the
    ///`PushConstant` storage class  **can**  have 8-bit integer members.
    ///If this feature is not enabled, 8-bit integer members  **must**  not be used
    ///in such objects.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StoragePushConstant8` capability.
    pub storage_push_constant_8: Bool32,
    ///[`shader_buffer_int_64_atomics`] indicates whether shaders  **can**  perform
    ///64-bit unsigned and signed integer atomic operations on buffers.
    pub shader_buffer_int_64_atomics: Bool32,
    ///[`shader_shared_int_64_atomics`] indicates whether shaders  **can**  perform
    ///64-bit unsigned and signed integer atomic operations on shared memory.
    pub shader_shared_int_64_atomics: Bool32,
    ///[`shader_float_16`] indicates
    ///whether 16-bit floats (halfs) are supported in shader code.
    ///This also indicates whether shader modules  **can**  declare the `Float16`
    ///capability.
    ///However, this only enables a subset of the storage classes that SPIR-V
    ///allows for the `Float16` SPIR-V capability: Declaring and using
    ///16-bit floats in the `Private`,
    ///`Workgroup` (for non-Block variables),
    ///and `Function` storage classes is enabled, while declaring them in
    ///the interface storage classes (e.g., `UniformConstant`, `Uniform`,
    ///`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
    ///not enabled.
    pub shader_float_16: Bool32,
    ///[`shader_int_8`] indicates
    ///whether 8-bit integers (signed and unsigned) are supported in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the `Int8`
    ///capability.
    ///However, this only enables a subset of the storage classes that SPIR-V
    ///allows for the `Int8` SPIR-V capability: Declaring and using 8-bit
    ///integers in the `Private`,
    ///`Workgroup` (for non-Block variables),
    ///and `Function` storage classes is enabled, while declaring them in
    ///the interface storage classes (e.g., `UniformConstant`, `Uniform`,
    ///`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
    ///not enabled.
    pub shader_int_8: Bool32,
    ///[`descriptor_indexing`] indicates
    ///whether the implementation supports the minimum set of descriptor
    ///indexing features as described in the [Feature
    ///Requirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-requirements) section.
    ///Enabling the [`descriptor_indexing`] member when [`CreateDevice`]
    ///is called does not imply the other minimum descriptor indexing features
    ///are also enabled.
    ///Those other descriptor indexing features  **must**  be enabled individually
    ///as needed by the application.
    pub descriptor_indexing: Bool32,
    ///[`shader_input_attachment_array_dynamic_indexing`] indicates whether arrays
    ///of input attachments  **can**  be indexed by dynamically uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`InputAttachmentArrayDynamicIndexing` capability.
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    ///[`shader_uniform_texel_buffer_array_dynamic_indexing`] indicates whether
    ///arrays of uniform texel buffers  **can**  be indexed by dynamically uniform
    ///integer expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`UniformTexelBufferArrayDynamicIndexing` capability.
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    ///[`shader_storage_texel_buffer_array_dynamic_indexing`] indicates whether
    ///arrays of storage texel buffers  **can**  be indexed by dynamically uniform
    ///integer expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  be indexed only by
    ///constant integral expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StorageTexelBufferArrayDynamicIndexing` capability.
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    ///[`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of uniform buffers  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`UniformBufferArrayNonUniformIndexing` capability.
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays
    ///of samplers or sampled images  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_SAMPLER`,
    ///`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
    ///`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`SampledImageArrayNonUniformIndexing` capability.
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of storage buffers  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StorageBufferArrayNonUniformIndexing` capability.
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_image_array_non_uniform_indexing`] indicates whether arrays
    ///of storage images  **can**  be indexed by non-uniform integer expressions in
    ///shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StorageImageArrayNonUniformIndexing` capability.
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    ///[`shader_input_attachment_array_non_uniform_indexing`] indicates whether
    ///arrays of input attachments  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`InputAttachmentArrayNonUniformIndexing` capability.
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    ///[`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of uniform texel buffers  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`UniformTexelBufferArrayNonUniformIndexing` capability.
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    ///[`shader_storage_texel_buffer_array_non_uniform_indexing`] indicates whether
    ///arrays of storage texel buffers  **can**  be indexed by non-uniform integer
    ///expressions in shader code.
    ///If this feature is not enabled, resources with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER` **must**  not be indexed by
    ///non-uniform integer expressions when aggregated into arrays in shader
    ///code.
    ///This also indicates whether shader modules  **can**  declare the
    ///`StorageTexelBufferArrayNonUniformIndexing` capability.
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    ///[`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether
    ///the implementation supports updating uniform buffer descriptors after a
    ///set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_sampled_image_update_after_bind`] indicates whether the
    ///implementation supports updating sampled image descriptors after a set
    ///is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_SAMPLER`,
    ///`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
    ///`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_image_update_after_bind`] indicates whether the
    ///implementation supports updating storage image descriptors after a set
    ///is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_buffer_update_after_bind`] indicates whether
    ///the implementation supports updating storage buffer descriptors after a
    ///set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`.
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_uniform_texel_buffer_update_after_bind`] indicates
    ///whether the implementation supports updating uniform texel buffer
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates
    ///whether the implementation supports updating storage texel buffer
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    ///[`descriptor_binding_update_unused_while_pending`] indicates whether the
    ///implementation supports updating descriptors while the set is in use.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT` **must**  not be
    ///used.
    pub descriptor_binding_update_unused_while_pending: Bool32,
    ///[`descriptor_binding_partially_bound`] indicates whether the
    ///implementation supports statically using a descriptor set binding in
    ///which some descriptors are not valid.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT` **must**  not be used.
    pub descriptor_binding_partially_bound: Bool32,
    ///[`descriptor_binding_variable_descriptor_count`] indicates whether the
    ///implementation supports descriptor sets with a variable-sized last
    ///binding.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT` **must**  not be
    ///used.
    pub descriptor_binding_variable_descriptor_count: Bool32,
    ///[`runtime_descriptor_array`] indicates whether the implementation
    ///supports the SPIR-V `RuntimeDescriptorArray` capability.
    ///If this feature is not enabled, descriptors  **must**  not be declared in
    ///runtime arrays.
    pub runtime_descriptor_array: Bool32,
    ///[`sampler_filter_minmax`] indicates
    ///whether the implementation supports a minimum set of required formats
    ///supporting min/max filtering as defined by the
    ///[`filterMinmaxSingleComponentFormats`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-filterMinmaxSingleComponentFormats-minimum-requirements)
    ///property minimum requirements.
    ///If this feature is not enabled, then no [`SamplerCreateInfo`][`p_next`] chain can include a
    /// [`SamplerReductionModeCreateInfo`] structure.
    pub sampler_filter_minmax: Bool32,
    ///[`scalar_block_layout`]
    ///indicates that the implementation supports the layout of resource blocks
    ///in shaders using [scalar
    ///alignment]().
    pub scalar_block_layout: Bool32,
    ///[`imageless_framebuffer`] indicates that the implementation supports
    ///specifying the image view for attachments at render pass begin time via
    ///[`RenderPassAttachmentBeginInfo`].
    pub imageless_framebuffer: Bool32,
    ///[`uniform_buffer_standard_layout`] indicates that the implementation
    ///supports the same layouts for uniform buffers as for storage and other
    ///kinds of buffers.
    ///See [Standard Buffer Layout]().
    pub uniform_buffer_standard_layout: Bool32,
    ///[`shader_subgroup_extended_types`] is a boolean specifying whether
    ///subgroup operations can use 8-bit integer, 16-bit integer, 64-bit
    ///integer, 16-bit floating-point, and vectors of these types in
    ///[group operations]() with
    ///[subgroup scope](), if the implementation
    ///supports the types.
    pub shader_subgroup_extended_types: Bool32,
    ///[`separate_depth_stencil_layouts`] indicates whether the implementation
    ///supports a [`ImageMemoryBarrier`] for a depth/stencil image with
    ///only one of `VK_IMAGE_ASPECT_DEPTH_BIT` or
    ///`VK_IMAGE_ASPECT_STENCIL_BIT` set, and whether
    ///`VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL`,
    ///`VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`,
    ///`VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL`, or
    ///`VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL` can be used.
    pub separate_depth_stencil_layouts: Bool32,
    ///[`host_query_reset`]
    ///indicates that the implementation supports resetting queries from the
    ///host with [`ResetQueryPool`].
    pub host_query_reset: Bool32,
    ///[`timeline_semaphore`]
    ///indicates whether semaphores created with a [`SemaphoreType`] of
    ///`VK_SEMAPHORE_TYPE_TIMELINE` are supported.
    pub timeline_semaphore: Bool32,
    ///[`buffer_device_address`] indicates that the implementation supports
    ///accessing buffer memory in shaders as storage buffers via an address
    ///queried from [`GetBufferDeviceAddress`].
    pub buffer_device_address: Bool32,
    ///[`buffer_device_address_capture_replay`] indicates that the implementation
    ///supports saving and reusing buffer and device addresses, e.g. for trace
    ///capture and replay.
    pub buffer_device_address_capture_replay: Bool32,
    ///[`buffer_device_address_multi_device`] indicates that the implementation
    ///supports the [`buffer_device_address`]
    ///, `rayTracingPipeline` and `rayQuery` features
    ///for logical devices created with multiple physical devices.
    ///If this feature is not supported, buffer
    ///and acceleration structure
    ///addresses  **must**  not be queried on a logical device created with more
    ///than one physical device.
    pub buffer_device_address_multi_device: Bool32,
    ///[`vulkan_memory_model`]
    ///indicates whether the Vulkan Memory Model is supported, as defined in
    ///[Vulkan Memory Model]().
    ///This also indicates whether shader modules  **can**  declare the
    ///`VulkanMemoryModel` capability.
    pub vulkan_memory_model: Bool32,
    ///[`vulkan_memory_model_device_scope`] indicates whether the Vulkan Memory
    ///Model can use [`Device`] scope synchronization.
    ///This also indicates whether shader modules  **can**  declare the
    ///`VulkanMemoryModelDeviceScope` capability.
    pub vulkan_memory_model_device_scope: Bool32,
    ///[`vulkan_memory_model_availability_visibility_chains`] indicates whether
    ///the Vulkan Memory Model can use [availability and visibility chains]() with more than one
    /// element.
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
    ///[`shader_output_viewport_index`]
    ///indicates whether the implementation supports the
    ///`ShaderViewportIndex` SPIR-V capability enabling variables decorated
    ///with the `ViewportIndex` built-in to be exported from vertex or
    ///tessellation evaluation shaders.
    ///If this feature is not enabled, the `ViewportIndex` built-in
    ///decoration  **must**  not be used on outputs in vertex or tessellation
    ///evaluation shaders.
    pub shader_output_viewport_index: Bool32,
    ///[`shader_output_layer`] indicates whether
    ///the implementation supports the `ShaderLayer` SPIR-V capability
    ///enabling variables decorated with the `Layer` built-in to be exported
    ///from vertex or tessellation evaluation shaders.
    ///If this feature is not enabled, the `Layer` built-in decoration  **must**
    ///not be used on outputs in vertex or tessellation evaluation shaders.
    pub shader_output_layer: Bool32,
    ///If
    ///[`subgroup_broadcast_dynamic_id`] is [`TRUE`], the “Id” operand of
    ///`OpGroupNonUniformBroadcast` **can**  be dynamically uniform within a
    ///subgroup, and the “Index” operand of
    ///`OpGroupNonUniformQuadBroadcast` **can**  be dynamically uniform within
    ///the derivative group.
    ///If it is [`FALSE`], these operands  **must**  be constants.
    pub subgroup_broadcast_dynamic_id: Bool32,
}
impl<'lt> Default for PhysicalDeviceVulkan12Features<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceVulkan12Features,
            p_next: std::ptr::null_mut(),
            sampler_mirror_clamp_to_edge: 0,
            draw_indirect_count: 0,
            storage_buffer_8_bit_access: 0,
            uniform_and_storage_buffer_8_bit_access: 0,
            storage_push_constant_8: 0,
            shader_buffer_int_64_atomics: 0,
            shader_shared_int_64_atomics: 0,
            shader_float_16: 0,
            shader_int_8: 0,
            descriptor_indexing: 0,
            shader_input_attachment_array_dynamic_indexing: 0,
            shader_uniform_texel_buffer_array_dynamic_indexing: 0,
            shader_storage_texel_buffer_array_dynamic_indexing: 0,
            shader_uniform_buffer_array_non_uniform_indexing: 0,
            shader_sampled_image_array_non_uniform_indexing: 0,
            shader_storage_buffer_array_non_uniform_indexing: 0,
            shader_storage_image_array_non_uniform_indexing: 0,
            shader_input_attachment_array_non_uniform_indexing: 0,
            shader_uniform_texel_buffer_array_non_uniform_indexing: 0,
            shader_storage_texel_buffer_array_non_uniform_indexing: 0,
            descriptor_binding_uniform_buffer_update_after_bind: 0,
            descriptor_binding_sampled_image_update_after_bind: 0,
            descriptor_binding_storage_image_update_after_bind: 0,
            descriptor_binding_storage_buffer_update_after_bind: 0,
            descriptor_binding_uniform_texel_buffer_update_after_bind: 0,
            descriptor_binding_storage_texel_buffer_update_after_bind: 0,
            descriptor_binding_update_unused_while_pending: 0,
            descriptor_binding_partially_bound: 0,
            descriptor_binding_variable_descriptor_count: 0,
            runtime_descriptor_array: 0,
            sampler_filter_minmax: 0,
            scalar_block_layout: 0,
            imageless_framebuffer: 0,
            uniform_buffer_standard_layout: 0,
            shader_subgroup_extended_types: 0,
            separate_depth_stencil_layouts: 0,
            host_query_reset: 0,
            timeline_semaphore: 0,
            buffer_device_address: 0,
            buffer_device_address_capture_replay: 0,
            buffer_device_address_multi_device: 0,
            vulkan_memory_model: 0,
            vulkan_memory_model_device_scope: 0,
            vulkan_memory_model_availability_visibility_chains: 0,
            shader_output_viewport_index: 0,
            shader_output_layer: 0,
            subgroup_broadcast_dynamic_id: 0,
        }
    }
}
impl<'lt> PhysicalDeviceVulkan12Features<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::sampler_mirror_clamp_to_edge`]
    pub fn sampler_mirror_clamp_to_edge_raw(&self) -> Bool32 {
        self.sampler_mirror_clamp_to_edge
    }
    ///Gets the raw value of [`Self::draw_indirect_count`]
    pub fn draw_indirect_count_raw(&self) -> Bool32 {
        self.draw_indirect_count
    }
    ///Gets the raw value of [`Self::storage_buffer_8_bit_access`]
    pub fn storage_buffer_8_bit_access_raw(&self) -> Bool32 {
        self.storage_buffer_8_bit_access
    }
    ///Gets the raw value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn uniform_and_storage_buffer_8_bit_access_raw(&self) -> Bool32 {
        self.uniform_and_storage_buffer_8_bit_access
    }
    ///Gets the raw value of [`Self::storage_push_constant_8`]
    pub fn storage_push_constant_8_raw(&self) -> Bool32 {
        self.storage_push_constant_8
    }
    ///Gets the raw value of [`Self::shader_buffer_int_64_atomics`]
    pub fn shader_buffer_int_64_atomics_raw(&self) -> Bool32 {
        self.shader_buffer_int_64_atomics
    }
    ///Gets the raw value of [`Self::shader_shared_int_64_atomics`]
    pub fn shader_shared_int_64_atomics_raw(&self) -> Bool32 {
        self.shader_shared_int_64_atomics
    }
    ///Gets the raw value of [`Self::shader_float_16`]
    pub fn shader_float_16_raw(&self) -> Bool32 {
        self.shader_float_16
    }
    ///Gets the raw value of [`Self::shader_int_8`]
    pub fn shader_int_8_raw(&self) -> Bool32 {
        self.shader_int_8
    }
    ///Gets the raw value of [`Self::descriptor_indexing`]
    pub fn descriptor_indexing_raw(&self) -> Bool32 {
        self.descriptor_indexing
    }
    ///Gets the raw value of [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn shader_input_attachment_array_dynamic_indexing_raw(&self) -> Bool32 {
        self.shader_input_attachment_array_dynamic_indexing
    }
    ///Gets the raw value of [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing_raw(&self) -> Bool32 {
        self.shader_uniform_texel_buffer_array_dynamic_indexing
    }
    ///Gets the raw value of [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn shader_storage_texel_buffer_array_dynamic_indexing_raw(&self) -> Bool32 {
        self.shader_storage_texel_buffer_array_dynamic_indexing
    }
    ///Gets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_uniform_buffer_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn shader_sampled_image_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_sampled_image_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_storage_buffer_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn shader_storage_image_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_storage_image_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn shader_input_attachment_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_input_attachment_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing_raw(&self) -> Bool32 {
        self.shader_storage_texel_buffer_array_non_uniform_indexing
    }
    ///Gets the raw value of [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_buffer_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_uniform_buffer_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn descriptor_binding_sampled_image_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_sampled_image_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn descriptor_binding_storage_image_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_storage_image_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_buffer_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_storage_buffer_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_storage_texel_buffer_update_after_bind
    }
    ///Gets the raw value of [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn descriptor_binding_update_unused_while_pending_raw(&self) -> Bool32 {
        self.descriptor_binding_update_unused_while_pending
    }
    ///Gets the raw value of [`Self::descriptor_binding_partially_bound`]
    pub fn descriptor_binding_partially_bound_raw(&self) -> Bool32 {
        self.descriptor_binding_partially_bound
    }
    ///Gets the raw value of [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn descriptor_binding_variable_descriptor_count_raw(&self) -> Bool32 {
        self.descriptor_binding_variable_descriptor_count
    }
    ///Gets the raw value of [`Self::runtime_descriptor_array`]
    pub fn runtime_descriptor_array_raw(&self) -> Bool32 {
        self.runtime_descriptor_array
    }
    ///Gets the raw value of [`Self::sampler_filter_minmax`]
    pub fn sampler_filter_minmax_raw(&self) -> Bool32 {
        self.sampler_filter_minmax
    }
    ///Gets the raw value of [`Self::scalar_block_layout`]
    pub fn scalar_block_layout_raw(&self) -> Bool32 {
        self.scalar_block_layout
    }
    ///Gets the raw value of [`Self::imageless_framebuffer`]
    pub fn imageless_framebuffer_raw(&self) -> Bool32 {
        self.imageless_framebuffer
    }
    ///Gets the raw value of [`Self::uniform_buffer_standard_layout`]
    pub fn uniform_buffer_standard_layout_raw(&self) -> Bool32 {
        self.uniform_buffer_standard_layout
    }
    ///Gets the raw value of [`Self::shader_subgroup_extended_types`]
    pub fn shader_subgroup_extended_types_raw(&self) -> Bool32 {
        self.shader_subgroup_extended_types
    }
    ///Gets the raw value of [`Self::separate_depth_stencil_layouts`]
    pub fn separate_depth_stencil_layouts_raw(&self) -> Bool32 {
        self.separate_depth_stencil_layouts
    }
    ///Gets the raw value of [`Self::host_query_reset`]
    pub fn host_query_reset_raw(&self) -> Bool32 {
        self.host_query_reset
    }
    ///Gets the raw value of [`Self::timeline_semaphore`]
    pub fn timeline_semaphore_raw(&self) -> Bool32 {
        self.timeline_semaphore
    }
    ///Gets the raw value of [`Self::buffer_device_address`]
    pub fn buffer_device_address_raw(&self) -> Bool32 {
        self.buffer_device_address
    }
    ///Gets the raw value of [`Self::buffer_device_address_capture_replay`]
    pub fn buffer_device_address_capture_replay_raw(&self) -> Bool32 {
        self.buffer_device_address_capture_replay
    }
    ///Gets the raw value of [`Self::buffer_device_address_multi_device`]
    pub fn buffer_device_address_multi_device_raw(&self) -> Bool32 {
        self.buffer_device_address_multi_device
    }
    ///Gets the raw value of [`Self::vulkan_memory_model`]
    pub fn vulkan_memory_model_raw(&self) -> Bool32 {
        self.vulkan_memory_model
    }
    ///Gets the raw value of [`Self::vulkan_memory_model_device_scope`]
    pub fn vulkan_memory_model_device_scope_raw(&self) -> Bool32 {
        self.vulkan_memory_model_device_scope
    }
    ///Gets the raw value of [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn vulkan_memory_model_availability_visibility_chains_raw(&self) -> Bool32 {
        self.vulkan_memory_model_availability_visibility_chains
    }
    ///Gets the raw value of [`Self::shader_output_viewport_index`]
    pub fn shader_output_viewport_index_raw(&self) -> Bool32 {
        self.shader_output_viewport_index
    }
    ///Gets the raw value of [`Self::shader_output_layer`]
    pub fn shader_output_layer_raw(&self) -> Bool32 {
        self.shader_output_layer
    }
    ///Gets the raw value of [`Self::subgroup_broadcast_dynamic_id`]
    pub fn subgroup_broadcast_dynamic_id_raw(&self) -> Bool32 {
        self.subgroup_broadcast_dynamic_id
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::sampler_mirror_clamp_to_edge`]
    pub fn set_sampler_mirror_clamp_to_edge_raw(&mut self, value: Bool32) -> &mut Self {
        self.sampler_mirror_clamp_to_edge = value;
        self
    }
    ///Sets the raw value of [`Self::draw_indirect_count`]
    pub fn set_draw_indirect_count_raw(&mut self, value: Bool32) -> &mut Self {
        self.draw_indirect_count = value;
        self
    }
    ///Sets the raw value of [`Self::storage_buffer_8_bit_access`]
    pub fn set_storage_buffer_8_bit_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.storage_buffer_8_bit_access = value;
        self
    }
    ///Sets the raw value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn set_uniform_and_storage_buffer_8_bit_access_raw(&mut self, value: Bool32) -> &mut Self {
        self.uniform_and_storage_buffer_8_bit_access = value;
        self
    }
    ///Sets the raw value of [`Self::storage_push_constant_8`]
    pub fn set_storage_push_constant_8_raw(&mut self, value: Bool32) -> &mut Self {
        self.storage_push_constant_8 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_int_64_atomics`]
    pub fn set_shader_buffer_int_64_atomics_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_buffer_int_64_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_int_64_atomics`]
    pub fn set_shader_shared_int_64_atomics_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_shared_int_64_atomics = value;
        self
    }
    ///Sets the raw value of [`Self::shader_float_16`]
    pub fn set_shader_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_int_8`]
    pub fn set_shader_int_8_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_int_8 = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_indexing`]
    pub fn set_descriptor_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn set_shader_input_attachment_array_dynamic_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_input_attachment_array_dynamic_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn set_shader_uniform_texel_buffer_array_dynamic_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_uniform_texel_buffer_array_dynamic_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn set_shader_storage_texel_buffer_array_dynamic_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_texel_buffer_array_dynamic_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn set_shader_sampled_image_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn set_shader_storage_buffer_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn set_shader_storage_image_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn set_shader_input_attachment_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn set_shader_uniform_texel_buffer_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn set_shader_storage_texel_buffer_array_non_uniform_indexing_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_texel_buffer_array_non_uniform_indexing = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn set_descriptor_binding_uniform_buffer_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_uniform_buffer_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn set_descriptor_binding_sampled_image_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_sampled_image_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn set_descriptor_binding_storage_image_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_storage_image_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn set_descriptor_binding_storage_buffer_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_storage_buffer_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn set_descriptor_binding_uniform_texel_buffer_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn set_descriptor_binding_storage_texel_buffer_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_storage_texel_buffer_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn set_descriptor_binding_update_unused_while_pending_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_update_unused_while_pending = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_partially_bound`]
    pub fn set_descriptor_binding_partially_bound_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_partially_bound = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn set_descriptor_binding_variable_descriptor_count_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_variable_descriptor_count = value;
        self
    }
    ///Sets the raw value of [`Self::runtime_descriptor_array`]
    pub fn set_runtime_descriptor_array_raw(&mut self, value: Bool32) -> &mut Self {
        self.runtime_descriptor_array = value;
        self
    }
    ///Sets the raw value of [`Self::sampler_filter_minmax`]
    pub fn set_sampler_filter_minmax_raw(&mut self, value: Bool32) -> &mut Self {
        self.sampler_filter_minmax = value;
        self
    }
    ///Sets the raw value of [`Self::scalar_block_layout`]
    pub fn set_scalar_block_layout_raw(&mut self, value: Bool32) -> &mut Self {
        self.scalar_block_layout = value;
        self
    }
    ///Sets the raw value of [`Self::imageless_framebuffer`]
    pub fn set_imageless_framebuffer_raw(&mut self, value: Bool32) -> &mut Self {
        self.imageless_framebuffer = value;
        self
    }
    ///Sets the raw value of [`Self::uniform_buffer_standard_layout`]
    pub fn set_uniform_buffer_standard_layout_raw(&mut self, value: Bool32) -> &mut Self {
        self.uniform_buffer_standard_layout = value;
        self
    }
    ///Sets the raw value of [`Self::shader_subgroup_extended_types`]
    pub fn set_shader_subgroup_extended_types_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_subgroup_extended_types = value;
        self
    }
    ///Sets the raw value of [`Self::separate_depth_stencil_layouts`]
    pub fn set_separate_depth_stencil_layouts_raw(&mut self, value: Bool32) -> &mut Self {
        self.separate_depth_stencil_layouts = value;
        self
    }
    ///Sets the raw value of [`Self::host_query_reset`]
    pub fn set_host_query_reset_raw(&mut self, value: Bool32) -> &mut Self {
        self.host_query_reset = value;
        self
    }
    ///Sets the raw value of [`Self::timeline_semaphore`]
    pub fn set_timeline_semaphore_raw(&mut self, value: Bool32) -> &mut Self {
        self.timeline_semaphore = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address`]
    pub fn set_buffer_device_address_raw(&mut self, value: Bool32) -> &mut Self {
        self.buffer_device_address = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_capture_replay`]
    pub fn set_buffer_device_address_capture_replay_raw(&mut self, value: Bool32) -> &mut Self {
        self.buffer_device_address_capture_replay = value;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_multi_device`]
    pub fn set_buffer_device_address_multi_device_raw(&mut self, value: Bool32) -> &mut Self {
        self.buffer_device_address_multi_device = value;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model`]
    pub fn set_vulkan_memory_model_raw(&mut self, value: Bool32) -> &mut Self {
        self.vulkan_memory_model = value;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model_device_scope`]
    pub fn set_vulkan_memory_model_device_scope_raw(&mut self, value: Bool32) -> &mut Self {
        self.vulkan_memory_model_device_scope = value;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn set_vulkan_memory_model_availability_visibility_chains_raw(&mut self, value: Bool32) -> &mut Self {
        self.vulkan_memory_model_availability_visibility_chains = value;
        self
    }
    ///Sets the raw value of [`Self::shader_output_viewport_index`]
    pub fn set_shader_output_viewport_index_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_output_viewport_index = value;
        self
    }
    ///Sets the raw value of [`Self::shader_output_layer`]
    pub fn set_shader_output_layer_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_output_layer = value;
        self
    }
    ///Sets the raw value of [`Self::subgroup_broadcast_dynamic_id`]
    pub fn set_subgroup_broadcast_dynamic_id_raw(&mut self, value: Bool32) -> &mut Self {
        self.subgroup_broadcast_dynamic_id = value;
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
    ///Gets the value of [`Self::sampler_mirror_clamp_to_edge`]
    pub fn sampler_mirror_clamp_to_edge(&self) -> bool {
        unsafe { std::mem::transmute(self.sampler_mirror_clamp_to_edge as u8) }
    }
    ///Gets the value of [`Self::draw_indirect_count`]
    pub fn draw_indirect_count(&self) -> bool {
        unsafe { std::mem::transmute(self.draw_indirect_count as u8) }
    }
    ///Gets the value of [`Self::storage_buffer_8_bit_access`]
    pub fn storage_buffer_8_bit_access(&self) -> bool {
        unsafe { std::mem::transmute(self.storage_buffer_8_bit_access as u8) }
    }
    ///Gets the value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn uniform_and_storage_buffer_8_bit_access(&self) -> bool {
        unsafe { std::mem::transmute(self.uniform_and_storage_buffer_8_bit_access as u8) }
    }
    ///Gets the value of [`Self::storage_push_constant_8`]
    pub fn storage_push_constant_8(&self) -> bool {
        unsafe { std::mem::transmute(self.storage_push_constant_8 as u8) }
    }
    ///Gets the value of [`Self::shader_buffer_int_64_atomics`]
    pub fn shader_buffer_int_64_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_buffer_int_64_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_shared_int_64_atomics`]
    pub fn shader_shared_int_64_atomics(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_shared_int_64_atomics as u8) }
    }
    ///Gets the value of [`Self::shader_float_16`]
    pub fn shader_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_int_8`]
    pub fn shader_int_8(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_int_8 as u8) }
    }
    ///Gets the value of [`Self::descriptor_indexing`]
    pub fn descriptor_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn shader_input_attachment_array_dynamic_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_input_attachment_array_dynamic_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_uniform_texel_buffer_array_dynamic_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_texel_buffer_array_dynamic_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_uniform_buffer_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn shader_sampled_image_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_sampled_image_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_buffer_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_buffer_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn shader_storage_image_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_image_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn shader_input_attachment_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_input_attachment_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_uniform_texel_buffer_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_texel_buffer_array_non_uniform_indexing as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_buffer_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_uniform_buffer_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn descriptor_binding_sampled_image_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_sampled_image_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn descriptor_binding_storage_image_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_storage_image_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_buffer_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_storage_buffer_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_uniform_texel_buffer_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_storage_texel_buffer_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn descriptor_binding_update_unused_while_pending(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_update_unused_while_pending as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_partially_bound`]
    pub fn descriptor_binding_partially_bound(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_partially_bound as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn descriptor_binding_variable_descriptor_count(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_variable_descriptor_count as u8) }
    }
    ///Gets the value of [`Self::runtime_descriptor_array`]
    pub fn runtime_descriptor_array(&self) -> bool {
        unsafe { std::mem::transmute(self.runtime_descriptor_array as u8) }
    }
    ///Gets the value of [`Self::sampler_filter_minmax`]
    pub fn sampler_filter_minmax(&self) -> bool {
        unsafe { std::mem::transmute(self.sampler_filter_minmax as u8) }
    }
    ///Gets the value of [`Self::scalar_block_layout`]
    pub fn scalar_block_layout(&self) -> bool {
        unsafe { std::mem::transmute(self.scalar_block_layout as u8) }
    }
    ///Gets the value of [`Self::imageless_framebuffer`]
    pub fn imageless_framebuffer(&self) -> bool {
        unsafe { std::mem::transmute(self.imageless_framebuffer as u8) }
    }
    ///Gets the value of [`Self::uniform_buffer_standard_layout`]
    pub fn uniform_buffer_standard_layout(&self) -> bool {
        unsafe { std::mem::transmute(self.uniform_buffer_standard_layout as u8) }
    }
    ///Gets the value of [`Self::shader_subgroup_extended_types`]
    pub fn shader_subgroup_extended_types(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_subgroup_extended_types as u8) }
    }
    ///Gets the value of [`Self::separate_depth_stencil_layouts`]
    pub fn separate_depth_stencil_layouts(&self) -> bool {
        unsafe { std::mem::transmute(self.separate_depth_stencil_layouts as u8) }
    }
    ///Gets the value of [`Self::host_query_reset`]
    pub fn host_query_reset(&self) -> bool {
        unsafe { std::mem::transmute(self.host_query_reset as u8) }
    }
    ///Gets the value of [`Self::timeline_semaphore`]
    pub fn timeline_semaphore(&self) -> bool {
        unsafe { std::mem::transmute(self.timeline_semaphore as u8) }
    }
    ///Gets the value of [`Self::buffer_device_address`]
    pub fn buffer_device_address(&self) -> bool {
        unsafe { std::mem::transmute(self.buffer_device_address as u8) }
    }
    ///Gets the value of [`Self::buffer_device_address_capture_replay`]
    pub fn buffer_device_address_capture_replay(&self) -> bool {
        unsafe { std::mem::transmute(self.buffer_device_address_capture_replay as u8) }
    }
    ///Gets the value of [`Self::buffer_device_address_multi_device`]
    pub fn buffer_device_address_multi_device(&self) -> bool {
        unsafe { std::mem::transmute(self.buffer_device_address_multi_device as u8) }
    }
    ///Gets the value of [`Self::vulkan_memory_model`]
    pub fn vulkan_memory_model(&self) -> bool {
        unsafe { std::mem::transmute(self.vulkan_memory_model as u8) }
    }
    ///Gets the value of [`Self::vulkan_memory_model_device_scope`]
    pub fn vulkan_memory_model_device_scope(&self) -> bool {
        unsafe { std::mem::transmute(self.vulkan_memory_model_device_scope as u8) }
    }
    ///Gets the value of [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn vulkan_memory_model_availability_visibility_chains(&self) -> bool {
        unsafe { std::mem::transmute(self.vulkan_memory_model_availability_visibility_chains as u8) }
    }
    ///Gets the value of [`Self::shader_output_viewport_index`]
    pub fn shader_output_viewport_index(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_output_viewport_index as u8) }
    }
    ///Gets the value of [`Self::shader_output_layer`]
    pub fn shader_output_layer(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_output_layer as u8) }
    }
    ///Gets the value of [`Self::subgroup_broadcast_dynamic_id`]
    pub fn subgroup_broadcast_dynamic_id(&self) -> bool {
        unsafe { std::mem::transmute(self.subgroup_broadcast_dynamic_id as u8) }
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
    ///Gets a mutable reference to the value of [`Self::sampler_mirror_clamp_to_edge`]
    pub fn sampler_mirror_clamp_to_edge_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.sampler_mirror_clamp_to_edge as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.sampler_mirror_clamp_to_edge as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::draw_indirect_count`]
    pub fn draw_indirect_count_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.draw_indirect_count as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.draw_indirect_count as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::storage_buffer_8_bit_access`]
    pub fn storage_buffer_8_bit_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.storage_buffer_8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.storage_buffer_8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn uniform_and_storage_buffer_8_bit_access_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.uniform_and_storage_buffer_8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.uniform_and_storage_buffer_8_bit_access as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::storage_push_constant_8`]
    pub fn storage_push_constant_8_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.storage_push_constant_8 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.storage_push_constant_8 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_buffer_int_64_atomics`]
    pub fn shader_buffer_int_64_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_buffer_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_buffer_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_shared_int_64_atomics`]
    pub fn shader_shared_int_64_atomics_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_shared_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_shared_int_64_atomics as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_float_16`]
    pub fn shader_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_int_8`]
    pub fn shader_int_8_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_int_8 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_int_8 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_indexing`]
    pub fn descriptor_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn shader_input_attachment_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_input_attachment_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_input_attachment_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_uniform_texel_buffer_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_uniform_texel_buffer_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn shader_storage_texel_buffer_array_dynamic_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_texel_buffer_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_texel_buffer_array_dynamic_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_uniform_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_uniform_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn shader_sampled_image_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_sampled_image_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_sampled_image_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn shader_storage_image_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_image_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_image_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn shader_input_attachment_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_input_attachment_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_input_attachment_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_uniform_texel_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_uniform_texel_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_texel_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_texel_buffer_array_non_uniform_indexing as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_uniform_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_uniform_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn descriptor_binding_sampled_image_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_sampled_image_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_sampled_image_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn descriptor_binding_storage_image_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_storage_image_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_storage_image_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_storage_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_storage_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_uniform_texel_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_uniform_texel_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_storage_texel_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_storage_texel_buffer_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn descriptor_binding_update_unused_while_pending_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_update_unused_while_pending as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_update_unused_while_pending as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_binding_partially_bound`]
    pub fn descriptor_binding_partially_bound_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_partially_bound as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_partially_bound as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn descriptor_binding_variable_descriptor_count_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_variable_descriptor_count as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_variable_descriptor_count as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::runtime_descriptor_array`]
    pub fn runtime_descriptor_array_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.runtime_descriptor_array as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.runtime_descriptor_array as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::sampler_filter_minmax`]
    pub fn sampler_filter_minmax_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.sampler_filter_minmax as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.sampler_filter_minmax as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::scalar_block_layout`]
    pub fn scalar_block_layout_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.scalar_block_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.scalar_block_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::imageless_framebuffer`]
    pub fn imageless_framebuffer_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.imageless_framebuffer as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.imageless_framebuffer as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::uniform_buffer_standard_layout`]
    pub fn uniform_buffer_standard_layout_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.uniform_buffer_standard_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.uniform_buffer_standard_layout as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_subgroup_extended_types`]
    pub fn shader_subgroup_extended_types_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_subgroup_extended_types as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_subgroup_extended_types as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::separate_depth_stencil_layouts`]
    pub fn separate_depth_stencil_layouts_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.separate_depth_stencil_layouts as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.separate_depth_stencil_layouts as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::host_query_reset`]
    pub fn host_query_reset_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.host_query_reset as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.host_query_reset as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::timeline_semaphore`]
    pub fn timeline_semaphore_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.timeline_semaphore as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.timeline_semaphore as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::buffer_device_address`]
    pub fn buffer_device_address_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.buffer_device_address as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.buffer_device_address as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::buffer_device_address_capture_replay`]
    pub fn buffer_device_address_capture_replay_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.buffer_device_address_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.buffer_device_address_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::buffer_device_address_multi_device`]
    pub fn buffer_device_address_multi_device_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.buffer_device_address_multi_device as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.buffer_device_address_multi_device as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::vulkan_memory_model`]
    pub fn vulkan_memory_model_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vulkan_memory_model as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vulkan_memory_model as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::vulkan_memory_model_device_scope`]
    pub fn vulkan_memory_model_device_scope_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vulkan_memory_model_device_scope as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vulkan_memory_model_device_scope as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn vulkan_memory_model_availability_visibility_chains_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.vulkan_memory_model_availability_visibility_chains as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.vulkan_memory_model_availability_visibility_chains as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_output_viewport_index`]
    pub fn shader_output_viewport_index_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_output_viewport_index as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_output_viewport_index as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_output_layer`]
    pub fn shader_output_layer_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_output_layer as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_output_layer as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::subgroup_broadcast_dynamic_id`]
    pub fn subgroup_broadcast_dynamic_id_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.subgroup_broadcast_dynamic_id as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.subgroup_broadcast_dynamic_id as *mut Bool32)
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
    ///Sets the raw value of [`Self::sampler_mirror_clamp_to_edge`]
    pub fn set_sampler_mirror_clamp_to_edge(&mut self, value: bool) -> &mut Self {
        self.sampler_mirror_clamp_to_edge = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::draw_indirect_count`]
    pub fn set_draw_indirect_count(&mut self, value: bool) -> &mut Self {
        self.draw_indirect_count = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::storage_buffer_8_bit_access`]
    pub fn set_storage_buffer_8_bit_access(&mut self, value: bool) -> &mut Self {
        self.storage_buffer_8_bit_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::uniform_and_storage_buffer_8_bit_access`]
    pub fn set_uniform_and_storage_buffer_8_bit_access(&mut self, value: bool) -> &mut Self {
        self.uniform_and_storage_buffer_8_bit_access = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::storage_push_constant_8`]
    pub fn set_storage_push_constant_8(&mut self, value: bool) -> &mut Self {
        self.storage_push_constant_8 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_buffer_int_64_atomics`]
    pub fn set_shader_buffer_int_64_atomics(&mut self, value: bool) -> &mut Self {
        self.shader_buffer_int_64_atomics = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_shared_int_64_atomics`]
    pub fn set_shader_shared_int_64_atomics(&mut self, value: bool) -> &mut Self {
        self.shader_shared_int_64_atomics = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_float_16`]
    pub fn set_shader_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_int_8`]
    pub fn set_shader_int_8(&mut self, value: bool) -> &mut Self {
        self.shader_int_8 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_indexing`]
    pub fn set_descriptor_indexing(&mut self, value: bool) -> &mut Self {
        self.descriptor_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_dynamic_indexing`]
    pub fn set_shader_input_attachment_array_dynamic_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_input_attachment_array_dynamic_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_texel_buffer_array_dynamic_indexing`]
    pub fn set_shader_uniform_texel_buffer_array_dynamic_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_uniform_texel_buffer_array_dynamic_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_texel_buffer_array_dynamic_indexing`]
    pub fn set_shader_storage_texel_buffer_array_dynamic_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_storage_texel_buffer_array_dynamic_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing`]
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing`]
    pub fn set_shader_sampled_image_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing`]
    pub fn set_shader_storage_buffer_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing`]
    pub fn set_shader_storage_image_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing`]
    pub fn set_shader_input_attachment_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_texel_buffer_array_non_uniform_indexing`]
    pub fn set_shader_uniform_texel_buffer_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_uniform_texel_buffer_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_texel_buffer_array_non_uniform_indexing`]
    pub fn set_shader_storage_texel_buffer_array_non_uniform_indexing(&mut self, value: bool) -> &mut Self {
        self.shader_storage_texel_buffer_array_non_uniform_indexing = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_uniform_buffer_update_after_bind`]
    pub fn set_descriptor_binding_uniform_buffer_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_uniform_buffer_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_sampled_image_update_after_bind`]
    pub fn set_descriptor_binding_sampled_image_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_sampled_image_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_image_update_after_bind`]
    pub fn set_descriptor_binding_storage_image_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_storage_image_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_buffer_update_after_bind`]
    pub fn set_descriptor_binding_storage_buffer_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_storage_buffer_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_uniform_texel_buffer_update_after_bind`]
    pub fn set_descriptor_binding_uniform_texel_buffer_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_uniform_texel_buffer_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_storage_texel_buffer_update_after_bind`]
    pub fn set_descriptor_binding_storage_texel_buffer_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_storage_texel_buffer_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_update_unused_while_pending`]
    pub fn set_descriptor_binding_update_unused_while_pending(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_update_unused_while_pending = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_partially_bound`]
    pub fn set_descriptor_binding_partially_bound(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_partially_bound = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_variable_descriptor_count`]
    pub fn set_descriptor_binding_variable_descriptor_count(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_variable_descriptor_count = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::runtime_descriptor_array`]
    pub fn set_runtime_descriptor_array(&mut self, value: bool) -> &mut Self {
        self.runtime_descriptor_array = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::sampler_filter_minmax`]
    pub fn set_sampler_filter_minmax(&mut self, value: bool) -> &mut Self {
        self.sampler_filter_minmax = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::scalar_block_layout`]
    pub fn set_scalar_block_layout(&mut self, value: bool) -> &mut Self {
        self.scalar_block_layout = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::imageless_framebuffer`]
    pub fn set_imageless_framebuffer(&mut self, value: bool) -> &mut Self {
        self.imageless_framebuffer = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::uniform_buffer_standard_layout`]
    pub fn set_uniform_buffer_standard_layout(&mut self, value: bool) -> &mut Self {
        self.uniform_buffer_standard_layout = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_subgroup_extended_types`]
    pub fn set_shader_subgroup_extended_types(&mut self, value: bool) -> &mut Self {
        self.shader_subgroup_extended_types = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::separate_depth_stencil_layouts`]
    pub fn set_separate_depth_stencil_layouts(&mut self, value: bool) -> &mut Self {
        self.separate_depth_stencil_layouts = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::host_query_reset`]
    pub fn set_host_query_reset(&mut self, value: bool) -> &mut Self {
        self.host_query_reset = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::timeline_semaphore`]
    pub fn set_timeline_semaphore(&mut self, value: bool) -> &mut Self {
        self.timeline_semaphore = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address`]
    pub fn set_buffer_device_address(&mut self, value: bool) -> &mut Self {
        self.buffer_device_address = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_capture_replay`]
    pub fn set_buffer_device_address_capture_replay(&mut self, value: bool) -> &mut Self {
        self.buffer_device_address_capture_replay = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::buffer_device_address_multi_device`]
    pub fn set_buffer_device_address_multi_device(&mut self, value: bool) -> &mut Self {
        self.buffer_device_address_multi_device = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model`]
    pub fn set_vulkan_memory_model(&mut self, value: bool) -> &mut Self {
        self.vulkan_memory_model = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model_device_scope`]
    pub fn set_vulkan_memory_model_device_scope(&mut self, value: bool) -> &mut Self {
        self.vulkan_memory_model_device_scope = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::vulkan_memory_model_availability_visibility_chains`]
    pub fn set_vulkan_memory_model_availability_visibility_chains(&mut self, value: bool) -> &mut Self {
        self.vulkan_memory_model_availability_visibility_chains = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_output_viewport_index`]
    pub fn set_shader_output_viewport_index(&mut self, value: bool) -> &mut Self {
        self.shader_output_viewport_index = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_output_layer`]
    pub fn set_shader_output_layer(&mut self, value: bool) -> &mut Self {
        self.shader_output_layer = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::subgroup_broadcast_dynamic_id`]
    pub fn set_subgroup_broadcast_dynamic_id(&mut self, value: bool) -> &mut Self {
        self.subgroup_broadcast_dynamic_id = value as u8 as u32;
        self
    }
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
///   class="base"><span class="strut"
///   style="height:0.66666em;vertical-align:-0.08333em;"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span> **can**  be preserved in 16-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**
///   be used for 16-bit floating-point types.
/// - [`shader_signed_zero_inf_nan_preserve_float_32`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span aria-hidden="true" class="katex-html"><span
///   class="base"><span style="height:0.66666em;vertical-align:-0.08333em;"
///   class="strut"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span> **can**  be preserved in 32-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**
///   be used for 32-bit floating-point types.
/// - [`shader_signed_zero_inf_nan_preserve_float_64`] is a boolean value indicating whether sign of
///   a zero, Nans and <span class="katex"><span aria-hidden="true" class="katex-html"><span
///   class="base"><span style="height:0.66666em;vertical-align:-0.08333em;"
///   class="strut"></span><span class="mord">±</span><span
///   class="mord">∞</span></span></span></span> **can**  be preserved in 64-bit floating-point
///   computations. It also indicates whether the `SignedZeroInfNanPreserve` execution mode  **can**
///   be used for 64-bit floating-point types.
/// - [`shader_denorm_preserve_float_16`] is a boolean value indicating whether denormals  **can**
///   be preserved in 16-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode  **can**  be used for 16-bit floating-point types.
/// - [`shader_denorm_preserve_float_32`] is a boolean value indicating whether denormals  **can**
///   be preserved in 32-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode  **can**  be used for 32-bit floating-point types.
/// - [`shader_denorm_preserve_float_64`] is a boolean value indicating whether denormals  **can**
///   be preserved in 64-bit floating-point computations. It also indicates whether the
///   `DenormPreserve` execution mode  **can**  be used for 64-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_16`] is a boolean value indicating whether denormals
///   **can**  be flushed to zero in 16-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode  **can**  be used for 16-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_32`] is a boolean value indicating whether denormals
///   **can**  be flushed to zero in 32-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode  **can**  be used for 32-bit floating-point types.
/// - [`shader_denorm_flush_to_zero_float_64`] is a boolean value indicating whether denormals
///   **can**  be flushed to zero in 64-bit floating-point computations. It also indicates whether
///   the `DenormFlushToZero` execution mode  **can**  be used for 64-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_16`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 16-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can**  be used for 16-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_32`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 32-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can**  be used for 32-bit floating-point types.
/// - [`shader_rounding_mode_rte_float_64`] is a boolean value indicating whether an implementation
///   supports the round-to-nearest-even rounding mode for 64-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTE` execution mode
///   **can**  be used for 64-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_16`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 16-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can**  be used for 16-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_32`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 32-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can**  be used for 32-bit floating-point types.
/// - [`shader_rounding_mode_rtz_float_64`] is a boolean value indicating whether an implementation
///   supports the round-towards-zero rounding mode for 64-bit floating-point arithmetic and
///   conversion instructions. It also indicates whether the `RoundingModeRTZ` execution mode
///   **can**  be used for 64-bit floating-point types.
/// - [`max_update_after_bind_descriptors_in_all_pools`] is the maximum number of descriptors
///   (summed over all descriptor types) that  **can**  be created across all pools that are created
///   with the `VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` bit set. Pool creation  **may**
///   fail when this limit is exceeded, or when the space this limit represents is unable to satisfy
///   a pool creation due to fragmentation.
/// - [`shader_uniform_buffer_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether uniform buffer descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of uniform
///   buffers  **may**  execute multiple times in order to access all the descriptors.
/// - [`shader_sampled_image_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether sampler and image descriptors natively support nonuniform indexing. If this is
///   [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array
///   of samplers or images  **may**  execute multiple times in order to access all the descriptors.
/// - [`shader_storage_buffer_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether storage buffer descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of storage
///   buffers  **may**  execute multiple times in order to access all the descriptors.
/// - [`shader_storage_image_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether storage image descriptors natively support nonuniform indexing. If this is [`FALSE`],
///   then a single dynamic instance of an instruction that nonuniformly indexes an array of storage
///   images  **may**  execute multiple times in order to access all the descriptors.
/// - [`shader_input_attachment_array_non_uniform_indexing_native`] is a boolean value indicating
///   whether input attachment descriptors natively support nonuniform indexing. If this is
///   [`FALSE`], then a single dynamic instance of an instruction that nonuniformly indexes an array
///   of input attachments  **may**  execute multiple times in order to access all the descriptors.
/// - [`robust_buffer_access_update_after_bind`] is a boolean value indicating whether
///   [`robustBufferAccess`]() **can**  be enabled in a device simultaneously with
///   `descriptorBindingUniformBufferUpdateAfterBind`,
///   `descriptorBindingStorageBufferUpdateAfterBind`,
///   `descriptorBindingUniformTexelBufferUpdateAfterBind`, and/or
///   `descriptorBindingStorageTexelBufferUpdateAfterBind`. If this is [`FALSE`], then either
///   `robustBufferAccess` **must**  be disabled or all of these update-after-bind features
///   **must**  be disabled.
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
///   While an application  **can**  allocate dynamic uniform buffer descriptors from a pool created
///   with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these
///   descriptors  **must**  not be present in any descriptor set layout that includes bindings
///   created with `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT`.
/// - [`max_descriptor_set_update_after_bind_storage_buffers`] is similar to
///   `maxDescriptorSetStorageBuffers` but counts descriptors from descriptor sets created with or
///   without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_update_after_bind_storage_buffers_dynamic`] is similar to
///   `maxDescriptorSetStorageBuffersDynamic` but counts descriptors from descriptor sets created
///   with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
///   While an application  **can**  allocate dynamic storage buffer descriptors from a pool created
///   with the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`, bindings for these
///   descriptors  **must**  not be present in any descriptor set layout that includes bindings
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
///   of supported depth resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT` **must**  be included in
///   the set but implementations  **may**  support additional modes.
/// - [`supported_stencil_resolve_modes`] is a bitmask of [`ResolveModeFlagBits`] indicating the set
///   of supported stencil resolve modes. `VK_RESOLVE_MODE_SAMPLE_ZERO_BIT` **must**  be included in
///   the set but implementations  **may**  support additional modes. `VK_RESOLVE_MODE_AVERAGE_BIT`
///   **must**  not be included in the set.
/// - [`independent_resolve_none`] is [`TRUE`] if the implementation supports setting the depth and
///   stencil resolve modes to different values when one of those modes is `VK_RESOLVE_MODE_NONE`.
///   Otherwise the implementation only supports setting both modes to the same value.
/// - [`independent_resolve`] is [`TRUE`] if the implementation supports all combinations of the
///   supported depth and stencil resolve modes, including setting either depth or stencil resolve
///   mode to `VK_RESOLVE_MODE_NONE`. An implementation that supports [`independent_resolve`]
///   **must**  also support [`independent_resolve_none`].
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
/// functionality.The members of [`PhysicalDeviceVulkan12Properties`] **must**  have the same
///values as the corresponding members of
///[`PhysicalDeviceDriverProperties`],
///[`PhysicalDeviceFloatControlsProperties`],
///[`PhysicalDeviceDescriptorIndexingProperties`],
///[`PhysicalDeviceDepthStencilResolveProperties`],
///[`PhysicalDeviceSamplerFilterMinmaxProperties`], and
///[`PhysicalDeviceTimelineSemaphoreProperties`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES`
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
#[doc(alias = "VkPhysicalDeviceVulkan12Properties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Properties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub driver_id: DriverId,
    ///No documentation found
    pub driver_name: [c_char; MAX_DRIVER_NAME_SIZE as usize],
    ///No documentation found
    pub driver_info: [c_char; MAX_DRIVER_INFO_SIZE as usize],
    ///No documentation found
    pub conformance_version: ConformanceVersion,
    ///No documentation found
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    ///No documentation found
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    ///No documentation found
    pub shader_signed_zero_inf_nan_preserve_float_16: Bool32,
    ///No documentation found
    pub shader_signed_zero_inf_nan_preserve_float_32: Bool32,
    ///No documentation found
    pub shader_signed_zero_inf_nan_preserve_float_64: Bool32,
    ///No documentation found
    pub shader_denorm_preserve_float_16: Bool32,
    ///No documentation found
    pub shader_denorm_preserve_float_32: Bool32,
    ///No documentation found
    pub shader_denorm_preserve_float_64: Bool32,
    ///No documentation found
    pub shader_denorm_flush_to_zero_float_16: Bool32,
    ///No documentation found
    pub shader_denorm_flush_to_zero_float_32: Bool32,
    ///No documentation found
    pub shader_denorm_flush_to_zero_float_64: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rte_float_16: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rte_float_32: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rte_float_64: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rtz_float_16: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rtz_float_32: Bool32,
    ///No documentation found
    pub shader_rounding_mode_rtz_float_64: Bool32,
    ///No documentation found
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    ///No documentation found
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    ///No documentation found
    pub robust_buffer_access_update_after_bind: Bool32,
    ///No documentation found
    pub quad_divergent_implicit_lod: Bool32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    ///No documentation found
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    ///No documentation found
    pub max_per_stage_update_after_bind_resources: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_samplers: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    ///No documentation found
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
    ///No documentation found
    pub supported_depth_resolve_modes: ResolveModeFlags,
    ///No documentation found
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    ///No documentation found
    pub independent_resolve_none: Bool32,
    ///No documentation found
    pub independent_resolve: Bool32,
    ///No documentation found
    pub filter_minmax_single_component_formats: Bool32,
    ///No documentation found
    pub filter_minmax_image_component_mapping: Bool32,
    ///No documentation found
    pub max_timeline_semaphore_value_difference: u64,
    ///No documentation found
    pub framebuffer_integer_color_sample_counts: SampleCountFlags,
}
impl<'lt> Default for PhysicalDeviceVulkan12Properties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceVulkan12Properties,
            p_next: std::ptr::null_mut(),
            driver_id: Default::default(),
            driver_name: [b'\0' as i8; MAX_DRIVER_NAME_SIZE as usize],
            driver_info: [b'\0' as i8; MAX_DRIVER_INFO_SIZE as usize],
            conformance_version: Default::default(),
            denorm_behavior_independence: Default::default(),
            rounding_mode_independence: Default::default(),
            shader_signed_zero_inf_nan_preserve_float_16: 0,
            shader_signed_zero_inf_nan_preserve_float_32: 0,
            shader_signed_zero_inf_nan_preserve_float_64: 0,
            shader_denorm_preserve_float_16: 0,
            shader_denorm_preserve_float_32: 0,
            shader_denorm_preserve_float_64: 0,
            shader_denorm_flush_to_zero_float_16: 0,
            shader_denorm_flush_to_zero_float_32: 0,
            shader_denorm_flush_to_zero_float_64: 0,
            shader_rounding_mode_rte_float_16: 0,
            shader_rounding_mode_rte_float_32: 0,
            shader_rounding_mode_rte_float_64: 0,
            shader_rounding_mode_rtz_float_16: 0,
            shader_rounding_mode_rtz_float_32: 0,
            shader_rounding_mode_rtz_float_64: 0,
            max_update_after_bind_descriptors_in_all_pools: 0,
            shader_uniform_buffer_array_non_uniform_indexing_native: 0,
            shader_sampled_image_array_non_uniform_indexing_native: 0,
            shader_storage_buffer_array_non_uniform_indexing_native: 0,
            shader_storage_image_array_non_uniform_indexing_native: 0,
            shader_input_attachment_array_non_uniform_indexing_native: 0,
            robust_buffer_access_update_after_bind: 0,
            quad_divergent_implicit_lod: 0,
            max_per_stage_descriptor_update_after_bind_samplers: 0,
            max_per_stage_descriptor_update_after_bind_uniform_buffers: 0,
            max_per_stage_descriptor_update_after_bind_storage_buffers: 0,
            max_per_stage_descriptor_update_after_bind_sampled_images: 0,
            max_per_stage_descriptor_update_after_bind_storage_images: 0,
            max_per_stage_descriptor_update_after_bind_input_attachments: 0,
            max_per_stage_update_after_bind_resources: 0,
            max_descriptor_set_update_after_bind_samplers: 0,
            max_descriptor_set_update_after_bind_uniform_buffers: 0,
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: 0,
            max_descriptor_set_update_after_bind_storage_buffers: 0,
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: 0,
            max_descriptor_set_update_after_bind_sampled_images: 0,
            max_descriptor_set_update_after_bind_storage_images: 0,
            max_descriptor_set_update_after_bind_input_attachments: 0,
            supported_depth_resolve_modes: Default::default(),
            supported_stencil_resolve_modes: Default::default(),
            independent_resolve_none: 0,
            independent_resolve: 0,
            filter_minmax_single_component_formats: 0,
            filter_minmax_image_component_mapping: 0,
            max_timeline_semaphore_value_difference: 0,
            framebuffer_integer_color_sample_counts: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceVulkan12Properties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn shader_signed_zero_inf_nan_preserve_float_16_raw(&self) -> Bool32 {
        self.shader_signed_zero_inf_nan_preserve_float_16
    }
    ///Gets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn shader_signed_zero_inf_nan_preserve_float_32_raw(&self) -> Bool32 {
        self.shader_signed_zero_inf_nan_preserve_float_32
    }
    ///Gets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn shader_signed_zero_inf_nan_preserve_float_64_raw(&self) -> Bool32 {
        self.shader_signed_zero_inf_nan_preserve_float_64
    }
    ///Gets the raw value of [`Self::shader_denorm_preserve_float_16`]
    pub fn shader_denorm_preserve_float_16_raw(&self) -> Bool32 {
        self.shader_denorm_preserve_float_16
    }
    ///Gets the raw value of [`Self::shader_denorm_preserve_float_32`]
    pub fn shader_denorm_preserve_float_32_raw(&self) -> Bool32 {
        self.shader_denorm_preserve_float_32
    }
    ///Gets the raw value of [`Self::shader_denorm_preserve_float_64`]
    pub fn shader_denorm_preserve_float_64_raw(&self) -> Bool32 {
        self.shader_denorm_preserve_float_64
    }
    ///Gets the raw value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn shader_denorm_flush_to_zero_float_16_raw(&self) -> Bool32 {
        self.shader_denorm_flush_to_zero_float_16
    }
    ///Gets the raw value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn shader_denorm_flush_to_zero_float_32_raw(&self) -> Bool32 {
        self.shader_denorm_flush_to_zero_float_32
    }
    ///Gets the raw value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn shader_denorm_flush_to_zero_float_64_raw(&self) -> Bool32 {
        self.shader_denorm_flush_to_zero_float_64
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn shader_rounding_mode_rte_float_16_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rte_float_16
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn shader_rounding_mode_rte_float_32_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rte_float_32
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn shader_rounding_mode_rte_float_64_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rte_float_64
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn shader_rounding_mode_rtz_float_16_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rtz_float_16
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn shader_rounding_mode_rtz_float_32_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rtz_float_32
    }
    ///Gets the raw value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn shader_rounding_mode_rtz_float_64_raw(&self) -> Bool32 {
        self.shader_rounding_mode_rtz_float_64
    }
    ///Gets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_uniform_buffer_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn shader_sampled_image_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_sampled_image_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_storage_buffer_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn shader_storage_image_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_storage_image_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn shader_input_attachment_array_non_uniform_indexing_native_raw(&self) -> Bool32 {
        self.shader_input_attachment_array_non_uniform_indexing_native
    }
    ///Gets the raw value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn robust_buffer_access_update_after_bind_raw(&self) -> Bool32 {
        self.robust_buffer_access_update_after_bind
    }
    ///Gets the raw value of [`Self::quad_divergent_implicit_lod`]
    pub fn quad_divergent_implicit_lod_raw(&self) -> Bool32 {
        self.quad_divergent_implicit_lod
    }
    ///Gets the raw value of [`Self::independent_resolve_none`]
    pub fn independent_resolve_none_raw(&self) -> Bool32 {
        self.independent_resolve_none
    }
    ///Gets the raw value of [`Self::independent_resolve`]
    pub fn independent_resolve_raw(&self) -> Bool32 {
        self.independent_resolve
    }
    ///Gets the raw value of [`Self::filter_minmax_single_component_formats`]
    pub fn filter_minmax_single_component_formats_raw(&self) -> Bool32 {
        self.filter_minmax_single_component_formats
    }
    ///Gets the raw value of [`Self::filter_minmax_image_component_mapping`]
    pub fn filter_minmax_image_component_mapping_raw(&self) -> Bool32 {
        self.filter_minmax_image_component_mapping
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_64 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_16`]
    pub fn set_shader_denorm_preserve_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_preserve_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_32`]
    pub fn set_shader_denorm_preserve_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_preserve_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_64`]
    pub fn set_shader_denorm_preserve_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_preserve_float_64 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn set_shader_denorm_flush_to_zero_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn set_shader_denorm_flush_to_zero_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn set_shader_denorm_flush_to_zero_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_64 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn set_shader_rounding_mode_rte_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rte_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn set_shader_rounding_mode_rte_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rte_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn set_shader_rounding_mode_rte_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rte_float_64 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn set_shader_rounding_mode_rtz_float_16_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rtz_float_16 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn set_shader_rounding_mode_rtz_float_32_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rtz_float_32 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn set_shader_rounding_mode_rtz_float_64_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_rounding_mode_rtz_float_64 = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn set_shader_sampled_image_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn set_shader_storage_buffer_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn set_shader_storage_image_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn set_shader_input_attachment_array_non_uniform_indexing_native_raw(&mut self, value: Bool32) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing_native = value;
        self
    }
    ///Sets the raw value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn set_robust_buffer_access_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.robust_buffer_access_update_after_bind = value;
        self
    }
    ///Sets the raw value of [`Self::quad_divergent_implicit_lod`]
    pub fn set_quad_divergent_implicit_lod_raw(&mut self, value: Bool32) -> &mut Self {
        self.quad_divergent_implicit_lod = value;
        self
    }
    ///Sets the raw value of [`Self::independent_resolve_none`]
    pub fn set_independent_resolve_none_raw(&mut self, value: Bool32) -> &mut Self {
        self.independent_resolve_none = value;
        self
    }
    ///Sets the raw value of [`Self::independent_resolve`]
    pub fn set_independent_resolve_raw(&mut self, value: Bool32) -> &mut Self {
        self.independent_resolve = value;
        self
    }
    ///Sets the raw value of [`Self::filter_minmax_single_component_formats`]
    pub fn set_filter_minmax_single_component_formats_raw(&mut self, value: Bool32) -> &mut Self {
        self.filter_minmax_single_component_formats = value;
        self
    }
    ///Sets the raw value of [`Self::filter_minmax_image_component_mapping`]
    pub fn set_filter_minmax_image_component_mapping_raw(&mut self, value: Bool32) -> &mut Self {
        self.filter_minmax_image_component_mapping = value;
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
    ///Gets the value of [`Self::driver_id`]
    pub fn driver_id(&self) -> DriverId {
        self.driver_id
    }
    ///Gets the value of [`Self::driver_name`]
    pub fn driver_name(&self) -> &[c_char; MAX_DRIVER_NAME_SIZE as usize] {
        &self.driver_name
    }
    ///Gets the value of [`Self::driver_info`]
    pub fn driver_info(&self) -> &[c_char; MAX_DRIVER_INFO_SIZE as usize] {
        &self.driver_info
    }
    ///Gets the value of [`Self::conformance_version`]
    pub fn conformance_version(&self) -> ConformanceVersion {
        self.conformance_version
    }
    ///Gets the value of [`Self::denorm_behavior_independence`]
    pub fn denorm_behavior_independence(&self) -> ShaderFloatControlsIndependence {
        self.denorm_behavior_independence
    }
    ///Gets the value of [`Self::rounding_mode_independence`]
    pub fn rounding_mode_independence(&self) -> ShaderFloatControlsIndependence {
        self.rounding_mode_independence
    }
    ///Gets the value of [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn shader_signed_zero_inf_nan_preserve_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_signed_zero_inf_nan_preserve_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn shader_signed_zero_inf_nan_preserve_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_signed_zero_inf_nan_preserve_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn shader_signed_zero_inf_nan_preserve_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_signed_zero_inf_nan_preserve_float_64 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_preserve_float_16`]
    pub fn shader_denorm_preserve_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_preserve_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_preserve_float_32`]
    pub fn shader_denorm_preserve_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_preserve_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_preserve_float_64`]
    pub fn shader_denorm_preserve_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_preserve_float_64 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn shader_denorm_flush_to_zero_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_flush_to_zero_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn shader_denorm_flush_to_zero_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_flush_to_zero_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn shader_denorm_flush_to_zero_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_denorm_flush_to_zero_float_64 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn shader_rounding_mode_rte_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rte_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn shader_rounding_mode_rte_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rte_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn shader_rounding_mode_rte_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rte_float_64 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn shader_rounding_mode_rtz_float_16(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rtz_float_16 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn shader_rounding_mode_rtz_float_32(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rtz_float_32 as u8) }
    }
    ///Gets the value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn shader_rounding_mode_rtz_float_64(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_rounding_mode_rtz_float_64 as u8) }
    }
    ///Gets the value of [`Self::max_update_after_bind_descriptors_in_all_pools`]
    pub fn max_update_after_bind_descriptors_in_all_pools(&self) -> u32 {
        self.max_update_after_bind_descriptors_in_all_pools
    }
    ///Gets the value of [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_uniform_buffer_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn shader_sampled_image_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_sampled_image_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_buffer_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn shader_storage_image_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_storage_image_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn shader_input_attachment_array_non_uniform_indexing_native(&self) -> bool {
        unsafe { std::mem::transmute(self.shader_input_attachment_array_non_uniform_indexing_native as u8) }
    }
    ///Gets the value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn robust_buffer_access_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.robust_buffer_access_update_after_bind as u8) }
    }
    ///Gets the value of [`Self::quad_divergent_implicit_lod`]
    pub fn quad_divergent_implicit_lod(&self) -> bool {
        unsafe { std::mem::transmute(self.quad_divergent_implicit_lod as u8) }
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_samplers`]
    pub fn max_per_stage_descriptor_update_after_bind_samplers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_samplers
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_uniform_buffers`]
    pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_uniform_buffers
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_storage_buffers`]
    pub fn max_per_stage_descriptor_update_after_bind_storage_buffers(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_storage_buffers
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_sampled_images`]
    pub fn max_per_stage_descriptor_update_after_bind_sampled_images(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_sampled_images
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_storage_images`]
    pub fn max_per_stage_descriptor_update_after_bind_storage_images(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_storage_images
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_update_after_bind_input_attachments`]
    pub fn max_per_stage_descriptor_update_after_bind_input_attachments(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_input_attachments
    }
    ///Gets the value of [`Self::max_per_stage_update_after_bind_resources`]
    pub fn max_per_stage_update_after_bind_resources(&self) -> u32 {
        self.max_per_stage_update_after_bind_resources
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_samplers`]
    pub fn max_descriptor_set_update_after_bind_samplers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_samplers
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_uniform_buffers`]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_uniform_buffers
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_uniform_buffers_dynamic`]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_storage_buffers`]
    pub fn max_descriptor_set_update_after_bind_storage_buffers(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_buffers
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_storage_buffers_dynamic`]
    pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_buffers_dynamic
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_sampled_images`]
    pub fn max_descriptor_set_update_after_bind_sampled_images(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_sampled_images
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_storage_images`]
    pub fn max_descriptor_set_update_after_bind_storage_images(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_storage_images
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_input_attachments`]
    pub fn max_descriptor_set_update_after_bind_input_attachments(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_input_attachments
    }
    ///Gets the value of [`Self::supported_depth_resolve_modes`]
    pub fn supported_depth_resolve_modes(&self) -> ResolveModeFlags {
        self.supported_depth_resolve_modes
    }
    ///Gets the value of [`Self::supported_stencil_resolve_modes`]
    pub fn supported_stencil_resolve_modes(&self) -> ResolveModeFlags {
        self.supported_stencil_resolve_modes
    }
    ///Gets the value of [`Self::independent_resolve_none`]
    pub fn independent_resolve_none(&self) -> bool {
        unsafe { std::mem::transmute(self.independent_resolve_none as u8) }
    }
    ///Gets the value of [`Self::independent_resolve`]
    pub fn independent_resolve(&self) -> bool {
        unsafe { std::mem::transmute(self.independent_resolve as u8) }
    }
    ///Gets the value of [`Self::filter_minmax_single_component_formats`]
    pub fn filter_minmax_single_component_formats(&self) -> bool {
        unsafe { std::mem::transmute(self.filter_minmax_single_component_formats as u8) }
    }
    ///Gets the value of [`Self::filter_minmax_image_component_mapping`]
    pub fn filter_minmax_image_component_mapping(&self) -> bool {
        unsafe { std::mem::transmute(self.filter_minmax_image_component_mapping as u8) }
    }
    ///Gets the value of [`Self::max_timeline_semaphore_value_difference`]
    pub fn max_timeline_semaphore_value_difference(&self) -> u64 {
        self.max_timeline_semaphore_value_difference
    }
    ///Gets the value of [`Self::framebuffer_integer_color_sample_counts`]
    pub fn framebuffer_integer_color_sample_counts(&self) -> SampleCountFlags {
        self.framebuffer_integer_color_sample_counts
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
    ///Gets a mutable reference to the value of [`Self::driver_id`]
    pub fn driver_id_mut(&mut self) -> &mut DriverId {
        &mut self.driver_id
    }
    ///Gets a mutable reference to the value of [`Self::driver_name`]
    pub fn driver_name_mut(&mut self) -> &mut [c_char; MAX_DRIVER_NAME_SIZE as usize] {
        &mut self.driver_name
    }
    ///Gets a mutable reference to the value of [`Self::driver_info`]
    pub fn driver_info_mut(&mut self) -> &mut [c_char; MAX_DRIVER_INFO_SIZE as usize] {
        &mut self.driver_info
    }
    ///Gets a mutable reference to the value of [`Self::conformance_version`]
    pub fn conformance_version_mut(&mut self) -> &mut ConformanceVersion {
        &mut self.conformance_version
    }
    ///Gets a mutable reference to the value of [`Self::denorm_behavior_independence`]
    pub fn denorm_behavior_independence_mut(&mut self) -> &mut ShaderFloatControlsIndependence {
        &mut self.denorm_behavior_independence
    }
    ///Gets a mutable reference to the value of [`Self::rounding_mode_independence`]
    pub fn rounding_mode_independence_mut(&mut self) -> &mut ShaderFloatControlsIndependence {
        &mut self.rounding_mode_independence
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn shader_signed_zero_inf_nan_preserve_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn shader_signed_zero_inf_nan_preserve_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn shader_signed_zero_inf_nan_preserve_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_signed_zero_inf_nan_preserve_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_preserve_float_16`]
    pub fn shader_denorm_preserve_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_preserve_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_preserve_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_preserve_float_32`]
    pub fn shader_denorm_preserve_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_preserve_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_preserve_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_preserve_float_64`]
    pub fn shader_denorm_preserve_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_preserve_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_preserve_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn shader_denorm_flush_to_zero_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_flush_to_zero_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_flush_to_zero_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn shader_denorm_flush_to_zero_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_flush_to_zero_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_flush_to_zero_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn shader_denorm_flush_to_zero_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_denorm_flush_to_zero_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_denorm_flush_to_zero_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn shader_rounding_mode_rte_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rte_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rte_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn shader_rounding_mode_rte_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rte_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rte_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn shader_rounding_mode_rte_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rte_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rte_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn shader_rounding_mode_rtz_float_16_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rtz_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rtz_float_16 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn shader_rounding_mode_rtz_float_32_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rtz_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rtz_float_32 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn shader_rounding_mode_rtz_float_64_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_rounding_mode_rtz_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_rounding_mode_rtz_float_64 as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_update_after_bind_descriptors_in_all_pools`]
    pub fn max_update_after_bind_descriptors_in_all_pools_mut(&mut self) -> &mut u32 {
        &mut self.max_update_after_bind_descriptors_in_all_pools
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn shader_uniform_buffer_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_uniform_buffer_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_uniform_buffer_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn shader_sampled_image_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_sampled_image_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_sampled_image_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn shader_storage_buffer_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_buffer_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_buffer_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn shader_storage_image_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_storage_image_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_storage_image_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn shader_input_attachment_array_non_uniform_indexing_native_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shader_input_attachment_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shader_input_attachment_array_non_uniform_indexing_native as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn robust_buffer_access_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.robust_buffer_access_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.robust_buffer_access_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::quad_divergent_implicit_lod`]
    pub fn quad_divergent_implicit_lod_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.quad_divergent_implicit_lod as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.quad_divergent_implicit_lod as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_samplers`]
    pub fn max_per_stage_descriptor_update_after_bind_samplers_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_samplers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_uniform_buffers`]
    pub fn max_per_stage_descriptor_update_after_bind_uniform_buffers_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_uniform_buffers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_storage_buffers`]
    pub fn max_per_stage_descriptor_update_after_bind_storage_buffers_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_storage_buffers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_sampled_images`]
    pub fn max_per_stage_descriptor_update_after_bind_sampled_images_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_sampled_images
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_storage_images`]
    pub fn max_per_stage_descriptor_update_after_bind_storage_images_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_storage_images
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_input_attachments`]
    pub fn max_per_stage_descriptor_update_after_bind_input_attachments_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_input_attachments
    }
    ///Gets a mutable reference to the value of [`Self::max_per_stage_update_after_bind_resources`]
    pub fn max_per_stage_update_after_bind_resources_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_update_after_bind_resources
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_samplers`]
    pub fn max_descriptor_set_update_after_bind_samplers_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_samplers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_uniform_buffers`]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_uniform_buffers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_uniform_buffers_dynamic`]
    pub fn max_descriptor_set_update_after_bind_uniform_buffers_dynamic_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_storage_buffers`]
    pub fn max_descriptor_set_update_after_bind_storage_buffers_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_storage_buffers
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_storage_buffers_dynamic`]
    pub fn max_descriptor_set_update_after_bind_storage_buffers_dynamic_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_storage_buffers_dynamic
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_sampled_images`]
    pub fn max_descriptor_set_update_after_bind_sampled_images_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_sampled_images
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_storage_images`]
    pub fn max_descriptor_set_update_after_bind_storage_images_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_storage_images
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_input_attachments`]
    pub fn max_descriptor_set_update_after_bind_input_attachments_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_input_attachments
    }
    ///Gets a mutable reference to the value of [`Self::supported_depth_resolve_modes`]
    pub fn supported_depth_resolve_modes_mut(&mut self) -> &mut ResolveModeFlags {
        &mut self.supported_depth_resolve_modes
    }
    ///Gets a mutable reference to the value of [`Self::supported_stencil_resolve_modes`]
    pub fn supported_stencil_resolve_modes_mut(&mut self) -> &mut ResolveModeFlags {
        &mut self.supported_stencil_resolve_modes
    }
    ///Gets a mutable reference to the value of [`Self::independent_resolve_none`]
    pub fn independent_resolve_none_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.independent_resolve_none as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.independent_resolve_none as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::independent_resolve`]
    pub fn independent_resolve_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.independent_resolve as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.independent_resolve as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::filter_minmax_single_component_formats`]
    pub fn filter_minmax_single_component_formats_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.filter_minmax_single_component_formats as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.filter_minmax_single_component_formats as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::filter_minmax_image_component_mapping`]
    pub fn filter_minmax_image_component_mapping_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.filter_minmax_image_component_mapping as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.filter_minmax_image_component_mapping as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::max_timeline_semaphore_value_difference`]
    pub fn max_timeline_semaphore_value_difference_mut(&mut self) -> &mut u64 {
        &mut self.max_timeline_semaphore_value_difference
    }
    ///Gets a mutable reference to the value of [`Self::framebuffer_integer_color_sample_counts`]
    pub fn framebuffer_integer_color_sample_counts_mut(&mut self) -> &mut SampleCountFlags {
        &mut self.framebuffer_integer_color_sample_counts
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
    ///Sets the raw value of [`Self::driver_id`]
    pub fn set_driver_id(&mut self, value: crate::vulkan1_2::DriverId) -> &mut Self {
        self.driver_id = value;
        self
    }
    ///Sets the raw value of [`Self::driver_name`]
    pub fn set_driver_name(
        &mut self,
        value: [std::os::raw::c_char; crate::vulkan1_2::MAX_DRIVER_NAME_SIZE as usize],
    ) -> &mut Self {
        self.driver_name = value;
        self
    }
    ///Sets the raw value of [`Self::driver_info`]
    pub fn set_driver_info(
        &mut self,
        value: [std::os::raw::c_char; crate::vulkan1_2::MAX_DRIVER_INFO_SIZE as usize],
    ) -> &mut Self {
        self.driver_info = value;
        self
    }
    ///Sets the raw value of [`Self::conformance_version`]
    pub fn set_conformance_version(&mut self, value: crate::vulkan1_2::ConformanceVersion) -> &mut Self {
        self.conformance_version = value;
        self
    }
    ///Sets the raw value of [`Self::denorm_behavior_independence`]
    pub fn set_denorm_behavior_independence(
        &mut self,
        value: crate::vulkan1_2::ShaderFloatControlsIndependence,
    ) -> &mut Self {
        self.denorm_behavior_independence = value;
        self
    }
    ///Sets the raw value of [`Self::rounding_mode_independence`]
    pub fn set_rounding_mode_independence(
        &mut self,
        value: crate::vulkan1_2::ShaderFloatControlsIndependence,
    ) -> &mut Self {
        self.rounding_mode_independence = value;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_16`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_32`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_signed_zero_inf_nan_preserve_float_64`]
    pub fn set_shader_signed_zero_inf_nan_preserve_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_signed_zero_inf_nan_preserve_float_64 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_16`]
    pub fn set_shader_denorm_preserve_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_preserve_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_32`]
    pub fn set_shader_denorm_preserve_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_preserve_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_preserve_float_64`]
    pub fn set_shader_denorm_preserve_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_preserve_float_64 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_16`]
    pub fn set_shader_denorm_flush_to_zero_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_32`]
    pub fn set_shader_denorm_flush_to_zero_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_denorm_flush_to_zero_float_64`]
    pub fn set_shader_denorm_flush_to_zero_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_denorm_flush_to_zero_float_64 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_16`]
    pub fn set_shader_rounding_mode_rte_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rte_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_32`]
    pub fn set_shader_rounding_mode_rte_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rte_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rte_float_64`]
    pub fn set_shader_rounding_mode_rte_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rte_float_64 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_16`]
    pub fn set_shader_rounding_mode_rtz_float_16(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rtz_float_16 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_32`]
    pub fn set_shader_rounding_mode_rtz_float_32(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rtz_float_32 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_rounding_mode_rtz_float_64`]
    pub fn set_shader_rounding_mode_rtz_float_64(&mut self, value: bool) -> &mut Self {
        self.shader_rounding_mode_rtz_float_64 = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_update_after_bind_descriptors_in_all_pools`]
    pub fn set_max_update_after_bind_descriptors_in_all_pools(&mut self, value: u32) -> &mut Self {
        self.max_update_after_bind_descriptors_in_all_pools = value;
        self
    }
    ///Sets the raw value of [`Self::shader_uniform_buffer_array_non_uniform_indexing_native`]
    pub fn set_shader_uniform_buffer_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_uniform_buffer_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_sampled_image_array_non_uniform_indexing_native`]
    pub fn set_shader_sampled_image_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_sampled_image_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_buffer_array_non_uniform_indexing_native`]
    pub fn set_shader_storage_buffer_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_storage_buffer_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_storage_image_array_non_uniform_indexing_native`]
    pub fn set_shader_storage_image_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_storage_image_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shader_input_attachment_array_non_uniform_indexing_native`]
    pub fn set_shader_input_attachment_array_non_uniform_indexing_native(&mut self, value: bool) -> &mut Self {
        self.shader_input_attachment_array_non_uniform_indexing_native = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::robust_buffer_access_update_after_bind`]
    pub fn set_robust_buffer_access_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.robust_buffer_access_update_after_bind = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::quad_divergent_implicit_lod`]
    pub fn set_quad_divergent_implicit_lod(&mut self, value: bool) -> &mut Self {
        self.quad_divergent_implicit_lod = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_samplers`]
    pub fn set_max_per_stage_descriptor_update_after_bind_samplers(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_samplers = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_uniform_buffers`]
    pub fn set_max_per_stage_descriptor_update_after_bind_uniform_buffers(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_uniform_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_storage_buffers`]
    pub fn set_max_per_stage_descriptor_update_after_bind_storage_buffers(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_storage_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_sampled_images`]
    pub fn set_max_per_stage_descriptor_update_after_bind_sampled_images(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_sampled_images = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_storage_images`]
    pub fn set_max_per_stage_descriptor_update_after_bind_storage_images(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_storage_images = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_update_after_bind_input_attachments`]
    pub fn set_max_per_stage_descriptor_update_after_bind_input_attachments(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_input_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_update_after_bind_resources`]
    pub fn set_max_per_stage_update_after_bind_resources(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_update_after_bind_resources = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_samplers`]
    pub fn set_max_descriptor_set_update_after_bind_samplers(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_samplers = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_uniform_buffers`]
    pub fn set_max_descriptor_set_update_after_bind_uniform_buffers(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_uniform_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_uniform_buffers_dynamic`]
    pub fn set_max_descriptor_set_update_after_bind_uniform_buffers_dynamic(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_uniform_buffers_dynamic = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_storage_buffers`]
    pub fn set_max_descriptor_set_update_after_bind_storage_buffers(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_storage_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_storage_buffers_dynamic`]
    pub fn set_max_descriptor_set_update_after_bind_storage_buffers_dynamic(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_storage_buffers_dynamic = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_sampled_images`]
    pub fn set_max_descriptor_set_update_after_bind_sampled_images(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_sampled_images = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_storage_images`]
    pub fn set_max_descriptor_set_update_after_bind_storage_images(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_storage_images = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_input_attachments`]
    pub fn set_max_descriptor_set_update_after_bind_input_attachments(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_input_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::supported_depth_resolve_modes`]
    pub fn set_supported_depth_resolve_modes(&mut self, value: crate::vulkan1_2::ResolveModeFlags) -> &mut Self {
        self.supported_depth_resolve_modes = value;
        self
    }
    ///Sets the raw value of [`Self::supported_stencil_resolve_modes`]
    pub fn set_supported_stencil_resolve_modes(&mut self, value: crate::vulkan1_2::ResolveModeFlags) -> &mut Self {
        self.supported_stencil_resolve_modes = value;
        self
    }
    ///Sets the raw value of [`Self::independent_resolve_none`]
    pub fn set_independent_resolve_none(&mut self, value: bool) -> &mut Self {
        self.independent_resolve_none = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::independent_resolve`]
    pub fn set_independent_resolve(&mut self, value: bool) -> &mut Self {
        self.independent_resolve = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::filter_minmax_single_component_formats`]
    pub fn set_filter_minmax_single_component_formats(&mut self, value: bool) -> &mut Self {
        self.filter_minmax_single_component_formats = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::filter_minmax_image_component_mapping`]
    pub fn set_filter_minmax_image_component_mapping(&mut self, value: bool) -> &mut Self {
        self.filter_minmax_image_component_mapping = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::max_timeline_semaphore_value_difference`]
    pub fn set_max_timeline_semaphore_value_difference(&mut self, value: u64) -> &mut Self {
        self.max_timeline_semaphore_value_difference = value;
        self
    }
    ///Sets the raw value of [`Self::framebuffer_integer_color_sample_counts`]
    pub fn set_framebuffer_integer_color_sample_counts(
        &mut self,
        value: crate::vulkan1_0::SampleCountFlags,
    ) -> &mut Self {
        self.framebuffer_integer_color_sample_counts = value;
        self
    }
}
