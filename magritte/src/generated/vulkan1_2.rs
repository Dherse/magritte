#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
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
/// - [`SEMAPHORE_TYPE_BINARY`] specifies a *binary semaphore* type that
///has a boolean payload indicating whether the semaphore is currently
///signaled or unsignaled.
///When created, the semaphore is in the unsignaled state.
/// - [`SEMAPHORE_TYPE_TIMELINE`] specifies a *timeline semaphore* type
///that has a strictly increasing 64-bit unsigned integer payload
///indicating whether the semaphore is signaled with respect to a
///particular reference value.
///When created, the semaphore payload has the value given by the
///`initialValue` field of [`SemaphoreTypeCreateInfo`].
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SemaphoreType(i32);
impl const Default for SemaphoreType {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for SemaphoreType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("SemaphoreType")
            .field(match *self {
                Self::SEMAPHORE_TYPE_BINARY => &"SEMAPHORE_TYPE_BINARY",
                Self::SEMAPHORE_TYPE_TIMELINE => &"SEMAPHORE_TYPE_TIMELINE",
                other => unreachable!("invalid value for `SemaphoreType`: {:?}", other),
            })
            .finish()
    }
}
impl SemaphoreType {
    ///[`SEMAPHORE_TYPE_BINARY`] specifies a *binary semaphore* type that
    ///has a boolean payload indicating whether the semaphore is currently
    ///signaled or unsignaled.
    ///When created, the semaphore is in the unsignaled state.
    pub const SEMAPHORE_TYPE_BINARY: Self = Self(0);
    ///[`SEMAPHORE_TYPE_TIMELINE`] specifies a *timeline semaphore* type
    ///that has a strictly increasing 64-bit unsigned integer payload
    ///indicating whether the semaphore is signaled with respect to a
    ///particular reference value.
    ///When created, the semaphore payload has the value given by the
    ///`initialValue` field of [`SemaphoreTypeCreateInfo`].
    pub const SEMAPHORE_TYPE_TIMELINE: Self = Self(1);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_timeline_semaphore`]
    pub const SEMAPHORE_TYPE_BINARY_KHR: Self = Self::VK_SEMAPHORE_TYPE_BINARY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_timeline_semaphore`]
    pub const SEMAPHORE_TYPE_TIMELINE_KHR: Self = Self::VK_SEMAPHORE_TYPE_TIMELINE;
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
/// - [`SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE`] specifies that texel
///values are combined by computing a weighted average of values in the
///footprint, using weights as specified in
///[the image operations chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-unnormalized-to-integer).
/// - [`SAMPLER_REDUCTION_MODE_MIN`] specifies that texel values are
///combined by taking the component-wise minimum of values in the footprint
///with non-zero weights.
/// - [`SAMPLER_REDUCTION_MODE_MAX`] specifies that texel values are
///combined by taking the component-wise maximum of values in the footprint
///with non-zero weights.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SamplerReductionMode(i32);
impl const Default for SamplerReductionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for SamplerReductionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("SamplerReductionMode")
            .field(match *self {
                Self::SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE => &"SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE",
                Self::SAMPLER_REDUCTION_MODE_MIN => &"SAMPLER_REDUCTION_MODE_MIN",
                Self::SAMPLER_REDUCTION_MODE_MAX => &"SAMPLER_REDUCTION_MODE_MAX",
                other => unreachable!("invalid value for `SamplerReductionMode`: {:?}", other),
            })
            .finish()
    }
}
impl SamplerReductionMode {
    ///[`SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE`] specifies that texel
    ///values are combined by computing a weighted average of values in the
    ///footprint, using weights as specified in
    ///[the image operations chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-unnormalized-to-integer).
    pub const SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: Self = Self(0);
    ///[`SAMPLER_REDUCTION_MODE_MIN`] specifies that texel values are
    ///combined by taking the component-wise minimum of values in the footprint
    ///with non-zero weights.
    pub const SAMPLER_REDUCTION_MODE_MIN: Self = Self(1);
    ///[`SAMPLER_REDUCTION_MODE_MAX`] specifies that texel values are
    ///combined by taking the component-wise maximum of values in the footprint
    ///with non-zero weights.
    pub const SAMPLER_REDUCTION_MODE_MAX: Self = Self(2);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_sampler_filter_minmax`]
    pub const SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT: Self = Self::VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_sampler_filter_minmax`]
    pub const SAMPLER_REDUCTION_MODE_MIN_EXT: Self = Self::VK_SAMPLER_REDUCTION_MODE_MIN;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_sampler_filter_minmax`]
    pub const SAMPLER_REDUCTION_MODE_MAX_EXT: Self = Self::VK_SAMPLER_REDUCTION_MODE_MAX;
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
///# Description
///
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DriverId(i32);
impl const Default for DriverId {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DriverId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("DriverId")
            .field(match *self {
                Self::DRIVER_ID_AMD_PROPRIETARY => &"DRIVER_ID_AMD_PROPRIETARY",
                Self::DRIVER_ID_AMD_OPEN_SOURCE => &"DRIVER_ID_AMD_OPEN_SOURCE",
                Self::DRIVER_ID_MESA_RADV => &"DRIVER_ID_MESA_RADV",
                Self::DRIVER_ID_NVIDIA_PROPRIETARY => &"DRIVER_ID_NVIDIA_PROPRIETARY",
                Self::DRIVER_ID_INTEL_PROPRIETARY_WINDOWS => &"DRIVER_ID_INTEL_PROPRIETARY_WINDOWS",
                Self::DRIVER_ID_INTEL_OPEN_SOURCE_MESA => &"DRIVER_ID_INTEL_OPEN_SOURCE_MESA",
                Self::DRIVER_ID_IMAGINATION_PROPRIETARY => &"DRIVER_ID_IMAGINATION_PROPRIETARY",
                Self::DRIVER_ID_QUALCOMM_PROPRIETARY => &"DRIVER_ID_QUALCOMM_PROPRIETARY",
                Self::DRIVER_ID_ARM_PROPRIETARY => &"DRIVER_ID_ARM_PROPRIETARY",
                Self::DRIVER_ID_GOOGLE_SWIFTSHADER => &"DRIVER_ID_GOOGLE_SWIFTSHADER",
                Self::DRIVER_ID_GGP_PROPRIETARY => &"DRIVER_ID_GGP_PROPRIETARY",
                Self::DRIVER_ID_BROADCOM_PROPRIETARY => &"DRIVER_ID_BROADCOM_PROPRIETARY",
                Self::DRIVER_ID_MESA_LLVMPIPE => &"DRIVER_ID_MESA_LLVMPIPE",
                Self::DRIVER_ID_MOLTENVK => &"DRIVER_ID_MOLTENVK",
                Self::DRIVER_ID_COREAVI_PROPRIETARY => &"DRIVER_ID_COREAVI_PROPRIETARY",
                Self::DRIVER_ID_JUICE_PROPRIETARY => &"DRIVER_ID_JUICE_PROPRIETARY",
                Self::DRIVER_ID_VERISILICON_PROPRIETARY => &"DRIVER_ID_VERISILICON_PROPRIETARY",
                Self::DRIVER_ID_MESA_TURNIP => &"DRIVER_ID_MESA_TURNIP",
                Self::DRIVER_ID_MESA_V3DV => &"DRIVER_ID_MESA_V3DV",
                Self::DRIVER_ID_MESA_PANVK => &"DRIVER_ID_MESA_PANVK",
                Self::DRIVER_ID_SAMSUNG_PROPRIETARY => &"DRIVER_ID_SAMSUNG_PROPRIETARY",
                Self::DRIVER_ID_MESA_VENUS => &"DRIVER_ID_MESA_VENUS",
                other => unreachable!("invalid value for `DriverId`: {:?}", other),
            })
            .finish()
    }
}
impl DriverId {
    ///No documentation found
    pub const DRIVER_ID_AMD_PROPRIETARY: Self = Self(1);
    ///No documentation found
    pub const DRIVER_ID_AMD_OPEN_SOURCE: Self = Self(2);
    ///No documentation found
    pub const DRIVER_ID_MESA_RADV: Self = Self(3);
    ///No documentation found
    pub const DRIVER_ID_NVIDIA_PROPRIETARY: Self = Self(4);
    ///No documentation found
    pub const DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    ///No documentation found
    pub const DRIVER_ID_INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    ///No documentation found
    pub const DRIVER_ID_IMAGINATION_PROPRIETARY: Self = Self(7);
    ///No documentation found
    pub const DRIVER_ID_QUALCOMM_PROPRIETARY: Self = Self(8);
    ///No documentation found
    pub const DRIVER_ID_ARM_PROPRIETARY: Self = Self(9);
    ///No documentation found
    pub const DRIVER_ID_GOOGLE_SWIFTSHADER: Self = Self(10);
    ///No documentation found
    pub const DRIVER_ID_GGP_PROPRIETARY: Self = Self(11);
    ///No documentation found
    pub const DRIVER_ID_BROADCOM_PROPRIETARY: Self = Self(12);
    ///No documentation found
    pub const DRIVER_ID_MESA_LLVMPIPE: Self = Self(13);
    ///No documentation found
    pub const DRIVER_ID_MOLTENVK: Self = Self(14);
    ///No documentation found
    pub const DRIVER_ID_COREAVI_PROPRIETARY: Self = Self(15);
    ///No documentation found
    pub const DRIVER_ID_JUICE_PROPRIETARY: Self = Self(16);
    ///No documentation found
    pub const DRIVER_ID_VERISILICON_PROPRIETARY: Self = Self(17);
    ///No documentation found
    pub const DRIVER_ID_MESA_TURNIP: Self = Self(18);
    ///No documentation found
    pub const DRIVER_ID_MESA_V3DV: Self = Self(19);
    ///No documentation found
    pub const DRIVER_ID_MESA_PANVK: Self = Self(20);
    ///No documentation found
    pub const DRIVER_ID_SAMSUNG_PROPRIETARY: Self = Self(21);
    ///No documentation found
    pub const DRIVER_ID_MESA_VENUS: Self = Self(22);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_AMD_PROPRIETARY_KHR: Self = Self::VK_DRIVER_ID_AMD_PROPRIETARY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_AMD_OPEN_SOURCE_KHR: Self = Self::VK_DRIVER_ID_AMD_OPEN_SOURCE;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_MESA_RADV_KHR: Self = Self::VK_DRIVER_ID_MESA_RADV;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_NVIDIA_PROPRIETARY_KHR: Self = Self::VK_DRIVER_ID_NVIDIA_PROPRIETARY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self::VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_INTEL_OPEN_SOURCE_MESA_KHR: Self = Self::VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_IMAGINATION_PROPRIETARY_KHR: Self = Self::VK_DRIVER_ID_IMAGINATION_PROPRIETARY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_QUALCOMM_PROPRIETARY_KHR: Self = Self::VK_DRIVER_ID_QUALCOMM_PROPRIETARY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_ARM_PROPRIETARY_KHR: Self = Self::VK_DRIVER_ID_ARM_PROPRIETARY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_GOOGLE_SWIFTSHADER_KHR: Self = Self::VK_DRIVER_ID_GOOGLE_SWIFTSHADER;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_GGP_PROPRIETARY_KHR: Self = Self::VK_DRIVER_ID_GGP_PROPRIETARY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_driver_properties`]
    pub const DRIVER_ID_BROADCOM_PROPRIETARY_KHR: Self = Self::VK_DRIVER_ID_BROADCOM_PROPRIETARY;
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
/// - [`SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY`] specifies that
///shader float controls for 32-bit floating point **can** be set
///independently; other bit widths **must** be set identically to each other.
/// - [`SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL`] specifies that shader
///float controls for all bit widths **can** be set independently.
/// - [`SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE`] specifies that shader
///float controls for all bit widths **must** be set identically.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShaderFloatControlsIndependence(i32);
impl const Default for ShaderFloatControlsIndependence {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ShaderFloatControlsIndependence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ShaderFloatControlsIndependence")
            .field(match *self {
                Self::SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY => {
                    &"SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY"
                },
                Self::SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL => &"SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL",
                Self::SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE => &"SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE",
                other => unreachable!("invalid value for `ShaderFloatControlsIndependence`: {:?}", other),
            })
            .finish()
    }
}
impl ShaderFloatControlsIndependence {
    ///[`SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY`] specifies that
    ///shader float controls for 32-bit floating point **can** be set
    ///independently; other bit widths **must** be set identically to each other.
    pub const SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: Self = Self(0);
    ///[`SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL`] specifies that shader
    ///float controls for all bit widths **can** be set independently.
    pub const SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: Self = Self(1);
    ///[`SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE`] specifies that shader
    ///float controls for all bit widths **must** be set identically.
    pub const SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: Self = Self(2);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_shader_float_controls`]
    pub const SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR: Self =
        Self::VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_shader_float_controls`]
    pub const SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR: Self = Self::VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_shader_float_controls`]
    pub const SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR: Self = Self::VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE;
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
