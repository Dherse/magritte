//![VK_EXT_custom_border_color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_custom_border_color.html) - device extension
//!# Description
//!This extension provides cross-vendor functionality to specify a custom
//!border color for use when the sampler address mode
//!`VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER` is used.To create a sampler which uses a custom border
//! color set
//![`SamplerCreateInfo::border_color`] to one of:
//! - `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`
//! - `VK_BORDER_COLOR_INT_CUSTOM_EXT`
//!When `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT` or
//!`VK_BORDER_COLOR_INT_CUSTOM_EXT` is used, applications must provide a
//![`SamplerCustomBorderColorCreateInfoEXT`] in the `pNext` chain for
//![`SamplerCreateInfo`].
//!# Revision
//!12
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Liam Middlebrook [liam-middlebrook](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_custom_border_color]
//!   @liam-middlebrook%0A<<Here describe the issue or question you have about the
//!   VK_EXT_custom_border_color extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceCustomBorderColorFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceCustomBorderColorPropertiesEXT`]
//! - Extending [`SamplerCreateInfo`]:  - [`SamplerCustomBorderColorCreateInfoEXT`]
//!# New constants
//! - [`EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME`]
//! - [`EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION`]
//! - Extending [`BorderColor`]:  - `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`  -
//!   `VK_BORDER_COLOR_INT_CUSTOM_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT`
//!# Known issues & F.A.Q
//!1) Should VkClearColorValue be used for the border color value, or should we
//!have our own struct/union? Do we need to specify the type of the input
//!values for the components? This is more of a concern if VkClearColorValue is
//!used here because it provides a union of float,int,uint types. **RESOLVED** : Will reuse
//! existing VkClearColorValue structure in order to
//!easily take advantage of float,int,uint borderColor types.2) For hardware which supports a
//! limited number of border colors what
//!happens if that number is exceeded? Should this be handled by the driver
//!unbeknownst to the application? In Revision 1 we had solved this issue using
//!a new Object type, however that may have lead to additional system resource
//!consumption which would otherwise not be required. **RESOLVED** : Added
//![`PhysicalDeviceCustomBorderColorPropertiesEXT::max_custom_border_color_samplers`]
//!for tracking implementation-specific limit, and Valid Usage statement
//!handling overflow.3) Should this be supported for immutable samplers at all, or by a feature
//!bit? Some implementations may not be able to support custom border colors on
//!immutable samplers — is it worthwhile enabling this to work on them for
//!implementations that can support it, or forbidding it entirely. **RESOLVED** : Samplers created
//! with a custom border color are forbidden from
//!being immutable.
//!This resolves concerns for implementations where the custom border color is
//!an index to a LUT instead of being directly embedded into sampler state.4) Should UINT and SINT
//! (unsigned integer and signed integer) border color
//!types be separated or should they be combined into one generic INT (integer)
//!type? **RESOLVED** : Separating these does not make much sense as the existing fixed
//!border color types do not have this distinction, and there is no reason in
//!hardware to do so.
//!This separation would also create unnecessary work and considerations for
//!the application.
//!# Version History
//! - Revision 1, 2019-10-10 (Joshua Ashton)  - Internal revisions.
//! - Revision 2, 2019-10-11 (Liam Middlebrook)  - Remove VkCustomBorderColor object and associated
//!   functions  - Add issues concerning HW limitations for custom border color count
//! - Revision 3, 2019-10-12 (Joshua Ashton)  - Re-expose the limits for the maximum number of
//!   unique border colors  - Add extra details about border color tracking  - Fix typos
//! - Revision 4, 2019-10-12 (Joshua Ashton)  - Changed maxUniqueCustomBorderColors to a uint32_t
//!   from a VkDeviceSize
//! - Revision 5, 2019-10-14 (Liam Middlebrook)  - Added features bit
//! - Revision 6, 2019-10-15 (Joshua Ashton)  - Type-ize VK_BORDER_COLOR_CUSTOM  - Fix const-ness on
//!   `pNext` of VkSamplerCustomBorderColorCreateInfoEXT
//! - Revision 7, 2019-11-26 (Liam Middlebrook)  - Renamed maxUniqueCustomBorderColors to
//!   maxCustomBorderColors
//! - Revision 8, 2019-11-29 (Joshua Ashton)  - Renamed borderColor member of
//!   VkSamplerCustomBorderColorCreateInfoEXT to customBorderColor
//! - Revision 9, 2020-02-19 (Joshua Ashton)  - Renamed maxCustomBorderColors to
//!   maxCustomBorderColorSamplers
//! - Revision 10, 2020-02-21 (Joshua Ashton)  - Added format to
//!   VkSamplerCustomBorderColorCreateInfoEXT and feature bit
//! - Revision 11, 2020-04-07 (Joshua Ashton)  - Dropped UINT/SINT border color differences,
//!   consolidated types
//! - Revision 12, 2020-04-16 (Joshua Ashton)  - Renamed VK_BORDER_COLOR_CUSTOM_FLOAT_EXT to
//!   VK_BORDER_COLOR_FLOAT_CUSTOM_EXT for consistency
//!# Other info
//! * 2020-04-16
//! * No known IP claims.
//! * - Joshua Ashton, Valve  - Hans-Kristian Arntzen, Valve  - Philip Rebohle, Valve  - Liam
//!   Middlebrook, NVIDIA  - Jeff Bolz, NVIDIA  - Tobias Hector, AMD  - Jason Ekstrand, Intel  -
//!   Spencer Fricke, Samsung Electronics  - Graeme Leese, Broadcom  - Jesse Hall, Google  -
//!   Jan-Harald Fredriksen, ARM  - Tom Olson, ARM  - Stuart Smith, Imagination Technologies  -
//!   Donald Scorgie, Imagination Technologies  - Alex Walters, Imagination Technologies  - Peter
//!   Quayle, Imagination Technologies
//!# Related
//! - [`PhysicalDeviceCustomBorderColorFeaturesEXT`]
//! - [`PhysicalDeviceCustomBorderColorPropertiesEXT`]
//! - [`SamplerCustomBorderColorCreateInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, ClearColorValue, Format, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION")]
pub const EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME")]
pub const EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_custom_border_color");
///[VkSamplerCustomBorderColorCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html) - Structure specifying custom border color
///# C Specifications
///In addition to the predefined border color values, applications  **can**  provide
///a custom border color value by including the
///[`SamplerCustomBorderColorCreateInfoEXT`] structure in the
///[`SamplerCreateInfo`]::[`p_next`] chain.The [`SamplerCustomBorderColorCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_custom_border_color
///typedef struct VkSamplerCustomBorderColorCreateInfoEXT {
///    VkStructureType      sType;
///    const void*          pNext;
///    VkClearColorValue    customBorderColor;
///    VkFormat             format;
///} VkSamplerCustomBorderColorCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`custom_border_color`] is a [`ClearColorValue`] representing the desired custom sampler
///   border color.
/// - [`format`] is a [`Format`] representing the format of the sampled image view(s). This field may be `VK_FORMAT_UNDEFINED` if the [customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat) feature is enabled.
///# Description
///## Valid Usage
/// -    If provided [`format`] is not `VK_FORMAT_UNDEFINED` then the [`SamplerCreateInfo::border_color`] type  **must**  match the sampled type of the provided [`format`], as shown in the *SPIR-V Sampled Type* column of the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat) table
/// - If the [customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat)
///   feature is not enabled then [`format`] **must**  not be `VK_FORMAT_UNDEFINED`
/// - If the sampler is used to sample an image view of `VK_FORMAT_B4G4R4A4_UNORM_PACK16`,
///   `VK_FORMAT_B5G6R5_UNORM_PACK16`, or `VK_FORMAT_B5G5R5A1_UNORM_PACK16` format then [`format`]
///   **must**  not be `VK_FORMAT_UNDEFINED`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT`
/// - [`format`] **must**  be a valid [`Format`] value
///# Related
/// - [`VK_EXT_custom_border_color`]
/// - [`ClearColorValue`]
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
#[doc(alias = "VkSamplerCustomBorderColorCreateInfoEXT")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`custom_border_color`] is a [`ClearColorValue`] representing the
    ///desired custom sampler border color.
    pub custom_border_color: ClearColorValue,
    ///[`format`] is a [`Format`] representing the format of the sampled
    ///image view(s).
    ///This field may be `VK_FORMAT_UNDEFINED` if the
    ///[customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat)
    ///feature is enabled.
    pub format: Format,
}
impl<'lt> Default for SamplerCustomBorderColorCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            custom_border_color: unsafe { std::mem::zeroed() },
            format: Default::default(),
        }
    }
}
impl<'lt> SamplerCustomBorderColorCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::custom_border_color`]
    pub fn custom_border_color(&self) -> ClearColorValue {
        self.custom_border_color
    }
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::custom_border_color`]
    pub fn custom_border_color_mut(&mut self) -> &mut ClearColorValue {
        &mut self.custom_border_color
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::custom_border_color`]
    pub fn set_custom_border_color(mut self, value: crate::vulkan1_0::ClearColorValue) -> Self {
        self.custom_border_color = value;
        self
    }
    ///Sets the value of [`Self::format`]
    pub fn set_format(mut self, value: crate::vulkan1_0::Format) -> Self {
        self.format = value;
        self
    }
}
///[VkPhysicalDeviceCustomBorderColorPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html) - Structure describing whether custom border colors can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCustomBorderColorPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_custom_border_color
///typedef struct VkPhysicalDeviceCustomBorderColorPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxCustomBorderColorSamplers;
///} VkPhysicalDeviceCustomBorderColorPropertiesEXT;
///```
///# Members
/// - [`max_custom_border_color_samplers`] indicates the maximum number of samplers with custom
///   border colors which  **can**  simultaneously exist on a device.
///# Description
///If the [`PhysicalDeviceCustomBorderColorPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_custom_border_color`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceCustomBorderColorPropertiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_custom_border_color_samplers`] indicates the maximum number of
    ///samplers with custom border colors which  **can**  simultaneously exist on a
    ///device.
    pub max_custom_border_color_samplers: u32,
}
impl<'lt> Default for PhysicalDeviceCustomBorderColorPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_custom_border_color_samplers: 0,
        }
    }
}
impl<'lt> PhysicalDeviceCustomBorderColorPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::max_custom_border_color_samplers`]
    pub fn max_custom_border_color_samplers(&self) -> u32 {
        self.max_custom_border_color_samplers
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
    ///Gets a mutable reference to the value of [`Self::max_custom_border_color_samplers`]
    pub fn max_custom_border_color_samplers_mut(&mut self) -> &mut u32 {
        &mut self.max_custom_border_color_samplers
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::max_custom_border_color_samplers`]
    pub fn set_max_custom_border_color_samplers(mut self, value: u32) -> Self {
        self.max_custom_border_color_samplers = value;
        self
    }
}
///[VkPhysicalDeviceCustomBorderColorFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html) - Structure describing whether custom border colors can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceCustomBorderColorFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_custom_border_color
///typedef struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           customBorderColors;
///    VkBool32           customBorderColorWithoutFormat;
///} VkPhysicalDeviceCustomBorderColorFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`custom_border_colors`] indicates that the implementation supports providing a `borderColor`
///   value with one of the following values at sampler creation time:  -
///   `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`  - `VK_BORDER_COLOR_INT_CUSTOM_EXT`
/// - [`custom_border_color_without_format`] indicates that explicit formats are not required for
///   custom border colors and the value of the `format` member of the
///   [`SamplerCustomBorderColorCreateInfoEXT`] structure  **may**  be `VK_FORMAT_UNDEFINED`. If
///   this feature bit is not set, applications  **must**  provide the [`Format`] of the image
///   view(s) being sampled by this sampler in the `format` member of the
///   [`SamplerCustomBorderColorCreateInfoEXT`] structure.
///If the [`PhysicalDeviceCustomBorderColorFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceCustomBorderColorFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT`
///# Related
/// - [`VK_EXT_custom_border_color`]
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
#[doc(alias = "VkPhysicalDeviceCustomBorderColorFeaturesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`custom_border_colors`] indicates that
    ///the implementation supports providing a `borderColor` value with one
    ///of the following values at sampler creation time:
    /// - `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`
    /// - `VK_BORDER_COLOR_INT_CUSTOM_EXT`
    pub custom_border_colors: Bool32,
    ///[`custom_border_color_without_format`] indicates that explicit formats are
    ///not required for custom border colors and the value of the `format`
    ///member of the [`SamplerCustomBorderColorCreateInfoEXT`] structure
    /// **may**  be `VK_FORMAT_UNDEFINED`.
    ///If this feature bit is not set, applications  **must**  provide the
    ///[`Format`] of the image view(s) being sampled by this sampler in the
    ///`format` member of the [`SamplerCustomBorderColorCreateInfoEXT`]
    ///structure.
    pub custom_border_color_without_format: Bool32,
}
impl<'lt> Default for PhysicalDeviceCustomBorderColorFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            custom_border_colors: 0,
            custom_border_color_without_format: 0,
        }
    }
}
impl<'lt> PhysicalDeviceCustomBorderColorFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::custom_border_colors`]
    pub fn custom_border_colors_raw(&self) -> Bool32 {
        self.custom_border_colors
    }
    ///Gets the raw value of [`Self::custom_border_color_without_format`]
    pub fn custom_border_color_without_format_raw(&self) -> Bool32 {
        self.custom_border_color_without_format
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::custom_border_colors`]
    pub fn set_custom_border_colors_raw(mut self, value: Bool32) -> Self {
        self.custom_border_colors = value;
        self
    }
    ///Sets the raw value of [`Self::custom_border_color_without_format`]
    pub fn set_custom_border_color_without_format_raw(mut self, value: Bool32) -> Self {
        self.custom_border_color_without_format = value;
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
    ///Gets the value of [`Self::custom_border_colors`]
    pub fn custom_border_colors(&self) -> bool {
        unsafe { std::mem::transmute(self.custom_border_colors as u8) }
    }
    ///Gets the value of [`Self::custom_border_color_without_format`]
    pub fn custom_border_color_without_format(&self) -> bool {
        unsafe { std::mem::transmute(self.custom_border_color_without_format as u8) }
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
    ///Gets a mutable reference to the value of [`Self::custom_border_colors`]
    pub fn custom_border_colors_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.custom_border_colors as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.custom_border_colors as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::custom_border_color_without_format`]
    pub fn custom_border_color_without_format_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.custom_border_color_without_format as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.custom_border_color_without_format as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::custom_border_colors`]
    pub fn set_custom_border_colors(mut self, value: bool) -> Self {
        self.custom_border_colors = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::custom_border_color_without_format`]
    pub fn set_custom_border_color_without_format(mut self, value: bool) -> Self {
        self.custom_border_color_without_format = value as u8 as u32;
        self
    }
}
