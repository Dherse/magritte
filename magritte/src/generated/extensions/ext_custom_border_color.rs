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
///In addition to the predefined border color values, applications **can** provide
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
///Valid Usage
/// -    If provided [`format`] is not `VK_FORMAT_UNDEFINED` then the [`SamplerCreateInfo::border_color`] type **must** match the sampled type of the provided [`format`], as shown in the *SPIR-V Sampled Type* column of the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-numericformat) table
/// - If the [customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat)
///   feature is not enabled then [`format`]**must** not be `VK_FORMAT_UNDEFINED`
/// - If the sampler is used to sample an image view of `VK_FORMAT_B4G4R4A4_UNORM_PACK16`,
///   `VK_FORMAT_B5G6R5_UNORM_PACK16`, or `VK_FORMAT_B5G5R5A1_UNORM_PACK16` format then
///   [`format`]**must** not be `VK_FORMAT_UNDEFINED`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT`
/// - [`format`]**must** be a valid [`Format`] value
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`custom_border_color`] is a [`ClearColorValue`] representing the
    ///desired custom sampler border color.
    custom_border_color: ClearColorValue,
    ///[`format`] is a [`Format`] representing the format of the sampled
    ///image view(s).
    ///This field may be `VK_FORMAT_UNDEFINED` if the
    ///[customBorderColorWithoutFormat](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-customBorderColorWithoutFormat)
    ///feature is enabled.
    format: Format,
}
impl<'lt> Default for SamplerCustomBorderColorCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            custom_border_color: Default::default(),
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
    ///Sets the raw value of [`Self::custom_border_color`]
    pub fn set_custom_border_color(&mut self, value: crate::vulkan1_0::ClearColorValue) -> &mut Self {
        self.custom_border_color = value;
        self
    }
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
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
///   border colors which **can** simultaneously exist on a device.
///# Description
///If the [`PhysicalDeviceCustomBorderColorPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    s_type: StructureType,
    ///No documentation found
    p_next: *mut BaseOutStructure<'lt>,
    ///[`max_custom_border_color_samplers`] indicates the maximum number of
    ///samplers with custom border colors which **can** simultaneously exist on a
    ///device.
    max_custom_border_color_samplers: u32,
}
impl<'lt> Default for PhysicalDeviceCustomBorderColorPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            max_custom_border_color_samplers: 0,
        }
    }
}
impl<'lt> PhysicalDeviceCustomBorderColorPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::max_custom_border_color_samplers`]
    pub fn max_custom_border_color_samplers_raw(&self) -> u32 {
        self.max_custom_border_color_samplers
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::max_custom_border_color_samplers`]
    pub fn set_max_custom_border_color_samplers_raw(&mut self, value: u32) -> &mut Self {
        self.max_custom_border_color_samplers = value;
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
    ///Sets the raw value of [`Self::max_custom_border_color_samplers`]
    pub fn set_max_custom_border_color_samplers(&mut self, value: u32) -> &mut Self {
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
///   [`SamplerCustomBorderColorCreateInfoEXT`] structure **may** be `VK_FORMAT_UNDEFINED`. If this
///   feature bit is not set, applications **must** provide the [`Format`] of the image view(s)
///   being sampled by this sampler in the `format` member of the
///   [`SamplerCustomBorderColorCreateInfoEXT`] structure.
///If the [`PhysicalDeviceCustomBorderColorFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceCustomBorderColorFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT`
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`custom_border_colors`] indicates that
    ///the implementation supports providing a `borderColor` value with one
    ///of the following values at sampler creation time:
    /// - `VK_BORDER_COLOR_FLOAT_CUSTOM_EXT`
    /// - `VK_BORDER_COLOR_INT_CUSTOM_EXT`
    custom_border_colors: Bool32,
    ///[`custom_border_color_without_format`] indicates that explicit formats are
    ///not required for custom border colors and the value of the `format`
    ///member of the [`SamplerCustomBorderColorCreateInfoEXT`] structure
    ///**may** be `VK_FORMAT_UNDEFINED`.
    ///If this feature bit is not set, applications **must** provide the
    ///[`Format`] of the image view(s) being sampled by this sampler in the
    ///`format` member of the [`SamplerCustomBorderColorCreateInfoEXT`]
    ///structure.
    custom_border_color_without_format: Bool32,
}
impl<'lt> Default for PhysicalDeviceCustomBorderColorFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            custom_border_colors: 0,
            custom_border_color_without_format: 0,
        }
    }
}
impl<'lt> PhysicalDeviceCustomBorderColorFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
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
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::custom_border_colors`]
    pub fn set_custom_border_colors_raw(&mut self, value: Bool32) -> &mut Self {
        self.custom_border_colors = value;
        self
    }
    ///Sets the raw value of [`Self::custom_border_color_without_format`]
    pub fn set_custom_border_color_without_format_raw(&mut self, value: Bool32) -> &mut Self {
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
    ///Sets the raw value of [`Self::custom_border_colors`]
    pub fn set_custom_border_colors(&mut self, value: bool) -> &mut Self {
        self.custom_border_colors = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::custom_border_color_without_format`]
    pub fn set_custom_border_color_without_format(&mut self, value: bool) -> &mut Self {
        self.custom_border_color_without_format = value as u8 as u32;
        self
    }
}
