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
//!`[`VK_EXT_shader_viewport_index_layer`]` is supported.
//!The `GlobalInvocationId.xy` is equal to the index of the local workgroup
//!multiplied by the size of the local workgroup plus the
//!`LocalInvocationId` and the [`Rect2D`] of the
//![`RenderPassBeginInfo::render_area`].This extension allows a subpass’s pipeline bind point to be
//!`VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI`.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_create_renderpass2`]`
//! - Requires `[`VK_KHR_synchronization2`]`
//!# Contacts
//! - Hueilong Wang [wyvernathuawei](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_HUAWEI_subpass_shading]
//!   @wyvernathuawei%0A<<Here describe the issue or question you have about the
//!   VK_HUAWEI_subpass_shading extension>>)
//!# New functions & commands
//! - [`CmdSubpassShadingHUAWEI`]
//! - [`GetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`]
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
//! - [`CmdSubpassShadingHUAWEI`]
//! - [`GetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
///A subpass shading pipeline is a compute pipeline which  **must**  be called only
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
#[doc(alias = "VkSubpassShadingPipelineCreateInfoHUAWEI")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
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
            s_type: StructureType::SubpassShadingPipelineCreateInfoHuawei,
            p_next: std::ptr::null_mut(),
            render_pass: Default::default(),
            subpass: 0,
        }
    }
}
impl<'lt> SubpassShadingPipelineCreateInfoHUAWEI<'lt> {
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
    ///Sets the raw value of [`Self::render_pass`]
    pub fn set_render_pass(&mut self, value: crate::vulkan1_0::RenderPass) -> &mut Self {
        self.render_pass = value;
        self
    }
    ///Sets the raw value of [`Self::subpass`]
    pub fn set_subpass(&mut self, value: u32) -> &mut Self {
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
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI`
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
#[doc(alias = "VkPhysicalDeviceSubpassShadingPropertiesHUAWEI")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
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
            s_type: StructureType::PhysicalDeviceSubpassShadingPropertiesHuawei,
            p_next: std::ptr::null_mut(),
            max_subpass_shading_workgroup_size_aspect_ratio: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSubpassShadingPropertiesHUAWEI<'lt> {
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
    ///Sets the raw value of [`Self::max_subpass_shading_workgroup_size_aspect_ratio`]
    pub fn set_max_subpass_shading_workgroup_size_aspect_ratio(&mut self, value: u32) -> &mut Self {
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
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceSubpassShadingFeaturesHUAWEI`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI`
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
#[doc(alias = "VkPhysicalDeviceSubpassShadingFeaturesHUAWEI")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
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
            s_type: StructureType::PhysicalDeviceSubpassShadingFeaturesHuawei,
            p_next: std::ptr::null_mut(),
            subpass_shading: 0,
        }
    }
}
impl<'lt> PhysicalDeviceSubpassShadingFeaturesHUAWEI<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::subpass_shading`]
    pub fn subpass_shading_raw(&self) -> Bool32 {
        self.subpass_shading
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::subpass_shading`]
    pub fn set_subpass_shading_raw(&mut self, value: Bool32) -> &mut Self {
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
    ///Sets the raw value of [`Self::subpass_shading`]
    pub fn set_subpass_shading(&mut self, value: bool) -> &mut Self {
        self.subpass_shading = value as u8 as u32;
        self
    }
}
