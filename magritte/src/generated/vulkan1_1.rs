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
use std::marker::PhantomData;
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
        DescriptorSet
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
        self as i32
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
        AllClipPlanes
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
        self as i32
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
        UpperLeft
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
        self as i32
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
        RgbIdentity
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
        self as i32
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
        ItuFull
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
        self as i32
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
        CositedEven
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
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
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
///This structure **can** be used in [`GetPhysicalDeviceFeatures2`] or **can** be
///included in the [`p_next`] chain of a [`DeviceCreateInfo`] structure,
///in which case it controls which features are enabled in the device in lieu
///of `pEnabledFeatures`.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceFeatures2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`features`] is a [`PhysicalDeviceFeatures`] structure describing
    ///the fine-grained features of the Vulkan 1.0 API.
    features: PhysicalDeviceFeatures,
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
///properties defined by extensions.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
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
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceProperties2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`properties`] is a [`PhysicalDeviceProperties`] structure
    ///describing properties of the physical device.
    ///This structure is written with the same values as if it were written by
    ///[`GetPhysicalDeviceProperties`].
    properties: PhysicalDeviceProperties,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`DrmFormatModifierPropertiesList2EXT`],
///   [`DrmFormatModifierPropertiesListEXT`], [`FormatProperties3`], [`VideoDecodeH264ProfileEXT`],
///   [`VideoDecodeH265ProfileEXT`], [`VideoEncodeH264ProfileEXT`], [`VideoEncodeH265ProfileEXT`],
///   [`VideoProfileKHR`], or [`VideoProfilesKHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct FormatProperties2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`format_properties`] is a [`FormatProperties`] structure
    ///describing features supported by the requested format.
    format_properties: FormatProperties,
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
///[`image_format_properties`] will be filled with zero.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`AndroidHardwareBufferUsageANDROID`],
///   [`ExternalImageFormatProperties`], [`FilterCubicImageViewImageFormatPropertiesEXT`],
///   [`SamplerYcbcrConversionImageFormatProperties`], or [`TextureLodGatherFormatPropertiesAMD`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageFormatProperties2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    ///The [`p_next`] chain of [`ImageFormatProperties2`] is used to allow
    ///the specification of additional capabilities to be returned from
    ///[`GetPhysicalDeviceImageFormatProperties2`].
    p_next: *const BaseOutStructure<'lt>,
    ///[`image_format_properties`] is a [`ImageFormatProperties`] structure
    ///in which capabilities are returned.
    image_format_properties: ImageFormatProperties,
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
///[`s_type`] and [`p_next`] added for extensibility.Valid Usage
/// - [`tiling`]**must** be `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` if and only if the [`p_next`]
///   chain includes [`PhysicalDeviceImageDrmFormatModifierInfoEXT`]
/// - If [`tiling`] is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT` and [`flags`] contains
///   `VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT`, then the [`p_next`] chain **must** include a
///   [`ImageFormatListCreateInfo`] structure with non-zero `viewFormatCount`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`ImageFormatListCreateInfo`],
///   [`ImageStencilUsageCreateInfo`], [`PhysicalDeviceExternalImageFormatInfo`],
///   [`PhysicalDeviceImageDrmFormatModifierInfoEXT`], or
///   [`PhysicalDeviceImageViewImageFormatInfoEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`format`]**must** be a valid [`Format`] value
/// - [`type_`]**must** be a valid [`ImageType`] value
/// - [`tiling`]**must** be a valid [`ImageTiling`] value
/// - [`usage`]**must** be a valid combination of [`ImageUsageFlagBits`] values
/// - [`usage`]**must** not be `0`
/// - [`flags`]**must** be a valid combination of [`ImageCreateFlagBits`] values
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceImageFormatInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    ///The [`p_next`] chain of [`PhysicalDeviceImageFormatInfo2`] is used
    ///to provide additional image parameters to
    ///[`GetPhysicalDeviceImageFormatProperties2`].
    p_next: *mut BaseInStructure<'lt>,
    ///[`format`] is a [`Format`] value indicating the image format,
    ///corresponding to [`ImageCreateInfo`]::[`format`].
    format: Format,
    ///[`type_`] is a [`ImageType`] value indicating the image type,
    ///corresponding to [`ImageCreateInfo`]::`imageType`.
    type_: ImageType,
    ///[`tiling`] is a [`ImageTiling`] value indicating the image tiling,
    ///corresponding to [`ImageCreateInfo`]::[`tiling`].
    tiling: ImageTiling,
    ///[`usage`] is a bitmask of [`ImageUsageFlagBits`] indicating the
    ///intended usage of the image, corresponding to
    ///[`ImageCreateInfo`]::[`usage`].
    usage: ImageUsageFlags,
    ///[`flags`] is a bitmask of [`ImageCreateFlagBits`] indicating
    ///additional parameters of the image, corresponding to
    ///[`ImageCreateInfo`]::[`flags`].
    flags: ImageCreateFlags,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`QueueFamilyCheckpointProperties2NV`],
///   [`QueueFamilyCheckpointPropertiesNV`], [`QueueFamilyGlobalPriorityPropertiesKHR`],
///   [`QueueFamilyQueryResultStatusProperties2KHR`], or [`VideoQueueFamilyProperties2KHR`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueueFamilyProperties2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`queue_family_properties`] is a [`QueueFamilyProperties`] structure
    ///which is populated with the same values as in
    ///[`GetPhysicalDeviceQueueFamilyProperties`].
    queue_family_properties: QueueFamilyProperties,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`PhysicalDeviceMemoryBudgetPropertiesEXT`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`memory_properties`] is a [`PhysicalDeviceMemoryProperties`]
    ///structure which is populated with the same values as in
    ///[`GetPhysicalDeviceMemoryProperties`].
    memory_properties: PhysicalDeviceMemoryProperties,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SparseImageFormatProperties2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`properties`] is a [`SparseImageFormatProperties`] structure
    ///which is populated with the same values as in
    ///[`GetPhysicalDeviceSparseImageFormatProperties`].
    properties: SparseImageFormatProperties,
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
///Valid Usage
/// - [`samples`]**must** be a bit value that is set in [`ImageFormatProperties::sample_counts`]
///   returned by [`GetPhysicalDeviceImageFormatProperties`] with [`format`], [`type_`], [`tiling`],
///   and [`usage`] equal to those in this command and `flags` equal to the value that is set in
///   [`ImageCreateInfo::flags`] when the image is created
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`format`]**must** be a valid [`Format`] value
/// - [`type_`]**must** be a valid [`ImageType`] value
/// - [`samples`]**must** be a valid [`SampleCountFlagBits`] value
/// - [`usage`]**must** be a valid combination of [`ImageUsageFlagBits`] values
/// - [`usage`]**must** not be `0`
/// - [`tiling`]**must** be a valid [`ImageTiling`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceSparseImageFormatInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`format`] is the image format.
    format: Format,
    ///[`type_`] is the dimensionality of image.
    type_: ImageType,
    ///[`samples`] is a [`SampleCountFlagBits`] value specifying the
    ///number of samples per texel.
    samples: SampleCountFlagBits,
    ///[`usage`] is a bitmask describing the intended usage of the image.
    usage: ImageUsageFlags,
    ///[`tiling`] is the tiling arrangement of the texel blocks in memory.
    tiling: ImageTiling,
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
///   **must** not declare the `SPV_KHR_variable_pointers` extension or the
///   `VariablePointersStorageBuffer` capability.
/// - [`variable_pointers`] specifies whether the implementation supports the SPIR-V
///   `VariablePointers` capability. When this feature is not enabled, shader modules **must** not
///   declare the `VariablePointers` capability.
///If the [`PhysicalDeviceVariablePointersFeatures`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceVariablePointersFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage
/// - If [`variable_pointers`] is enabled then [`variable_pointers_storage_buffer`]**must** also be
///   enabled
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
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
///   specifying which types of imported handle `handleType`**can** be exported from.
/// - [`compatible_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBits`] specifying
///   handle types which **can** be specified at the same time as `handleType` when creating an
///   image compatible with external memory.
///# Description
///[`compatible_handle_types`]**must** include at least `handleType`.
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalMemoryProperties {
    ///[`external_memory_features`] is a bitmask of
    ///[`ExternalMemoryFeatureFlagBits`] specifying the features of
    ///`handleType`.
    external_memory_features: ExternalMemoryFeatureFlags,
    ///[`export_from_imported_handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying which types of
    ///imported handle `handleType`**can** be exported from.
    export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    ///[`compatible_handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying handle types which
    ///**can** be specified at the same time as `handleType` when creating an
    ///image compatible with external memory.
    compatible_handle_types: ExternalMemoryHandleTypeFlags,
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
///`VK_ERROR_FORMAT_NOT_SUPPORTED`.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO`
/// - If [`handle_type`] is not `0`, [`handle_type`]**must** be a valid
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceExternalImageFormatInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the memory handle type that will be used with the memory
    ///associated with the image.
    handle_type: ExternalMemoryHandleTypeFlagBits,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalImageFormatProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`external_memory_properties`] is a [`ExternalMemoryProperties`]
    ///structure specifying various capabilities of the external handle type
    ///when used with the specified image creation parameters.
    external_memory_properties: ExternalMemoryProperties,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be a valid combination of [`BufferCreateFlagBits`] values
/// - [`usage`]**must** be a valid combination of [`BufferUsageFlagBits`] values
/// - [`usage`]**must** not be `0`
/// - [`handle_type`]**must** be a valid [`ExternalMemoryHandleTypeFlagBits`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceExternalBufferInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`BufferCreateFlagBits`] describing
    ///additional parameters of the buffer, corresponding to
    ///[`BufferCreateInfo`]::[`flags`].
    flags: BufferCreateFlags,
    ///[`usage`] is a bitmask of [`BufferUsageFlagBits`] describing the
    ///intended usage of the buffer, corresponding to
    ///[`BufferCreateInfo`]::[`usage`].
    usage: BufferUsageFlags,
    ///[`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value
    ///specifying the memory handle type that will be used with the memory
    ///associated with the buffer.
    handle_type: ExternalMemoryHandleTypeFlagBits,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalBufferProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`external_memory_properties`] is a [`ExternalMemoryProperties`]
    ///structure specifying various capabilities of the external handle type
    ///when used with the specified buffer creation parameters.
    external_memory_properties: ExternalMemoryProperties,
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
///corresponding implementation-dependent property.[`device_uuid`]**must** be immutable for a given
/// device across instances,
///processes, driver APIs, driver versions, and system reboots.Applications **can** compare the
/// [`driver_uuid`] value across instance and
///process boundaries, and **can** make similar queries in external APIs to
///determine whether they are capable of sharing memory objects and resources
///using them with the device.[`device_uuid`] and/or [`driver_uuid`]**must** be used to determine
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
///Windows operating system, the contents of [`device_luid`]**can** be cast to
///an `LUID` object and **must** be equal to the locally unique identifier of a
///`IDXGIAdapter1` object that corresponds to `physicalDevice`.
///If [`device_luid_valid`] is [`TRUE`], [`device_node_mask`]**must**
///contain exactly one bit.
///If Vulkan is running on an operating system that supports the Direct3D 12
///API and `physicalDevice` corresponds to an individual device in a linked
///device adapter, [`device_node_mask`] identifies the Direct3D 12 node
///corresponding to `physicalDevice`.
///Otherwise, [`device_node_mask`]**must** be `1`.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceIdProperties<'lt> {
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
}
///[VkExternalMemoryImageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryImageCreateInfo.html) - Specify that an image may be backed by external memory
///# C Specifications
///To define a set of external memory handle types that **may** be used as backing
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO`
/// - [`handle_types`]**must** be a valid combination of [`ExternalMemoryHandleTypeFlagBits`] values
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalMemoryImageCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_types`] is zero, or a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying one or more external
    ///memory handle types.
    handle_types: ExternalMemoryHandleTypeFlags,
}
///[VkExternalMemoryBufferCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryBufferCreateInfo.html) - Specify that a buffer may be backed by external memory
///# C Specifications
///To define a set of external memory handle types that **may** be used as backing
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO`
/// - [`handle_types`]**must** be a valid combination of [`ExternalMemoryHandleTypeFlagBits`] values
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalMemoryBufferCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_types`] is zero, or a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying one or more external
    ///memory handle types.
    handle_types: ExternalMemoryHandleTypeFlags,
}
///[VkExportMemoryAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportMemoryAllocateInfo.html) - Specify exportable handle types for a device memory object
///# C Specifications
///When allocating memory whose payload **may** be exported to another process or
///Vulkan instance, add a [`ExportMemoryAllocateInfo`] structure to the
///[`p_next`] chain of the [`MemoryAllocateInfo`] structure, specifying
///the handle types that **may** be exported.The [`ExportMemoryAllocateInfo`] structure is defined
/// as:
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
///   memory handle types the application **can** export from the resulting allocation. The
///   application **can** request multiple handle types for the same allocation.
///# Description
///Valid Usage
/// - The bits in [`handle_types`]**must** be supported and compatible, as reported by
///   [`ExternalImageFormatProperties`] or [`ExternalBufferProperties`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO`
/// - [`handle_types`]**must** be a valid combination of [`ExternalMemoryHandleTypeFlagBits`] values
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExportMemoryAllocateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBits`] specifying one or more memory
    ///handle types the application **can** export from the resulting allocation.
    ///The application **can** request multiple handle types for the same
    ///allocation.
    handle_types: ExternalMemoryHandleTypeFlags,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of [`SemaphoreTypeCreateInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`handle_type`]**must** be a valid [`ExternalSemaphoreHandleTypeFlagBits`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceExternalSemaphoreInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalSemaphoreHandleTypeFlagBits`] value
    ///specifying the external semaphore handle type for which capabilities
    ///will be returned.
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
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
///   specifying which types of imported handle `handleType`**can** be exported from.
/// - [`compatible_handle_types`] is a bitmask of [`ExternalSemaphoreHandleTypeFlagBits`] specifying
///   handle types which **can** be specified at the same time as `handleType` when creating a
///   semaphore.
/// - [`external_semaphore_features`] is a bitmask of [`ExternalSemaphoreFeatureFlagBits`]
///   describing the features of `handleType`.
///# Description
///If `handleType` is not supported by the implementation, then
///[`ExternalSemaphoreProperties`]::[`external_semaphore_features`] will be
///set to zero.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalSemaphoreProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`export_from_imported_handle_types`] is a bitmask of
    ///[`ExternalSemaphoreHandleTypeFlagBits`] specifying which types of
    ///imported handle `handleType`**can** be exported from.
    export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    ///[`compatible_handle_types`] is a bitmask of
    ///[`ExternalSemaphoreHandleTypeFlagBits`] specifying handle types
    ///which **can** be specified at the same time as `handleType` when
    ///creating a semaphore.
    compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    ///[`external_semaphore_features`] is a bitmask of
    ///[`ExternalSemaphoreFeatureFlagBits`] describing the features of
    ///`handleType`.
    external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
///[VkExportSemaphoreCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportSemaphoreCreateInfo.html) - Structure specifying handle types that can be exported from a semaphore
///# C Specifications
///To create a semaphore whose payload **can** be exported to external handles,
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
///   more semaphore handle types the application **can** export from the resulting semaphore. The
///   application **can** request multiple handle types for the same semaphore.
///# Description
///Valid Usage
/// - The bits in [`handle_types`]**must** be supported and compatible, as reported by
///   [`ExternalSemaphoreProperties`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO`
/// - [`handle_types`]**must** be a valid combination of [`ExternalSemaphoreHandleTypeFlagBits`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExportSemaphoreCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_types`] is a bitmask of
    ///[`ExternalSemaphoreHandleTypeFlagBits`] specifying one or more
    ///semaphore handle types the application **can** export from the resulting
    ///semaphore.
    ///The application **can** request multiple handle types for the same
    ///semaphore.
    handle_types: ExternalSemaphoreHandleTypeFlags,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`handle_type`]**must** be a valid [`ExternalFenceHandleTypeFlagBits`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceExternalFenceInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_type`] is a [`ExternalFenceHandleTypeFlagBits`] value
    ///specifying an external fence handle type for which capabilities will be
    ///returned.
    handle_type: ExternalFenceHandleTypeFlagBits,
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
///   indicating which types of imported handle `handleType`**can** be exported from.
/// - [`compatible_handle_types`] is a bitmask of [`ExternalFenceHandleTypeFlagBits`] specifying
///   handle types which **can** be specified at the same time as `handleType` when creating a
///   fence.
/// - [`external_fence_features`] is a bitmask of [`ExternalFenceFeatureFlagBits`] indicating the
///   features of `handleType`.
///# Description
///If `handleType` is not supported by the implementation, then
///[`ExternalFenceProperties`]::[`external_fence_features`] will be set to
///zero.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalFenceProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    s_type: StructureType,
    ///No documentation found
    p_next: *const BaseOutStructure<'lt>,
    ///[`export_from_imported_handle_types`] is a bitmask of
    ///[`ExternalFenceHandleTypeFlagBits`] indicating which types of
    ///imported handle `handleType`**can** be exported from.
    export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    ///[`compatible_handle_types`] is a bitmask of
    ///[`ExternalFenceHandleTypeFlagBits`] specifying handle types which
    ///**can** be specified at the same time as `handleType` when creating a
    ///fence.
    compatible_handle_types: ExternalFenceHandleTypeFlags,
    ///[`external_fence_features`] is a bitmask of
    ///[`ExternalFenceFeatureFlagBits`] indicating the features of
    ///`handleType`.
    external_fence_features: ExternalFenceFeatureFlags,
}
///[VkExportFenceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExportFenceCreateInfo.html) - Structure specifying handle types that can be exported from a fence
///# C Specifications
///To create a fence whose payload **can** be exported to external handles, add a
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
///   fence handle types the application **can** export from the resulting fence. The application
///   **can** request multiple handle types for the same fence.
///# Description
///Valid Usage
/// - The bits in [`handle_types`]**must** be supported and compatible, as reported by
///   [`ExternalFenceProperties`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO`
/// - [`handle_types`]**must** be a valid combination of [`ExternalFenceHandleTypeFlagBits`] values
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExportFenceCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`handle_types`] is a bitmask of
    ///[`ExternalFenceHandleTypeFlagBits`] specifying one or more fence
    ///handle types the application **can** export from the resulting fence.
    ///The application **can** request multiple handle types for the same fence.
    handle_types: ExternalFenceHandleTypeFlags,
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
///   render pass. If this feature is not enabled, the view mask of each subpass **must** always be
///   zero.
/// - [`multiview_geometry_shader`] specifies whether the implementation supports multiview rendering within a render pass, with [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#geometry). If this feature is not enabled, then a pipeline compiled against a subpass with a non-zero view mask **must** not include a geometry shader.
/// - [`multiview_tessellation_shader`] specifies whether the implementation supports multiview rendering within a render pass, with [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation). If this feature is not enabled, then a pipeline compiled against a subpass with a non-zero view mask **must** not include any tessellation shaders.
///If the [`PhysicalDeviceMultiviewFeatures`] structure is included in the [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceMultiviewFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage
/// - If [`multiview_geometry_shader`] is enabled then [`multiview`]**must** also be enabled
/// - If [`multiview_tessellation_shader`] is enabled then [`multiview`]**must** also be enabled
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`multiview`] specifies whether
    ///the implementation supports multiview rendering within a render pass.
    ///If this feature is not enabled, the view mask of each subpass **must**
    ///always be zero.
    multiview: Bool32,
    ///[`multiview_geometry_shader`]
    ///specifies whether the implementation supports multiview rendering within
    ///a render pass, with [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#geometry).
    ///If this feature is not enabled, then a pipeline compiled against a
    ///subpass with a non-zero view mask **must** not include a geometry shader.
    multiview_geometry_shader: Bool32,
    ///[`multiview_tessellation_shader`] specifies whether the implementation
    ///supports multiview rendering within a render pass, with
    ///[tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#tessellation).
    ///If this feature is not enabled, then a pipeline compiled against a
    ///subpass with a non-zero view mask **must** not include any tessellation
    ///shaders.
    multiview_tessellation_shader: Bool32,
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
/// - [`max_multiview_view_count`] is one greater than the maximum view index that **can** be used
///   in a subpass.
/// - [`max_multiview_instance_index`] is the maximum valid value of instance index allowed to be
///   generated by a drawing command recorded within a subpass of a multiview render pass instance.
///If the [`PhysicalDeviceMultiviewProperties`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    max_multiview_view_count: u32,
    ///No documentation found
    max_multiview_instance_index: u32,
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
/// - [`p_view_masks`] is a pointer to an array of [`subpass_count`] view masks, where each mask is
///   a bitfield of view indices describing which views rendering is broadcast to in each subpass,
///   when multiview is enabled. If [`subpass_count`] is zero, each view mask is treated as zero.
/// - [`dependency_count`] is zero or the number of dependencies in the render pass.
/// - [`p_view_offsets`] is a pointer to an array of [`dependency_count`] view offsets, one for each
///   dependency. If [`dependency_count`] is zero, each dependency’s view offset is treated as zero.
///   Each view offset controls which views in the source subpass the views in the destination
///   subpass depend on.
/// - [`correlation_mask_count`] is zero or the number of correlation masks.
/// - [`p_correlation_masks`] is a pointer to an array of [`correlation_mask_count`] view masks
///   indicating sets of views that **may** be more efficient to render concurrently.
///# Description
///When a subpass uses a non-zero view mask, *multiview* functionality is
///considered to be enabled.
///Multiview is all-or-nothing for a render pass - that is, either all
///subpasses **must** have a non-zero view mask (though some subpasses **may** have
///only one view) or all **must** be zero.
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
///broadcasting.Some implementations **may** not support multiview in conjunction with
///[geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview-gs) or
///[tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview-tess).When multiview is enabled, the `VK_DEPENDENCY_VIEW_LOCAL_BIT` bit in a
///dependency **can** be used to express a view-local dependency, meaning that
///each view in the destination subpass depends on a single view in the source
///subpass.
///Unlike pipeline barriers, a subpass dependency **can** potentially have a
///different view mask in the source subpass and the destination subpass.
///If the dependency is view-local, then each view (dstView) in the
///destination subpass depends on the view dstView +
///[`p_view_offsets`][dependency] in the source subpass.
///If there is not such a view in the source subpass, then this dependency does
///not affect that view in the destination subpass.
///If the dependency is not view-local, then all views in the destination
///subpass depend on all views in the source subpass, and the view offset is
///ignored.
///A non-zero view offset is not allowed in a self-dependency.The elements of
/// [`p_correlation_masks`] are a set of masks of views
///indicating that views in the same mask **may** exhibit spatial coherency
///between the views, making it more efficient to render them concurrently.
///Correlation masks **must** not have a functional effect on the results of the
///multiview rendering.When multiview is enabled, at the beginning of each subpass all non-render
///pass state is undefined.
///In particular, each time [`CmdBeginRenderPass`] or
///[`CmdNextSubpass`] is called the graphics pipeline **must** be bound, any
///relevant descriptor sets or vertex/index buffers **must** be bound, and any
///relevant dynamic state or push constants **must** be set before they are used.A multiview
/// subpass **can** declare that its shaders will write per-view
///attributes for all views in a single invocation, by setting the
///`VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` bit in the subpass
///description.
///The only supported per-view attributes are position and viewport mask, and
///per-view position and viewport masks are written to output array variables
///decorated with `PositionPerViewNV` and `ViewportMaskPerViewNV`,
///respectively.
///If `[`VK_NV_viewport_array2`]` is not supported and enabled,
///`ViewportMaskPerViewNV`**must** not be used.
///Values written to elements of `PositionPerViewNV` and
///`ViewportMaskPerViewNV`**must** not depend on the `ViewIndex`.
///The shader **must** also write to an output variable decorated with
///`Position`, and the value written to `Position`**must** equal the value
///written to `PositionPerViewNV`[`ViewIndex`].
///Similarly, if `ViewportMaskPerViewNV` is written to then the shader **must**
///also write to an output variable decorated with `ViewportMaskNV`, and the
///value written to `ViewportMaskNV`**must** equal the value written to
///`ViewportMaskPerViewNV`[`ViewIndex`].
///Implementations will either use values taken from `Position` and
///`ViewportMaskNV` and invoke the shader once for each view, or will use
///values taken from `PositionPerViewNV` and `ViewportMaskPerViewNV` and
///invoke the shader fewer times.
///The values written to `Position` and `ViewportMaskNV`**must** not depend
///on the values written to `PositionPerViewNV` and
///`ViewportMaskPerViewNV`, or vice versa (to allow compilers to eliminate
///the unused outputs).
///All attributes that do not have `*PerViewNV` counterparts **must** not depend
///on `ViewIndex`.Per-view attributes are all-or-nothing for a subpass.
///That is, all pipelines compiled against a subpass that includes the
///`VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX` bit **must** write
///per-view attributes to the `*PerViewNV[]` shader outputs, in addition to the
///non-per-view (e.g. `Position`) outputs.
///Pipelines compiled against a subpass that does not include this bit **must**
///not include the `*PerViewNV[]` outputs in their interfaces.Valid Usage
/// - Each view index **must** not be set in more than one element of [`p_correlation_masks`]
/// - If the [`multiview`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiview)
///   feature is not enabled, each element of [`p_view_masks`]**must** be `0`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO`
/// - If [`subpass_count`] is not `0`, [`p_view_masks`]**must** be a valid pointer to an array of
///   [`subpass_count`]`uint32_t` values
/// - If [`dependency_count`] is not `0`, [`p_view_offsets`]**must** be a valid pointer to an array
///   of [`dependency_count`]`int32_t` values
/// - If [`correlation_mask_count`] is not `0`, [`p_correlation_masks`]**must** be a valid pointer
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RenderPassMultiviewCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`subpass_count`] is zero or the number of subpasses in the render
    ///pass.
    subpass_count: u32,
    ///[`p_view_masks`] is a pointer to an array of [`subpass_count`] view
    ///masks, where each mask is a bitfield of view indices describing which
    ///views rendering is broadcast to in each subpass, when multiview is
    ///enabled.
    ///If [`subpass_count`] is zero, each view mask is treated as zero.
    p_view_masks: *mut u32,
    ///[`dependency_count`] is zero or the number of dependencies in the
    ///render pass.
    dependency_count: u32,
    ///[`p_view_offsets`] is a pointer to an array of [`dependency_count`]
    ///view offsets, one for each dependency.
    ///If [`dependency_count`] is zero, each dependency’s view offset is
    ///treated as zero.
    ///Each view offset controls which views in the source subpass the views in
    ///the destination subpass depend on.
    p_view_offsets: *mut i32,
    ///[`correlation_mask_count`] is zero or the number of correlation masks.
    correlation_mask_count: u32,
    ///[`p_correlation_masks`] is a pointer to an array of
    ///[`correlation_mask_count`] view masks indicating sets of views that **may**
    ///be more efficient to render concurrently.
    p_correlation_masks: *mut u32,
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
///   [`subset_allocation`]**must** be [`FALSE`].
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceGroupProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`physical_device_count`] is the number of physical devices in the
    ///group.
    physical_device_count: u32,
    ///[`physical_devices`] is an array of [`MAX_DEVICE_GROUP_SIZE`][`PhysicalDevice`] handles
    /// representing all physical devices in the group.
    ///The first [`physical_device_count`] elements of the array will be valid.
    physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE],
    ///[`subset_allocation`] specifies whether logical devices created from
    ///the group support allocating device memory on a subset of devices, via
    ///the `deviceMask` member of the [`MemoryAllocateFlagsInfo`].
    ///If this is [`FALSE`], then all device memory allocations are made
    ///across all physical devices in the group.
    ///If [`physical_device_count`] is `1`, then [`subset_allocation`]**must**
    ///be [`FALSE`].
    subset_allocation: Bool32,
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
///   **must** be allocated on each device in the mask, if `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is
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
/// allocations from a multi-instance heap **may** consume
///memory on all physical devices even if the [`device_mask`] excludes some
///devices.
///If [`PhysicalDeviceGroupProperties::subset_allocation`] is
///[`TRUE`], then memory is only consumed for the devices in the device
///mask.Valid Usage
/// - If `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is set, [`device_mask`]**must** be a valid device mask
/// - If `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is set, [`device_mask`]**must** not be zero
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO`
/// - [`flags`]**must** be a valid combination of [`MemoryAllocateFlagBits`] values
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MemoryAllocateFlagsInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`MemoryAllocateFlagBits`] controlling
    ///the allocation.
    flags: MemoryAllocateFlags,
    ///[`device_mask`] is a mask of physical devices in the logical device,
    ///indicating that memory **must** be allocated on each device in the mask, if
    ///`VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT` is set in [`flags`].
    device_mask: u32,
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
///Valid Usage
/// - [`buffer`]**must** not already be backed by a memory object
/// - [`buffer`]**must** not have been created with any sparse memory binding flags
/// - [`memory_offset`]**must** be less than the size of [`memory`]
/// - [`memory`]**must** have been allocated using one of the memory types allowed in the
///   `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetBufferMemoryRequirements`] with [`buffer`]
/// - [`memory_offset`]**must** be an integer multiple of the `alignment` member of the
///   [`MemoryRequirements`] structure returned from a call to [`GetBufferMemoryRequirements`] with
///   [`buffer`]
/// - The `size` member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetBufferMemoryRequirements`] with [`buffer`]**must** be less than or equal to the size of
///   [`memory`] minus [`memory_offset`]
/// - If [`buffer`] requires a dedicated allocation (as reported by [`GetBufferMemoryRequirements2`]
///   in [`MemoryDedicatedRequirements::requires_dedicated_allocation`] for [`buffer`]),
///   [`memory`]**must** have been allocated with [`MemoryDedicatedAllocateInfo`]::[`buffer`] equal
///   to [`buffer`]
/// - If the [`MemoryAllocateInfo`] provided when [`memory`] was allocated included a
///   [`MemoryDedicatedAllocateInfo`] structure in its [`p_next`] chain, and
///   [`MemoryDedicatedAllocateInfo`]::[`buffer`] was not [`crate::utils::Handle::null`], then
///   [`buffer`]**must** equal [`MemoryDedicatedAllocateInfo`]::[`buffer`], and
///   [`memory_offset`]**must** be zero
/// - If [`buffer`] was created with the `VK_BUFFER_CREATE_PROTECTED_BIT` bit set, the buffer
///   **must** be bound to a memory object allocated with a memory type that reports
///   `VK_MEMORY_PROPERTY_PROTECTED_BIT`
/// - If [`buffer`] was created with the `VK_BUFFER_CREATE_PROTECTED_BIT` bit not set, the buffer
///   **must** not be bound to a memory object allocated with a memory type that reports
///   `VK_MEMORY_PROPERTY_PROTECTED_BIT`
/// - If [`buffer`] was created with [`DedicatedAllocationBufferCreateInfoNV::dedicated_allocation`]
///   equal to [`TRUE`], [`memory`]**must** have been allocated with
///   [`DedicatedAllocationMemoryAllocateInfoNV`]::[`buffer`] equal to a buffer handle created with
///   identical creation parameters to [`buffer`] and [`memory_offset`]**must** be zero
/// - If the value of [`ExportMemoryAllocateInfo::handle_types`] used to allocate [`memory`] is not
///   `0`, it **must** include at least one of the handles set in
///   [`ExternalMemoryBufferCreateInfo::handle_types`] when [`buffer`] was created
/// - If [`memory`] was allocated by a memory import operation, that is not
///   [`ImportAndroidHardwareBufferInfoANDROID`] with a non-`NULL`[`buffer`] value, the external
///   handle type of the imported memory **must** also have been set in
///   [`ExternalMemoryBufferCreateInfo::handle_types`] when [`buffer`] was created
/// - If [`memory`] was allocated with the [`ImportAndroidHardwareBufferInfoANDROID`] memory import
///   operation with a non-`NULL`[`buffer`] value,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`**must** also have been
///   set in [`ExternalMemoryBufferCreateInfo::handle_types`] when [`buffer`] was created
/// - If the [`PhysicalDeviceBufferDeviceAddressFeatures::buffer_device_address`] feature is enabled
///   and [`buffer`] was created with the `VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT` bit set,
///   [`memory`]**must** have been allocated with the `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT` bit
///   set
/// - If [`buffer`] was created with [`BufferCollectionBufferCreateInfoFUCHSIA`] chained to
///   [`BufferCreateInfo`]::[`p_next`], [`memory`]**must** be allocated with a
///   [`ImportMemoryBufferCollectionFUCHSIA`] chained to [`MemoryAllocateInfo`]::[`p_next`]
/// - If the [`p_next`] chain includes a [`BindBufferMemoryDeviceGroupInfo`] structure, all
///   instances of [`memory`] specified by
///   [`BindBufferMemoryDeviceGroupInfo::p_device_indices`]**must** have been allocated
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`BindBufferMemoryDeviceGroupInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`buffer`]**must** be a valid [`Buffer`] handle
/// - [`memory`]**must** be a valid [`DeviceMemory`] handle
/// - Both of [`buffer`], and [`memory`]**must** have been created, allocated, or retrieved from the
///   same [`Device`]
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindBufferMemoryInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`buffer`] is the buffer to be attached to memory.
    buffer: Buffer,
    ///[`memory`] is a [`DeviceMemory`] object describing the device
    ///memory to attach.
    memory: DeviceMemory,
    ///[`memory_offset`] is the start offset of the region of [`memory`]
    ///which is to be bound to the buffer.
    ///The number of bytes returned in the
    ///[`MemoryRequirements`]::`size` member in [`memory`], starting
    ///from [`memory_offset`] bytes, will be bound to the specified buffer.
    memory_offset: DeviceSize,
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
/// - [`device_index_count`] is the number of elements in [`p_device_indices`].
/// - [`p_device_indices`] is a pointer to an array of device indices.
///# Description
///If the [`p_next`] chain of [`BindBufferMemoryInfo`] includes a
///[`BindBufferMemoryDeviceGroupInfo`] structure, then that structure
///determines how memory is bound to buffers across multiple devices in a
///device group.If [`device_index_count`] is greater than zero, then on device index i
///the buffer is attached to the instance of `memory` on the physical
///device with device index [`p_device_indices`][i].If [`device_index_count`] is zero and `memory`
/// comes from a memory heap
///with the `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as if
///[`p_device_indices`] contains consecutive indices from zero to the number of
///physical devices in the logical device, minus one.
///In other words, by default each physical device attaches to its own instance
///of `memory`.If [`device_index_count`] is zero and `memory` comes from a memory heap
///without the `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as
///if [`p_device_indices`] contains an array of zeros.
///In other words, by default each physical device attaches to instance zero.Valid Usage
/// - [`device_index_count`]**must** either be zero or equal to the number of physical devices in
///   the logical device
/// - All elements of [`p_device_indices`]**must** be valid device indices
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO`
/// - If [`device_index_count`] is not `0`, [`p_device_indices`]**must** be a valid pointer to an
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindBufferMemoryDeviceGroupInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`device_index_count`] is the number of elements in
    ///[`p_device_indices`].
    device_index_count: u32,
    ///[`p_device_indices`] is a pointer to an array of device indices.
    p_device_indices: *mut u32,
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
///Valid Usage
/// - [`image`]**must** not already be backed by a memory object
/// - [`image`]**must** not have been created with any sparse memory binding flags
/// - [`memory_offset`]**must** be less than the size of [`memory`]
/// - If [`image`] requires a dedicated allocation (as reported by [`GetImageMemoryRequirements2`]
///   in [`MemoryDedicatedRequirements::requires_dedicated_allocation`] for [`image`]),
///   [`memory`]**must** have been created with [`MemoryDedicatedAllocateInfo`]::[`image`] equal to
///   [`image`]
/// - If the [dedicated allocation image aliasing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-dedicatedAllocationImageAliasing)
///   feature is not enabled, and the [`MemoryAllocateInfo`] provided when [`memory`] was allocated
///   included a [`MemoryDedicatedAllocateInfo`] structure in its [`p_next`] chain, and
///   [`MemoryDedicatedAllocateInfo`]::[`image`] was not [`crate::utils::Handle::null`], then
///   [`image`]**must** equal [`MemoryDedicatedAllocateInfo`]::[`image`] and
///   [`memory_offset`]**must** be zero
/// - If the [dedicated allocation image aliasing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-dedicatedAllocationImageAliasing)
///   feature is enabled, and the [`MemoryAllocateInfo`] provided when [`memory`] was allocated
///   included a [`MemoryDedicatedAllocateInfo`] structure in its [`p_next`] chain, and
///   [`MemoryDedicatedAllocateInfo`]::[`image`] was not [`crate::utils::Handle::null`], then
///   [`memory_offset`]**must** be zero, and [`image`]**must** be either equal to
///   [`MemoryDedicatedAllocateInfo`]::[`image`] or an image that was created using the same
///   parameters in [`ImageCreateInfo`], with the exception that `extent` and `arrayLayers`**may**
///   differ subject to the following restrictions: every dimension in the `extent` parameter of the
///   image being bound **must** be equal to or smaller than the original image for which the
///   allocation was created; and the `arrayLayers` parameter of the image being bound **must** be
///   equal to or smaller than the original image for which the allocation was created
/// - If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit set, the image **must** be
///   bound to a memory object allocated with a memory type that reports
///   `VK_MEMORY_PROPERTY_PROTECTED_BIT`
/// - If image was created with the `VK_IMAGE_CREATE_PROTECTED_BIT` bit not set, the image **must**
///   not be bound to a memory object created with a memory type that reports
///   `VK_MEMORY_PROPERTY_PROTECTED_BIT`
/// - If [`image`] was created with [`DedicatedAllocationImageCreateInfoNV::dedicated_allocation`]
///   equal to [`TRUE`], [`memory`]**must** have been created with
///   [`DedicatedAllocationMemoryAllocateInfoNV`]::[`image`] equal to an image handle created with
///   identical creation parameters to [`image`] and [`memory_offset`]**must** be zero
/// - If the value of [`ExportMemoryAllocateInfo::handle_types`] used to allocate [`memory`] is not
///   `0`, it **must** include at least one of the handles set in
///   [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
/// - If [`memory`] was created by a memory import operation, that is not
///   [`ImportAndroidHardwareBufferInfoANDROID`] with a non-`NULL``buffer` value, the external
///   handle type of the imported memory **must** also have been set in
///   [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
/// - If [`memory`] was created with the [`ImportAndroidHardwareBufferInfoANDROID`] memory import
///   operation with a non-`NULL``buffer` value,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`**must** also have been
///   set in [`ExternalMemoryImageCreateInfo::handle_types`] when [`image`] was created
/// - If the [`p_next`] chain does not include a [`BindImagePlaneMemoryInfo`] structure,
///   [`memory`]**must** have been allocated using one of the memory types allowed in the
///   `memoryTypeBits` member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetImageMemoryRequirements2`] with [`image`]
/// - If the [`p_next`] chain does not include a [`BindImagePlaneMemoryInfo`] structure,
///   [`memory_offset`]**must** be an integer multiple of the `alignment` member of the
///   [`MemoryRequirements`] structure returned from a call to [`GetImageMemoryRequirements2`] with
///   [`image`]
/// - If the [`p_next`] chain does not include a [`BindImagePlaneMemoryInfo`] structure, the
///   difference of the size of [`memory`] and [`memory_offset`]**must** be greater than or equal to
///   the `size` member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetImageMemoryRequirements2`] with the same [`image`]
/// - If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, [`image`]**must**
///   have been created with the `VK_IMAGE_CREATE_DISJOINT_BIT` bit set
/// - If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, [`memory`]**must**
///   have been allocated using one of the memory types allowed in the `memoryTypeBits` member of
///   the [`MemoryRequirements`] structure returned from a call to [`GetImageMemoryRequirements2`]
///   with [`image`] and where [`BindImagePlaneMemoryInfo::plane_aspect`] corresponds to the
///   [`ImagePlaneMemoryRequirementsInfo::plane_aspect`] in the [`ImageMemoryRequirementsInfo2`]
///   structure’s [`p_next`] chain
/// - If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure,
///   [`memory_offset`]**must** be an integer multiple of the `alignment` member of the
///   [`MemoryRequirements`] structure returned from a call to [`GetImageMemoryRequirements2`] with
///   [`image`] and where [`BindImagePlaneMemoryInfo::plane_aspect`] corresponds to the
///   [`ImagePlaneMemoryRequirementsInfo::plane_aspect`] in the [`ImageMemoryRequirementsInfo2`]
///   structure’s [`p_next`] chain
/// - If the [`p_next`] chain includes a [`BindImagePlaneMemoryInfo`] structure, the difference of
///   the size of [`memory`] and [`memory_offset`]**must** be greater than or equal to the `size`
///   member of the [`MemoryRequirements`] structure returned from a call to
///   [`GetImageMemoryRequirements2`] with the same [`image`] and where
///   [`BindImagePlaneMemoryInfo::plane_aspect`] corresponds to the
///   [`ImagePlaneMemoryRequirementsInfo::plane_aspect`] in the [`ImageMemoryRequirementsInfo2`]
///   structure’s [`p_next`] chain
/// - If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, all instances
///   of [`memory`] specified by [`BindImageMemoryDeviceGroupInfo::p_device_indices`]**must** have
///   been allocated
/// - If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, and
///   [`BindImageMemoryDeviceGroupInfo::split_instance_bind_region_count`] is not zero, then
///   [`image`]**must** have been created with the `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT`
///   bit set
/// - If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, all elements
///   of [`BindImageMemoryDeviceGroupInfo::p_split_instance_bind_regions`]**must** be valid
///   rectangles contained within the dimensions of [`image`]
/// - If the [`p_next`] chain includes a [`BindImageMemoryDeviceGroupInfo`] structure, the union of
///   the areas of all elements of [`BindImageMemoryDeviceGroupInfo::p_split_instance_bind_regions`]
///   that correspond to the same instance of [`image`]**must** cover the entire image
/// - If [`image`] was created with a valid swapchain handle in
///   [`ImageSwapchainCreateInfoKHR::swapchain`], then the [`p_next`] chain **must** include a
///   [`BindImageMemorySwapchainInfoKHR`] structure containing the same swapchain handle
/// - If the [`p_next`] chain includes a [`BindImageMemorySwapchainInfoKHR`] structure,
///   [`memory`]**must** be [`crate::utils::Handle::null`]
/// - If the [`p_next`] chain does not include a [`BindImageMemorySwapchainInfoKHR`] structure,
///   [`memory`]**must** be a valid [`DeviceMemory`] handle
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`BindImageMemoryDeviceGroupInfo`],
///   [`BindImageMemorySwapchainInfoKHR`], or [`BindImagePlaneMemoryInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`image`]**must** be a valid [`Image`] handle
/// - Both of [`image`], and [`memory`] that are valid handles of non-ignored parameters **must**
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindImageMemoryInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`image`] is the image to be attached to memory.
    image: Image,
    ///[`memory`] is a [`DeviceMemory`] object describing the device
    ///memory to attach.
    memory: DeviceMemory,
    ///[`memory_offset`] is the start offset of the region of [`memory`]
    ///which is to be bound to the image.
    ///The number of bytes returned in the
    ///[`MemoryRequirements`]::`size` member in [`memory`], starting
    ///from [`memory_offset`] bytes, will be bound to the specified image.
    memory_offset: DeviceSize,
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
/// - [`device_index_count`] is the number of elements in [`p_device_indices`].
/// - [`p_device_indices`] is a pointer to an array of device indices.
/// - [`split_instance_bind_region_count`] is the number of elements in
///   [`p_split_instance_bind_regions`].
/// - [`p_split_instance_bind_regions`] is a pointer to an array of [`Rect2D`] structures describing
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
///[`p_split_instance_bind_regions`] is a pointer to an array of N<sup>2</sup>
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
///[`p_device_indices`] contains consecutive indices from zero to the number of
///physical devices in the logical device, minus one.
///In other words, by default each physical device attaches to its own instance
///of the memory.If [`split_instance_bind_region_count`] and [`device_index_count`] are zero
///and the memory comes from a memory heap without the
///`VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` bit set, then it is as if
///[`p_device_indices`] contains an array of zeros.
///In other words, by default each physical device attaches to instance zero.Valid Usage
/// - At least one of [`device_index_count`] and [`split_instance_bind_region_count`]**must** be
///   zero
/// - [`device_index_count`]**must** either be zero or equal to the number of physical devices in
///   the logical device
/// - All elements of [`p_device_indices`]**must** be valid device indices
/// - [`split_instance_bind_region_count`]**must** either be zero or equal to the number of physical
///   devices in the logical device squared
/// - Elements of [`p_split_instance_bind_regions`] that correspond to the same instance of an image
///   **must** not overlap
/// - The `offset.x` member of any element of [`p_split_instance_bind_regions`]**must** be a
///   multiple of the sparse image block width
///   ([`SparseImageFormatProperties`]::`imageGranularity.width`) of all non-metadata aspects of the
///   image
/// - The `offset.y` member of any element of [`p_split_instance_bind_regions`]**must** be a
///   multiple of the sparse image block height
///   ([`SparseImageFormatProperties`]::`imageGranularity.height`) of all non-metadata aspects of
///   the image
/// - The `extent.width` member of any element of [`p_split_instance_bind_regions`]**must** either
///   be a multiple of the sparse image block width of all non-metadata aspects of the image, or
///   else `extent.width` +  `offset.x`**must** equal the width of the image subresource
/// - The `extent.height` member of any element of [`p_split_instance_bind_regions`]**must** either
///   be a multiple of the sparse image block height of all non-metadata aspects of the image, or
///   else `extent.height` +  `offset.y`**must** equal the height of the image subresource
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO`
/// - If [`device_index_count`] is not `0`, [`p_device_indices`]**must** be a valid pointer to an
///   array of [`device_index_count`]`uint32_t` values
/// - If [`split_instance_bind_region_count`] is not `0`, [`p_split_instance_bind_regions`]**must**
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindImageMemoryDeviceGroupInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`device_index_count`] is the number of elements in
    ///[`p_device_indices`].
    device_index_count: u32,
    ///[`p_device_indices`] is a pointer to an array of device indices.
    p_device_indices: *mut u32,
    ///[`split_instance_bind_region_count`] is the number of elements in
    ///[`p_split_instance_bind_regions`].
    split_instance_bind_region_count: u32,
    ///[`p_split_instance_bind_regions`] is a pointer to an array of
    ///[`Rect2D`] structures describing which regions of the image are
    ///attached to each instance of memory.
    p_split_instance_bind_regions: *mut Rect2D,
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
/// - [`device_render_area_count`] is the number of elements in the [`p_device_render_areas`] array.
/// - [`p_device_render_areas`] is a pointer to an array of [`Rect2D`] structures defining the
///   render area for each physical device.
///# Description
///The [`device_mask`] serves several purposes.
///It is an upper bound on the set of physical devices that **can** be used during
///the render pass instance, and the initial device mask when the render pass
///instance begins.
///In addition, commands transitioning to the next subpass in a render pass
///instance and commands ending the render pass instance, and, accordingly
///render pass attachment load, store, and resolve operations and subpass
///dependencies corresponding to the render pass instance, are executed on the
///physical devices included in the device mask provided here.If [`device_render_area_count`] is
/// not zero, then the elements of
///[`p_device_render_areas`] override the value of
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
///devices.Valid Usage
/// - [`device_mask`]**must** be a valid device mask value
/// - [`device_mask`]**must** not be zero
/// - [`device_mask`]**must** be a subset of the command buffer’s initial device mask
/// - [`device_render_area_count`]**must** either be zero or equal to the number of physical devices
///   in the logical device
/// - The `offset.x` member of any element of [`p_device_render_areas`]**must** be greater than or
///   equal to 0
/// - The `offset.y` member of any element of [`p_device_render_areas`]**must** be greater than or
///   equal to 0
/// -    The sum of the `offset.x` and `extent.width` members of any element of [`p_device_render_areas`]**must** be less than or equal to [`maxFramebufferWidth`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFramebufferWidth)
/// -    The sum of the `offset.y` and `extent.height` members of any element of [`p_device_render_areas`]**must** be less than or equal to [`maxFramebufferHeight`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxFramebufferHeight)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO`
/// - If [`device_render_area_count`] is not `0`, [`p_device_render_areas`]**must** be a valid
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceGroupRenderPassBeginInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`device_mask`] is the device mask for the render pass instance.
    device_mask: u32,
    ///[`device_render_area_count`] is the number of elements in the
    ///[`p_device_render_areas`] array.
    device_render_area_count: u32,
    ///[`p_device_render_areas`] is a pointer to an array of [`Rect2D`]
    ///structures defining the render area for each physical device.
    p_device_render_areas: *mut Rect2D,
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
///that **can** ever be in the device mask in the command buffer.If this structure is not present,
/// the initial value of a command buffer’s
///device mask is set to include all physical devices in the logical device
///when the command buffer begins recording.Valid Usage
/// - [`device_mask`]**must** be a valid device mask value
/// - [`device_mask`]**must** not be zero
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceGroupCommandBufferBeginInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`device_mask`] is the initial value of the command buffer’s device
    ///mask.
    device_mask: u32,
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
/// - [`wait_semaphore_count`] is the number of elements in the [`p_wait_semaphore_device_indices`]
///   array.
/// - [`p_wait_semaphore_device_indices`] is a pointer to an array of [`wait_semaphore_count`]
///   device indices indicating which physical device executes the semaphore wait operation in the
///   corresponding element of [`SubmitInfo::p_wait_semaphores`].
/// - [`command_buffer_count`] is the number of elements in the [`p_command_buffer_device_masks`]
///   array.
/// - [`p_command_buffer_device_masks`] is a pointer to an array of [`command_buffer_count`] device
///   masks indicating which physical devices execute the command buffer in the corresponding
///   element of [`SubmitInfo::p_command_buffers`]. A physical device executes the command buffer if
///   the corresponding bit is set in the mask.
/// - [`signal_semaphore_count`] is the number of elements in the
///   [`p_signal_semaphore_device_indices`] array.
/// - [`p_signal_semaphore_device_indices`] is a pointer to an array of [`signal_semaphore_count`]
///   device indices indicating which physical device executes the semaphore signal operation in the
///   corresponding element of [`SubmitInfo::p_signal_semaphores`].
///# Description
///If this structure is not present, semaphore operations and command buffers
///execute on device index zero.Valid Usage
/// - [`wait_semaphore_count`]**must** equal [`SubmitInfo`]::[`wait_semaphore_count`]
/// - [`command_buffer_count`]**must** equal [`SubmitInfo`]::[`command_buffer_count`]
/// - [`signal_semaphore_count`]**must** equal [`SubmitInfo`]::[`signal_semaphore_count`]
/// - All elements of [`p_wait_semaphore_device_indices`] and
///   [`p_signal_semaphore_device_indices`]**must** be valid device indices
/// - All elements of [`p_command_buffer_device_masks`]**must** be valid device masks
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO`
/// - If [`wait_semaphore_count`] is not `0`, [`p_wait_semaphore_device_indices`]**must** be a valid
///   pointer to an array of [`wait_semaphore_count`]`uint32_t` values
/// - If [`command_buffer_count`] is not `0`, [`p_command_buffer_device_masks`]**must** be a valid
///   pointer to an array of [`command_buffer_count`]`uint32_t` values
/// - If [`signal_semaphore_count`] is not `0`, [`p_signal_semaphore_device_indices`]**must** be a
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceGroupSubmitInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`wait_semaphore_count`] is the number of elements in the
    ///[`p_wait_semaphore_device_indices`] array.
    wait_semaphore_count: u32,
    ///[`p_wait_semaphore_device_indices`] is a pointer to an array of
    ///[`wait_semaphore_count`] device indices indicating which physical device
    ///executes the semaphore wait operation in the corresponding element of
    ///[`SubmitInfo`]::`pWaitSemaphores`.
    p_wait_semaphore_device_indices: *mut u32,
    ///[`command_buffer_count`] is the number of elements in the
    ///[`p_command_buffer_device_masks`] array.
    command_buffer_count: u32,
    ///[`p_command_buffer_device_masks`] is a pointer to an array of
    ///[`command_buffer_count`] device masks indicating which physical devices
    ///execute the command buffer in the corresponding element of
    ///[`SubmitInfo`]::`pCommandBuffers`.
    ///A physical device executes the command buffer if the corresponding bit
    ///is set in the mask.
    p_command_buffer_device_masks: *mut u32,
    ///[`signal_semaphore_count`] is the number of elements in the
    ///[`p_signal_semaphore_device_indices`] array.
    signal_semaphore_count: u32,
    ///[`p_signal_semaphore_device_indices`] is a pointer to an array of
    ///[`signal_semaphore_count`] device indices indicating which physical
    ///device executes the semaphore signal operation in the corresponding
    ///element of [`SubmitInfo`]::`pSignalSemaphores`.
    p_signal_semaphore_device_indices: *mut u32,
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
///[`memory_device_index`] are assumed to be zero.Valid Usage
/// - [`resource_device_index`] and [`memory_device_index`]**must** both be valid device indices
/// - Each memory allocation bound in this batch **must** have allocated an instance for
///   [`memory_device_index`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceGroupBindSparseInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`resource_device_index`] is a device index indicating which instance of
    ///the resource is bound.
    resource_device_index: u32,
    ///[`memory_device_index`] is a device index indicating which instance of
    ///the memory the resource instance is bound to.
    memory_device_index: u32,
}
///[VkDeviceGroupDeviceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupDeviceCreateInfo.html) - Create a logical device from multiple physical devices
///# C Specifications
///A logical device **can** be created that connects to one or more physical
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
/// - [`physical_device_count`] is the number of elements in the [`p_physical_devices`] array.
/// - [`p_physical_devices`] is a pointer to an array of physical device handles belonging to the
///   same device group.
///# Description
///The elements of the [`p_physical_devices`] array are an ordered list of the
///physical devices that the logical device represents.
///These **must** be a subset of a single device group, and need not be in the
///same order as they were enumerated.
///The order of the physical devices in the [`p_physical_devices`] array
///determines the *device index* of each physical device, with element i
///being assigned a device index of i.
///Certain commands and structures refer to one or more physical devices by
///using device indices or *device masks* formed using device indices.A logical device created
/// without using [`DeviceGroupDeviceCreateInfo`],
///or with [`physical_device_count`] equal to zero, is equivalent to a
///[`physical_device_count`] of one and [`p_physical_devices`] pointing to the
///`physicalDevice` parameter to [`CreateDevice`].
///In particular, the device index of that physical device is zero.Valid Usage
/// - Each element of [`p_physical_devices`]**must** be unique
/// - All elements of [`p_physical_devices`]**must** be in the same device group as enumerated by
///   [`EnumeratePhysicalDeviceGroups`]
/// - If [`physical_device_count`] is not `0`, the `physicalDevice` parameter of
///   [`CreateDevice`]**must** be an element of [`p_physical_devices`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO`
/// - If [`physical_device_count`] is not `0`, [`p_physical_devices`]**must** be a valid pointer to
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceGroupDeviceCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`physical_device_count`] is the number of elements in the
    ///[`p_physical_devices`] array.
    physical_device_count: u32,
    ///[`p_physical_devices`] is a pointer to an array of physical device
    ///handles belonging to the same device group.
    p_physical_devices: *mut PhysicalDevice,
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
///Valid Usage
/// - [`dst_binding`]**must** be a valid binding in the descriptor set layout implicitly specified
///   when using a descriptor update template to update descriptors
/// -  [`dst_array_element`] and [`descriptor_count`]**must** be less than or equal to the number of array elements in the descriptor set binding implicitly specified when using a descriptor update template to update descriptors, and all applicable consecutive bindings, as described by [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-updates-consecutive)
/// - If `descriptor` type is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`,
///   [`dst_array_element`]**must** be an integer multiple of `4`
/// - If `descriptor` type is `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`,
///   [`descriptor_count`]**must** be an integer multiple of `4`
///Valid Usage (Implicit)
/// - [`descriptor_type`]**must** be a valid [`DescriptorType`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DescriptorUpdateTemplateEntry {
    ///[`dst_binding`] is the descriptor binding to update when using this
    ///descriptor update template.
    dst_binding: u32,
    ///[`dst_array_element`] is the starting element in the array belonging to
    ///[`dst_binding`].
    ///If the descriptor binding identified by [`dst_binding`] has a
    ///descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
    ///[`dst_array_element`] specifies the starting byte offset to update.
    dst_array_element: u32,
    ///[`descriptor_count`] is the number of descriptors to update.
    ///If [`descriptor_count`] is greater than the number of remaining array
    ///elements in the destination binding, those affect consecutive bindings
    ///in a manner similar to [`WriteDescriptorSet`] above.
    ///If the descriptor binding identified by [`dst_binding`] has a
    ///descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
    ///[`descriptor_count`] specifies the number of bytes to update and the
    ///remaining array elements in the destination binding refer to the
    ///remaining number of bytes in it.
    descriptor_count: u32,
    ///[`descriptor_type`] is a [`DescriptorType`] specifying the type of
    ///the descriptor.
    descriptor_type: DescriptorType,
    ///[`offset`] is the offset in bytes of the first binding in the raw data
    ///structure.
    offset: usize,
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
    stride: usize,
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
///   [`p_descriptor_update_entries`] array.
/// - [`p_descriptor_update_entries`] is a pointer to an array of [`DescriptorUpdateTemplateEntry`]
///   structures describing the descriptors to be updated by the descriptor update template.
/// - [`template_type`] Specifies the type of the descriptor update template. If set to
///   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET` it **can** only be used to update
///   descriptor sets with a fixed [`descriptor_set_layout`]. If set to
///   `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR` it **can** only be used to push
///   descriptor sets using the provided [`pipeline_bind_point`], [`pipeline_layout`], and [`set`]
///   number.
/// - [`descriptor_set_layout`] is the descriptor set layout used to build the descriptor update
///   template. All descriptor sets which are going to be updated through the newly created
///   descriptor update template **must** be created with a layout that matches (is the same as, or
///   defined identically to) this layout. This parameter is ignored if [`template_type`] is not
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
///Valid Usage
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`,
///   [`descriptor_set_layout`]**must** be a valid [`DescriptorSetLayout`] handle
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`,
///   [`pipeline_bind_point`]**must** be a valid [`PipelineBindPoint`] value
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`,
///   [`pipeline_layout`]**must** be a valid [`PipelineLayout`] handle
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`,
///   [`set`]**must** be the unique set number in the pipeline layout that uses a descriptor set
///   layout that was created with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR`
/// - If [`template_type`] is `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`,
///   [`descriptor_set_layout`]**must** not contain a binding with type
///   `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be `0`
/// - [`p_descriptor_update_entries`]**must** be a valid pointer to an array of
///   [`descriptor_update_entry_count`] valid [`DescriptorUpdateTemplateEntry`] structures
/// - [`template_type`]**must** be a valid [`DescriptorUpdateTemplateType`] value
/// - [`descriptor_update_entry_count`]**must** be greater than `0`
/// - Both of [`descriptor_set_layout`], and [`pipeline_layout`] that are valid handles of
///   non-ignored parameters **must** have been created, allocated, or retrieved from the same
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DescriptorUpdateTemplateCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: DescriptorUpdateTemplateCreateFlags,
    ///[`descriptor_update_entry_count`] is the number of elements in the
    ///[`p_descriptor_update_entries`] array.
    descriptor_update_entry_count: u32,
    ///[`p_descriptor_update_entries`] is a pointer to an array of
    ///[`DescriptorUpdateTemplateEntry`] structures describing the
    ///descriptors to be updated by the descriptor update template.
    p_descriptor_update_entries: *mut DescriptorUpdateTemplateEntry,
    ///[`template_type`] Specifies the type of the descriptor update template.
    ///If set to `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET` it
    ///**can** only be used to update descriptor sets with a fixed
    ///[`descriptor_set_layout`].
    ///If set to `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
    ///it **can** only be used to push descriptor sets using the provided
    ///[`pipeline_bind_point`], [`pipeline_layout`], and [`set`] number.
    template_type: DescriptorUpdateTemplateType,
    ///[`descriptor_set_layout`] is the descriptor set layout used to build the
    ///descriptor update template.
    ///All descriptor sets which are going to be updated through the newly
    ///created descriptor update template **must** be created with a layout that
    ///matches (is the same as, or defined identically to) this layout.
    ///This parameter is ignored if [`template_type`] is not
    ///`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`.
    descriptor_set_layout: DescriptorSetLayout,
    ///[`pipeline_bind_point`] is a [`PipelineBindPoint`] indicating the
    ///type of the pipeline that will use the descriptors.
    ///This parameter is ignored if [`template_type`] is not
    ///`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
    pipeline_bind_point: PipelineBindPoint,
    ///[`pipeline_layout`] is a [`PipelineLayout`] object used to program
    ///the bindings.
    ///This parameter is ignored if [`template_type`] is not
    ///`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
    pipeline_layout: PipelineLayout,
    ///[`set`] is the set number of the descriptor set in the pipeline layout
    ///that will be updated.
    ///This parameter is ignored if [`template_type`] is not
    ///`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
    set: u32,
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
/// - [`aspect_mask`] is a mask of which aspect(s) **can** be accessed within the specified subpass.
///# Description
///This structure specifies an aspect mask for a specific input attachment of a
///specific subpass in the render pass.[`subpass`] and [`input_attachment_index`] index into the
/// render pass as:
///```c
///pCreateInfo->pSubpasses[subpass].pInputAttachments[inputAttachmentIndex]
///```
///Valid Usage
/// - [`aspect_mask`]**must** not include `VK_IMAGE_ASPECT_METADATA_BIT`
/// - [`aspect_mask`]**must** not include `VK_IMAGE_ASPECT_MEMORY_PLANE*_i_*BIT_EXT` for any index
///   *i*
///Valid Usage (Implicit)
/// - [`aspect_mask`]**must** be a valid combination of [`ImageAspectFlagBits`] values
/// - [`aspect_mask`]**must** not be `0`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct InputAttachmentAspectReference {
    ///[`subpass`] is an index into the `pSubpasses` array of the parent
    ///[`RenderPassCreateInfo`] structure.
    subpass: u32,
    ///[`input_attachment_index`] is an index into the `pInputAttachments`
    ///of the specified subpass.
    input_attachment_index: u32,
    ///[`aspect_mask`] is a mask of which aspect(s) **can** be accessed within
    ///the specified subpass.
    aspect_mask: ImageAspectFlags,
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
/// - [`aspect_reference_count`] is the number of elements in the [`p_aspect_references`] array.
/// - [`p_aspect_references`] is a pointer to an array of
///   [`aspect_reference_count`][`InputAttachmentAspectReference`] structures containing a mask
///   describing which aspect(s) **can** be accessed for a given input attachment within a given
///   subpass.
///# Description
///To specify which aspects of an input attachment **can** be read, add a
///[`RenderPassInputAttachmentAspectCreateInfo`] structure to the
///[`p_next`] chain of the [`RenderPassCreateInfo`] structure:An application **can** access any
/// aspect of an input attachment that does not
///have a specified aspect mask in the [`p_aspect_references`] array.
///Otherwise, an application **must** not access aspect(s) of an input attachment
///other than those in its specified aspect mask.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO`
/// - [`p_aspect_references`]**must** be a valid pointer to an array of [`aspect_reference_count`]
///   valid [`InputAttachmentAspectReference`] structures
/// - [`aspect_reference_count`]**must** be greater than `0`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RenderPassInputAttachmentAspectCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`aspect_reference_count`] is the number of elements in the
    ///[`p_aspect_references`] array.
    aspect_reference_count: u32,
    ///[`p_aspect_references`] is a pointer to an array of
    ///[`aspect_reference_count`][`InputAttachmentAspectReference`]
    ///structures containing a mask describing which aspect(s) **can** be accessed
    ///for a given input attachment within a given subpass.
    p_aspect_references: *mut InputAttachmentAspectReference,
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
///If the [`PhysicalDevice16BitStorageFeatures`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDevice16BitStorageFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures<'lt> {
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
///[`subgroup_size`]**must** be greater than or equal to 4.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    subgroup_size: u32,
    ///No documentation found
    supported_stages: ShaderStageFlags,
    ///No documentation found
    supported_operations: SubgroupFeatureFlags,
    ///No documentation found
    quad_operations_in_all_stages: Bool32,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`buffer`]**must** be a valid [`Buffer`] handle
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BufferMemoryRequirementsInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`buffer`] is the buffer to query.
    buffer: Buffer,
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
///Valid Usage
/// - If [`image`] was created with a *multi-planar* format and the `VK_IMAGE_CREATE_DISJOINT_BIT`
///   flag, there **must** be a [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`]
///   chain of the [`ImageMemoryRequirementsInfo2`] structure
/// - If [`image`] was created with `VK_IMAGE_CREATE_DISJOINT_BIT` and with
///   `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then there **must** be a
///   [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the
///   [`ImageMemoryRequirementsInfo2`] structure
/// - If [`image`] was not created with the `VK_IMAGE_CREATE_DISJOINT_BIT` flag, there **must** not
///   be a [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the
///   [`ImageMemoryRequirementsInfo2`] structure
/// - If [`image`] was created with a single-plane format and with any `tiling` other than
///   `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then there **must** not be a
///   [`ImagePlaneMemoryRequirementsInfo`] included in the [`p_next`] chain of the
///   [`ImageMemoryRequirementsInfo2`] structure
/// - If [`image`] was created with the
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` external memory handle
///   type, then [`image`]**must** be bound to memory
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`ImagePlaneMemoryRequirementsInfo`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`image`]**must** be a valid [`Image`] handle
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageMemoryRequirementsInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`image`] is the image to query.
    image: Image,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`image`]**must** be a valid [`Image`] handle
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageSparseMemoryRequirementsInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`image`] is the image to query.
    image: Image,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`MemoryDedicatedRequirements`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MemoryRequirements2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`memory_requirements`] is a [`MemoryRequirements`] structure
    ///describing the memory requirements of the resource.
    memory_requirements: MemoryRequirements,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2`
/// - [`p_next`]**must** be `NULL`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SparseImageMemoryRequirements2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`memory_requirements`] is a [`SparseImageMemoryRequirements`]
    ///structure describing the memory requirements of the sparse image.
    memory_requirements: SparseImageMemoryRequirements,
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
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDevicePointClippingProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    point_clipping_behavior: PointClippingBehavior,
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
///   **may** get better performance if a dedicated allocation is used.
/// - [`requires_dedicated_allocation`] specifies that a dedicated allocation is required for this
///   resource.
///# Description
///To determine the dedicated allocation requirements of a buffer or image
///resource, add a [`MemoryDedicatedRequirements`] structure to the
///[`p_next`] chain of the [`MemoryRequirements2`] structure passed as the
///`pMemoryRequirements` parameter of [`GetBufferMemoryRequirements2`]
///or [`GetImageMemoryRequirements2`], respectively.Constraints on the values returned for buffer
/// resources are:
/// - [`requires_dedicated_allocation`]**may** be [`TRUE`] if the [`p_next`] chain of
///   [`BufferCreateInfo`] for the call to [`CreateBuffer`] used to create the buffer being queried
///   included a [`ExternalMemoryBufferCreateInfo`] structure, and any of the handle types specified
///   in [`ExternalMemoryBufferCreateInfo::handle_types`] requires dedicated allocation, as reported
///   by [`GetPhysicalDeviceExternalBufferProperties`] in
///   [`ExternalBufferProperties`]::`externalMemoryProperties.externalMemoryFeatures`. Otherwise,
///   [`requires_dedicated_allocation`] will be [`FALSE`].
/// - When the implementation sets [`requires_dedicated_allocation`] to [`TRUE`], it **must** also
///   set [`prefers_dedicated_allocation`] to [`TRUE`].
/// - If `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` was set in [`BufferCreateInfo::flags`] when `buffer`
///   was created, then both [`prefers_dedicated_allocation`] and [`requires_dedicated_allocation`]
///   will be [`FALSE`].
///Constraints on the values returned for image resources are:
/// - [`requires_dedicated_allocation`]**may** be [`TRUE`] if the [`p_next`] chain of
///   [`ImageCreateInfo`] for the call to [`CreateImage`] used to create the image being queried
///   included a [`ExternalMemoryImageCreateInfo`] structure, and any of the handle types specified
///   in [`ExternalMemoryImageCreateInfo::handle_types`] requires dedicated allocation, as reported
///   by [`GetPhysicalDeviceImageFormatProperties2`] in
///   [`ExternalImageFormatProperties`]::`externalMemoryProperties.externalMemoryFeatures`.
///   Otherwise, [`requires_dedicated_allocation`] will be [`FALSE`].
/// - If `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` was set in [`ImageCreateInfo::flags`] when `image` was
///   created, then both [`prefers_dedicated_allocation`] and [`requires_dedicated_allocation`] will
///   be [`FALSE`].
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MemoryDedicatedRequirements<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`prefers_dedicated_allocation`] specifies that the implementation would
    ///prefer a dedicated allocation for this resource.
    ///The application is still free to suballocate the resource but it **may**
    ///get better performance if a dedicated allocation is used.
    prefers_dedicated_allocation: Bool32,
    ///[`requires_dedicated_allocation`] specifies that a dedicated allocation
    ///is required for this resource.
    requires_dedicated_allocation: Bool32,
}
///[VkMemoryDedicatedAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryDedicatedAllocateInfo.html) - Specify a dedicated memory allocation resource
///# C Specifications
///If the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`]
///structure, then that structure includes a handle of the sole buffer or image
///resource that the memory **can** be bound to.The [`MemoryDedicatedAllocateInfo`] structure is
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
///Valid Usage
/// - At least one of [`image`] and [`buffer`]**must** be [`crate::utils::Handle::null`]
/// - If [`image`] is not [`crate::utils::Handle::null`] and the memory is not an imported Android
///   Hardware Buffer, [`MemoryAllocateInfo::allocation_size`]**must** equal the
///   [`MemoryRequirements::size`] of the image
/// - If [`image`] is not [`crate::utils::Handle::null`], [`image`]**must** have been created
///   without `VK_IMAGE_CREATE_SPARSE_BINDING_BIT` set in [`ImageCreateInfo::flags`]
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and the memory is not an imported Android
///   Hardware Buffer, [`MemoryAllocateInfo::allocation_size`]**must** equal the
///   [`MemoryRequirements::size`] of the buffer
/// - If [`buffer`] is not [`crate::utils::Handle::null`], [`buffer`]**must** have been created
///   without `VK_BUFFER_CREATE_SPARSE_BINDING_BIT` set in [`BufferCreateInfo::flags`]
/// - If [`image`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a memory
///   import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`, and the external handle was created by
///   the Vulkan API, then the memory being imported **must** also be a dedicated image allocation
///   and [`image`] must be identical to the image associated with the imported memory
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a
///   memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`,
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, or
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`, and the external handle was created by
///   the Vulkan API, then the memory being imported **must** also be a dedicated buffer allocation
///   and [`buffer`]**must** be identical to the buffer associated with the imported memory
/// - If [`image`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a memory
///   import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`, the memory
///   being imported **must** also be a dedicated image allocation and [`image`]**must** be
///   identical to the image associated with the imported memory
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a
///   memory import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`, the
///   memory being imported **must** also be a dedicated buffer allocation and [`buffer`]**must** be
///   identical to the buffer associated with the imported memory
/// - If [`image`] is not [`crate::utils::Handle::null`], [`image`]**must** not have been created
///   with `VK_IMAGE_CREATE_DISJOINT_BIT` set in [`ImageCreateInfo::flags`]
/// - If [`image`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a memory
///   import operation with handle type `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`, the
///   memory being imported **must** also be a dedicated image allocation and [`image`]**must** be
///   identical to the image associated with the imported memory
/// - If [`buffer`] is not [`crate::utils::Handle::null`] and [`MemoryAllocateInfo`] defines a
///   memory import operation with handle type
///   `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`, the memory being imported **must**
///   also be a dedicated buffer allocation and [`buffer`]**must** be identical to the buffer
///   associated with the imported memory
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO`
/// - If [`image`] is not [`crate::utils::Handle::null`], [`image`]**must** be a valid [`Image`]
///   handle
/// - If [`buffer`] is not [`crate::utils::Handle::null`], [`buffer`]**must** be a valid [`Buffer`]
///   handle
/// - Both of [`buffer`], and [`image`] that are valid handles of non-ignored parameters **must**
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MemoryDedicatedAllocateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`image`] is [`crate::utils::Handle::null`] or a handle of an image which this
    ///memory will be bound to.
    image: Image,
    ///[`buffer`] is [`crate::utils::Handle::null`] or a handle of a buffer which this
    ///memory will be bound to.
    buffer: Buffer,
}
///[VkImageViewUsageCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageViewUsageCreateInfo.html) - Specify the intended usage of an image view
///# C Specifications
///The set of usages for the created image view **can** be restricted compared to
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
///determining the valid usage conditions of [`ImageViewCreateInfo`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO`
/// - [`usage`]**must** be a valid combination of [`ImageUsageFlagBits`] values
/// - [`usage`]**must** not be `0`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImageViewUsageCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`usage`] is a bitmask of [`ImageUsageFlagBits`] specifying
    ///allowed usages of the image view.
    usage: ImageUsageFlags,
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
///`VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT`.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO`
/// - [`domain_origin`]**must** be a valid [`TessellationDomainOrigin`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PipelineTessellationDomainOriginStateCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`domain_origin`] is a [`TessellationDomainOrigin`] value
    ///controlling the origin of the tessellation domain space.
    domain_origin: TessellationDomainOrigin,
}
///[VkSamplerYcbcrConversionInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionInfo.html) - Structure specifying Y′C<sub>B</sub>C<sub>R</sub> conversion to a sampler or image view
///# C Specifications
///To create a sampler with Y′C<sub>B</sub>C<sub>R</sub> conversion enabled, add a
///[`SamplerYcbcrConversionInfo`] structure to the [`p_next`] chain of the
///[`SamplerCreateInfo`] structure.
///To create a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion, the
///[`samplerYcbcrConversion` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-samplerYcbcrConversion)**must** be enabled.
///Conversion **must** be fixed at pipeline creation time, through use of a
///combined image sampler with an immutable sampler in
///[`DescriptorSetLayoutBinding`].A [`SamplerYcbcrConversionInfo`]**must** be provided for samplers
/// to be
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO`
/// - [`conversion`]**must** be a valid [`SamplerYcbcrConversion`] handle
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SamplerYcbcrConversionInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`conversion`] is a [`SamplerYcbcrConversion`] handle created with
    ///[`CreateSamplerYcbcrConversion`].
    conversion: SamplerYcbcrConversion,
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
/// - [`force_explicit_reconstruction`]**can** be used to ensure that reconstruction is done
///   explicitly, if supported.
///# Description
///If the [`p_next`] chain includes a [`ExternalFormatANDROID`] structure
///with non-zero `externalFormat` member, the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion
///object represents an *external format conversion*, and [`format`]**must** be
///`VK_FORMAT_UNDEFINED`.
///Such conversions **must** only be used to sample image views with a matching
///[external
///format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats).
///When creating an external format conversion, the value of [`components`]
///is ignored.Valid Usage
/// - If an external format conversion is being created, [`format`]**must** be `VK_FORMAT_UNDEFINED`
/// - If an external format conversion is not being created, [`format`]**must** represent unsigned
///   normalized values (i.e. the format must be a `UNORM` format)
/// - The [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features)
///   of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion **must** support
///   `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT` or
///   `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`
/// -    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`, [`x_chroma_offset`] and [`y_chroma_offset`]**must** not be `VK_CHROMA_LOCATION_COSITED_EVEN` if the corresponding components are [downsampled](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction)
/// -    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT`, [`x_chroma_offset`] and [`y_chroma_offset`]**must** not be `VK_CHROMA_LOCATION_MIDPOINT` if the corresponding components are [downsampled](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction)
/// - If the format has a `_422` or `_420` suffix, then `components.g`**must** be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
/// - If the format has a `_422` or `_420` suffix, then `components.a`**must** be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings),
///   `VK_COMPONENT_SWIZZLE_ONE`, or `VK_COMPONENT_SWIZZLE_ZERO`
/// - If the format has a `_422` or `_420` suffix, then `components.r`**must** be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
///   or `VK_COMPONENT_SWIZZLE_B`
/// - If the format has a `_422` or `_420` suffix, then `components.b`**must** be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
///   or `VK_COMPONENT_SWIZZLE_R`
/// - If the format has a `_422` or `_420` suffix, and if either `components.r` or `components.b` is
///   the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings),
///   both values **must** be the identity swizzle
/// -    If [`ycbcr_model`] is not `VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY`, then `components.r`, `components.g`, and `components.b`**must** correspond to components of the [`format`]; that is, `components.r`, `components.g`, and `components.b`**must** not be `VK_COMPONENT_SWIZZLE_ZERO` or `VK_COMPONENT_SWIZZLE_ONE`, and **must** not correspond to a component containing zero or one as a consequence of [conversion to RGBA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-conversion-to-rgba)
/// - If [`ycbcr_range`] is `VK_SAMPLER_YCBCR_RANGE_ITU_NARROW` then the R, G and B components
///   obtained by applying the `component` swizzle to [`format`]**must** each have a bit-depth
///   greater than or equal to 8
/// -    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`[`force_explicit_reconstruction`]**must** be [`FALSE`]
/// - If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features)
///   of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`,
///   [`chroma_filter`]**must** not be `VK_FILTER_LINEAR`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of [`ExternalFormatANDROID`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`format`]**must** be a valid [`Format`] value
/// - [`ycbcr_model`]**must** be a valid [`SamplerYcbcrModelConversion`] value
/// - [`ycbcr_range`]**must** be a valid [`SamplerYcbcrRange`] value
/// - [`components`]**must** be a valid [`ComponentMapping`] structure
/// - [`x_chroma_offset`]**must** be a valid [`ChromaLocation`] value
/// - [`y_chroma_offset`]**must** be a valid [`ChromaLocation`] value
/// - [`chroma_filter`]**must** be a valid [`Filter`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SamplerYcbcrConversionCreateInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`format`] is the format of the image from which color information
    ///will be retrieved.
    format: Format,
    ///[`ycbcr_model`] describes the color matrix for conversion between color
    ///models.
    ycbcr_model: SamplerYcbcrModelConversion,
    ///[`ycbcr_range`] describes whether the encoded values have headroom and
    ///foot room, or whether the encoding uses the full numerical range.
    ycbcr_range: SamplerYcbcrRange,
    ///[`components`] applies a *swizzle* based on [`ComponentSwizzle`]
    ///enums prior to range expansion and color model conversion.
    components: ComponentMapping,
    ///[`x_chroma_offset`] describes the
    ///[sample location](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction) associated with
    ///downsampled chroma components in the x dimension.
    ///[`x_chroma_offset`] has no effect for formats in which chroma components
    ///are not downsampled horizontally.
    x_chroma_offset: ChromaLocation,
    ///[`y_chroma_offset`] describes the
    ///[sample location](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction) associated with
    ///downsampled chroma components in the y dimension.
    ///[`y_chroma_offset`] has no effect for formats in which the chroma
    ///components are not downsampled vertically.
    y_chroma_offset: ChromaLocation,
    ///[`chroma_filter`] is the filter for chroma reconstruction.
    chroma_filter: Filter,
    ///[`force_explicit_reconstruction`]**can** be used to ensure that
    ///reconstruction is done explicitly, if supported.
    force_explicit_reconstruction: Bool32,
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
///Valid Usage
/// - If the image’s `tiling` is `VK_IMAGE_TILING_LINEAR` or `VK_IMAGE_TILING_OPTIMAL`, then
///   [`plane_aspect`]**must** be a single valid *format plane* for the image (that is, for a
///   two-plane image [`plane_aspect`]**must** be `VK_IMAGE_ASPECT_PLANE_0_BIT` or
///   `VK_IMAGE_ASPECT_PLANE_1_BIT`, and for a three-plane image [`plane_aspect`]**must** be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or `VK_IMAGE_ASPECT_PLANE_2_BIT`)
/// - If the image’s `tiling` is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then
///   [`plane_aspect`]**must** be a single valid *memory plane* for the image (that is,
///   `aspectMask`**must** specify a plane index that is less than the
///   [`DrmFormatModifierPropertiesEXT::drm_format_modifier_plane_count`] associated with the
///   image’s `format` and [`ImageDrmFormatModifierPropertiesEXT::drm_format_modifier`])
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO`
/// - [`plane_aspect`]**must** be a valid [`ImageAspectFlagBits`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BindImagePlaneMemoryInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the
    ///aspect of the disjoint image plane to bind.
    plane_aspect: ImageAspectFlagBits,
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
///Valid Usage
/// - If the image’s `tiling` is `VK_IMAGE_TILING_LINEAR` or `VK_IMAGE_TILING_OPTIMAL`, then
///   [`plane_aspect`]**must** be a single valid *format plane* for the image (that is, for a
///   two-plane image [`plane_aspect`]**must** be `VK_IMAGE_ASPECT_PLANE_0_BIT` or
///   `VK_IMAGE_ASPECT_PLANE_1_BIT`, and for a three-plane image [`plane_aspect`]**must** be
///   `VK_IMAGE_ASPECT_PLANE_0_BIT`, `VK_IMAGE_ASPECT_PLANE_1_BIT` or `VK_IMAGE_ASPECT_PLANE_2_BIT`)
/// - If the image’s `tiling` is `VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, then
///   [`plane_aspect`]**must** be a single valid *memory plane* for the image (that is,
///   `aspectMask`**must** specify a plane index that is less than the
///   [`DrmFormatModifierPropertiesEXT::drm_format_modifier_plane_count`] associated with the
///   image’s `format` and [`ImageDrmFormatModifierPropertiesEXT::drm_format_modifier`])
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO`
/// - [`plane_aspect`]**must** be a valid [`ImageAspectFlagBits`] value
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ImagePlaneMemoryRequirementsInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the
    ///aspect corresponding to the image plane to query.
    plane_aspect: ImageAspectFlagBits,
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
/// - [`sampler_ycbcr_conversion`] specifies whether the implementation supports [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion). If [`sampler_ycbcr_conversion`] is [`FALSE`], sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is not supported, and samplers using sampler Y′C<sub>B</sub>C<sub>R</sub> conversion **must** not be used.
///If the [`PhysicalDeviceSamplerYcbcrConversionFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceSamplerYcbcrConversionFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`sampler_ycbcr_conversion`] specifies whether the implementation
    ///supports [sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-YCbCr-conversion).
    ///If [`sampler_ycbcr_conversion`] is [`FALSE`], sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///conversion is not supported, and samplers using sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///conversion **must** not be used.
    sampler_ycbcr_conversion: Bool32,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SamplerYcbcrConversionImageFormatProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`combined_image_sampler_descriptor_count`] is the number of combined
    ///image sampler descriptors that the implementation uses to access the
    ///format.
    combined_image_sampler_descriptor_count: u32,
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
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO`
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ProtectedSubmitInfo<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///No documentation found
    s_type: StructureType,
    ///No documentation found
    p_next: *mut BaseInStructure<'lt>,
    ///[`protected_submit`] specifies whether the batch is protected.
    ///If [`protected_submit`] is [`TRUE`], the batch is protected.
    ///If [`protected_submit`] is [`FALSE`], the batch is unprotected.
    ///If the [`SubmitInfo`]::[`p_next`] chain does not include this
    ///structure, the batch is unprotected.
    protected_submit: Bool32,
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
///[`PhysicalDeviceProtectedMemoryFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`protected_memory`]
    ///specifies whether protected memory is supported.
    protected_memory: Bool32,
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
///   values. If this limit is [`FALSE`], applications **must** not perform these operations. See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-protected-access-rules](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-protected-access-rules)
///   for more information.
///If the [`PhysicalDeviceProtectedMemoryProperties`] structure is included in the [`p_next`] chain
/// of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    protected_no_fault: Bool32,
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
///   chain of [`DeviceQueueInfo2`]**can** be used to provide additional device queue parameters to
///   [`GetDeviceQueue2`].
/// - [`flags`] is a [`DeviceQueueCreateFlags`] value indicating the flags used to create the device
///   queue.
/// - [`queue_family_index`] is the index of the queue family to which the queue belongs.
/// - [`queue_index`] is the index within this queue family of the queue to retrieve.
///# Description
///The queue returned by [`GetDeviceQueue2`]**must** have the same
///[`flags`] value from this structure as that used at device creation time
///in a [`DeviceQueueCreateInfo`] structure.
///If no matching [`flags`] were specified at device creation time, then the
///handle returned in `pQueue`**must** be `NULL`.Valid Usage
/// - [`queue_family_index`]**must** be one of the queue family indices specified when `device` was
///   created, via the [`DeviceQueueCreateInfo`] structure
/// - [`flags`]**must** be equal to [`DeviceQueueCreateInfo`]::[`flags`] for a
///   [`DeviceQueueCreateInfo`] structure for the queue family indicated by [`queue_family_index`]
///   when `device` was created
/// - [`queue_index`]**must** be less than [`DeviceQueueCreateInfo::queue_count`] for the
///   corresponding queue family and flags indicated by [`queue_family_index`] and [`flags`] when
///   `device` was created
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2`
/// - [`p_next`]**must** be `NULL`
/// - [`flags`]**must** be a valid combination of [`DeviceQueueCreateFlagBits`] values
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
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DeviceQueueInfo2<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    ///The [`p_next`] chain of [`DeviceQueueInfo2`]**can** be used to
    ///provide additional device queue parameters to [`GetDeviceQueue2`].
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a [`DeviceQueueCreateFlags`] value indicating the
    ///flags used to create the device queue.
    flags: DeviceQueueCreateFlags,
    ///[`queue_family_index`] is the index of the queue family to which the
    ///queue belongs.
    queue_family_index: u32,
    ///[`queue_index`] is the index within this queue family of the queue to
    ///retrieve.
    queue_index: u32,
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
///   constraints on the size of a descriptor set itself. Applications **can** query whether a
///   descriptor set that goes beyond this limit is supported using
///   [`GetDescriptorSetLayoutSupport`].
/// - [`max_memory_allocation_size`] is the maximum size of a memory allocation that **can** be
///   created, even if there is more space available in the heap.
///If the [`PhysicalDeviceMaintenance3Properties`] structure is included in the [`p_next`] chain of
/// the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    max_per_set_descriptors: u32,
    ///No documentation found
    max_memory_allocation_size: DeviceSize,
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
/// - [`supported`] specifies whether the descriptor set layout **can** be created.
///# Description
///[`supported`] is set to [`TRUE`] if the descriptor set **can** be
///created, or else is set to [`FALSE`].Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT`
/// - [`p_next`]**must** be `NULL` or a pointer to a valid instance of
///   [`DescriptorSetVariableDescriptorCountLayoutSupport`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DescriptorSetLayoutSupport<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`supported`] specifies whether the descriptor set layout **can** be
    ///created.
    supported: Bool32,
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
///   `DrawParameters` capability. When this feature is not enabled, shader modules **must** not
///   declare the `SPV_KHR_shader_draw_parameters` extension or the `DrawParameters` capability.
///If the [`PhysicalDeviceShaderDrawParametersFeatures`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShaderDrawParametersFeatures`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES`
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
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shader_draw_parameters`] specifies whether the implementation supports
    ///the SPIR-V `DrawParameters` capability.
    ///When this feature is not enabled, shader modules **must** not declare the
    ///`SPV_KHR_shader_draw_parameters` extension or the `DrawParameters`
    ///capability.
    shader_draw_parameters: Bool32,
}
