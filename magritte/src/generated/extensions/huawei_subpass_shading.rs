//![VK_HUAWEI_subpass_shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_HUAWEI_subpass_shading.html) - device extension
//!# Description
//!This extension allows applications to execute a subpass shading pipeline in
//!a subpass of a render pass in order to save memory bandwidth for algorithms
//!like tile-based deferred rendering and forward plus.
//!A subpass shading pipeline is a pipeline with the compute pipeline ability,
//!allowed to read values from input attachments, and only allowed to be
//!dispatched inside a stand-alone subpass.
//!Its work dimension is defined by the render pass’s render area size.
//!Its workgroup size (width, height) shall be a power-of-two number in width
//!or height, with minimum value from 8, and maximum value shall be decided
//!from the render pass attachments and sample counts but depends on
//!implementation.The `GlobalInvocationId.xy` of a subpass shading pipeline is equal to the
//!`FragCoord.xy` of a graphic pipeline in the same render pass subtracted
//!the [`Rect2D`] of the
//![`RenderPassBeginInfo::render_area`].
//!`GlobalInvocationId.z` is mapped to the Layer if
//!`[`ext_shader_viewport_index_layer`]` is supported.
//!The `GlobalInvocationId.xy` is equal to the index of the local workgroup
//!multiplied by the size of the local workgroup plus the
//!`LocalInvocationId` and the [`Rect2D`] of the
//![`RenderPassBeginInfo::render_area`].This extension allows a subpass’s pipeline bind point to be
//!`VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_create_renderpass2`]`
//! - Requires `[`khr_synchronization2`]`
//!# Contacts
//! - Hueilong Wang [wyvernathuawei](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_HUAWEI_subpass_shading]
//!   @wyvernathuawei%0A<<Here describe the issue or question you have about the
//!   VK_HUAWEI_subpass_shading extension>>)
//!# New functions & commands
//! - [`cmd_subpass_shading_huawei`]
//! - [`get_device_subpass_shading_max_workgroup_size_huawei`]
//!# New structures
//! - Extending [`ComputePipelineCreateInfo`]:  - [`SubpassShadingPipelineCreateInfoHUAWEI`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceSubpassShadingFeaturesHUAWEI`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`]
//!# New constants
//! - [`HUAWEI_SUBPASS_SHADING_EXTENSION_NAME`]
//! - [`HUAWEI_SUBPASS_SHADING_SPEC_VERSION`]
//! - Extending [`PipelineBindPoint`]:  - `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
//! - Extending [`ShaderStageFlagBits`]:  - `VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI`  -
//!   `VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI`
//!# Version History
//! - Revision 2, 2021-06-28 (Hueilong Wang)  - Change vkGetSubpassShadingMaxWorkgroupSizeHUAWEI to vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI to resolve issue [`pub1564`](https://github.com/KhronosGroup/Vulkan-Docs/issues/1564)
//! - Revision 1, 2020-12-15 (Hueilong Wang)  - Initial draft.
//!# Other info
//! * 2021-06-01
//! * - This extension requires [`SPV_HUAWEI_subpass_shading`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/HUAWEI/SPV_HUAWEI_subpass_shading.html).
//!   - This extension provides API support for [`GL_HUAWEI_subpass_shading`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/huawei/GLSL_HUAWEI_subpass_shading.txt).
//! * - Hueilong Wang
//!# Related
//! - [`PhysicalDeviceSubpassShadingFeaturesHUAWEI`]
//! - [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`]
//! - [`SubpassShadingPipelineCreateInfoHUAWEI`]
//! - [`cmd_subpass_shading_huawei`]
//! - [`get_device_subpass_shading_max_workgroup_size_huawei`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{
        BaseOutStructure, Bool32, CommandBuffer, Device, Extent2D, RenderPass, StructureType, VulkanResultCodes,
    },
    AsRaw, Unique, VulkanResult,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION")]
pub const HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME")]
pub const HUAWEI_SUBPASS_SHADING_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_HUAWEI_subpass_shading");
///[vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html) - Query maximum supported subpass shading workgroup size for a give render pass
///# C Specifications
///A subpass shading pipeline’s workgroup size is a 2D vector with number of
///power-of-two in width and height.
///The maximum number of width and height is implementation dependent, and  **may**
///vary for different formats and sample counts of attachments in a render
///pass.To query the maximum workgroup size, call:
///```c
///// Provided by VK_HUAWEI_subpass_shading
///VkResult vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
///    VkDevice                                    device,
///    VkRenderPass                                renderpass,
///    VkExtent2D*                                 pMaxWorkgroupSize);
///```
///# Parameters
/// - [`device`] is a handle to a local device object that was used to create the given render pass.
/// - `renderPass` is a handle to a render pass object describing the environment in which the pipeline will be used. The pipeline  **must**  only be used with a render pass instance compatible with the one provided. See [Render Pass Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) for more information.
/// - [`p_max_workgroup_size`] is a pointer to a [`Extent2D`] structure.
///# Description
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`renderpass`] **must**  be a valid [`RenderPass`] handle
/// - [`p_max_workgroup_size`] **must**  be a valid pointer to a [`Extent2D`] structure
/// - [`renderpass`] **must**  have been created, allocated, or retrieved from [`device`]
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
///   `VK_ERROR_SURFACE_LOST_KHR`
///# Related
/// - [`huawei_subpass_shading`]
/// - [`Device`]
/// - [`Extent2D`]
/// - [`RenderPass`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
pub type FNGetDeviceSubpassShadingMaxWorkgroupSizeHuawei = Option<
    unsafe extern "system" fn(
        device: Device,
        renderpass: RenderPass,
        p_max_workgroup_size: *mut Extent2D,
    ) -> VulkanResultCodes,
>;
///[vkCmdSubpassShadingHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html) - Dispatch compute work items
///# C Specifications
///A subpass shading dispatches a compute pipeline work with the work dimension
///of render area of the calling subpass and work groups are partitioned by
///specified work group size.
///Subpass operations like subpassLoad and subpassLoadMS are allowed to be
///used.To record a subpass shading, call:
///```c
///// Provided by VK_HUAWEI_subpass_shading
///void vkCmdSubpassShadingHUAWEI(
///    VkCommandBuffer                             commandBuffer);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
///# Description
///When the command is executed, a global workgroup consisting of ceil (render
///area size / local workgroup size) local workgroups is assembled.
///## Valid Usage
/// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]()
///   **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
/// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
/// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering together with
///   minmax filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be
///   sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
/// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
/// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
/// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point
///   used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline
///   bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
///   [[descriptorsets-compatibility]]()
/// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a
///   push constant value  **must**  have been set for the same pipeline bind point, with a
///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to
///   create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
/// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind
///   point used by this command
/// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any
///   dynamic state, that state  **must**  have been set or inherited (if the
///   `[`nv_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done so
///   after any previously bound pipeline with the corresponding state not specified as dynamic
/// - There  **must**  not have been any calls to dynamic state setting commands for any state not
///   specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this
///   command, since that pipeline was bound
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to
///   sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`,
///   `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or
///   `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a
///   LOD bias or any offset values, in any shader stage
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be a protected resource
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  only be used with `OpImageSample*` or
///   `OpImageSparseSample*` instructions
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
/// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the image view’s format
/// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the buffer view’s format
/// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result
///   of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**
///   have a `Width` of 64
/// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
///   **must**  have a `Width` of 64
/// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with
///   the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with
///   the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - This command must be called in a subpass with bind point
///   `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`. No draw commands can be called in the same
///   subpass. Only one [`cmd_subpass_shading_huawei`] command can be called in a subpass
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - This command  **must**  only be called inside of a render pass instance
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`huawei_subpass_shading`]
/// - [`CommandBuffer`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
pub type FNCmdSubpassShadingHuawei = Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;
///[VkSubpassShadingPipelineCreateInfoHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html) - Structure specifying parameters of a newly created subpass shading pipeline
///# C Specifications
///A subpass shading pipeline is a compute pipeline which  **must**  be called only
///in a subpass of a render pass with work dimensions specified by render area
///size.
///The subpass shading pipeline shader is a compute shader allowed to access
///input attachments specified in the calling subpass.
///To create a subpass shading pipeline, call [`create_compute_pipelines`]
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
/// - [`render_pass`] is a handle to a render pass object describing the environment in which the pipeline will be used. The pipeline  **must**  only be used with a render pass instance compatible with the one provided. See [Render Pass Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) for more information.
/// - [`subpass`] is the index of the subpass in the render pass where this pipeline will be used.
///# Description
///## Valid Usage
/// - [`subpass`] **must**  be created with `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI` bind
///   point
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI`
///# Related
/// - [`huawei_subpass_shading`]
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
#[doc(alias = "VkSubpassShadingPipelineCreateInfoHUAWEI")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct SubpassShadingPipelineCreateInfoHUAWEI<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`render_pass`] is a handle to a render pass object describing the
    ///environment in which the pipeline will be used.
    ///The pipeline  **must**  only be used with a render pass instance compatible
    ///with the one provided.
    ///See [Render Pass Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) for more
    ///information.
    pub render_pass: RenderPass,
    ///[`subpass`] is the index of the subpass in the render pass where this
    ///pipeline will be used.
    pub subpass: u32,
}
impl<'lt> Default for SubpassShadingPipelineCreateInfoHUAWEI<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI,
            p_next: std::ptr::null_mut(),
            render_pass: Default::default(),
            subpass: 0,
        }
    }
}
impl<'lt> SubpassShadingPipelineCreateInfoHUAWEI<'lt> {
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
    ///Gets the value of [`Self::render_pass`]
    pub fn render_pass(&self) -> RenderPass {
        self.render_pass
    }
    ///Gets the value of [`Self::subpass`]
    pub fn subpass(&self) -> u32 {
        self.subpass
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
    ///Gets a mutable reference to the value of [`Self::render_pass`]
    pub fn render_pass_mut(&mut self) -> &mut RenderPass {
        &mut self.render_pass
    }
    ///Gets a mutable reference to the value of [`Self::subpass`]
    pub fn subpass_mut(&mut self) -> &mut u32 {
        &mut self.subpass
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
    ///Sets the value of [`Self::render_pass`]
    pub fn set_render_pass(mut self, value: crate::vulkan1_0::RenderPass) -> Self {
        self.render_pass = value;
        self
    }
    ///Sets the value of [`Self::subpass`]
    pub fn set_subpass(mut self, value: u32) -> Self {
        self.subpass = value;
        self
    }
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
///   [`max_subpass_shading_workgroup_size_aspect_ratio`] **must**  be a power-of-two value, and
///   **must**  be less than or equal to max(`WorkgroupSize.x` / `WorkgroupSize.y`,
///   `WorkgroupSize.y` / `WorkgroupSize.x`).
///# Description
///If the [`PhysicalDeviceSubpassShadingPropertiesHUAWEI`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI`
///# Related
/// - [`huawei_subpass_shading`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceSubpassShadingPropertiesHUAWEI")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_subpass_shading_workgroup_size_aspect_ratio`] indicates the maximum
    ///ratio between the width and height of the portion of the subpass shading
    ///shader workgroup size.
    ///[`max_subpass_shading_workgroup_size_aspect_ratio`] **must**  be a power-of-two
    ///value, and  **must**  be less than or equal to max(`WorkgroupSize.x` /
    ///`WorkgroupSize.y`, `WorkgroupSize.y` / `WorkgroupSize.x`).
    pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
}
impl<'lt> Default for PhysicalDeviceSubpassShadingPropertiesHUAWEI<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI,
            p_next: std::ptr::null_mut(),
            max_subpass_shading_workgroup_size_aspect_ratio: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSubpassShadingPropertiesHUAWEI<'lt> {
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
    ///Gets the value of [`Self::max_subpass_shading_workgroup_size_aspect_ratio`]
    pub fn max_subpass_shading_workgroup_size_aspect_ratio(&self) -> u32 {
        self.max_subpass_shading_workgroup_size_aspect_ratio
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
    ///Gets a mutable reference to the value of
    /// [`Self::max_subpass_shading_workgroup_size_aspect_ratio`]
    pub fn max_subpass_shading_workgroup_size_aspect_ratio_mut(&mut self) -> &mut u32 {
        &mut self.max_subpass_shading_workgroup_size_aspect_ratio
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
    ///Sets the value of [`Self::max_subpass_shading_workgroup_size_aspect_ratio`]
    pub fn set_max_subpass_shading_workgroup_size_aspect_ratio(mut self, value: u32) -> Self {
        self.max_subpass_shading_workgroup_size_aspect_ratio = value;
        self
    }
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
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceSubpassShadingFeaturesHUAWEI`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI`
///# Related
/// - [`huawei_subpass_shading`]
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
#[doc(alias = "VkPhysicalDeviceSubpassShadingFeaturesHUAWEI")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`subpass_shading`] specifies whether
    ///subpass shading is supported.
    pub subpass_shading: Bool32,
}
impl<'lt> Default for PhysicalDeviceSubpassShadingFeaturesHUAWEI<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI,
            p_next: std::ptr::null_mut(),
            subpass_shading: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSubpassShadingFeaturesHUAWEI<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::subpass_shading`]
    pub fn subpass_shading_raw(&self) -> Bool32 {
        self.subpass_shading
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::subpass_shading`]
    pub fn set_subpass_shading_raw(mut self, value: Bool32) -> Self {
        self.subpass_shading = value;
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
    ///Gets the value of [`Self::subpass_shading`]
    pub fn subpass_shading(&self) -> bool {
        unsafe { std::mem::transmute(self.subpass_shading as u8) }
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
    ///Gets a mutable reference to the value of [`Self::subpass_shading`]
    pub fn subpass_shading_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.subpass_shading as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.subpass_shading as *mut Bool32)
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
    ///Sets the value of [`Self::subpass_shading`]
    pub fn set_subpass_shading(mut self, value: bool) -> Self {
        self.subpass_shading = value as u8 as u32;
        self
    }
}
impl Device {
    ///[vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html) - Query maximum supported subpass shading workgroup size for a give render pass
    ///# C Specifications
    ///A subpass shading pipeline’s workgroup size is a 2D vector with number of
    ///power-of-two in width and height.
    ///The maximum number of width and height is implementation dependent, and  **may**
    ///vary for different formats and sample counts of attachments in a render
    ///pass.To query the maximum workgroup size, call:
    ///```c
    ///// Provided by VK_HUAWEI_subpass_shading
    ///VkResult vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(
    ///    VkDevice                                    device,
    ///    VkRenderPass                                renderpass,
    ///    VkExtent2D*                                 pMaxWorkgroupSize);
    ///```
    ///# Parameters
    /// - [`device`] is a handle to a local device object that was used to create the given render
    ///   pass.
    /// - `renderPass` is a handle to a render pass object describing the environment in which the pipeline will be used. The pipeline  **must**  only be used with a render pass instance compatible with the one provided. See [Render Pass Compatibility](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-compatibility) for more information.
    /// - [`p_max_workgroup_size`] is a pointer to a [`Extent2D`] structure.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`renderpass`] **must**  be a valid [`RenderPass`] handle
    /// - [`p_max_workgroup_size`] **must**  be a valid pointer to a [`Extent2D`] structure
    /// - [`renderpass`] **must**  have been created, allocated, or retrieved from [`device`]
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  -
    ///   `VK_ERROR_SURFACE_LOST_KHR`
    ///# Related
    /// - [`huawei_subpass_shading`]
    /// - [`Device`]
    /// - [`Extent2D`]
    /// - [`RenderPass`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei(
        self: &Unique<Device>,
        renderpass: RenderPass,
    ) -> VulkanResult<Extent2D> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .huawei_subpass_shading()
            .and_then(|vtable| vtable.get_device_subpass_shading_max_workgroup_size_huawei())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .huawei_subpass_shading()
            .and_then(|vtable| vtable.get_device_subpass_shading_max_workgroup_size_huawei())
            .unwrap_unchecked();
        let mut p_max_workgroup_size = MaybeUninit::<Extent2D>::uninit();
        let _return = _function(self.as_raw(), renderpass, p_max_workgroup_size.as_mut_ptr());
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => {
                VulkanResult::Success(_return, p_max_workgroup_size.assume_init())
            },
            e => VulkanResult::Err(e),
        }
    }
}
impl CommandBuffer {
    ///[vkCmdSubpassShadingHUAWEI](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html) - Dispatch compute work items
    ///# C Specifications
    ///A subpass shading dispatches a compute pipeline work with the work dimension
    ///of render area of the calling subpass and work groups are partitioned by
    ///specified work group size.
    ///Subpass operations like subpassLoad and subpassLoadMS are allowed to be
    ///used.To record a subpass shading, call:
    ///```c
    ///// Provided by VK_HUAWEI_subpass_shading
    ///void vkCmdSubpassShadingHUAWEI(
    ///    VkCommandBuffer                             commandBuffer);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    ///# Description
    ///When the command is executed, a global workgroup consisting of ceil (render
    ///area size / local workgroup size) local workgroups is assembled.
    ///## Valid Usage
    /// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format
    ///   features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
    /// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
    /// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
    ///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified
    ///   by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
    ///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this
    ///   command  **must**  have a [`ImageViewType`] and format that supports cubic filtering
    ///   together with minmax filtering, as specified by
    ///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
    ///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**
    ///   only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
    /// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel
    ///   buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s
    ///   format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
    /// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
    ///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format
    ///   feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
    /// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind
    ///   point used by this command, a descriptor set  **must**  have been bound to *n* at the same
    ///   pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
    ///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
    ///   [[descriptorsets-compatibility]]()
    /// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
    ///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command,
    ///   a push constant value  **must**  have been set for the same pipeline bind point, with a
    ///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used
    ///   to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
    /// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
    ///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline
    ///   bind point used by this command
    /// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires
    ///   any dynamic state, that state  **must**  have been set or inherited (if the
    ///   `[`nv_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done
    ///   so after any previously bound pipeline with the corresponding state not specified as
    ///   dynamic
    /// - There  **must**  not have been any calls to dynamic state setting commands for any state
    ///   not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used
    ///   by this command, since that pipeline was bound
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used to sample from any [`Image`] with a [`ImageView`] of the type
    ///   `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`,
    ///   `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
    ///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that
    ///   includes a LOD bias or any offset values, in any shader stage
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a uniform buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a storage buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be a protected resource
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  only be used with `OpImageSample*` or
    ///   `OpImageSparseSample*` instructions
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
    /// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the image view’s format
    /// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the buffer view’s format
    /// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created
    ///   with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created
    ///   with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - This command must be called in a subpass with bind point
    ///   `VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`. No draw commands can be called in the
    ///   same subpass. Only one [`cmd_subpass_shading_huawei`] command can be called in a subpass
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - This command  **must**  only be called inside of a render pass instance
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`huawei_subpass_shading`]
    /// - [`CommandBuffer`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSubpassShadingHUAWEI")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_subpass_shading_huawei(self: &Unique<CommandBuffer>) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .huawei_subpass_shading()
            .and_then(|vtable| vtable.cmd_subpass_shading_huawei())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .huawei_subpass_shading()
            .and_then(|vtable| vtable.cmd_subpass_shading_huawei())
            .unwrap_unchecked();
        let _return = _function(self.as_raw());
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_HUAWEI_subpass_shading`
pub struct DeviceHuaweiSubpassShadingVTable {
    ///See [`FNGetDeviceSubpassShadingMaxWorkgroupSizeHuawei`] for more information.
    pub get_device_subpass_shading_max_workgroup_size_huawei: FNGetDeviceSubpassShadingMaxWorkgroupSizeHuawei,
    ///See [`FNCmdSubpassShadingHuawei`] for more information.
    pub cmd_subpass_shading_huawei: FNCmdSubpassShadingHuawei,
}
impl DeviceHuaweiSubpassShadingVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_device_subpass_shading_max_workgroup_size_huawei: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI").as_ptr(),
                ))
            },
            cmd_subpass_shading_huawei: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSubpassShadingHUAWEI").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_device_subpass_shading_max_workgroup_size_huawei`]. See
    /// [`FNGetDeviceSubpassShadingMaxWorkgroupSizeHuawei`] for more information.
    pub fn get_device_subpass_shading_max_workgroup_size_huawei(
        &self,
    ) -> FNGetDeviceSubpassShadingMaxWorkgroupSizeHuawei {
        self.get_device_subpass_shading_max_workgroup_size_huawei
    }
    ///Gets [`Self::cmd_subpass_shading_huawei`]. See [`FNCmdSubpassShadingHuawei`] for more
    /// information.
    pub fn cmd_subpass_shading_huawei(&self) -> FNCmdSubpassShadingHuawei {
        self.cmd_subpass_shading_huawei
    }
}
