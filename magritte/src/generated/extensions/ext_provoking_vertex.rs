use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_SPEC_VERSION")]
pub const EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME")]
pub const EXT_PROVOKING_VERTEX_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_provoking_vertex");
///[VkProvokingVertexModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkProvokingVertexModeEXT.html) - Specify which vertex in a primitive is the provoking vertex
///# C Specifications
///Possible values of
///[`PipelineRasterizationProvokingVertexStateCreateInfoEXT::provoking_vertex_mode`]
///are:
///```c
///// Provided by VK_EXT_provoking_vertex
///typedef enum VkProvokingVertexModeEXT {
///    VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT = 0,
///    VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT = 1,
///} VkProvokingVertexModeEXT;
///```
///# Description
/// - [`ProvokingVertexModeFirstVertexExt`] specifies that the provoking vertex is the first
///   non-adjacency vertex in the list of vertices used by a primitive.
/// - [`ProvokingVertexModeLastVertexExt`] specifies that the provoking vertex is the last
///   non-adjacency vertex in the list of vertices used by a primitive.
///These modes are described more precisely in
///[Primitive Topologies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-topologies).
///# Related
/// - [`VK_EXT_provoking_vertex`]
/// - [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkProvokingVertexModeEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ProvokingVertexModeEXT {
    ///[`ProvokingVertexModeFirstVertexExt`] specifies that the
    ///provoking vertex is the first non-adjacency vertex in the list of
    ///vertices used by a primitive.
    ProvokingVertexModeFirstVertexExt = 0,
    ///[`ProvokingVertexModeLastVertexExt`] specifies that the
    ///provoking vertex is the last non-adjacency vertex in the list of
    ///vertices used by a primitive.
    ProvokingVertexModeLastVertexExt = 1,
}
impl const Default for ProvokingVertexModeEXT {
    fn default() -> Self {
        ProvokingVertexModeFirstVertexExt
    }
}
impl ProvokingVertexModeEXT {
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
///[VkPhysicalDeviceProvokingVertexFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexFeaturesEXT.html) - Structure describing the provoking vertex features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceProvokingVertexFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_provoking_vertex
///typedef struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           provokingVertexLast;
///    VkBool32           transformFeedbackPreservesProvokingVertex;
///} VkPhysicalDeviceProvokingVertexFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`provoking_vertex_last`] indicates whether the implementation supports the
///   `VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT`[`ProvokingVertexModeEXT`] for flat shading.
/// - [`transform_feedback_preserves_provoking_vertex`] indicates that the order of vertices within each primitive written by transform feedback will preserve the provoking vertex. This does not apply to triangle fan primitives when [`transformFeedbackPreservesTriangleFanProvokingVertex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-transformFeedbackPreservesTriangleFanProvokingVertex) is [`FALSE`]. [`transform_feedback_preserves_provoking_vertex`]**must** be [`FALSE`] when the [`VK_EXT_transform_feedback`] extension is not supported.
///If the [`PhysicalDeviceProvokingVertexFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceProvokingVertexFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.When
/// [`PhysicalDeviceProvokingVertexFeaturesEXT`] is in the [`p_next`]
///chain of [`DeviceCreateInfo`] but the
///[transform feedback feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-transformFeedback) is not enabled,
///the value of [`transform_feedback_preserves_provoking_vertex`] is ignored.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT`
///# Related
/// - [`VK_EXT_provoking_vertex`]
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
pub struct PhysicalDeviceProvokingVertexFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`provoking_vertex_last`] indicates
    ///whether the implementation supports the
    ///`VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT`[`ProvokingVertexModeEXT`] for flat shading.
    provoking_vertex_last: Bool32,
    ///[`transform_feedback_preserves_provoking_vertex`] indicates that the order
    ///of vertices within each primitive written by transform feedback will
    ///preserve the provoking vertex.
    ///This does not apply to triangle fan primitives when
    ///[`transformFeedbackPreservesTriangleFanProvokingVertex`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-transformFeedbackPreservesTriangleFanProvokingVertex)
    ///is [`FALSE`].
    ///[`transform_feedback_preserves_provoking_vertex`]**must** be [`FALSE`]
    ///when the [`VK_EXT_transform_feedback`] extension is not supported.
    transform_feedback_preserves_provoking_vertex: Bool32,
}
///[VkPhysicalDeviceProvokingVertexPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProvokingVertexPropertiesEXT.html) - Structure describing provoking vertex properties supported by an implementation
///# C Specifications
///The [`PhysicalDeviceProvokingVertexPropertiesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_provoking_vertex
///typedef struct VkPhysicalDeviceProvokingVertexPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           provokingVertexModePerPipeline;
///    VkBool32           transformFeedbackPreservesTriangleFanProvokingVertex;
///} VkPhysicalDeviceProvokingVertexPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`provoking_vertex_mode_per_pipeline`] indicates whether the implementation supports graphics
///   pipelines with different provoking vertex modes within the same render pass instance.
/// - [`transform_feedback_preserves_triangle_fan_provoking_vertex`] indicates whether the
///   implementation can preserve the provoking vertex order when writing triangle fan vertices to
///   transform feedback.
///# Description
///If the [`PhysicalDeviceProvokingVertexPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_provoking_vertex`]
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
pub struct PhysicalDeviceProvokingVertexPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`provoking_vertex_mode_per_pipeline`] indicates whether the
    ///implementation supports graphics pipelines with different provoking
    ///vertex modes within the same render pass instance.
    provoking_vertex_mode_per_pipeline: Bool32,
    ///[`transform_feedback_preserves_triangle_fan_provoking_vertex`] indicates
    ///whether the implementation can preserve the provoking vertex order when
    ///writing triangle fan vertices to transform feedback.
    transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
}
///[VkPipelineRasterizationProvokingVertexStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html) - Structure specifying provoking vertex mode used by a graphics pipeline
///# C Specifications
///For a given primitive topology, the pipeline’s provoking vertex mode
///determines which vertex is the provoking vertex.
///To specify the provoking vertex mode, include a
///[`PipelineRasterizationProvokingVertexStateCreateInfoEXT`] structure in
///the [`PipelineRasterizationStateCreateInfo`]::[`p_next`] chain when
///creating the pipeline.The [`PipelineRasterizationProvokingVertexStateCreateInfoEXT`] structure
///is defined as:
///```c
///// Provided by VK_EXT_provoking_vertex
///typedef struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkProvokingVertexModeEXT    provokingVertexMode;
///} VkPipelineRasterizationProvokingVertexStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`provoking_vertex_mode`] is a [`ProvokingVertexModeEXT`] value selecting the provoking vertex
///   mode.
///# Description
///If this struct is not provided when creating the pipeline, the pipeline will
///use the `VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT` mode.If the
///[provokingVertexModePerPipeline](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-provokingVertexModePerPipeline)
///limit is [`FALSE`], then all pipelines bound within a render pass
///instance **must** have the same [`provoking_vertex_mode`].Valid Usage
/// - If [`provoking_vertex_mode`] is `VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT`, then the [provokingVertexLast](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-provokingVertexLast)
///   feature **must** be enabled
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT`
/// - [`provoking_vertex_mode`]**must** be a valid [`ProvokingVertexModeEXT`] value
///# Related
/// - [`VK_EXT_provoking_vertex`]
/// - [`ProvokingVertexModeEXT`]
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
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`provoking_vertex_mode`] is a [`ProvokingVertexModeEXT`] value
    ///selecting the provoking vertex mode.
    provoking_vertex_mode: ProvokingVertexModeEXT,
}
