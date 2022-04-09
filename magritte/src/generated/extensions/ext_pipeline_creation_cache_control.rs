//![VK_EXT_pipeline_creation_cache_control](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_creation_cache_control.html) - device extension
//!# Description
//!This extension adds flags to `Vk*PipelineCreateInfo` and
//![`PipelineCacheCreateInfo`] structures with the aim of improving the
//!predictability of pipeline creation cost.
//!The goal is to provide information about potentially expensive hazards
//!within the client driver during pipeline creation to the application before
//!carrying them out rather than after.
//!# Revision
//!3
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Gregory Grebe [grgrebe_amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_pipeline_creation_cache_control]
//!   @grgrebe_amd%0A<<Here describe the issue or question you have about the
//!   VK_EXT_pipeline_creation_cache_control extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDevicePipelineCreationCacheControlFeaturesEXT`]
//!# New enums
//! - [`PipelineCacheCreateFlagBits`]
//!# New constants
//! - [`EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME`]
//! - [`EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION`]
//! - Extending [`PipelineCacheCreateFlagBits`]:  -
//!   `VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT`
//! - Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT`
//!   - `VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT`
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT`  -
//!   `VK_PIPELINE_COMPILE_REQUIRED_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2019-11-01 (Gregory Grebe)  - Initial revision
//! - Revision 2, 2020-02-24 (Gregory Grebe)  - Initial public revision
//! - Revision 3, 2020-03-23 (Tobias Hector)  - Changed `VK_PIPELINE_COMPILE_REQUIRED_EXT` to a
//!   success code, adding an alias for the original `VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT`. Also
//!   updated the xml to include these codes as return values.
//!# Other info
//! * 2020-03-23
//! * - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//! * - Gregory Grebe, AMD  - Tobias Hector, AMD  - Matthaeus Chajdas, AMD  - Mitch Singer, AMD  -
//!   Spencer Fricke, Samsung Electronics  - Stuart Smith, Imagination Technologies  - Jeff Bolz,
//!   NVIDIA Corporation  - Daniel Koch, NVIDIA Corporation  - Dan Ginsburg, Valve Corporation  -
//!   Jeff Leger, QUALCOMM  - Michal Pietrasiuk, Intel  - Jan-Harald Fredriksen, Arm Limited
//!# Related
//! - [`PhysicalDevicePipelineCreationCacheControlFeaturesEXT`]
//! - [`PipelineCacheCreateFlagBits`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION")]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME")]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_EXT_pipeline_creation_cache_control");
///[VkPipelineCacheCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html) - Bitmask specifying the behavior of the pipeline cache
///# C Specifications
///Bits which  **can**  be set in [`PipelineCacheCreateInfo::flags`],
///specifying behavior of the pipeline cache, are:
///```c
///// Provided by VK_EXT_pipeline_creation_cache_control
///typedef enum VkPipelineCacheCreateFlagBits {
///  // Provided by VK_VERSION_1_3
///    VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT = 0x00000001,
///  // Provided by VK_EXT_pipeline_creation_cache_control
///    VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT =
/// VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT,
///} VkPipelineCacheCreateFlagBits;
///```
/// # Description
/// - [`EXTERNALLY_SYNCHRONIZED`] specifies that all commands that modify the created [`PipelineCache`] will be [externally synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-threadingbehavior). When set, the implementation  **may**  skip any unnecessary processing needed to support simultaneous modification from multiple threads where allowed.
/// # Related
/// - [`VK_EXT_pipeline_creation_cache_control`]
/// - [`PipelineCacheCreateFlags`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineCacheCreateFlagBits")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineCacheCreateFlagBits(u32);
impl const Default for PipelineCacheCreateFlagBits {
    fn default() -> Self {
        Self(0)
    }
}
impl PipelineCacheCreateFlagBits {
    ///[`EXTERNALLY_SYNCHRONIZED`] specifies
    ///that all commands that modify the created [`PipelineCache`] will be
    ///[externally synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-threadingbehavior).
    ///When set, the implementation  **may**  skip any unnecessary processing needed
    ///to support simultaneous modification from multiple threads where
    ///allowed.
    ///
    ///Provided by [`crate::vulkan1_3`]
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for PipelineCacheCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PipelineCacheCreateFlagBits))
            .field(match *self {
                Self::EXTERNALLY_SYNCHRONIZED => &"EXTERNALLY_SYNCHRONIZED",
                other => unreachable!(
                    concat!("invalid value for", stringify!(PipelineCacheCreateFlagBits), ": {:?}"),
                    other
                ),
            })
            .finish()
    }
}
impl std::fmt::Display for PipelineCacheCreateFlagBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::EXTERNALLY_SYNCHRONIZED => &"EXTERNALLY_SYNCHRONIZED",
            other => unreachable!(
                concat!("invalid value for", stringify!(PipelineCacheCreateFlagBits), ": {:?}"),
                other
            ),
        })
    }
}
