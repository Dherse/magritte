#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
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
/// - [`DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`] specifies that
///the descriptor update template will be used for descriptor set updates
///only.
/// - [`DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`] specifies
///that the descriptor update template will be used for push descriptor
///updates only.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DescriptorUpdateTemplateType(i32);
impl const Default for DescriptorUpdateTemplateType {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for DescriptorUpdateTemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("DescriptorUpdateTemplateType")
            .field(match *self {
                Self::DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET => {
                    &"DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET"
                },
                Self::DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR => {
                    &"DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR"
                },
                other => unreachable!("invalid value for `DescriptorUpdateTemplateType`: {:?}", other),
            })
            .finish()
    }
}
impl DescriptorUpdateTemplateType {
    ///[`DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`] specifies that
    ///the descriptor update template will be used for descriptor set updates
    ///only.
    pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET: Self = Self(0);
    ///[`DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`] specifies
    ///that the descriptor update template will be used for push descriptor
    ///updates only.
    ///
    ///Provided by [`crate::extensions::khr_descriptor_update_template`]
    pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR: Self = Self(1);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_descriptor_update_template`]
    pub const DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR: Self =
        Self::DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET;
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
/// - [`POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES`] specifies that the
///primitive is discarded if the vertex lies outside any clip plane,
///including the planes bounding the view volume.
/// - [`POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY`] specifies that
///the primitive is discarded only if the vertex lies outside any user clip
///plane.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PointClippingBehavior(i32);
impl const Default for PointClippingBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PointClippingBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PointClippingBehavior")
            .field(match *self {
                Self::POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES => &"POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES",
                Self::POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY => &"POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY",
                other => unreachable!("invalid value for `PointClippingBehavior`: {:?}", other),
            })
            .finish()
    }
}
impl PointClippingBehavior {
    ///[`POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES`] specifies that the
    ///primitive is discarded if the vertex lies outside any clip plane,
    ///including the planes bounding the view volume.
    pub const POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: Self = Self(0);
    ///[`POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY`] specifies that
    ///the primitive is discarded only if the vertex lies outside any user clip
    ///plane.
    pub const POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: Self = Self(1);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_maintenance_2`]
    pub const POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR: Self = Self::POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_maintenance_2`]
    pub const POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR: Self =
        Self::POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY;
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
/// - [`TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT`] specifies that the origin
///of the domain space is in the upper left corner, as shown in figure
///[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul).
/// - [`TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT`] specifies that the origin
///of the domain space is in the lower left corner, as shown in figure
///[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll).This enum affects how the `VertexOrderCw` and `VertexOrderCcw`
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct TessellationDomainOrigin(i32);
impl const Default for TessellationDomainOrigin {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for TessellationDomainOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("TessellationDomainOrigin")
            .field(match *self {
                Self::TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT => &"TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT",
                Self::TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT => &"TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT",
                other => unreachable!("invalid value for `TessellationDomainOrigin`: {:?}", other),
            })
            .finish()
    }
}
impl TessellationDomainOrigin {
    ///[`TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT`] specifies that the origin
    ///of the domain space is in the upper left corner, as shown in figure
    ///[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ul).
    pub const TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: Self = Self(0);
    ///[`TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT`] specifies that the origin
    ///of the domain space is in the lower left corner, as shown in figure
    ///[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#img-tessellation-topology-ll).
    pub const TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: Self = Self(1);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_maintenance_2`]
    pub const TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR: Self = Self::TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_maintenance_2`]
    pub const TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR: Self = Self::TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT;
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
/// - [`SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY`] specifies that the
///input values to the conversion are unmodified.
/// - [`SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY`] specifies no
///model conversion but the inputs are range expanded as for Y′C<sub>B</sub>C<sub>R</sub>.
/// - [`SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709`] specifies the color
///model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.709 and
///described in the “BT.709 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the
///[Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
/// - [`SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601`] specifies the color
///model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.601 and
///described in the “BT.601 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the
///[Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
/// - [`SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020`] specifies the color
///model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.2020 and
///described in the “BT.2020 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the
///[Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).In the `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_*` color models, for the
///input to the sampler Y′C<sub>B</sub>C<sub>R</sub> range expansion and model conversion:
/// - the Y (Y′ luma) component corresponds to the G component of an RGB
///image.
/// - the CB (C<sub>B</sub> or “U” blue color difference) component corresponds to
///the B component of an RGB image.
/// - the CR (C<sub>R</sub> or “V” red color difference) component corresponds to the
///R component of an RGB image.
/// - the alpha component, if present, is not modified by color model
///conversion.These rules reflect the mapping of components after the component swizzle
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SamplerYcbcrModelConversion(i32);
impl const Default for SamplerYcbcrModelConversion {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for SamplerYcbcrModelConversion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("SamplerYcbcrModelConversion")
            .field(match *self {
                Self::SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY => &"SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY",
                Self::SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY => &"SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY",
                Self::SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709 => &"SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709",
                Self::SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601 => &"SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601",
                Self::SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020 => &"SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020",
                other => unreachable!("invalid value for `SamplerYcbcrModelConversion`: {:?}", other),
            })
            .finish()
    }
}
impl SamplerYcbcrModelConversion {
    ///[`SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY`] specifies that the
    ///input values to the conversion are unmodified.
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: Self = Self(0);
    ///[`SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY`] specifies no
    ///model conversion but the inputs are range expanded as for Y′C<sub>B</sub>C<sub>R</sub>.
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: Self = Self(1);
    ///[`SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709`] specifies the color
    ///model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.709 and
    ///described in the “BT.709 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the
    ///[Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: Self = Self(2);
    ///[`SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601`] specifies the color
    ///model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.601 and
    ///described in the “BT.601 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the
    ///[Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: Self = Self(3);
    ///[`SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020`] specifies the color
    ///model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.2020 and
    ///described in the “BT.2020 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the
    ///[Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: Self = Self(4);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR: Self = Self::SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR: Self =
        Self::SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR: Self = Self::SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR: Self = Self::SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR: Self = Self::SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020;
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
/// - [`SAMPLER_YCBCR_RANGE_ITU_FULL`] specifies that the full range of
///the encoded values are valid and interpreted according to the ITU “full
///range” quantization rules.
/// - [`SAMPLER_YCBCR_RANGE_ITU_NARROW`] specifies that headroom and foot
///room are reserved in the numerical range of encoded values, and the
///remaining values are expanded according to the ITU “narrow range”
///quantization rules.The formulae for these conversions is described in the
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SamplerYcbcrRange(i32);
impl const Default for SamplerYcbcrRange {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for SamplerYcbcrRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("SamplerYcbcrRange")
            .field(match *self {
                Self::SAMPLER_YCBCR_RANGE_ITU_FULL => &"SAMPLER_YCBCR_RANGE_ITU_FULL",
                Self::SAMPLER_YCBCR_RANGE_ITU_NARROW => &"SAMPLER_YCBCR_RANGE_ITU_NARROW",
                other => unreachable!("invalid value for `SamplerYcbcrRange`: {:?}", other),
            })
            .finish()
    }
}
impl SamplerYcbcrRange {
    ///[`SAMPLER_YCBCR_RANGE_ITU_FULL`] specifies that the full range of
    ///the encoded values are valid and interpreted according to the ITU “full
    ///range” quantization rules.
    pub const SAMPLER_YCBCR_RANGE_ITU_FULL: Self = Self(0);
    ///[`SAMPLER_YCBCR_RANGE_ITU_NARROW`] specifies that headroom and foot
    ///room are reserved in the numerical range of encoded values, and the
    ///remaining values are expanded according to the ITU “narrow range”
    ///quantization rules.
    pub const SAMPLER_YCBCR_RANGE_ITU_NARROW: Self = Self(1);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const SAMPLER_YCBCR_RANGE_ITU_FULL_KHR: Self = Self::SAMPLER_YCBCR_RANGE_ITU_FULL;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR: Self = Self::SAMPLER_YCBCR_RANGE_ITU_NARROW;
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
/// - [`CHROMA_LOCATION_COSITED_EVEN`] specifies that downsampled chroma
///samples are aligned with luma samples with even coordinates.
/// - [`CHROMA_LOCATION_MIDPOINT`] specifies that downsampled chroma
///samples are located half way between each even luma sample and the
///nearest higher odd luma sample.
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ChromaLocation(i32);
impl const Default for ChromaLocation {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ChromaLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ChromaLocation")
            .field(match *self {
                Self::CHROMA_LOCATION_COSITED_EVEN => &"CHROMA_LOCATION_COSITED_EVEN",
                Self::CHROMA_LOCATION_MIDPOINT => &"CHROMA_LOCATION_MIDPOINT",
                other => unreachable!("invalid value for `ChromaLocation`: {:?}", other),
            })
            .finish()
    }
}
impl ChromaLocation {
    ///[`CHROMA_LOCATION_COSITED_EVEN`] specifies that downsampled chroma
    ///samples are aligned with luma samples with even coordinates.
    pub const CHROMA_LOCATION_COSITED_EVEN: Self = Self(0);
    ///[`CHROMA_LOCATION_MIDPOINT`] specifies that downsampled chroma
    ///samples are located half way between each even luma sample and the
    ///nearest higher odd luma sample.
    pub const CHROMA_LOCATION_MIDPOINT: Self = Self(1);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const CHROMA_LOCATION_COSITED_EVEN_KHR: Self = Self::CHROMA_LOCATION_COSITED_EVEN;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::khr_sampler_ycbcr_conversion`]
    pub const CHROMA_LOCATION_MIDPOINT_KHR: Self = Self::CHROMA_LOCATION_MIDPOINT;
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
