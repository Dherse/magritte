//![VK_EXT_sample_locations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_sample_locations.html) - device extension
//!# Description
//!This extension allows an application to modify the locations of samples
//!within a pixel used in rasterization.
//!Additionally, it allows applications to specify different sample locations
//!for each pixel in a group of adjacent pixels, which  **can**  increase
//!antialiasing quality (particularly if a custom resolve shader is used that
//!takes advantage of these different locations).It is common for implementations to optimize the
//! storage of depth values by
//!storing values that  **can**  be used to reconstruct depth at each sample
//!location, rather than storing separate depth values for each sample.
//!For example, the depth values from a single triangle  **may**  be represented
//!using plane equations.
//!When the depth value for a sample is needed, it is automatically evaluated
//!at the sample location.
//!Modifying the sample locations causes the reconstruction to no longer
//!evaluate the same depth values as when the samples were originally
//!generated, thus the depth aspect of a depth/stencil attachment  **must**  be
//!cleared before rendering to it using different sample locations.Some implementations  **may**
//! need to evaluate depth image values while
//!performing image layout transitions.
//!To accommodate this, instances of the [`SampleLocationsInfoEXT`]
//!structure  **can**  be specified for each situation where an explicit or
//!automatic layout transition has to take place.
//![`SampleLocationsInfoEXT`] **can**  be chained from
//![`ImageMemoryBarrier`] structures to provide sample locations for layout
//!transitions performed by [`cmd_wait_events`] and
//![`cmd_pipeline_barrier`] calls, and
//![`RenderPassSampleLocationsBeginInfoEXT`] **can**  be chained from
//![`RenderPassBeginInfo`] to provide sample locations for layout
//!transitions performed implicitly by a render pass instance.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_sample_locations]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the VK_EXT_sample_locations
//!   extension>>)
//!# New functions & commands
//! - [`cmd_set_sample_locations_ext`]
//! - [`get_physical_device_multisample_properties_ext`]
//!# New structures
//! - [`AttachmentSampleLocationsEXT`]
//! - [`MultisamplePropertiesEXT`]
//! - [`SampleLocationEXT`]
//! - [`SubpassSampleLocationsEXT`]
//! - Extending [`ImageMemoryBarrier`], [`ImageMemoryBarrier2`]:  - [`SampleLocationsInfoEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceSampleLocationsPropertiesEXT`]
//! - Extending [`PipelineMultisampleStateCreateInfo`]:  -
//!   [`PipelineSampleLocationsStateCreateInfoEXT`]
//! - Extending [`RenderPassBeginInfo`]:  - [`RenderPassSampleLocationsBeginInfoEXT`]
//!# New constants
//! - [`EXT_SAMPLE_LOCATIONS_EXTENSION_NAME`]
//! - [`EXT_SAMPLE_LOCATIONS_SPEC_VERSION`]
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT`
//! - Extending [`ImageCreateFlagBits`]:  -
//!   `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT`  -
//!   `VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT`
//!# Version History
//! - Revision 1, 2017-08-02 (Daniel Rakos)  - Internal revisions
//!# Other info
//! * 2017-08-02
//! * - Mais Alnasser, AMD  - Matthaeus G. Chajdas, AMD  - Maciej Jesionowski, AMD  - Daniel Rakos,
//!   AMD  - Slawomir Grajewski, Intel  - Jeff Bolz, NVIDIA  - Bill Licea-Kane, Qualcomm
//!# Related
//! - [`AttachmentSampleLocationsEXT`]
//! - [`MultisamplePropertiesEXT`]
//! - [`PhysicalDeviceSampleLocationsPropertiesEXT`]
//! - [`PipelineSampleLocationsStateCreateInfoEXT`]
//! - [`RenderPassSampleLocationsBeginInfoEXT`]
//! - [`SampleLocationEXT`]
//! - [`SampleLocationsInfoEXT`]
//! - [`SubpassSampleLocationsEXT`]
//! - [`cmd_set_sample_locations_ext`]
//! - [`get_physical_device_multisample_properties_ext`]
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
        BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Device, Extent2D, Instance, PhysicalDevice,
        SampleCountFlagBits, SampleCountFlags, StructureType,
    },
    AsRaw, Unique,
};
use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION")]
pub const EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME")]
pub const EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_sample_locations");
///[vkGetPhysicalDeviceMultisamplePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) - Report sample count specific multisampling capabilities of a physical device
///# C Specifications
///To query additional multisampling capabilities which  **may**  be supported for a
///specific sample count, beyond the minimum capabilities described for
///[Limits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits) above, call:
///```c
///// Provided by VK_EXT_sample_locations
///void vkGetPhysicalDeviceMultisamplePropertiesEXT(
///    VkPhysicalDevice                            physicalDevice,
///    VkSampleCountFlagBits                       samples,
///    VkMultisamplePropertiesEXT*                 pMultisampleProperties);
///```
///# Parameters
/// - [`physical_device`] is the physical device from which to query the additional multisampling
///   capabilities.
/// - [`samples`] is a [`SampleCountFlagBits`] value specifying the sample count to query
///   capabilities for.
/// - [`p_multisample_properties`] is a pointer to a [`MultisamplePropertiesEXT`] structure in which
///   information about additional multisampling capabilities specific to the sample count is
///   returned.
///# Description
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`samples`] **must**  be a valid [`SampleCountFlagBits`] value
/// - [`p_multisample_properties`] **must**  be a valid pointer to a [`MultisamplePropertiesEXT`]
///   structure
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`MultisamplePropertiesEXT`]
/// - [`PhysicalDevice`]
/// - [`SampleCountFlagBits`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
pub type FNGetPhysicalDeviceMultisamplePropertiesExt = Option<
    for<'lt> unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        samples: SampleCountFlagBits,
        p_multisample_properties: *mut MultisamplePropertiesEXT<'lt>,
    ),
>;
///[vkCmdSetSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html) - Set sample locations dynamically for a command buffer
///# C Specifications
///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the sample locations used
///for rasterization, call:
///```c
///// Provided by VK_EXT_sample_locations
///void vkCmdSetSampleLocationsEXT(
///    VkCommandBuffer                             commandBuffer,
///    const VkSampleLocationsInfoEXT*             pSampleLocationsInfo);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`p_sample_locations_info`] is the sample locations state to set.
///# Description
///This command sets the custom sample locations for subsequent drawing
///commands when the graphics pipeline is created with
///`VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` set in
///[`PipelineDynamicStateCreateInfo::dynamic_states`], and when the
///[`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`]
///property of the bound graphics pipeline is [`TRUE`].
///Otherwise, this state is specified by the
///[`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_info`]
///values used to create the currently active pipeline.
///## Valid Usage
/// - The `sampleLocationsPerPixel` member of [`p_sample_locations_info`] **must**  equal the
///   `rasterizationSamples` member of the [`PipelineMultisampleStateCreateInfo`] structure the
///   bound graphics pipeline has been created with
/// - If [`PhysicalDeviceSampleLocationsPropertiesEXT::variable_sample_locations`] is [`FALSE`] then
///   the current render pass  **must**  have been begun by specifying a
///   [`RenderPassSampleLocationsBeginInfoEXT`] structure whose `pPostSubpassSampleLocations` member
///   contains an element with a `subpassIndex` matching the current subpass index and the
///   `sampleLocationsInfo` member of that element  **must**  match the sample locations state
///   pointed to by [`p_sample_locations_info`]
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_sample_locations_info`] **must**  be a valid pointer to a valid [`SampleLocationsInfoEXT`]
///   structure
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`CommandBuffer`]
/// - [`SampleLocationsInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdSetSampleLocationsEXT")]
pub type FNCmdSetSampleLocationsExt = Option<
    for<'lt> unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_sample_locations_info: *const SampleLocationsInfoEXT<'lt>,
    ),
>;
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
///that  **can**  be queried using
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
#[doc(alias = "VkSampleLocationEXT")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SampleLocationEXT {
    ///[`x`] is the horizontal coordinate of the sample’s location.
    pub x: f32,
    ///[`y`] is the vertical coordinate of the sample’s location.
    pub y: f32,
}
impl Default for SampleLocationEXT {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
impl SampleLocationEXT {
    ///Gets the value of [`Self::x`]
    pub fn x(&self) -> f32 {
        self.x
    }
    ///Gets the value of [`Self::y`]
    pub fn y(&self) -> f32 {
        self.y
    }
    ///Gets a mutable reference to the value of [`Self::x`]
    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    ///Gets a mutable reference to the value of [`Self::y`]
    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    ///Sets the value of [`Self::x`]
    pub fn set_x(mut self, value: f32) -> Self {
        self.x = value;
        self
    }
    ///Sets the value of [`Self::y`]
    pub fn set_y(mut self, value: f32) -> Self {
        self.y = value;
        self
    }
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
/// - [`sample_locations_count`] is the number of sample locations in [`sample_locations`].
/// - [`sample_locations`] is a pointer to an array of
///   [`sample_locations_count`][`SampleLocationEXT`] structures.
///# Description
///This structure  **can**  be used either to specify the sample locations to be
///used for rendering or to specify the set of sample locations an image
///subresource has been last rendered with for the purposes of layout
///transitions of depth/stencil images created with
///`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT`.The sample locations in
/// [`sample_locations`] specify
///[`sample_locations_per_pixel`] number of sample locations for each pixel in
///the grid of the size specified in [`sample_location_grid_size`].
///The sample location for sample i at the pixel grid location
///(x,y) is taken from [`sample_locations`][(x +  y ×
///`sampleLocationGridSize.width`) × [`sample_locations_per_pixel`]
///+  i].If the render pass has a fragment density map, the implementation will
///choose the sample locations for the fragment and the contents of
///[`sample_locations`] **may**  be ignored.
///## Valid Usage
/// - [`sample_locations_per_pixel`] **must**  be a bit value that is set in
///   [`PhysicalDeviceSampleLocationsPropertiesEXT::sample_location_sample_counts`]
/// - [`sample_locations_count`] **must**  equal [`sample_locations_per_pixel`] ×
///   `sampleLocationGridSize.width` × `sampleLocationGridSize.height`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT`
/// - If [`sample_locations_count`] is not `0`, [`sample_locations`] **must**  be a valid pointer to
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
/// - [`cmd_set_sample_locations_ext`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSampleLocationsInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SampleLocationsInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`sample_locations_per_pixel`] is a [`SampleCountFlagBits`] value
    ///specifying the number of sample locations per pixel.
    pub sample_locations_per_pixel: SampleCountFlagBits,
    ///[`sample_location_grid_size`] is the size of the sample location grid to
    ///select custom sample locations for.
    pub sample_location_grid_size: Extent2D,
    ///[`sample_locations_count`] is the number of sample locations in
    ///[`sample_locations`].
    pub sample_locations_count: u32,
    ///[`sample_locations`] is a pointer to an array of
    ///[`sample_locations_count`][`SampleLocationEXT`] structures.
    pub sample_locations: *const SampleLocationEXT,
}
impl<'lt> Default for SampleLocationsInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SAMPLE_LOCATIONS_INFO_EXT,
            p_next: std::ptr::null(),
            sample_locations_per_pixel: Default::default(),
            sample_location_grid_size: Default::default(),
            sample_locations_count: 0,
            sample_locations: std::ptr::null(),
        }
    }
}
impl<'lt> SampleLocationsInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::sample_locations`]
    pub fn sample_locations_raw(&self) -> *const SampleLocationEXT {
        self.sample_locations
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::sample_locations`]
    pub fn set_sample_locations_raw(mut self, value: *const SampleLocationEXT) -> Self {
        self.sample_locations = value;
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
    ///Gets the value of [`Self::sample_locations_per_pixel`]
    pub fn sample_locations_per_pixel(&self) -> SampleCountFlagBits {
        self.sample_locations_per_pixel
    }
    ///Gets the value of [`Self::sample_location_grid_size`]
    pub fn sample_location_grid_size(&self) -> Extent2D {
        self.sample_location_grid_size
    }
    ///Gets the value of [`Self::sample_locations_count`]
    pub fn sample_locations_count(&self) -> u32 {
        self.sample_locations_count
    }
    ///Gets the value of [`Self::sample_locations`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn sample_locations(&self) -> &[SampleLocationEXT] {
        std::slice::from_raw_parts(self.sample_locations, self.sample_locations_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::sample_locations_per_pixel`]
    pub fn sample_locations_per_pixel_mut(&mut self) -> &mut SampleCountFlagBits {
        &mut self.sample_locations_per_pixel
    }
    ///Gets a mutable reference to the value of [`Self::sample_location_grid_size`]
    pub fn sample_location_grid_size_mut(&mut self) -> &mut Extent2D {
        &mut self.sample_location_grid_size
    }
    ///Gets a mutable reference to the value of [`Self::sample_locations_count`]
    pub fn sample_locations_count_mut(&mut self) -> &mut u32 {
        &mut self.sample_locations_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::sample_locations_per_pixel`]
    pub fn set_sample_locations_per_pixel(mut self, value: crate::vulkan1_0::SampleCountFlagBits) -> Self {
        self.sample_locations_per_pixel = value;
        self
    }
    ///Sets the value of [`Self::sample_location_grid_size`]
    pub fn set_sample_location_grid_size(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.sample_location_grid_size = value;
        self
    }
    ///Sets the value of [`Self::sample_locations_count`]
    pub fn set_sample_locations_count(mut self, value: u32) -> Self {
        self.sample_locations_count = value;
        self
    }
    ///Sets the value of [`Self::sample_locations`]
    pub fn set_sample_locations(
        mut self,
        value: &'lt [crate::extensions::ext_sample_locations::SampleLocationEXT],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.sample_locations = value.as_ptr();
        self.sample_locations_count = len_;
        self
    }
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
///values specified in [`sample_locations_info`] are ignored.
///## Valid Usage
/// - [`attachment_index`] **must**  be less than the `attachmentCount` specified in
///   [`RenderPassCreateInfo`] the render pass specified by [`RenderPassBeginInfo::render_pass`] was
///   created with
///
///## Valid Usage (Implicit)
/// - [`sample_locations_info`] **must**  be a valid [`SampleLocationsInfoEXT`] structure
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
#[doc(alias = "VkAttachmentSampleLocationsEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AttachmentSampleLocationsEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`attachment_index`] is the index of the attachment for which the
    ///sample locations state is provided.
    pub attachment_index: u32,
    ///[`sample_locations_info`] is the sample locations state to use for the
    ///layout transition of the given attachment from the initial layout of the
    ///attachment to the image layout specified for the attachment in the first
    ///subpass using it.
    pub sample_locations_info: SampleLocationsInfoEXT<'lt>,
}
impl<'lt> Default for AttachmentSampleLocationsEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            attachment_index: 0,
            sample_locations_info: Default::default(),
        }
    }
}
impl<'lt> AttachmentSampleLocationsEXT<'lt> {
    ///Gets the value of [`Self::attachment_index`]
    pub fn attachment_index(&self) -> u32 {
        self.attachment_index
    }
    ///Gets the value of [`Self::sample_locations_info`]
    pub fn sample_locations_info(&self) -> SampleLocationsInfoEXT<'lt> {
        self.sample_locations_info
    }
    ///Gets a mutable reference to the value of [`Self::attachment_index`]
    pub fn attachment_index_mut(&mut self) -> &mut u32 {
        &mut self.attachment_index
    }
    ///Gets a mutable reference to the value of [`Self::sample_locations_info`]
    pub fn sample_locations_info_mut(&mut self) -> &mut SampleLocationsInfoEXT<'lt> {
        &mut self.sample_locations_info
    }
    ///Sets the value of [`Self::attachment_index`]
    pub fn set_attachment_index(mut self, value: u32) -> Self {
        self.attachment_index = value;
        self
    }
    ///Sets the value of [`Self::sample_locations_info`]
    pub fn set_sample_locations_info(
        mut self,
        value: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT<'lt>,
    ) -> Self {
        self.sample_locations_info = value;
        self
    }
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
///ignored.
///## Valid Usage
/// - [`subpass_index`] **must**  be less than the `subpassCount` specified in
///   [`RenderPassCreateInfo`] the render pass specified by [`RenderPassBeginInfo::render_pass`] was
///   created with
///
///## Valid Usage (Implicit)
/// - [`sample_locations_info`] **must**  be a valid [`SampleLocationsInfoEXT`] structure
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
#[doc(alias = "VkSubpassSampleLocationsEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SubpassSampleLocationsEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`subpass_index`] is the index of the subpass for which the sample
    ///locations state is provided.
    pub subpass_index: u32,
    ///[`sample_locations_info`] is the sample locations state to use for the
    ///layout transition of the depth/stencil attachment away from the image
    ///layout the attachment is used with in the subpass specified in
    ///[`subpass_index`].
    pub sample_locations_info: SampleLocationsInfoEXT<'lt>,
}
impl<'lt> Default for SubpassSampleLocationsEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            subpass_index: 0,
            sample_locations_info: Default::default(),
        }
    }
}
impl<'lt> SubpassSampleLocationsEXT<'lt> {
    ///Gets the value of [`Self::subpass_index`]
    pub fn subpass_index(&self) -> u32 {
        self.subpass_index
    }
    ///Gets the value of [`Self::sample_locations_info`]
    pub fn sample_locations_info(&self) -> SampleLocationsInfoEXT<'lt> {
        self.sample_locations_info
    }
    ///Gets a mutable reference to the value of [`Self::subpass_index`]
    pub fn subpass_index_mut(&mut self) -> &mut u32 {
        &mut self.subpass_index
    }
    ///Gets a mutable reference to the value of [`Self::sample_locations_info`]
    pub fn sample_locations_info_mut(&mut self) -> &mut SampleLocationsInfoEXT<'lt> {
        &mut self.sample_locations_info
    }
    ///Sets the value of [`Self::subpass_index`]
    pub fn set_subpass_index(mut self, value: u32) -> Self {
        self.subpass_index = value;
        self
    }
    ///Sets the value of [`Self::sample_locations_info`]
    pub fn set_sample_locations_info(
        mut self,
        value: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT<'lt>,
    ) -> Self {
        self.sample_locations_info = value;
        self
    }
}
///[VkRenderPassSampleLocationsBeginInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html) - Structure specifying sample locations to use for the layout transition of custom sample locations compatible depth/stencil attachments
///# C Specifications
///The image layout of the depth aspect of a depth/stencil attachment referring
///to an image created with
///`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` is dependent
///on the last sample locations used to render to the image subresource, thus
///preserving the contents of such depth/stencil attachments across subpass
///boundaries requires the application to specify these sample locations
///whenever a layout transition of the attachment  **may**  occur.
///This information  **can**  be provided by adding a
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
///   [`attachment_initial_sample_locations`] array.
/// - [`attachment_initial_sample_locations`] is a pointer to an array of
///   [`attachment_initial_sample_locations_count`][`AttachmentSampleLocationsEXT`] structures
///   specifying the attachment indices and their corresponding sample location state. Each element
///   of [`attachment_initial_sample_locations`] **can**  specify the sample location state to use
///   in the automatic layout transition performed to transition a depth/stencil attachment from the
///   initial layout of the attachment to the image layout specified for the attachment in the first
///   subpass using it.
/// - [`post_subpass_sample_locations_count`] is the number of elements in the
///   [`post_subpass_sample_locations`] array.
/// - [`post_subpass_sample_locations`] is a pointer to an array of
///   [`post_subpass_sample_locations_count`][`SubpassSampleLocationsEXT`] structures specifying the
///   subpass indices and their corresponding sample location state. Each element of
///   [`post_subpass_sample_locations`] **can**  specify the sample location state to use in the
///   automatic layout transition performed to transition the depth/stencil attachment used by the
///   specified subpass to the image layout specified in a dependent subpass or to the final layout
///   of the attachment in case the specified subpass is the last subpass using that attachment. In
///   addition, if [`PhysicalDeviceSampleLocationsPropertiesEXT::variable_sample_locations`] is
///   [`FALSE`], each element of [`post_subpass_sample_locations`] **must**  specify the sample
///   location state that matches the sample locations used by all pipelines that will be bound to a
///   command buffer during the specified subpass. If `variableSampleLocations` is [`TRUE`], the
///   sample locations used for rasterization do not depend on [`post_subpass_sample_locations`].
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT`
/// - If [`attachment_initial_sample_locations_count`] is not `0`,
///   [`attachment_initial_sample_locations`] **must**  be a valid pointer to an array of
///   [`attachment_initial_sample_locations_count`] valid [`AttachmentSampleLocationsEXT`]
///   structures
/// - If [`post_subpass_sample_locations_count`] is not `0`, [`post_subpass_sample_locations`]
///   **must**  be a valid pointer to an array of [`post_subpass_sample_locations_count`] valid
///   [`SubpassSampleLocationsEXT`] structures
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
#[doc(alias = "VkRenderPassSampleLocationsBeginInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct RenderPassSampleLocationsBeginInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`attachment_initial_sample_locations_count`] is the number of elements in
    ///the [`attachment_initial_sample_locations`] array.
    pub attachment_initial_sample_locations_count: u32,
    ///[`attachment_initial_sample_locations`] is a pointer to an array of
    ///[`attachment_initial_sample_locations_count`][`AttachmentSampleLocationsEXT`] structures
    /// specifying the attachment indices and their corresponding sample location state.
    ///Each element of [`attachment_initial_sample_locations`] **can**  specify the
    ///sample location state to use in the automatic layout transition
    ///performed to transition a depth/stencil attachment from the initial
    ///layout of the attachment to the image layout specified for the
    ///attachment in the first subpass using it.
    pub attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT<'lt>,
    ///[`post_subpass_sample_locations_count`] is the number of elements in the
    ///[`post_subpass_sample_locations`] array.
    pub post_subpass_sample_locations_count: u32,
    ///[`post_subpass_sample_locations`] is a pointer to an array of
    ///[`post_subpass_sample_locations_count`][`SubpassSampleLocationsEXT`]
    ///structures specifying the subpass indices and their corresponding sample
    ///location state.
    ///Each element of [`post_subpass_sample_locations`] **can**  specify the
    ///sample location state to use in the automatic layout transition
    ///performed to transition the depth/stencil attachment used by the
    ///specified subpass to the image layout specified in a dependent subpass
    ///or to the final layout of the attachment in case the specified subpass
    ///is the last subpass using that attachment.
    ///In addition, if
    ///[`PhysicalDeviceSampleLocationsPropertiesEXT`]::`variableSampleLocations`
    ///is [`FALSE`], each element of [`post_subpass_sample_locations`] **must**  specify the sample
    /// location state that matches the sample locations used by all pipelines that will be
    /// bound to a command buffer during the specified subpass.
    ///If `variableSampleLocations` is [`TRUE`], the sample locations
    ///used for rasterization do not depend on
    ///[`post_subpass_sample_locations`].
    pub post_subpass_sample_locations: *const SubpassSampleLocationsEXT<'lt>,
}
impl<'lt> Default for RenderPassSampleLocationsBeginInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
            p_next: std::ptr::null(),
            attachment_initial_sample_locations_count: 0,
            attachment_initial_sample_locations: std::ptr::null(),
            post_subpass_sample_locations_count: 0,
            post_subpass_sample_locations: std::ptr::null(),
        }
    }
}
impl<'lt> RenderPassSampleLocationsBeginInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::attachment_initial_sample_locations`]
    pub fn attachment_initial_sample_locations_raw(&self) -> *const AttachmentSampleLocationsEXT<'lt> {
        self.attachment_initial_sample_locations
    }
    ///Gets the raw value of [`Self::post_subpass_sample_locations`]
    pub fn post_subpass_sample_locations_raw(&self) -> *const SubpassSampleLocationsEXT<'lt> {
        self.post_subpass_sample_locations
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::attachment_initial_sample_locations`]
    pub fn set_attachment_initial_sample_locations_raw(
        mut self,
        value: *const AttachmentSampleLocationsEXT<'lt>,
    ) -> Self {
        self.attachment_initial_sample_locations = value;
        self
    }
    ///Sets the raw value of [`Self::post_subpass_sample_locations`]
    pub fn set_post_subpass_sample_locations_raw(mut self, value: *const SubpassSampleLocationsEXT<'lt>) -> Self {
        self.post_subpass_sample_locations = value;
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
    ///Gets the value of [`Self::attachment_initial_sample_locations_count`]
    pub fn attachment_initial_sample_locations_count(&self) -> u32 {
        self.attachment_initial_sample_locations_count
    }
    ///Gets the value of [`Self::attachment_initial_sample_locations`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn attachment_initial_sample_locations(&self) -> &[AttachmentSampleLocationsEXT<'lt>] {
        std::slice::from_raw_parts(
            self.attachment_initial_sample_locations,
            self.attachment_initial_sample_locations_count as usize,
        )
    }
    ///Gets the value of [`Self::post_subpass_sample_locations_count`]
    pub fn post_subpass_sample_locations_count(&self) -> u32 {
        self.post_subpass_sample_locations_count
    }
    ///Gets the value of [`Self::post_subpass_sample_locations`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn post_subpass_sample_locations(&self) -> &[SubpassSampleLocationsEXT<'lt>] {
        std::slice::from_raw_parts(
            self.post_subpass_sample_locations,
            self.post_subpass_sample_locations_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::attachment_initial_sample_locations_count`]
    pub fn attachment_initial_sample_locations_count_mut(&mut self) -> &mut u32 {
        &mut self.attachment_initial_sample_locations_count
    }
    ///Gets a mutable reference to the value of [`Self::post_subpass_sample_locations_count`]
    pub fn post_subpass_sample_locations_count_mut(&mut self) -> &mut u32 {
        &mut self.post_subpass_sample_locations_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::attachment_initial_sample_locations_count`]
    pub fn set_attachment_initial_sample_locations_count(mut self, value: u32) -> Self {
        self.attachment_initial_sample_locations_count = value;
        self
    }
    ///Sets the value of [`Self::attachment_initial_sample_locations`]
    pub fn set_attachment_initial_sample_locations(
        mut self,
        value: &'lt [crate::extensions::ext_sample_locations::AttachmentSampleLocationsEXT<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.attachment_initial_sample_locations = value.as_ptr();
        self.attachment_initial_sample_locations_count = len_;
        self
    }
    ///Sets the value of [`Self::post_subpass_sample_locations_count`]
    pub fn set_post_subpass_sample_locations_count(mut self, value: u32) -> Self {
        self.post_subpass_sample_locations_count = value;
        self
    }
    ///Sets the value of [`Self::post_subpass_sample_locations`]
    pub fn set_post_subpass_sample_locations(
        mut self,
        value: &'lt [crate::extensions::ext_sample_locations::SubpassSampleLocationsEXT<'lt>],
    ) -> Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.post_subpass_sample_locations = value.as_ptr();
        self.post_subpass_sample_locations_count = len_;
        self
    }
}
///[VkPipelineSampleLocationsStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html) - Structure specifying sample locations for a pipeline
///# C Specifications
///Applications  **can**  also control the sample locations used for rasterization.If the
/// [`p_next`] chain of the [`PipelineMultisampleStateCreateInfo`]
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
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT`
/// - [`sample_locations_info`] **must**  be a valid [`SampleLocationsInfoEXT`] structure
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
#[doc(alias = "VkPipelineSampleLocationsStateCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineSampleLocationsStateCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`sample_locations_enable`] controls whether custom sample locations are
    ///used.
    ///If [`sample_locations_enable`] is [`FALSE`], the default sample
    ///locations are used and the values specified in [`sample_locations_info`]
    ///are ignored.
    pub sample_locations_enable: Bool32,
    ///[`sample_locations_info`] is the sample locations to use during
    ///rasterization if [`sample_locations_enable`] is [`TRUE`] and the
    ///graphics pipeline is not created with
    ///`VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT`.
    pub sample_locations_info: SampleLocationsInfoEXT<'lt>,
}
impl<'lt> Default for PipelineSampleLocationsStateCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            sample_locations_enable: 0,
            sample_locations_info: Default::default(),
        }
    }
}
impl<'lt> PipelineSampleLocationsStateCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::sample_locations_enable`]
    pub fn sample_locations_enable_raw(&self) -> Bool32 {
        self.sample_locations_enable
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::sample_locations_enable`]
    pub fn set_sample_locations_enable_raw(mut self, value: Bool32) -> Self {
        self.sample_locations_enable = value;
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
    ///Gets the value of [`Self::sample_locations_enable`]
    pub fn sample_locations_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.sample_locations_enable as u8) }
    }
    ///Gets the value of [`Self::sample_locations_info`]
    pub fn sample_locations_info(&self) -> SampleLocationsInfoEXT<'lt> {
        self.sample_locations_info
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::sample_locations_enable`]
    pub fn sample_locations_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.sample_locations_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.sample_locations_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::sample_locations_info`]
    pub fn sample_locations_info_mut(&mut self) -> &mut SampleLocationsInfoEXT<'lt> {
        &mut self.sample_locations_info
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::sample_locations_enable`]
    pub fn set_sample_locations_enable(mut self, value: bool) -> Self {
        self.sample_locations_enable = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::sample_locations_info`]
    pub fn set_sample_locations_info(
        mut self,
        value: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT<'lt>,
    ) -> Self {
        self.sample_locations_info = value;
        self
    }
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
///   locations  **can**  vary that is supported for all sample counts in
///   [`sample_location_sample_counts`].
/// - [`sample_location_coordinate_range`][2] is the range of supported sample location coordinates.
/// - [`sample_location_sub_pixel_bits`] is the number of bits of subpixel precision for sample
///   locations.
/// - [`variable_sample_locations`] specifies whether the sample locations used by all pipelines
///   that will be bound to a command buffer during a subpass  **must**  match. If set to [`TRUE`],
///   the implementation supports variable sample locations in a subpass. If set to [`FALSE`], then
///   the sample locations  **must**  stay constant in each subpass.
///# Description
///If the [`PhysicalDeviceSampleLocationsPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT`
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
#[doc(alias = "VkPhysicalDeviceSampleLocationsPropertiesEXT")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`sample_location_sample_counts`]
    ///is a bitmask of [`SampleCountFlagBits`] indicating the sample counts
    ///supporting custom sample locations.
    pub sample_location_sample_counts: SampleCountFlags,
    ///[`max_sample_location_grid_size`] is
    ///the maximum size of the pixel grid in which sample locations  **can**  vary
    ///that is supported for all sample counts in
    ///[`sample_location_sample_counts`].
    pub max_sample_location_grid_size: Extent2D,
    ///[`sample_location_coordinate_range`][2] is the range of supported sample
    ///location coordinates.
    pub sample_location_coordinate_range: [f32; 2 as usize],
    ///[`sample_location_sub_pixel_bits`]
    ///is the number of bits of subpixel precision for sample locations.
    pub sample_location_sub_pixel_bits: u32,
    ///[`variable_sample_locations`]
    ///specifies whether the sample locations used by all pipelines that will
    ///be bound to a command buffer during a subpass  **must**  match.
    ///If set to [`TRUE`], the implementation supports variable sample
    ///locations in a subpass.
    ///If set to [`FALSE`], then the sample locations  **must**  stay constant
    ///in each subpass.
    pub variable_sample_locations: Bool32,
}
impl<'lt> Default for PhysicalDeviceSampleLocationsPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            sample_location_sample_counts: Default::default(),
            max_sample_location_grid_size: Default::default(),
            sample_location_coordinate_range: [0.0; 2 as usize],
            sample_location_sub_pixel_bits: 0,
            variable_sample_locations: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSampleLocationsPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::variable_sample_locations`]
    pub fn variable_sample_locations_raw(&self) -> Bool32 {
        self.variable_sample_locations
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::variable_sample_locations`]
    pub fn set_variable_sample_locations_raw(mut self, value: Bool32) -> Self {
        self.variable_sample_locations = value;
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
    ///Gets the value of [`Self::sample_location_sample_counts`]
    pub fn sample_location_sample_counts(&self) -> SampleCountFlags {
        self.sample_location_sample_counts
    }
    ///Gets the value of [`Self::max_sample_location_grid_size`]
    pub fn max_sample_location_grid_size(&self) -> Extent2D {
        self.max_sample_location_grid_size
    }
    ///Gets the value of [`Self::sample_location_coordinate_range`]
    pub fn sample_location_coordinate_range(&self) -> &[f32; 2 as usize] {
        &self.sample_location_coordinate_range
    }
    ///Gets the value of [`Self::sample_location_sub_pixel_bits`]
    pub fn sample_location_sub_pixel_bits(&self) -> u32 {
        self.sample_location_sub_pixel_bits
    }
    ///Gets the value of [`Self::variable_sample_locations`]
    pub fn variable_sample_locations(&self) -> bool {
        unsafe { std::mem::transmute(self.variable_sample_locations as u8) }
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
    ///Gets a mutable reference to the value of [`Self::sample_location_sample_counts`]
    pub fn sample_location_sample_counts_mut(&mut self) -> &mut SampleCountFlags {
        &mut self.sample_location_sample_counts
    }
    ///Gets a mutable reference to the value of [`Self::max_sample_location_grid_size`]
    pub fn max_sample_location_grid_size_mut(&mut self) -> &mut Extent2D {
        &mut self.max_sample_location_grid_size
    }
    ///Gets a mutable reference to the value of [`Self::sample_location_coordinate_range`]
    pub fn sample_location_coordinate_range_mut(&mut self) -> &mut [f32; 2 as usize] {
        &mut self.sample_location_coordinate_range
    }
    ///Gets a mutable reference to the value of [`Self::sample_location_sub_pixel_bits`]
    pub fn sample_location_sub_pixel_bits_mut(&mut self) -> &mut u32 {
        &mut self.sample_location_sub_pixel_bits
    }
    ///Gets a mutable reference to the value of [`Self::variable_sample_locations`]
    pub fn variable_sample_locations_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.variable_sample_locations as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.variable_sample_locations as *mut Bool32)
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
    ///Sets the value of [`Self::sample_location_sample_counts`]
    pub fn set_sample_location_sample_counts(mut self, value: crate::vulkan1_0::SampleCountFlags) -> Self {
        self.sample_location_sample_counts = value;
        self
    }
    ///Sets the value of [`Self::max_sample_location_grid_size`]
    pub fn set_max_sample_location_grid_size(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.max_sample_location_grid_size = value;
        self
    }
    ///Sets the value of [`Self::sample_location_coordinate_range`]
    pub fn set_sample_location_coordinate_range(mut self, value: [f32; 2 as usize]) -> Self {
        self.sample_location_coordinate_range = value;
        self
    }
    ///Sets the value of [`Self::sample_location_sub_pixel_bits`]
    pub fn set_sample_location_sub_pixel_bits(mut self, value: u32) -> Self {
        self.sample_location_sub_pixel_bits = value;
        self
    }
    ///Sets the value of [`Self::variable_sample_locations`]
    pub fn set_variable_sample_locations(mut self, value: bool) -> Self {
        self.variable_sample_locations = value as u8 as u32;
        self
    }
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
///   locations  **can**  vary.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_EXT_sample_locations`]
/// - [`Extent2D`]
/// - [`StructureType`]
/// - [`get_physical_device_multisample_properties_ext`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMultisamplePropertiesEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct MultisamplePropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_sample_location_grid_size`] is the maximum size of the pixel grid in
    ///which sample locations  **can**  vary.
    pub max_sample_location_grid_size: Extent2D,
}
impl<'lt> Default for MultisamplePropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::MULTISAMPLE_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_sample_location_grid_size: Default::default(),
        }
    }
}
impl<'lt> MultisamplePropertiesEXT<'lt> {
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
    ///Gets the value of [`Self::max_sample_location_grid_size`]
    pub fn max_sample_location_grid_size(&self) -> Extent2D {
        self.max_sample_location_grid_size
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
    ///Gets a mutable reference to the value of [`Self::max_sample_location_grid_size`]
    pub fn max_sample_location_grid_size_mut(&mut self) -> &mut Extent2D {
        &mut self.max_sample_location_grid_size
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
    ///Sets the value of [`Self::max_sample_location_grid_size`]
    pub fn set_max_sample_location_grid_size(mut self, value: crate::vulkan1_0::Extent2D) -> Self {
        self.max_sample_location_grid_size = value;
        self
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceMultisamplePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) - Report sample count specific multisampling capabilities of a physical device
    ///# C Specifications
    ///To query additional multisampling capabilities which  **may**  be supported for a
    ///specific sample count, beyond the minimum capabilities described for
    ///[Limits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits) above, call:
    ///```c
    ///// Provided by VK_EXT_sample_locations
    ///void vkGetPhysicalDeviceMultisamplePropertiesEXT(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    VkSampleCountFlagBits                       samples,
    ///    VkMultisamplePropertiesEXT*                 pMultisampleProperties);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device from which to query the additional
    ///   multisampling capabilities.
    /// - [`samples`] is a [`SampleCountFlagBits`] value specifying the sample count to query
    ///   capabilities for.
    /// - [`p_multisample_properties`] is a pointer to a [`MultisamplePropertiesEXT`] structure in
    ///   which information about additional multisampling capabilities specific to the sample count
    ///   is returned.
    ///# Description
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`samples`] **must**  be a valid [`SampleCountFlagBits`] value
    /// - [`p_multisample_properties`] **must**  be a valid pointer to a
    ///   [`MultisamplePropertiesEXT`] structure
    ///# Related
    /// - [`VK_EXT_sample_locations`]
    /// - [`MultisamplePropertiesEXT`]
    /// - [`PhysicalDevice`]
    /// - [`SampleCountFlagBits`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceMultisamplePropertiesEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_multisample_properties_ext<'a: 'this, 'this, 'lt>(
        self: &'this Unique<'a, PhysicalDevice>,
        samples: SampleCountFlagBits,
        p_multisample_properties: Option<MultisamplePropertiesEXT<'lt>>,
    ) -> MultisamplePropertiesEXT<'lt> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .ext_sample_locations()
            .expect("extension/version not loaded")
            .get_physical_device_multisample_properties_ext()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .ext_sample_locations()
            .unwrap_unchecked()
            .get_physical_device_multisample_properties_ext()
            .unwrap_unchecked();
        let mut p_multisample_properties = p_multisample_properties
            .unwrap_or_else(|| MaybeUninit::<MultisamplePropertiesEXT<'lt>>::zeroed().assume_init());
        let _return = _function(self.as_raw(), samples, &mut p_multisample_properties);
        {
            p_multisample_properties.p_next = std::ptr::null_mut();
            p_multisample_properties
        }
    }
}
impl CommandBuffer {
    ///[vkCmdSetSampleLocationsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html) - Set sample locations dynamically for a command buffer
    ///# C Specifications
    ///To [dynamically set](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipelines-dynamic-state) the sample locations used
    ///for rasterization, call:
    ///```c
    ///// Provided by VK_EXT_sample_locations
    ///void vkCmdSetSampleLocationsEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    const VkSampleLocationsInfoEXT*             pSampleLocationsInfo);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`p_sample_locations_info`] is the sample locations state to set.
    ///# Description
    ///This command sets the custom sample locations for subsequent drawing
    ///commands when the graphics pipeline is created with
    ///`VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` set in
    ///[`PipelineDynamicStateCreateInfo::dynamic_states`], and when the
    ///[`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`]
    ///property of the bound graphics pipeline is [`TRUE`].
    ///Otherwise, this state is specified by the
    ///[`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_info`]
    ///values used to create the currently active pipeline.
    ///## Valid Usage
    /// - The `sampleLocationsPerPixel` member of [`p_sample_locations_info`] **must**  equal the
    ///   `rasterizationSamples` member of the [`PipelineMultisampleStateCreateInfo`] structure the
    ///   bound graphics pipeline has been created with
    /// - If [`PhysicalDeviceSampleLocationsPropertiesEXT::variable_sample_locations`] is [`FALSE`]
    ///   then the current render pass  **must**  have been begun by specifying a
    ///   [`RenderPassSampleLocationsBeginInfoEXT`] structure whose `pPostSubpassSampleLocations`
    ///   member contains an element with a `subpassIndex` matching the current subpass index and
    ///   the `sampleLocationsInfo` member of that element  **must**  match the sample locations
    ///   state pointed to by [`p_sample_locations_info`]
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_sample_locations_info`] **must**  be a valid pointer to a valid
    ///   [`SampleLocationsInfoEXT`] structure
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_EXT_sample_locations`]
    /// - [`CommandBuffer`]
    /// - [`SampleLocationsInfoEXT`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdSetSampleLocationsEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_set_sample_locations_ext<'a: 'this, 'this, 'lt>(
        self: &'this mut Unique<'a, CommandBuffer>,
        p_sample_locations_info: &SampleLocationsInfoEXT<'lt>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_sample_locations()
            .expect("extension/version not loaded")
            .cmd_set_sample_locations_ext()
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_sample_locations()
            .unwrap_unchecked()
            .cmd_set_sample_locations_ext()
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            p_sample_locations_info as *const SampleLocationsInfoEXT<'lt>,
        );
        ()
    }
}
///The V-table of [`Instance`] for functions from `VK_EXT_sample_locations`
pub struct InstanceExtSampleLocationsVTable {
    ///See [`FNGetPhysicalDeviceMultisamplePropertiesExt`] for more information.
    pub get_physical_device_multisample_properties_ext: FNGetPhysicalDeviceMultisamplePropertiesExt,
}
impl InstanceExtSampleLocationsVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Instance,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Instance,
    ) -> Self {
        Self {
            get_physical_device_multisample_properties_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceMultisamplePropertiesEXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_multisample_properties_ext`]. See
    /// [`FNGetPhysicalDeviceMultisamplePropertiesExt`] for more information.
    pub fn get_physical_device_multisample_properties_ext(&self) -> FNGetPhysicalDeviceMultisamplePropertiesExt {
        self.get_physical_device_multisample_properties_ext
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_sample_locations`
pub struct DeviceExtSampleLocationsVTable {
    ///See [`FNCmdSetSampleLocationsExt`] for more information.
    pub cmd_set_sample_locations_ext: FNCmdSetSampleLocationsExt,
}
impl DeviceExtSampleLocationsVTable {
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
            cmd_set_sample_locations_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetSampleLocationsEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::cmd_set_sample_locations_ext`]. See [`FNCmdSetSampleLocationsExt`] for more
    /// information.
    pub fn cmd_set_sample_locations_ext(&self) -> FNCmdSetSampleLocationsExt {
        self.cmd_set_sample_locations_ext
    }
}
