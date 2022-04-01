use crate::{
    core::UUID_SIZE,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, Buffer, BufferCreateFlags, BufferUsageFlags, ComponentMapping,
        DescriptorSetLayout, DescriptorType, DeviceMemory, DeviceQueueCreateFlags, DeviceSize, Filter, Format,
        FormatProperties, Image, ImageAspectFlagBits, ImageAspectFlags, ImageCreateFlags, ImageFormatProperties,
        ImageTiling, ImageType, ImageUsageFlags, MemoryRequirements, PhysicalDevice, PhysicalDeviceFeatures,
        PhysicalDeviceMemoryProperties, PhysicalDeviceProperties, PipelineBindPoint, PipelineLayout,
        QueueFamilyProperties, Rect2D, SampleCountFlagBits, ShaderStageFlags, SparseImageFormatProperties,
        SparseImageMemoryRequirements, StructureType,
    },
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
};
///[VK_LUID_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LUID_SIZE.html) - Length of a locally unique device identifier
///# C Specifications
///[`LUID_SIZE`] is the length in `uint8_t` values of an array
///containing a locally unique device identifier, as returned in
///[`PhysicalDeviceIdProperties`]::deviceLUID.
///```c
///#define VK_LUID_SIZE                      8U
///```
///or the equivalent
///```c
///#define VK_LUID_SIZE_KHR                  VK_LUID_SIZE
///```
///# Related
/// - [`VK_KHR_external_fence_capabilities`]
/// - [`VK_KHR_external_memory_capabilities`]
/// - [`VK_KHR_external_semaphore_capabilities`]
/// - [`crate::vulkan1_1`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_LUID_SIZE")]
pub const LUID_SIZE: u32 = 8;
///[VK_QUEUE_FAMILY_EXTERNAL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_EXTERNAL.html) - External queue family index sentinel
///# C Specifications
///The special queue family index [`QUEUE_FAMILY_EXTERNAL`] represents any
///queue external to the resource’s current Vulkan instance, as long as the
///queue uses the same underlying
///device group or
///physical device, and the same driver version as the resource’s
///[`Device`], as indicated by
///[`PhysicalDeviceIdProperties::device_uuid`] and
///[`PhysicalDeviceIdProperties::driver_uuid`].
///```c
///#define VK_QUEUE_FAMILY_EXTERNAL          (~1U)
///```
///or the equivalent
///```c
///#define VK_QUEUE_FAMILY_EXTERNAL_KHR      VK_QUEUE_FAMILY_EXTERNAL
///```
///# Related
/// - [`VK_KHR_external_memory`]
/// - [`crate::vulkan1_1`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL")]
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;
///[VK_MAX_DEVICE_GROUP_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DEVICE_GROUP_SIZE.html) - Length of a physical device handle array
///# C Specifications
///[`MAX_DEVICE_GROUP_SIZE`] is the length of an array containing
///[`PhysicalDevice`] handle values representing all physical devices in a
///group, as returned in
///[`PhysicalDeviceGroupProperties`]::physicalDevices.
///```c
///#define VK_MAX_DEVICE_GROUP_SIZE          32U
///```
///or the equivalent
///```c
///#define VK_MAX_DEVICE_GROUP_SIZE_KHR      VK_MAX_DEVICE_GROUP_SIZE
///```
///# Related
/// - [`VK_KHR_device_group_creation`]
/// - [`crate::vulkan1_1`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE")]
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
///[VkDescriptorUpdateTemplateType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateType.html) - Indicates the valid usage of the descriptor update template
///# C Specifications
///The descriptor update template type is determined by the
///[`DescriptorUpdateTemplateCreateInfo::template_type`] property,
///which takes the following values:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkDescriptorUpdateTemplateType {
///    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET = 0,
///  // Provided by VK_VERSION_1_1 with VK_KHR_push_descriptor, VK_KHR_descriptor_update_template
/// with VK_KHR_push_descriptor
///    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR = 1,
///  // Provided by VK_KHR_descriptor_update_template
///    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR =
/// VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET,
///} VkDescriptorUpdateTemplateType;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_descriptor_update_template
///typedef VkDescriptorUpdateTemplateType VkDescriptorUpdateTemplateTypeKHR;
///```
///# Description
/// - [`DescriptorSet`] specifies that the descriptor update template will be used for descriptor
///   set updates only.
/// - [`PushDescriptorsKhr`] specifies that the descriptor update template will be used for push
///   descriptor updates only.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`DescriptorUpdateTemplateCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDescriptorUpdateTemplateType")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum DescriptorUpdateTemplateType {
    ///[`DescriptorSet`] specifies that
    ///the descriptor update template will be used for descriptor set updates
    ///only.
    DescriptorSet = 0,
    ///[`PushDescriptorsKhr`] specifies
    ///that the descriptor update template will be used for push descriptor
    ///updates only.
    ///
    ///Provided by [`crate::extensions::khr_descriptor_update_template`]
    PushDescriptorsKhr = 1,
}
impl const Default for DescriptorUpdateTemplateType {
    fn default() -> Self {
        Self::DescriptorSet
    }
}
impl DescriptorUpdateTemplateType {
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
///[VkPointClippingBehavior](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPointClippingBehavior.html) - Enum specifying the point clipping behavior
///# C Specifications
///Possible values of
///[`PhysicalDevicePointClippingProperties::point_clipping_behavior`],
///specifying clipping behavior of a point primitive whose vertex lies outside
///the clip volume, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkPointClippingBehavior {
///    VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES = 0,
///    VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY = 1,
///  // Provided by VK_KHR_maintenance2
///    VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR = VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES,
///  // Provided by VK_KHR_maintenance2
///    VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR =
/// VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY,
///} VkPointClippingBehavior;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance2
///typedef VkPointClippingBehavior VkPointClippingBehaviorKHR;
///```
///# Description
/// - [`AllClipPlanes`] specifies that the primitive is discarded if the vertex lies outside any
///   clip plane, including the planes bounding the view volume.
/// - [`UserClipPlanesOnly`] specifies that the primitive is discarded only if the vertex lies
///   outside any user clip plane.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`PhysicalDevicePointClippingProperties`]
/// - [`PhysicalDeviceVulkan11Properties`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPointClippingBehavior")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum PointClippingBehavior {
    ///[`AllClipPlanes`] specifies that the
    ///primitive is discarded if the vertex lies outside any clip plane,
    ///including the planes bounding the view volume.
    AllClipPlanes = 0,
    ///[`UserClipPlanesOnly`] specifies that
    ///the primitive is discarded only if the vertex lies outside any user clip
    ///plane.
    UserClipPlanesOnly = 1,
}
impl const Default for PointClippingBehavior {
    fn default() -> Self {
        Self::AllClipPlanes
    }
}
impl PointClippingBehavior {
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
///[VkTessellationDomainOrigin](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTessellationDomainOrigin.html) - Enum describing tessellation domain origin
///# C Specifications
///The possible tessellation domain origins are specified by the
///[`TessellationDomainOrigin`] enumeration:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkTessellationDomainOrigin {
///    VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT = 0,
///    VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT = 1,
///  // Provided by VK_KHR_maintenance2
///    VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR = VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT,
///  // Provided by VK_KHR_maintenance2
///    VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR = VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT,
///} VkTessellationDomainOrigin;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance2
///typedef VkTessellationDomainOrigin VkTessellationDomainOriginKHR;
///```
///# Description
/// - [`UpperLeft`] specifies that the origin of the domain space is in the upper left corner, as shown in figure [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul).
/// - [`LowerLeft`] specifies that the origin of the domain space is in the lower left corner, as shown in figure [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll).
///This enum affects how the `VertexOrderCw` and `VertexOrderCcw`
///tessellation execution modes are interpreted, since the winding is defined
///relative to the orientation of the domain.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`PipelineTessellationDomainOriginStateCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkTessellationDomainOrigin")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum TessellationDomainOrigin {
    ///[`UpperLeft`] specifies that the origin
    ///of the domain space is in the upper left corner, as shown in figure
    ///[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul).
    UpperLeft = 0,
    ///[`LowerLeft`] specifies that the origin
    ///of the domain space is in the lower left corner, as shown in figure
    ///[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll).
    LowerLeft = 1,
}
impl const Default for TessellationDomainOrigin {
    fn default() -> Self {
        Self::UpperLeft
    }
}
impl TessellationDomainOrigin {
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
///[VkSamplerYcbcrModelConversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrModelConversion.html) - Color model component of a color space
///# C Specifications
///[`SamplerYcbcrModelConversion`] defines the conversion from the source
///color model to the shader color model.
///Possible values are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkSamplerYcbcrModelConversion {
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY = 0,
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY = 1,
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709 = 2,
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601 = 3,
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020 = 4,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR =
/// VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR =
/// VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR =
/// VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR =
/// VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR =
/// VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020,
///} VkSamplerYcbcrModelConversion;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkSamplerYcbcrModelConversion VkSamplerYcbcrModelConversionKHR;
///```
///# Description
/// - [`RgbIdentity`] specifies that the input values to the conversion are unmodified.
/// - [`YcbcrIdentity`] specifies no model conversion but the inputs are range expanded as for
///   Y′C<sub>B</sub>C<sub>R</sub>.
/// - [`Ycbcr709`] specifies the color model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.709 and described in the “BT.709 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
/// - [`Ycbcr601`] specifies the color model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.601 and described in the “BT.601 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
/// - [`Ycbcr2020`] specifies the color model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′
///   defined in BT.2020 and described in the “BT.2020 Y′C<sub>B</sub>C<sub>R</sub> conversion” section
///   of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
///In the `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_*` color models, for the
///input to the sampler Y′C<sub>B</sub>C<sub>R</sub> range expansion and model conversion:
/// - the Y (Y′ luma) component corresponds to the G component of an RGB image.
/// - the CB (C<sub>B</sub> or “U” blue color difference) component corresponds to the B component
///   of an RGB image.
/// - the CR (C<sub>R</sub> or “V” red color difference) component corresponds to the R component of
///   an RGB image.
/// - the alpha component, if present, is not modified by color model conversion.
///These rules reflect the mapping of components after the component swizzle
///operation (controlled by
///[`SamplerYcbcrConversionCreateInfo::components`]).
///# Related
/// - [`crate::vulkan1_1`]
/// - [`AndroidHardwareBufferFormatProperties2ANDROID`]
/// - [`AndroidHardwareBufferFormatPropertiesANDROID`]
/// - [`BufferCollectionPropertiesFUCHSIA`]
/// - [`SamplerYcbcrConversionCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSamplerYcbcrModelConversion")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum SamplerYcbcrModelConversion {
    ///[`RgbIdentity`] specifies that the
    ///input values to the conversion are unmodified.
    RgbIdentity = 0,
    ///[`YcbcrIdentity`] specifies no
    ///model conversion but the inputs are range expanded as for Y′C<sub>B</sub>C<sub>R</sub>.
    YcbcrIdentity = 1,
    ///[`Ycbcr709`] specifies the color
    ///model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.709 and
    ///described in the “BT.709 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the
    ///[Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
    Ycbcr709 = 2,
    ///[`Ycbcr601`] specifies the color
    ///model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.601 and
    ///described in the “BT.601 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the
    ///[Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
    Ycbcr601 = 3,
    ///[`Ycbcr2020`] specifies the color
    ///model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.2020 and
    ///described in the “BT.2020 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the
    ///[Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
    Ycbcr2020 = 4,
}
impl const Default for SamplerYcbcrModelConversion {
    fn default() -> Self {
        Self::RgbIdentity
    }
}
impl SamplerYcbcrModelConversion {
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
///[VkSamplerYcbcrRange](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrRange.html) - Range of encoded values in a color space
///# C Specifications
///The [`SamplerYcbcrRange`] enum describes whether color components are
///encoded using the full range of numerical values or whether values are
///reserved for headroom and foot room.
///[`SamplerYcbcrRange`] is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkSamplerYcbcrRange {
///    VK_SAMPLER_YCBCR_RANGE_ITU_FULL = 0,
///    VK_SAMPLER_YCBCR_RANGE_ITU_NARROW = 1,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR = VK_SAMPLER_YCBCR_RANGE_ITU_FULL,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR = VK_SAMPLER_YCBCR_RANGE_ITU_NARROW,
///} VkSamplerYcbcrRange;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkSamplerYcbcrRange VkSamplerYcbcrRangeKHR;
///```
///# Description
/// - [`ItuFull`] specifies that the full range of the encoded values are valid and interpreted
///   according to the ITU “full range” quantization rules.
/// - [`ItuNarrow`] specifies that headroom and foot room are reserved in the numerical range of
///   encoded values, and the remaining values are expanded according to the ITU “narrow range”
///   quantization rules.
///The formulae for these conversions is described in the
///[Sampler Y′C<sub>B</sub>C<sub>R</sub> Range
///Expansion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-sampler-YCbCr-conversion-rangeexpand) section of the [Image Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures) chapter.No range modification takes place if `ycbcrModel` is
///`VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY`; the `ycbcrRange`
///field of [`SamplerYcbcrConversionCreateInfo`] is ignored in this case.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`AndroidHardwareBufferFormatProperties2ANDROID`]
/// - [`AndroidHardwareBufferFormatPropertiesANDROID`]
/// - [`BufferCollectionPropertiesFUCHSIA`]
/// - [`SamplerYcbcrConversionCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSamplerYcbcrRange")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum SamplerYcbcrRange {
    ///[`ItuFull`] specifies that the full range of
    ///the encoded values are valid and interpreted according to the ITU “full
    ///range” quantization rules.
    ItuFull = 0,
    ///[`ItuNarrow`] specifies that headroom and foot
    ///room are reserved in the numerical range of encoded values, and the
    ///remaining values are expanded according to the ITU “narrow range”
    ///quantization rules.
    ItuNarrow = 1,
}
impl const Default for SamplerYcbcrRange {
    fn default() -> Self {
        Self::ItuFull
    }
}
impl SamplerYcbcrRange {
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
///[VkChromaLocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkChromaLocation.html) - Position of downsampled chroma samples
///# C Specifications
///The [`ChromaLocation`] enum defines the location of downsampled chroma
///component samples relative to the luma samples, and is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkChromaLocation {
///    VK_CHROMA_LOCATION_COSITED_EVEN = 0,
///    VK_CHROMA_LOCATION_MIDPOINT = 1,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_CHROMA_LOCATION_COSITED_EVEN_KHR = VK_CHROMA_LOCATION_COSITED_EVEN,
///  // Provided by VK_KHR_sampler_ycbcr_conversion
///    VK_CHROMA_LOCATION_MIDPOINT_KHR = VK_CHROMA_LOCATION_MIDPOINT,
///} VkChromaLocation;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkChromaLocation VkChromaLocationKHR;
///```
///# Description
/// - [`CositedEven`] specifies that downsampled chroma samples are aligned with luma samples with
///   even coordinates.
/// - [`Midpoint`] specifies that downsampled chroma samples are located half way between each even
///   luma sample and the nearest higher odd luma sample.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`AndroidHardwareBufferFormatProperties2ANDROID`]
/// - [`AndroidHardwareBufferFormatPropertiesANDROID`]
/// - [`BufferCollectionPropertiesFUCHSIA`]
/// - [`SamplerYcbcrConversionCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkChromaLocation")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum ChromaLocation {
    ///[`CositedEven`] specifies that downsampled chroma
    ///samples are aligned with luma samples with even coordinates.
    CositedEven = 0,
    ///[`Midpoint`] specifies that downsampled chroma
    ///samples are located half way between each even luma sample and the
    ///nearest higher odd luma sample.
    Midpoint = 1,
}
impl const Default for ChromaLocation {
    fn default() -> Self {
        Self::CositedEven
    }
}
impl ChromaLocation {
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
///[VkDeviceQueueCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueCreateFlagBits.html) - Bitmask specifying behavior of the queue
///# C Specifications
///Bits which  **can**  be set in [`DeviceQueueCreateInfo::flags`],
///specifying usage behavior of a queue, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkDeviceQueueCreateFlagBits {
///  // Provided by VK_VERSION_1_1
///    VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT = 0x00000001,
///} VkDeviceQueueCreateFlagBits;
///```
///# Description
/// - [`Protected`] specifies that the device queue is a protected-capable queue.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`DeviceQueueCreateFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceQueueCreateFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum DeviceQueueCreateFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`Protected`] specifies that the device
    ///queue is a protected-capable queue.
    Protected = 1,
}
impl const Default for DeviceQueueCreateFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl DeviceQueueCreateFlagBits {
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
///[VkSubgroupFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlagBits.html) - Bitmask describing what group operations are supported with subgroup scope
///# C Specifications
///Bits which  **can**  be set in
///[`PhysicalDeviceSubgroupProperties::supported_operations`]
///and
///[`PhysicalDeviceVulkan11Properties::subgroup_supported_operations`]
///to specify supported [group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) with
///[subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup) are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkSubgroupFeatureFlagBits {
///    VK_SUBGROUP_FEATURE_BASIC_BIT = 0x00000001,
///    VK_SUBGROUP_FEATURE_VOTE_BIT = 0x00000002,
///    VK_SUBGROUP_FEATURE_ARITHMETIC_BIT = 0x00000004,
///    VK_SUBGROUP_FEATURE_BALLOT_BIT = 0x00000008,
///    VK_SUBGROUP_FEATURE_SHUFFLE_BIT = 0x00000010,
///    VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT = 0x00000020,
///    VK_SUBGROUP_FEATURE_CLUSTERED_BIT = 0x00000040,
///    VK_SUBGROUP_FEATURE_QUAD_BIT = 0x00000080,
///  // Provided by VK_NV_shader_subgroup_partitioned
///    VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV = 0x00000100,
///} VkSubgroupFeatureFlagBits;
///```
///# Description
/// - [`SubgroupFeatureBasic`] specifies the device will accept SPIR-V shader modules containing the
///   `GroupNonUniform` capability.
/// - [`SubgroupFeatureVote`] specifies the device will accept SPIR-V shader modules containing the
///   `GroupNonUniformVote` capability.
/// - [`SubgroupFeatureArithmetic`] specifies the device will accept SPIR-V shader modules
///   containing the `GroupNonUniformArithmetic` capability.
/// - [`SubgroupFeatureBallot`] specifies the device will accept SPIR-V shader modules containing
///   the `GroupNonUniformBallot` capability.
/// - [`SubgroupFeatureShuffle`] specifies the device will accept SPIR-V shader modules containing
///   the `GroupNonUniformShuffle` capability.
/// - [`SubgroupFeatureShuffleRelative`] specifies the device will accept SPIR-V shader modules
///   containing the `GroupNonUniformShuffleRelative` capability.
/// - [`SubgroupFeatureClustered`] specifies the device will accept SPIR-V shader modules containing
///   the `GroupNonUniformClustered` capability.
/// - [`SubgroupFeatureQuad`] specifies the device will accept SPIR-V shader modules containing the
///   `GroupNonUniformQuad` capability.
/// - [`PartitionedNv`] specifies the device will accept SPIR-V shader modules containing the
///   `GroupNonUniformPartitionedNV` capability.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`SubgroupFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSubgroupFeatureFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum SubgroupFeatureFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`SubgroupFeatureBasic`]
    ///specifies the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniform` capability.
    SubgroupFeatureBasic = 1,
    ///[`SubgroupFeatureVote`] specifies
    ///the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformVote` capability.
    SubgroupFeatureVote = 2,
    ///[`SubgroupFeatureArithmetic`] specifies the device will
    ///accept SPIR-V shader modules containing the
    ///`GroupNonUniformArithmetic` capability.
    SubgroupFeatureArithmetic = 4,
    ///[`SubgroupFeatureBallot`]
    ///specifies the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformBallot` capability.
    SubgroupFeatureBallot = 8,
    ///[`SubgroupFeatureShuffle`]
    ///specifies the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformShuffle` capability.
    SubgroupFeatureShuffle = 16,
    ///[`SubgroupFeatureShuffleRelative`] specifies the device will
    ///accept SPIR-V shader modules containing the
    ///`GroupNonUniformShuffleRelative` capability.
    SubgroupFeatureShuffleRelative = 32,
    ///[`SubgroupFeatureClustered`]
    ///specifies the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformClustered` capability.
    SubgroupFeatureClustered = 64,
    ///[`SubgroupFeatureQuad`] specifies
    ///the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformQuad` capability.
    SubgroupFeatureQuad = 128,
    ///[`PartitionedNv`] specifies the device will
    ///accept SPIR-V shader modules containing the
    ///`GroupNonUniformPartitionedNV` capability.
    ///
    ///Provided by [`crate::extensions::nv_shader_subgroup_partitioned`]
    PartitionedNv = 256,
}
impl const Default for SubgroupFeatureFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl SubgroupFeatureFlagBits {
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
///[VkExternalMemoryHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html) - Bit specifying external memory handle types
///# C Specifications
///Possible values of
///[`PhysicalDeviceExternalImageFormatInfo::handle_type`], specifying
///an external memory handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalMemoryHandleTypeFlagBits {
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT = 0x00000001,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT = 0x00000002,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 0x00000004,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT = 0x00000008,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT = 0x00000010,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT = 0x00000020,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT = 0x00000040,
///  // Provided by VK_EXT_external_memory_dma_buf
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT = 0x00000200,
///  // Provided by VK_ANDROID_external_memory_android_hardware_buffer
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID = 0x00000400,
///  // Provided by VK_EXT_external_memory_host
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT = 0x00000080,
///  // Provided by VK_EXT_external_memory_host
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT = 0x00000100,
///  // Provided by VK_FUCHSIA_external_memory
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA = 0x00000800,
///  // Provided by VK_NV_external_memory_rdma
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV = 0x00001000,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT,
///} VkExternalMemoryHandleTypeFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory_capabilities
///typedef VkExternalMemoryHandleTypeFlagBits VkExternalMemoryHandleTypeFlagBitsKHR;
///```
///# Description
/// - [`ExternalMemoryHandleTypeOpaqueFd`] specifies a POSIX file descriptor handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible
///   with the POSIX system calls `dup`, `dup2`, `close`, and the non-standard system call `dup3`.
///   Additionally, it  **must**  be transportable over a socket using an `SCM_RIGHTS` control
///   message. It owns a reference to the underlying memory resource represented by its Vulkan
///   memory object.
/// - [`ExternalMemoryHandleTypeOpaqueWin32`] specifies an NT handle that has only limited valid
///   usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the
///   functions `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
///   and `SetHandleInformation`. It owns a reference to the underlying memory resource represented
///   by its Vulkan memory object.
/// - [`ExternalMemoryHandleTypeOpaqueWin32Kmt`] specifies a global share handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It is not compatible with any
///   native APIs. It does not own a reference to the underlying memory resource represented by its
///   Vulkan memory object, and will therefore become invalid when all Vulkan memory objects
///   associated with it are destroyed.
/// - [`ExternalMemoryHandleTypeD3D11Texture`] specifies an NT handle returned by
///   `IDXGIResource1`::`CreateSharedHandle` referring to a Direct3D 10 or 11 texture resource. It
///   owns a reference to the memory used by the Direct3D resource.
/// - [`ExternalMemoryHandleTypeD3D11TextureKmt`] specifies a global share handle returned by
///   `IDXGIResource`::`GetSharedHandle` referring to a Direct3D 10 or 11 texture resource. It does
///   not own a reference to the underlying Direct3D resource, and will therefore become invalid
///   when all Vulkan memory objects and Direct3D resources associated with it are destroyed.
/// - [`ExternalMemoryHandleTypeD3D12Heap`] specifies an NT handle returned by
///   `ID3D12Device`::`CreateSharedHandle` referring to a Direct3D 12 heap resource. It owns a
///   reference to the resources used by the Direct3D heap.
/// - [`ExternalMemoryHandleTypeD3D12Resource`] specifies an NT handle returned by
///   `ID3D12Device`::`CreateSharedHandle` referring to a Direct3D 12 committed resource. It owns a
///   reference to the memory used by the Direct3D resource.
/// - [`HostAllocationExt`] specifies a host pointer returned by a host memory allocation command.
///   It does not own a reference to the underlying memory resource, and will therefore become
///   invalid if the host memory is freed.
/// - [`HostMappedForeignMemoryExt`] specifies a host pointer to *host mapped foreign memory*. It
///   does not own a reference to the underlying memory resource, and will therefore become invalid
///   if the foreign memory is unmapped or otherwise becomes no longer available.
/// - [`DmaBufExt`] is a file descriptor for a Linux dma_buf. It owns a reference to the underlying
///   memory resource represented by its Vulkan memory object.
/// - [`AndroidHardwareBufferAndroid`] specifies an [`AHardwareBuffer`] object defined by the Android NDK. See [Android Hardware Buffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer) for more details of this handle type.
/// - [`ZirconVmoFuchsia`] is a Zircon handle to a virtual memory object.
/// - [`RdmaAddressNv`] is a handle to an allocation accessible by remote devices. It owns a
///   reference to the underlying memory resource represented by its Vulkan memory object.
///Some external memory handle types can only be shared within the same
///underlying physical device and/or the same driver version, as defined in the
///following table:
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryHandleTypeFlags`]
/// - [`ImportMemoryFdInfoKHR`]
/// - [`ImportMemoryHostPointerInfoEXT`]
/// - [`ImportMemoryWin32HandleInfoKHR`]
/// - [`ImportMemoryZirconHandleInfoFUCHSIA`]
/// - [`MemoryGetFdInfoKHR`]
/// - [`MemoryGetRemoteAddressInfoNV`]
/// - [`MemoryGetWin32HandleInfoKHR`]
/// - [`MemoryGetZirconHandleInfoFUCHSIA`]
/// - [`PhysicalDeviceExternalBufferInfo`]
/// - [`PhysicalDeviceExternalImageFormatInfo`]
/// - [`GetMemoryFdPropertiesKHR`]
/// - [`GetMemoryHostPointerPropertiesEXT`]
/// - [`GetMemoryWin32HandlePropertiesKHR`]
/// - [`GetMemoryZirconHandlePropertiesFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryHandleTypeFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum ExternalMemoryHandleTypeFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`ExternalMemoryHandleTypeOpaqueFd`] specifies a POSIX
    ///file descriptor handle that has only limited valid usage outside of
    ///Vulkan and other compatible APIs.
    ///It  **must**  be compatible with the POSIX system calls `dup`, `dup2`,
    ///`close`, and the non-standard system call `dup3`.
    ///Additionally, it  **must**  be transportable over a socket using an
    ///`SCM_RIGHTS` control message.
    ///It owns a reference to the underlying memory resource represented by its
    ///Vulkan memory object.
    ExternalMemoryHandleTypeOpaqueFd = 1,
    ///[`ExternalMemoryHandleTypeOpaqueWin32`] specifies an NT
    ///handle that has only limited valid usage outside of Vulkan and other
    ///compatible APIs.
    ///It  **must**  be compatible with the functions `DuplicateHandle`,
    ///`CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
    ///and `SetHandleInformation`.
    ///It owns a reference to the underlying memory resource represented by its
    ///Vulkan memory object.
    ExternalMemoryHandleTypeOpaqueWin32 = 2,
    ///[`ExternalMemoryHandleTypeOpaqueWin32Kmt`] specifies a
    ///global share handle that has only limited valid usage outside of Vulkan
    ///and other compatible APIs.
    ///It is not compatible with any native APIs.
    ///It does not own a reference to the underlying memory resource
    ///represented by its Vulkan memory object, and will therefore become
    ///invalid when all Vulkan memory objects associated with it are destroyed.
    ExternalMemoryHandleTypeOpaqueWin32Kmt = 4,
    ///[`ExternalMemoryHandleTypeD3D11Texture`] specifies an NT
    ///handle returned by `IDXGIResource1`::`CreateSharedHandle`
    ///referring to a Direct3D 10 or 11 texture resource.
    ///It owns a reference to the memory used by the Direct3D resource.
    ExternalMemoryHandleTypeD3D11Texture = 8,
    ///[`ExternalMemoryHandleTypeD3D11TextureKmt`] specifies a
    ///global share handle returned by `IDXGIResource`::`GetSharedHandle`
    ///referring to a Direct3D 10 or 11 texture resource.
    ///It does not own a reference to the underlying Direct3D resource, and
    ///will therefore become invalid when all Vulkan memory objects and
    ///Direct3D resources associated with it are destroyed.
    ExternalMemoryHandleTypeD3D11TextureKmt = 16,
    ///[`ExternalMemoryHandleTypeD3D12Heap`] specifies an NT
    ///handle returned by `ID3D12Device`::`CreateSharedHandle` referring
    ///to a Direct3D 12 heap resource.
    ///It owns a reference to the resources used by the Direct3D heap.
    ExternalMemoryHandleTypeD3D12Heap = 32,
    ///[`ExternalMemoryHandleTypeD3D12Resource`] specifies an NT
    ///handle returned by `ID3D12Device`::`CreateSharedHandle` referring
    ///to a Direct3D 12 committed resource.
    ///It owns a reference to the memory used by the Direct3D resource.
    ExternalMemoryHandleTypeD3D12Resource = 64,
    ///[`DmaBufExt`] is a file
    ///descriptor for a Linux dma_buf.
    ///It owns a reference to the underlying memory resource represented by its
    ///Vulkan memory object.
    ///
    ///Provided by [`crate::extensions::ext_external_memory_dma_buf`]
    DmaBufExt = 512,
    ///[`AndroidHardwareBufferAndroid`]
    ///specifies an [`AHardwareBuffer`] object defined by the Android NDK.
    ///See [Android Hardware Buffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer)
    ///for more details of this handle type.
    ///
    ///Provided by [`crate::extensions::android_external_memory_android_hardware_buffer`]
    AndroidHardwareBufferAndroid = 1024,
    ///[`HostAllocationExt`] specifies a
    ///host pointer returned by a host memory allocation command.
    ///It does not own a reference to the underlying memory resource, and will
    ///therefore become invalid if the host memory is freed.
    ///
    ///Provided by [`crate::extensions::ext_external_memory_host`]
    HostAllocationExt = 128,
    ///[`HostMappedForeignMemoryExt`]
    ///specifies a host pointer to *host mapped foreign memory*.
    ///It does not own a reference to the underlying memory resource, and will
    ///therefore become invalid if the foreign memory is unmapped or otherwise
    ///becomes no longer available.
    ///
    ///Provided by [`crate::extensions::ext_external_memory_host`]
    HostMappedForeignMemoryExt = 256,
    ///[`ZirconVmoFuchsia`] is a Zircon
    ///handle to a virtual memory object.
    ///
    ///Provided by [`crate::extensions::fuchsia_external_memory`]
    ZirconVmoFuchsia = 2048,
    ///[`RdmaAddressNv`] is a handle to
    ///an allocation accessible by remote devices.
    ///It owns a reference to the underlying memory resource represented by its
    ///Vulkan memory object.
    ///
    ///Provided by [`crate::extensions::nv_external_memory_rdma`]
    RdmaAddressNv = 4096,
}
impl const Default for ExternalMemoryHandleTypeFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl ExternalMemoryHandleTypeFlagBits {
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
///[VkExternalMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html) - Bitmask specifying features of an external memory handle type
///# C Specifications
///Bits which  **may**  be set in
///[`ExternalMemoryProperties::external_memory_features`], specifying
///features of an external memory handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalMemoryFeatureFlagBits {
///    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT = 0x00000001,
///    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT = 0x00000002,
///    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT = 0x00000004,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR =
/// VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR = VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR = VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT,
///} VkExternalMemoryFeatureFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory_capabilities
///typedef VkExternalMemoryFeatureFlagBits VkExternalMemoryFeatureFlagBitsKHR;
///```
///# Description
/// - [`ExternalMemoryFeatureDedicatedOnly`] specifies that images or buffers created with the
///   specified parameters and handle type  **must**  use the mechanisms defined by
///   [`MemoryDedicatedRequirements`] and [`MemoryDedicatedAllocateInfo`] to create (or import) a
///   dedicated allocation for the image or buffer.
/// - [`ExternalMemoryFeatureExportable`] specifies that handles of this type  **can**  be exported
///   from Vulkan memory objects.
/// - [`ExternalMemoryFeatureImportable`] specifies that handles of this type  **can**  be imported
///   as Vulkan memory objects.
///Because their semantics in external APIs roughly align with that of an image
///or buffer with a dedicated allocation in Vulkan, implementations are
/// **required**  to report [`ExternalMemoryFeatureDedicatedOnly`] for
///the following external handle types:
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` for images only
///Implementations  **must**  not report
///[`ExternalMemoryFeatureDedicatedOnly`] for buffers with
///external handle type
///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`.
///Implementations  **must**  not report
///[`ExternalMemoryFeatureDedicatedOnly`] for images or buffers
///with external handle type
///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`, or
///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryFeatureFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum ExternalMemoryFeatureFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`ExternalMemoryFeatureDedicatedOnly`] specifies that
    ///images or buffers created with the specified parameters and handle type
    /// **must**  use the mechanisms defined by [`MemoryDedicatedRequirements`]
    ///and [`MemoryDedicatedAllocateInfo`] to create (or import) a
    ///dedicated allocation for the image or buffer.
    ExternalMemoryFeatureDedicatedOnly = 1,
    ///[`ExternalMemoryFeatureExportable`] specifies that handles
    ///of this type  **can**  be exported from Vulkan memory objects.
    ExternalMemoryFeatureExportable = 2,
    ///[`ExternalMemoryFeatureImportable`] specifies that handles
    ///of this type  **can**  be imported as Vulkan memory objects.
    ExternalMemoryFeatureImportable = 4,
}
impl const Default for ExternalMemoryFeatureFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl ExternalMemoryFeatureFlagBits {
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
///[VkExternalSemaphoreHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html) - Bitmask of valid external semaphore handle types
///# C Specifications
///Bits which  **may**  be set in
///[`PhysicalDeviceExternalSemaphoreInfo::handle_type`], specifying an
///external semaphore handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalSemaphoreHandleTypeFlagBits {
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT = 0x00000001,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT = 0x00000002,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 0x00000004,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT = 0x00000008,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT = 0x00000010,
///  // Provided by VK_FUCHSIA_external_semaphore
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA = 0x00000080,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT,
///} VkExternalSemaphoreHandleTypeFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_semaphore_capabilities
///typedef VkExternalSemaphoreHandleTypeFlagBits VkExternalSemaphoreHandleTypeFlagBitsKHR;
///```
///# Description
/// - [`ExternalSemaphoreHandleTypeOpaqueFd`] specifies a POSIX file descriptor handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible
///   with the POSIX system calls `dup`, `dup2`, `close`, and the non-standard system call `dup3`.
///   Additionally, it  **must**  be transportable over a socket using an `SCM_RIGHTS` control
///   message. It owns a reference to the underlying synchronization primitive represented by its
///   Vulkan semaphore object.
/// - [`ExternalSemaphoreHandleTypeOpaqueWin32`] specifies an NT handle that has only limited valid
///   usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the
///   functions `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
///   and `SetHandleInformation`. It owns a reference to the underlying synchronization primitive
///   represented by its Vulkan semaphore object.
/// - [`ExternalSemaphoreHandleTypeOpaqueWin32Kmt`] specifies a global share handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It is not compatible with any
///   native APIs. It does not own a reference to the underlying synchronization primitive
///   represented by its Vulkan semaphore object, and will therefore become invalid when all Vulkan
///   semaphore objects associated with it are destroyed.
/// - [`ExternalSemaphoreHandleTypeD3D12Fence`] specifies an NT handle returned by
///   `ID3D12Device`::`CreateSharedHandle` referring to a Direct3D 12 fence, or
///   `ID3D11Device5`::[`CreateFence`] referring to a Direct3D 11 fence. It owns a reference to the
///   underlying synchronization primitive associated with the Direct3D fence.
/// - [`ExternalSemaphoreHandleTypeD3D11Fence`] is an alias of
///   [`ExternalSemaphoreHandleTypeD3D12Fence`] with the same meaning. It is provided for
///   convenience and code clarity when interacting with D3D11 fences.
/// - [`ExternalSemaphoreHandleTypeSyncFd`] specifies a POSIX file descriptor handle to a Linux Sync
///   File or Android Fence object. It can be used with any native API accepting a valid sync file
///   or fence as input. It owns a reference to the underlying synchronization primitive associated
///   with the file descriptor. Implementations which support importing this handle type  **must**
///   accept any type of sync or fence FD supported by the native system they are running on.
/// - [`ZirconEventFuchsia`] specifies a handle to a Zircon event object. It can be used with any
///   native API that accepts a Zircon event handle. Zircon event handles are created with
///   `ZX_RIGHTS_BASIC` and `ZX_RIGHTS_SIGNAL` rights. Vulkan on Fuchsia uses only the
///   ZX_EVENT_SIGNALED bit when signaling or waiting.
///Some external semaphore handle types can only be shared within the same
///underlying physical device and/or the same driver version, as defined in the
///following table:
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalSemaphoreHandleTypeFlags`]
/// - [`ImportSemaphoreFdInfoKHR`]
/// - [`ImportSemaphoreWin32HandleInfoKHR`]
/// - [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
/// - [`PhysicalDeviceExternalSemaphoreInfo`]
/// - [`SemaphoreGetFdInfoKHR`]
/// - [`SemaphoreGetWin32HandleInfoKHR`]
/// - [`SemaphoreGetZirconHandleInfoFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalSemaphoreHandleTypeFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum ExternalSemaphoreHandleTypeFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`ExternalSemaphoreHandleTypeOpaqueFd`] specifies a POSIX
    ///file descriptor handle that has only limited valid usage outside of
    ///Vulkan and other compatible APIs.
    ///It  **must**  be compatible with the POSIX system calls `dup`, `dup2`,
    ///`close`, and the non-standard system call `dup3`.
    ///Additionally, it  **must**  be transportable over a socket using an
    ///`SCM_RIGHTS` control message.
    ///It owns a reference to the underlying synchronization primitive
    ///represented by its Vulkan semaphore object.
    ExternalSemaphoreHandleTypeOpaqueFd = 1,
    ///[`ExternalSemaphoreHandleTypeOpaqueWin32`] specifies an NT
    ///handle that has only limited valid usage outside of Vulkan and other
    ///compatible APIs.
    ///It  **must**  be compatible with the functions `DuplicateHandle`,
    ///`CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
    ///and `SetHandleInformation`.
    ///It owns a reference to the underlying synchronization primitive
    ///represented by its Vulkan semaphore object.
    ExternalSemaphoreHandleTypeOpaqueWin32 = 2,
    ///[`ExternalSemaphoreHandleTypeOpaqueWin32Kmt`] specifies a
    ///global share handle that has only limited valid usage outside of Vulkan
    ///and other compatible APIs.
    ///It is not compatible with any native APIs.
    ///It does not own a reference to the underlying synchronization primitive
    ///represented by its Vulkan semaphore object, and will therefore become
    ///invalid when all Vulkan semaphore objects associated with it are
    ///destroyed.
    ExternalSemaphoreHandleTypeOpaqueWin32Kmt = 4,
    ///[`ExternalSemaphoreHandleTypeD3D12Fence`] specifies an NT
    ///handle returned by `ID3D12Device`::`CreateSharedHandle` referring
    ///to a Direct3D 12 fence, or `ID3D11Device5`::[`CreateFence`]
    ///referring to a Direct3D 11 fence.
    ///It owns a reference to the underlying synchronization primitive
    ///associated with the Direct3D fence.
    ExternalSemaphoreHandleTypeD3D12Fence = 8,
    ///[`ExternalSemaphoreHandleTypeSyncFd`] specifies a POSIX
    ///file descriptor handle to a Linux Sync File or Android Fence object.
    ///It can be used with any native API accepting a valid sync file or fence
    ///as input.
    ///It owns a reference to the underlying synchronization primitive
    ///associated with the file descriptor.
    ///Implementations which support importing this handle type  **must**  accept
    ///any type of sync or fence FD supported by the native system they are
    ///running on.
    ExternalSemaphoreHandleTypeSyncFd = 16,
    ///[`ZirconEventFuchsia`]
    ///specifies a handle to a Zircon event object.
    ///It can be used with any native API that accepts a Zircon event handle.
    ///Zircon event handles are created with `ZX_RIGHTS_BASIC` and
    ///`ZX_RIGHTS_SIGNAL` rights.
    ///Vulkan on Fuchsia uses only the ZX_EVENT_SIGNALED bit when signaling or
    ///waiting.
    ///
    ///Provided by [`crate::extensions::fuchsia_external_semaphore`]
    ZirconEventFuchsia = 128,
}
impl const Default for ExternalSemaphoreHandleTypeFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl ExternalSemaphoreHandleTypeFlagBits {
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
///[VkExternalSemaphoreFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html) - Bitfield describing features of an external semaphore handle type
///# C Specifications
///Bits which  **may**  be set in
///[`ExternalSemaphoreProperties::external_semaphore_features`],
///specifying the features of an external semaphore handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalSemaphoreFeatureFlagBits {
///    VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT = 0x00000001,
///    VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT = 0x00000002,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT,
///} VkExternalSemaphoreFeatureFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_semaphore_capabilities
///typedef VkExternalSemaphoreFeatureFlagBits VkExternalSemaphoreFeatureFlagBitsKHR;
///```
///# Description
/// - [`ExternalSemaphoreFeatureExportable`] specifies that handles of this type  **can**  be
///   exported from Vulkan semaphore objects.
/// - [`ExternalSemaphoreFeatureImportable`] specifies that handles of this type  **can**  be
///   imported as Vulkan semaphore objects.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalSemaphoreFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalSemaphoreFeatureFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum ExternalSemaphoreFeatureFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`ExternalSemaphoreFeatureExportable`] specifies that
    ///handles of this type  **can**  be exported from Vulkan semaphore objects.
    ExternalSemaphoreFeatureExportable = 1,
    ///[`ExternalSemaphoreFeatureImportable`] specifies that
    ///handles of this type  **can**  be imported as Vulkan semaphore objects.
    ExternalSemaphoreFeatureImportable = 2,
}
impl const Default for ExternalSemaphoreFeatureFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl ExternalSemaphoreFeatureFlagBits {
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
///[VkSemaphoreImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBits.html) - Bitmask specifying additional parameters of semaphore payload import
///# C Specifications
///Bits which  **can**  be set in
/// - [`ImportSemaphoreWin32HandleInfoKHR::flags`]
/// - [`ImportSemaphoreFdInfoKHR::flags`]
/// - [`ImportSemaphoreZirconHandleInfoFUCHSIA::flags`]
///specifying additional parameters of a semaphore import operation are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkSemaphoreImportFlagBits {
///    VK_SEMAPHORE_IMPORT_TEMPORARY_BIT = 0x00000001,
///  // Provided by VK_KHR_external_semaphore
///    VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR = VK_SEMAPHORE_IMPORT_TEMPORARY_BIT,
///} VkSemaphoreImportFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_semaphore
///typedef VkSemaphoreImportFlagBits VkSemaphoreImportFlagBitsKHR;
///```
///# Description
///These bits have the following meanings:
/// - [`SemaphoreImportTemporary`] specifies that the semaphore payload will be imported only temporarily, as described in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), regardless of the permanence of `handleType`.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`SemaphoreImportFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSemaphoreImportFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum SemaphoreImportFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`SemaphoreImportTemporary`] specifies that the semaphore
    ///payload will be imported only temporarily, as described in
    ///[Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing),
    ///regardless of the permanence of `handleType`.
    SemaphoreImportTemporary = 1,
}
impl const Default for SemaphoreImportFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl SemaphoreImportFlagBits {
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
///[VkExternalFenceHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html) - Bitmask of valid external fence handle types
///# C Specifications
///Bits which  **may**  be set in
///  * [`PhysicalDeviceExternalFenceInfo::handle_type`]
///  * [`ExternalFenceProperties::export_from_imported_handle_types`]
///  * [`ExternalFenceProperties::compatible_handle_types`]
///indicate external fence handle types, and are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalFenceHandleTypeFlagBits {
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT = 0x00000001,
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT = 0x00000002,
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 0x00000004,
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT = 0x00000008,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR =
/// VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR =
/// VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR =
/// VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR = VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT,
///} VkExternalFenceHandleTypeFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence_capabilities
///typedef VkExternalFenceHandleTypeFlagBits VkExternalFenceHandleTypeFlagBitsKHR;
///```
///# Description
/// - [`ExternalFenceHandleTypeOpaqueFd`] specifies a POSIX file descriptor handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible
///   with the POSIX system calls `dup`, `dup2`, `close`, and the non-standard system call `dup3`.
///   Additionally, it  **must**  be transportable over a socket using an `SCM_RIGHTS` control
///   message. It owns a reference to the underlying synchronization primitive represented by its
///   Vulkan fence object.
/// - [`ExternalFenceHandleTypeOpaqueWin32`] specifies an NT handle that has only limited valid
///   usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the
///   functions `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
///   and `SetHandleInformation`. It owns a reference to the underlying synchronization primitive
///   represented by its Vulkan fence object.
/// - [`ExternalFenceHandleTypeOpaqueWin32Kmt`] specifies a global share handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It is not compatible with any
///   native APIs. It does not own a reference to the underlying synchronization primitive
///   represented by its Vulkan fence object, and will therefore become invalid when all Vulkan
///   fence objects associated with it are destroyed.
/// - [`ExternalFenceHandleTypeSyncFd`] specifies a POSIX file descriptor handle to a Linux Sync
///   File or Android Fence. It can be used with any native API accepting a valid sync file or fence
///   as input. It owns a reference to the underlying synchronization primitive associated with the
///   file descriptor. Implementations which support importing this handle type  **must**  accept
///   any type of sync or fence FD supported by the native system they are running on.
///Some external fence handle types can only be shared within the same
///underlying physical device and/or the same driver version, as defined in the
///following table:
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalFenceHandleTypeFlags`]
/// - [`FenceGetFdInfoKHR`]
/// - [`FenceGetWin32HandleInfoKHR`]
/// - [`ImportFenceFdInfoKHR`]
/// - [`ImportFenceWin32HandleInfoKHR`]
/// - [`PhysicalDeviceExternalFenceInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalFenceHandleTypeFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum ExternalFenceHandleTypeFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`ExternalFenceHandleTypeOpaqueFd`] specifies a POSIX file
    ///descriptor handle that has only limited valid usage outside of Vulkan
    ///and other compatible APIs.
    ///It  **must**  be compatible with the POSIX system calls `dup`, `dup2`,
    ///`close`, and the non-standard system call `dup3`.
    ///Additionally, it  **must**  be transportable over a socket using an
    ///`SCM_RIGHTS` control message.
    ///It owns a reference to the underlying synchronization primitive
    ///represented by its Vulkan fence object.
    ExternalFenceHandleTypeOpaqueFd = 1,
    ///[`ExternalFenceHandleTypeOpaqueWin32`] specifies an NT
    ///handle that has only limited valid usage outside of Vulkan and other
    ///compatible APIs.
    ///It  **must**  be compatible with the functions `DuplicateHandle`,
    ///`CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
    ///and `SetHandleInformation`.
    ///It owns a reference to the underlying synchronization primitive
    ///represented by its Vulkan fence object.
    ExternalFenceHandleTypeOpaqueWin32 = 2,
    ///[`ExternalFenceHandleTypeOpaqueWin32Kmt`] specifies a
    ///global share handle that has only limited valid usage outside of Vulkan
    ///and other compatible APIs.
    ///It is not compatible with any native APIs.
    ///It does not own a reference to the underlying synchronization primitive
    ///represented by its Vulkan fence object, and will therefore become
    ///invalid when all Vulkan fence objects associated with it are destroyed.
    ExternalFenceHandleTypeOpaqueWin32Kmt = 4,
    ///[`ExternalFenceHandleTypeSyncFd`] specifies a POSIX file
    ///descriptor handle to a Linux Sync File or Android Fence.
    ///It can be used with any native API accepting a valid sync file or fence
    ///as input.
    ///It owns a reference to the underlying synchronization primitive
    ///associated with the file descriptor.
    ///Implementations which support importing this handle type  **must**  accept
    ///any type of sync or fence FD supported by the native system they are
    ///running on.
    ExternalFenceHandleTypeSyncFd = 8,
}
impl const Default for ExternalFenceHandleTypeFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl ExternalFenceHandleTypeFlagBits {
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
///[VkExternalFenceFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html) - Bitfield describing features of an external fence handle type
///# C Specifications
///Bits which  **may**  be set in
///[`ExternalFenceProperties::external_fence_features`], indicating
///features of a fence external handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalFenceFeatureFlagBits {
///    VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT = 0x00000001,
///    VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT = 0x00000002,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR = VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR = VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT,
///} VkExternalFenceFeatureFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence_capabilities
///typedef VkExternalFenceFeatureFlagBits VkExternalFenceFeatureFlagBitsKHR;
///```
///# Description
/// - [`ExternalFenceFeatureExportable`] specifies handles of this type  **can**  be exported from
///   Vulkan fence objects.
/// - [`ExternalFenceFeatureImportable`] specifies handles of this type  **can**  be imported to
///   Vulkan fence objects.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalFenceFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalFenceFeatureFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum ExternalFenceFeatureFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`ExternalFenceFeatureExportable`] specifies handles of this
    ///type  **can**  be exported from Vulkan fence objects.
    ExternalFenceFeatureExportable = 1,
    ///[`ExternalFenceFeatureImportable`] specifies handles of this
    ///type  **can**  be imported to Vulkan fence objects.
    ExternalFenceFeatureImportable = 2,
}
impl const Default for ExternalFenceFeatureFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl ExternalFenceFeatureFlagBits {
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
///[VkFenceImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html) - Bitmask specifying additional parameters of fence payload import
///# C Specifications
///Bits which  **can**  be set in
/// - [`ImportFenceWin32HandleInfoKHR::flags`]
/// - [`ImportFenceFdInfoKHR::flags`]
///specifying additional parameters of a fence import operation are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkFenceImportFlagBits {
///    VK_FENCE_IMPORT_TEMPORARY_BIT = 0x00000001,
///  // Provided by VK_KHR_external_fence
///    VK_FENCE_IMPORT_TEMPORARY_BIT_KHR = VK_FENCE_IMPORT_TEMPORARY_BIT,
///} VkFenceImportFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence
///typedef VkFenceImportFlagBits VkFenceImportFlagBitsKHR;
///```
///# Description
/// - [`FenceImportTemporary`] specifies that the fence payload will be imported only temporarily, as described in [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing), regardless of the permanence of `handleType`.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`FenceImportFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFenceImportFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum FenceImportFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`FenceImportTemporary`] specifies that the fence payload
    ///will be imported only temporarily, as described in
    ///[Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing),
    ///regardless of the permanence of `handleType`.
    FenceImportTemporary = 1,
}
impl const Default for FenceImportFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl FenceImportFlagBits {
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
///[VkPeerMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html) - Bitmask specifying supported peer memory features
///# C Specifications
///Bits which  **may**  be set in
///[`GetDeviceGroupPeerMemoryFeatures`]`::pPeerMemoryFeatures`,
///indicating supported peer memory features, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkPeerMemoryFeatureFlagBits {
///    VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT = 0x00000001,
///    VK_PEER_MEMORY_FEATURE_COPY_DST_BIT = 0x00000002,
///    VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT = 0x00000004,
///    VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT = 0x00000008,
///  // Provided by VK_KHR_device_group
///    VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR = VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT,
///  // Provided by VK_KHR_device_group
///    VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR = VK_PEER_MEMORY_FEATURE_COPY_DST_BIT,
///  // Provided by VK_KHR_device_group
///    VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR = VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT,
///  // Provided by VK_KHR_device_group
///    VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR = VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT,
///} VkPeerMemoryFeatureFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group
///typedef VkPeerMemoryFeatureFlagBits VkPeerMemoryFeatureFlagBitsKHR;
///```
///# Description
/// - [`PeerMemoryFeatureCopySrc`] specifies that the memory  **can**  be accessed as the source of
///   any `vkCmdCopy*` command.
/// - [`PeerMemoryFeatureCopyDst`] specifies that the memory  **can**  be accessed as the
///   destination of any `vkCmdCopy*` command.
/// - [`PeerMemoryFeatureGenericSrc`] specifies that the memory  **can**  be read as any memory
///   access type.
/// - [`PeerMemoryFeatureGenericDst`] specifies that the memory  **can**  be written as any memory
///   access type. Shader atomics are considered to be writes.
///[`PeerMemoryFeatureCopyDst`] **must**  be supported for all host
///local heaps and for at least one device-local memory heap.If a device does not support a peer
/// memory feature, it is still valid to use
///a resource that includes both local and peer memory bindings with the
///corresponding access type as long as only the local bindings are actually
///accessed.
///For example, an application doing split-frame rendering would use
///framebuffer attachments that include both local and peer memory bindings,
///but would scissor the rendering to only update local memory.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`PeerMemoryFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPeerMemoryFeatureFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum PeerMemoryFeatureFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`PeerMemoryFeatureCopySrc`] specifies that the memory  **can**
    ///be accessed as the source of any `vkCmdCopy*` command.
    PeerMemoryFeatureCopySrc = 1,
    ///[`PeerMemoryFeatureCopyDst`] specifies that the memory  **can**
    ///be accessed as the destination of any `vkCmdCopy*` command.
    PeerMemoryFeatureCopyDst = 2,
    ///[`PeerMemoryFeatureGenericSrc`] specifies that the memory
    /// **can**  be read as any memory access type.
    PeerMemoryFeatureGenericSrc = 4,
    ///[`PeerMemoryFeatureGenericDst`] specifies that the memory
    /// **can**  be written as any memory access type.
    ///Shader atomics are considered to be writes.
    PeerMemoryFeatureGenericDst = 8,
}
impl const Default for PeerMemoryFeatureFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl PeerMemoryFeatureFlagBits {
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
///[VkMemoryAllocateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html) - Bitmask specifying flags for a device memory allocation
///# C Specifications
///Bits which  **can**  be set in [`MemoryAllocateFlagsInfo::flags`],
///controlling device memory allocation, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkMemoryAllocateFlagBits {
///    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT = 0x00000001,
///  // Provided by VK_VERSION_1_2
///    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT = 0x00000002,
///  // Provided by VK_VERSION_1_2
///    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 0x00000004,
///  // Provided by VK_KHR_device_group
///    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR = VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT,
///  // Provided by VK_KHR_buffer_device_address
///    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR = VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT,
///  // Provided by VK_KHR_buffer_device_address
///    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR =
/// VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
///} VkMemoryAllocateFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group
///typedef VkMemoryAllocateFlagBits VkMemoryAllocateFlagBitsKHR;
///```
///# Description
/// - [`MemoryAllocateDeviceMask`] specifies that memory will be allocated for the devices in
///   [`MemoryAllocateFlagsInfo::device_mask`].
/// - [`DeviceAddress`] specifies that the memory  **can**  be attached to a buffer object created
///   with the `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` bit set in `usage`, and that the memory
///   handle  **can**  be used to retrieve an opaque address via
///   [`GetDeviceMemoryOpaqueCaptureAddress`].
/// - [`DeviceAddressCaptureReplay`] specifies that the memory’s address  **can**  be saved and
///   reused on a subsequent run (e.g. for trace capture and replay), see
///   [`BufferOpaqueCaptureAddressCreateInfo`] for more detail.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`MemoryAllocateFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryAllocateFlagBits")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum MemoryAllocateFlagBits {
    #[doc(hidden)]
    Empty = 0,
    ///[`MemoryAllocateDeviceMask`] specifies that memory will be
    ///allocated for the devices in
    ///[`MemoryAllocateFlagsInfo`]::`deviceMask`.
    MemoryAllocateDeviceMask = 1,
    ///[`DeviceAddress`] specifies that the memory
    /// **can**  be attached to a buffer object created with the
    ///`VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` bit set in `usage`,
    ///and that the memory handle  **can**  be used to retrieve an opaque address
    ///via [`GetDeviceMemoryOpaqueCaptureAddress`].
    ///
    ///Provided by [`crate::vulkan1_2`]
    DeviceAddress = 2,
    ///[`DeviceAddressCaptureReplay`] specifies
    ///that the memory’s address  **can**  be saved and reused on a subsequent run
    ///(e.g. for trace capture and replay), see
    ///[`BufferOpaqueCaptureAddressCreateInfo`] for more detail.
    ///
    ///Provided by [`crate::vulkan1_2`]
    DeviceAddressCaptureReplay = 4,
}
impl const Default for MemoryAllocateFlagBits {
    fn default() -> Self {
        Self::Empty
    }
}
impl MemoryAllocateFlagBits {
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
///[VkSubgroupFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlagBits.html) - Bitmask describing what group operations are supported with subgroup scope
///# C Specifications
///Bits which  **can**  be set in
///[`PhysicalDeviceSubgroupProperties::supported_operations`]
///and
///[`PhysicalDeviceVulkan11Properties::subgroup_supported_operations`]
///to specify supported [group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) with
///[subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup) are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkSubgroupFeatureFlagBits {
///    VK_SUBGROUP_FEATURE_BASIC_BIT = 0x00000001,
///    VK_SUBGROUP_FEATURE_VOTE_BIT = 0x00000002,
///    VK_SUBGROUP_FEATURE_ARITHMETIC_BIT = 0x00000004,
///    VK_SUBGROUP_FEATURE_BALLOT_BIT = 0x00000008,
///    VK_SUBGROUP_FEATURE_SHUFFLE_BIT = 0x00000010,
///    VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT = 0x00000020,
///    VK_SUBGROUP_FEATURE_CLUSTERED_BIT = 0x00000040,
///    VK_SUBGROUP_FEATURE_QUAD_BIT = 0x00000080,
///  // Provided by VK_NV_shader_subgroup_partitioned
///    VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV = 0x00000100,
///} VkSubgroupFeatureFlagBits;
///```
///# Description
/// - [`SubgroupFeatureBasic`] specifies the device will accept SPIR-V shader modules containing the
///   `GroupNonUniform` capability.
/// - [`SubgroupFeatureVote`] specifies the device will accept SPIR-V shader modules containing the
///   `GroupNonUniformVote` capability.
/// - [`SubgroupFeatureArithmetic`] specifies the device will accept SPIR-V shader modules
///   containing the `GroupNonUniformArithmetic` capability.
/// - [`SubgroupFeatureBallot`] specifies the device will accept SPIR-V shader modules containing
///   the `GroupNonUniformBallot` capability.
/// - [`SubgroupFeatureShuffle`] specifies the device will accept SPIR-V shader modules containing
///   the `GroupNonUniformShuffle` capability.
/// - [`SubgroupFeatureShuffleRelative`] specifies the device will accept SPIR-V shader modules
///   containing the `GroupNonUniformShuffleRelative` capability.
/// - [`SubgroupFeatureClustered`] specifies the device will accept SPIR-V shader modules containing
///   the `GroupNonUniformClustered` capability.
/// - [`SubgroupFeatureQuad`] specifies the device will accept SPIR-V shader modules containing the
///   `GroupNonUniformQuad` capability.
/// - [`PartitionedNv`] specifies the device will accept SPIR-V shader modules containing the
///   `GroupNonUniformPartitionedNV` capability.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`SubgroupFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSubgroupFeatureFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct SubgroupFeatureFlags(u32);
impl const Default for SubgroupFeatureFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn from(from: SubgroupFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl SubgroupFeatureFlags {
    ///[`SubgroupFeatureBasic`]
    ///specifies the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniform` capability.
    pub const SUBGROUP_FEATURE_BASIC: Self = Self(1);
    ///[`SubgroupFeatureVote`] specifies
    ///the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformVote` capability.
    pub const SUBGROUP_FEATURE_VOTE: Self = Self(2);
    ///[`SubgroupFeatureArithmetic`] specifies the device will
    ///accept SPIR-V shader modules containing the
    ///`GroupNonUniformArithmetic` capability.
    pub const SUBGROUP_FEATURE_ARITHMETIC: Self = Self(4);
    ///[`SubgroupFeatureBallot`]
    ///specifies the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformBallot` capability.
    pub const SUBGROUP_FEATURE_BALLOT: Self = Self(8);
    ///[`SubgroupFeatureShuffle`]
    ///specifies the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformShuffle` capability.
    pub const SUBGROUP_FEATURE_SHUFFLE: Self = Self(16);
    ///[`SubgroupFeatureShuffleRelative`] specifies the device will
    ///accept SPIR-V shader modules containing the
    ///`GroupNonUniformShuffleRelative` capability.
    pub const SUBGROUP_FEATURE_SHUFFLE_RELATIVE: Self = Self(32);
    ///[`SubgroupFeatureClustered`]
    ///specifies the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformClustered` capability.
    pub const SUBGROUP_FEATURE_CLUSTERED: Self = Self(64);
    ///[`SubgroupFeatureQuad`] specifies
    ///the device will accept SPIR-V shader modules containing the
    ///`GroupNonUniformQuad` capability.
    pub const SUBGROUP_FEATURE_QUAD: Self = Self(128);
    ///[`PartitionedNv`] specifies the device will
    ///accept SPIR-V shader modules containing the
    ///`GroupNonUniformPartitionedNV` capability.
    ///
    ///Provided by [`crate::extensions::nv_shader_subgroup_partitioned`]
    pub const PARTITIONED_NV: Self = Self(256);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::SUBGROUP_FEATURE_BASIC
            | Self::SUBGROUP_FEATURE_VOTE
            | Self::SUBGROUP_FEATURE_ARITHMETIC
            | Self::SUBGROUP_FEATURE_BALLOT
            | Self::SUBGROUP_FEATURE_SHUFFLE
            | Self::SUBGROUP_FEATURE_SHUFFLE_RELATIVE
            | Self::SUBGROUP_FEATURE_CLUSTERED
            | Self::SUBGROUP_FEATURE_QUAD
            | Self::PARTITIONED_NV
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
impl const std::ops::BitOr for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for SubgroupFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for SubgroupFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for SubgroupFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for SubgroupFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SubgroupFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SubgroupFeatureFlags> for SubgroupFeatureFlags {
    fn extend<T: IntoIterator<Item = SubgroupFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn extend<T: IntoIterator<Item = SubgroupFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<SubgroupFeatureFlagBits>>::from(i));
        }
    }
}
impl FromIterator<SubgroupFeatureFlags> for SubgroupFeatureFlags {
    fn from_iter<T: IntoIterator<Item = SubgroupFeatureFlags>>(iterator: T) -> SubgroupFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<SubgroupFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn from_iter<T: IntoIterator<Item = SubgroupFeatureFlagBits>>(iterator: T) -> SubgroupFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<SubgroupFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SubgroupFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SubgroupFeatureFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SubgroupFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SubgroupFeatureFlags::SUBGROUP_FEATURE_BASIC) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SUBGROUP_FEATURE_BASIC))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SUBGROUP_FEATURE_VOTE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SUBGROUP_FEATURE_VOTE))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SUBGROUP_FEATURE_ARITHMETIC) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SUBGROUP_FEATURE_ARITHMETIC))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SUBGROUP_FEATURE_BALLOT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SUBGROUP_FEATURE_BALLOT))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SUBGROUP_FEATURE_SHUFFLE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SUBGROUP_FEATURE_SHUFFLE))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SUBGROUP_FEATURE_SHUFFLE_RELATIVE) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SUBGROUP_FEATURE_SHUFFLE_RELATIVE))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SUBGROUP_FEATURE_CLUSTERED) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SUBGROUP_FEATURE_CLUSTERED))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::SUBGROUP_FEATURE_QUAD) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SUBGROUP_FEATURE_QUAD))?;
                    }
                    if self.0.contains(SubgroupFeatureFlags::PARTITIONED_NV) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PARTITIONED_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SubgroupFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDescriptorUpdateTemplateCreateFlags](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateFlags.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_VERSION_1_1
///typedef VkFlags VkDescriptorUpdateTemplateCreateFlags;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_descriptor_update_template
///typedef VkDescriptorUpdateTemplateCreateFlags VkDescriptorUpdateTemplateCreateFlagsKHR;
///```
///# Related
/// - [`crate::vulkan1_1`]
/// - [`DescriptorUpdateTemplateCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateCreateFlags(u32);
impl const Default for DescriptorUpdateTemplateCreateFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DescriptorUpdateTemplateCreateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(DescriptorUpdateTemplateCreateFlags))
            .field(&self.0)
            .finish()
    }
}
///[VkPeerMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPeerMemoryFeatureFlagBits.html) - Bitmask specifying supported peer memory features
///# C Specifications
///Bits which  **may**  be set in
///[`GetDeviceGroupPeerMemoryFeatures`]`::pPeerMemoryFeatures`,
///indicating supported peer memory features, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkPeerMemoryFeatureFlagBits {
///    VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT = 0x00000001,
///    VK_PEER_MEMORY_FEATURE_COPY_DST_BIT = 0x00000002,
///    VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT = 0x00000004,
///    VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT = 0x00000008,
///  // Provided by VK_KHR_device_group
///    VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR = VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT,
///  // Provided by VK_KHR_device_group
///    VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR = VK_PEER_MEMORY_FEATURE_COPY_DST_BIT,
///  // Provided by VK_KHR_device_group
///    VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR = VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT,
///  // Provided by VK_KHR_device_group
///    VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR = VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT,
///} VkPeerMemoryFeatureFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group
///typedef VkPeerMemoryFeatureFlagBits VkPeerMemoryFeatureFlagBitsKHR;
///```
///# Description
/// - [`PeerMemoryFeatureCopySrc`] specifies that the memory  **can**  be accessed as the source of
///   any `vkCmdCopy*` command.
/// - [`PeerMemoryFeatureCopyDst`] specifies that the memory  **can**  be accessed as the
///   destination of any `vkCmdCopy*` command.
/// - [`PeerMemoryFeatureGenericSrc`] specifies that the memory  **can**  be read as any memory
///   access type.
/// - [`PeerMemoryFeatureGenericDst`] specifies that the memory  **can**  be written as any memory
///   access type. Shader atomics are considered to be writes.
///[`PeerMemoryFeatureCopyDst`] **must**  be supported for all host
///local heaps and for at least one device-local memory heap.If a device does not support a peer
/// memory feature, it is still valid to use
///a resource that includes both local and peer memory bindings with the
///corresponding access type as long as only the local bindings are actually
///accessed.
///For example, an application doing split-frame rendering would use
///framebuffer attachments that include both local and peer memory bindings,
///but would scissor the rendering to only update local memory.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`PeerMemoryFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPeerMemoryFeatureFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PeerMemoryFeatureFlags(u32);
impl const Default for PeerMemoryFeatureFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn from(from: PeerMemoryFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl PeerMemoryFeatureFlags {
    ///[`PeerMemoryFeatureCopySrc`] specifies that the memory  **can**
    ///be accessed as the source of any `vkCmdCopy*` command.
    pub const PEER_MEMORY_FEATURE_COPY_SRC: Self = Self(1);
    ///[`PeerMemoryFeatureCopyDst`] specifies that the memory  **can**
    ///be accessed as the destination of any `vkCmdCopy*` command.
    pub const PEER_MEMORY_FEATURE_COPY_DST: Self = Self(2);
    ///[`PeerMemoryFeatureGenericSrc`] specifies that the memory
    /// **can**  be read as any memory access type.
    pub const PEER_MEMORY_FEATURE_GENERIC_SRC: Self = Self(4);
    ///[`PeerMemoryFeatureGenericDst`] specifies that the memory
    /// **can**  be written as any memory access type.
    ///Shader atomics are considered to be writes.
    pub const PEER_MEMORY_FEATURE_GENERIC_DST: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::PEER_MEMORY_FEATURE_COPY_SRC
            | Self::PEER_MEMORY_FEATURE_COPY_DST
            | Self::PEER_MEMORY_FEATURE_GENERIC_SRC
            | Self::PEER_MEMORY_FEATURE_GENERIC_DST
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
impl const std::ops::BitOr for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for PeerMemoryFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for PeerMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PeerMemoryFeatureFlags> for PeerMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = PeerMemoryFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = PeerMemoryFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<PeerMemoryFeatureFlagBits>>::from(i));
        }
    }
}
impl FromIterator<PeerMemoryFeatureFlags> for PeerMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = PeerMemoryFeatureFlags>>(iterator: T) -> PeerMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<PeerMemoryFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = PeerMemoryFeatureFlagBits>>(iterator: T) -> PeerMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<PeerMemoryFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PeerMemoryFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PeerMemoryFeatureFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PeerMemoryFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(PeerMemoryFeatureFlags::PEER_MEMORY_FEATURE_COPY_SRC) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PEER_MEMORY_FEATURE_COPY_SRC))?;
                    }
                    if self.0.contains(PeerMemoryFeatureFlags::PEER_MEMORY_FEATURE_COPY_DST) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PEER_MEMORY_FEATURE_COPY_DST))?;
                    }
                    if self.0.contains(PeerMemoryFeatureFlags::PEER_MEMORY_FEATURE_GENERIC_SRC) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PEER_MEMORY_FEATURE_GENERIC_SRC))?;
                    }
                    if self.0.contains(PeerMemoryFeatureFlags::PEER_MEMORY_FEATURE_GENERIC_DST) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(PEER_MEMORY_FEATURE_GENERIC_DST))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PeerMemoryFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkMemoryAllocateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagBits.html) - Bitmask specifying flags for a device memory allocation
///# C Specifications
///Bits which  **can**  be set in [`MemoryAllocateFlagsInfo::flags`],
///controlling device memory allocation, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkMemoryAllocateFlagBits {
///    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT = 0x00000001,
///  // Provided by VK_VERSION_1_2
///    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT = 0x00000002,
///  // Provided by VK_VERSION_1_2
///    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 0x00000004,
///  // Provided by VK_KHR_device_group
///    VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR = VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT,
///  // Provided by VK_KHR_buffer_device_address
///    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR = VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT,
///  // Provided by VK_KHR_buffer_device_address
///    VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR =
/// VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
///} VkMemoryAllocateFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group
///typedef VkMemoryAllocateFlagBits VkMemoryAllocateFlagBitsKHR;
///```
///# Description
/// - [`MemoryAllocateDeviceMask`] specifies that memory will be allocated for the devices in
///   [`MemoryAllocateFlagsInfo::device_mask`].
/// - [`DeviceAddress`] specifies that the memory  **can**  be attached to a buffer object created
///   with the `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` bit set in `usage`, and that the memory
///   handle  **can**  be used to retrieve an opaque address via
///   [`GetDeviceMemoryOpaqueCaptureAddress`].
/// - [`DeviceAddressCaptureReplay`] specifies that the memory’s address  **can**  be saved and
///   reused on a subsequent run (e.g. for trace capture and replay), see
///   [`BufferOpaqueCaptureAddressCreateInfo`] for more detail.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`MemoryAllocateFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryAllocateFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct MemoryAllocateFlags(u32);
impl const Default for MemoryAllocateFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn from(from: MemoryAllocateFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl MemoryAllocateFlags {
    ///[`MemoryAllocateDeviceMask`] specifies that memory will be
    ///allocated for the devices in
    ///[`MemoryAllocateFlagsInfo`]::`deviceMask`.
    pub const MEMORY_ALLOCATE_DEVICE_MASK: Self = Self(1);
    ///[`DeviceAddress`] specifies that the memory
    /// **can**  be attached to a buffer object created with the
    ///`VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` bit set in `usage`,
    ///and that the memory handle  **can**  be used to retrieve an opaque address
    ///via [`GetDeviceMemoryOpaqueCaptureAddress`].
    ///
    ///Provided by [`crate::vulkan1_2`]
    pub const DEVICE_ADDRESS: Self = Self(2);
    ///[`DeviceAddressCaptureReplay`] specifies
    ///that the memory’s address  **can**  be saved and reused on a subsequent run
    ///(e.g. for trace capture and replay), see
    ///[`BufferOpaqueCaptureAddressCreateInfo`] for more detail.
    ///
    ///Provided by [`crate::vulkan1_2`]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty() | Self::MEMORY_ALLOCATE_DEVICE_MASK | Self::DEVICE_ADDRESS | Self::DEVICE_ADDRESS_CAPTURE_REPLAY
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
impl const std::ops::BitOr for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for MemoryAllocateFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for MemoryAllocateFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for MemoryAllocateFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for MemoryAllocateFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for MemoryAllocateFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<MemoryAllocateFlags> for MemoryAllocateFlags {
    fn extend<T: IntoIterator<Item = MemoryAllocateFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn extend<T: IntoIterator<Item = MemoryAllocateFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<MemoryAllocateFlagBits>>::from(i));
        }
    }
}
impl FromIterator<MemoryAllocateFlags> for MemoryAllocateFlags {
    fn from_iter<T: IntoIterator<Item = MemoryAllocateFlags>>(iterator: T) -> MemoryAllocateFlags {
        let mut out = Self::empty();
        <Self as Extend<MemoryAllocateFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn from_iter<T: IntoIterator<Item = MemoryAllocateFlagBits>>(iterator: T) -> MemoryAllocateFlags {
        let mut out = Self::empty();
        <Self as Extend<MemoryAllocateFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for MemoryAllocateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(MemoryAllocateFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == MemoryAllocateFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(MemoryAllocateFlags::MEMORY_ALLOCATE_DEVICE_MASK) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(MEMORY_ALLOCATE_DEVICE_MASK))?;
                    }
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEVICE_ADDRESS))?;
                    }
                    if self.0.contains(MemoryAllocateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEVICE_ADDRESS_CAPTURE_REPLAY))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(MemoryAllocateFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkCommandPoolTrimFlags](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolTrimFlags.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_VERSION_1_1
///typedef VkFlags VkCommandPoolTrimFlags;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance1
///typedef VkCommandPoolTrimFlags VkCommandPoolTrimFlagsKHR;
///```
///# Related
/// - [`crate::vulkan1_1`]
/// - [`TrimCommandPool`]
/// - [`TrimCommandPoolKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct CommandPoolTrimFlags(u32);
impl const Default for CommandPoolTrimFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for CommandPoolTrimFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(CommandPoolTrimFlags)).field(&self.0).finish()
    }
}
///[VkExternalMemoryHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBits.html) - Bit specifying external memory handle types
///# C Specifications
///Possible values of
///[`PhysicalDeviceExternalImageFormatInfo::handle_type`], specifying
///an external memory handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalMemoryHandleTypeFlagBits {
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT = 0x00000001,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT = 0x00000002,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 0x00000004,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT = 0x00000008,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT = 0x00000010,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT = 0x00000020,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT = 0x00000040,
///  // Provided by VK_EXT_external_memory_dma_buf
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT = 0x00000200,
///  // Provided by VK_ANDROID_external_memory_android_hardware_buffer
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID = 0x00000400,
///  // Provided by VK_EXT_external_memory_host
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT = 0x00000080,
///  // Provided by VK_EXT_external_memory_host
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT = 0x00000100,
///  // Provided by VK_FUCHSIA_external_memory
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA = 0x00000800,
///  // Provided by VK_NV_external_memory_rdma
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV = 0x00001000,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR =
/// VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT,
///} VkExternalMemoryHandleTypeFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory_capabilities
///typedef VkExternalMemoryHandleTypeFlagBits VkExternalMemoryHandleTypeFlagBitsKHR;
///```
///# Description
/// - [`ExternalMemoryHandleTypeOpaqueFd`] specifies a POSIX file descriptor handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible
///   with the POSIX system calls `dup`, `dup2`, `close`, and the non-standard system call `dup3`.
///   Additionally, it  **must**  be transportable over a socket using an `SCM_RIGHTS` control
///   message. It owns a reference to the underlying memory resource represented by its Vulkan
///   memory object.
/// - [`ExternalMemoryHandleTypeOpaqueWin32`] specifies an NT handle that has only limited valid
///   usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the
///   functions `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
///   and `SetHandleInformation`. It owns a reference to the underlying memory resource represented
///   by its Vulkan memory object.
/// - [`ExternalMemoryHandleTypeOpaqueWin32Kmt`] specifies a global share handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It is not compatible with any
///   native APIs. It does not own a reference to the underlying memory resource represented by its
///   Vulkan memory object, and will therefore become invalid when all Vulkan memory objects
///   associated with it are destroyed.
/// - [`ExternalMemoryHandleTypeD3D11Texture`] specifies an NT handle returned by
///   `IDXGIResource1`::`CreateSharedHandle` referring to a Direct3D 10 or 11 texture resource. It
///   owns a reference to the memory used by the Direct3D resource.
/// - [`ExternalMemoryHandleTypeD3D11TextureKmt`] specifies a global share handle returned by
///   `IDXGIResource`::`GetSharedHandle` referring to a Direct3D 10 or 11 texture resource. It does
///   not own a reference to the underlying Direct3D resource, and will therefore become invalid
///   when all Vulkan memory objects and Direct3D resources associated with it are destroyed.
/// - [`ExternalMemoryHandleTypeD3D12Heap`] specifies an NT handle returned by
///   `ID3D12Device`::`CreateSharedHandle` referring to a Direct3D 12 heap resource. It owns a
///   reference to the resources used by the Direct3D heap.
/// - [`ExternalMemoryHandleTypeD3D12Resource`] specifies an NT handle returned by
///   `ID3D12Device`::`CreateSharedHandle` referring to a Direct3D 12 committed resource. It owns a
///   reference to the memory used by the Direct3D resource.
/// - [`HostAllocationExt`] specifies a host pointer returned by a host memory allocation command.
///   It does not own a reference to the underlying memory resource, and will therefore become
///   invalid if the host memory is freed.
/// - [`HostMappedForeignMemoryExt`] specifies a host pointer to *host mapped foreign memory*. It
///   does not own a reference to the underlying memory resource, and will therefore become invalid
///   if the foreign memory is unmapped or otherwise becomes no longer available.
/// - [`DmaBufExt`] is a file descriptor for a Linux dma_buf. It owns a reference to the underlying
///   memory resource represented by its Vulkan memory object.
/// - [`AndroidHardwareBufferAndroid`] specifies an [`AHardwareBuffer`] object defined by the Android NDK. See [Android Hardware Buffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer) for more details of this handle type.
/// - [`ZirconVmoFuchsia`] is a Zircon handle to a virtual memory object.
/// - [`RdmaAddressNv`] is a handle to an allocation accessible by remote devices. It owns a
///   reference to the underlying memory resource represented by its Vulkan memory object.
///Some external memory handle types can only be shared within the same
///underlying physical device and/or the same driver version, as defined in the
///following table:
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryHandleTypeFlags`]
/// - [`ImportMemoryFdInfoKHR`]
/// - [`ImportMemoryHostPointerInfoEXT`]
/// - [`ImportMemoryWin32HandleInfoKHR`]
/// - [`ImportMemoryZirconHandleInfoFUCHSIA`]
/// - [`MemoryGetFdInfoKHR`]
/// - [`MemoryGetRemoteAddressInfoNV`]
/// - [`MemoryGetWin32HandleInfoKHR`]
/// - [`MemoryGetZirconHandleInfoFUCHSIA`]
/// - [`PhysicalDeviceExternalBufferInfo`]
/// - [`PhysicalDeviceExternalImageFormatInfo`]
/// - [`GetMemoryFdPropertiesKHR`]
/// - [`GetMemoryHostPointerPropertiesEXT`]
/// - [`GetMemoryWin32HandlePropertiesKHR`]
/// - [`GetMemoryZirconHandlePropertiesFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryHandleTypeFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ExternalMemoryHandleTypeFlags(u32);
impl const Default for ExternalMemoryHandleTypeFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn from(from: ExternalMemoryHandleTypeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl ExternalMemoryHandleTypeFlags {
    ///[`ExternalMemoryHandleTypeOpaqueFd`] specifies a POSIX
    ///file descriptor handle that has only limited valid usage outside of
    ///Vulkan and other compatible APIs.
    ///It  **must**  be compatible with the POSIX system calls `dup`, `dup2`,
    ///`close`, and the non-standard system call `dup3`.
    ///Additionally, it  **must**  be transportable over a socket using an
    ///`SCM_RIGHTS` control message.
    ///It owns a reference to the underlying memory resource represented by its
    ///Vulkan memory object.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD: Self = Self(1);
    ///[`ExternalMemoryHandleTypeOpaqueWin32`] specifies an NT
    ///handle that has only limited valid usage outside of Vulkan and other
    ///compatible APIs.
    ///It  **must**  be compatible with the functions `DuplicateHandle`,
    ///`CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
    ///and `SetHandleInformation`.
    ///It owns a reference to the underlying memory resource represented by its
    ///Vulkan memory object.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32: Self = Self(2);
    ///[`ExternalMemoryHandleTypeOpaqueWin32Kmt`] specifies a
    ///global share handle that has only limited valid usage outside of Vulkan
    ///and other compatible APIs.
    ///It is not compatible with any native APIs.
    ///It does not own a reference to the underlying memory resource
    ///represented by its Vulkan memory object, and will therefore become
    ///invalid when all Vulkan memory objects associated with it are destroyed.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_KMT: Self = Self(4);
    ///[`ExternalMemoryHandleTypeD3D11Texture`] specifies an NT
    ///handle returned by `IDXGIResource1`::`CreateSharedHandle`
    ///referring to a Direct3D 10 or 11 texture resource.
    ///It owns a reference to the memory used by the Direct3D resource.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_TEXTURE: Self = Self(8);
    ///[`ExternalMemoryHandleTypeD3D11TextureKmt`] specifies a
    ///global share handle returned by `IDXGIResource`::`GetSharedHandle`
    ///referring to a Direct3D 10 or 11 texture resource.
    ///It does not own a reference to the underlying Direct3D resource, and
    ///will therefore become invalid when all Vulkan memory objects and
    ///Direct3D resources associated with it are destroyed.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_TEXTURE_KMT: Self = Self(16);
    ///[`ExternalMemoryHandleTypeD3D12Heap`] specifies an NT
    ///handle returned by `ID3D12Device`::`CreateSharedHandle` referring
    ///to a Direct3D 12 heap resource.
    ///It owns a reference to the resources used by the Direct3D heap.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_12_HEAP: Self = Self(32);
    ///[`ExternalMemoryHandleTypeD3D12Resource`] specifies an NT
    ///handle returned by `ID3D12Device`::`CreateSharedHandle` referring
    ///to a Direct3D 12 committed resource.
    ///It owns a reference to the memory used by the Direct3D resource.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_12_RESOURCE: Self = Self(64);
    ///[`DmaBufExt`] is a file
    ///descriptor for a Linux dma_buf.
    ///It owns a reference to the underlying memory resource represented by its
    ///Vulkan memory object.
    ///
    ///Provided by [`crate::extensions::ext_external_memory_dma_buf`]
    pub const DMA_BUF_EXT: Self = Self(512);
    ///[`AndroidHardwareBufferAndroid`]
    ///specifies an [`AHardwareBuffer`] object defined by the Android NDK.
    ///See [Android Hardware Buffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer)
    ///for more details of this handle type.
    ///
    ///Provided by [`crate::extensions::android_external_memory_android_hardware_buffer`]
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1024);
    ///[`HostAllocationExt`] specifies a
    ///host pointer returned by a host memory allocation command.
    ///It does not own a reference to the underlying memory resource, and will
    ///therefore become invalid if the host memory is freed.
    ///
    ///Provided by [`crate::extensions::ext_external_memory_host`]
    pub const HOST_ALLOCATION_EXT: Self = Self(128);
    ///[`HostMappedForeignMemoryExt`]
    ///specifies a host pointer to *host mapped foreign memory*.
    ///It does not own a reference to the underlying memory resource, and will
    ///therefore become invalid if the foreign memory is unmapped or otherwise
    ///becomes no longer available.
    ///
    ///Provided by [`crate::extensions::ext_external_memory_host`]
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(256);
    ///[`ZirconVmoFuchsia`] is a Zircon
    ///handle to a virtual memory object.
    ///
    ///Provided by [`crate::extensions::fuchsia_external_memory`]
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(2048);
    ///[`RdmaAddressNv`] is a handle to
    ///an allocation accessible by remote devices.
    ///It owns a reference to the underlying memory resource represented by its
    ///Vulkan memory object.
    ///
    ///Provided by [`crate::extensions::nv_external_memory_rdma`]
    pub const RDMA_ADDRESS_NV: Self = Self(4096);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_extension_375`]
    pub const RESERVED_13_NV: Self = Self(8192);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD
            | Self::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32
            | Self::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_KMT
            | Self::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_TEXTURE
            | Self::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_TEXTURE_KMT
            | Self::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_12_HEAP
            | Self::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_12_RESOURCE
            | Self::DMA_BUF_EXT
            | Self::ANDROID_HARDWARE_BUFFER_ANDROID
            | Self::HOST_ALLOCATION_EXT
            | Self::HOST_MAPPED_FOREIGN_MEMORY_EXT
            | Self::ZIRCON_VMO_FUCHSIA
            | Self::RDMA_ADDRESS_NV
            | Self::RESERVED_13_NV
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
impl const std::ops::BitOr for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalMemoryHandleTypeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryHandleTypeFlags> for ExternalMemoryHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ExternalMemoryHandleTypeFlagBits>>::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlags> for ExternalMemoryHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlags>>(iterator: T) -> ExternalMemoryHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBits>>(
        iterator: T,
    ) -> ExternalMemoryHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryHandleTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryHandleTypeFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryHandleTypeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_KMT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_KMT))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_TEXTURE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_TEXTURE))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_TEXTURE_KMT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_TEXTURE_KMT))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_12_HEAP)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_12_HEAP))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_12_RESOURCE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_12_RESOURCE))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::DMA_BUF_EXT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DMA_BUF_EXT))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::ANDROID_HARDWARE_BUFFER_ANDROID)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ANDROID_HARDWARE_BUFFER_ANDROID))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::HOST_ALLOCATION_EXT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(HOST_ALLOCATION_EXT))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlags::HOST_MAPPED_FOREIGN_MEMORY_EXT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(HOST_MAPPED_FOREIGN_MEMORY_EXT))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::ZIRCON_VMO_FUCHSIA) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ZIRCON_VMO_FUCHSIA))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::RDMA_ADDRESS_NV) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RDMA_ADDRESS_NV))?;
                    }
                    if self.0.contains(ExternalMemoryHandleTypeFlags::RESERVED_13_NV) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESERVED_13_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryHandleTypeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkExternalMemoryFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBits.html) - Bitmask specifying features of an external memory handle type
///# C Specifications
///Bits which  **may**  be set in
///[`ExternalMemoryProperties::external_memory_features`], specifying
///features of an external memory handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalMemoryFeatureFlagBits {
///    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT = 0x00000001,
///    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT = 0x00000002,
///    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT = 0x00000004,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR =
/// VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR = VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT,
///  // Provided by VK_KHR_external_memory_capabilities
///    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR = VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT,
///} VkExternalMemoryFeatureFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory_capabilities
///typedef VkExternalMemoryFeatureFlagBits VkExternalMemoryFeatureFlagBitsKHR;
///```
///# Description
/// - [`ExternalMemoryFeatureDedicatedOnly`] specifies that images or buffers created with the
///   specified parameters and handle type  **must**  use the mechanisms defined by
///   [`MemoryDedicatedRequirements`] and [`MemoryDedicatedAllocateInfo`] to create (or import) a
///   dedicated allocation for the image or buffer.
/// - [`ExternalMemoryFeatureExportable`] specifies that handles of this type  **can**  be exported
///   from Vulkan memory objects.
/// - [`ExternalMemoryFeatureImportable`] specifies that handles of this type  **can**  be imported
///   as Vulkan memory objects.
///Because their semantics in external APIs roughly align with that of an image
///or buffer with a dedicated allocation in Vulkan, implementations are
/// **required**  to report [`ExternalMemoryFeatureDedicatedOnly`] for
///the following external handle types:
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`
/// - `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` for images only
///Implementations  **must**  not report
///[`ExternalMemoryFeatureDedicatedOnly`] for buffers with
///external handle type
///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`.
///Implementations  **must**  not report
///[`ExternalMemoryFeatureDedicatedOnly`] for images or buffers
///with external handle type
///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT`, or
///`VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT`.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryFeatureFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ExternalMemoryFeatureFlags(u32);
impl const Default for ExternalMemoryFeatureFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn from(from: ExternalMemoryFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl ExternalMemoryFeatureFlags {
    ///[`ExternalMemoryFeatureDedicatedOnly`] specifies that
    ///images or buffers created with the specified parameters and handle type
    /// **must**  use the mechanisms defined by [`MemoryDedicatedRequirements`]
    ///and [`MemoryDedicatedAllocateInfo`] to create (or import) a
    ///dedicated allocation for the image or buffer.
    pub const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY: Self = Self(1);
    ///[`ExternalMemoryFeatureExportable`] specifies that handles
    ///of this type  **can**  be exported from Vulkan memory objects.
    pub const EXTERNAL_MEMORY_FEATURE_EXPORTABLE: Self = Self(2);
    ///[`ExternalMemoryFeatureImportable`] specifies that handles
    ///of this type  **can**  be imported as Vulkan memory objects.
    pub const EXTERNAL_MEMORY_FEATURE_IMPORTABLE: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY
            | Self::EXTERNAL_MEMORY_FEATURE_EXPORTABLE
            | Self::EXTERNAL_MEMORY_FEATURE_IMPORTABLE
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
impl const std::ops::BitOr for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalMemoryFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalMemoryFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryFeatureFlags> for ExternalMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ExternalMemoryFeatureFlagBits>>::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryFeatureFlags> for ExternalMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlags>>(iterator: T) -> ExternalMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlagBits>>(iterator: T) -> ExternalMemoryFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryFeatureFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(ExternalMemoryFeatureFlags::EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryFeatureFlags::EXTERNAL_MEMORY_FEATURE_EXPORTABLE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_FEATURE_EXPORTABLE))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryFeatureFlags::EXTERNAL_MEMORY_FEATURE_IMPORTABLE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_FEATURE_IMPORTABLE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkExternalSemaphoreHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreHandleTypeFlagBits.html) - Bitmask of valid external semaphore handle types
///# C Specifications
///Bits which  **may**  be set in
///[`PhysicalDeviceExternalSemaphoreInfo::handle_type`], specifying an
///external semaphore handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalSemaphoreHandleTypeFlagBits {
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT = 0x00000001,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT = 0x00000002,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 0x00000004,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT = 0x00000008,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT = 0x00000010,
///  // Provided by VK_FUCHSIA_external_semaphore
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA = 0x00000080,
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT,
///} VkExternalSemaphoreHandleTypeFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_semaphore_capabilities
///typedef VkExternalSemaphoreHandleTypeFlagBits VkExternalSemaphoreHandleTypeFlagBitsKHR;
///```
///# Description
/// - [`ExternalSemaphoreHandleTypeOpaqueFd`] specifies a POSIX file descriptor handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible
///   with the POSIX system calls `dup`, `dup2`, `close`, and the non-standard system call `dup3`.
///   Additionally, it  **must**  be transportable over a socket using an `SCM_RIGHTS` control
///   message. It owns a reference to the underlying synchronization primitive represented by its
///   Vulkan semaphore object.
/// - [`ExternalSemaphoreHandleTypeOpaqueWin32`] specifies an NT handle that has only limited valid
///   usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the
///   functions `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
///   and `SetHandleInformation`. It owns a reference to the underlying synchronization primitive
///   represented by its Vulkan semaphore object.
/// - [`ExternalSemaphoreHandleTypeOpaqueWin32Kmt`] specifies a global share handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It is not compatible with any
///   native APIs. It does not own a reference to the underlying synchronization primitive
///   represented by its Vulkan semaphore object, and will therefore become invalid when all Vulkan
///   semaphore objects associated with it are destroyed.
/// - [`ExternalSemaphoreHandleTypeD3D12Fence`] specifies an NT handle returned by
///   `ID3D12Device`::`CreateSharedHandle` referring to a Direct3D 12 fence, or
///   `ID3D11Device5`::[`CreateFence`] referring to a Direct3D 11 fence. It owns a reference to the
///   underlying synchronization primitive associated with the Direct3D fence.
/// - [`ExternalSemaphoreHandleTypeD3D11Fence`] is an alias of
///   [`ExternalSemaphoreHandleTypeD3D12Fence`] with the same meaning. It is provided for
///   convenience and code clarity when interacting with D3D11 fences.
/// - [`ExternalSemaphoreHandleTypeSyncFd`] specifies a POSIX file descriptor handle to a Linux Sync
///   File or Android Fence object. It can be used with any native API accepting a valid sync file
///   or fence as input. It owns a reference to the underlying synchronization primitive associated
///   with the file descriptor. Implementations which support importing this handle type  **must**
///   accept any type of sync or fence FD supported by the native system they are running on.
/// - [`ZirconEventFuchsia`] specifies a handle to a Zircon event object. It can be used with any
///   native API that accepts a Zircon event handle. Zircon event handles are created with
///   `ZX_RIGHTS_BASIC` and `ZX_RIGHTS_SIGNAL` rights. Vulkan on Fuchsia uses only the
///   ZX_EVENT_SIGNALED bit when signaling or waiting.
///Some external semaphore handle types can only be shared within the same
///underlying physical device and/or the same driver version, as defined in the
///following table:
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalSemaphoreHandleTypeFlags`]
/// - [`ImportSemaphoreFdInfoKHR`]
/// - [`ImportSemaphoreWin32HandleInfoKHR`]
/// - [`ImportSemaphoreZirconHandleInfoFUCHSIA`]
/// - [`PhysicalDeviceExternalSemaphoreInfo`]
/// - [`SemaphoreGetFdInfoKHR`]
/// - [`SemaphoreGetWin32HandleInfoKHR`]
/// - [`SemaphoreGetZirconHandleInfoFUCHSIA`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalSemaphoreHandleTypeFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ExternalSemaphoreHandleTypeFlags(u32);
impl const Default for ExternalSemaphoreHandleTypeFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn from(from: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl ExternalSemaphoreHandleTypeFlags {
    ///[`ExternalSemaphoreHandleTypeOpaqueFd`] specifies a POSIX
    ///file descriptor handle that has only limited valid usage outside of
    ///Vulkan and other compatible APIs.
    ///It  **must**  be compatible with the POSIX system calls `dup`, `dup2`,
    ///`close`, and the non-standard system call `dup3`.
    ///Additionally, it  **must**  be transportable over a socket using an
    ///`SCM_RIGHTS` control message.
    ///It owns a reference to the underlying synchronization primitive
    ///represented by its Vulkan semaphore object.
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD: Self = Self(1);
    ///[`ExternalSemaphoreHandleTypeOpaqueWin32`] specifies an NT
    ///handle that has only limited valid usage outside of Vulkan and other
    ///compatible APIs.
    ///It  **must**  be compatible with the functions `DuplicateHandle`,
    ///`CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
    ///and `SetHandleInformation`.
    ///It owns a reference to the underlying synchronization primitive
    ///represented by its Vulkan semaphore object.
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN_32: Self = Self(2);
    ///[`ExternalSemaphoreHandleTypeOpaqueWin32Kmt`] specifies a
    ///global share handle that has only limited valid usage outside of Vulkan
    ///and other compatible APIs.
    ///It is not compatible with any native APIs.
    ///It does not own a reference to the underlying synchronization primitive
    ///represented by its Vulkan semaphore object, and will therefore become
    ///invalid when all Vulkan semaphore objects associated with it are
    ///destroyed.
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN_32_KMT: Self = Self(4);
    ///[`ExternalSemaphoreHandleTypeD3D12Fence`] specifies an NT
    ///handle returned by `ID3D12Device`::`CreateSharedHandle` referring
    ///to a Direct3D 12 fence, or `ID3D11Device5`::[`CreateFence`]
    ///referring to a Direct3D 11 fence.
    ///It owns a reference to the underlying synchronization primitive
    ///associated with the Direct3D fence.
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_D_3_D_12_FENCE: Self = Self(8);
    ///[`ExternalSemaphoreHandleTypeSyncFd`] specifies a POSIX
    ///file descriptor handle to a Linux Sync File or Android Fence object.
    ///It can be used with any native API accepting a valid sync file or fence
    ///as input.
    ///It owns a reference to the underlying synchronization primitive
    ///associated with the file descriptor.
    ///Implementations which support importing this handle type  **must**  accept
    ///any type of sync or fence FD supported by the native system they are
    ///running on.
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD: Self = Self(16);
    ///[`ZirconEventFuchsia`]
    ///specifies a handle to a Zircon event object.
    ///It can be used with any native API that accepts a Zircon event handle.
    ///Zircon event handles are created with `ZX_RIGHTS_BASIC` and
    ///`ZX_RIGHTS_SIGNAL` rights.
    ///Vulkan on Fuchsia uses only the ZX_EVENT_SIGNALED bit when signaling or
    ///waiting.
    ///
    ///Provided by [`crate::extensions::fuchsia_external_semaphore`]
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(128);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_extension_374`]
    pub const RESERVED_5_NV: Self = Self(32);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_extension_374`]
    pub const RESERVED_6_NV: Self = Self(64);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD
            | Self::EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN_32
            | Self::EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN_32_KMT
            | Self::EXTERNAL_SEMAPHORE_HANDLE_TYPE_D_3_D_12_FENCE
            | Self::EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD
            | Self::ZIRCON_EVENT_FUCHSIA
            | Self::RESERVED_5_NV
            | Self::RESERVED_6_NV
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
impl const std::ops::BitOr for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalSemaphoreHandleTypeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalSemaphoreHandleTypeFlags> for ExternalSemaphoreHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ExternalSemaphoreHandleTypeFlagBits>>::from(i));
        }
    }
}
impl FromIterator<ExternalSemaphoreHandleTypeFlags> for ExternalSemaphoreHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlags>>(
        iterator: T,
    ) -> ExternalSemaphoreHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreHandleTypeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreHandleTypeFlagBits>>(
        iterator: T,
    ) -> ExternalSemaphoreHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreHandleTypeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalSemaphoreHandleTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalSemaphoreHandleTypeFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalSemaphoreHandleTypeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD))?;
                    }
                    if self
                        .0
                        .contains(ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN_32)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN_32))?;
                    }
                    if self
                        .0
                        .contains(ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN_32_KMT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN_32_KMT))?;
                    }
                    if self
                        .0
                        .contains(ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_D_3_D_12_FENCE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_SEMAPHORE_HANDLE_TYPE_D_3_D_12_FENCE))?;
                    }
                    if self
                        .0
                        .contains(ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::ZIRCON_EVENT_FUCHSIA) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ZIRCON_EVENT_FUCHSIA))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::RESERVED_5_NV) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESERVED_5_NV))?;
                    }
                    if self.0.contains(ExternalSemaphoreHandleTypeFlags::RESERVED_6_NV) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESERVED_6_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalSemaphoreHandleTypeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkExternalSemaphoreFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreFeatureFlagBits.html) - Bitfield describing features of an external semaphore handle type
///# C Specifications
///Bits which  **may**  be set in
///[`ExternalSemaphoreProperties::external_semaphore_features`],
///specifying the features of an external semaphore handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalSemaphoreFeatureFlagBits {
///    VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT = 0x00000001,
///    VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT = 0x00000002,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT,
///  // Provided by VK_KHR_external_semaphore_capabilities
///    VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR =
/// VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT,
///} VkExternalSemaphoreFeatureFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_semaphore_capabilities
///typedef VkExternalSemaphoreFeatureFlagBits VkExternalSemaphoreFeatureFlagBitsKHR;
///```
///# Description
/// - [`ExternalSemaphoreFeatureExportable`] specifies that handles of this type  **can**  be
///   exported from Vulkan semaphore objects.
/// - [`ExternalSemaphoreFeatureImportable`] specifies that handles of this type  **can**  be
///   imported as Vulkan semaphore objects.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalSemaphoreFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalSemaphoreFeatureFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ExternalSemaphoreFeatureFlags(u32);
impl const Default for ExternalSemaphoreFeatureFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn from(from: ExternalSemaphoreFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl ExternalSemaphoreFeatureFlags {
    ///[`ExternalSemaphoreFeatureExportable`] specifies that
    ///handles of this type  **can**  be exported from Vulkan semaphore objects.
    pub const EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE: Self = Self(1);
    ///[`ExternalSemaphoreFeatureImportable`] specifies that
    ///handles of this type  **can**  be imported as Vulkan semaphore objects.
    pub const EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty() | Self::EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE | Self::EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE
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
impl const std::ops::BitOr for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalSemaphoreFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalSemaphoreFeatureFlags> for ExternalSemaphoreFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalSemaphoreFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ExternalSemaphoreFeatureFlagBits>>::from(i));
        }
    }
}
impl FromIterator<ExternalSemaphoreFeatureFlags> for ExternalSemaphoreFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreFeatureFlags>>(iterator: T) -> ExternalSemaphoreFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalSemaphoreFeatureFlagBits>>(
        iterator: T,
    ) -> ExternalSemaphoreFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalSemaphoreFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalSemaphoreFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalSemaphoreFeatureFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalSemaphoreFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(ExternalSemaphoreFeatureFlags::EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE))?;
                    }
                    if self
                        .0
                        .contains(ExternalSemaphoreFeatureFlags::EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalSemaphoreFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkSemaphoreImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSemaphoreImportFlagBits.html) - Bitmask specifying additional parameters of semaphore payload import
///# C Specifications
///Bits which  **can**  be set in
/// - [`ImportSemaphoreWin32HandleInfoKHR::flags`]
/// - [`ImportSemaphoreFdInfoKHR::flags`]
/// - [`ImportSemaphoreZirconHandleInfoFUCHSIA::flags`]
///specifying additional parameters of a semaphore import operation are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkSemaphoreImportFlagBits {
///    VK_SEMAPHORE_IMPORT_TEMPORARY_BIT = 0x00000001,
///  // Provided by VK_KHR_external_semaphore
///    VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR = VK_SEMAPHORE_IMPORT_TEMPORARY_BIT,
///} VkSemaphoreImportFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_semaphore
///typedef VkSemaphoreImportFlagBits VkSemaphoreImportFlagBitsKHR;
///```
///# Description
///These bits have the following meanings:
/// - [`SemaphoreImportTemporary`] specifies that the semaphore payload will be imported only temporarily, as described in [Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing), regardless of the permanence of `handleType`.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`SemaphoreImportFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSemaphoreImportFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct SemaphoreImportFlags(u32);
impl const Default for SemaphoreImportFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn from(from: SemaphoreImportFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl SemaphoreImportFlags {
    ///[`SemaphoreImportTemporary`] specifies that the semaphore
    ///payload will be imported only temporarily, as described in
    ///[Importing Semaphore Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-semaphores-importing),
    ///regardless of the permanence of `handleType`.
    pub const SEMAPHORE_IMPORT_TEMPORARY: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty() | Self::SEMAPHORE_IMPORT_TEMPORARY
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
impl const std::ops::BitOr for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for SemaphoreImportFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for SemaphoreImportFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for SemaphoreImportFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for SemaphoreImportFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SemaphoreImportFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SemaphoreImportFlags> for SemaphoreImportFlags {
    fn extend<T: IntoIterator<Item = SemaphoreImportFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn extend<T: IntoIterator<Item = SemaphoreImportFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<SemaphoreImportFlagBits>>::from(i));
        }
    }
}
impl FromIterator<SemaphoreImportFlags> for SemaphoreImportFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreImportFlags>>(iterator: T) -> SemaphoreImportFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreImportFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn from_iter<T: IntoIterator<Item = SemaphoreImportFlagBits>>(iterator: T) -> SemaphoreImportFlags {
        let mut out = Self::empty();
        <Self as Extend<SemaphoreImportFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SemaphoreImportFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SemaphoreImportFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SemaphoreImportFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SemaphoreImportFlags::SEMAPHORE_IMPORT_TEMPORARY) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SEMAPHORE_IMPORT_TEMPORARY))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SemaphoreImportFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkExternalFenceHandleTypeFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceHandleTypeFlagBits.html) - Bitmask of valid external fence handle types
///# C Specifications
///Bits which  **may**  be set in
///  * [`PhysicalDeviceExternalFenceInfo::handle_type`]
///  * [`ExternalFenceProperties::export_from_imported_handle_types`]
///  * [`ExternalFenceProperties::compatible_handle_types`]
///indicate external fence handle types, and are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalFenceHandleTypeFlagBits {
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT = 0x00000001,
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT = 0x00000002,
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 0x00000004,
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT = 0x00000008,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR =
/// VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR =
/// VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR =
/// VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR = VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT,
///} VkExternalFenceHandleTypeFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence_capabilities
///typedef VkExternalFenceHandleTypeFlagBits VkExternalFenceHandleTypeFlagBitsKHR;
///```
///# Description
/// - [`ExternalFenceHandleTypeOpaqueFd`] specifies a POSIX file descriptor handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It  **must**  be compatible
///   with the POSIX system calls `dup`, `dup2`, `close`, and the non-standard system call `dup3`.
///   Additionally, it  **must**  be transportable over a socket using an `SCM_RIGHTS` control
///   message. It owns a reference to the underlying synchronization primitive represented by its
///   Vulkan fence object.
/// - [`ExternalFenceHandleTypeOpaqueWin32`] specifies an NT handle that has only limited valid
///   usage outside of Vulkan and other compatible APIs. It  **must**  be compatible with the
///   functions `DuplicateHandle`, `CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
///   and `SetHandleInformation`. It owns a reference to the underlying synchronization primitive
///   represented by its Vulkan fence object.
/// - [`ExternalFenceHandleTypeOpaqueWin32Kmt`] specifies a global share handle that has only
///   limited valid usage outside of Vulkan and other compatible APIs. It is not compatible with any
///   native APIs. It does not own a reference to the underlying synchronization primitive
///   represented by its Vulkan fence object, and will therefore become invalid when all Vulkan
///   fence objects associated with it are destroyed.
/// - [`ExternalFenceHandleTypeSyncFd`] specifies a POSIX file descriptor handle to a Linux Sync
///   File or Android Fence. It can be used with any native API accepting a valid sync file or fence
///   as input. It owns a reference to the underlying synchronization primitive associated with the
///   file descriptor. Implementations which support importing this handle type  **must**  accept
///   any type of sync or fence FD supported by the native system they are running on.
///Some external fence handle types can only be shared within the same
///underlying physical device and/or the same driver version, as defined in the
///following table:
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalFenceHandleTypeFlags`]
/// - [`FenceGetFdInfoKHR`]
/// - [`FenceGetWin32HandleInfoKHR`]
/// - [`ImportFenceFdInfoKHR`]
/// - [`ImportFenceWin32HandleInfoKHR`]
/// - [`PhysicalDeviceExternalFenceInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalFenceHandleTypeFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ExternalFenceHandleTypeFlags(u32);
impl const Default for ExternalFenceHandleTypeFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn from(from: ExternalFenceHandleTypeFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl ExternalFenceHandleTypeFlags {
    ///[`ExternalFenceHandleTypeOpaqueFd`] specifies a POSIX file
    ///descriptor handle that has only limited valid usage outside of Vulkan
    ///and other compatible APIs.
    ///It  **must**  be compatible with the POSIX system calls `dup`, `dup2`,
    ///`close`, and the non-standard system call `dup3`.
    ///Additionally, it  **must**  be transportable over a socket using an
    ///`SCM_RIGHTS` control message.
    ///It owns a reference to the underlying synchronization primitive
    ///represented by its Vulkan fence object.
    pub const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD: Self = Self(1);
    ///[`ExternalFenceHandleTypeOpaqueWin32`] specifies an NT
    ///handle that has only limited valid usage outside of Vulkan and other
    ///compatible APIs.
    ///It  **must**  be compatible with the functions `DuplicateHandle`,
    ///`CloseHandle`, `CompareObjectHandles`, `GetHandleInformation`,
    ///and `SetHandleInformation`.
    ///It owns a reference to the underlying synchronization primitive
    ///represented by its Vulkan fence object.
    pub const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN_32: Self = Self(2);
    ///[`ExternalFenceHandleTypeOpaqueWin32Kmt`] specifies a
    ///global share handle that has only limited valid usage outside of Vulkan
    ///and other compatible APIs.
    ///It is not compatible with any native APIs.
    ///It does not own a reference to the underlying synchronization primitive
    ///represented by its Vulkan fence object, and will therefore become
    ///invalid when all Vulkan fence objects associated with it are destroyed.
    pub const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN_32_KMT: Self = Self(4);
    ///[`ExternalFenceHandleTypeSyncFd`] specifies a POSIX file
    ///descriptor handle to a Linux Sync File or Android Fence.
    ///It can be used with any native API accepting a valid sync file or fence
    ///as input.
    ///It owns a reference to the underlying synchronization primitive
    ///associated with the file descriptor.
    ///Implementations which support importing this handle type  **must**  accept
    ///any type of sync or fence FD supported by the native system they are
    ///running on.
    pub const EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD: Self = Self(8);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_extension_374`]
    pub const RESERVED_4_NV: Self = Self(16);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_extension_374`]
    pub const RESERVED_5_NV: Self = Self(32);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD
            | Self::EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN_32
            | Self::EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN_32_KMT
            | Self::EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD
            | Self::RESERVED_4_NV
            | Self::RESERVED_5_NV
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
impl const std::ops::BitOr for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalFenceHandleTypeFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalFenceHandleTypeFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalFenceHandleTypeFlags> for ExternalFenceHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceHandleTypeFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceHandleTypeFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ExternalFenceHandleTypeFlagBits>>::from(i));
        }
    }
}
impl FromIterator<ExternalFenceHandleTypeFlags> for ExternalFenceHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceHandleTypeFlags>>(iterator: T) -> ExternalFenceHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceHandleTypeFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceHandleTypeFlagBits>>(iterator: T) -> ExternalFenceHandleTypeFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceHandleTypeFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalFenceHandleTypeFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalFenceHandleTypeFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalFenceHandleTypeFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(ExternalFenceHandleTypeFlags::EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD))?;
                    }
                    if self
                        .0
                        .contains(ExternalFenceHandleTypeFlags::EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN_32)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN_32))?;
                    }
                    if self
                        .0
                        .contains(ExternalFenceHandleTypeFlags::EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN_32_KMT)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN_32_KMT))?;
                    }
                    if self
                        .0
                        .contains(ExternalFenceHandleTypeFlags::EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD))?;
                    }
                    if self.0.contains(ExternalFenceHandleTypeFlags::RESERVED_4_NV) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESERVED_4_NV))?;
                    }
                    if self.0.contains(ExternalFenceHandleTypeFlags::RESERVED_5_NV) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(RESERVED_5_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalFenceHandleTypeFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkExternalFenceFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceFeatureFlagBits.html) - Bitfield describing features of an external fence handle type
///# C Specifications
///Bits which  **may**  be set in
///[`ExternalFenceProperties::external_fence_features`], indicating
///features of a fence external handle type, are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkExternalFenceFeatureFlagBits {
///    VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT = 0x00000001,
///    VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT = 0x00000002,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR = VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT,
///  // Provided by VK_KHR_external_fence_capabilities
///    VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR = VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT,
///} VkExternalFenceFeatureFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence_capabilities
///typedef VkExternalFenceFeatureFlagBits VkExternalFenceFeatureFlagBitsKHR;
///```
///# Description
/// - [`ExternalFenceFeatureExportable`] specifies handles of this type  **can**  be exported from
///   Vulkan fence objects.
/// - [`ExternalFenceFeatureImportable`] specifies handles of this type  **can**  be imported to
///   Vulkan fence objects.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalFenceFeatureFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalFenceFeatureFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ExternalFenceFeatureFlags(u32);
impl const Default for ExternalFenceFeatureFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn from(from: ExternalFenceFeatureFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl ExternalFenceFeatureFlags {
    ///[`ExternalFenceFeatureExportable`] specifies handles of this
    ///type  **can**  be exported from Vulkan fence objects.
    pub const EXTERNAL_FENCE_FEATURE_EXPORTABLE: Self = Self(1);
    ///[`ExternalFenceFeatureImportable`] specifies handles of this
    ///type  **can**  be imported to Vulkan fence objects.
    pub const EXTERNAL_FENCE_FEATURE_IMPORTABLE: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty() | Self::EXTERNAL_FENCE_FEATURE_EXPORTABLE | Self::EXTERNAL_FENCE_FEATURE_IMPORTABLE
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
impl const std::ops::BitOr for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for ExternalFenceFeatureFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalFenceFeatureFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalFenceFeatureFlags> for ExternalFenceFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceFeatureFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn extend<T: IntoIterator<Item = ExternalFenceFeatureFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ExternalFenceFeatureFlagBits>>::from(i));
        }
    }
}
impl FromIterator<ExternalFenceFeatureFlags> for ExternalFenceFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceFeatureFlags>>(iterator: T) -> ExternalFenceFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceFeatureFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn from_iter<T: IntoIterator<Item = ExternalFenceFeatureFlagBits>>(iterator: T) -> ExternalFenceFeatureFlags {
        let mut out = Self::empty();
        <Self as Extend<ExternalFenceFeatureFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalFenceFeatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalFenceFeatureFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalFenceFeatureFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(ExternalFenceFeatureFlags::EXTERNAL_FENCE_FEATURE_EXPORTABLE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_FENCE_FEATURE_EXPORTABLE))?;
                    }
                    if self
                        .0
                        .contains(ExternalFenceFeatureFlags::EXTERNAL_FENCE_FEATURE_IMPORTABLE)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_FENCE_FEATURE_IMPORTABLE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalFenceFeatureFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkFenceImportFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFenceImportFlagBits.html) - Bitmask specifying additional parameters of fence payload import
///# C Specifications
///Bits which  **can**  be set in
/// - [`ImportFenceWin32HandleInfoKHR::flags`]
/// - [`ImportFenceFdInfoKHR::flags`]
///specifying additional parameters of a fence import operation are:
///```c
///// Provided by VK_VERSION_1_1
///typedef enum VkFenceImportFlagBits {
///    VK_FENCE_IMPORT_TEMPORARY_BIT = 0x00000001,
///  // Provided by VK_KHR_external_fence
///    VK_FENCE_IMPORT_TEMPORARY_BIT_KHR = VK_FENCE_IMPORT_TEMPORARY_BIT,
///} VkFenceImportFlagBits;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence
///typedef VkFenceImportFlagBits VkFenceImportFlagBitsKHR;
///```
///# Description
/// - [`FenceImportTemporary`] specifies that the fence payload will be imported only temporarily, as described in [Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing), regardless of the permanence of `handleType`.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`FenceImportFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFenceImportFlags")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct FenceImportFlags(u32);
impl const Default for FenceImportFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl From<FenceImportFlagBits> for FenceImportFlags {
    fn from(from: FenceImportFlagBits) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl FenceImportFlags {
    ///[`FenceImportTemporary`] specifies that the fence payload
    ///will be imported only temporarily, as described in
    ///[Importing Fence Payloads](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-fences-importing),
    ///regardless of the permanence of `handleType`.
    pub const FENCE_IMPORT_TEMPORARY: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty() | Self::FENCE_IMPORT_TEMPORARY
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
impl const std::ops::BitOr for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for FenceImportFlags {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for FenceImportFlags {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for FenceImportFlags {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for FenceImportFlags {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for FenceImportFlags {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<FenceImportFlags> for FenceImportFlags {
    fn extend<T: IntoIterator<Item = FenceImportFlags>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<FenceImportFlagBits> for FenceImportFlags {
    fn extend<T: IntoIterator<Item = FenceImportFlagBits>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<FenceImportFlagBits>>::from(i));
        }
    }
}
impl FromIterator<FenceImportFlags> for FenceImportFlags {
    fn from_iter<T: IntoIterator<Item = FenceImportFlags>>(iterator: T) -> FenceImportFlags {
        let mut out = Self::empty();
        <Self as Extend<FenceImportFlags>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<FenceImportFlagBits> for FenceImportFlags {
    fn from_iter<T: IntoIterator<Item = FenceImportFlagBits>>(iterator: T) -> FenceImportFlags {
        let mut out = Self::empty();
        <Self as Extend<FenceImportFlagBits>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for FenceImportFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(FenceImportFlags);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == FenceImportFlags::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(FenceImportFlags::FENCE_IMPORT_TEMPORARY) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(FENCE_IMPORT_TEMPORARY))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(FenceImportFlags))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkPhysicalDeviceFeatures2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFeatures2.html) - Structure describing the fine-grained features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFeatures2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceFeatures2 {
///    VkStructureType             sType;
///    void*                       pNext;
///    VkPhysicalDeviceFeatures    features;
///} VkPhysicalDeviceFeatures2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_physical_device_properties2
///typedef VkPhysicalDeviceFeatures2 VkPhysicalDeviceFeatures2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`features`] is a [`PhysicalDeviceFeatures`] structure describing the fine-grained features of
///   the Vulkan 1.0 API.
///# Description
///The [`p_next`] chain of this structure is used to extend the structure with
///features defined by extensions.
///This structure  **can**  be used in [`GetPhysicalDeviceFeatures2`] or  **can**  be
///included in the [`p_next`] chain of a [`DeviceCreateInfo`] structure,
///in which case it controls which features are enabled in the device in lieu
///of `pEnabledFeatures`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`PhysicalDeviceFeatures`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceFeatures2`]
/// - [`GetPhysicalDeviceFeatures2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceFeatures2")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFeatures2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`features`] is a [`PhysicalDeviceFeatures`] structure describing
    ///the fine-grained features of the Vulkan 1.0 API.
    pub features: PhysicalDeviceFeatures,
}
impl<'lt> Default for PhysicalDeviceFeatures2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceFeatures2,
            p_next: std::ptr::null_mut(),
            features: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceFeatures2<'lt> {
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
    ///Gets the value of [`Self::features`]
    pub fn features(&self) -> PhysicalDeviceFeatures {
        self.features
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
    ///Gets a mutable reference to the value of [`Self::features`]
    pub fn features_mut(&mut self) -> &mut PhysicalDeviceFeatures {
        &mut self.features
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
    ///Sets the raw value of [`Self::features`]
    pub fn set_features(&mut self, value: crate::vulkan1_0::PhysicalDeviceFeatures) -> &mut Self {
        self.features = value;
        self
    }
}
///[VkPhysicalDeviceProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties2.html) - Structure specifying physical device properties
///# C Specifications
///The [`PhysicalDeviceProperties2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceProperties2 {
///    VkStructureType               sType;
///    void*                         pNext;
///    VkPhysicalDeviceProperties    properties;
///} VkPhysicalDeviceProperties2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_physical_device_properties2
///typedef VkPhysicalDeviceProperties2 VkPhysicalDeviceProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`properties`] is a [`PhysicalDeviceProperties`] structure describing properties of the
///   physical device. This structure is written with the same values as if it were written by
///   [`GetPhysicalDeviceProperties`].
///# Description
///The [`p_next`] chain of this structure is used to extend the structure with
///properties defined by extensions.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of
///   [`PhysicalDeviceAccelerationStructurePropertiesKHR`],
///   [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`],
///   [`PhysicalDeviceConservativeRasterizationPropertiesEXT`],
///   [`PhysicalDeviceCooperativeMatrixPropertiesNV`],
///   [`PhysicalDeviceCustomBorderColorPropertiesEXT`],
///   [`PhysicalDeviceDepthStencilResolveProperties`],
///   [`PhysicalDeviceDescriptorIndexingProperties`],
///   [`PhysicalDeviceDeviceGeneratedCommandsPropertiesNV`],
///   [`PhysicalDeviceDiscardRectanglePropertiesEXT`], [`PhysicalDeviceDriverProperties`],
///   [`PhysicalDeviceDrmPropertiesEXT`], [`PhysicalDeviceExternalMemoryHostPropertiesEXT`],
///   [`PhysicalDeviceFloatControlsProperties`], [`PhysicalDeviceFragmentDensityMap2PropertiesEXT`],
///   [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`],
///   [`PhysicalDeviceFragmentDensityMapPropertiesEXT`],
///   [`PhysicalDeviceFragmentShadingRateEnumsPropertiesNV`],
///   [`PhysicalDeviceFragmentShadingRatePropertiesKHR`], [`PhysicalDeviceIdProperties`],
///   [`PhysicalDeviceInlineUniformBlockProperties`],
///   [`PhysicalDeviceLineRasterizationPropertiesEXT`], [`PhysicalDeviceMaintenance3Properties`],
///   [`PhysicalDeviceMaintenance4Properties`], [`PhysicalDeviceMeshShaderPropertiesNV`],
///   [`PhysicalDeviceMultiDrawPropertiesEXT`],
///   [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`],
///   [`PhysicalDeviceMultiviewProperties`], [`PhysicalDevicePciBusInfoPropertiesEXT`],
///   [`PhysicalDevicePerformanceQueryPropertiesKHR`], [`PhysicalDevicePointClippingProperties`],
///   [`PhysicalDevicePortabilitySubsetPropertiesKHR`], [`PhysicalDeviceProtectedMemoryProperties`],
///   [`PhysicalDeviceProvokingVertexPropertiesEXT`], [`PhysicalDevicePushDescriptorPropertiesKHR`],
///   [`PhysicalDeviceRayTracingPipelinePropertiesKHR`], [`PhysicalDeviceRayTracingPropertiesNV`],
///   [`PhysicalDeviceRobustness2PropertiesEXT`], [`PhysicalDeviceSampleLocationsPropertiesEXT`],
///   [`PhysicalDeviceSamplerFilterMinmaxProperties`], [`PhysicalDeviceShaderCoreProperties2AMD`],
///   [`PhysicalDeviceShaderCorePropertiesAMD`],
///   [`PhysicalDeviceShaderIntegerDotProductProperties`],
///   [`PhysicalDeviceShaderSmBuiltinsPropertiesNV`],
///   [`PhysicalDeviceShadingRateImagePropertiesNV`], [`PhysicalDeviceSubgroupProperties`],
///   [`PhysicalDeviceSubgroupSizeControlProperties`],
///   [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`],
///   [`PhysicalDeviceTexelBufferAlignmentProperties`],
///   [`PhysicalDeviceTimelineSemaphoreProperties`],
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT`],
///   [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`], [`PhysicalDeviceVulkan11Properties`],
///   [`PhysicalDeviceVulkan12Properties`], or [`PhysicalDeviceVulkan13Properties`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`crate::vulkan1_1`]
/// - [`PhysicalDeviceProperties`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceProperties2`]
/// - [`GetPhysicalDeviceProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceProperties2")]
#[derive(Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceProperties2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`properties`] is a [`PhysicalDeviceProperties`] structure
    ///describing properties of the physical device.
    ///This structure is written with the same values as if it were written by
    ///[`GetPhysicalDeviceProperties`].
    pub properties: PhysicalDeviceProperties,
}
impl<'lt> Default for PhysicalDeviceProperties2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceProperties2,
            p_next: std::ptr::null_mut(),
            properties: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceProperties2<'lt> {
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
    ///Gets the value of [`Self::properties`]
    pub fn properties(&self) -> PhysicalDeviceProperties {
        self.properties
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
    ///Gets a mutable reference to the value of [`Self::properties`]
    pub fn properties_mut(&mut self) -> &mut PhysicalDeviceProperties {
        &mut self.properties
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
    ///Sets the raw value of [`Self::properties`]
    pub fn set_properties(&mut self, value: crate::vulkan1_0::PhysicalDeviceProperties) -> &mut Self {
        self.properties = value;
        self
    }
}
///[VkFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFormatProperties2.html) - Structure specifying image format properties
///# C Specifications
///The [`FormatProperties2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkFormatProperties2 {
///    VkStructureType       sType;
///    void*                 pNext;
///    VkFormatProperties    formatProperties;
///} VkFormatProperties2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_physical_device_properties2
///typedef VkFormatProperties2 VkFormatProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format_properties`] is a [`FormatProperties`] structure describing features supported by the
///   requested format.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`DrmFormatModifierPropertiesList2EXT`],
///   [`DrmFormatModifierPropertiesListEXT`], [`FormatProperties3`], [`VideoDecodeH264ProfileEXT`],
///   [`VideoDecodeH265ProfileEXT`], [`VideoEncodeH264ProfileEXT`], [`VideoEncodeH265ProfileEXT`],
///   [`VideoProfileKHR`], or [`VideoProfilesKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`crate::vulkan1_1`]
/// - [`FormatProperties`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceFormatProperties2`]
/// - [`GetPhysicalDeviceFormatProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFormatProperties2")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct FormatProperties2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`format_properties`] is a [`FormatProperties`] structure
    ///describing features supported by the requested format.
    pub format_properties: FormatProperties,
}
impl<'lt> Default for FormatProperties2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::FormatProperties2,
            p_next: std::ptr::null_mut(),
            format_properties: Default::default(),
        }
    }
}
impl<'lt> FormatProperties2<'lt> {
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
    ///Gets the value of [`Self::format_properties`]
    pub fn format_properties(&self) -> FormatProperties {
        self.format_properties
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
    ///Gets a mutable reference to the value of [`Self::format_properties`]
    pub fn format_properties_mut(&mut self) -> &mut FormatProperties {
        &mut self.format_properties
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
    ///Sets the raw value of [`Self::format_properties`]
    pub fn set_format_properties(&mut self, value: crate::vulkan1_0::FormatProperties) -> &mut Self {
        self.format_properties = value;
        self
    }
}
///[VkImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatProperties2.html) - Structure specifying an image format properties
///# C Specifications
///The [`ImageFormatProperties2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkImageFormatProperties2 {
///    VkStructureType            sType;
///    void*                      pNext;
///    VkImageFormatProperties    imageFormatProperties;
///} VkImageFormatProperties2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_physical_device_properties2
///typedef VkImageFormatProperties2 VkImageFormatProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure. The [`p_next`]
///   chain of [`ImageFormatProperties2`] is used to allow the specification of additional
///   capabilities to be returned from [`GetPhysicalDeviceImageFormatProperties2`].
/// - [`image_format_properties`] is a [`ImageFormatProperties`] structure in which capabilities are
///   returned.
///# Description
///If the combination of parameters to
///[`GetPhysicalDeviceImageFormatProperties2`] is not supported by the
///implementation for use in [`CreateImage`], then all members of
///[`image_format_properties`] will be filled with zero.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`AndroidHardwareBufferUsageANDROID`],
///   [`ExternalImageFormatProperties`], [`FilterCubicImageViewImageFormatPropertiesEXT`],
///   [`SamplerYcbcrConversionImageFormatProperties`], or [`TextureLodGatherFormatPropertiesAMD`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ImageFormatProperties`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceImageFormatProperties2`]
/// - [`GetPhysicalDeviceImageFormatProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageFormatProperties2")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageFormatProperties2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    ///The [`p_next`] chain of [`ImageFormatProperties2`] is used to allow
    ///the specification of additional capabilities to be returned from
    ///[`GetPhysicalDeviceImageFormatProperties2`].
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`image_format_properties`] is a [`ImageFormatProperties`] structure
    ///in which capabilities are returned.
    pub image_format_properties: ImageFormatProperties,
}
impl<'lt> Default for ImageFormatProperties2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ImageFormatProperties2,
            p_next: std::ptr::null_mut(),
            image_format_properties: Default::default(),
        }
    }
}
impl<'lt> ImageFormatProperties2<'lt> {
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
    ///Gets the value of [`Self::image_format_properties`]
    pub fn image_format_properties(&self) -> ImageFormatProperties {
        self.image_format_properties
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
    ///Gets a mutable reference to the value of [`Self::image_format_properties`]
    pub fn image_format_properties_mut(&mut self) -> &mut ImageFormatProperties {
        &mut self.image_format_properties
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
    ///Sets the raw value of [`Self::image_format_properties`]
    pub fn set_image_format_properties(&mut self, value: crate::vulkan1_0::ImageFormatProperties) -> &mut Self {
        self.image_format_properties = value;
        self
    }
}
///[VkPhysicalDeviceImageFormatInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageFormatInfo2.html) - Structure specifying image creation parameters
///# C Specifications
///The [`PhysicalDeviceImageFormatInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceImageFormatInfo2 {
///    VkStructureType       sType;
///    const void*           pNext;
///    VkFormat              format;
///    VkImageType           type;
///    VkImageTiling         tiling;
///    VkImageUsageFlags     usage;
///    VkImageCreateFlags    flags;
///} VkPhysicalDeviceImageFormatInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_physical_device_properties2
///typedef VkPhysicalDeviceImageFormatInfo2 VkPhysicalDeviceImageFormatInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure. The [`p_next`]
///   chain of [`PhysicalDeviceImageFormatInfo2`] is used to provide additional image parameters to
///   [`GetPhysicalDeviceImageFormatProperties2`].
/// - [`format`] is a [`Format`] value indicating the image format, corresponding to
///   [`ImageCreateInfo`]::[`format`].
/// - [`type_`] is a [`ImageType`] value indicating the image type, corresponding to
///   [`ImageCreateInfo::image_type`].
/// - [`tiling`] is a [`ImageTiling`] value indicating the image tiling, corresponding to
///   [`ImageCreateInfo`]::[`tiling`].
/// - [`usage`] is a bitmask of [`ImageUsageFlagBits`] indicating the intended usage of the image,
///   corresponding to [`ImageCreateInfo`]::[`usage`].
/// - [`flags`] is a bitmask of [`ImageCreateFlagBits`] indicating additional parameters of the
///   image, corresponding to [`ImageCreateInfo`]::[`flags`].
///# Description
///The members of [`PhysicalDeviceImageFormatInfo2`] correspond to the
///arguments to [`GetPhysicalDeviceImageFormatProperties`], with
///[`s_type`] and [`p_next`] added for extensibility.
///## Valid Usage
/// - [`tiling`] **must**  be `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` if and only if the
///   [`p_next`] chain includes [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
/// - If [`tiling`] is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` and [`flags`] contains
///   `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT`, then the [`p_next`] chain  **must**  include a
///   [`ImageFormatListCreateInfo`] structure with non-zero `viewFormatCount`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`ImageFormatListCreateInfo`],
///   [`ImageStencilUsageCreateInfo`], [`PhysicalDeviceExternalImageFormatInfo`],
///   [`PhysicalDeviceImageDrmFormatModifierInfoEXT`], or
///   [`PhysicalDeviceImageViewImageFormatInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`format`] **must**  be a valid [`Format`] value
/// - [`type_`] **must**  be a valid [`ImageType`] value
/// - [`tiling`] **must**  be a valid [`ImageTiling`] value
/// - [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
/// - [`usage`] **must**  not be `0`
/// - [`flags`] **must**  be a valid combination of [`ImageCreateFlagBits`] values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Format`]
/// - [`ImageCreateFlags`]
/// - [`ImageTiling`]
/// - [`ImageType`]
/// - [`ImageUsageFlags`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceImageFormatProperties2`]
/// - [`GetPhysicalDeviceImageFormatProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    ///The [`p_next`] chain of [`PhysicalDeviceImageFormatInfo2`] is used
    ///to provide additional image parameters to
    ///[`GetPhysicalDeviceImageFormatProperties2`].
    pub p_next: *const BaseInStructure<'lt>,
    ///[`format`] is a [`Format`] value indicating the image format,
    ///corresponding to [`ImageCreateInfo`]::[`format`].
    pub format: Format,
    ///[`type_`] is a [`ImageType`] value indicating the image type,
    ///corresponding to [`ImageCreateInfo`]::`imageType`.
    pub type_: ImageType,
    ///[`tiling`] is a [`ImageTiling`] value indicating the image tiling,
    ///corresponding to [`ImageCreateInfo`]::[`tiling`].
    pub tiling: ImageTiling,
    ///[`usage`] is a bitmask of [`ImageUsageFlagBits`] indicating the
    ///intended usage of the image, corresponding to
    ///[`ImageCreateInfo`]::[`usage`].
    pub usage: ImageUsageFlags,
    ///[`flags`] is a bitmask of [`ImageCreateFlagBits`] indicating
    ///additional parameters of the image, corresponding to
    ///[`ImageCreateInfo`]::[`flags`].
    pub flags: ImageCreateFlags,
}
impl<'lt> Default for PhysicalDeviceImageFormatInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceImageFormatInfo2,
            p_next: std::ptr::null(),
            format: Default::default(),
            type_: Default::default(),
            tiling: Default::default(),
            usage: Default::default(),
            flags: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceImageFormatInfo2<'lt> {
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
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> ImageType {
        self.type_
    }
    ///Gets the value of [`Self::tiling`]
    pub fn tiling(&self) -> ImageTiling {
        self.tiling
    }
    ///Gets the value of [`Self::usage`]
    pub fn usage(&self) -> ImageUsageFlags {
        self.usage
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> ImageCreateFlags {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut ImageType {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::tiling`]
    pub fn tiling_mut(&mut self) -> &mut ImageTiling {
        &mut self.tiling
    }
    ///Gets a mutable reference to the value of [`Self::usage`]
    pub fn usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.usage
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut ImageCreateFlags {
        &mut self.flags
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
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the raw value of [`Self::type_`]
    pub fn set_type_(&mut self, value: crate::vulkan1_0::ImageType) -> &mut Self {
        self.type_ = value;
        self
    }
    ///Sets the raw value of [`Self::tiling`]
    pub fn set_tiling(&mut self, value: crate::vulkan1_0::ImageTiling) -> &mut Self {
        self.tiling = value;
        self
    }
    ///Sets the raw value of [`Self::usage`]
    pub fn set_usage(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.usage = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::vulkan1_0::ImageCreateFlags) -> &mut Self {
        self.flags = value;
        self
    }
}
///[VkQueueFamilyProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyProperties2.html) - Structure providing information about a queue family
///# C Specifications
///The [`QueueFamilyProperties2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkQueueFamilyProperties2 {
///    VkStructureType            sType;
///    void*                      pNext;
///    VkQueueFamilyProperties    queueFamilyProperties;
///} VkQueueFamilyProperties2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_physical_device_properties2
///typedef VkQueueFamilyProperties2 VkQueueFamilyProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`queue_family_properties`] is a [`QueueFamilyProperties`] structure which is populated with
///   the same values as in [`GetPhysicalDeviceQueueFamilyProperties`].
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`QueueFamilyCheckpointProperties2NV`],
///   [`QueueFamilyCheckpointPropertiesNV`], [`QueueFamilyGlobalPriorityPropertiesKHR`],
///   [`QueueFamilyQueryResultStatusProperties2KHR`], or [`VideoQueueFamilyProperties2KHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`crate::vulkan1_1`]
/// - [`QueueFamilyProperties`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceQueueFamilyProperties2`]
/// - [`GetPhysicalDeviceQueueFamilyProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueueFamilyProperties2")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct QueueFamilyProperties2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`queue_family_properties`] is a [`QueueFamilyProperties`] structure
    ///which is populated with the same values as in
    ///[`GetPhysicalDeviceQueueFamilyProperties`].
    pub queue_family_properties: QueueFamilyProperties,
}
impl<'lt> Default for QueueFamilyProperties2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::QueueFamilyProperties2,
            p_next: std::ptr::null_mut(),
            queue_family_properties: Default::default(),
        }
    }
}
impl<'lt> QueueFamilyProperties2<'lt> {
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
    ///Gets the value of [`Self::queue_family_properties`]
    pub fn queue_family_properties(&self) -> QueueFamilyProperties {
        self.queue_family_properties
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
    ///Gets a mutable reference to the value of [`Self::queue_family_properties`]
    pub fn queue_family_properties_mut(&mut self) -> &mut QueueFamilyProperties {
        &mut self.queue_family_properties
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
    ///Sets the raw value of [`Self::queue_family_properties`]
    pub fn set_queue_family_properties(&mut self, value: crate::vulkan1_0::QueueFamilyProperties) -> &mut Self {
        self.queue_family_properties = value;
        self
    }
}
///[VkPhysicalDeviceMemoryProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryProperties2.html) - Structure specifying physical device memory properties
///# C Specifications
///The [`PhysicalDeviceMemoryProperties2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceMemoryProperties2 {
///    VkStructureType                     sType;
///    void*                               pNext;
///    VkPhysicalDeviceMemoryProperties    memoryProperties;
///} VkPhysicalDeviceMemoryProperties2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_physical_device_properties2
///typedef VkPhysicalDeviceMemoryProperties2 VkPhysicalDeviceMemoryProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_properties`] is a [`PhysicalDeviceMemoryProperties`] structure which is populated
///   with the same values as in [`GetPhysicalDeviceMemoryProperties`].
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`PhysicalDeviceMemoryBudgetPropertiesEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`crate::vulkan1_1`]
/// - [`PhysicalDeviceMemoryProperties`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceMemoryProperties2`]
/// - [`GetPhysicalDeviceMemoryProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceMemoryProperties2")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`memory_properties`] is a [`PhysicalDeviceMemoryProperties`]
    ///structure which is populated with the same values as in
    ///[`GetPhysicalDeviceMemoryProperties`].
    pub memory_properties: PhysicalDeviceMemoryProperties,
}
impl<'lt> Default for PhysicalDeviceMemoryProperties2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceMemoryProperties2,
            p_next: std::ptr::null_mut(),
            memory_properties: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceMemoryProperties2<'lt> {
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
    ///Gets the value of [`Self::memory_properties`]
    pub fn memory_properties(&self) -> PhysicalDeviceMemoryProperties {
        self.memory_properties
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
    ///Gets a mutable reference to the value of [`Self::memory_properties`]
    pub fn memory_properties_mut(&mut self) -> &mut PhysicalDeviceMemoryProperties {
        &mut self.memory_properties
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
    ///Sets the raw value of [`Self::memory_properties`]
    pub fn set_memory_properties(&mut self, value: crate::vulkan1_0::PhysicalDeviceMemoryProperties) -> &mut Self {
        self.memory_properties = value;
        self
    }
}
///[VkSparseImageFormatProperties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageFormatProperties2.html) - Structure specifying sparse image format properties
///# C Specifications
///The [`SparseImageFormatProperties2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkSparseImageFormatProperties2 {
///    VkStructureType                  sType;
///    void*                            pNext;
///    VkSparseImageFormatProperties    properties;
///} VkSparseImageFormatProperties2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_physical_device_properties2
///typedef VkSparseImageFormatProperties2 VkSparseImageFormatProperties2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`properties`] is a [`SparseImageFormatProperties`] structure which is populated with the same
///   values as in [`GetPhysicalDeviceSparseImageFormatProperties`].
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`SparseImageFormatProperties`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceSparseImageFormatProperties2`]
/// - [`GetPhysicalDeviceSparseImageFormatProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSparseImageFormatProperties2")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SparseImageFormatProperties2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`properties`] is a [`SparseImageFormatProperties`] structure
    ///which is populated with the same values as in
    ///[`GetPhysicalDeviceSparseImageFormatProperties`].
    pub properties: SparseImageFormatProperties,
}
impl<'lt> Default for SparseImageFormatProperties2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SparseImageFormatProperties2,
            p_next: std::ptr::null_mut(),
            properties: Default::default(),
        }
    }
}
impl<'lt> SparseImageFormatProperties2<'lt> {
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
    ///Gets the value of [`Self::properties`]
    pub fn properties(&self) -> SparseImageFormatProperties {
        self.properties
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
    ///Gets a mutable reference to the value of [`Self::properties`]
    pub fn properties_mut(&mut self) -> &mut SparseImageFormatProperties {
        &mut self.properties
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
    ///Sets the raw value of [`Self::properties`]
    pub fn set_properties(&mut self, value: crate::vulkan1_0::SparseImageFormatProperties) -> &mut Self {
        self.properties = value;
        self
    }
}
///[VkPhysicalDeviceSparseImageFormatInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSparseImageFormatInfo2.html) - Structure specifying sparse image format inputs
///# C Specifications
///The [`PhysicalDeviceSparseImageFormatInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceSparseImageFormatInfo2 {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkFormat                 format;
///    VkImageType              type;
///    VkSampleCountFlagBits    samples;
///    VkImageUsageFlags        usage;
///    VkImageTiling            tiling;
///} VkPhysicalDeviceSparseImageFormatInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_physical_device_properties2
///typedef VkPhysicalDeviceSparseImageFormatInfo2 VkPhysicalDeviceSparseImageFormatInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format`] is the image format.
/// - [`type_`] is the dimensionality of image.
/// - [`samples`] is a [`SampleCountFlagBits`] value specifying the number of samples per texel.
/// - [`usage`] is a bitmask describing the intended usage of the image.
/// - [`tiling`] is the tiling arrangement of the texel blocks in memory.
///# Description
///## Valid Usage
/// - [`samples`] **must**  be a bit value that is set in [`ImageFormatProperties::sample_counts`]
///   returned by [`GetPhysicalDeviceImageFormatProperties`] with [`format`], [`type_`], [`tiling`],
///   and [`usage`] equal to those in this command and `flags` equal to the value that is set in
///   [`ImageCreateInfo::flags`] when the image is created
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2`
/// - [`p_next`] **must**  be `NULL`
/// - [`format`] **must**  be a valid [`Format`] value
/// - [`type_`] **must**  be a valid [`ImageType`] value
/// - [`samples`] **must**  be a valid [`SampleCountFlagBits`] value
/// - [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
/// - [`usage`] **must**  not be `0`
/// - [`tiling`] **must**  be a valid [`ImageTiling`] value
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Format`]
/// - [`ImageTiling`]
/// - [`ImageType`]
/// - [`ImageUsageFlags`]
/// - [`SampleCountFlagBits`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceSparseImageFormatProperties2`]
/// - [`GetPhysicalDeviceSparseImageFormatProperties2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`format`] is the image format.
    pub format: Format,
    ///[`type_`] is the dimensionality of image.
    pub type_: ImageType,
    ///[`samples`] is a [`SampleCountFlagBits`] value specifying the
    ///number of samples per texel.
    pub samples: SampleCountFlagBits,
    ///[`usage`] is a bitmask describing the intended usage of the image.
    pub usage: ImageUsageFlags,
    ///[`tiling`] is the tiling arrangement of the texel blocks in memory.
    pub tiling: ImageTiling,
}
impl<'lt> Default for PhysicalDeviceSparseImageFormatInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceSparseImageFormatInfo2,
            p_next: std::ptr::null(),
            format: Default::default(),
            type_: Default::default(),
            samples: Default::default(),
            usage: Default::default(),
            tiling: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceSparseImageFormatInfo2<'lt> {
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
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> ImageType {
        self.type_
    }
    ///Gets the value of [`Self::samples`]
    pub fn samples(&self) -> SampleCountFlagBits {
        self.samples
    }
    ///Gets the value of [`Self::usage`]
    pub fn usage(&self) -> ImageUsageFlags {
        self.usage
    }
    ///Gets the value of [`Self::tiling`]
    pub fn tiling(&self) -> ImageTiling {
        self.tiling
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut ImageType {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::samples`]
    pub fn samples_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.samples
    }
    ///Gets a mutable reference to the value of [`Self::usage`]
    pub fn usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.usage
    }
    ///Gets a mutable reference to the value of [`Self::tiling`]
    pub fn tiling_mut(&mut self) -> &mut ImageTiling {
        &mut self.tiling
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
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the raw value of [`Self::type_`]
    pub fn set_type_(&mut self, value: crate::vulkan1_0::ImageType) -> &mut Self {
        self.type_ = value;
        self
    }
    ///Sets the raw value of [`Self::samples`]
    pub fn set_samples(&mut self, value: crate::vulkan1_0::SampleCountFlagBits) -> &mut Self {
        self.samples = value;
        self
    }
    ///Sets the raw value of [`Self::usage`]
    pub fn set_usage(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.usage = value;
        self
    }
    ///Sets the raw value of [`Self::tiling`]
    pub fn set_tiling(&mut self, value: crate::vulkan1_0::ImageTiling) -> &mut Self {
        self.tiling = value;
        self
    }
}
///[VkPhysicalDeviceVariablePointersFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceVariablePointersFeatures.html) - Structure describing variable pointers features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceVariablePointersFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceVariablePointersFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           variablePointersStorageBuffer;
///    VkBool32           variablePointers;
///} VkPhysicalDeviceVariablePointersFeatures;
///```
///
///```c
///// Provided by VK_VERSION_1_1
///typedef VkPhysicalDeviceVariablePointersFeatures VkPhysicalDeviceVariablePointerFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_variable_pointers
///typedef VkPhysicalDeviceVariablePointersFeatures VkPhysicalDeviceVariablePointersFeaturesKHR;
///```
///
///```c
///// Provided by VK_KHR_variable_pointers
///typedef VkPhysicalDeviceVariablePointersFeatures VkPhysicalDeviceVariablePointerFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`variable_pointers_storage_buffer`] specifies whether the implementation supports the SPIR-V
///   `VariablePointersStorageBuffer` capability. When this feature is not enabled, shader modules
///   **must**  not declare the `SPV_KHR_variable_pointers` extension or the
///   `VariablePointersStorageBuffer` capability.
/// - [`variable_pointers`] specifies whether the implementation supports the SPIR-V
///   `VariablePointers` capability. When this feature is not enabled, shader modules  **must**  not
///   declare the `VariablePointers` capability.
///If the [`PhysicalDeviceVariablePointersFeatures`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVariablePointersFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage
/// - If [`variable_pointers`] is enabled then [`variable_pointers_storage_buffer`] **must**  also
///   be enabled
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`
///# Related
/// - [`VK_KHR_variable_pointers`]
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkPhysicalDeviceVariablePointersFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
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
}
impl<'lt> Default for PhysicalDeviceVariablePointersFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceVariablePointersFeatures,
            p_next: std::ptr::null_mut(),
            variable_pointers_storage_buffer: 0,
            variable_pointers: 0,
        }
    }
}
impl<'lt> PhysicalDeviceVariablePointersFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::variable_pointers_storage_buffer`]
    pub fn variable_pointers_storage_buffer_raw(&self) -> Bool32 {
        self.variable_pointers_storage_buffer
    }
    ///Gets the raw value of [`Self::variable_pointers`]
    pub fn variable_pointers_raw(&self) -> Bool32 {
        self.variable_pointers
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::variable_pointers_storage_buffer`]
    pub fn variable_pointers_storage_buffer(&self) -> bool {
        unsafe { std::mem::transmute(self.variable_pointers_storage_buffer as u8) }
    }
    ///Gets the value of [`Self::variable_pointers`]
    pub fn variable_pointers(&self) -> bool {
        unsafe { std::mem::transmute(self.variable_pointers as u8) }
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
}
///[VkExternalMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryProperties.html) - Structure specifying external memory handle type capabilities
///# C Specifications
///The [`ExternalMemoryProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExternalMemoryProperties {
///    VkExternalMemoryFeatureFlags       externalMemoryFeatures;
///    VkExternalMemoryHandleTypeFlags    exportFromImportedHandleTypes;
///    VkExternalMemoryHandleTypeFlags    compatibleHandleTypes;
///} VkExternalMemoryProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory_capabilities
///typedef VkExternalMemoryProperties VkExternalMemoryPropertiesKHR;
///```
///# Members
/// - [`external_memory_features`] is a bitmask of [`ExternalMemoryFeatureFlagBits`] specifying the
///   features of `handleType`.
/// - [`export_from_imported_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBits`]
///   specifying which types of imported handle `handleType` **can**  be exported from.
/// - [`compatible_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBits`] specifying
///   handle types which  **can**  be specified at the same time as `handleType` when creating an
///   image compatible with external memory.
///# Description
///[`compatible_handle_types`] **must**  include at least `handleType`.
///Inclusion of a handle type in [`compatible_handle_types`] does not imply the
///values returned in [`ImageFormatProperties2`] will be the same when
///[`PhysicalDeviceExternalImageFormatInfo::handle_type`] is set to
///that type.
///The application is responsible for querying the capabilities of all handle
///types intended for concurrent use in a single image and intersecting them to
///obtain the compatible set of capabilities.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalBufferProperties`]
/// - [`ExternalImageFormatProperties`]
/// - [`ExternalMemoryFeatureFlags`]
/// - [`ExternalMemoryHandleTypeFlags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryProperties")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalMemoryProperties {
    ///[`external_memory_features`] is a bitmask of
    ///[`ExternalMemoryFeatureFlagBits`] specifying the features of
    ///`handleType`.
    pub external_memory_features: ExternalMemoryFeatureFlags,
    ///[`export_from_imported_handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying which types of
    ///imported handle `handleType` **can**  be exported from.
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    ///[`compatible_handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying handle types which
    /// **can**  be specified at the same time as `handleType` when creating an
    ///image compatible with external memory.
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
impl Default for ExternalMemoryProperties {
    fn default() -> Self {
        Self {
            external_memory_features: Default::default(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
        }
    }
}
impl ExternalMemoryProperties {
    ///Gets the value of [`Self::external_memory_features`]
    pub fn external_memory_features(&self) -> ExternalMemoryFeatureFlags {
        self.external_memory_features
    }
    ///Gets the value of [`Self::export_from_imported_handle_types`]
    pub fn export_from_imported_handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.export_from_imported_handle_types
    }
    ///Gets the value of [`Self::compatible_handle_types`]
    pub fn compatible_handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.compatible_handle_types
    }
    ///Gets a mutable reference to the value of [`Self::external_memory_features`]
    pub fn external_memory_features_mut(&mut self) -> &mut ExternalMemoryFeatureFlags {
        &mut self.external_memory_features
    }
    ///Gets a mutable reference to the value of [`Self::export_from_imported_handle_types`]
    pub fn export_from_imported_handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlags {
        &mut self.export_from_imported_handle_types
    }
    ///Gets a mutable reference to the value of [`Self::compatible_handle_types`]
    pub fn compatible_handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlags {
        &mut self.compatible_handle_types
    }
    ///Sets the raw value of [`Self::external_memory_features`]
    pub fn set_external_memory_features(&mut self, value: crate::vulkan1_1::ExternalMemoryFeatureFlags) -> &mut Self {
        self.external_memory_features = value;
        self
    }
    ///Sets the raw value of [`Self::export_from_imported_handle_types`]
    pub fn set_export_from_imported_handle_types(
        &mut self,
        value: crate::vulkan1_1::ExternalMemoryHandleTypeFlags,
    ) -> &mut Self {
        self.export_from_imported_handle_types = value;
        self
    }
    ///Sets the raw value of [`Self::compatible_handle_types`]
    pub fn set_compatible_handle_types(&mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlags) -> &mut Self {
        self.compatible_handle_types = value;
        self
    }
}
///[VkPhysicalDeviceExternalImageFormatInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalImageFormatInfo.html) - Structure specifying external image creation parameters
///# C Specifications
///To determine the image capabilities compatible with an external memory
///handle type, add a [`PhysicalDeviceExternalImageFormatInfo`] structure
///to the [`p_next`] chain of the [`PhysicalDeviceImageFormatInfo2`]
///structure and a [`ExternalImageFormatProperties`] structure to the
///[`p_next`] chain of the [`ImageFormatProperties2`] structure.The
/// [`PhysicalDeviceExternalImageFormatInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceExternalImageFormatInfo {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///} VkPhysicalDeviceExternalImageFormatInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory_capabilities
///typedef VkPhysicalDeviceExternalImageFormatInfo VkPhysicalDeviceExternalImageFormatInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the memory handle
///   type that will be used with the memory associated with the image.
///# Description
///If [`handle_type`] is 0, [`GetPhysicalDeviceImageFormatProperties2`]
///will behave as if [`PhysicalDeviceExternalImageFormatInfo`] was not
///present, and [`ExternalImageFormatProperties`] will be ignored.If [`handle_type`] is not
/// compatible with the `format`, `type`,
///`tiling`, `usage`, and `flags` specified in
///[`PhysicalDeviceImageFormatInfo2`], then
///[`GetPhysicalDeviceImageFormatProperties2`] returns
///`VK_ERROR_FORMAT_NOT_SUPPORTED`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO`
/// - If [`handle_type`] is not `0`, [`handle_type`] **must**  be a valid
///   [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceExternalImageFormatInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the memory handle type that will be used with the memory
    ///associated with the image.
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl<'lt> Default for PhysicalDeviceExternalImageFormatInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceExternalImageFormatInfo,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceExternalImageFormatInfo<'lt> {
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
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
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
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
}
///[VkExternalImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatProperties.html) - Structure specifying supported external handle properties
///# C Specifications
///The [`ExternalImageFormatProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExternalImageFormatProperties {
///    VkStructureType               sType;
///    void*                         pNext;
///    VkExternalMemoryProperties    externalMemoryProperties;
///} VkExternalImageFormatProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory_capabilities
///typedef VkExternalImageFormatProperties VkExternalImageFormatPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`external_memory_properties`] is a [`ExternalMemoryProperties`] structure specifying various
///   capabilities of the external handle type when used with the specified image creation
///   parameters.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryProperties`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalImageFormatProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExternalImageFormatProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`external_memory_properties`] is a [`ExternalMemoryProperties`]
    ///structure specifying various capabilities of the external handle type
    ///when used with the specified image creation parameters.
    pub external_memory_properties: ExternalMemoryProperties,
}
impl<'lt> Default for ExternalImageFormatProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExternalImageFormatProperties,
            p_next: std::ptr::null_mut(),
            external_memory_properties: Default::default(),
        }
    }
}
impl<'lt> ExternalImageFormatProperties<'lt> {
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
    ///Gets the value of [`Self::external_memory_properties`]
    pub fn external_memory_properties(&self) -> ExternalMemoryProperties {
        self.external_memory_properties
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
    ///Gets a mutable reference to the value of [`Self::external_memory_properties`]
    pub fn external_memory_properties_mut(&mut self) -> &mut ExternalMemoryProperties {
        &mut self.external_memory_properties
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
    ///Sets the raw value of [`Self::external_memory_properties`]
    pub fn set_external_memory_properties(&mut self, value: crate::vulkan1_1::ExternalMemoryProperties) -> &mut Self {
        self.external_memory_properties = value;
        self
    }
}
///[VkPhysicalDeviceExternalBufferInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalBufferInfo.html) - Structure specifying buffer creation parameters
///# C Specifications
///The [`PhysicalDeviceExternalBufferInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceExternalBufferInfo {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkBufferCreateFlags                   flags;
///    VkBufferUsageFlags                    usage;
///    VkExternalMemoryHandleTypeFlagBits    handleType;
///} VkPhysicalDeviceExternalBufferInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory_capabilities
///typedef VkPhysicalDeviceExternalBufferInfo VkPhysicalDeviceExternalBufferInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`BufferCreateFlagBits`] describing additional parameters of the
///   buffer, corresponding to [`BufferCreateInfo`]::[`flags`].
/// - [`usage`] is a bitmask of [`BufferUsageFlagBits`] describing the intended usage of the buffer,
///   corresponding to [`BufferCreateInfo`]::[`usage`].
/// - [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the memory handle
///   type that will be used with the memory associated with the buffer.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be a valid combination of [`BufferCreateFlagBits`] values
/// - [`usage`] **must**  be a valid combination of [`BufferUsageFlagBits`] values
/// - [`usage`] **must**  not be `0`
/// - [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
///# Related
/// - [`crate::vulkan1_1`]
/// - [`BufferCreateFlags`]
/// - [`BufferUsageFlags`]
/// - [`ExternalMemoryHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceExternalBufferProperties`]
/// - [`GetPhysicalDeviceExternalBufferPropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceExternalBufferInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`BufferCreateFlagBits`] describing
    ///additional parameters of the buffer, corresponding to
    ///[`BufferCreateInfo`]::[`flags`].
    pub flags: BufferCreateFlags,
    ///[`usage`] is a bitmask of [`BufferUsageFlagBits`] describing the
    ///intended usage of the buffer, corresponding to
    ///[`BufferCreateInfo`]::[`usage`].
    pub usage: BufferUsageFlags,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the memory handle type that will be used with the memory
    ///associated with the buffer.
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl<'lt> Default for PhysicalDeviceExternalBufferInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceExternalBufferInfo,
            p_next: std::ptr::null(),
            flags: Default::default(),
            usage: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceExternalBufferInfo<'lt> {
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
    pub fn flags(&self) -> BufferCreateFlags {
        self.flags
    }
    ///Gets the value of [`Self::usage`]
    pub fn usage(&self) -> BufferUsageFlags {
        self.usage
    }
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalMemoryHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut BufferCreateFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::usage`]
    pub fn usage_mut(&mut self) -> &mut BufferUsageFlags {
        &mut self.usage
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagBits {
        &mut self.handle_type
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
    pub fn set_flags(&mut self, value: crate::vulkan1_0::BufferCreateFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::usage`]
    pub fn set_usage(&mut self, value: crate::vulkan1_0::BufferUsageFlags) -> &mut Self {
        self.usage = value;
        self
    }
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
}
///[VkExternalBufferProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalBufferProperties.html) - Structure specifying supported external handle capabilities
///# C Specifications
///The [`ExternalBufferProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExternalBufferProperties {
///    VkStructureType               sType;
///    void*                         pNext;
///    VkExternalMemoryProperties    externalMemoryProperties;
///} VkExternalBufferProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory_capabilities
///typedef VkExternalBufferProperties VkExternalBufferPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`external_memory_properties`] is a [`ExternalMemoryProperties`] structure specifying various
///   capabilities of the external handle type when used with the specified buffer creation
///   parameters.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryProperties`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceExternalBufferProperties`]
/// - [`GetPhysicalDeviceExternalBufferPropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalBufferProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExternalBufferProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`external_memory_properties`] is a [`ExternalMemoryProperties`]
    ///structure specifying various capabilities of the external handle type
    ///when used with the specified buffer creation parameters.
    pub external_memory_properties: ExternalMemoryProperties,
}
impl<'lt> Default for ExternalBufferProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExternalBufferProperties,
            p_next: std::ptr::null_mut(),
            external_memory_properties: Default::default(),
        }
    }
}
impl<'lt> ExternalBufferProperties<'lt> {
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
    ///Gets the value of [`Self::external_memory_properties`]
    pub fn external_memory_properties(&self) -> ExternalMemoryProperties {
        self.external_memory_properties
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
    ///Gets a mutable reference to the value of [`Self::external_memory_properties`]
    pub fn external_memory_properties_mut(&mut self) -> &mut ExternalMemoryProperties {
        &mut self.external_memory_properties
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
    ///Sets the raw value of [`Self::external_memory_properties`]
    pub fn set_external_memory_properties(&mut self, value: crate::vulkan1_1::ExternalMemoryProperties) -> &mut Self {
        self.external_memory_properties = value;
        self
    }
}
///[VkPhysicalDeviceIDProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIDProperties.html) - Structure specifying IDs related to the physical device
///# C Specifications
///The [`PhysicalDeviceIdProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceIDProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    uint8_t            deviceUUID[VK_UUID_SIZE];
///    uint8_t            driverUUID[VK_UUID_SIZE];
///    uint8_t            deviceLUID[VK_LUID_SIZE];
///    uint32_t           deviceNodeMask;
///    VkBool32           deviceLUIDValid;
///} VkPhysicalDeviceIDProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence_capabilities, VK_KHR_external_memory_capabilities,
///// VK_KHR_external_semaphore_capabilities
///typedef VkPhysicalDeviceIDProperties VkPhysicalDeviceIDPropertiesKHR;
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
///If the [`PhysicalDeviceIdProperties`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.[`device_uuid`] **must**  be immutable for a
/// given device across instances,
///processes, driver APIs, driver versions, and system reboots.Applications  **can**  compare the
/// [`driver_uuid`] value across instance and
///process boundaries, and  **can**  make similar queries in external APIs to
///determine whether they are capable of sharing memory objects and resources
///using them with the device.[`device_uuid`] and/or [`driver_uuid`] **must**  be used to determine
/// whether
///a particular external object can be shared between driver components, where
///such a restriction exists as defined in the compatibility table for the
///particular object type:
/// - [External memory handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-memory-handle-types-compatibility)
/// - [External semaphore handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-semaphore-handle-types-compatibility)
/// - [External fence handle types compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#external-fence-handle-types-compatibility)
///If [`device_luid_valid`] is [`FALSE`], the values of [`device_luid`]
///and [`device_node_mask`] are undefined.
///If [`device_luid_valid`] is [`TRUE`] and Vulkan is running on the
///Windows operating system, the contents of [`device_luid`] **can**  be cast to
///an `LUID` object and  **must**  be equal to the locally unique identifier of a
///`IDXGIAdapter1` object that corresponds to `physicalDevice`.
///If [`device_luid_valid`] is [`TRUE`], [`device_node_mask`] **must**
///contain exactly one bit.
///If Vulkan is running on an operating system that supports the Direct3D 12
///API and `physicalDevice` corresponds to an individual device in a linked
///device adapter, [`device_node_mask`] identifies the Direct3D 12 node
///corresponding to `physicalDevice`.
///Otherwise, [`device_node_mask`] **must**  be `1`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES`
///# Related
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkPhysicalDeviceIDProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceIdProperties<'lt> {
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
}
impl<'lt> Default for PhysicalDeviceIdProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceIdProperties,
            p_next: std::ptr::null_mut(),
            device_uuid: [0; UUID_SIZE as usize],
            driver_uuid: [0; UUID_SIZE as usize],
            device_luid: [0; LUID_SIZE as usize],
            device_node_mask: 0,
            device_luid_valid: 0,
        }
    }
}
impl<'lt> PhysicalDeviceIdProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::device_luid_valid`]
    pub fn device_luid_valid_raw(&self) -> Bool32 {
        self.device_luid_valid
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
}
///[VkExternalMemoryImageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfo.html) - Specify that an image may be backed by external memory
///# C Specifications
///To define a set of external memory handle types that  **may**  be used as backing
///store for an image, add a [`ExternalMemoryImageCreateInfo`] structure to
///the [`p_next`] chain of the [`ImageCreateInfo`] structure.
///The [`ExternalMemoryImageCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExternalMemoryImageCreateInfo {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkExternalMemoryHandleTypeFlags    handleTypes;
///} VkExternalMemoryImageCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory
///typedef VkExternalMemoryImageCreateInfo VkExternalMemoryImageCreateInfoKHR;
///```
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_types`] is zero, or a bitmask of [`ExternalMemoryHandleTypeFlagBits`] specifying one
///   or more external memory handle types.
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO`
/// - [`handle_types`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBits`]
///   values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryHandleTypeFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryImageCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_types`] is zero, or a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying one or more external
    ///memory handle types.
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl<'lt> Default for ExternalMemoryImageCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExternalMemoryImageCreateInfo,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl<'lt> ExternalMemoryImageCreateInfo<'lt> {
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
    ///Gets the value of [`Self::handle_types`]
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.handle_types
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_types`]
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlags {
        &mut self.handle_types
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
    ///Sets the raw value of [`Self::handle_types`]
    pub fn set_handle_types(&mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlags) -> &mut Self {
        self.handle_types = value;
        self
    }
}
///[VkExternalMemoryBufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryBufferCreateInfo.html) - Specify that a buffer may be backed by external memory
///# C Specifications
///To define a set of external memory handle types that  **may**  be used as backing
///store for a buffer, add a [`ExternalMemoryBufferCreateInfo`] structure
///to the [`p_next`] chain of the [`BufferCreateInfo`] structure.
///The [`ExternalMemoryBufferCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExternalMemoryBufferCreateInfo {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkExternalMemoryHandleTypeFlags    handleTypes;
///} VkExternalMemoryBufferCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory
///typedef VkExternalMemoryBufferCreateInfo VkExternalMemoryBufferCreateInfoKHR;
///```
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_types`] is zero, or a bitmask of [`ExternalMemoryHandleTypeFlagBits`] specifying one
///   or more external memory handle types.
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO`
/// - [`handle_types`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBits`]
///   values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryHandleTypeFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryBufferCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExternalMemoryBufferCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_types`] is zero, or a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying one or more external
    ///memory handle types.
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl<'lt> Default for ExternalMemoryBufferCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExternalMemoryBufferCreateInfo,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl<'lt> ExternalMemoryBufferCreateInfo<'lt> {
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
    ///Gets the value of [`Self::handle_types`]
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.handle_types
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_types`]
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlags {
        &mut self.handle_types
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
    ///Sets the raw value of [`Self::handle_types`]
    pub fn set_handle_types(&mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlags) -> &mut Self {
        self.handle_types = value;
        self
    }
}
///[VkExportMemoryAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfo.html) - Specify exportable handle types for a device memory object
///# C Specifications
///When allocating memory whose payload  **may**  be exported to another process or
///Vulkan instance, add a [`ExportMemoryAllocateInfo`] structure to the
///[`p_next`] chain of the [`MemoryAllocateInfo`] structure, specifying
///the handle types that  **may**  be exported.The [`ExportMemoryAllocateInfo`] structure is
/// defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExportMemoryAllocateInfo {
///    VkStructureType                    sType;
///    const void*                        pNext;
///    VkExternalMemoryHandleTypeFlags    handleTypes;
///} VkExportMemoryAllocateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_memory
///typedef VkExportMemoryAllocateInfo VkExportMemoryAllocateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBits`] specifying one or more
///   memory handle types the application  **can**  export from the resulting allocation. The
///   application  **can**  request multiple handle types for the same allocation.
///# Description
///## Valid Usage
/// - The bits in [`handle_types`] **must**  be supported and compatible, as reported by
///   [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO`
/// - [`handle_types`] **must**  be a valid combination of [`ExternalMemoryHandleTypeFlagBits`]
///   values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalMemoryHandleTypeFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExportMemoryAllocateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExportMemoryAllocateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying one or more memory
    ///handle types the application  **can**  export from the resulting allocation.
    ///The application  **can**  request multiple handle types for the same
    ///allocation.
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl<'lt> Default for ExportMemoryAllocateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExportMemoryAllocateInfo,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl<'lt> ExportMemoryAllocateInfo<'lt> {
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
    ///Gets the value of [`Self::handle_types`]
    pub fn handle_types(&self) -> ExternalMemoryHandleTypeFlags {
        self.handle_types
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_types`]
    pub fn handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlags {
        &mut self.handle_types
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
    ///Sets the raw value of [`Self::handle_types`]
    pub fn set_handle_types(&mut self, value: crate::vulkan1_1::ExternalMemoryHandleTypeFlags) -> &mut Self {
        self.handle_types = value;
        self
    }
}
///[VkPhysicalDeviceExternalSemaphoreInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalSemaphoreInfo.html) - Structure specifying semaphore creation parameters.
///# C Specifications
///The [`PhysicalDeviceExternalSemaphoreInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceExternalSemaphoreInfo {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    VkExternalSemaphoreHandleTypeFlagBits    handleType;
///} VkPhysicalDeviceExternalSemaphoreInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_semaphore_capabilities
///typedef VkPhysicalDeviceExternalSemaphoreInfo VkPhysicalDeviceExternalSemaphoreInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value specifying the external
///   semaphore handle type for which capabilities will be returned.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`SemaphoreTypeCreateInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`handle_type`] **must**  be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalSemaphoreHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceExternalSemaphoreProperties`]
/// - [`GetPhysicalDeviceExternalSemaphorePropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceExternalSemaphoreInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the external semaphore handle type for which capabilities
    ///will be returned.
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl<'lt> Default for PhysicalDeviceExternalSemaphoreInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceExternalSemaphoreInfo,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceExternalSemaphoreInfo<'lt> {
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
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalSemaphoreHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlagBits {
        &mut self.handle_type
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
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
}
///[VkExternalSemaphoreProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalSemaphoreProperties.html) - Structure describing supported external semaphore handle features
///# C Specifications
///The [`ExternalSemaphoreProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExternalSemaphoreProperties {
///    VkStructureType                       sType;
///    void*                                 pNext;
///    VkExternalSemaphoreHandleTypeFlags    exportFromImportedHandleTypes;
///    VkExternalSemaphoreHandleTypeFlags    compatibleHandleTypes;
///    VkExternalSemaphoreFeatureFlags       externalSemaphoreFeatures;
///} VkExternalSemaphoreProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_semaphore_capabilities
///typedef VkExternalSemaphoreProperties VkExternalSemaphorePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`export_from_imported_handle_types`] is a bitmask of [`ExternalSemaphoreHandleTypeFlagBits`]
///   specifying which types of imported handle `handleType` **can**  be exported from.
/// - [`compatible_handle_types`] is a bitmask of [`ExternalSemaphoreHandleTypeFlagBits`] specifying
///   handle types which  **can**  be specified at the same time as `handleType` when creating a
///   semaphore.
/// - [`external_semaphore_features`] is a bitmask of [`ExternalSemaphoreFeatureFlagBits`]
///   describing the features of `handleType`.
///# Description
///If `handleType` is not supported by the implementation, then
///[`ExternalSemaphoreProperties`]::[`external_semaphore_features`] will be
///set to zero.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalSemaphoreFeatureFlags`]
/// - [`ExternalSemaphoreHandleTypeFlags`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceExternalSemaphoreProperties`]
/// - [`GetPhysicalDeviceExternalSemaphorePropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalSemaphoreProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExternalSemaphoreProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`export_from_imported_handle_types`] is a bitmask of
    ///[`ExternalSemaphoreHandleTypeFlagBits`] specifying which types of
    ///imported handle `handleType` **can**  be exported from.
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    ///[`compatible_handle_types`] is a bitmask of
    ///[`ExternalSemaphoreHandleTypeFlagBits`] specifying handle types
    ///which  **can**  be specified at the same time as `handleType` when
    ///creating a semaphore.
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    ///[`external_semaphore_features`] is a bitmask of
    ///[`ExternalSemaphoreFeatureFlagBits`] describing the features of
    ///`handleType`.
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
impl<'lt> Default for ExternalSemaphoreProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExternalSemaphoreProperties,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_semaphore_features: Default::default(),
        }
    }
}
impl<'lt> ExternalSemaphoreProperties<'lt> {
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
    ///Gets the value of [`Self::export_from_imported_handle_types`]
    pub fn export_from_imported_handle_types(&self) -> ExternalSemaphoreHandleTypeFlags {
        self.export_from_imported_handle_types
    }
    ///Gets the value of [`Self::compatible_handle_types`]
    pub fn compatible_handle_types(&self) -> ExternalSemaphoreHandleTypeFlags {
        self.compatible_handle_types
    }
    ///Gets the value of [`Self::external_semaphore_features`]
    pub fn external_semaphore_features(&self) -> ExternalSemaphoreFeatureFlags {
        self.external_semaphore_features
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
    ///Gets a mutable reference to the value of [`Self::export_from_imported_handle_types`]
    pub fn export_from_imported_handle_types_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlags {
        &mut self.export_from_imported_handle_types
    }
    ///Gets a mutable reference to the value of [`Self::compatible_handle_types`]
    pub fn compatible_handle_types_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlags {
        &mut self.compatible_handle_types
    }
    ///Gets a mutable reference to the value of [`Self::external_semaphore_features`]
    pub fn external_semaphore_features_mut(&mut self) -> &mut ExternalSemaphoreFeatureFlags {
        &mut self.external_semaphore_features
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
    ///Sets the raw value of [`Self::export_from_imported_handle_types`]
    pub fn set_export_from_imported_handle_types(
        &mut self,
        value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlags,
    ) -> &mut Self {
        self.export_from_imported_handle_types = value;
        self
    }
    ///Sets the raw value of [`Self::compatible_handle_types`]
    pub fn set_compatible_handle_types(
        &mut self,
        value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlags,
    ) -> &mut Self {
        self.compatible_handle_types = value;
        self
    }
    ///Sets the raw value of [`Self::external_semaphore_features`]
    pub fn set_external_semaphore_features(
        &mut self,
        value: crate::vulkan1_1::ExternalSemaphoreFeatureFlags,
    ) -> &mut Self {
        self.external_semaphore_features = value;
        self
    }
}
///[VkExportSemaphoreCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreCreateInfo.html) - Structure specifying handle types that can be exported from a semaphore
///# C Specifications
///To create a semaphore whose payload  **can**  be exported to external handles,
///add a [`ExportSemaphoreCreateInfo`] structure to the [`p_next`] chain
///of the [`SemaphoreCreateInfo`] structure.
///The [`ExportSemaphoreCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExportSemaphoreCreateInfo {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkExternalSemaphoreHandleTypeFlags    handleTypes;
///} VkExportSemaphoreCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_semaphore
///typedef VkExportSemaphoreCreateInfo VkExportSemaphoreCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_types`] is a bitmask of [`ExternalSemaphoreHandleTypeFlagBits`] specifying one or
///   more semaphore handle types the application  **can**  export from the resulting semaphore. The
///   application  **can**  request multiple handle types for the same semaphore.
///# Description
///## Valid Usage
/// - The bits in [`handle_types`] **must**  be supported and compatible, as reported by
///   [`ExternalSemaphoreProperties`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO`
/// - [`handle_types`] **must**  be a valid combination of [`ExternalSemaphoreHandleTypeFlagBits`]
///   values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalSemaphoreHandleTypeFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExportSemaphoreCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExportSemaphoreCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_types`] is a bitmask of
    ///[`ExternalSemaphoreHandleTypeFlagBits`] specifying one or more
    ///semaphore handle types the application  **can**  export from the resulting
    ///semaphore.
    ///The application  **can**  request multiple handle types for the same
    ///semaphore.
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}
impl<'lt> Default for ExportSemaphoreCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExportSemaphoreCreateInfo,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl<'lt> ExportSemaphoreCreateInfo<'lt> {
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
    ///Gets the value of [`Self::handle_types`]
    pub fn handle_types(&self) -> ExternalSemaphoreHandleTypeFlags {
        self.handle_types
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_types`]
    pub fn handle_types_mut(&mut self) -> &mut ExternalSemaphoreHandleTypeFlags {
        &mut self.handle_types
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
    ///Sets the raw value of [`Self::handle_types`]
    pub fn set_handle_types(&mut self, value: crate::vulkan1_1::ExternalSemaphoreHandleTypeFlags) -> &mut Self {
        self.handle_types = value;
        self
    }
}
///[VkPhysicalDeviceExternalFenceInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExternalFenceInfo.html) - Structure specifying fence creation parameters.
///# C Specifications
///The [`PhysicalDeviceExternalFenceInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceExternalFenceInfo {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkExternalFenceHandleTypeFlagBits    handleType;
///} VkPhysicalDeviceExternalFenceInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence_capabilities
///typedef VkPhysicalDeviceExternalFenceInfo VkPhysicalDeviceExternalFenceInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value specifying an external fence
///   handle type for which capabilities will be returned.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO`
/// - [`p_next`] **must**  be `NULL`
/// - [`handle_type`] **must**  be a valid [`ExternalFenceHandleTypeFlagBits`] value
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalFenceHandleTypeFlagBits`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceExternalFenceProperties`]
/// - [`GetPhysicalDeviceExternalFencePropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceExternalFenceInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value
    ///specifying an external fence handle type for which capabilities will be
    ///returned.
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
impl<'lt> Default for PhysicalDeviceExternalFenceInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceExternalFenceInfo,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceExternalFenceInfo<'lt> {
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
    ///Gets the value of [`Self::handle_type`]
    pub fn handle_type(&self) -> ExternalFenceHandleTypeFlagBits {
        self.handle_type
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_type`]
    pub fn handle_type_mut(&mut self) -> &mut ExternalFenceHandleTypeFlagBits {
        &mut self.handle_type
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
    ///Sets the raw value of [`Self::handle_type`]
    pub fn set_handle_type(&mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlagBits) -> &mut Self {
        self.handle_type = value;
        self
    }
}
///[VkExternalFenceProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalFenceProperties.html) - Structure describing supported external fence handle features
///# C Specifications
///The [`ExternalFenceProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExternalFenceProperties {
///    VkStructureType                   sType;
///    void*                             pNext;
///    VkExternalFenceHandleTypeFlags    exportFromImportedHandleTypes;
///    VkExternalFenceHandleTypeFlags    compatibleHandleTypes;
///    VkExternalFenceFeatureFlags       externalFenceFeatures;
///} VkExternalFenceProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence_capabilities
///typedef VkExternalFenceProperties VkExternalFencePropertiesKHR;
///```
///# Members
/// - [`export_from_imported_handle_types`] is a bitmask of [`ExternalFenceHandleTypeFlagBits`]
///   indicating which types of imported handle `handleType` **can**  be exported from.
/// - [`compatible_handle_types`] is a bitmask of [`ExternalFenceHandleTypeFlagBits`] specifying
///   handle types which  **can**  be specified at the same time as `handleType` when creating a
///   fence.
/// - [`external_fence_features`] is a bitmask of [`ExternalFenceFeatureFlagBits`] indicating the
///   features of `handleType`.
///# Description
///If `handleType` is not supported by the implementation, then
///[`ExternalFenceProperties`]::[`external_fence_features`] will be set to
///zero.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalFenceFeatureFlags`]
/// - [`ExternalFenceHandleTypeFlags`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceExternalFenceProperties`]
/// - [`GetPhysicalDeviceExternalFencePropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalFenceProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExternalFenceProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`export_from_imported_handle_types`] is a bitmask of
    ///[`ExternalFenceHandleTypeFlagBits`] indicating which types of
    ///imported handle `handleType` **can**  be exported from.
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    ///[`compatible_handle_types`] is a bitmask of
    ///[`ExternalFenceHandleTypeFlagBits`] specifying handle types which
    /// **can**  be specified at the same time as `handleType` when creating a
    ///fence.
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    ///[`external_fence_features`] is a bitmask of
    ///[`ExternalFenceFeatureFlagBits`] indicating the features of
    ///`handleType`.
    pub external_fence_features: ExternalFenceFeatureFlags,
}
impl<'lt> Default for ExternalFenceProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExternalFenceProperties,
            p_next: std::ptr::null_mut(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
            external_fence_features: Default::default(),
        }
    }
}
impl<'lt> ExternalFenceProperties<'lt> {
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
    ///Gets the value of [`Self::export_from_imported_handle_types`]
    pub fn export_from_imported_handle_types(&self) -> ExternalFenceHandleTypeFlags {
        self.export_from_imported_handle_types
    }
    ///Gets the value of [`Self::compatible_handle_types`]
    pub fn compatible_handle_types(&self) -> ExternalFenceHandleTypeFlags {
        self.compatible_handle_types
    }
    ///Gets the value of [`Self::external_fence_features`]
    pub fn external_fence_features(&self) -> ExternalFenceFeatureFlags {
        self.external_fence_features
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
    ///Gets a mutable reference to the value of [`Self::export_from_imported_handle_types`]
    pub fn export_from_imported_handle_types_mut(&mut self) -> &mut ExternalFenceHandleTypeFlags {
        &mut self.export_from_imported_handle_types
    }
    ///Gets a mutable reference to the value of [`Self::compatible_handle_types`]
    pub fn compatible_handle_types_mut(&mut self) -> &mut ExternalFenceHandleTypeFlags {
        &mut self.compatible_handle_types
    }
    ///Gets a mutable reference to the value of [`Self::external_fence_features`]
    pub fn external_fence_features_mut(&mut self) -> &mut ExternalFenceFeatureFlags {
        &mut self.external_fence_features
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
    ///Sets the raw value of [`Self::export_from_imported_handle_types`]
    pub fn set_export_from_imported_handle_types(
        &mut self,
        value: crate::vulkan1_1::ExternalFenceHandleTypeFlags,
    ) -> &mut Self {
        self.export_from_imported_handle_types = value;
        self
    }
    ///Sets the raw value of [`Self::compatible_handle_types`]
    pub fn set_compatible_handle_types(&mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlags) -> &mut Self {
        self.compatible_handle_types = value;
        self
    }
    ///Sets the raw value of [`Self::external_fence_features`]
    pub fn set_external_fence_features(&mut self, value: crate::vulkan1_1::ExternalFenceFeatureFlags) -> &mut Self {
        self.external_fence_features = value;
        self
    }
}
///[VkExportFenceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportFenceCreateInfo.html) - Structure specifying handle types that can be exported from a fence
///# C Specifications
///To create a fence whose payload  **can**  be exported to external handles, add a
///[`ExportFenceCreateInfo`] structure to the [`p_next`] chain of the
///[`FenceCreateInfo`] structure.
///The [`ExportFenceCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkExportFenceCreateInfo {
///    VkStructureType                   sType;
///    const void*                       pNext;
///    VkExternalFenceHandleTypeFlags    handleTypes;
///} VkExportFenceCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_external_fence
///typedef VkExportFenceCreateInfo VkExportFenceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`handle_types`] is a bitmask of [`ExternalFenceHandleTypeFlagBits`] specifying one or more
///   fence handle types the application  **can**  export from the resulting fence. The application
///   **can**  request multiple handle types for the same fence.
///# Description
///## Valid Usage
/// - The bits in [`handle_types`] **must**  be supported and compatible, as reported by
///   [`ExternalFenceProperties`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO`
/// - [`handle_types`] **must**  be a valid combination of [`ExternalFenceHandleTypeFlagBits`]
///   values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ExternalFenceHandleTypeFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExportFenceCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ExportFenceCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`handle_types`] is a bitmask of
    ///[`ExternalFenceHandleTypeFlagBits`] specifying one or more fence
    ///handle types the application  **can**  export from the resulting fence.
    ///The application  **can**  request multiple handle types for the same fence.
    pub handle_types: ExternalFenceHandleTypeFlags,
}
impl<'lt> Default for ExportFenceCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ExportFenceCreateInfo,
            p_next: std::ptr::null(),
            handle_types: Default::default(),
        }
    }
}
impl<'lt> ExportFenceCreateInfo<'lt> {
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
    ///Gets the value of [`Self::handle_types`]
    pub fn handle_types(&self) -> ExternalFenceHandleTypeFlags {
        self.handle_types
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::handle_types`]
    pub fn handle_types_mut(&mut self) -> &mut ExternalFenceHandleTypeFlags {
        &mut self.handle_types
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
    ///Sets the raw value of [`Self::handle_types`]
    pub fn set_handle_types(&mut self, value: crate::vulkan1_1::ExternalFenceHandleTypeFlags) -> &mut Self {
        self.handle_types = value;
        self
    }
}
///[VkPhysicalDeviceMultiviewFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewFeatures.html) - Structure describing multiview features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceMultiviewFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceMultiviewFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           multiview;
///    VkBool32           multiviewGeometryShader;
///    VkBool32           multiviewTessellationShader;
///} VkPhysicalDeviceMultiviewFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_multiview
///typedef VkPhysicalDeviceMultiviewFeatures VkPhysicalDeviceMultiviewFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`multiview`] specifies whether the implementation supports multiview rendering within a
///   render pass. If this feature is not enabled, the view mask of each subpass  **must**  always
///   be zero.
/// - [`multiview_geometry_shader`] specifies whether the implementation supports multiview rendering within a render pass, with [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#geometry). If this feature is not enabled, then a pipeline compiled against a subpass with a non-zero view mask  **must**  not include a geometry shader.
/// - [`multiview_tessellation_shader`] specifies whether the implementation supports multiview rendering within a render pass, with [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation). If this feature is not enabled, then a pipeline compiled against a subpass with a non-zero view mask  **must**  not include any tessellation shaders.
///If the [`PhysicalDeviceMultiviewFeatures`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMultiviewFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage
/// - If [`multiview_geometry_shader`] is enabled then [`multiview`] **must**  also be enabled
/// - If [`multiview_tessellation_shader`] is enabled then [`multiview`] **must**  also be enabled
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES`
///# Related
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkPhysicalDeviceMultiviewFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`multiview`] specifies whether
    ///the implementation supports multiview rendering within a render pass.
    ///If this feature is not enabled, the view mask of each subpass  **must**
    ///always be zero.
    pub multiview: Bool32,
    ///[`multiview_geometry_shader`]
    ///specifies whether the implementation supports multiview rendering within
    ///a render pass, with [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#geometry).
    ///If this feature is not enabled, then a pipeline compiled against a
    ///subpass with a non-zero view mask  **must**  not include a geometry shader.
    pub multiview_geometry_shader: Bool32,
    ///[`multiview_tessellation_shader`] specifies whether the implementation
    ///supports multiview rendering within a render pass, with
    ///[tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation).
    ///If this feature is not enabled, then a pipeline compiled against a
    ///subpass with a non-zero view mask  **must**  not include any tessellation
    ///shaders.
    pub multiview_tessellation_shader: Bool32,
}
impl<'lt> Default for PhysicalDeviceMultiviewFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceMultiviewFeatures,
            p_next: std::ptr::null_mut(),
            multiview: 0,
            multiview_geometry_shader: 0,
            multiview_tessellation_shader: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMultiviewFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
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
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
}
///[VkPhysicalDeviceMultiviewProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewProperties.html) - Structure describing multiview limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceMultiviewProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceMultiviewProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxMultiviewViewCount;
///    uint32_t           maxMultiviewInstanceIndex;
///} VkPhysicalDeviceMultiviewProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_multiview
///typedef VkPhysicalDeviceMultiviewProperties VkPhysicalDeviceMultiviewPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`max_multiview_view_count`] is one greater than the maximum view index that  **can**  be used
///   in a subpass.
/// - [`max_multiview_instance_index`] is the maximum valid value of instance index allowed to be
///   generated by a drawing command recorded within a subpass of a multiview render pass instance.
///If the [`PhysicalDeviceMultiviewProperties`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceMultiviewProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub max_multiview_view_count: u32,
    ///No documentation found
    pub max_multiview_instance_index: u32,
}
impl<'lt> Default for PhysicalDeviceMultiviewProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceMultiviewProperties,
            p_next: std::ptr::null_mut(),
            max_multiview_view_count: 0,
            max_multiview_instance_index: 0,
        }
    }
}
impl<'lt> PhysicalDeviceMultiviewProperties<'lt> {
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
    ///Gets the value of [`Self::max_multiview_view_count`]
    pub fn max_multiview_view_count(&self) -> u32 {
        self.max_multiview_view_count
    }
    ///Gets the value of [`Self::max_multiview_instance_index`]
    pub fn max_multiview_instance_index(&self) -> u32 {
        self.max_multiview_instance_index
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
    ///Gets a mutable reference to the value of [`Self::max_multiview_view_count`]
    pub fn max_multiview_view_count_mut(&mut self) -> &mut u32 {
        &mut self.max_multiview_view_count
    }
    ///Gets a mutable reference to the value of [`Self::max_multiview_instance_index`]
    pub fn max_multiview_instance_index_mut(&mut self) -> &mut u32 {
        &mut self.max_multiview_instance_index
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
}
///[VkRenderPassMultiviewCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassMultiviewCreateInfo.html) - Structure containing multiview information for all subpasses
///# C Specifications
///If the [`RenderPassCreateInfo`]::[`p_next`] chain includes a
///[`RenderPassMultiviewCreateInfo`] structure, then that structure
///includes an array of view masks, view offsets, and correlation masks for the
///render pass.The [`RenderPassMultiviewCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkRenderPassMultiviewCreateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           subpassCount;
///    const uint32_t*    pViewMasks;
///    uint32_t           dependencyCount;
///    const int32_t*     pViewOffsets;
///    uint32_t           correlationMaskCount;
///    const uint32_t*    pCorrelationMasks;
///} VkRenderPassMultiviewCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_multiview
///typedef VkRenderPassMultiviewCreateInfo VkRenderPassMultiviewCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`subpass_count`] is zero or the number of subpasses in the render pass.
/// - [`view_masks`] is a pointer to an array of [`subpass_count`] view masks, where each mask is a
///   bitfield of view indices describing which views rendering is broadcast to in each subpass,
///   when multiview is enabled. If [`subpass_count`] is zero, each view mask is treated as zero.
/// - [`dependency_count`] is zero or the number of dependencies in the render pass.
/// - [`view_offsets`] is a pointer to an array of [`dependency_count`] view offsets, one for each
///   dependency. If [`dependency_count`] is zero, each dependency’s view offset is treated as zero.
///   Each view offset controls which views in the source subpass the views in the destination
///   subpass depend on.
/// - [`correlation_mask_count`] is zero or the number of correlation masks.
/// - [`correlation_masks`] is a pointer to an array of [`correlation_mask_count`] view masks
///   indicating sets of views that  **may**  be more efficient to render concurrently.
///# Description
///When a subpass uses a non-zero view mask, *multiview* functionality is
///considered to be enabled.
///Multiview is all-or-nothing for a render pass - that is, either all
///subpasses  **must**  have a non-zero view mask (though some subpasses  **may**  have
///only one view) or all  **must**  be zero.
///Multiview causes all drawing and clear commands in the subpass to behave as
///if they were broadcast to each view, where a view is represented by one
///layer of the framebuffer attachments.
///All draws and clears are broadcast to each *view index* whose bit is set in
///the view mask.
///The view index is provided in the `ViewIndex` shader input variable, and
///color, depth/stencil, and input attachments all read/write the layer of the
///framebuffer corresponding to the view index.If the view mask is zero for all subpasses,
/// multiview is considered to be
///disabled and all drawing commands execute normally, without this additional
///broadcasting.Some implementations  **may**  not support multiview in conjunction with
///[geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview-gs) or
///[tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview-tess).When multiview is enabled, the `VK_DEPENDENCY_VIEW_LOCAL_BIT` bit in a
///dependency  **can**  be used to express a view-local dependency, meaning that
///each view in the destination subpass depends on a single view in the source
///subpass.
///Unlike pipeline barriers, a subpass dependency  **can**  potentially have a
///different view mask in the source subpass and the destination subpass.
///If the dependency is view-local, then each view (dstView) in the
///destination subpass depends on the view dstView +
///[`view_offsets`][dependency] in the source subpass.
///If there is not such a view in the source subpass, then this dependency does
///not affect that view in the destination subpass.
///If the dependency is not view-local, then all views in the destination
///subpass depend on all views in the source subpass, and the view offset is
///ignored.
///A non-zero view offset is not allowed in a self-dependency.The elements of [`correlation_masks`]
/// are a set of masks of views
///indicating that views in the same mask  **may**  exhibit spatial coherency
///between the views, making it more efficient to render them concurrently.
///Correlation masks  **must**  not have a functional effect on the results of the
///multiview rendering.When multiview is enabled, at the beginning of each subpass all non-render
///pass state is undefined.
///In particular, each time [`CmdBeginRenderPass`] or
///[`CmdNextSubpass`] is called the graphics pipeline  **must**  be bound, any
///relevant descriptor sets or vertex/index buffers  **must**  be bound, and any
///relevant dynamic state or push constants  **must**  be set before they are used.A multiview
/// subpass  **can**  declare that its shaders will write per-view
///attributes for all views in a single invocation, by setting the
///`VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` bit in the subpass
///description.
///The only supported per-view attributes are position and viewport mask, and
///per-view position and viewport masks are written to output array variables
///decorated with `PositionPerViewNV` and `ViewportMaskPerViewNV`,
///respectively.
///If `[`VK_NV_viewport_array2`]` is not supported and enabled,
///`ViewportMaskPerViewNV` **must**  not be used.
///Values written to elements of `PositionPerViewNV` and
///`ViewportMaskPerViewNV` **must**  not depend on the `ViewIndex`.
///The shader  **must**  also write to an output variable decorated with
///`Position`, and the value written to `Position` **must**  equal the value
///written to `PositionPerViewNV`[`ViewIndex`].
///Similarly, if `ViewportMaskPerViewNV` is written to then the shader  **must**
///also write to an output variable decorated with `ViewportMaskNV`, and the
///value written to `ViewportMaskNV` **must**  equal the value written to
///`ViewportMaskPerViewNV`[`ViewIndex`].
///Implementations will either use values taken from `Position` and
///`ViewportMaskNV` and invoke the shader once for each view, or will use
///values taken from `PositionPerViewNV` and `ViewportMaskPerViewNV` and
///invoke the shader fewer times.
///The values written to `Position` and `ViewportMaskNV` **must**  not depend
///on the values written to `PositionPerViewNV` and
///`ViewportMaskPerViewNV`, or vice versa (to allow compilers to eliminate
///the unused outputs).
///All attributes that do not have `*PerViewNV` counterparts  **must**  not depend
///on `ViewIndex`.Per-view attributes are all-or-nothing for a subpass.
///That is, all pipelines compiled against a subpass that includes the
///`VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` bit  **must**  write
///per-view attributes to the `*PerViewNV[]` shader outputs, in addition to the
///non-per-view (e.g. `Position`) outputs.
///Pipelines compiled against a subpass that does not include this bit  **must**
///not include the `*PerViewNV[]` outputs in their interfaces.
///## Valid Usage
/// - Each view index  **must**  not be set in more than one element of [`correlation_masks`]
/// - If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview)
///   feature is not enabled, each element of [`view_masks`] **must**  be `0`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO`
/// - If [`subpass_count`] is not `0`, [`view_masks`] **must**  be a valid pointer to an array of
///   [`subpass_count`]`uint32_t` values
/// - If [`dependency_count`] is not `0`, [`view_offsets`] **must**  be a valid pointer to an array
///   of [`dependency_count`]`int32_t` values
/// - If [`correlation_mask_count`] is not `0`, [`correlation_masks`] **must**  be a valid pointer
///   to an array of [`correlation_mask_count`]`uint32_t` values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRenderPassMultiviewCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct RenderPassMultiviewCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`subpass_count`] is zero or the number of subpasses in the render
    ///pass.
    pub subpass_count: u32,
    ///[`view_masks`] is a pointer to an array of [`subpass_count`] view
    ///masks, where each mask is a bitfield of view indices describing which
    ///views rendering is broadcast to in each subpass, when multiview is
    ///enabled.
    ///If [`subpass_count`] is zero, each view mask is treated as zero.
    pub view_masks: *const u32,
    ///[`dependency_count`] is zero or the number of dependencies in the
    ///render pass.
    pub dependency_count: u32,
    ///[`view_offsets`] is a pointer to an array of [`dependency_count`]
    ///view offsets, one for each dependency.
    ///If [`dependency_count`] is zero, each dependency’s view offset is
    ///treated as zero.
    ///Each view offset controls which views in the source subpass the views in
    ///the destination subpass depend on.
    pub view_offsets: *const i32,
    ///[`correlation_mask_count`] is zero or the number of correlation masks.
    pub correlation_mask_count: u32,
    ///[`correlation_masks`] is a pointer to an array of
    ///[`correlation_mask_count`] view masks indicating sets of views that  **may**
    ///be more efficient to render concurrently.
    pub correlation_masks: *const u32,
}
impl<'lt> Default for RenderPassMultiviewCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RenderPassMultiviewCreateInfo,
            p_next: std::ptr::null(),
            subpass_count: 0,
            view_masks: std::ptr::null(),
            dependency_count: 0,
            view_offsets: std::ptr::null(),
            correlation_mask_count: 0,
            correlation_masks: std::ptr::null(),
        }
    }
}
impl<'lt> RenderPassMultiviewCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::view_masks`]
    pub fn view_masks_raw(&self) -> *const u32 {
        self.view_masks
    }
    ///Gets the raw value of [`Self::view_offsets`]
    pub fn view_offsets_raw(&self) -> *const i32 {
        self.view_offsets
    }
    ///Gets the raw value of [`Self::correlation_masks`]
    pub fn correlation_masks_raw(&self) -> *const u32 {
        self.correlation_masks
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::view_masks`]
    pub fn set_view_masks_raw(&mut self, value: *const u32) -> &mut Self {
        self.view_masks = value;
        self
    }
    ///Sets the raw value of [`Self::view_offsets`]
    pub fn set_view_offsets_raw(&mut self, value: *const i32) -> &mut Self {
        self.view_offsets = value;
        self
    }
    ///Sets the raw value of [`Self::correlation_masks`]
    pub fn set_correlation_masks_raw(&mut self, value: *const u32) -> &mut Self {
        self.correlation_masks = value;
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
    ///Gets the value of [`Self::subpass_count`]
    pub fn subpass_count(&self) -> u32 {
        self.subpass_count
    }
    ///Gets the value of [`Self::view_masks`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn view_masks(&self) -> &[u32] {
        std::slice::from_raw_parts(self.view_masks, self.subpass_count as usize)
    }
    ///Gets the value of [`Self::dependency_count`]
    pub fn dependency_count(&self) -> u32 {
        self.dependency_count
    }
    ///Gets the value of [`Self::view_offsets`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn view_offsets(&self) -> &[i32] {
        std::slice::from_raw_parts(self.view_offsets, self.dependency_count as usize)
    }
    ///Gets the value of [`Self::correlation_mask_count`]
    pub fn correlation_mask_count(&self) -> u32 {
        self.correlation_mask_count
    }
    ///Gets the value of [`Self::correlation_masks`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn correlation_masks(&self) -> &[u32] {
        std::slice::from_raw_parts(self.correlation_masks, self.correlation_mask_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::subpass_count`]
    pub fn subpass_count_mut(&mut self) -> &mut u32 {
        &mut self.subpass_count
    }
    ///Gets a mutable reference to the value of [`Self::dependency_count`]
    pub fn dependency_count_mut(&mut self) -> &mut u32 {
        &mut self.dependency_count
    }
    ///Gets a mutable reference to the value of [`Self::correlation_mask_count`]
    pub fn correlation_mask_count_mut(&mut self) -> &mut u32 {
        &mut self.correlation_mask_count
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
    ///Sets the raw value of [`Self::subpass_count`]
    pub fn set_subpass_count(&mut self, value: u32) -> &mut Self {
        self.subpass_count = value;
        self
    }
    ///Sets the raw value of [`Self::view_masks`]
    pub fn set_view_masks(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.view_masks = value.as_ptr();
        self.subpass_count = len_;
        self
    }
    ///Sets the raw value of [`Self::dependency_count`]
    pub fn set_dependency_count(&mut self, value: u32) -> &mut Self {
        self.dependency_count = value;
        self
    }
    ///Sets the raw value of [`Self::view_offsets`]
    pub fn set_view_offsets(&mut self, value: &'lt [i32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.view_offsets = value.as_ptr();
        self.dependency_count = len_;
        self
    }
    ///Sets the raw value of [`Self::correlation_mask_count`]
    pub fn set_correlation_mask_count(&mut self, value: u32) -> &mut Self {
        self.correlation_mask_count = value;
        self
    }
    ///Sets the raw value of [`Self::correlation_masks`]
    pub fn set_correlation_masks(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.correlation_masks = value.as_ptr();
        self.correlation_mask_count = len_;
        self
    }
}
///[VkPhysicalDeviceGroupProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceGroupProperties.html) - Structure specifying physical device group properties
///# C Specifications
///The [`PhysicalDeviceGroupProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceGroupProperties {
///    VkStructureType     sType;
///    void*               pNext;
///    uint32_t            physicalDeviceCount;
///    VkPhysicalDevice    physicalDevices[VK_MAX_DEVICE_GROUP_SIZE];
///    VkBool32            subsetAllocation;
///} VkPhysicalDeviceGroupProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group_creation
///typedef VkPhysicalDeviceGroupProperties VkPhysicalDeviceGroupPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`physical_device_count`] is the number of physical devices in the group.
/// - [`physical_devices`] is an array of [`MAX_DEVICE_GROUP_SIZE`][`PhysicalDevice`] handles
///   representing all physical devices in the group. The first [`physical_device_count`] elements
///   of the array will be valid.
/// - [`subset_allocation`] specifies whether logical devices created from the group support
///   allocating device memory on a subset of devices, via the `deviceMask` member of the
///   [`MemoryAllocateFlagsInfo`]. If this is [`FALSE`], then all device memory allocations are made
///   across all physical devices in the group. If [`physical_device_count`] is `1`, then
///   [`subset_allocation`] **must**  be [`FALSE`].
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Bool32`]
/// - [`PhysicalDevice`]
/// - [`StructureType`]
/// - [`EnumeratePhysicalDeviceGroups`]
/// - [`EnumeratePhysicalDeviceGroupsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceGroupProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceGroupProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`physical_device_count`] is the number of physical devices in the
    ///group.
    pub physical_device_count: u32,
    ///[`physical_devices`] is an array of [`MAX_DEVICE_GROUP_SIZE`][`PhysicalDevice`] handles
    /// representing all physical devices in the group.
    ///The first [`physical_device_count`] elements of the array will be valid.
    pub physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
    ///[`subset_allocation`] specifies whether logical devices created from
    ///the group support allocating device memory on a subset of devices, via
    ///the `deviceMask` member of the [`MemoryAllocateFlagsInfo`].
    ///If this is [`FALSE`], then all device memory allocations are made
    ///across all physical devices in the group.
    ///If [`physical_device_count`] is `1`, then [`subset_allocation`] **must**
    ///be [`FALSE`].
    pub subset_allocation: Bool32,
}
impl<'lt> Default for PhysicalDeviceGroupProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceGroupProperties,
            p_next: std::ptr::null_mut(),
            physical_device_count: 0,
            physical_devices: [Default::default(); MAX_DEVICE_GROUP_SIZE as usize],
            subset_allocation: 0,
        }
    }
}
impl<'lt> PhysicalDeviceGroupProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::subset_allocation`]
    pub fn subset_allocation_raw(&self) -> Bool32 {
        self.subset_allocation
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::subset_allocation`]
    pub fn set_subset_allocation_raw(&mut self, value: Bool32) -> &mut Self {
        self.subset_allocation = value;
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
    ///Gets the value of [`Self::physical_device_count`]
    pub fn physical_device_count(&self) -> u32 {
        self.physical_device_count
    }
    ///Gets the value of [`Self::physical_devices`]
    pub fn physical_devices(&self) -> &[PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize] {
        &self.physical_devices
    }
    ///Gets the value of [`Self::subset_allocation`]
    pub fn subset_allocation(&self) -> bool {
        unsafe { std::mem::transmute(self.subset_allocation as u8) }
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
    ///Gets a mutable reference to the value of [`Self::physical_device_count`]
    pub fn physical_device_count_mut(&mut self) -> &mut u32 {
        &mut self.physical_device_count
    }
    ///Gets a mutable reference to the value of [`Self::physical_devices`]
    pub fn physical_devices_mut(&mut self) -> &mut [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize] {
        &mut self.physical_devices
    }
    ///Gets a mutable reference to the value of [`Self::subset_allocation`]
    pub fn subset_allocation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.subset_allocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.subset_allocation as *mut Bool32)
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
    ///Sets the raw value of [`Self::physical_device_count`]
    pub fn set_physical_device_count(&mut self, value: u32) -> &mut Self {
        self.physical_device_count = value;
        self
    }
    ///Sets the raw value of [`Self::physical_devices`]
    pub fn set_physical_devices(
        &mut self,
        value: [crate::vulkan1_0::PhysicalDevice; crate::vulkan1_1::MAX_DEVICE_GROUP_SIZE as usize],
    ) -> &mut Self {
        self.physical_devices = value;
        self
    }
    ///Sets the raw value of [`Self::subset_allocation`]
    pub fn set_subset_allocation(&mut self, value: bool) -> &mut Self {
        self.subset_allocation = value as u8 as u32;
        self
    }
}
///[VkMemoryAllocateFlagsInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateFlagsInfo.html) - Structure controlling how many instances of memory will be allocated
///# C Specifications
///If the [`p_next`] chain of [`MemoryAllocateInfo`] includes a
///[`MemoryAllocateFlagsInfo`] structure, then that structure includes
///flags and a device mask controlling how many instances of the memory will be
///allocated.The [`MemoryAllocateFlagsInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkMemoryAllocateFlagsInfo {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkMemoryAllocateFlags    flags;
///    uint32_t                 deviceMask;
///} VkMemoryAllocateFlagsInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group
///typedef VkMemoryAllocateFlagsInfo VkMemoryAllocateFlagsInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`MemoryAllocateFlagBits`] controlling the allocation.
/// - [`device_mask`] is a mask of physical devices in the logical device, indicating that memory
///   **must**  be allocated on each device in the mask, if `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is
///   set in [`flags`].
///# Description
///If `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is not set, the number of
///instances allocated depends on whether
///`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` is set in the memory heap.
///If `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` is set, then memory is allocated
///for every physical device in the logical device (as if [`device_mask`] has
///bits set for all device indices).
///If `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` is not set, then a single
///instance of memory is allocated (as if [`device_mask`] is set to one).On some implementations,
/// allocations from a multi-instance heap  **may**  consume
///memory on all physical devices even if the [`device_mask`] excludes some
///devices.
///If [`PhysicalDeviceGroupProperties::subset_allocation`] is
///[`TRUE`], then memory is only consumed for the devices in the device
///mask.
///## Valid Usage
/// - If `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is set, [`device_mask`] **must**  be a valid device
///   mask
/// - If `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is set, [`device_mask`] **must**  not be zero
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO`
/// - [`flags`] **must**  be a valid combination of [`MemoryAllocateFlagBits`] values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`MemoryAllocateFlags`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryAllocateFlagsInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryAllocateFlagsInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`MemoryAllocateFlagBits`] controlling
    ///the allocation.
    pub flags: MemoryAllocateFlags,
    ///[`device_mask`] is a mask of physical devices in the logical device,
    ///indicating that memory  **must**  be allocated on each device in the mask, if
    ///`VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is set in [`flags`].
    pub device_mask: u32,
}
impl<'lt> Default for MemoryAllocateFlagsInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MemoryAllocateFlagsInfo,
            p_next: std::ptr::null(),
            flags: Default::default(),
            device_mask: 0,
        }
    }
}
impl<'lt> MemoryAllocateFlagsInfo<'lt> {
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
    pub fn flags(&self) -> MemoryAllocateFlags {
        self.flags
    }
    ///Gets the value of [`Self::device_mask`]
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut MemoryAllocateFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::device_mask`]
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
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
    pub fn set_flags(&mut self, value: crate::vulkan1_1::MemoryAllocateFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::device_mask`]
    pub fn set_device_mask(&mut self, value: u32) -> &mut Self {
        self.device_mask = value;
        self
    }
}
///[VkBindBufferMemoryInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryInfo.html) - Structure specifying how to bind a buffer to memory
///# C Specifications
///[`BindBufferMemoryInfo`] contains members corresponding to the
///parameters of [`BindBufferMemory`].The [`BindBufferMemoryInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkBindBufferMemoryInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBuffer           buffer;
///    VkDeviceMemory     memory;
///    VkDeviceSize       memoryOffset;
///} VkBindBufferMemoryInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_bind_memory2
///typedef VkBindBufferMemoryInfo VkBindBufferMemoryInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer`] is the buffer to be attached to memory.
/// - [`memory`] is a [`DeviceMemory`] object describing the device memory to attach.
/// - [`memory_offset`] is the start offset of the region of [`memory`] which is to be bound to the
///   buffer. The number of bytes returned in the [`MemoryRequirements::size`] member in [`memory`],
///   starting from [`memory_offset`] bytes, will be bound to the specified buffer.
///# Description
///## Valid Usage
/// - [`buffer`] **must**  not already be backed by a memory object
/// - [`buffer`] **must**  not have been created with any sparse memory binding flags
/// - [`memory_offset`] **must**  be less than the size of [`memory`]
/// - [`memory`] **must**  have been allocated using one of the memory types allowed in the
///   `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetBufferMemoryRequirements`] with [`buffer`]
/// - [`memory_offset`] **must**  be an integer multiple of the `alignment` member of the
///   [`MemoryRequirements`] structure returned from a call to [`GetBufferMemoryRequirements`] with
///   [`buffer`]
/// - The `size` member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetBufferMemoryRequirements`] with [`buffer`] **must**  be less than or equal to the size of
///   [`memory`] minus [`memory_offset`]
/// - If [`buffer`] requires a dedicated allocation (as reported by [`GetBufferMemoryRequirements2`]
///   in [`MemoryDedicatedRequirements::requires_dedicated_allocation`] for [`buffer`]), [`memory`]
///   **must**  have been allocated with [`MemoryDedicatedAllocateInfo`]::[`buffer`] equal to
///   [`buffer`]
/// - If the [`MemoryAllocateInfo`] provided when [`memory`] was allocated included a
///   [`MemoryDedicatedAllocateInfo`] structure in its [`p_next`] chain, and
///   [`MemoryDedicatedAllocateInfo`]::[`buffer`] was not [`crate::utils::Handle::null`], then
///   [`buffer`] **must**  equal [`MemoryDedicatedAllocateInfo`]::[`buffer`], and [`memory_offset`]
///   **must**  be zero
/// - If [`buffer`] was created with the `VK_BUFFER_CREATE_PROTECTED_BIT` bit set, the buffer
///   **must**  be bound to a memory object allocated with a memory type that reports
///   `VK_MEMORY_PROPERTY_PROTECTED_BIT`
/// - If [`buffer`] was created with the `VK_BUFFER_CREATE_PROTECTED_BIT` bit not set, the buffer
///   **must**  not be bound to a memory object allocated with a memory type that reports
///   `VK_MEMORY_PROPERTY_PROTECTED_BIT`
/// - If [`buffer`] was created with [`DedicatedAllocationBufferCreateInfoNV::dedicated_allocation`]
///   equal to [`TRUE`], [`memory`] **must**  have been allocated with
///   [`DedicatedAllocationMemoryAllocateInfoNV`]::[`buffer`] equal to a buffer handle created with
///   identical creation parameters to [`buffer`] and [`memory_offset`] **must**  be zero
/// - If the value of [`ExportMemoryAllocateInfo::handle_types`] used to allocate [`memory`] is not
///   `0`, it  **must**  include at least one of the handles set in
///   [`ExternalMemoryBufferCreateInfo::handle_types`] when [`buffer`] was created
/// - If [`memory`] was allocated by a memory import operation, that is not
///   [`ImportAndroidHardwareBufferInfoANDROID`] with a non-`NULL`[`buffer`] value, the external
///   handle type of the imported memory  **must**  also have been set in
///   [`ExternalMemoryBufferCreateInfo::handle_types`] when [`buffer`] was created
/// - If [`memory`] was allocated with the [`ImportAndroidHardwareBufferInfoANDROID`] memory import
///   operation with a non-`NULL`[`buffer`] value,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` **must**  also have been
///   set in [`ExternalMemoryBufferCreateInfo::handle_types`] when [`buffer`] was created
/// - If the [`PhysicalDeviceBufferDeviceAddressFeatures::buffer_device_address`] feature is enabled
///   and [`buffer`] was created with the `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` bit set,
///   [`memory`] **must**  have been allocated with the `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT` bit
///   set
/// - If [`buffer`] was created with [`BufferCollectionBufferCreateInfoFUCHSIA`] chained to
///   [`BufferCreateInfo`]::[`p_next`], [`memory`] **must**  be allocated with a
///   [`ImportMemoryBufferCollectionFUCHSIA`] chained to [`MemoryAllocateInfo`]::[`p_next`]
/// - If the [`p_next`] chain includes a [`BindBufferMemoryDeviceGroupInfo`] structure, all
///   instances of [`memory`] specified by [`BindBufferMemoryDeviceGroupInfo::device_indices`]
///   **must**  have been allocated
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`BindBufferMemoryDeviceGroupInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`buffer`] **must**  be a valid [`Buffer`] handle
/// - [`memory`] **must**  be a valid [`DeviceMemory`] handle
/// - Both of [`buffer`], and [`memory`] **must**  have been created, allocated, or retrieved from
///   the same [`Device`]
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Buffer`]
/// - [`DeviceMemory`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`BindBufferMemory2`]
/// - [`BindBufferMemory2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindBufferMemoryInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BindBufferMemoryInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`buffer`] is the buffer to be attached to memory.
    pub buffer: Buffer,
    ///[`memory`] is a [`DeviceMemory`] object describing the device
    ///memory to attach.
    pub memory: DeviceMemory,
    ///[`memory_offset`] is the start offset of the region of [`memory`]
    ///which is to be bound to the buffer.
    ///The number of bytes returned in the
    ///[`MemoryRequirements`]::`size` member in [`memory`], starting
    ///from [`memory_offset`] bytes, will be bound to the specified buffer.
    pub memory_offset: DeviceSize,
}
impl<'lt> Default for BindBufferMemoryInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BindBufferMemoryInfo,
            p_next: std::ptr::null(),
            buffer: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
        }
    }
}
impl<'lt> BindBufferMemoryInfo<'lt> {
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
    ///Gets the value of [`Self::memory`]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }
    ///Gets the value of [`Self::memory_offset`]
    pub fn memory_offset(&self) -> DeviceSize {
        self.memory_offset
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::buffer`]
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::memory`]
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Gets a mutable reference to the value of [`Self::memory_offset`]
    pub fn memory_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_offset
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
    ///Sets the raw value of [`Self::memory`]
    pub fn set_memory(&mut self, value: crate::vulkan1_0::DeviceMemory) -> &mut Self {
        self.memory = value;
        self
    }
    ///Sets the raw value of [`Self::memory_offset`]
    pub fn set_memory_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.memory_offset = value;
        self
    }
}
///[VkBindBufferMemoryDeviceGroupInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindBufferMemoryDeviceGroupInfo.html) - Structure specifying device within a group to bind to
///# C Specifications
///The [`BindBufferMemoryDeviceGroupInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkBindBufferMemoryDeviceGroupInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           deviceIndexCount;
///    const uint32_t*    pDeviceIndices;
///} VkBindBufferMemoryDeviceGroupInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_bind_memory2 with VK_KHR_device_group
///typedef VkBindBufferMemoryDeviceGroupInfo VkBindBufferMemoryDeviceGroupInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_index_count`] is the number of elements in [`device_indices`].
/// - [`device_indices`] is a pointer to an array of device indices.
///# Description
///If the [`p_next`] chain of [`BindBufferMemoryInfo`] includes a
///[`BindBufferMemoryDeviceGroupInfo`] structure, then that structure
///determines how memory is bound to buffers across multiple devices in a
///device group.If [`device_index_count`] is greater than zero, then on device index i
///the buffer is attached to the instance of `memory` on the physical
///device with device index [`device_indices`][i].If [`device_index_count`] is zero and `memory`
/// comes from a memory heap
///with the `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as if
///[`device_indices`] contains consecutive indices from zero to the number of
///physical devices in the logical device, minus one.
///In other words, by default each physical device attaches to its own instance
///of `memory`.If [`device_index_count`] is zero and `memory` comes from a memory heap
///without the `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as
///if [`device_indices`] contains an array of zeros.
///In other words, by default each physical device attaches to instance zero.
///## Valid Usage
/// - [`device_index_count`] **must**  either be zero or equal to the number of physical devices in
///   the logical device
/// - All elements of [`device_indices`] **must**  be valid device indices
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO`
/// - If [`device_index_count`] is not `0`, [`device_indices`] **must**  be a valid pointer to an
///   array of [`device_index_count`]`uint32_t` values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`device_index_count`] is the number of elements in
    ///[`device_indices`].
    pub device_index_count: u32,
    ///[`device_indices`] is a pointer to an array of device indices.
    pub device_indices: *const u32,
}
impl<'lt> Default for BindBufferMemoryDeviceGroupInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BindBufferMemoryDeviceGroupInfo,
            p_next: std::ptr::null(),
            device_index_count: 0,
            device_indices: std::ptr::null(),
        }
    }
}
impl<'lt> BindBufferMemoryDeviceGroupInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::device_indices`]
    pub fn device_indices_raw(&self) -> *const u32 {
        self.device_indices
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_indices`]
    pub fn set_device_indices_raw(&mut self, value: *const u32) -> &mut Self {
        self.device_indices = value;
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
    ///Gets the value of [`Self::device_index_count`]
    pub fn device_index_count(&self) -> u32 {
        self.device_index_count
    }
    ///Gets the value of [`Self::device_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn device_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.device_indices, self.device_index_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::device_index_count`]
    pub fn device_index_count_mut(&mut self) -> &mut u32 {
        &mut self.device_index_count
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
    ///Sets the raw value of [`Self::device_index_count`]
    pub fn set_device_index_count(&mut self, value: u32) -> &mut Self {
        self.device_index_count = value;
        self
    }
    ///Sets the raw value of [`Self::device_indices`]
    pub fn set_device_indices(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.device_indices = value.as_ptr();
        self.device_index_count = len_;
        self
    }
}
///[VkBindImageMemoryInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryInfo.html) - Structure specifying how to bind an image to memory
///# C Specifications
///[`BindImageMemoryInfo`] contains members corresponding to the parameters
///of [`BindImageMemory`].The [`BindImageMemoryInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkBindImageMemoryInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkImage            image;
///    VkDeviceMemory     memory;
///    VkDeviceSize       memoryOffset;
///} VkBindImageMemoryInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_bind_memory2
///typedef VkBindImageMemoryInfo VkBindImageMemoryInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image`] is the image to be attached to memory.
/// - [`memory`] is a [`DeviceMemory`] object describing the device memory to attach.
/// - [`memory_offset`] is the start offset of the region of [`memory`] which is to be bound to the
///   image. The number of bytes returned in the [`MemoryRequirements::size`] member in [`memory`],
///   starting from [`memory_offset`] bytes, will be bound to the specified image.
///# Description
///## Valid Usage
/// - [`image`] **must**  not already be backed by a memory object
/// - [`image`] **must**  not have been created with any sparse memory binding flags
/// - [`memory_offset`] **must**  be less than the size of [`memory`]
/// - If [`image`] requires a dedicated allocation (as reported by [`GetImageMemoryRequirements2`]
///   in [`MemoryDedicatedRequirements::requires_dedicated_allocation`] for [`image`]), [`memory`]
///   **must**  have been created with [`MemoryDedicatedAllocateInfo`]::[`image`] equal to [`image`]
/// - If the [dedicated allocation image aliasing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-dedicatedAllocationImageAliasing)
///   feature is not enabled, and the [`MemoryAllocateInfo`] provided when [`memory`] was allocated
///   included a [`MemoryDedicatedAllocateInfo`] structure in its [`p_next`] chain, and
///   [`MemoryDedicatedAllocateInfo`]::[`image`] was not [`crate::utils::Handle::null`], then
///   [`image`] **must**  equal [`MemoryDedicatedAllocateInfo`]::[`image`] and [`memory_offset`]
///   **must**  be zero
/// - If the [dedicated allocation image aliasing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-dedicatedAllocationImageAliasing)
///   feature is enabled, and the [`MemoryAllocateInfo`] provided when [`memory`] was allocated
///   included a [`MemoryDedicatedAllocateInfo`] structure in its [`p_next`] chain, and
///   [`MemoryDedicatedAllocateInfo`]::[`image`] was not [`crate::utils::Handle::null`], then
///   [`memory_offset`] **must**  be zero, and [`image`] **must**  be either equal to
///   [`MemoryDedicatedAllocateInfo`]::[`image`] or an image that was created using the same
///   parameters in [`ImageCreateInfo`], with the exception that `extent` and `arrayLayers` **may**
///   differ subject to the following restrictions: every dimension in the `extent` parameter of the
///   image being bound  **must**  be equal to or smaller than the original image for which the
///   allocation was created; and the `arrayLayers` parameter of the image being bound  **must**  be
///   equal to or smaller than the original image for which the allocation was created
/// - If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit set, the image  **must**  be
///   bound to a memory object allocated with a memory type that reports
///   `VK_MEMORY_PROPERTY_PROTECTED_BIT`
/// - If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit not set, the image  **must**
///   not be bound to a memory object created with a memory type that reports
///   `VK_MEMORY_PROPERTY_PROTECTED_BIT`
/// - If [`image`] was created with [`DedicatedAllocationImageCreateInfoNV::dedicated_allocation`]
///   equal to [`TRUE`], [`memory`] **must**  have been created with
///   [`DedicatedAllocationMemoryAllocateInfoNV`]::[`image`] equal to an image handle created with
///   identical creation parameters to [`image`] and [`memory_offset`] **must**  be zero
/// - If the value of [`ExportMemoryAllocateInfo::handle_types`] used to allocate [`memory`] is not
///   `0`, it  **must**  include at least one of the handles set in
///   [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
/// - If [`memory`] was created by a memory import operation, that is not
///   [`ImportAndroidHardwareBufferInfoANDROID`] with a non-`NULL``buffer` value, the external
///   handle type of the imported memory  **must**  also have been set in
///   [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
/// - If [`memory`] was created with the [`ImportAndroidHardwareBufferInfoANDROID`] memory import
///   operation with a non-`NULL``buffer` value,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` **must**  also have been
///   set in [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
/// - If the [`p_next`] chain does not include a [`BindImagePlaneMemoryInfo`] structure, [`memory`]
///   **must**  have been allocated using one of the memory types allowed in the `memoryTypeBits`
///   member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetImageMemoryRequirements2`] with [`image`]
/// - If the [`p_next`] chain does not include a [`BindImagePlaneMemoryInfo`] structure,
///   [`memory_offset`] **must**  be an integer multiple of the `alignment` member of the
///   [`MemoryRequirements`] structure returned from a call to [`GetImageMemoryRequirements2`] with
///   [`image`]
/// - If the [`p_next`] chain does not include a [`BindImagePlaneMemoryInfo`] structure, the
///   difference of the size of [`memory`] and [`memory_offset`] **must**  be greater than or equal
///   to the `size` member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetImageMemoryRequirements2`] with the same [`image`]
/// - If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, [`image`] **must**
///   have been created with the `VK_IMAGE_CREATE_DISJOINT_BIT` bit set
/// - If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, [`memory`] **must**
///   have been allocated using one of the memory types allowed in the `memoryTypeBits` member of
///   the [`MemoryRequirements`] structure returned from a call to [`GetImageMemoryRequirements2`]
///   with [`image`] and where [`BindImagePlaneMemoryInfo::plane_aspect`] corresponds to the
///   [`ImagePlaneMemoryRequirementsInfo::plane_aspect`] in the [`ImageMemoryRequirementsInfo2`]
///   structure’s [`p_next`] chain
/// - If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, [`memory_offset`]
///   **must**  be an integer multiple of the `alignment` member of the [`MemoryRequirements`]
///   structure returned from a call to [`GetImageMemoryRequirements2`] with [`image`] and where
///   [`BindImagePlaneMemoryInfo::plane_aspect`] corresponds to the
///   [`ImagePlaneMemoryRequirementsInfo::plane_aspect`] in the [`ImageMemoryRequirementsInfo2`]
///   structure’s [`p_next`] chain
/// - If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, the difference of
///   the size of [`memory`] and [`memory_offset`] **must**  be greater than or equal to the `size`
///   member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetImageMemoryRequirements2`] with the same [`image`] and where
///   [`BindImagePlaneMemoryInfo::plane_aspect`] corresponds to the
///   [`ImagePlaneMemoryRequirementsInfo::plane_aspect`] in the [`ImageMemoryRequirementsInfo2`]
///   structure’s [`p_next`] chain
/// - If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, all instances
///   of [`memory`] specified by [`BindImageMemoryDeviceGroupInfo::device_indices`] **must**  have
///   been allocated
/// - If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, and
///   [`BindImageMemoryDeviceGroupInfo::split_instance_bind_region_count`] is not zero, then
///   [`image`] **must**  have been created with the
///   `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT` bit set
/// - If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, all elements
///   of [`BindImageMemoryDeviceGroupInfo::split_instance_bind_regions`] **must**  be valid
///   rectangles contained within the dimensions of [`image`]
/// - If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, the union of
///   the areas of all elements of [`BindImageMemoryDeviceGroupInfo::split_instance_bind_regions`]
///   that correspond to the same instance of [`image`] **must**  cover the entire image
/// - If [`image`] was created with a valid swapchain handle in
///   [`ImageSwapchainCreateInfoKHR::swapchain`], then the [`p_next`] chain  **must**  include a
///   [`BindImageMemorySwapchainInfoKHR`] structure containing the same swapchain handle
/// - If the [`p_next`] chain includes a [`BindImageMemorySwapchainInfoKHR`] structure, [`memory`]
///   **must**  be [`crate::utils::Handle::null`]
/// - If the [`p_next`] chain does not include a [`BindImageMemorySwapchainInfoKHR`] structure,
///   [`memory`] **must**  be a valid [`DeviceMemory`] handle
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**
///   be either `NULL` or a pointer to a valid instance of [`BindImageMemoryDeviceGroupInfo`],
///   [`BindImageMemorySwapchainInfoKHR`], or [`BindImagePlaneMemoryInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`image`] **must**  be a valid [`Image`] handle
/// - Both of [`image`], and [`memory`] that are valid handles of non-ignored parameters  **must**
///   have been created, allocated, or retrieved from the same [`Device`]
///# Related
/// - [`crate::vulkan1_1`]
/// - [`DeviceMemory`]
/// - [`DeviceSize`]
/// - [`Image`]
/// - [`StructureType`]
/// - [`BindImageMemory2`]
/// - [`BindImageMemory2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindImageMemoryInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BindImageMemoryInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`image`] is the image to be attached to memory.
    pub image: Image,
    ///[`memory`] is a [`DeviceMemory`] object describing the device
    ///memory to attach.
    pub memory: DeviceMemory,
    ///[`memory_offset`] is the start offset of the region of [`memory`]
    ///which is to be bound to the image.
    ///The number of bytes returned in the
    ///[`MemoryRequirements`]::`size` member in [`memory`], starting
    ///from [`memory_offset`] bytes, will be bound to the specified image.
    pub memory_offset: DeviceSize,
}
impl<'lt> Default for BindImageMemoryInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BindImageMemoryInfo,
            p_next: std::ptr::null(),
            image: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
        }
    }
}
impl<'lt> BindImageMemoryInfo<'lt> {
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
    ///Gets the value of [`Self::image`]
    pub fn image(&self) -> Image {
        self.image
    }
    ///Gets the value of [`Self::memory`]
    pub fn memory(&self) -> DeviceMemory {
        self.memory
    }
    ///Gets the value of [`Self::memory_offset`]
    pub fn memory_offset(&self) -> DeviceSize {
        self.memory_offset
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image`]
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
    }
    ///Gets a mutable reference to the value of [`Self::memory`]
    pub fn memory_mut(&mut self) -> &mut DeviceMemory {
        &mut self.memory
    }
    ///Gets a mutable reference to the value of [`Self::memory_offset`]
    pub fn memory_offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.memory_offset
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
    ///Sets the raw value of [`Self::image`]
    pub fn set_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.image = value;
        self
    }
    ///Sets the raw value of [`Self::memory`]
    pub fn set_memory(&mut self, value: crate::vulkan1_0::DeviceMemory) -> &mut Self {
        self.memory = value;
        self
    }
    ///Sets the raw value of [`Self::memory_offset`]
    pub fn set_memory_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.memory_offset = value;
        self
    }
}
///[VkBindImageMemoryDeviceGroupInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemoryDeviceGroupInfo.html) - Structure specifying device within a group to bind to
///# C Specifications
///The [`BindImageMemoryDeviceGroupInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkBindImageMemoryDeviceGroupInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           deviceIndexCount;
///    const uint32_t*    pDeviceIndices;
///    uint32_t           splitInstanceBindRegionCount;
///    const VkRect2D*    pSplitInstanceBindRegions;
///} VkBindImageMemoryDeviceGroupInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_bind_memory2 with VK_KHR_device_group
///typedef VkBindImageMemoryDeviceGroupInfo VkBindImageMemoryDeviceGroupInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_index_count`] is the number of elements in [`device_indices`].
/// - [`device_indices`] is a pointer to an array of device indices.
/// - [`split_instance_bind_region_count`] is the number of elements in
///   [`split_instance_bind_regions`].
/// - [`split_instance_bind_regions`] is a pointer to an array of [`Rect2D`] structures describing
///   which regions of the image are attached to each instance of memory.
///# Description
///If the [`p_next`] chain of [`BindImageMemoryInfo`] includes a
///[`BindImageMemoryDeviceGroupInfo`] structure, then that structure
///determines how memory is bound to images across multiple devices in a device
///group.If [`device_index_count`] is greater than zero, then on device index i`image` is attached
/// to the instance of the memory on the physical device
///with device index pDeviceIndices[i].Let N be the number of physical devices in the logical
/// device.
///If [`split_instance_bind_region_count`] is greater than zero, then
///[`split_instance_bind_regions`] is a pointer to an array of N<sup>2</sup>
///rectangles, where the image region specified by the rectangle at element
///i*N+j in resource instance i is bound to the memory instance
///j.
///The blocks of the memory that are bound to each sparse image block region
///use an offset in memory, relative to `memoryOffset`, computed as if the
///whole image was being bound to a contiguous range of memory.
///In other words, horizontally adjacent image blocks use consecutive blocks of
///memory, vertically adjacent image blocks are separated by the number of
///bytes per block multiplied by the width in blocks of `image`, and the
///block at (0,0) corresponds to memory starting at `memoryOffset`.If
/// [`split_instance_bind_region_count`] and [`device_index_count`] are zero
///and the memory comes from a memory heap with the
///`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as if
///[`device_indices`] contains consecutive indices from zero to the number of
///physical devices in the logical device, minus one.
///In other words, by default each physical device attaches to its own instance
///of the memory.If [`split_instance_bind_region_count`] and [`device_index_count`] are zero
///and the memory comes from a memory heap without the
///`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as if
///[`device_indices`] contains an array of zeros.
///In other words, by default each physical device attaches to instance zero.
///## Valid Usage
/// - At least one of [`device_index_count`] and [`split_instance_bind_region_count`] **must**  be
///   zero
/// - [`device_index_count`] **must**  either be zero or equal to the number of physical devices in
///   the logical device
/// - All elements of [`device_indices`] **must**  be valid device indices
/// - [`split_instance_bind_region_count`] **must**  either be zero or equal to the number of
///   physical devices in the logical device squared
/// - Elements of [`split_instance_bind_regions`] that correspond to the same instance of an image
///   **must**  not overlap
/// - The `offset.x` member of any element of [`split_instance_bind_regions`] **must**  be a
///   multiple of the sparse image block width
///   ([`SparseImageFormatProperties`]::`imageGranularity.width`) of all non-metadata aspects of the
///   image
/// - The `offset.y` member of any element of [`split_instance_bind_regions`] **must**  be a
///   multiple of the sparse image block height
///   ([`SparseImageFormatProperties`]::`imageGranularity.height`) of all non-metadata aspects of
///   the image
/// - The `extent.width` member of any element of [`split_instance_bind_regions`] **must**  either
///   be a multiple of the sparse image block width of all non-metadata aspects of the image, or
///   else `extent.width` +  `offset.x` **must**  equal the width of the image subresource
/// - The `extent.height` member of any element of [`split_instance_bind_regions`] **must**  either
///   be a multiple of the sparse image block height of all non-metadata aspects of the image, or
///   else `extent.height` +  `offset.y` **must**  equal the height of the image subresource
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO`
/// - If [`device_index_count`] is not `0`, [`device_indices`] **must**  be a valid pointer to an
///   array of [`device_index_count`]`uint32_t` values
/// - If [`split_instance_bind_region_count`] is not `0`, [`split_instance_bind_regions`] **must**
///   be a valid pointer to an array of [`split_instance_bind_region_count`][`Rect2D`] structures
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Rect2D`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindImageMemoryDeviceGroupInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`device_index_count`] is the number of elements in
    ///[`device_indices`].
    pub device_index_count: u32,
    ///[`device_indices`] is a pointer to an array of device indices.
    pub device_indices: *const u32,
    ///[`split_instance_bind_region_count`] is the number of elements in
    ///[`split_instance_bind_regions`].
    pub split_instance_bind_region_count: u32,
    ///[`split_instance_bind_regions`] is a pointer to an array of
    ///[`Rect2D`] structures describing which regions of the image are
    ///attached to each instance of memory.
    pub split_instance_bind_regions: *const Rect2D,
}
impl<'lt> Default for BindImageMemoryDeviceGroupInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BindImageMemoryDeviceGroupInfo,
            p_next: std::ptr::null(),
            device_index_count: 0,
            device_indices: std::ptr::null(),
            split_instance_bind_region_count: 0,
            split_instance_bind_regions: std::ptr::null(),
        }
    }
}
impl<'lt> BindImageMemoryDeviceGroupInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::device_indices`]
    pub fn device_indices_raw(&self) -> *const u32 {
        self.device_indices
    }
    ///Gets the raw value of [`Self::split_instance_bind_regions`]
    pub fn split_instance_bind_regions_raw(&self) -> *const Rect2D {
        self.split_instance_bind_regions
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_indices`]
    pub fn set_device_indices_raw(&mut self, value: *const u32) -> &mut Self {
        self.device_indices = value;
        self
    }
    ///Sets the raw value of [`Self::split_instance_bind_regions`]
    pub fn set_split_instance_bind_regions_raw(&mut self, value: *const Rect2D) -> &mut Self {
        self.split_instance_bind_regions = value;
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
    ///Gets the value of [`Self::device_index_count`]
    pub fn device_index_count(&self) -> u32 {
        self.device_index_count
    }
    ///Gets the value of [`Self::device_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn device_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.device_indices, self.device_index_count as usize)
    }
    ///Gets the value of [`Self::split_instance_bind_region_count`]
    pub fn split_instance_bind_region_count(&self) -> u32 {
        self.split_instance_bind_region_count
    }
    ///Gets the value of [`Self::split_instance_bind_regions`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn split_instance_bind_regions(&self) -> &[Rect2D] {
        std::slice::from_raw_parts(
            self.split_instance_bind_regions,
            self.split_instance_bind_region_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::device_index_count`]
    pub fn device_index_count_mut(&mut self) -> &mut u32 {
        &mut self.device_index_count
    }
    ///Gets a mutable reference to the value of [`Self::split_instance_bind_region_count`]
    pub fn split_instance_bind_region_count_mut(&mut self) -> &mut u32 {
        &mut self.split_instance_bind_region_count
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
    ///Sets the raw value of [`Self::device_index_count`]
    pub fn set_device_index_count(&mut self, value: u32) -> &mut Self {
        self.device_index_count = value;
        self
    }
    ///Sets the raw value of [`Self::device_indices`]
    pub fn set_device_indices(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.device_indices = value.as_ptr();
        self.device_index_count = len_;
        self
    }
    ///Sets the raw value of [`Self::split_instance_bind_region_count`]
    pub fn set_split_instance_bind_region_count(&mut self, value: u32) -> &mut Self {
        self.split_instance_bind_region_count = value;
        self
    }
    ///Sets the raw value of [`Self::split_instance_bind_regions`]
    pub fn set_split_instance_bind_regions(&mut self, value: &'lt [crate::vulkan1_0::Rect2D]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.split_instance_bind_regions = value.as_ptr();
        self.split_instance_bind_region_count = len_;
        self
    }
}
///[VkDeviceGroupRenderPassBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupRenderPassBeginInfo.html) - Set the initial device mask and render areas for a render pass instance
///# C Specifications
///If the [`p_next`] chain of [`RenderPassBeginInfo`]
///or [`RenderingInfo`]
///includes a [`DeviceGroupRenderPassBeginInfo`] structure, then that
///structure includes a device mask and set of render areas for the render pass
///instance.The [`DeviceGroupRenderPassBeginInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkDeviceGroupRenderPassBeginInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           deviceMask;
///    uint32_t           deviceRenderAreaCount;
///    const VkRect2D*    pDeviceRenderAreas;
///} VkDeviceGroupRenderPassBeginInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group
///typedef VkDeviceGroupRenderPassBeginInfo VkDeviceGroupRenderPassBeginInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_mask`] is the device mask for the render pass instance.
/// - [`device_render_area_count`] is the number of elements in the [`device_render_areas`] array.
/// - [`device_render_areas`] is a pointer to an array of [`Rect2D`] structures defining the render
///   area for each physical device.
///# Description
///The [`device_mask`] serves several purposes.
///It is an upper bound on the set of physical devices that  **can**  be used during
///the render pass instance, and the initial device mask when the render pass
///instance begins.
///In addition, commands transitioning to the next subpass in a render pass
///instance and commands ending the render pass instance, and, accordingly
///render pass attachment load, store, and resolve operations and subpass
///dependencies corresponding to the render pass instance, are executed on the
///physical devices included in the device mask provided here.If [`device_render_area_count`] is
/// not zero, then the elements of
///[`device_render_areas`] override the value of
///[`RenderPassBeginInfo::render_area`], and provide a render area
///specific to each physical device.
///These render areas serve the same purpose as
///[`RenderPassBeginInfo::render_area`], including controlling the
///region of attachments that are cleared by `VK_ATTACHMENT_LOAD_OP_CLEAR`
///and that are resolved into resolve attachments.If this structure is not present, the render pass
/// instance’s device mask is
///the value of [`DeviceGroupCommandBufferBeginInfo`]::[`device_mask`].
///If this structure is not present or if [`device_render_area_count`] is zero,
///[`RenderPassBeginInfo::render_area`] is used for all physical
///devices.
///## Valid Usage
/// - [`device_mask`] **must**  be a valid device mask value
/// - [`device_mask`] **must**  not be zero
/// - [`device_mask`] **must**  be a subset of the command buffer’s initial device mask
/// - [`device_render_area_count`] **must**  either be zero or equal to the number of physical
///   devices in the logical device
/// - The `offset.x` member of any element of [`device_render_areas`] **must**  be greater than or
///   equal to 0
/// - The `offset.y` member of any element of [`device_render_areas`] **must**  be greater than or
///   equal to 0
/// - The sum of the `offset.x` and `extent.width` members of any element of [`device_render_areas`]
///   **must**  be less than or equal to [`maxFramebufferWidth`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFramebufferWidth)
/// -    The sum of the `offset.y` and `extent.height` members of any element of [`device_render_areas`] **must**  be less than or equal to [`maxFramebufferHeight`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFramebufferHeight)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO`
/// - If [`device_render_area_count`] is not `0`, [`device_render_areas`] **must**  be a valid
///   pointer to an array of [`device_render_area_count`][`Rect2D`] structures
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Rect2D`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupRenderPassBeginInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`device_mask`] is the device mask for the render pass instance.
    pub device_mask: u32,
    ///[`device_render_area_count`] is the number of elements in the
    ///[`device_render_areas`] array.
    pub device_render_area_count: u32,
    ///[`device_render_areas`] is a pointer to an array of [`Rect2D`]
    ///structures defining the render area for each physical device.
    pub device_render_areas: *const Rect2D,
}
impl<'lt> Default for DeviceGroupRenderPassBeginInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DeviceGroupRenderPassBeginInfo,
            p_next: std::ptr::null(),
            device_mask: 0,
            device_render_area_count: 0,
            device_render_areas: std::ptr::null(),
        }
    }
}
impl<'lt> DeviceGroupRenderPassBeginInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::device_render_areas`]
    pub fn device_render_areas_raw(&self) -> *const Rect2D {
        self.device_render_areas
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_render_areas`]
    pub fn set_device_render_areas_raw(&mut self, value: *const Rect2D) -> &mut Self {
        self.device_render_areas = value;
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
    ///Gets the value of [`Self::device_mask`]
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Gets the value of [`Self::device_render_area_count`]
    pub fn device_render_area_count(&self) -> u32 {
        self.device_render_area_count
    }
    ///Gets the value of [`Self::device_render_areas`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn device_render_areas(&self) -> &[Rect2D] {
        std::slice::from_raw_parts(self.device_render_areas, self.device_render_area_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::device_mask`]
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
    }
    ///Gets a mutable reference to the value of [`Self::device_render_area_count`]
    pub fn device_render_area_count_mut(&mut self) -> &mut u32 {
        &mut self.device_render_area_count
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
    ///Sets the raw value of [`Self::device_mask`]
    pub fn set_device_mask(&mut self, value: u32) -> &mut Self {
        self.device_mask = value;
        self
    }
    ///Sets the raw value of [`Self::device_render_area_count`]
    pub fn set_device_render_area_count(&mut self, value: u32) -> &mut Self {
        self.device_render_area_count = value;
        self
    }
    ///Sets the raw value of [`Self::device_render_areas`]
    pub fn set_device_render_areas(&mut self, value: &'lt [crate::vulkan1_0::Rect2D]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.device_render_areas = value.as_ptr();
        self.device_render_area_count = len_;
        self
    }
}
///[VkDeviceGroupCommandBufferBeginInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupCommandBufferBeginInfo.html) - Set the initial device mask for a command buffer
///# C Specifications
///If the [`p_next`] chain of [`CommandBufferBeginInfo`] includes a
///[`DeviceGroupCommandBufferBeginInfo`] structure, then that structure
///includes an initial device mask for the command buffer.The [`DeviceGroupCommandBufferBeginInfo`]
/// structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkDeviceGroupCommandBufferBeginInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           deviceMask;
///} VkDeviceGroupCommandBufferBeginInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group
///typedef VkDeviceGroupCommandBufferBeginInfo VkDeviceGroupCommandBufferBeginInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_mask`] is the initial value of the command buffer’s device mask.
///# Description
///The initial device mask also acts as an upper bound on the set of devices
///that  **can**  ever be in the device mask in the command buffer.If this structure is not
/// present, the initial value of a command buffer’s
///device mask is set to include all physical devices in the logical device
///when the command buffer begins recording.
///## Valid Usage
/// - [`device_mask`] **must**  be a valid device mask value
/// - [`device_mask`] **must**  not be zero
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`device_mask`] is the initial value of the command buffer’s device
    ///mask.
    pub device_mask: u32,
}
impl<'lt> Default for DeviceGroupCommandBufferBeginInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DeviceGroupCommandBufferBeginInfo,
            p_next: std::ptr::null(),
            device_mask: 0,
        }
    }
}
impl<'lt> DeviceGroupCommandBufferBeginInfo<'lt> {
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
    ///Gets the value of [`Self::device_mask`]
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::device_mask`]
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
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
    ///Sets the raw value of [`Self::device_mask`]
    pub fn set_device_mask(&mut self, value: u32) -> &mut Self {
        self.device_mask = value;
        self
    }
}
///[VkDeviceGroupSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSubmitInfo.html) - Structure indicating which physical devices execute semaphore operations and command buffers
///# C Specifications
///If the [`p_next`] chain of [`SubmitInfo`] includes a
///[`DeviceGroupSubmitInfo`] structure, then that structure includes device
///indices and masks specifying which physical devices execute semaphore
///operations and command buffers.The [`DeviceGroupSubmitInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkDeviceGroupSubmitInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           waitSemaphoreCount;
///    const uint32_t*    pWaitSemaphoreDeviceIndices;
///    uint32_t           commandBufferCount;
///    const uint32_t*    pCommandBufferDeviceMasks;
///    uint32_t           signalSemaphoreCount;
///    const uint32_t*    pSignalSemaphoreDeviceIndices;
///} VkDeviceGroupSubmitInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group
///typedef VkDeviceGroupSubmitInfo VkDeviceGroupSubmitInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`wait_semaphore_count`] is the number of elements in the [`wait_semaphore_device_indices`]
///   array.
/// - [`wait_semaphore_device_indices`] is a pointer to an array of [`wait_semaphore_count`] device
///   indices indicating which physical device executes the semaphore wait operation in the
///   corresponding element of [`SubmitInfo::wait_semaphores`].
/// - [`command_buffer_count`] is the number of elements in the [`command_buffer_device_masks`]
///   array.
/// - [`command_buffer_device_masks`] is a pointer to an array of [`command_buffer_count`] device
///   masks indicating which physical devices execute the command buffer in the corresponding
///   element of [`SubmitInfo::command_buffers`]. A physical device executes the command buffer if
///   the corresponding bit is set in the mask.
/// - [`signal_semaphore_count`] is the number of elements in the
///   [`signal_semaphore_device_indices`] array.
/// - [`signal_semaphore_device_indices`] is a pointer to an array of [`signal_semaphore_count`]
///   device indices indicating which physical device executes the semaphore signal operation in the
///   corresponding element of [`SubmitInfo::signal_semaphores`].
///# Description
///If this structure is not present, semaphore operations and command buffers
///execute on device index zero.
///## Valid Usage
/// - [`wait_semaphore_count`] **must**  equal [`SubmitInfo`]::[`wait_semaphore_count`]
/// - [`command_buffer_count`] **must**  equal [`SubmitInfo`]::[`command_buffer_count`]
/// - [`signal_semaphore_count`] **must**  equal [`SubmitInfo`]::[`signal_semaphore_count`]
/// - All elements of [`wait_semaphore_device_indices`] and [`signal_semaphore_device_indices`]
///   **must**  be valid device indices
/// - All elements of [`command_buffer_device_masks`] **must**  be valid device masks
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO`
/// - If [`wait_semaphore_count`] is not `0`, [`wait_semaphore_device_indices`] **must**  be a valid
///   pointer to an array of [`wait_semaphore_count`]`uint32_t` values
/// - If [`command_buffer_count`] is not `0`, [`command_buffer_device_masks`] **must**  be a valid
///   pointer to an array of [`command_buffer_count`]`uint32_t` values
/// - If [`signal_semaphore_count`] is not `0`, [`signal_semaphore_device_indices`] **must**  be a
///   valid pointer to an array of [`signal_semaphore_count`]`uint32_t` values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupSubmitInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceGroupSubmitInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`wait_semaphore_count`] is the number of elements in the
    ///[`wait_semaphore_device_indices`] array.
    pub wait_semaphore_count: u32,
    ///[`wait_semaphore_device_indices`] is a pointer to an array of
    ///[`wait_semaphore_count`] device indices indicating which physical device
    ///executes the semaphore wait operation in the corresponding element of
    ///[`SubmitInfo`]::`pWaitSemaphores`.
    pub wait_semaphore_device_indices: *const u32,
    ///[`command_buffer_count`] is the number of elements in the
    ///[`command_buffer_device_masks`] array.
    pub command_buffer_count: u32,
    ///[`command_buffer_device_masks`] is a pointer to an array of
    ///[`command_buffer_count`] device masks indicating which physical devices
    ///execute the command buffer in the corresponding element of
    ///[`SubmitInfo`]::`pCommandBuffers`.
    ///A physical device executes the command buffer if the corresponding bit
    ///is set in the mask.
    pub command_buffer_device_masks: *const u32,
    ///[`signal_semaphore_count`] is the number of elements in the
    ///[`signal_semaphore_device_indices`] array.
    pub signal_semaphore_count: u32,
    ///[`signal_semaphore_device_indices`] is a pointer to an array of
    ///[`signal_semaphore_count`] device indices indicating which physical
    ///device executes the semaphore signal operation in the corresponding
    ///element of [`SubmitInfo`]::`pSignalSemaphores`.
    pub signal_semaphore_device_indices: *const u32,
}
impl<'lt> Default for DeviceGroupSubmitInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DeviceGroupSubmitInfo,
            p_next: std::ptr::null(),
            wait_semaphore_count: 0,
            wait_semaphore_device_indices: std::ptr::null(),
            command_buffer_count: 0,
            command_buffer_device_masks: std::ptr::null(),
            signal_semaphore_count: 0,
            signal_semaphore_device_indices: std::ptr::null(),
        }
    }
}
impl<'lt> DeviceGroupSubmitInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::wait_semaphore_device_indices`]
    pub fn wait_semaphore_device_indices_raw(&self) -> *const u32 {
        self.wait_semaphore_device_indices
    }
    ///Gets the raw value of [`Self::command_buffer_device_masks`]
    pub fn command_buffer_device_masks_raw(&self) -> *const u32 {
        self.command_buffer_device_masks
    }
    ///Gets the raw value of [`Self::signal_semaphore_device_indices`]
    pub fn signal_semaphore_device_indices_raw(&self) -> *const u32 {
        self.signal_semaphore_device_indices
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_device_indices`]
    pub fn set_wait_semaphore_device_indices_raw(&mut self, value: *const u32) -> &mut Self {
        self.wait_semaphore_device_indices = value;
        self
    }
    ///Sets the raw value of [`Self::command_buffer_device_masks`]
    pub fn set_command_buffer_device_masks_raw(&mut self, value: *const u32) -> &mut Self {
        self.command_buffer_device_masks = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_device_indices`]
    pub fn set_signal_semaphore_device_indices_raw(&mut self, value: *const u32) -> &mut Self {
        self.signal_semaphore_device_indices = value;
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
    ///Gets the value of [`Self::wait_semaphore_count`]
    pub fn wait_semaphore_count(&self) -> u32 {
        self.wait_semaphore_count
    }
    ///Gets the value of [`Self::wait_semaphore_device_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn wait_semaphore_device_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.wait_semaphore_device_indices, self.wait_semaphore_count as usize)
    }
    ///Gets the value of [`Self::command_buffer_count`]
    pub fn command_buffer_count(&self) -> u32 {
        self.command_buffer_count
    }
    ///Gets the value of [`Self::command_buffer_device_masks`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn command_buffer_device_masks(&self) -> &[u32] {
        std::slice::from_raw_parts(self.command_buffer_device_masks, self.command_buffer_count as usize)
    }
    ///Gets the value of [`Self::signal_semaphore_count`]
    pub fn signal_semaphore_count(&self) -> u32 {
        self.signal_semaphore_count
    }
    ///Gets the value of [`Self::signal_semaphore_device_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn signal_semaphore_device_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(
            self.signal_semaphore_device_indices,
            self.signal_semaphore_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::wait_semaphore_count`]
    pub fn wait_semaphore_count_mut(&mut self) -> &mut u32 {
        &mut self.wait_semaphore_count
    }
    ///Gets a mutable reference to the value of [`Self::command_buffer_count`]
    pub fn command_buffer_count_mut(&mut self) -> &mut u32 {
        &mut self.command_buffer_count
    }
    ///Gets a mutable reference to the value of [`Self::signal_semaphore_count`]
    pub fn signal_semaphore_count_mut(&mut self) -> &mut u32 {
        &mut self.signal_semaphore_count
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
    ///Sets the raw value of [`Self::wait_semaphore_count`]
    pub fn set_wait_semaphore_count(&mut self, value: u32) -> &mut Self {
        self.wait_semaphore_count = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_device_indices`]
    pub fn set_wait_semaphore_device_indices(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.wait_semaphore_device_indices = value.as_ptr();
        self.wait_semaphore_count = len_;
        self
    }
    ///Sets the raw value of [`Self::command_buffer_count`]
    pub fn set_command_buffer_count(&mut self, value: u32) -> &mut Self {
        self.command_buffer_count = value;
        self
    }
    ///Sets the raw value of [`Self::command_buffer_device_masks`]
    pub fn set_command_buffer_device_masks(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.command_buffer_device_masks = value.as_ptr();
        self.command_buffer_count = len_;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_count`]
    pub fn set_signal_semaphore_count(&mut self, value: u32) -> &mut Self {
        self.signal_semaphore_count = value;
        self
    }
    ///Sets the raw value of [`Self::signal_semaphore_device_indices`]
    pub fn set_signal_semaphore_device_indices(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.signal_semaphore_device_indices = value.as_ptr();
        self.signal_semaphore_count = len_;
        self
    }
}
///[VkDeviceGroupBindSparseInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupBindSparseInfo.html) - Structure indicating which instances are bound
///# C Specifications
///If the [`p_next`] chain of [`BindSparseInfo`] includes a
///[`DeviceGroupBindSparseInfo`] structure, then that structure includes
///device indices specifying which instance of the resources and memory are
///bound.The [`DeviceGroupBindSparseInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkDeviceGroupBindSparseInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    uint32_t           resourceDeviceIndex;
///    uint32_t           memoryDeviceIndex;
///} VkDeviceGroupBindSparseInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group
///typedef VkDeviceGroupBindSparseInfo VkDeviceGroupBindSparseInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`resource_device_index`] is a device index indicating which instance of the resource is
///   bound.
/// - [`memory_device_index`] is a device index indicating which instance of the memory the resource
///   instance is bound to.
///# Description
///These device indices apply to all buffer and image memory binds included in
///the batch pointing to this structure.
///The semaphore waits and signals for the batch are executed only by the
///physical device specified by the [`resource_device_index`].If this structure is not present,
/// [`resource_device_index`] and
///[`memory_device_index`] are assumed to be zero.
///## Valid Usage
/// - [`resource_device_index`] and [`memory_device_index`] **must**  both be valid device indices
/// - Each memory allocation bound in this batch  **must**  have allocated an instance for
///   [`memory_device_index`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupBindSparseInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceGroupBindSparseInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`resource_device_index`] is a device index indicating which instance of
    ///the resource is bound.
    pub resource_device_index: u32,
    ///[`memory_device_index`] is a device index indicating which instance of
    ///the memory the resource instance is bound to.
    pub memory_device_index: u32,
}
impl<'lt> Default for DeviceGroupBindSparseInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DeviceGroupBindSparseInfo,
            p_next: std::ptr::null(),
            resource_device_index: 0,
            memory_device_index: 0,
        }
    }
}
impl<'lt> DeviceGroupBindSparseInfo<'lt> {
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
    ///Gets the value of [`Self::resource_device_index`]
    pub fn resource_device_index(&self) -> u32 {
        self.resource_device_index
    }
    ///Gets the value of [`Self::memory_device_index`]
    pub fn memory_device_index(&self) -> u32 {
        self.memory_device_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::resource_device_index`]
    pub fn resource_device_index_mut(&mut self) -> &mut u32 {
        &mut self.resource_device_index
    }
    ///Gets a mutable reference to the value of [`Self::memory_device_index`]
    pub fn memory_device_index_mut(&mut self) -> &mut u32 {
        &mut self.memory_device_index
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
    ///Sets the raw value of [`Self::resource_device_index`]
    pub fn set_resource_device_index(&mut self, value: u32) -> &mut Self {
        self.resource_device_index = value;
        self
    }
    ///Sets the raw value of [`Self::memory_device_index`]
    pub fn set_memory_device_index(&mut self, value: u32) -> &mut Self {
        self.memory_device_index = value;
        self
    }
}
///[VkDeviceGroupDeviceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html) - Create a logical device from multiple physical devices
///# C Specifications
///A logical device  **can**  be created that connects to one or more physical
///devices by adding a [`DeviceGroupDeviceCreateInfo`] structure to the
///[`p_next`] chain of [`DeviceCreateInfo`].
///The [`DeviceGroupDeviceCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkDeviceGroupDeviceCreateInfo {
///    VkStructureType            sType;
///    const void*                pNext;
///    uint32_t                   physicalDeviceCount;
///    const VkPhysicalDevice*    pPhysicalDevices;
///} VkDeviceGroupDeviceCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_device_group_creation
///typedef VkDeviceGroupDeviceCreateInfo VkDeviceGroupDeviceCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`physical_device_count`] is the number of elements in the [`physical_devices`] array.
/// - [`physical_devices`] is a pointer to an array of physical device handles belonging to the same
///   device group.
///# Description
///The elements of the [`physical_devices`] array are an ordered list of the
///physical devices that the logical device represents.
///These  **must**  be a subset of a single device group, and need not be in the
///same order as they were enumerated.
///The order of the physical devices in the [`physical_devices`] array
///determines the *device index* of each physical device, with element i
///being assigned a device index of i.
///Certain commands and structures refer to one or more physical devices by
///using device indices or *device masks* formed using device indices.A logical device created
/// without using [`DeviceGroupDeviceCreateInfo`],
///or with [`physical_device_count`] equal to zero, is equivalent to a
///[`physical_device_count`] of one and [`physical_devices`] pointing to the
///`physicalDevice` parameter to [`CreateDevice`].
///In particular, the device index of that physical device is zero.
///## Valid Usage
/// - Each element of [`physical_devices`] **must**  be unique
/// - All elements of [`physical_devices`] **must**  be in the same device group as enumerated by
///   [`EnumeratePhysicalDeviceGroups`]
/// - If [`physical_device_count`] is not `0`, the `physicalDevice` parameter of [`CreateDevice`]
///   **must**  be an element of [`physical_devices`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO`
/// - If [`physical_device_count`] is not `0`, [`physical_devices`] **must**  be a valid pointer to
///   an array of [`physical_device_count`] valid [`PhysicalDevice`] handles
///# Related
/// - [`crate::vulkan1_1`]
/// - [`PhysicalDevice`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupDeviceCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceGroupDeviceCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`physical_device_count`] is the number of elements in the
    ///[`physical_devices`] array.
    pub physical_device_count: u32,
    ///[`physical_devices`] is a pointer to an array of physical device
    ///handles belonging to the same device group.
    pub physical_devices: *const PhysicalDevice,
}
impl<'lt> Default for DeviceGroupDeviceCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DeviceGroupDeviceCreateInfo,
            p_next: std::ptr::null(),
            physical_device_count: 0,
            physical_devices: std::ptr::null(),
        }
    }
}
impl<'lt> DeviceGroupDeviceCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::physical_devices`]
    pub fn physical_devices_raw(&self) -> *const PhysicalDevice {
        self.physical_devices
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::physical_devices`]
    pub fn set_physical_devices_raw(&mut self, value: *const PhysicalDevice) -> &mut Self {
        self.physical_devices = value;
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
    ///Gets the value of [`Self::physical_device_count`]
    pub fn physical_device_count(&self) -> u32 {
        self.physical_device_count
    }
    ///Gets the value of [`Self::physical_devices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn physical_devices(&self) -> &[PhysicalDevice] {
        std::slice::from_raw_parts(self.physical_devices, self.physical_device_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::physical_device_count`]
    pub fn physical_device_count_mut(&mut self) -> &mut u32 {
        &mut self.physical_device_count
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
    ///Sets the raw value of [`Self::physical_device_count`]
    pub fn set_physical_device_count(&mut self, value: u32) -> &mut Self {
        self.physical_device_count = value;
        self
    }
    ///Sets the raw value of [`Self::physical_devices`]
    pub fn set_physical_devices(&mut self, value: &'lt [crate::vulkan1_0::PhysicalDevice]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.physical_devices = value.as_ptr();
        self.physical_device_count = len_;
        self
    }
}
///[VkDescriptorUpdateTemplateEntry](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateEntry.html) - Describes a single descriptor update of the descriptor update template
///# C Specifications
///The [`DescriptorUpdateTemplateEntry`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkDescriptorUpdateTemplateEntry {
///    uint32_t            dstBinding;
///    uint32_t            dstArrayElement;
///    uint32_t            descriptorCount;
///    VkDescriptorType    descriptorType;
///    size_t              offset;
///    size_t              stride;
///} VkDescriptorUpdateTemplateEntry;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_descriptor_update_template
///typedef VkDescriptorUpdateTemplateEntry VkDescriptorUpdateTemplateEntryKHR;
///```
///# Members
/// - [`dst_binding`] is the descriptor binding to update when using this descriptor update
///   template.
/// - [`dst_array_element`] is the starting element in the array belonging to [`dst_binding`]. If
///   the descriptor binding identified by [`dst_binding`] has a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`dst_array_element`] specifies the starting
///   byte offset to update.
/// - [`descriptor_count`] is the number of descriptors to update. If [`descriptor_count`] is
///   greater than the number of remaining array elements in the destination binding, those affect
///   consecutive bindings in a manner similar to [`WriteDescriptorSet`] above. If the descriptor
///   binding identified by [`dst_binding`] has a descriptor type of
///   `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`descriptor_count`] specifies the number of
///   bytes to update and the remaining array elements in the destination binding refer to the
///   remaining number of bytes in it.
/// - [`descriptor_type`] is a [`DescriptorType`] specifying the type of the descriptor.
/// - [`offset`] is the offset in bytes of the first binding in the raw data structure.
/// - [`stride`] is the stride in bytes between two consecutive array elements of the descriptor
///   update informations in the raw data structure. The actual pointer ptr for each array element j
///   of update entry i is computed using the following formula: ```c     const char *ptr = (const
///   char *)pData + pDescriptorUpdateEntries[i].offset + j * pDescriptorUpdateEntries[i].stride ```
///   The stride is useful in case the bindings are stored in structs along with other data. If
///   [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then the value of [`stride`]
///   is ignored and the stride is assumed to be `1`, i.e. the descriptor update information for
///   them is always specified as a contiguous range.
///# Description
///## Valid Usage
/// - [`dst_binding`] **must**  be a valid binding in the descriptor set layout implicitly specified
///   when using a descriptor update template to update descriptors
/// -  [`dst_array_element`] and [`descriptor_count`] **must**  be less than or equal to the number of array elements in the descriptor set binding implicitly specified when using a descriptor update template to update descriptors, and all applicable consecutive bindings, as described by [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive)
/// - If `descriptor` type is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, [`dst_array_element`]
///   **must**  be an integer multiple of `4`
/// - If `descriptor` type is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, [`descriptor_count`]
///   **must**  be an integer multiple of `4`
///
///## Valid Usage (Implicit)
/// - [`descriptor_type`] **must**  be a valid [`DescriptorType`] value
///# Related
/// - [`crate::vulkan1_1`]
/// - [`DescriptorType`]
/// - [`DescriptorUpdateTemplateCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDescriptorUpdateTemplateEntry")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DescriptorUpdateTemplateEntry {
    ///[`dst_binding`] is the descriptor binding to update when using this
    ///descriptor update template.
    pub dst_binding: u32,
    ///[`dst_array_element`] is the starting element in the array belonging to
    ///[`dst_binding`].
    ///If the descriptor binding identified by [`dst_binding`] has a
    ///descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
    ///[`dst_array_element`] specifies the starting byte offset to update.
    pub dst_array_element: u32,
    ///[`descriptor_count`] is the number of descriptors to update.
    ///If [`descriptor_count`] is greater than the number of remaining array
    ///elements in the destination binding, those affect consecutive bindings
    ///in a manner similar to [`WriteDescriptorSet`] above.
    ///If the descriptor binding identified by [`dst_binding`] has a
    ///descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
    ///[`descriptor_count`] specifies the number of bytes to update and the
    ///remaining array elements in the destination binding refer to the
    ///remaining number of bytes in it.
    pub descriptor_count: u32,
    ///[`descriptor_type`] is a [`DescriptorType`] specifying the type of
    ///the descriptor.
    pub descriptor_type: DescriptorType,
    ///[`offset`] is the offset in bytes of the first binding in the raw data
    ///structure.
    pub offset: usize,
    ///[`stride`] is the stride in bytes between two consecutive array
    ///elements of the descriptor update informations in the raw data
    ///structure.
    ///The actual pointer ptr for each array element j of update entry i is
    ///computed using the following formula:
    ///```c
    ///    const char *ptr = (const char *)pData + pDescriptorUpdateEntries[i].offset + j * pDescriptorUpdateEntries[i].stride
    /// ```
    ///The stride is useful in case the bindings are stored in structs along with
    ///other data.
    ///If [`descriptor_type`] is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`
    ///then the value of [`stride`] is ignored and the stride is assumed to be
    ///`1`, i.e. the descriptor update information for them is always specified as
    ///a contiguous range.
    pub stride: usize,
}
impl Default for DescriptorUpdateTemplateEntry {
    fn default() -> Self {
        Self {
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 0,
            descriptor_type: Default::default(),
            offset: 0,
            stride: 0,
        }
    }
}
impl DescriptorUpdateTemplateEntry {
    ///Gets the value of [`Self::dst_binding`]
    pub fn dst_binding(&self) -> u32 {
        self.dst_binding
    }
    ///Gets the value of [`Self::dst_array_element`]
    pub fn dst_array_element(&self) -> u32 {
        self.dst_array_element
    }
    ///Gets the value of [`Self::descriptor_count`]
    pub fn descriptor_count(&self) -> u32 {
        self.descriptor_count
    }
    ///Gets the value of [`Self::descriptor_type`]
    pub fn descriptor_type(&self) -> DescriptorType {
        self.descriptor_type
    }
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> usize {
        self.offset
    }
    ///Gets the value of [`Self::stride`]
    pub fn stride(&self) -> usize {
        self.stride
    }
    ///Gets a mutable reference to the value of [`Self::dst_binding`]
    pub fn dst_binding_mut(&mut self) -> &mut u32 {
        &mut self.dst_binding
    }
    ///Gets a mutable reference to the value of [`Self::dst_array_element`]
    pub fn dst_array_element_mut(&mut self) -> &mut u32 {
        &mut self.dst_array_element
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_count`]
    pub fn descriptor_count_mut(&mut self) -> &mut u32 {
        &mut self.descriptor_count
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_type`]
    pub fn descriptor_type_mut(&mut self) -> &mut DescriptorType {
        &mut self.descriptor_type
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }
    ///Gets a mutable reference to the value of [`Self::stride`]
    pub fn stride_mut(&mut self) -> &mut usize {
        &mut self.stride
    }
    ///Sets the raw value of [`Self::dst_binding`]
    pub fn set_dst_binding(&mut self, value: u32) -> &mut Self {
        self.dst_binding = value;
        self
    }
    ///Sets the raw value of [`Self::dst_array_element`]
    pub fn set_dst_array_element(&mut self, value: u32) -> &mut Self {
        self.dst_array_element = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_count`]
    pub fn set_descriptor_count(&mut self, value: u32) -> &mut Self {
        self.descriptor_count = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_type`]
    pub fn set_descriptor_type(&mut self, value: crate::vulkan1_0::DescriptorType) -> &mut Self {
        self.descriptor_type = value;
        self
    }
    ///Sets the raw value of [`Self::offset`]
    pub fn set_offset(&mut self, value: usize) -> &mut Self {
        self.offset = value;
        self
    }
    ///Sets the raw value of [`Self::stride`]
    pub fn set_stride(&mut self, value: usize) -> &mut Self {
        self.stride = value;
        self
    }
}
///[VkDescriptorUpdateTemplateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateCreateInfo.html) - Structure specifying parameters of a newly created descriptor update template
///# C Specifications
///The [`DescriptorUpdateTemplateCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkDescriptorUpdateTemplateCreateInfo {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    VkDescriptorUpdateTemplateCreateFlags     flags;
///    uint32_t                                  descriptorUpdateEntryCount;
///    const VkDescriptorUpdateTemplateEntry*    pDescriptorUpdateEntries;
///    VkDescriptorUpdateTemplateType            templateType;
///    VkDescriptorSetLayout                     descriptorSetLayout;
///    VkPipelineBindPoint                       pipelineBindPoint;
///    VkPipelineLayout                          pipelineLayout;
///    uint32_t                                  set;
///} VkDescriptorUpdateTemplateCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_descriptor_update_template
///typedef VkDescriptorUpdateTemplateCreateInfo VkDescriptorUpdateTemplateCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`descriptor_update_entry_count`] is the number of elements in the
///   [`descriptor_update_entries`] array.
/// - [`descriptor_update_entries`] is a pointer to an array of [`DescriptorUpdateTemplateEntry`]
///   structures describing the descriptors to be updated by the descriptor update template.
/// - [`template_type`] Specifies the type of the descriptor update template. If set to
///   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET` it  **can**  only be used to update
///   descriptor sets with a fixed [`descriptor_set_layout`]. If set to
///   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR` it  **can**  only be used to push
///   descriptor sets using the provided [`pipeline_bind_point`], [`pipeline_layout`], and [`set`]
///   number.
/// - [`descriptor_set_layout`] is the descriptor set layout used to build the descriptor update
///   template. All descriptor sets which are going to be updated through the newly created
///   descriptor update template  **must**  be created with a layout that matches (is the same as,
///   or defined identically to) this layout. This parameter is ignored if [`template_type`] is not
///   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`.
/// - [`pipeline_bind_point`] is a [`PipelineBindPoint`] indicating the type of the pipeline that
///   will use the descriptors. This parameter is ignored if [`template_type`] is not
///   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
/// - [`pipeline_layout`] is a [`PipelineLayout`] object used to program the bindings. This
///   parameter is ignored if [`template_type`] is not
///   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
/// - [`set`] is the set number of the descriptor set in the pipeline layout that will be updated.
///   This parameter is ignored if [`template_type`] is not
///   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
///# Description
///## Valid Usage
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`,
///   [`descriptor_set_layout`] **must**  be a valid [`DescriptorSetLayout`] handle
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`,
///   [`pipeline_bind_point`] **must**  be a valid [`PipelineBindPoint`] value
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`,
///   [`pipeline_layout`] **must**  be a valid [`PipelineLayout`] handle
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`, [`set`]
///   **must**  be the unique set number in the pipeline layout that uses a descriptor set layout
///   that was created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`,
///   [`descriptor_set_layout`] **must**  not contain a binding with type
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be `0`
/// - [`descriptor_update_entries`] **must**  be a valid pointer to an array of
///   [`descriptor_update_entry_count`] valid [`DescriptorUpdateTemplateEntry`] structures
/// - [`template_type`] **must**  be a valid [`DescriptorUpdateTemplateType`] value
/// - [`descriptor_update_entry_count`] **must**  be greater than `0`
/// - Both of [`descriptor_set_layout`], and [`pipeline_layout`] that are valid handles of
///   non-ignored parameters  **must**  have been created, allocated, or retrieved from the same
///   [`Device`]
///# Related
/// - [`crate::vulkan1_1`]
/// - [`DescriptorSetLayout`]
/// - [`DescriptorUpdateTemplateCreateFlags`]
/// - [`DescriptorUpdateTemplateEntry`]
/// - [`DescriptorUpdateTemplateType`]
/// - [`PipelineBindPoint`]
/// - [`PipelineLayout`]
/// - [`StructureType`]
/// - [`CreateDescriptorUpdateTemplate`]
/// - [`CreateDescriptorUpdateTemplateKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: DescriptorUpdateTemplateCreateFlags,
    ///[`descriptor_update_entry_count`] is the number of elements in the
    ///[`descriptor_update_entries`] array.
    pub descriptor_update_entry_count: u32,
    ///[`descriptor_update_entries`] is a pointer to an array of
    ///[`DescriptorUpdateTemplateEntry`] structures describing the
    ///descriptors to be updated by the descriptor update template.
    pub descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    ///[`template_type`] Specifies the type of the descriptor update template.
    ///If set to `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET` it
    /// **can**  only be used to update descriptor sets with a fixed
    ///[`descriptor_set_layout`].
    ///If set to `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
    ///it  **can**  only be used to push descriptor sets using the provided
    ///[`pipeline_bind_point`], [`pipeline_layout`], and [`set`] number.
    pub template_type: DescriptorUpdateTemplateType,
    ///[`descriptor_set_layout`] is the descriptor set layout used to build the
    ///descriptor update template.
    ///All descriptor sets which are going to be updated through the newly
    ///created descriptor update template  **must**  be created with a layout that
    ///matches (is the same as, or defined identically to) this layout.
    ///This parameter is ignored if [`template_type`] is not
    ///`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`.
    pub descriptor_set_layout: DescriptorSetLayout,
    ///[`pipeline_bind_point`] is a [`PipelineBindPoint`] indicating the
    ///type of the pipeline that will use the descriptors.
    ///This parameter is ignored if [`template_type`] is not
    ///`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
    pub pipeline_bind_point: PipelineBindPoint,
    ///[`pipeline_layout`] is a [`PipelineLayout`] object used to program
    ///the bindings.
    ///This parameter is ignored if [`template_type`] is not
    ///`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
    pub pipeline_layout: PipelineLayout,
    ///[`set`] is the set number of the descriptor set in the pipeline layout
    ///that will be updated.
    ///This parameter is ignored if [`template_type`] is not
    ///`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
    pub set: u32,
}
impl<'lt> Default for DescriptorUpdateTemplateCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DescriptorUpdateTemplateCreateInfo,
            p_next: std::ptr::null(),
            flags: Default::default(),
            descriptor_update_entry_count: 0,
            descriptor_update_entries: std::ptr::null(),
            template_type: Default::default(),
            descriptor_set_layout: Default::default(),
            pipeline_bind_point: Default::default(),
            pipeline_layout: Default::default(),
            set: 0,
        }
    }
}
impl<'lt> DescriptorUpdateTemplateCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::descriptor_update_entries`]
    pub fn descriptor_update_entries_raw(&self) -> *const DescriptorUpdateTemplateEntry {
        self.descriptor_update_entries
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_update_entries`]
    pub fn set_descriptor_update_entries_raw(&mut self, value: *const DescriptorUpdateTemplateEntry) -> &mut Self {
        self.descriptor_update_entries = value;
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
    pub fn flags(&self) -> DescriptorUpdateTemplateCreateFlags {
        self.flags
    }
    ///Gets the value of [`Self::descriptor_update_entry_count`]
    pub fn descriptor_update_entry_count(&self) -> u32 {
        self.descriptor_update_entry_count
    }
    ///Gets the value of [`Self::descriptor_update_entries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn descriptor_update_entries(&self) -> &[DescriptorUpdateTemplateEntry] {
        std::slice::from_raw_parts(
            self.descriptor_update_entries,
            self.descriptor_update_entry_count as usize,
        )
    }
    ///Gets the value of [`Self::template_type`]
    pub fn template_type(&self) -> DescriptorUpdateTemplateType {
        self.template_type
    }
    ///Gets the value of [`Self::descriptor_set_layout`]
    pub fn descriptor_set_layout(&self) -> DescriptorSetLayout {
        self.descriptor_set_layout
    }
    ///Gets the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point(&self) -> PipelineBindPoint {
        self.pipeline_bind_point
    }
    ///Gets the value of [`Self::pipeline_layout`]
    pub fn pipeline_layout(&self) -> PipelineLayout {
        self.pipeline_layout
    }
    ///Gets the value of [`Self::set`]
    pub fn set(&self) -> u32 {
        self.set
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DescriptorUpdateTemplateCreateFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_update_entry_count`]
    pub fn descriptor_update_entry_count_mut(&mut self) -> &mut u32 {
        &mut self.descriptor_update_entry_count
    }
    ///Gets a mutable reference to the value of [`Self::template_type`]
    pub fn template_type_mut(&mut self) -> &mut DescriptorUpdateTemplateType {
        &mut self.template_type
    }
    ///Gets a mutable reference to the value of [`Self::descriptor_set_layout`]
    pub fn descriptor_set_layout_mut(&mut self) -> &mut DescriptorSetLayout {
        &mut self.descriptor_set_layout
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_bind_point`]
    pub fn pipeline_bind_point_mut(&mut self) -> &mut PipelineBindPoint {
        &mut self.pipeline_bind_point
    }
    ///Gets a mutable reference to the value of [`Self::pipeline_layout`]
    pub fn pipeline_layout_mut(&mut self) -> &mut PipelineLayout {
        &mut self.pipeline_layout
    }
    ///Gets a mutable reference to the value of [`Self::set`]
    pub fn set_mut(&mut self) -> &mut u32 {
        &mut self.set
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
    pub fn set_flags(&mut self, value: crate::vulkan1_1::DescriptorUpdateTemplateCreateFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_update_entry_count`]
    pub fn set_descriptor_update_entry_count(&mut self, value: u32) -> &mut Self {
        self.descriptor_update_entry_count = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_update_entries`]
    pub fn set_descriptor_update_entries(
        &mut self,
        value: &'lt [crate::vulkan1_1::DescriptorUpdateTemplateEntry],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.descriptor_update_entries = value.as_ptr();
        self.descriptor_update_entry_count = len_;
        self
    }
    ///Sets the raw value of [`Self::template_type`]
    pub fn set_template_type(&mut self, value: crate::vulkan1_1::DescriptorUpdateTemplateType) -> &mut Self {
        self.template_type = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_set_layout`]
    pub fn set_descriptor_set_layout(&mut self, value: crate::vulkan1_0::DescriptorSetLayout) -> &mut Self {
        self.descriptor_set_layout = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_bind_point`]
    pub fn set_pipeline_bind_point(&mut self, value: crate::vulkan1_0::PipelineBindPoint) -> &mut Self {
        self.pipeline_bind_point = value;
        self
    }
    ///Sets the raw value of [`Self::pipeline_layout`]
    pub fn set_pipeline_layout(&mut self, value: crate::vulkan1_0::PipelineLayout) -> &mut Self {
        self.pipeline_layout = value;
        self
    }
    ///Sets the raw value of [`Self::set`]
    pub fn set_set(&mut self, value: u32) -> &mut Self {
        self.set = value;
        self
    }
}
///[VkInputAttachmentAspectReference](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInputAttachmentAspectReference.html) - Structure specifying a subpass/input attachment pair and an aspect mask that can: be read.
///# C Specifications
///The [`InputAttachmentAspectReference`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkInputAttachmentAspectReference {
///    uint32_t              subpass;
///    uint32_t              inputAttachmentIndex;
///    VkImageAspectFlags    aspectMask;
///} VkInputAttachmentAspectReference;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance2
///typedef VkInputAttachmentAspectReference VkInputAttachmentAspectReferenceKHR;
///```
///# Members
/// - [`subpass`] is an index into the `pSubpasses` array of the parent [`RenderPassCreateInfo`]
///   structure.
/// - [`input_attachment_index`] is an index into the `pInputAttachments` of the specified subpass.
/// - [`aspect_mask`] is a mask of which aspect(s)  **can**  be accessed within the specified
///   subpass.
///# Description
///This structure specifies an aspect mask for a specific input attachment of a
///specific subpass in the render pass.[`subpass`] and [`input_attachment_index`] index into the
/// render pass as:
///```c
///pCreateInfo->pSubpasses[subpass].pInputAttachments[inputAttachmentIndex]
///```
///
///## Valid Usage
/// - [`aspect_mask`] **must**  not include `VK_IMAGE_ASPECT_METADATA_BIT`
/// - [`aspect_mask`] **must**  not include `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` for any index
///   *i*
///
///## Valid Usage (Implicit)
/// - [`aspect_mask`] **must**  be a valid combination of [`ImageAspectFlagBits`] values
/// - [`aspect_mask`] **must**  not be `0`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ImageAspectFlags`]
/// - [`RenderPassInputAttachmentAspectCreateInfo`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkInputAttachmentAspectReference")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct InputAttachmentAspectReference {
    ///[`subpass`] is an index into the `pSubpasses` array of the parent
    ///[`RenderPassCreateInfo`] structure.
    pub subpass: u32,
    ///[`input_attachment_index`] is an index into the `pInputAttachments`
    ///of the specified subpass.
    pub input_attachment_index: u32,
    ///[`aspect_mask`] is a mask of which aspect(s)  **can**  be accessed within
    ///the specified subpass.
    pub aspect_mask: ImageAspectFlags,
}
impl Default for InputAttachmentAspectReference {
    fn default() -> Self {
        Self {
            subpass: 0,
            input_attachment_index: 0,
            aspect_mask: Default::default(),
        }
    }
}
impl InputAttachmentAspectReference {
    ///Gets the value of [`Self::subpass`]
    pub fn subpass(&self) -> u32 {
        self.subpass
    }
    ///Gets the value of [`Self::input_attachment_index`]
    pub fn input_attachment_index(&self) -> u32 {
        self.input_attachment_index
    }
    ///Gets the value of [`Self::aspect_mask`]
    pub fn aspect_mask(&self) -> ImageAspectFlags {
        self.aspect_mask
    }
    ///Gets a mutable reference to the value of [`Self::subpass`]
    pub fn subpass_mut(&mut self) -> &mut u32 {
        &mut self.subpass
    }
    ///Gets a mutable reference to the value of [`Self::input_attachment_index`]
    pub fn input_attachment_index_mut(&mut self) -> &mut u32 {
        &mut self.input_attachment_index
    }
    ///Gets a mutable reference to the value of [`Self::aspect_mask`]
    pub fn aspect_mask_mut(&mut self) -> &mut ImageAspectFlags {
        &mut self.aspect_mask
    }
    ///Sets the raw value of [`Self::subpass`]
    pub fn set_subpass(&mut self, value: u32) -> &mut Self {
        self.subpass = value;
        self
    }
    ///Sets the raw value of [`Self::input_attachment_index`]
    pub fn set_input_attachment_index(&mut self, value: u32) -> &mut Self {
        self.input_attachment_index = value;
        self
    }
    ///Sets the raw value of [`Self::aspect_mask`]
    pub fn set_aspect_mask(&mut self, value: crate::vulkan1_0::ImageAspectFlags) -> &mut Self {
        self.aspect_mask = value;
        self
    }
}
///[VkRenderPassInputAttachmentAspectCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassInputAttachmentAspectCreateInfo.html) - Structure specifying, for a given subpass/input attachment pair, which aspect can: be read.
///# C Specifications
///The [`RenderPassInputAttachmentAspectCreateInfo`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkRenderPassInputAttachmentAspectCreateInfo {
///    VkStructureType                            sType;
///    const void*                                pNext;
///    uint32_t                                   aspectReferenceCount;
///    const VkInputAttachmentAspectReference*    pAspectReferences;
///} VkRenderPassInputAttachmentAspectCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance2
///typedef VkRenderPassInputAttachmentAspectCreateInfo
/// VkRenderPassInputAttachmentAspectCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`aspect_reference_count`] is the number of elements in the [`aspect_references`] array.
/// - [`aspect_references`] is a pointer to an array of
///   [`aspect_reference_count`][`InputAttachmentAspectReference`] structures containing a mask
///   describing which aspect(s)  **can**  be accessed for a given input attachment within a given
///   subpass.
///# Description
///To specify which aspects of an input attachment  **can**  be read, add a
///[`RenderPassInputAttachmentAspectCreateInfo`] structure to the
///[`p_next`] chain of the [`RenderPassCreateInfo`] structure:An application  **can**  access any
/// aspect of an input attachment that does not
///have a specified aspect mask in the [`aspect_references`] array.
///Otherwise, an application  **must**  not access aspect(s) of an input attachment
///other than those in its specified aspect mask.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO`
/// - [`aspect_references`] **must**  be a valid pointer to an array of [`aspect_reference_count`]
///   valid [`InputAttachmentAspectReference`] structures
/// - [`aspect_reference_count`] **must**  be greater than `0`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`InputAttachmentAspectReference`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRenderPassInputAttachmentAspectCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`aspect_reference_count`] is the number of elements in the
    ///[`aspect_references`] array.
    pub aspect_reference_count: u32,
    ///[`aspect_references`] is a pointer to an array of
    ///[`aspect_reference_count`][`InputAttachmentAspectReference`]
    ///structures containing a mask describing which aspect(s)  **can**  be accessed
    ///for a given input attachment within a given subpass.
    pub aspect_references: *const InputAttachmentAspectReference,
}
impl<'lt> Default for RenderPassInputAttachmentAspectCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RenderPassInputAttachmentAspectCreateInfo,
            p_next: std::ptr::null(),
            aspect_reference_count: 0,
            aspect_references: std::ptr::null(),
        }
    }
}
impl<'lt> RenderPassInputAttachmentAspectCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::aspect_references`]
    pub fn aspect_references_raw(&self) -> *const InputAttachmentAspectReference {
        self.aspect_references
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::aspect_references`]
    pub fn set_aspect_references_raw(&mut self, value: *const InputAttachmentAspectReference) -> &mut Self {
        self.aspect_references = value;
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
    ///Gets the value of [`Self::aspect_reference_count`]
    pub fn aspect_reference_count(&self) -> u32 {
        self.aspect_reference_count
    }
    ///Gets the value of [`Self::aspect_references`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn aspect_references(&self) -> &[InputAttachmentAspectReference] {
        std::slice::from_raw_parts(self.aspect_references, self.aspect_reference_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::aspect_reference_count`]
    pub fn aspect_reference_count_mut(&mut self) -> &mut u32 {
        &mut self.aspect_reference_count
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
    ///Sets the raw value of [`Self::aspect_reference_count`]
    pub fn set_aspect_reference_count(&mut self, value: u32) -> &mut Self {
        self.aspect_reference_count = value;
        self
    }
    ///Sets the raw value of [`Self::aspect_references`]
    pub fn set_aspect_references(
        &mut self,
        value: &'lt [crate::vulkan1_1::InputAttachmentAspectReference],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.aspect_references = value.as_ptr();
        self.aspect_reference_count = len_;
        self
    }
}
///[VkPhysicalDevice16BitStorageFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html) - Structure describing features supported by VK_KHR_16bit_storage
///# C Specifications
///The [`PhysicalDevice16BitStorageFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDevice16BitStorageFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           storageBuffer16BitAccess;
///    VkBool32           uniformAndStorageBuffer16BitAccess;
///    VkBool32           storagePushConstant16;
///    VkBool32           storageInputOutput16;
///} VkPhysicalDevice16BitStorageFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_16bit_storage
///typedef VkPhysicalDevice16BitStorageFeatures VkPhysicalDevice16BitStorageFeaturesKHR;
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
///If the [`PhysicalDevice16BitStorageFeatures`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevice16BitStorageFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES`
///# Related
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkPhysicalDevice16BitStorageFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures<'lt> {
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
}
impl<'lt> Default for PhysicalDevice16BitStorageFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDevice16BitStorageFeatures,
            p_next: std::ptr::null_mut(),
            storage_buffer_16_bit_access: 0,
            uniform_and_storage_buffer_16_bit_access: 0,
            storage_push_constant_16: 0,
            storage_input_output_16: 0,
        }
    }
}
impl<'lt> PhysicalDevice16BitStorageFeatures<'lt> {
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
}
///[VkPhysicalDeviceSubgroupProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubgroupProperties.html) - Structure describing subgroup support for an implementation
///# C Specifications
///The [`PhysicalDeviceSubgroupProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceSubgroupProperties {
///    VkStructureType           sType;
///    void*                     pNext;
///    uint32_t                  subgroupSize;
///    VkShaderStageFlags        supportedStages;
///    VkSubgroupFeatureFlags    supportedOperations;
///    VkBool32                  quadOperationsInAllStages;
///} VkPhysicalDeviceSubgroupProperties;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`subgroup_size`] is the default number of invocations in each subgroup. [`subgroup_size`] is
///   at least 1 if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or
///   `VK_QUEUE_COMPUTE_BIT`. [`subgroup_size`] is a power-of-two.
/// - [`supported_stages`] is a bitfield of [`ShaderStageFlagBits`] describing the shader stages that [group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) with [subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup) are supported in. [`supported_stages`] will have the `VK_SHADER_STAGE_COMPUTE_BIT` bit set if any of the physical device’s queues support `VK_QUEUE_COMPUTE_BIT`.
/// - [`supported_operations`] is a bitmask of [`SubgroupFeatureFlagBits`] specifying the sets of [group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) with [subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup) supported on this device. [`supported_operations`] will have the `VK_SUBGROUP_FEATURE_BASIC_BIT` bit set if any of the physical device’s queues support `VK_QUEUE_GRAPHICS_BIT` or `VK_QUEUE_COMPUTE_BIT`.
/// - [`quad_operations_in_all_stages`] is a boolean specifying whether [quad group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-quad-operations)
///   are available in all stages, or are restricted to fragment and compute stages.
///If the [`PhysicalDeviceSubgroupProperties`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.If [`supported_operations`] includes [`VK_SUBGROUP_FEATURE_QUAD_BIT`,](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subgroup-quad)
///or [`shaderSubgroupUniformControlFlow`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-shaderSubgroupUniformControlFlow) is enabled,
///[`subgroup_size`] **must**  be greater than or equal to 4.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Bool32`]
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
#[doc(alias = "VkPhysicalDeviceSubgroupProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub subgroup_size: u32,
    ///No documentation found
    pub supported_stages: ShaderStageFlags,
    ///No documentation found
    pub supported_operations: SubgroupFeatureFlags,
    ///No documentation found
    pub quad_operations_in_all_stages: Bool32,
}
impl<'lt> Default for PhysicalDeviceSubgroupProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceSubgroupProperties,
            p_next: std::ptr::null_mut(),
            subgroup_size: 0,
            supported_stages: Default::default(),
            supported_operations: Default::default(),
            quad_operations_in_all_stages: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSubgroupProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::quad_operations_in_all_stages`]
    pub fn quad_operations_in_all_stages_raw(&self) -> Bool32 {
        self.quad_operations_in_all_stages
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::quad_operations_in_all_stages`]
    pub fn set_quad_operations_in_all_stages_raw(&mut self, value: Bool32) -> &mut Self {
        self.quad_operations_in_all_stages = value;
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
    ///Gets the value of [`Self::subgroup_size`]
    pub fn subgroup_size(&self) -> u32 {
        self.subgroup_size
    }
    ///Gets the value of [`Self::supported_stages`]
    pub fn supported_stages(&self) -> ShaderStageFlags {
        self.supported_stages
    }
    ///Gets the value of [`Self::supported_operations`]
    pub fn supported_operations(&self) -> SubgroupFeatureFlags {
        self.supported_operations
    }
    ///Gets the value of [`Self::quad_operations_in_all_stages`]
    pub fn quad_operations_in_all_stages(&self) -> bool {
        unsafe { std::mem::transmute(self.quad_operations_in_all_stages as u8) }
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
    ///Gets a mutable reference to the value of [`Self::subgroup_size`]
    pub fn subgroup_size_mut(&mut self) -> &mut u32 {
        &mut self.subgroup_size
    }
    ///Gets a mutable reference to the value of [`Self::supported_stages`]
    pub fn supported_stages_mut(&mut self) -> &mut ShaderStageFlags {
        &mut self.supported_stages
    }
    ///Gets a mutable reference to the value of [`Self::supported_operations`]
    pub fn supported_operations_mut(&mut self) -> &mut SubgroupFeatureFlags {
        &mut self.supported_operations
    }
    ///Gets a mutable reference to the value of [`Self::quad_operations_in_all_stages`]
    pub fn quad_operations_in_all_stages_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.quad_operations_in_all_stages as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.quad_operations_in_all_stages as *mut Bool32)
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
    ///Sets the raw value of [`Self::subgroup_size`]
    pub fn set_subgroup_size(&mut self, value: u32) -> &mut Self {
        self.subgroup_size = value;
        self
    }
    ///Sets the raw value of [`Self::supported_stages`]
    pub fn set_supported_stages(&mut self, value: crate::vulkan1_0::ShaderStageFlags) -> &mut Self {
        self.supported_stages = value;
        self
    }
    ///Sets the raw value of [`Self::supported_operations`]
    pub fn set_supported_operations(&mut self, value: crate::vulkan1_1::SubgroupFeatureFlags) -> &mut Self {
        self.supported_operations = value;
        self
    }
    ///Sets the raw value of [`Self::quad_operations_in_all_stages`]
    pub fn set_quad_operations_in_all_stages(&mut self, value: bool) -> &mut Self {
        self.quad_operations_in_all_stages = value as u8 as u32;
        self
    }
}
///[VkBufferMemoryRequirementsInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferMemoryRequirementsInfo2.html) - (None)
///# C Specifications
///The [`BufferMemoryRequirementsInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkBufferMemoryRequirementsInfo2 {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBuffer           buffer;
///} VkBufferMemoryRequirementsInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_memory_requirements2
///typedef VkBufferMemoryRequirementsInfo2 VkBufferMemoryRequirementsInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer`] is the buffer to query.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2`
/// - [`p_next`] **must**  be `NULL`
/// - [`buffer`] **must**  be a valid [`Buffer`] handle
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Buffer`]
/// - [`StructureType`]
/// - [`GetBufferMemoryRequirements2`]
/// - [`GetBufferMemoryRequirements2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBufferMemoryRequirementsInfo2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BufferMemoryRequirementsInfo2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`buffer`] is the buffer to query.
    pub buffer: Buffer,
}
impl<'lt> Default for BufferMemoryRequirementsInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BufferMemoryRequirementsInfo2,
            p_next: std::ptr::null(),
            buffer: Default::default(),
        }
    }
}
impl<'lt> BufferMemoryRequirementsInfo2<'lt> {
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
///[VkImageMemoryRequirementsInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageMemoryRequirementsInfo2.html) - (None)
///# C Specifications
///The [`ImageMemoryRequirementsInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkImageMemoryRequirementsInfo2 {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkImage            image;
///} VkImageMemoryRequirementsInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_memory_requirements2
///typedef VkImageMemoryRequirementsInfo2 VkImageMemoryRequirementsInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image`] is the image to query.
///# Description
///## Valid Usage
/// - If [`image`] was created with a *multi-planar* format and the `VK_IMAGE_CREATE_DISJOINT_BIT`
///   flag, there  **must**  be a [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`]
///   chain of the [`ImageMemoryRequirementsInfo2`] structure
/// - If [`image`] was created with `VK_IMAGE_CREATE_DISJOINT_BIT` and with
///   `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then there  **must**  be a
///   [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the
///   [`ImageMemoryRequirementsInfo2`] structure
/// - If [`image`] was not created with the `VK_IMAGE_CREATE_DISJOINT_BIT` flag, there  **must**
///   not be a [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the
///   [`ImageMemoryRequirementsInfo2`] structure
/// - If [`image`] was created with a single-plane format and with any `tiling` other than
///   `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then there  **must**  not be a
///   [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the
///   [`ImageMemoryRequirementsInfo2`] structure
/// - If [`image`] was created with the
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` external memory handle
///   type, then [`image`] **must**  be bound to memory
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`ImagePlaneMemoryRequirementsInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`image`] **must**  be a valid [`Image`] handle
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Image`]
/// - [`StructureType`]
/// - [`GetImageMemoryRequirements2`]
/// - [`GetImageMemoryRequirements2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageMemoryRequirementsInfo2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageMemoryRequirementsInfo2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`image`] is the image to query.
    pub image: Image,
}
impl<'lt> Default for ImageMemoryRequirementsInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ImageMemoryRequirementsInfo2,
            p_next: std::ptr::null(),
            image: Default::default(),
        }
    }
}
impl<'lt> ImageMemoryRequirementsInfo2<'lt> {
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
    ///Gets the value of [`Self::image`]
    pub fn image(&self) -> Image {
        self.image
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image`]
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
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
    ///Sets the raw value of [`Self::image`]
    pub fn set_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.image = value;
        self
    }
}
///[VkImageSparseMemoryRequirementsInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSparseMemoryRequirementsInfo2.html) - (None)
///# C Specifications
///The [`ImageSparseMemoryRequirementsInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkImageSparseMemoryRequirementsInfo2 {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkImage            image;
///} VkImageSparseMemoryRequirementsInfo2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_memory_requirements2
///typedef VkImageSparseMemoryRequirementsInfo2 VkImageSparseMemoryRequirementsInfo2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image`] is the image to query.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2`
/// - [`p_next`] **must**  be `NULL`
/// - [`image`] **must**  be a valid [`Image`] handle
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Image`]
/// - [`StructureType`]
/// - [`GetImageSparseMemoryRequirements2`]
/// - [`GetImageSparseMemoryRequirements2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageSparseMemoryRequirementsInfo2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`image`] is the image to query.
    pub image: Image,
}
impl<'lt> Default for ImageSparseMemoryRequirementsInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ImageSparseMemoryRequirementsInfo2,
            p_next: std::ptr::null(),
            image: Default::default(),
        }
    }
}
impl<'lt> ImageSparseMemoryRequirementsInfo2<'lt> {
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
    ///Gets the value of [`Self::image`]
    pub fn image(&self) -> Image {
        self.image
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image`]
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
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
    ///Sets the raw value of [`Self::image`]
    pub fn set_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.image = value;
        self
    }
}
///[VkMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryRequirements2.html) - Structure specifying memory requirements
///# C Specifications
///The [`MemoryRequirements2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkMemoryRequirements2 {
///    VkStructureType         sType;
///    void*                   pNext;
///    VkMemoryRequirements    memoryRequirements;
///} VkMemoryRequirements2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_memory_requirements2, VK_NV_ray_tracing
///typedef VkMemoryRequirements2 VkMemoryRequirements2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_requirements`] is a [`MemoryRequirements`] structure describing the memory
///   requirements of the resource.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`MemoryDedicatedRequirements`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`crate::vulkan1_1`]
/// - [`MemoryRequirements`]
/// - [`StructureType`]
/// - [`VideoGetMemoryPropertiesKHR`]
/// - [`GetBufferMemoryRequirements2`]
/// - [`GetBufferMemoryRequirements2KHR`]
/// - [`GetDeviceBufferMemoryRequirements`]
/// - [`GetDeviceBufferMemoryRequirementsKHR`]
/// - [`GetDeviceImageMemoryRequirements`]
/// - [`GetDeviceImageMemoryRequirementsKHR`]
/// - [`GetGeneratedCommandsMemoryRequirementsNV`]
/// - [`GetImageMemoryRequirements2`]
/// - [`GetImageMemoryRequirements2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryRequirements2")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryRequirements2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`memory_requirements`] is a [`MemoryRequirements`] structure
    ///describing the memory requirements of the resource.
    pub memory_requirements: MemoryRequirements,
}
impl<'lt> Default for MemoryRequirements2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MemoryRequirements2,
            p_next: std::ptr::null_mut(),
            memory_requirements: Default::default(),
        }
    }
}
impl<'lt> MemoryRequirements2<'lt> {
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
    ///Gets the value of [`Self::memory_requirements`]
    pub fn memory_requirements(&self) -> MemoryRequirements {
        self.memory_requirements
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
    ///Gets a mutable reference to the value of [`Self::memory_requirements`]
    pub fn memory_requirements_mut(&mut self) -> &mut MemoryRequirements {
        &mut self.memory_requirements
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
    ///Sets the raw value of [`Self::memory_requirements`]
    pub fn set_memory_requirements(&mut self, value: crate::vulkan1_0::MemoryRequirements) -> &mut Self {
        self.memory_requirements = value;
        self
    }
}
///[VkSparseImageMemoryRequirements2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSparseImageMemoryRequirements2.html) - (None)
///# C Specifications
///The [`SparseImageMemoryRequirements2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkSparseImageMemoryRequirements2 {
///    VkStructureType                    sType;
///    void*                              pNext;
///    VkSparseImageMemoryRequirements    memoryRequirements;
///} VkSparseImageMemoryRequirements2;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_get_memory_requirements2
///typedef VkSparseImageMemoryRequirements2 VkSparseImageMemoryRequirements2KHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`memory_requirements`] is a [`SparseImageMemoryRequirements`] structure describing the memory
///   requirements of the sparse image.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`SparseImageMemoryRequirements`]
/// - [`StructureType`]
/// - [`GetDeviceImageSparseMemoryRequirements`]
/// - [`GetDeviceImageSparseMemoryRequirementsKHR`]
/// - [`GetImageSparseMemoryRequirements2`]
/// - [`GetImageSparseMemoryRequirements2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSparseImageMemoryRequirements2")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SparseImageMemoryRequirements2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`memory_requirements`] is a [`SparseImageMemoryRequirements`]
    ///structure describing the memory requirements of the sparse image.
    pub memory_requirements: SparseImageMemoryRequirements,
}
impl<'lt> Default for SparseImageMemoryRequirements2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SparseImageMemoryRequirements2,
            p_next: std::ptr::null_mut(),
            memory_requirements: Default::default(),
        }
    }
}
impl<'lt> SparseImageMemoryRequirements2<'lt> {
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
    ///Gets the value of [`Self::memory_requirements`]
    pub fn memory_requirements(&self) -> SparseImageMemoryRequirements {
        self.memory_requirements
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
    ///Gets a mutable reference to the value of [`Self::memory_requirements`]
    pub fn memory_requirements_mut(&mut self) -> &mut SparseImageMemoryRequirements {
        &mut self.memory_requirements
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
    ///Sets the raw value of [`Self::memory_requirements`]
    pub fn set_memory_requirements(&mut self, value: crate::vulkan1_0::SparseImageMemoryRequirements) -> &mut Self {
        self.memory_requirements = value;
        self
    }
}
///[VkPhysicalDevicePointClippingProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePointClippingProperties.html) - Structure describing the point clipping behavior supported by an implementation
///# C Specifications
///The [`PhysicalDevicePointClippingProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDevicePointClippingProperties {
///    VkStructureType            sType;
///    void*                      pNext;
///    VkPointClippingBehavior    pointClippingBehavior;
///} VkPhysicalDevicePointClippingProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance2
///typedef VkPhysicalDevicePointClippingProperties VkPhysicalDevicePointClippingPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`point_clipping_behavior`] is a [`PointClippingBehavior`] value specifying the point clipping
///   behavior supported by the implementation.
///If the [`PhysicalDevicePointClippingProperties`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`PointClippingBehavior`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDevicePointClippingProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDevicePointClippingProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub point_clipping_behavior: PointClippingBehavior,
}
impl<'lt> Default for PhysicalDevicePointClippingProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDevicePointClippingProperties,
            p_next: std::ptr::null_mut(),
            point_clipping_behavior: Default::default(),
        }
    }
}
impl<'lt> PhysicalDevicePointClippingProperties<'lt> {
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
    ///Gets the value of [`Self::point_clipping_behavior`]
    pub fn point_clipping_behavior(&self) -> PointClippingBehavior {
        self.point_clipping_behavior
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
    ///Gets a mutable reference to the value of [`Self::point_clipping_behavior`]
    pub fn point_clipping_behavior_mut(&mut self) -> &mut PointClippingBehavior {
        &mut self.point_clipping_behavior
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
    ///Sets the raw value of [`Self::point_clipping_behavior`]
    pub fn set_point_clipping_behavior(&mut self, value: crate::vulkan1_1::PointClippingBehavior) -> &mut Self {
        self.point_clipping_behavior = value;
        self
    }
}
///[VkMemoryDedicatedRequirements](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedRequirements.html) - Structure describing dedicated allocation requirements of buffer and image resources
///# C Specifications
///The [`MemoryDedicatedRequirements`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkMemoryDedicatedRequirements {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           prefersDedicatedAllocation;
///    VkBool32           requiresDedicatedAllocation;
///} VkMemoryDedicatedRequirements;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_dedicated_allocation
///typedef VkMemoryDedicatedRequirements VkMemoryDedicatedRequirementsKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`prefers_dedicated_allocation`] specifies that the implementation would prefer a dedicated
///   allocation for this resource. The application is still free to suballocate the resource but it
///   **may**  get better performance if a dedicated allocation is used.
/// - [`requires_dedicated_allocation`] specifies that a dedicated allocation is required for this
///   resource.
///# Description
///To determine the dedicated allocation requirements of a buffer or image
///resource, add a [`MemoryDedicatedRequirements`] structure to the
///[`p_next`] chain of the [`MemoryRequirements2`] structure passed as the
///`pMemoryRequirements` parameter of [`GetBufferMemoryRequirements2`]
///or [`GetImageMemoryRequirements2`], respectively.Constraints on the values returned for buffer
/// resources are:
/// - [`requires_dedicated_allocation`] **may**  be [`TRUE`] if the [`p_next`] chain of
///   [`BufferCreateInfo`] for the call to [`CreateBuffer`] used to create the buffer being queried
///   included a [`ExternalMemoryBufferCreateInfo`] structure, and any of the handle types specified
///   in [`ExternalMemoryBufferCreateInfo::handle_types`] requires dedicated allocation, as reported
///   by [`GetPhysicalDeviceExternalBufferProperties`] in
///   [`ExternalBufferProperties`]::`externalMemoryProperties.externalMemoryFeatures`. Otherwise,
///   [`requires_dedicated_allocation`] will be [`FALSE`].
/// - When the implementation sets [`requires_dedicated_allocation`] to [`TRUE`], it  **must**  also
///   set [`prefers_dedicated_allocation`] to [`TRUE`].
/// - If `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` was set in [`BufferCreateInfo::flags`] when `buffer`
///   was created, then both [`prefers_dedicated_allocation`] and [`requires_dedicated_allocation`]
///   will be [`FALSE`].
///Constraints on the values returned for image resources are:
/// - [`requires_dedicated_allocation`] **may**  be [`TRUE`] if the [`p_next`] chain of
///   [`ImageCreateInfo`] for the call to [`CreateImage`] used to create the image being queried
///   included a [`ExternalMemoryImageCreateInfo`] structure, and any of the handle types specified
///   in [`ExternalMemoryImageCreateInfo::handle_types`] requires dedicated allocation, as reported
///   by [`GetPhysicalDeviceImageFormatProperties2`] in
///   [`ExternalImageFormatProperties`]::`externalMemoryProperties.externalMemoryFeatures`.
///   Otherwise, [`requires_dedicated_allocation`] will be [`FALSE`].
/// - If `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` was set in [`ImageCreateInfo::flags`] when `image` was
///   created, then both [`prefers_dedicated_allocation`] and [`requires_dedicated_allocation`] will
///   be [`FALSE`].
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS`
///# Related
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkMemoryDedicatedRequirements")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryDedicatedRequirements<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`prefers_dedicated_allocation`] specifies that the implementation would
    ///prefer a dedicated allocation for this resource.
    ///The application is still free to suballocate the resource but it  **may**
    ///get better performance if a dedicated allocation is used.
    pub prefers_dedicated_allocation: Bool32,
    ///[`requires_dedicated_allocation`] specifies that a dedicated allocation
    ///is required for this resource.
    pub requires_dedicated_allocation: Bool32,
}
impl<'lt> Default for MemoryDedicatedRequirements<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MemoryDedicatedRequirements,
            p_next: std::ptr::null_mut(),
            prefers_dedicated_allocation: 0,
            requires_dedicated_allocation: 0,
        }
    }
}
impl<'lt> MemoryDedicatedRequirements<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::prefers_dedicated_allocation`]
    pub fn prefers_dedicated_allocation_raw(&self) -> Bool32 {
        self.prefers_dedicated_allocation
    }
    ///Gets the raw value of [`Self::requires_dedicated_allocation`]
    pub fn requires_dedicated_allocation_raw(&self) -> Bool32 {
        self.requires_dedicated_allocation
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::prefers_dedicated_allocation`]
    pub fn set_prefers_dedicated_allocation_raw(&mut self, value: Bool32) -> &mut Self {
        self.prefers_dedicated_allocation = value;
        self
    }
    ///Sets the raw value of [`Self::requires_dedicated_allocation`]
    pub fn set_requires_dedicated_allocation_raw(&mut self, value: Bool32) -> &mut Self {
        self.requires_dedicated_allocation = value;
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
    ///Gets the value of [`Self::prefers_dedicated_allocation`]
    pub fn prefers_dedicated_allocation(&self) -> bool {
        unsafe { std::mem::transmute(self.prefers_dedicated_allocation as u8) }
    }
    ///Gets the value of [`Self::requires_dedicated_allocation`]
    pub fn requires_dedicated_allocation(&self) -> bool {
        unsafe { std::mem::transmute(self.requires_dedicated_allocation as u8) }
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
    ///Gets a mutable reference to the value of [`Self::prefers_dedicated_allocation`]
    pub fn prefers_dedicated_allocation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.prefers_dedicated_allocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.prefers_dedicated_allocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::requires_dedicated_allocation`]
    pub fn requires_dedicated_allocation_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.requires_dedicated_allocation as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.requires_dedicated_allocation as *mut Bool32)
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
    ///Sets the raw value of [`Self::prefers_dedicated_allocation`]
    pub fn set_prefers_dedicated_allocation(&mut self, value: bool) -> &mut Self {
        self.prefers_dedicated_allocation = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::requires_dedicated_allocation`]
    pub fn set_requires_dedicated_allocation(&mut self, value: bool) -> &mut Self {
        self.requires_dedicated_allocation = value as u8 as u32;
        self
    }
}
///[VkMemoryDedicatedAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedAllocateInfo.html) - Specify a dedicated memory allocation resource
///# C Specifications
///If the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`]
///structure, then that structure includes a handle of the sole buffer or image
///resource that the memory  **can**  be bound to.The [`MemoryDedicatedAllocateInfo`] structure is
/// defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkMemoryDedicatedAllocateInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkImage            image;
///    VkBuffer           buffer;
///} VkMemoryDedicatedAllocateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_dedicated_allocation
///typedef VkMemoryDedicatedAllocateInfo VkMemoryDedicatedAllocateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`image`] is [`crate::utils::Handle::null`] or a handle of an image which this memory will be
///   bound to.
/// - [`buffer`] is [`crate::utils::Handle::null`] or a handle of a buffer which this memory will be
///   bound to.
///# Description
///## Valid Usage
/// - At least one of [`image`] and [`buffer`] **must**  be [`crate::utils::Handle::null`]
/// - If [`image`] is not [`crate::utils::Handle::null`] and the memory is not an imported Android
///   Hardware Buffer, [`MemoryAllocateInfo::allocation_size`] **must**  equal the
///   [`MemoryRequirements::size`] of the image
/// - If [`image`] is not [`crate::utils::Handle::null`], [`image`] **must**  have been created
///   without `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` set in [`ImageCreateInfo::flags`]
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and the memory is not an imported Android
///   Hardware Buffer, [`MemoryAllocateInfo::allocation_size`] **must**  equal the
///   [`MemoryRequirements::size`] of the buffer
/// - If [`buffer`] is not [`crate::utils::Handle::null`], [`buffer`] **must**  have been created
///   without `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` set in [`BufferCreateInfo::flags`]
/// - If [`image`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a memory
///   import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`, and the external handle was created by
///   the Vulkan API, then the memory being imported  **must**  also be a dedicated image allocation
///   and [`image`] must be identical to the image associated with the imported memory
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a
///   memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`, and the external handle was created by
///   the Vulkan API, then the memory being imported  **must**  also be a dedicated buffer
///   allocation and [`buffer`] **must**  be identical to the buffer associated with the imported
///   memory
/// - If [`image`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a memory
///   import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`, the memory
///   being imported  **must**  also be a dedicated image allocation and [`image`] **must**  be
///   identical to the image associated with the imported memory
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a
///   memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`, the
///   memory being imported  **must**  also be a dedicated buffer allocation and [`buffer`] **must**
///   be identical to the buffer associated with the imported memory
/// - If [`image`] is not [`crate::utils::Handle::null`], [`image`] **must**  not have been created
///   with `VK_IMAGE_CREATE_DISJOINT_BIT` set in [`ImageCreateInfo::flags`]
/// - If [`image`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a memory
///   import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`, the
///   memory being imported  **must**  also be a dedicated image allocation and [`image`] **must**
///   be identical to the image associated with the imported memory
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a
///   memory import operation with handle type
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`, the memory being imported  **must**
///   also be a dedicated buffer allocation and [`buffer`] **must**  be identical to the buffer
///   associated with the imported memory
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO`
/// - If [`image`] is not [`crate::utils::Handle::null`], [`image`] **must**  be a valid [`Image`]
///   handle
/// - If [`buffer`] is not [`crate::utils::Handle::null`], [`buffer`] **must**  be a valid
///   [`Buffer`] handle
/// - Both of [`buffer`], and [`image`] that are valid handles of non-ignored parameters  **must**
///   have been created, allocated, or retrieved from the same [`Device`]
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Buffer`]
/// - [`Image`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryDedicatedAllocateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MemoryDedicatedAllocateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`image`] is [`crate::utils::Handle::null`] or a handle of an image which this
    ///memory will be bound to.
    pub image: Image,
    ///[`buffer`] is [`crate::utils::Handle::null`] or a handle of a buffer which this
    ///memory will be bound to.
    pub buffer: Buffer,
}
impl<'lt> Default for MemoryDedicatedAllocateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MemoryDedicatedAllocateInfo,
            p_next: std::ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
        }
    }
}
impl<'lt> MemoryDedicatedAllocateInfo<'lt> {
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
    ///Gets the value of [`Self::image`]
    pub fn image(&self) -> Image {
        self.image
    }
    ///Gets the value of [`Self::buffer`]
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::image`]
    pub fn image_mut(&mut self) -> &mut Image {
        &mut self.image
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
    ///Sets the raw value of [`Self::image`]
    pub fn set_image(&mut self, value: crate::vulkan1_0::Image) -> &mut Self {
        self.image = value;
        self
    }
    ///Sets the raw value of [`Self::buffer`]
    pub fn set_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.buffer = value;
        self
    }
}
///[VkImageViewUsageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewUsageCreateInfo.html) - Specify the intended usage of an image view
///# C Specifications
///The set of usages for the created image view  **can**  be restricted compared to
///the parent image’s [`usage`] flags by adding a
///[`ImageViewUsageCreateInfo`] structure to the [`p_next`] chain of
///[`ImageViewCreateInfo`].The [`ImageViewUsageCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkImageViewUsageCreateInfo {
///    VkStructureType      sType;
///    const void*          pNext;
///    VkImageUsageFlags    usage;
///} VkImageViewUsageCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance2
///typedef VkImageViewUsageCreateInfo VkImageViewUsageCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`usage`] is a bitmask of [`ImageUsageFlagBits`] specifying allowed usages of the image view.
///# Description
///When this structure is chained to [`ImageViewCreateInfo`] the
///[`usage`] field overrides the implicit [`usage`] parameter inherited
///from image creation time and its value is used instead for the purposes of
///determining the valid usage conditions of [`ImageViewCreateInfo`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO`
/// - [`usage`] **must**  be a valid combination of [`ImageUsageFlagBits`] values
/// - [`usage`] **must**  not be `0`
///# Related
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkImageViewUsageCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageViewUsageCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`usage`] is a bitmask of [`ImageUsageFlagBits`] specifying
    ///allowed usages of the image view.
    pub usage: ImageUsageFlags,
}
impl<'lt> Default for ImageViewUsageCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ImageViewUsageCreateInfo,
            p_next: std::ptr::null(),
            usage: Default::default(),
        }
    }
}
impl<'lt> ImageViewUsageCreateInfo<'lt> {
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
    ///Gets the value of [`Self::usage`]
    pub fn usage(&self) -> ImageUsageFlags {
        self.usage
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::usage`]
    pub fn usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.usage
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
    ///Sets the raw value of [`Self::usage`]
    pub fn set_usage(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.usage = value;
        self
    }
}
///[VkPipelineTessellationDomainOriginStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineTessellationDomainOriginStateCreateInfo.html) - Structure specifying the orientation of the tessellation domain
///# C Specifications
///The [`PipelineTessellationDomainOriginStateCreateInfo`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPipelineTessellationDomainOriginStateCreateInfo {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkTessellationDomainOrigin    domainOrigin;
///} VkPipelineTessellationDomainOriginStateCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance2
///typedef VkPipelineTessellationDomainOriginStateCreateInfo
/// VkPipelineTessellationDomainOriginStateCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`domain_origin`] is a [`TessellationDomainOrigin`] value controlling the origin of the
///   tessellation domain space.
///# Description
///If the [`PipelineTessellationDomainOriginStateCreateInfo`] structure is
///included in the [`p_next`] chain of
///[`PipelineTessellationStateCreateInfo`], it controls the origin of the
///tessellation domain.
///If this structure is not present, it is as if [`domain_origin`] was
///`VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO`
/// - [`domain_origin`] **must**  be a valid [`TessellationDomainOrigin`] value
///# Related
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
/// - [`TessellationDomainOrigin`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineTessellationDomainOriginStateCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`domain_origin`] is a [`TessellationDomainOrigin`] value
    ///controlling the origin of the tessellation domain space.
    pub domain_origin: TessellationDomainOrigin,
}
impl<'lt> Default for PipelineTessellationDomainOriginStateCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PipelineTessellationDomainOriginStateCreateInfo,
            p_next: std::ptr::null(),
            domain_origin: Default::default(),
        }
    }
}
impl<'lt> PipelineTessellationDomainOriginStateCreateInfo<'lt> {
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
    ///Gets the value of [`Self::domain_origin`]
    pub fn domain_origin(&self) -> TessellationDomainOrigin {
        self.domain_origin
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::domain_origin`]
    pub fn domain_origin_mut(&mut self) -> &mut TessellationDomainOrigin {
        &mut self.domain_origin
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
    ///Sets the raw value of [`Self::domain_origin`]
    pub fn set_domain_origin(&mut self, value: crate::vulkan1_1::TessellationDomainOrigin) -> &mut Self {
        self.domain_origin = value;
        self
    }
}
///[VkSamplerYcbcrConversionInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionInfo.html) - Structure specifying Y′C<sub>B</sub>C<sub>R</sub> conversion to a sampler or image view
///# C Specifications
///To create a sampler with Y′C<sub>B</sub>C<sub>R</sub> conversion enabled, add a
///[`SamplerYcbcrConversionInfo`] structure to the [`p_next`] chain of the
///[`SamplerCreateInfo`] structure.
///To create a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion, the
///[`samplerYcbcrConversion` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-samplerYcbcrConversion) **must**  be enabled.
///Conversion  **must**  be fixed at pipeline creation time, through use of a
///combined image sampler with an immutable sampler in
///[`DescriptorSetLayoutBinding`].A [`SamplerYcbcrConversionInfo`] **must**  be provided for
/// samplers to be
///used with image views that access `VK_IMAGE_ASPECT_COLOR_BIT` if the
///format is one of the [formats
///that require a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
///, or if the image view has an
///[external format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats)
///.The [`SamplerYcbcrConversionInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkSamplerYcbcrConversionInfo {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkSamplerYcbcrConversion    conversion;
///} VkSamplerYcbcrConversionInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkSamplerYcbcrConversionInfo VkSamplerYcbcrConversionInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`conversion`] is a [`SamplerYcbcrConversion`] handle created with
///   [`CreateSamplerYcbcrConversion`].
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO`
/// - [`conversion`] **must**  be a valid [`SamplerYcbcrConversion`] handle
///# Related
/// - [`crate::vulkan1_1`]
/// - [`SamplerYcbcrConversion`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSamplerYcbcrConversionInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SamplerYcbcrConversionInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`conversion`] is a [`SamplerYcbcrConversion`] handle created with
    ///[`CreateSamplerYcbcrConversion`].
    pub conversion: SamplerYcbcrConversion,
}
impl<'lt> Default for SamplerYcbcrConversionInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SamplerYcbcrConversionInfo,
            p_next: std::ptr::null(),
            conversion: Default::default(),
        }
    }
}
impl<'lt> SamplerYcbcrConversionInfo<'lt> {
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
    ///Gets the value of [`Self::conversion`]
    pub fn conversion(&self) -> SamplerYcbcrConversion {
        self.conversion
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::conversion`]
    pub fn conversion_mut(&mut self) -> &mut SamplerYcbcrConversion {
        &mut self.conversion
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
    ///Sets the raw value of [`Self::conversion`]
    pub fn set_conversion(&mut self, value: crate::vulkan1_1::SamplerYcbcrConversion) -> &mut Self {
        self.conversion = value;
        self
    }
}
///[VkSamplerYcbcrConversionCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html) - Structure specifying the parameters of the newly created conversion
///# C Specifications
///The [`SamplerYcbcrConversionCreateInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkSamplerYcbcrConversionCreateInfo {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkFormat                         format;
///    VkSamplerYcbcrModelConversion    ycbcrModel;
///    VkSamplerYcbcrRange              ycbcrRange;
///    VkComponentMapping               components;
///    VkChromaLocation                 xChromaOffset;
///    VkChromaLocation                 yChromaOffset;
///    VkFilter                         chromaFilter;
///    VkBool32                         forceExplicitReconstruction;
///} VkSamplerYcbcrConversionCreateInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkSamplerYcbcrConversionCreateInfo VkSamplerYcbcrConversionCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`format`] is the format of the image from which color information will be retrieved.
/// - [`ycbcr_model`] describes the color matrix for conversion between color models.
/// - [`ycbcr_range`] describes whether the encoded values have headroom and foot room, or whether
///   the encoding uses the full numerical range.
/// - [`components`] applies a *swizzle* based on [`ComponentSwizzle`] enums prior to range
///   expansion and color model conversion.
/// - [`x_chroma_offset`] describes the [sample location](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction)
///   associated with downsampled chroma components in the x dimension. [`x_chroma_offset`] has no
///   effect for formats in which chroma components are not downsampled horizontally.
/// - [`y_chroma_offset`] describes the [sample location](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction)
///   associated with downsampled chroma components in the y dimension. [`y_chroma_offset`] has no
///   effect for formats in which the chroma components are not downsampled vertically.
/// - [`chroma_filter`] is the filter for chroma reconstruction.
/// - [`force_explicit_reconstruction`] **can**  be used to ensure that reconstruction is done
///   explicitly, if supported.
///# Description
///If the [`p_next`] chain includes a [`ExternalFormatANDROID`] structure
///with non-zero `externalFormat` member, the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion
///object represents an *external format conversion*, and [`format`] **must**  be
///`VK_FORMAT_UNDEFINED`.
///Such conversions  **must**  only be used to sample image views with a matching
///[external
///format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats).
///When creating an external format conversion, the value of [`components`]
///is ignored.
///## Valid Usage
/// - If an external format conversion is being created, [`format`] **must**  be
///   `VK_FORMAT_UNDEFINED`
/// - If an external format conversion is not being created, [`format`] **must**  represent unsigned
///   normalized values (i.e. the format must be a `UNORM` format)
/// - The [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features)
///   of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion  **must**  support
///   `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT` or
///   `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`
/// -    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`, [`x_chroma_offset`] and [`y_chroma_offset`] **must**  not be `VK_CHROMA_LOCATION_COSITED_EVEN` if the corresponding components are [downsampled](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction)
/// -    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT`, [`x_chroma_offset`] and [`y_chroma_offset`] **must**  not be `VK_CHROMA_LOCATION_MIDPOINT` if the corresponding components are [downsampled](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction)
/// - If the format has a `_422` or `_420` suffix, then `components.g` **must**  be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
/// - If the format has a `_422` or `_420` suffix, then `components.a` **must**  be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings),
///   `VK_COMPONENT_SWIZZLE_ONE`, or `VK_COMPONENT_SWIZZLE_ZERO`
/// - If the format has a `_422` or `_420` suffix, then `components.r` **must**  be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
///   or `VK_COMPONENT_SWIZZLE_B`
/// - If the format has a `_422` or `_420` suffix, then `components.b` **must**  be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
///   or `VK_COMPONENT_SWIZZLE_R`
/// - If the format has a `_422` or `_420` suffix, and if either `components.r` or `components.b` is
///   the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings),
///   both values  **must**  be the identity swizzle
/// -    If [`ycbcr_model`] is not `VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY`, then `components.r`, `components.g`, and `components.b` **must**  correspond to components of the [`format`]; that is, `components.r`, `components.g`, and `components.b` **must**  not be `VK_COMPONENT_SWIZZLE_ZERO` or `VK_COMPONENT_SWIZZLE_ONE`, and  **must**  not correspond to a component containing zero or one as a consequence of [conversion to RGBA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-conversion-to-rgba)
/// - If [`ycbcr_range`] is `VK_SAMPLER_YCBCR_RANGE_ITU_NARROW` then the R, G and B components
///   obtained by applying the `component` swizzle to [`format`] **must**  each have a bit-depth
///   greater than or equal to 8
/// -    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`[`force_explicit_reconstruction`] **must**  be [`FALSE`]
/// - If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features)
///   of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`, [`chroma_filter`]
///   **must**  not be `VK_FILTER_LINEAR`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`ExternalFormatANDROID`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`format`] **must**  be a valid [`Format`] value
/// - [`ycbcr_model`] **must**  be a valid [`SamplerYcbcrModelConversion`] value
/// - [`ycbcr_range`] **must**  be a valid [`SamplerYcbcrRange`] value
/// - [`components`] **must**  be a valid [`ComponentMapping`] structure
/// - [`x_chroma_offset`] **must**  be a valid [`ChromaLocation`] value
/// - [`y_chroma_offset`] **must**  be a valid [`ChromaLocation`] value
/// - [`chroma_filter`] **must**  be a valid [`Filter`] value
///If [`chroma_filter`] is `VK_FILTER_NEAREST`, chroma samples are
///reconstructed to luma component resolution using nearest-neighbour sampling.
///Otherwise, chroma samples are reconstructed using interpolation.
///More details can be found in [the
///description of sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-sampler-YCbCr-conversion) in the [Image
///Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures) chapter.
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Bool32`]
/// - [`ChromaLocation`]
/// - [`ComponentMapping`]
/// - [`Filter`]
/// - [`Format`]
/// - [`SamplerYcbcrModelConversion`]
/// - [`SamplerYcbcrRange`]
/// - [`StructureType`]
/// - [`CreateSamplerYcbcrConversion`]
/// - [`CreateSamplerYcbcrConversionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSamplerYcbcrConversionCreateInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`format`] is the format of the image from which color information
    ///will be retrieved.
    pub format: Format,
    ///[`ycbcr_model`] describes the color matrix for conversion between color
    ///models.
    pub ycbcr_model: SamplerYcbcrModelConversion,
    ///[`ycbcr_range`] describes whether the encoded values have headroom and
    ///foot room, or whether the encoding uses the full numerical range.
    pub ycbcr_range: SamplerYcbcrRange,
    ///[`components`] applies a *swizzle* based on [`ComponentSwizzle`]
    ///enums prior to range expansion and color model conversion.
    pub components: ComponentMapping,
    ///[`x_chroma_offset`] describes the
    ///[sample location](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction) associated with
    ///downsampled chroma components in the x dimension.
    ///[`x_chroma_offset`] has no effect for formats in which chroma components
    ///are not downsampled horizontally.
    pub x_chroma_offset: ChromaLocation,
    ///[`y_chroma_offset`] describes the
    ///[sample location](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction) associated with
    ///downsampled chroma components in the y dimension.
    ///[`y_chroma_offset`] has no effect for formats in which the chroma
    ///components are not downsampled vertically.
    pub y_chroma_offset: ChromaLocation,
    ///[`chroma_filter`] is the filter for chroma reconstruction.
    pub chroma_filter: Filter,
    ///[`force_explicit_reconstruction`] **can**  be used to ensure that
    ///reconstruction is done explicitly, if supported.
    pub force_explicit_reconstruction: Bool32,
}
impl<'lt> Default for SamplerYcbcrConversionCreateInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SamplerYcbcrConversionCreateInfo,
            p_next: std::ptr::null(),
            format: Default::default(),
            ycbcr_model: Default::default(),
            ycbcr_range: Default::default(),
            components: Default::default(),
            x_chroma_offset: Default::default(),
            y_chroma_offset: Default::default(),
            chroma_filter: Default::default(),
            force_explicit_reconstruction: 0,
        }
    }
}
impl<'lt> SamplerYcbcrConversionCreateInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::force_explicit_reconstruction`]
    pub fn force_explicit_reconstruction_raw(&self) -> Bool32 {
        self.force_explicit_reconstruction
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::force_explicit_reconstruction`]
    pub fn set_force_explicit_reconstruction_raw(&mut self, value: Bool32) -> &mut Self {
        self.force_explicit_reconstruction = value;
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
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets the value of [`Self::ycbcr_model`]
    pub fn ycbcr_model(&self) -> SamplerYcbcrModelConversion {
        self.ycbcr_model
    }
    ///Gets the value of [`Self::ycbcr_range`]
    pub fn ycbcr_range(&self) -> SamplerYcbcrRange {
        self.ycbcr_range
    }
    ///Gets the value of [`Self::components`]
    pub fn components(&self) -> ComponentMapping {
        self.components
    }
    ///Gets the value of [`Self::x_chroma_offset`]
    pub fn x_chroma_offset(&self) -> ChromaLocation {
        self.x_chroma_offset
    }
    ///Gets the value of [`Self::y_chroma_offset`]
    pub fn y_chroma_offset(&self) -> ChromaLocation {
        self.y_chroma_offset
    }
    ///Gets the value of [`Self::chroma_filter`]
    pub fn chroma_filter(&self) -> Filter {
        self.chroma_filter
    }
    ///Gets the value of [`Self::force_explicit_reconstruction`]
    pub fn force_explicit_reconstruction(&self) -> bool {
        unsafe { std::mem::transmute(self.force_explicit_reconstruction as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::ycbcr_model`]
    pub fn ycbcr_model_mut(&mut self) -> &mut SamplerYcbcrModelConversion {
        &mut self.ycbcr_model
    }
    ///Gets a mutable reference to the value of [`Self::ycbcr_range`]
    pub fn ycbcr_range_mut(&mut self) -> &mut SamplerYcbcrRange {
        &mut self.ycbcr_range
    }
    ///Gets a mutable reference to the value of [`Self::components`]
    pub fn components_mut(&mut self) -> &mut ComponentMapping {
        &mut self.components
    }
    ///Gets a mutable reference to the value of [`Self::x_chroma_offset`]
    pub fn x_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.x_chroma_offset
    }
    ///Gets a mutable reference to the value of [`Self::y_chroma_offset`]
    pub fn y_chroma_offset_mut(&mut self) -> &mut ChromaLocation {
        &mut self.y_chroma_offset
    }
    ///Gets a mutable reference to the value of [`Self::chroma_filter`]
    pub fn chroma_filter_mut(&mut self) -> &mut Filter {
        &mut self.chroma_filter
    }
    ///Gets a mutable reference to the value of [`Self::force_explicit_reconstruction`]
    pub fn force_explicit_reconstruction_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.force_explicit_reconstruction as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.force_explicit_reconstruction as *mut Bool32)
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
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the raw value of [`Self::ycbcr_model`]
    pub fn set_ycbcr_model(&mut self, value: crate::vulkan1_1::SamplerYcbcrModelConversion) -> &mut Self {
        self.ycbcr_model = value;
        self
    }
    ///Sets the raw value of [`Self::ycbcr_range`]
    pub fn set_ycbcr_range(&mut self, value: crate::vulkan1_1::SamplerYcbcrRange) -> &mut Self {
        self.ycbcr_range = value;
        self
    }
    ///Sets the raw value of [`Self::components`]
    pub fn set_components(&mut self, value: crate::vulkan1_0::ComponentMapping) -> &mut Self {
        self.components = value;
        self
    }
    ///Sets the raw value of [`Self::x_chroma_offset`]
    pub fn set_x_chroma_offset(&mut self, value: crate::vulkan1_1::ChromaLocation) -> &mut Self {
        self.x_chroma_offset = value;
        self
    }
    ///Sets the raw value of [`Self::y_chroma_offset`]
    pub fn set_y_chroma_offset(&mut self, value: crate::vulkan1_1::ChromaLocation) -> &mut Self {
        self.y_chroma_offset = value;
        self
    }
    ///Sets the raw value of [`Self::chroma_filter`]
    pub fn set_chroma_filter(&mut self, value: crate::vulkan1_0::Filter) -> &mut Self {
        self.chroma_filter = value;
        self
    }
    ///Sets the raw value of [`Self::force_explicit_reconstruction`]
    pub fn set_force_explicit_reconstruction(&mut self, value: bool) -> &mut Self {
        self.force_explicit_reconstruction = value as u8 as u32;
        self
    }
}
///[VkBindImagePlaneMemoryInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImagePlaneMemoryInfo.html) - Structure specifying how to bind an image plane to memory
///# C Specifications
///In order to bind *planes* of a *disjoint image*, add a
///[`BindImagePlaneMemoryInfo`] structure to the [`p_next`] chain of
///[`BindImageMemoryInfo`].The [`BindImagePlaneMemoryInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkBindImagePlaneMemoryInfo {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkImageAspectFlagBits    planeAspect;
///} VkBindImagePlaneMemoryInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkBindImagePlaneMemoryInfo VkBindImagePlaneMemoryInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the aspect of the disjoint
///   image plane to bind.
///# Description
///## Valid Usage
/// - If the image’s `tiling` is `VK_IMAGE_TILING_LINEAR` or `VK_IMAGE_TILING_OPTIMAL`, then
///   [`plane_aspect`] **must**  be a single valid *format plane* for the image (that is, for a
///   two-plane image [`plane_aspect`] **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT` or
///   `VK_IMAGE_ASPECT_PLANE_1_BIT`, and for a three-plane image [`plane_aspect`] **must**  be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or `VK_IMAGE_ASPECT_PLANE_2_BIT`)
/// - If the image’s `tiling` is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then [`plane_aspect`]
///   **must**  be a single valid *memory plane* for the image (that is, `aspectMask` **must**
///   specify a plane index that is less than the
///   [`DrmFormatModifierPropertiesEXT::drm_format_modifier_plane_count`] associated with the
///   image’s `format` and [`ImageDrmFormatModifierPropertiesEXT::drm_format_modifier`])
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO`
/// - [`plane_aspect`] **must**  be a valid [`ImageAspectFlagBits`] value
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ImageAspectFlagBits`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindImagePlaneMemoryInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BindImagePlaneMemoryInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the
    ///aspect of the disjoint image plane to bind.
    pub plane_aspect: ImageAspectFlagBits,
}
impl<'lt> Default for BindImagePlaneMemoryInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::BindImagePlaneMemoryInfo,
            p_next: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
impl<'lt> BindImagePlaneMemoryInfo<'lt> {
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
    ///Gets the value of [`Self::plane_aspect`]
    pub fn plane_aspect(&self) -> ImageAspectFlagBits {
        self.plane_aspect
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::plane_aspect`]
    pub fn plane_aspect_mut(&mut self) -> &mut ImageAspectFlagBits {
        &mut self.plane_aspect
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
    ///Sets the raw value of [`Self::plane_aspect`]
    pub fn set_plane_aspect(&mut self, value: crate::vulkan1_0::ImageAspectFlagBits) -> &mut Self {
        self.plane_aspect = value;
        self
    }
}
///[VkImagePlaneMemoryRequirementsInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImagePlaneMemoryRequirementsInfo.html) - Structure specifying image plane for memory requirements
///# C Specifications
///To determine the memory requirements for a plane of a disjoint image, add a
///[`ImagePlaneMemoryRequirementsInfo`] structure to the [`p_next`] chain
///of the [`ImageMemoryRequirementsInfo2`] structure.The [`ImagePlaneMemoryRequirementsInfo`]
/// structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkImagePlaneMemoryRequirementsInfo {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkImageAspectFlagBits    planeAspect;
///} VkImagePlaneMemoryRequirementsInfo;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkImagePlaneMemoryRequirementsInfo VkImagePlaneMemoryRequirementsInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the aspect corresponding to the
///   image plane to query.
///# Description
///## Valid Usage
/// - If the image’s `tiling` is `VK_IMAGE_TILING_LINEAR` or `VK_IMAGE_TILING_OPTIMAL`, then
///   [`plane_aspect`] **must**  be a single valid *format plane* for the image (that is, for a
///   two-plane image [`plane_aspect`] **must**  be `VK_IMAGE_ASPECT_PLANE_0_BIT` or
///   `VK_IMAGE_ASPECT_PLANE_1_BIT`, and for a three-plane image [`plane_aspect`] **must**  be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or `VK_IMAGE_ASPECT_PLANE_2_BIT`)
/// - If the image’s `tiling` is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then [`plane_aspect`]
///   **must**  be a single valid *memory plane* for the image (that is, `aspectMask` **must**
///   specify a plane index that is less than the
///   [`DrmFormatModifierPropertiesEXT::drm_format_modifier_plane_count`] associated with the
///   image’s `format` and [`ImageDrmFormatModifierPropertiesEXT::drm_format_modifier`])
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO`
/// - [`plane_aspect`] **must**  be a valid [`ImageAspectFlagBits`] value
///# Related
/// - [`crate::vulkan1_1`]
/// - [`ImageAspectFlagBits`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImagePlaneMemoryRequirementsInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the
    ///aspect corresponding to the image plane to query.
    pub plane_aspect: ImageAspectFlagBits,
}
impl<'lt> Default for ImagePlaneMemoryRequirementsInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ImagePlaneMemoryRequirementsInfo,
            p_next: std::ptr::null(),
            plane_aspect: Default::default(),
        }
    }
}
impl<'lt> ImagePlaneMemoryRequirementsInfo<'lt> {
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
    ///Gets the value of [`Self::plane_aspect`]
    pub fn plane_aspect(&self) -> ImageAspectFlagBits {
        self.plane_aspect
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::plane_aspect`]
    pub fn plane_aspect_mut(&mut self) -> &mut ImageAspectFlagBits {
        &mut self.plane_aspect
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
    ///Sets the raw value of [`Self::plane_aspect`]
    pub fn set_plane_aspect(&mut self, value: crate::vulkan1_0::ImageAspectFlagBits) -> &mut Self {
        self.plane_aspect = value;
        self
    }
}
///[VkPhysicalDeviceSamplerYcbcrConversionFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSamplerYcbcrConversionFeatures.html) - Structure describing Y′C<sub>B</sub>C<sub>R</sub> conversion features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceSamplerYcbcrConversionFeatures`] structure is
///defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           samplerYcbcrConversion;
///} VkPhysicalDeviceSamplerYcbcrConversionFeatures;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkPhysicalDeviceSamplerYcbcrConversionFeatures
/// VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`sampler_ycbcr_conversion`] specifies whether the implementation supports [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion). If [`sampler_ycbcr_conversion`] is [`FALSE`], sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is not supported, and samplers using sampler Y′C<sub>B</sub>C<sub>R</sub> conversion  **must**  not be used.
///If the [`PhysicalDeviceSamplerYcbcrConversionFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceSamplerYcbcrConversionFeatures`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES`
///# Related
/// - [`VK_KHR_sampler_ycbcr_conversion`]
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkPhysicalDeviceSamplerYcbcrConversionFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`sampler_ycbcr_conversion`] specifies whether the implementation
    ///supports [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion).
    ///If [`sampler_ycbcr_conversion`] is [`FALSE`], sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///conversion is not supported, and samplers using sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///conversion  **must**  not be used.
    pub sampler_ycbcr_conversion: Bool32,
}
impl<'lt> Default for PhysicalDeviceSamplerYcbcrConversionFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceSamplerYcbcrConversionFeatures,
            p_next: std::ptr::null_mut(),
            sampler_ycbcr_conversion: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSamplerYcbcrConversionFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::sampler_ycbcr_conversion`]
    pub fn sampler_ycbcr_conversion_raw(&self) -> Bool32 {
        self.sampler_ycbcr_conversion
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::sampler_ycbcr_conversion`]
    pub fn set_sampler_ycbcr_conversion_raw(&mut self, value: Bool32) -> &mut Self {
        self.sampler_ycbcr_conversion = value;
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
    ///Gets the value of [`Self::sampler_ycbcr_conversion`]
    pub fn sampler_ycbcr_conversion(&self) -> bool {
        unsafe { std::mem::transmute(self.sampler_ycbcr_conversion as u8) }
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
    ///Sets the raw value of [`Self::sampler_ycbcr_conversion`]
    pub fn set_sampler_ycbcr_conversion(&mut self, value: bool) -> &mut Self {
        self.sampler_ycbcr_conversion = value as u8 as u32;
        self
    }
}
///[VkSamplerYcbcrConversionImageFormatProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionImageFormatProperties.html) - Structure specifying combined image sampler descriptor count for multi-planar images
///# C Specifications
///To determine the number of combined image samplers required to support a
///multi-planar format, add [`SamplerYcbcrConversionImageFormatProperties`]
///to the [`p_next`] chain of the [`ImageFormatProperties2`] structure in
///a call to [`GetPhysicalDeviceImageFormatProperties2`].The
/// [`SamplerYcbcrConversionImageFormatProperties`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkSamplerYcbcrConversionImageFormatProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           combinedImageSamplerDescriptorCount;
///} VkSamplerYcbcrConversionImageFormatProperties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkSamplerYcbcrConversionImageFormatProperties
/// VkSamplerYcbcrConversionImageFormatPropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`combined_image_sampler_descriptor_count`] is the number of combined image sampler
///   descriptors that the implementation uses to access the format.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSamplerYcbcrConversionImageFormatProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`combined_image_sampler_descriptor_count`] is the number of combined
    ///image sampler descriptors that the implementation uses to access the
    ///format.
    pub combined_image_sampler_descriptor_count: u32,
}
impl<'lt> Default for SamplerYcbcrConversionImageFormatProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SamplerYcbcrConversionImageFormatProperties,
            p_next: std::ptr::null_mut(),
            combined_image_sampler_descriptor_count: 0,
        }
    }
}
impl<'lt> SamplerYcbcrConversionImageFormatProperties<'lt> {
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
    ///Gets the value of [`Self::combined_image_sampler_descriptor_count`]
    pub fn combined_image_sampler_descriptor_count(&self) -> u32 {
        self.combined_image_sampler_descriptor_count
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
    ///Gets a mutable reference to the value of [`Self::combined_image_sampler_descriptor_count`]
    pub fn combined_image_sampler_descriptor_count_mut(&mut self) -> &mut u32 {
        &mut self.combined_image_sampler_descriptor_count
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
    ///Sets the raw value of [`Self::combined_image_sampler_descriptor_count`]
    pub fn set_combined_image_sampler_descriptor_count(&mut self, value: u32) -> &mut Self {
        self.combined_image_sampler_descriptor_count = value;
        self
    }
}
///[VkProtectedSubmitInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkProtectedSubmitInfo.html) - Structure indicating whether the submission is protected
///# C Specifications
///If the [`p_next`] chain of [`SubmitInfo`] includes a
///[`ProtectedSubmitInfo`] structure, then the structure indicates whether
///the batch is protected.
///The [`ProtectedSubmitInfo`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkProtectedSubmitInfo {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkBool32           protectedSubmit;
///} VkProtectedSubmitInfo;
///```
///# Members
/// - [`protected_submit`] specifies whether the batch is protected. If [`protected_submit`] is
///   [`TRUE`], the batch is protected. If [`protected_submit`] is [`FALSE`], the batch is
///   unprotected. If the [`SubmitInfo`]::[`p_next`] chain does not include this structure, the
///   batch is unprotected.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO`
///# Related
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkProtectedSubmitInfo")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ProtectedSubmitInfo<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    pub s_type: StructureType,
    ///No documentation found
    pub p_next: *const BaseInStructure<'lt>,
    ///[`protected_submit`] specifies whether the batch is protected.
    ///If [`protected_submit`] is [`TRUE`], the batch is protected.
    ///If [`protected_submit`] is [`FALSE`], the batch is unprotected.
    ///If the [`SubmitInfo`]::[`p_next`] chain does not include this
    ///structure, the batch is unprotected.
    pub protected_submit: Bool32,
}
impl<'lt> Default for ProtectedSubmitInfo<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::ProtectedSubmitInfo,
            p_next: std::ptr::null(),
            protected_submit: 0,
        }
    }
}
impl<'lt> ProtectedSubmitInfo<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::protected_submit`]
    pub fn protected_submit_raw(&self) -> Bool32 {
        self.protected_submit
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::protected_submit`]
    pub fn set_protected_submit_raw(&mut self, value: Bool32) -> &mut Self {
        self.protected_submit = value;
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
    ///Gets the value of [`Self::protected_submit`]
    pub fn protected_submit(&self) -> bool {
        unsafe { std::mem::transmute(self.protected_submit as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::protected_submit`]
    pub fn protected_submit_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.protected_submit as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.protected_submit as *mut Bool32)
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
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::protected_submit`]
    pub fn set_protected_submit(&mut self, value: bool) -> &mut Self {
        self.protected_submit = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceProtectedMemoryFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryFeatures.html) - Structure describing protected memory features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceProtectedMemoryFeatures`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceProtectedMemoryFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           protectedMemory;
///} VkPhysicalDeviceProtectedMemoryFeatures;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`protected_memory`] specifies whether protected memory is supported.
///If the [`PhysicalDeviceProtectedMemoryFeatures`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceProtectedMemoryFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES`
///# Related
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkPhysicalDeviceProtectedMemoryFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`protected_memory`]
    ///specifies whether protected memory is supported.
    pub protected_memory: Bool32,
}
impl<'lt> Default for PhysicalDeviceProtectedMemoryFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceProtectedMemoryFeatures,
            p_next: std::ptr::null_mut(),
            protected_memory: 0,
        }
    }
}
impl<'lt> PhysicalDeviceProtectedMemoryFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::protected_memory`]
    pub fn protected_memory_raw(&self) -> Bool32 {
        self.protected_memory
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::protected_memory`]
    pub fn set_protected_memory_raw(&mut self, value: Bool32) -> &mut Self {
        self.protected_memory = value;
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
    ///Gets the value of [`Self::protected_memory`]
    pub fn protected_memory(&self) -> bool {
        unsafe { std::mem::transmute(self.protected_memory as u8) }
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
    ///Sets the raw value of [`Self::protected_memory`]
    pub fn set_protected_memory(&mut self, value: bool) -> &mut Self {
        self.protected_memory = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceProtectedMemoryProperties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProtectedMemoryProperties.html) - Structure describing protected memory properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceProtectedMemoryProperties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceProtectedMemoryProperties {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           protectedNoFault;
///} VkPhysicalDeviceProtectedMemoryProperties;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`protected_no_fault`] specifies how an implementation behaves when an application attempts to
///   write to unprotected memory in a protected queue operation, read from protected memory in an unprotected
///   queue operation, or perform a query in a protected queue operation. If this limit is [`TRUE`],
///   such writes will be discarded or have undefined values written, reads and queries will return undefined
///   values. If this limit is [`FALSE`], applications  **must**  not perform these operations. See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-protected-access-rules](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-protected-access-rules)
///   for more information.
///If the [`PhysicalDeviceProtectedMemoryProperties`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES`
///# Related
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkPhysicalDeviceProtectedMemoryProperties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub protected_no_fault: Bool32,
}
impl<'lt> Default for PhysicalDeviceProtectedMemoryProperties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceProtectedMemoryProperties,
            p_next: std::ptr::null_mut(),
            protected_no_fault: 0,
        }
    }
}
impl<'lt> PhysicalDeviceProtectedMemoryProperties<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
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
    ///Gets the value of [`Self::protected_no_fault`]
    pub fn protected_no_fault(&self) -> bool {
        unsafe { std::mem::transmute(self.protected_no_fault as u8) }
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
    ///Sets the raw value of [`Self::protected_no_fault`]
    pub fn set_protected_no_fault(&mut self, value: bool) -> &mut Self {
        self.protected_no_fault = value as u8 as u32;
        self
    }
}
///[VkDeviceQueueInfo2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceQueueInfo2.html) - Structure specifying the parameters used for device queue creation
///# C Specifications
///The [`DeviceQueueInfo2`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkDeviceQueueInfo2 {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkDeviceQueueCreateFlags    flags;
///    uint32_t                    queueFamilyIndex;
///    uint32_t                    queueIndex;
///} VkDeviceQueueInfo2;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure. The [`p_next`]
///   chain of [`DeviceQueueInfo2`] **can**  be used to provide additional device queue parameters
///   to [`GetDeviceQueue2`].
/// - [`flags`] is a [`DeviceQueueCreateFlags`] value indicating the flags used to create the device
///   queue.
/// - [`queue_family_index`] is the index of the queue family to which the queue belongs.
/// - [`queue_index`] is the index within this queue family of the queue to retrieve.
///# Description
///The queue returned by [`GetDeviceQueue2`] **must**  have the same
///[`flags`] value from this structure as that used at device creation time
///in a [`DeviceQueueCreateInfo`] structure.
///If no matching [`flags`] were specified at device creation time, then the
///handle returned in `pQueue` **must**  be `NULL`.
///## Valid Usage
/// - [`queue_family_index`] **must**  be one of the queue family indices specified when `device`
///   was created, via the [`DeviceQueueCreateInfo`] structure
/// - [`flags`] **must**  be equal to [`DeviceQueueCreateInfo`]::[`flags`] for a
///   [`DeviceQueueCreateInfo`] structure for the queue family indicated by [`queue_family_index`]
///   when `device` was created
/// - [`queue_index`] **must**  be less than [`DeviceQueueCreateInfo::queue_count`] for the
///   corresponding queue family and flags indicated by [`queue_family_index`] and [`flags`] when
///   `device` was created
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`
/// - [`p_next`] **must**  be `NULL`
/// - [`flags`] **must**  be a valid combination of [`DeviceQueueCreateFlagBits`] values
///# Related
/// - [`crate::vulkan1_1`]
/// - [`DeviceQueueCreateFlags`]
/// - [`StructureType`]
/// - [`GetDeviceQueue2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceQueueInfo2")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceQueueInfo2<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    ///The [`p_next`] chain of [`DeviceQueueInfo2`] **can**  be used to
    ///provide additional device queue parameters to [`GetDeviceQueue2`].
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a [`DeviceQueueCreateFlags`] value indicating the
    ///flags used to create the device queue.
    pub flags: DeviceQueueCreateFlags,
    ///[`queue_family_index`] is the index of the queue family to which the
    ///queue belongs.
    pub queue_family_index: u32,
    ///[`queue_index`] is the index within this queue family of the queue to
    ///retrieve.
    pub queue_index: u32,
}
impl<'lt> Default for DeviceQueueInfo2<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DeviceQueueInfo2,
            p_next: std::ptr::null(),
            flags: Default::default(),
            queue_family_index: 0,
            queue_index: 0,
        }
    }
}
impl<'lt> DeviceQueueInfo2<'lt> {
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
    pub fn flags(&self) -> DeviceQueueCreateFlags {
        self.flags
    }
    ///Gets the value of [`Self::queue_family_index`]
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    ///Gets the value of [`Self::queue_index`]
    pub fn queue_index(&self) -> u32 {
        self.queue_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DeviceQueueCreateFlags {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::queue_family_index`]
    pub fn queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.queue_family_index
    }
    ///Gets a mutable reference to the value of [`Self::queue_index`]
    pub fn queue_index_mut(&mut self) -> &mut u32 {
        &mut self.queue_index
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
    pub fn set_flags(&mut self, value: crate::vulkan1_0::DeviceQueueCreateFlags) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_index`]
    pub fn set_queue_family_index(&mut self, value: u32) -> &mut Self {
        self.queue_family_index = value;
        self
    }
    ///Sets the raw value of [`Self::queue_index`]
    pub fn set_queue_index(&mut self, value: u32) -> &mut Self {
        self.queue_index = value;
        self
    }
}
///[VkPhysicalDeviceMaintenance3Properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMaintenance3Properties.html) - Structure describing descriptor set properties
///# C Specifications
///The [`PhysicalDeviceMaintenance3Properties`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceMaintenance3Properties {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxPerSetDescriptors;
///    VkDeviceSize       maxMemoryAllocationSize;
///} VkPhysicalDeviceMaintenance3Properties;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance3
///typedef VkPhysicalDeviceMaintenance3Properties VkPhysicalDeviceMaintenance3PropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///# Description
/// - [`max_per_set_descriptors`] is a maximum number of descriptors (summed over all descriptor
///   types) in a single descriptor set that is guaranteed to satisfy any implementation-dependent
///   constraints on the size of a descriptor set itself. Applications  **can**  query whether a
///   descriptor set that goes beyond this limit is supported using
///   [`GetDescriptorSetLayoutSupport`].
/// - [`max_memory_allocation_size`] is the maximum size of a memory allocation that  **can**  be
///   created, even if there is more space available in the heap.
///If the [`PhysicalDeviceMaintenance3Properties`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES`
///# Related
/// - [`crate::vulkan1_1`]
/// - [`DeviceSize`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceMaintenance3Properties")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///No documentation found
    pub max_per_set_descriptors: u32,
    ///No documentation found
    pub max_memory_allocation_size: DeviceSize,
}
impl<'lt> Default for PhysicalDeviceMaintenance3Properties<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceMaintenance3Properties,
            p_next: std::ptr::null_mut(),
            max_per_set_descriptors: 0,
            max_memory_allocation_size: Default::default(),
        }
    }
}
impl<'lt> PhysicalDeviceMaintenance3Properties<'lt> {
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
///[VkDescriptorSetLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutSupport.html) - Structure returning information about whether a descriptor set layout can be supported
///# C Specifications
///Information about support for the descriptor set layout is returned in a
///[`DescriptorSetLayoutSupport`] structure:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkDescriptorSetLayoutSupport {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           supported;
///} VkDescriptorSetLayoutSupport;
///```
///or the equivalent
///```c
///// Provided by VK_KHR_maintenance3
///typedef VkDescriptorSetLayoutSupport VkDescriptorSetLayoutSupportKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`supported`] specifies whether the descriptor set layout  **can**  be created.
///# Description
///[`supported`] is set to [`TRUE`] if the descriptor set  **can**  be
///created, or else is set to [`FALSE`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`DescriptorSetVariableDescriptorCountLayoutSupport`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
///# Related
/// - [`crate::vulkan1_1`]
/// - [`Bool32`]
/// - [`StructureType`]
/// - [`GetDescriptorSetLayoutSupport`]
/// - [`GetDescriptorSetLayoutSupportKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDescriptorSetLayoutSupport")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DescriptorSetLayoutSupport<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`supported`] specifies whether the descriptor set layout  **can**  be
    ///created.
    pub supported: Bool32,
}
impl<'lt> Default for DescriptorSetLayoutSupport<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DescriptorSetLayoutSupport,
            p_next: std::ptr::null_mut(),
            supported: 0,
        }
    }
}
impl<'lt> DescriptorSetLayoutSupport<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::supported`]
    pub fn supported_raw(&self) -> Bool32 {
        self.supported
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::supported`]
    pub fn set_supported_raw(&mut self, value: Bool32) -> &mut Self {
        self.supported = value;
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
    ///Gets the value of [`Self::supported`]
    pub fn supported(&self) -> bool {
        unsafe { std::mem::transmute(self.supported as u8) }
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
    ///Gets a mutable reference to the value of [`Self::supported`]
    pub fn supported_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.supported as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.supported as *mut Bool32)
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
    ///Sets the raw value of [`Self::supported`]
    pub fn set_supported(&mut self, value: bool) -> &mut Self {
        self.supported = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceShaderDrawParametersFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderDrawParametersFeatures.html) - Structure describing shader draw parameter features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderDrawParametersFeatures`] structure is defined
///as:
///```c
///// Provided by VK_VERSION_1_1
///typedef struct VkPhysicalDeviceShaderDrawParametersFeatures {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shaderDrawParameters;
///} VkPhysicalDeviceShaderDrawParametersFeatures;
///```
///
///```c
///// Provided by VK_VERSION_1_1
///typedef VkPhysicalDeviceShaderDrawParametersFeatures
/// VkPhysicalDeviceShaderDrawParameterFeatures;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
///
/// - [`shader_draw_parameters`] specifies whether the implementation supports the SPIR-V
///   `DrawParameters` capability. When this feature is not enabled, shader modules  **must**  not
///   declare the `SPV_KHR_shader_draw_parameters` extension or the `DrawParameters` capability.
///If the [`PhysicalDeviceShaderDrawParametersFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderDrawParametersFeatures`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES`
///# Related
/// - [`crate::vulkan1_1`]
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
#[doc(alias = "VkPhysicalDeviceShaderDrawParametersFeatures")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_draw_parameters`] specifies whether the implementation supports
    ///the SPIR-V `DrawParameters` capability.
    ///When this feature is not enabled, shader modules  **must**  not declare the
    ///`SPV_KHR_shader_draw_parameters` extension or the `DrawParameters`
    ///capability.
    pub shader_draw_parameters: Bool32,
}
impl<'lt> Default for PhysicalDeviceShaderDrawParametersFeatures<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PhysicalDeviceShaderDrawParametersFeatures,
            p_next: std::ptr::null_mut(),
            shader_draw_parameters: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderDrawParametersFeatures<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
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
    ///Sets the raw value of [`Self::shader_draw_parameters`]
    pub fn set_shader_draw_parameters(&mut self, value: bool) -> &mut Self {
        self.shader_draw_parameters = value as u8 as u32;
        self
    }
}
///[VkDescriptorUpdateTemplate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplate.html) - Opaque handle to a descriptor update template
///# C Specifications
///A descriptor update template specifies a mapping from descriptor update
///information in host memory to descriptors in a descriptor set.
///It is designed to avoid passing redundant information to the driver when
///frequently updating the same set of descriptors in descriptor sets.Descriptor update template
/// objects are represented by
///[`DescriptorUpdateTemplate`] handles:
///```c
///// Provided by VK_VERSION_1_1
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorUpdateTemplate)
///```
///or the equivalent
///```c
///// Provided by VK_KHR_descriptor_update_template
///typedef VkDescriptorUpdateTemplate VkDescriptorUpdateTemplateKHR;
///```
///# Related
/// - [`crate::vulkan1_1`]
/// - [`CmdPushDescriptorSetWithTemplateKHR`]
/// - [`CreateDescriptorUpdateTemplate`]
/// - [`CreateDescriptorUpdateTemplateKHR`]
/// - [`DestroyDescriptorUpdateTemplate`]
/// - [`DestroyDescriptorUpdateTemplateKHR`]
/// - [`UpdateDescriptorSetWithTemplate`]
/// - [`UpdateDescriptorSetWithTemplateKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDescriptorUpdateTemplate")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct DescriptorUpdateTemplate(pub u64);
impl DescriptorUpdateTemplate {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for DescriptorUpdateTemplate {}
impl Default for DescriptorUpdateTemplate {
    fn default() -> Self {
        Self::null()
    }
}
///[VkSamplerYcbcrConversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversion.html) - Opaque handle to a device-specific sampler Y′C<sub>B</sub>C<sub>R</sub> conversion description
///# C Specifications
///A sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is an opaque representation of a
///device-specific sampler Y′C<sub>B</sub>C<sub>R</sub> conversion description, represented as a
///[`SamplerYcbcrConversion`] handle:
///```c
///// Provided by VK_VERSION_1_1
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSamplerYcbcrConversion)
///```
///or the equivalent
///```c
///// Provided by VK_KHR_sampler_ycbcr_conversion
///typedef VkSamplerYcbcrConversion VkSamplerYcbcrConversionKHR;
///```
///# Related
/// - [`crate::vulkan1_1`]
/// - [`SamplerYcbcrConversionInfo`]
/// - [`CreateSamplerYcbcrConversion`]
/// - [`CreateSamplerYcbcrConversionKHR`]
/// - [`DestroySamplerYcbcrConversion`]
/// - [`DestroySamplerYcbcrConversionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSamplerYcbcrConversion")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct SamplerYcbcrConversion(pub u64);
impl SamplerYcbcrConversion {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for SamplerYcbcrConversion {}
impl Default for SamplerYcbcrConversion {
    fn default() -> Self {
        Self::null()
    }
}
