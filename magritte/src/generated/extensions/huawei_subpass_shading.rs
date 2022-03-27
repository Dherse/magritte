use crate::vulkan1_0::{BaseOutStructure, Bool32, RenderPass, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION")]
pub const HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME")]
pub const HUAWEI_SUBPASS_SHADING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_HUAWEI_subpass_shading");
///[VkSubpassShadingPipelineCreateInfoHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html) - Structure specifying parameters of a newly created subpass shading pipeline
///# C Specifications
///A subpass shading pipeline is a compute pipeline which **must** be called only
///in a subpass of a render pass with work dimensions specified by render area
///size.
///The subpass shading pipeline shader is a compute shader allowed to access
///input attachments specified in the calling subpass.
///To create a subpass shading pipeline, call [`CreateComputePipelines`]
///with [`SubpassShadingPipelineCreateInfoHUAWEI`] in the [`p_next`] chain
///of [`ComputePipelineCreateInfo`].The [`SubpassShadingPipelineCreateInfoHUAWEI`] structure is
/// defined as:
///```c
///// Provided by VK_HUAWEI_subpass_shading
///typedef struct VkSubpassShadingPipelineCreateInfoHUAWEI {
///    VkStructureType    sType;
///    void*              pNext;
///    VkRenderPass       renderPass;
///    uint32_t           subpass;
///} VkSubpassShadingPipelineCreateInfoHUAWEI;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`render_pass`] is a handle to a render pass object describing the environment in which the pipeline will be used. The pipeline **must** only be used with a render pass instance compatible with the one provided. See [Render Pass Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) for more information.
/// - [`subpass`] is the index of the subpass in the render pass where this pipeline will be used.
///# Description
///Valid Usage
/// - [`subpass`]**must** be created with `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI` bind point
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI`
///# Related
/// - [`VK_HUAWEI_subpass_shading`]
/// - [`RenderPass`]
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
pub struct SubpassShadingPipelineCreateInfoHUAWEI<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`render_pass`] is a handle to a render pass object describing the
    ///environment in which the pipeline will be used.
    ///The pipeline **must** only be used with a render pass instance compatible
    ///with the one provided.
    ///See [Render Pass Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) for more
    ///information.
    render_pass: RenderPass,
    ///[`subpass`] is the index of the subpass in the render pass where this
    ///pipeline will be used.
    subpass: u32,
}
///[VkPhysicalDeviceSubpassShadingPropertiesHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html) - Structure describing subpass shading properties supported by an implementation
///# C Specifications
///The [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`] structure is
///defined as:
///```c
///// Provided by VK_HUAWEI_subpass_shading
///typedef struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxSubpassShadingWorkgroupSizeAspectRatio;
///} VkPhysicalDeviceSubpassShadingPropertiesHUAWEI;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_subpass_shading_workgroup_size_aspect_ratio`] indicates the maximum ratio between the
///   width and height of the portion of the subpass shading shader workgroup size.
///   [`max_subpass_shading_workgroup_size_aspect_ratio`]**must** be a power-of-two value, and
///   **must** be less than or equal to max(`WorkgroupSize.x` / `WorkgroupSize.y`, `WorkgroupSize.y`
///   / `WorkgroupSize.x`).
///# Description
///If the [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI`
///# Related
/// - [`VK_HUAWEI_subpass_shading`]
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
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_subpass_shading_workgroup_size_aspect_ratio`] indicates the maximum
    ///ratio between the width and height of the portion of the subpass shading
    ///shader workgroup size.
    ///[`max_subpass_shading_workgroup_size_aspect_ratio`]**must** be a power-of-two
    ///value, and **must** be less than or equal to max(`WorkgroupSize.x` /
    ///`WorkgroupSize.y`, `WorkgroupSize.y` / `WorkgroupSize.x`).
    max_subpass_shading_workgroup_size_aspect_ratio: u32,
}
///[VkPhysicalDeviceSubpassShadingFeaturesHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html) - Structure describing whether subpass shading is enabled
///# C Specifications
///The [`PhysicalDeviceSubpassShadingFeaturesHUAWEI`] structure is defined
///as:
///```c
///// Provided by VK_HUAWEI_subpass_shading
///typedef struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           subpassShading;
///} VkPhysicalDeviceSubpassShadingFeaturesHUAWEI;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`subpass_shading`] specifies whether subpass shading is supported.
///If the [`PhysicalDeviceSubpassShadingFeaturesHUAWEI`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceSubpassShadingFeaturesHUAWEI`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI`
///# Related
/// - [`VK_HUAWEI_subpass_shading`]
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
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`subpass_shading`] specifies whether
    ///subpass shading is supported.
    subpass_shading: Bool32,
}
