#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
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
/// - [`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32`] specifies that
///the statistic is returned as a 32-bit boolean value which **must** be
///either [`TRUE`] or [`FALSE`] and **should** be read from the
///`b32` field of [`PipelineExecutableStatisticValueKHR`].
/// - [`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64`] specifies that
///the statistic is returned as a signed 64-bit integer and **should** be read
///from the `i64` field of [`PipelineExecutableStatisticValueKHR`].
/// - [`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64`] specifies that
///the statistic is returned as an unsigned 64-bit integer and **should** be
///read from the `u64` field of
///[`PipelineExecutableStatisticValueKHR`].
/// - [`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64`] specifies that
///the statistic is returned as a 64-bit floating-point value and **should**
///be read from the `f64` field of
///[`PipelineExecutableStatisticValueKHR`].
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineExecutableStatisticFormatKHR(i32);
impl const Default for PipelineExecutableStatisticFormatKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PipelineExecutableStatisticFormatKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PipelineExecutableStatisticFormatKHR")
            .field(match *self {
                Self::PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32 => &"PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32",
                Self::PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64 => &"PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64",
                Self::PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64 => &"PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64",
                Self::PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64 => &"PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64",
                other => unreachable!("invalid value for `PipelineExecutableStatisticFormatKHR`: {:?}", other),
            })
            .finish()
    }
}
impl PipelineExecutableStatisticFormatKHR {
    ///[`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32`] specifies that
    ///the statistic is returned as a 32-bit boolean value which **must** be
    ///either [`TRUE`] or [`FALSE`] and **should** be read from the
    ///`b32` field of [`PipelineExecutableStatisticValueKHR`].
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32: Self = Self(0);
    ///[`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64`] specifies that
    ///the statistic is returned as a signed 64-bit integer and **should** be read
    ///from the `i64` field of [`PipelineExecutableStatisticValueKHR`].
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64: Self = Self(1);
    ///[`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64`] specifies that
    ///the statistic is returned as an unsigned 64-bit integer and **should** be
    ///read from the `u64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64: Self = Self(2);
    ///[`PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64`] specifies that
    ///the statistic is returned as a 64-bit floating-point value and **should**
    ///be read from the `f64` field of
    ///[`PipelineExecutableStatisticValueKHR`].
    pub const PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64: Self = Self(3);
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
