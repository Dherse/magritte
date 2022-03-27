use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Extent2D, Offset2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_QCOM_fragment_density_map_offset");
///[VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html) - Structure describing fragment density map offet features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`] structure is
///defined as:
///```c
///// Provided by VK_QCOM_fragment_density_map_offset
///typedef struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fragmentDensityMapOffset;
///} VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - `fragmentDensityMapOffsets` specifies whether the implementation supports [fragment density map offsets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapoffsets)
///If the [`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM`]**can** also be used in the [`p_next`]
/// chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM`
///# Related
/// - [`VK_QCOM_fragment_density_map_offset`]
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
pub struct PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///No documentation found
    fragment_density_map_offset: Bool32,
}
///[VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html) - Structure describing fragment density map offset properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`] structure
///is defined as:
///```c
///// Provided by VK_QCOM_fragment_density_map_offset
///typedef struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
///    VkStructureType    sType;
///    void*              pNext;
///    VkExtent2D         fragmentDensityOffsetGranularity;
///} VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_offset_granularity`] is the granularity for [fragment density offsets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapoffsets).
///# Description
///If the [`PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM`
///# Related
/// - [`VK_QCOM_fragment_density_map_offset`]
/// - [`Extent2D`]
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
pub struct PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`fragment_density_offset_granularity`] is the granularity for
    ///[fragment density offsets](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapoffsets).
    fragment_density_offset_granularity: Extent2D,
}
///[VkSubpassFragmentDensityMapOffsetEndInfoQCOM](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html) - Structure specifying fragment density map offset subpass end information
///# C Specifications
///If the [`SubpassEndInfo`]::[`p_next`] chain includes a
///[`SubpassFragmentDensityMapOffsetEndInfoQCOM`] structure, then that
///structure includes an array of fragment density map offsets per layer for
///the render pass.The [`SubpassFragmentDensityMapOffsetEndInfoQCOM`] structure is defined
///as:
///```c
///// Provided by VK_QCOM_fragment_density_map_offset
///typedef struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
///    VkStructureType      sType;
///    const void*          pNext;
///    uint32_t             fragmentDensityOffsetCount;
///    const VkOffset2D*    pFragmentDensityOffsets;
///} VkSubpassFragmentDensityMapOffsetEndInfoQCOM;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_offset_count`] is the number of offsets being specified.
/// - [`p_fragment_density_offsets`] is a pointer to an array of [`Offset2D`] structs, each of which
///   describes the offset per layer.
///# Description
///The array elements are given per `layer` as defined by
///[Fetch Density Value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymap-fetch-density-value), where
///index = layer.
///Each (x,y) offset is in framebuffer pixels and shifts the fetch of the
///fragment density map by that amount.
///Offsets can be positive or negative.Offset values specified for any subpass that is not the last
/// subpass in the
///render pass are ignored.
///If the [`SubpassEndInfo`]::[`p_next`] chain for the last subpass of a
///renderpass does not include
///[`SubpassFragmentDensityMapOffsetEndInfoQCOM`], or if
///[`fragment_density_offset_count`] is zero, then the offset (0,0) is
///used for [Fetch Density Value](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragmentdensitymap-fetch-density-value).Valid Usage
/// - If the [`fragmentDensityMapOffsets`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-fragmentDensityMapOffsets)
///   feature is not enabled or fragment density map is not enabled in the render pass,
///   [`fragment_density_offset_count`]**must** equal `0`.
/// - If [`SubpassDescription`]`::fragmentDensityMapAttachment` is not is not [`ATTACHMENT_UNUSED`]
///   and was not created with `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`,
///   [`fragment_density_offset_count`]**must** equal `0`.
/// - If [`SubpassDescription::p_depth_stencil_attachment`] is not is not [`ATTACHMENT_UNUSED`] and
///   was not created with `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`,
///   [`fragment_density_offset_count`]**must** equal `0`.
/// - If any element of [`SubpassDescription::p_input_attachments`] is not is not
///   [`ATTACHMENT_UNUSED`] and was not created with
///   `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`,
///   [`fragment_density_offset_count`]**must** equal `0`.
/// - If any element of [`SubpassDescription::p_color_attachments`] is not is not
///   [`ATTACHMENT_UNUSED`] and was not created with
///   `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`,
///   [`fragment_density_offset_count`]**must** equal `0`.
/// - If any element of [`SubpassDescription::p_resolve_attachments`] is not is not
///   [`ATTACHMENT_UNUSED`] and was not created with
///   `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`,
///   [`fragment_density_offset_count`]**must** equal `0`.
/// - If any element of [`SubpassDescription::p_preserve_attachments`] is not is not
///   [`ATTACHMENT_UNUSED`] and was not created with
///   `VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM`,
///   [`fragment_density_offset_count`]**must** equal `0`.
/// - If [`fragment_density_offset_count`] is not `0` and multiview is enabled for the render pass,
///   [`fragment_density_offset_count`]**must** equal the `layerCount` that was specified in
///   creating the fragment density map attachment view.
/// - If [`fragment_density_offset_count`] is not `0` and multiview is not enabled for the render
///   pass, [`fragment_density_offset_count`]**must** equal `1`.
/// - The `x` component of each element of [`p_fragment_density_offsets`]**must** be an integer
///   multiple of `fragmentDensityOffsetGranularity.width`.
/// - The `y` component of each element of [`p_fragment_density_offsets`]**must** be an integer
///   multiple of `fragmentDensityOffsetGranularity.height`.
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM`
/// - If [`fragment_density_offset_count`] is not `0`, [`p_fragment_density_offsets`]**must** be a
///   valid pointer to an array of [`fragment_density_offset_count`][`Offset2D`] structures
///# Related
/// - [`VK_QCOM_fragment_density_map_offset`]
/// - [`Offset2D`]
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
pub struct SubpassFragmentDensityMapOffsetEndInfoQCOM<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`fragment_density_offset_count`] is the number of offsets being
    ///specified.
    fragment_density_offset_count: u32,
    ///[`p_fragment_density_offsets`] is a pointer to an array of
    ///[`Offset2D`] structs, each of which describes the offset per layer.
    p_fragment_density_offsets: *mut Offset2D,
}
