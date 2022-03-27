use crate::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, Extent2D, SampleCountFlagBits, SampleCountFlags, StructureType,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION")]
pub const EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME")]
pub const EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_sample_locations");
///[VkSampleLocationEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleLocationEXT.html) - Structure specifying the coordinates of a sample location
///# C Specifications
///The [`SampleLocationEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_sample_locations
///typedef struct VkSampleLocationEXT {
///    float    x;
///    float    y;
///} VkSampleLocationEXT;
///```
///# Members
/// - [`x`] is the horizontal coordinate of the sample’s location.
/// - [`y`] is the vertical coordinate of the sample’s location.
///# Description
///The domain space of the sample location coordinates has an upper-left origin
///within the pixel in framebuffer space.The values specified in a [`SampleLocationEXT`] structure
/// are always
///clamped to the implementation-dependent sample location coordinate range
///[`sampleLocationCoordinateRange`[0],`sampleLocationCoordinateRange`[1]]
///that **can** be queried using
///[`PhysicalDeviceSampleLocationsPropertiesEXT`].
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`SampleLocationsInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SampleLocationEXT {
    ///[`x`] is the horizontal coordinate of the sample’s location.
    x: f32,
    ///[`y`] is the vertical coordinate of the sample’s location.
    y: f32,
}
///[VkSampleLocationsInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleLocationsInfoEXT.html) - Structure specifying a set of sample locations
///# C Specifications
///The [`SampleLocationsInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_sample_locations
///typedef struct VkSampleLocationsInfoEXT {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkSampleCountFlagBits         sampleLocationsPerPixel;
///    VkExtent2D                    sampleLocationGridSize;
///    uint32_t                      sampleLocationsCount;
///    const VkSampleLocationEXT*    pSampleLocations;
///} VkSampleLocationsInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sample_locations_per_pixel`] is a [`SampleCountFlagBits`] value specifying the number of
///   sample locations per pixel.
/// - [`sample_location_grid_size`] is the size of the sample location grid to select custom sample
///   locations for.
/// - [`sample_locations_count`] is the number of sample locations in [`p_sample_locations`].
/// - [`p_sample_locations`] is a pointer to an array of
///   [`sample_locations_count`][`SampleLocationEXT`] structures.
///# Description
///This structure **can** be used either to specify the sample locations to be
///used for rendering or to specify the set of sample locations an image
///subresource has been last rendered with for the purposes of layout
///transitions of depth/stencil images created with
///`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT`.The sample locations in
/// [`p_sample_locations`] specify
///[`sample_locations_per_pixel`] number of sample locations for each pixel in
///the grid of the size specified in [`sample_location_grid_size`].
///The sample location for sample i at the pixel grid location
///(x,y) is taken from [`p_sample_locations`][(x +  y ×
///`sampleLocationGridSize.width`) × [`sample_locations_per_pixel`]
///+  i].If the render pass has a fragment density map, the implementation will
///choose the sample locations for the fragment and the contents of
///[`p_sample_locations`]**may** be ignored.Valid Usage
/// - [`sample_locations_per_pixel`]**must** be a bit value that is set in
///   [`PhysicalDeviceSampleLocationsPropertiesEXT::sample_location_sample_counts`]
/// - [`sample_locations_count`]**must** equal [`sample_locations_per_pixel`] ×
///   `sampleLocationGridSize.width` × `sampleLocationGridSize.height`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT`
/// - If [`sample_locations_count`] is not `0`, [`p_sample_locations`]**must** be a valid pointer to
///   an array of [`sample_locations_count`][`SampleLocationEXT`] structures
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`AttachmentSampleLocationsEXT`]
/// - [`Extent2D`]
/// - [`PipelineSampleLocationsStateCreateInfoEXT`]
/// - [`SampleCountFlagBits`]
/// - [`SampleLocationEXT`]
/// - [`StructureType`]
/// - [`SubpassSampleLocationsEXT`]
/// - [`CmdSetSampleLocationsEXT`]
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
pub struct SampleLocationsInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`sample_locations_per_pixel`] is a [`SampleCountFlagBits`] value
    ///specifying the number of sample locations per pixel.
    sample_locations_per_pixel: SampleCountFlagBits,
    ///[`sample_location_grid_size`] is the size of the sample location grid to
    ///select custom sample locations for.
    sample_location_grid_size: Extent2D,
    ///[`sample_locations_count`] is the number of sample locations in
    ///[`p_sample_locations`].
    sample_locations_count: u32,
    ///[`p_sample_locations`] is a pointer to an array of
    ///[`sample_locations_count`][`SampleLocationEXT`] structures.
    p_sample_locations: *mut SampleLocationEXT,
}
///[VkAttachmentSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleLocationsEXT.html) - Structure specifying the sample locations state to use in the initial layout transition of attachments
///# C Specifications
///The [`AttachmentSampleLocationsEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_sample_locations
///typedef struct VkAttachmentSampleLocationsEXT {
///    uint32_t                    attachmentIndex;
///    VkSampleLocationsInfoEXT    sampleLocationsInfo;
///} VkAttachmentSampleLocationsEXT;
///```
///# Members
/// - [`attachment_index`] is the index of the attachment for which the sample locations state is
///   provided.
/// - [`sample_locations_info`] is the sample locations state to use for the layout transition of
///   the given attachment from the initial layout of the attachment to the image layout specified
///   for the attachment in the first subpass using it.
///# Description
///If the image referenced by the framebuffer attachment at index
///[`attachment_index`] was not created with
///`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` then the
///values specified in [`sample_locations_info`] are ignored.Valid Usage
/// - [`attachment_index`]**must** be less than the `attachmentCount` specified in
///   [`RenderPassCreateInfo`] the render pass specified by [`RenderPassBeginInfo::render_pass`] was
///   created with
///Valid Usage (Implicit)
/// - [`sample_locations_info`]**must** be a valid [`SampleLocationsInfoEXT`] structure
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`RenderPassSampleLocationsBeginInfoEXT`]
/// - [`SampleLocationsInfoEXT`]
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
pub struct AttachmentSampleLocationsEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`attachment_index`] is the index of the attachment for which the
    ///sample locations state is provided.
    attachment_index: u32,
    ///[`sample_locations_info`] is the sample locations state to use for the
    ///layout transition of the given attachment from the initial layout of the
    ///attachment to the image layout specified for the attachment in the first
    ///subpass using it.
    sample_locations_info: SampleLocationsInfoEXT<'lt>,
}
///[VkSubpassSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubpassSampleLocationsEXT.html) - Structure specifying the sample locations state to use for layout transitions of attachments performed after a given subpass
///# C Specifications
///The [`SubpassSampleLocationsEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_sample_locations
///typedef struct VkSubpassSampleLocationsEXT {
///    uint32_t                    subpassIndex;
///    VkSampleLocationsInfoEXT    sampleLocationsInfo;
///} VkSubpassSampleLocationsEXT;
///```
///# Members
/// - [`subpass_index`] is the index of the subpass for which the sample locations state is
///   provided.
/// - [`sample_locations_info`] is the sample locations state to use for the layout transition of
///   the depth/stencil attachment away from the image layout the attachment is used with in the
///   subpass specified in [`subpass_index`].
///# Description
///If the image referenced by the depth/stencil attachment used in the subpass
///identified by [`subpass_index`] was not created with
///`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` or if the
///subpass does not use a depth/stencil attachment, and
///[`PhysicalDeviceSampleLocationsPropertiesEXT::variable_sample_locations`]
///is [`TRUE`] then the values specified in [`sample_locations_info`] are
///ignored.Valid Usage
/// - [`subpass_index`]**must** be less than the `subpassCount` specified in
///   [`RenderPassCreateInfo`] the render pass specified by [`RenderPassBeginInfo::render_pass`] was
///   created with
///Valid Usage (Implicit)
/// - [`sample_locations_info`]**must** be a valid [`SampleLocationsInfoEXT`] structure
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`RenderPassSampleLocationsBeginInfoEXT`]
/// - [`SampleLocationsInfoEXT`]
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
pub struct SubpassSampleLocationsEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`subpass_index`] is the index of the subpass for which the sample
    ///locations state is provided.
    subpass_index: u32,
    ///[`sample_locations_info`] is the sample locations state to use for the
    ///layout transition of the depth/stencil attachment away from the image
    ///layout the attachment is used with in the subpass specified in
    ///[`subpass_index`].
    sample_locations_info: SampleLocationsInfoEXT<'lt>,
}
///[VkRenderPassSampleLocationsBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html) - Structure specifying sample locations to use for the layout transition of custom sample locations compatible depth/stencil attachments
///# C Specifications
///The image layout of the depth aspect of a depth/stencil attachment referring
///to an image created with
///`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` is dependent
///on the last sample locations used to render to the image subresource, thus
///preserving the contents of such depth/stencil attachments across subpass
///boundaries requires the application to specify these sample locations
///whenever a layout transition of the attachment **may** occur.
///This information **can** be provided by adding a
///[`RenderPassSampleLocationsBeginInfoEXT`] structure to the [`p_next`]
///chain of [`RenderPassBeginInfo`].The [`RenderPassSampleLocationsBeginInfoEXT`] structure is
/// defined as:
///```c
///// Provided by VK_EXT_sample_locations
///typedef struct VkRenderPassSampleLocationsBeginInfoEXT {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    uint32_t                                 attachmentInitialSampleLocationsCount;
///    const VkAttachmentSampleLocationsEXT*    pAttachmentInitialSampleLocations;
///    uint32_t                                 postSubpassSampleLocationsCount;
///    const VkSubpassSampleLocationsEXT*       pPostSubpassSampleLocations;
///} VkRenderPassSampleLocationsBeginInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`attachment_initial_sample_locations_count`] is the number of elements in the
///   [`p_attachment_initial_sample_locations`] array.
/// - [`p_attachment_initial_sample_locations`] is a pointer to an array of
///   [`attachment_initial_sample_locations_count`][`AttachmentSampleLocationsEXT`] structures
///   specifying the attachment indices and their corresponding sample location state. Each element
///   of [`p_attachment_initial_sample_locations`]**can** specify the sample location state to use
///   in the automatic layout transition performed to transition a depth/stencil attachment from the
///   initial layout of the attachment to the image layout specified for the attachment in the first
///   subpass using it.
/// - [`post_subpass_sample_locations_count`] is the number of elements in the
///   [`p_post_subpass_sample_locations`] array.
/// - [`p_post_subpass_sample_locations`] is a pointer to an array of
///   [`post_subpass_sample_locations_count`][`SubpassSampleLocationsEXT`] structures specifying the
///   subpass indices and their corresponding sample location state. Each element of
///   [`p_post_subpass_sample_locations`]**can** specify the sample location state to use in the
///   automatic layout transition performed to transition the depth/stencil attachment used by the
///   specified subpass to the image layout specified in a dependent subpass or to the final layout
///   of the attachment in case the specified subpass is the last subpass using that attachment. In
///   addition, if [`PhysicalDeviceSampleLocationsPropertiesEXT::variable_sample_locations`] is
///   [`FALSE`], each element of [`p_post_subpass_sample_locations`]**must** specify the sample
///   location state that matches the sample locations used by all pipelines that will be bound to a
///   command buffer during the specified subpass. If `variableSampleLocations` is [`TRUE`], the
///   sample locations used for rasterization do not depend on [`p_post_subpass_sample_locations`].
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT`
/// - If [`attachment_initial_sample_locations_count`] is not `0`,
///   [`p_attachment_initial_sample_locations`]**must** be a valid pointer to an array of
///   [`attachment_initial_sample_locations_count`] valid [`AttachmentSampleLocationsEXT`]
///   structures
/// - If [`post_subpass_sample_locations_count`] is not `0`,
///   [`p_post_subpass_sample_locations`]**must** be a valid pointer to an array of
///   [`post_subpass_sample_locations_count`] valid [`SubpassSampleLocationsEXT`] structures
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`AttachmentSampleLocationsEXT`]
/// - [`StructureType`]
/// - [`SubpassSampleLocationsEXT`]
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
pub struct RenderPassSampleLocationsBeginInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`attachment_initial_sample_locations_count`] is the number of elements in
    ///the [`p_attachment_initial_sample_locations`] array.
    attachment_initial_sample_locations_count: u32,
    ///[`p_attachment_initial_sample_locations`] is a pointer to an array of
    ///[`attachment_initial_sample_locations_count`][`AttachmentSampleLocationsEXT`] structures
    /// specifying the attachment indices and their corresponding sample location state.
    ///Each element of [`p_attachment_initial_sample_locations`]**can** specify the
    ///sample location state to use in the automatic layout transition
    ///performed to transition a depth/stencil attachment from the initial
    ///layout of the attachment to the image layout specified for the
    ///attachment in the first subpass using it.
    p_attachment_initial_sample_locations: *mut AttachmentSampleLocationsEXT<'lt>,
    ///[`post_subpass_sample_locations_count`] is the number of elements in the
    ///[`p_post_subpass_sample_locations`] array.
    post_subpass_sample_locations_count: u32,
    ///[`p_post_subpass_sample_locations`] is a pointer to an array of
    ///[`post_subpass_sample_locations_count`][`SubpassSampleLocationsEXT`]
    ///structures specifying the subpass indices and their corresponding sample
    ///location state.
    ///Each element of [`p_post_subpass_sample_locations`]**can** specify the
    ///sample location state to use in the automatic layout transition
    ///performed to transition the depth/stencil attachment used by the
    ///specified subpass to the image layout specified in a dependent subpass
    ///or to the final layout of the attachment in case the specified subpass
    ///is the last subpass using that attachment.
    ///In addition, if
    ///[`PhysicalDeviceSampleLocationsPropertiesEXT`]::`variableSampleLocations`
    ///is [`FALSE`], each element of [`p_post_subpass_sample_locations`]**must** specify the sample
    /// location state that matches the sample locations used by all pipelines that will be
    /// bound to a command buffer during the specified subpass.
    ///If `variableSampleLocations` is [`TRUE`], the sample locations
    ///used for rasterization do not depend on
    ///[`p_post_subpass_sample_locations`].
    p_post_subpass_sample_locations: *mut SubpassSampleLocationsEXT<'lt>,
}
///[VkPipelineSampleLocationsStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html) - Structure specifying sample locations for a pipeline
///# C Specifications
///Applications **can** also control the sample locations used for rasterization.If the [`p_next`]
/// chain of the [`PipelineMultisampleStateCreateInfo`]
///structure specified at pipeline creation time includes a
///[`PipelineSampleLocationsStateCreateInfoEXT`] structure, then that
///structure controls the sample locations used when rasterizing primitives
///with the pipeline.The [`PipelineSampleLocationsStateCreateInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_sample_locations
///typedef struct VkPipelineSampleLocationsStateCreateInfoEXT {
///    VkStructureType             sType;
///    const void*                 pNext;
///    VkBool32                    sampleLocationsEnable;
///    VkSampleLocationsInfoEXT    sampleLocationsInfo;
///} VkPipelineSampleLocationsStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sample_locations_enable`] controls whether custom sample locations are used. If
///   [`sample_locations_enable`] is [`FALSE`], the default sample locations are used and the values
///   specified in [`sample_locations_info`] are ignored.
/// - [`sample_locations_info`] is the sample locations to use during rasterization if
///   [`sample_locations_enable`] is [`TRUE`] and the graphics pipeline is not created with
///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT`.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT`
/// - [`sample_locations_info`]**must** be a valid [`SampleLocationsInfoEXT`] structure
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`Bool32`]
/// - [`SampleLocationsInfoEXT`]
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
pub struct PipelineSampleLocationsStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`sample_locations_enable`] controls whether custom sample locations are
    ///used.
    ///If [`sample_locations_enable`] is [`FALSE`], the default sample
    ///locations are used and the values specified in [`sample_locations_info`]
    ///are ignored.
    sample_locations_enable: Bool32,
    ///[`sample_locations_info`] is the sample locations to use during
    ///rasterization if [`sample_locations_enable`] is [`TRUE`] and the
    ///graphics pipeline is not created with
    ///`VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT`.
    sample_locations_info: SampleLocationsInfoEXT<'lt>,
}
///[VkPhysicalDeviceSampleLocationsPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html) - Structure describing sample location limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceSampleLocationsPropertiesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_sample_locations
///typedef struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
///    VkStructureType       sType;
///    void*                 pNext;
///    VkSampleCountFlags    sampleLocationSampleCounts;
///    VkExtent2D            maxSampleLocationGridSize;
///    float                 sampleLocationCoordinateRange[2];
///    uint32_t              sampleLocationSubPixelBits;
///    VkBool32              variableSampleLocations;
///} VkPhysicalDeviceSampleLocationsPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sample_location_sample_counts`] is a bitmask of [`SampleCountFlagBits`] indicating the
///   sample counts supporting custom sample locations.
/// - [`max_sample_location_grid_size`] is the maximum size of the pixel grid in which sample
///   locations **can** vary that is supported for all sample counts in
///   [`sample_location_sample_counts`].
/// - [`sample_location_coordinate_range`][2] is the range of supported sample location coordinates.
/// - [`sample_location_sub_pixel_bits`] is the number of bits of subpixel precision for sample
///   locations.
/// - [`variable_sample_locations`] specifies whether the sample locations used by all pipelines
///   that will be bound to a command buffer during a subpass **must** match. If set to [`TRUE`],
///   the implementation supports variable sample locations in a subpass. If set to [`FALSE`], then
///   the sample locations **must** stay constant in each subpass.
///# Description
///If the [`PhysicalDeviceSampleLocationsPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`Bool32`]
/// - [`Extent2D`]
/// - [`SampleCountFlags`]
/// - [`StructureType`]
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
pub struct PhysicalDeviceSampleLocationsPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`sample_location_sample_counts`]
    ///is a bitmask of [`SampleCountFlagBits`] indicating the sample counts
    ///supporting custom sample locations.
    sample_location_sample_counts: SampleCountFlags,
    ///[`max_sample_location_grid_size`] is
    ///the maximum size of the pixel grid in which sample locations **can** vary
    ///that is supported for all sample counts in
    ///[`sample_location_sample_counts`].
    max_sample_location_grid_size: Extent2D,
    ///[`sample_location_coordinate_range`][2] is the range of supported sample
    ///location coordinates.
    sample_location_coordinate_range: [f32; 2],
    ///[`sample_location_sub_pixel_bits`]
    ///is the number of bits of subpixel precision for sample locations.
    sample_location_sub_pixel_bits: u32,
    ///[`variable_sample_locations`]
    ///specifies whether the sample locations used by all pipelines that will
    ///be bound to a command buffer during a subpass **must** match.
    ///If set to [`TRUE`], the implementation supports variable sample
    ///locations in a subpass.
    ///If set to [`FALSE`], then the sample locations **must** stay constant
    ///in each subpass.
    variable_sample_locations: Bool32,
}
///[VkMultisamplePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultisamplePropertiesEXT.html) - Structure returning information about sample count specific additional multisampling capabilities
///# C Specifications
///The [`MultisamplePropertiesEXT`] structure is defined as
///```c
///// Provided by VK_EXT_sample_locations
///typedef struct VkMultisamplePropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkExtent2D         maxSampleLocationGridSize;
///} VkMultisamplePropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_sample_location_grid_size`] is the maximum size of the pixel grid in which sample
///   locations **can** vary.
///# Description
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`GetPhysicalDeviceMultisamplePropertiesEXT`]
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
pub struct MultisamplePropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`max_sample_location_grid_size`] is the maximum size of the pixel grid in
    ///which sample locations **can** vary.
    max_sample_location_grid_size: Extent2D,
}
