use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DeviceSize, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_TRANSFORM_FEEDBACK_SPEC_VERSION")]
pub const EXT_TRANSFORM_FEEDBACK_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME")]
pub const EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_transform_feedback");
///[VkPhysicalDeviceTransformFeedbackFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html) - Structure describing transform feedback features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceTransformFeedbackFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_transform_feedback
///typedef struct VkPhysicalDeviceTransformFeedbackFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           transformFeedback;
///    VkBool32           geometryStreams;
///} VkPhysicalDeviceTransformFeedbackFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`transform_feedback`] indicates whether the implementation supports transform feedback and
///   shader modules **can** declare the `TransformFeedback` capability.
/// - [`geometry_streams`] indicates whether the implementation supports the `GeometryStreams`
///   SPIR-V capability.
///If the [`PhysicalDeviceTransformFeedbackFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceTransformFeedbackFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT`
///# Related
/// - [`VK_EXT_transform_feedback`]
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
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`transform_feedback`] indicates whether
    ///the implementation supports transform feedback and shader modules **can**
    ///declare the `TransformFeedback` capability.
    transform_feedback: Bool32,
    ///[`geometry_streams`] indicates whether the
    ///implementation supports the `GeometryStreams` SPIR-V capability.
    geometry_streams: Bool32,
}
impl<'lt> Default for PhysicalDeviceTransformFeedbackFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            transform_feedback: 0,
            geometry_streams: 0,
        }
    }
}
impl<'lt> PhysicalDeviceTransformFeedbackFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::transform_feedback`]
    pub fn transform_feedback_raw(&self) -> Bool32 {
        self.transform_feedback
    }
    ///Gets the raw value of [`Self::geometry_streams`]
    pub fn geometry_streams_raw(&self) -> Bool32 {
        self.geometry_streams
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback`]
    pub fn set_transform_feedback_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback = value;
        self
    }
    ///Sets the raw value of [`Self::geometry_streams`]
    pub fn set_geometry_streams_raw(&mut self, value: Bool32) -> &mut Self {
        self.geometry_streams = value;
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
    ///Gets the value of [`Self::transform_feedback`]
    pub fn transform_feedback(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback as u8) }
    }
    ///Gets the value of [`Self::geometry_streams`]
    pub fn geometry_streams(&self) -> bool {
        unsafe { std::mem::transmute(self.geometry_streams as u8) }
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
    ///Gets a mutable reference to the value of [`Self::transform_feedback`]
    pub fn transform_feedback_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::geometry_streams`]
    pub fn geometry_streams_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.geometry_streams as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.geometry_streams as *mut Bool32)
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
    ///Sets the raw value of [`Self::transform_feedback`]
    pub fn set_transform_feedback(&mut self, value: bool) -> &mut Self {
        self.transform_feedback = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::geometry_streams`]
    pub fn set_geometry_streams(&mut self, value: bool) -> &mut Self {
        self.geometry_streams = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceTransformFeedbackPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html) - Structure describing transform feedback properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceTransformFeedbackPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_transform_feedback
///typedef struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxTransformFeedbackStreams;
///    uint32_t           maxTransformFeedbackBuffers;
///    VkDeviceSize       maxTransformFeedbackBufferSize;
///    uint32_t           maxTransformFeedbackStreamDataSize;
///    uint32_t           maxTransformFeedbackBufferDataSize;
///    uint32_t           maxTransformFeedbackBufferDataStride;
///    VkBool32           transformFeedbackQueries;
///    VkBool32           transformFeedbackStreamsLinesTriangles;
///    VkBool32           transformFeedbackRasterizationStreamSelect;
///    VkBool32           transformFeedbackDraw;
///} VkPhysicalDeviceTransformFeedbackPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_transform_feedback_streams`] is the maximum number of vertex streams that can be output
///   from geometry shaders declared with the `GeometryStreams` capability. If the implementation
///   does not support [`PhysicalDeviceTransformFeedbackFeaturesEXT::geometry_streams`] then
///   [`max_transform_feedback_streams`]**must** be set to `1`.
/// - [`max_transform_feedback_buffers`] is the maximum number of transform feedback buffers that can be bound for capturing shader outputs from the last [pre-rasterization shader stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
/// - [`max_transform_feedback_buffer_size`] is the maximum size that can be specified when binding
///   a buffer for transform feedback in [`CmdBindTransformFeedbackBuffersEXT`].
/// - [`max_transform_feedback_stream_data_size`] is the maximum amount of data in bytes for each
///   vertex that captured to one or more transform feedback buffers associated with a specific
///   vertex stream.
/// - [`max_transform_feedback_buffer_data_size`] is the maximum amount of data in bytes for each
///   vertex that can be captured to a specific transform feedback buffer.
/// - [`max_transform_feedback_buffer_data_stride`] is the maximum stride between each capture of
///   vertex data to the buffer.
/// - [`transform_feedback_queries`] is [`TRUE`] if the implementation supports the
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` query type. [`transform_feedback_queries`] is
///   [`FALSE`] if queries of this type **cannot** be created.
/// - [`transform_feedback_streams_lines_triangles`] is [`TRUE`] if the implementation supports the
///   geometry shader `OpExecutionMode` of `OutputLineStrip` and `OutputTriangleStrip` in addition
///   to `OutputPoints` when more than one vertex stream is output. If
///   [`transform_feedback_streams_lines_triangles`] is [`FALSE`] the implementation only supports
///   an `OpExecutionMode` of `OutputPoints` when more than one vertex stream is output from the
///   geometry shader.
/// - [`transform_feedback_rasterization_stream_select`] is [`TRUE`] if the implementation supports
///   the `GeometryStreams` SPIR-V capability and the application can use
///   [`PipelineRasterizationStateStreamCreateInfoEXT`] to modify which vertex stream output is used
///   for rasterization. Otherwise vertex stream `0`**must** always be used for rasterization.
/// - [`transform_feedback_draw`] is [`TRUE`] if the implementation supports the
///   [`CmdDrawIndirectByteCountEXT`] function otherwise the function **must** not be called.
///# Description
///If the [`PhysicalDeviceTransformFeedbackPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_transform_feedback`]
/// - [`Bool32`]
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`max_transform_feedback_streams`]
    ///is the maximum number of vertex streams that can be output from geometry
    ///shaders declared with the `GeometryStreams` capability.
    ///If the implementation does not support
    ///[`PhysicalDeviceTransformFeedbackFeaturesEXT`]::`geometryStreams`
    ///then [`max_transform_feedback_streams`]**must** be set to `1`.
    max_transform_feedback_streams: u32,
    ///[`max_transform_feedback_buffers`]
    ///is the maximum number of transform feedback buffers that can be bound
    ///for capturing shader outputs from the last
    ///[pre-rasterization shader
    ///stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
    max_transform_feedback_buffers: u32,
    ///[`max_transform_feedback_buffer_size`] is the maximum size that can be
    ///specified when binding a buffer for transform feedback in
    ///[`CmdBindTransformFeedbackBuffersEXT`].
    max_transform_feedback_buffer_size: DeviceSize,
    ///[`max_transform_feedback_stream_data_size`] is the maximum amount of data
    ///in bytes for each vertex that captured to one or more transform feedback
    ///buffers associated with a specific vertex stream.
    max_transform_feedback_stream_data_size: u32,
    ///[`max_transform_feedback_buffer_data_size`] is the maximum amount of data
    ///in bytes for each vertex that can be captured to a specific transform
    ///feedback buffer.
    max_transform_feedback_buffer_data_size: u32,
    ///[`max_transform_feedback_buffer_data_stride`] is the maximum stride between
    ///each capture of vertex data to the buffer.
    max_transform_feedback_buffer_data_stride: u32,
    ///[`transform_feedback_queries`] is
    ///[`TRUE`] if the implementation supports the
    ///`VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` query type.
    ///[`transform_feedback_queries`] is [`FALSE`] if queries of this type
    ///**cannot** be created.
    transform_feedback_queries: Bool32,
    ///[`transform_feedback_streams_lines_triangles`] is [`TRUE`] if the
    ///implementation supports the geometry shader `OpExecutionMode` of
    ///`OutputLineStrip` and `OutputTriangleStrip` in addition to
    ///`OutputPoints` when more than one vertex stream is output.
    ///If [`transform_feedback_streams_lines_triangles`] is [`FALSE`] the
    ///implementation only supports an `OpExecutionMode` of
    ///`OutputPoints` when more than one vertex stream is output from the
    ///geometry shader.
    transform_feedback_streams_lines_triangles: Bool32,
    ///[`transform_feedback_rasterization_stream_select`] is [`TRUE`] if the
    ///implementation supports the `GeometryStreams` SPIR-V capability and
    ///the application can use
    ///[`PipelineRasterizationStateStreamCreateInfoEXT`] to modify which
    ///vertex stream output is used for rasterization.
    ///Otherwise vertex stream `0`**must** always be used for rasterization.
    transform_feedback_rasterization_stream_select: Bool32,
    ///[`transform_feedback_draw`] is
    ///[`TRUE`] if the implementation supports the
    ///[`CmdDrawIndirectByteCountEXT`] function otherwise the function
    ///**must** not be called.
    transform_feedback_draw: Bool32,
}
impl<'lt> Default for PhysicalDeviceTransformFeedbackPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            max_transform_feedback_streams: 0,
            max_transform_feedback_buffers: 0,
            max_transform_feedback_buffer_size: Default::default(),
            max_transform_feedback_stream_data_size: 0,
            max_transform_feedback_buffer_data_size: 0,
            max_transform_feedback_buffer_data_stride: 0,
            transform_feedback_queries: 0,
            transform_feedback_streams_lines_triangles: 0,
            transform_feedback_rasterization_stream_select: 0,
            transform_feedback_draw: 0,
        }
    }
}
impl<'lt> PhysicalDeviceTransformFeedbackPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::max_transform_feedback_streams`]
    pub fn max_transform_feedback_streams_raw(&self) -> u32 {
        self.max_transform_feedback_streams
    }
    ///Gets the raw value of [`Self::max_transform_feedback_buffers`]
    pub fn max_transform_feedback_buffers_raw(&self) -> u32 {
        self.max_transform_feedback_buffers
    }
    ///Gets the raw value of [`Self::max_transform_feedback_stream_data_size`]
    pub fn max_transform_feedback_stream_data_size_raw(&self) -> u32 {
        self.max_transform_feedback_stream_data_size
    }
    ///Gets the raw value of [`Self::max_transform_feedback_buffer_data_size`]
    pub fn max_transform_feedback_buffer_data_size_raw(&self) -> u32 {
        self.max_transform_feedback_buffer_data_size
    }
    ///Gets the raw value of [`Self::max_transform_feedback_buffer_data_stride`]
    pub fn max_transform_feedback_buffer_data_stride_raw(&self) -> u32 {
        self.max_transform_feedback_buffer_data_stride
    }
    ///Gets the raw value of [`Self::transform_feedback_queries`]
    pub fn transform_feedback_queries_raw(&self) -> Bool32 {
        self.transform_feedback_queries
    }
    ///Gets the raw value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn transform_feedback_streams_lines_triangles_raw(&self) -> Bool32 {
        self.transform_feedback_streams_lines_triangles
    }
    ///Gets the raw value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn transform_feedback_rasterization_stream_select_raw(&self) -> Bool32 {
        self.transform_feedback_rasterization_stream_select
    }
    ///Gets the raw value of [`Self::transform_feedback_draw`]
    pub fn transform_feedback_draw_raw(&self) -> Bool32 {
        self.transform_feedback_draw
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_streams`]
    pub fn set_max_transform_feedback_streams_raw(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_streams = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_buffers`]
    pub fn set_max_transform_feedback_buffers_raw(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_stream_data_size`]
    pub fn set_max_transform_feedback_stream_data_size_raw(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_stream_data_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_buffer_data_size`]
    pub fn set_max_transform_feedback_buffer_data_size_raw(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_buffer_data_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_buffer_data_stride`]
    pub fn set_max_transform_feedback_buffer_data_stride_raw(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_buffer_data_stride = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_queries`]
    pub fn set_transform_feedback_queries_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_queries = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn set_transform_feedback_streams_lines_triangles_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_streams_lines_triangles = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn set_transform_feedback_rasterization_stream_select_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_rasterization_stream_select = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_draw`]
    pub fn set_transform_feedback_draw_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_draw = value;
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
    ///Gets the value of [`Self::max_transform_feedback_streams`]
    pub fn max_transform_feedback_streams(&self) -> u32 {
        self.max_transform_feedback_streams
    }
    ///Gets the value of [`Self::max_transform_feedback_buffers`]
    pub fn max_transform_feedback_buffers(&self) -> u32 {
        self.max_transform_feedback_buffers
    }
    ///Gets the value of [`Self::max_transform_feedback_buffer_size`]
    pub fn max_transform_feedback_buffer_size(&self) -> DeviceSize {
        self.max_transform_feedback_buffer_size
    }
    ///Gets the value of [`Self::max_transform_feedback_stream_data_size`]
    pub fn max_transform_feedback_stream_data_size(&self) -> u32 {
        self.max_transform_feedback_stream_data_size
    }
    ///Gets the value of [`Self::max_transform_feedback_buffer_data_size`]
    pub fn max_transform_feedback_buffer_data_size(&self) -> u32 {
        self.max_transform_feedback_buffer_data_size
    }
    ///Gets the value of [`Self::max_transform_feedback_buffer_data_stride`]
    pub fn max_transform_feedback_buffer_data_stride(&self) -> u32 {
        self.max_transform_feedback_buffer_data_stride
    }
    ///Gets the value of [`Self::transform_feedback_queries`]
    pub fn transform_feedback_queries(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_queries as u8) }
    }
    ///Gets the value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn transform_feedback_streams_lines_triangles(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_streams_lines_triangles as u8) }
    }
    ///Gets the value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn transform_feedback_rasterization_stream_select(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_rasterization_stream_select as u8) }
    }
    ///Gets the value of [`Self::transform_feedback_draw`]
    pub fn transform_feedback_draw(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_draw as u8) }
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
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_streams`]
    pub fn max_transform_feedback_streams_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_buffers`]
    pub fn max_transform_feedback_buffers_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_buffer_size`]
    pub fn max_transform_feedback_buffer_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.max_transform_feedback_buffer_size
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_stream_data_size`]
    pub fn max_transform_feedback_stream_data_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_buffer_data_size`]
    pub fn max_transform_feedback_buffer_data_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_buffer_data_stride`]
    pub fn max_transform_feedback_buffer_data_stride_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::transform_feedback_queries`]
    pub fn transform_feedback_queries_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_queries as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_queries as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::transform_feedback_streams_lines_triangles`]
    pub fn transform_feedback_streams_lines_triangles_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_streams_lines_triangles as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_streams_lines_triangles as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::transform_feedback_rasterization_stream_select`]
    pub fn transform_feedback_rasterization_stream_select_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_rasterization_stream_select as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_rasterization_stream_select as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::transform_feedback_draw`]
    pub fn transform_feedback_draw_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_draw as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_draw as *mut Bool32)
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
    ///Sets the raw value of [`Self::max_transform_feedback_streams`]
    pub fn set_max_transform_feedback_streams(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_streams = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_buffers`]
    pub fn set_max_transform_feedback_buffers(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_buffers = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_buffer_size`]
    pub fn set_max_transform_feedback_buffer_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.max_transform_feedback_buffer_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_stream_data_size`]
    pub fn set_max_transform_feedback_stream_data_size(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_stream_data_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_buffer_data_size`]
    pub fn set_max_transform_feedback_buffer_data_size(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_buffer_data_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_transform_feedback_buffer_data_stride`]
    pub fn set_max_transform_feedback_buffer_data_stride(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_buffer_data_stride = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_queries`]
    pub fn set_transform_feedback_queries(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_queries = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn set_transform_feedback_streams_lines_triangles(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_streams_lines_triangles = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn set_transform_feedback_rasterization_stream_select(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_rasterization_stream_select = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_draw`]
    pub fn set_transform_feedback_draw(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_draw = value as u8 as u32;
        self
    }
}
///[VkPipelineRasterizationStateStreamCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html) - Structure defining the geometry stream used for rasterization
///# C Specifications
///The vertex stream used for rasterization is specified by adding a
///[`PipelineRasterizationStateStreamCreateInfoEXT`] structure to the
///[`p_next`] chain of a [`PipelineRasterizationStateCreateInfo`]
///structure.The [`PipelineRasterizationStateStreamCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_transform_feedback
///typedef struct VkPipelineRasterizationStateStreamCreateInfoEXT {
///    VkStructureType                                     sType;
///    const void*                                         pNext;
///    VkPipelineRasterizationStateStreamCreateFlagsEXT    flags;
///    uint32_t                                            rasterizationStream;
///} VkPipelineRasterizationStateStreamCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`rasterization_stream`] is the vertex stream selected for rasterization.
///# Description
///If this structure is not present, [`rasterization_stream`] is assumed to be
///zero.Valid Usage
/// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::geometry_streams`]**must** be enabled
/// - [`rasterization_stream`]**must** be less than
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_streams`]
/// - [`rasterization_stream`]**must** be zero if
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::
///   transform_feedback_rasterization_stream_select`] is [`FALSE`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT`
/// - [`flags`]**must** be `0`
///# Related
/// - [`VK_EXT_transform_feedback`]
/// - [`PipelineRasterizationStateStreamCreateFlagsEXT`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    flags: PipelineRasterizationStateStreamCreateFlagsEXT,
    ///[`rasterization_stream`] is the vertex stream selected for
    ///rasterization.
    rasterization_stream: u32,
}
impl<'lt> Default for PipelineRasterizationStateStreamCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            rasterization_stream: 0,
        }
    }
}
impl<'lt> PipelineRasterizationStateStreamCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::rasterization_stream`]
    pub fn rasterization_stream_raw(&self) -> u32 {
        self.rasterization_stream
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_stream`]
    pub fn set_rasterization_stream_raw(&mut self, value: u32) -> &mut Self {
        self.rasterization_stream = value;
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
    pub fn flags(&self) -> PipelineRasterizationStateStreamCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::rasterization_stream`]
    pub fn rasterization_stream(&self) -> u32 {
        self.rasterization_stream
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineRasterizationStateStreamCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::rasterization_stream`]
    pub fn rasterization_stream_mut(&mut self) -> &mut u32 {
        &mut getter
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
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::rasterization_stream`]
    pub fn set_rasterization_stream(&mut self, value: u32) -> &mut Self {
        self.rasterization_stream = value;
        self
    }
}
