//![VK_EXT_fragment_density_map](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_fragment_density_map.html) - device extension
//!# Description
//!This extension allows an application to specify areas of the render target
//!where the fragment shader may be invoked fewer times.
//!These fragments are broadcasted out to multiple pixels to cover the render
//!target.The primary use of this extension is to reduce workloads in areas where
//!lower quality may not be perceived such as the distorted edges of a lens or
//!the periphery of a user’s gaze.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Matthew Netsch [mnetsch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_fragment_density_map]
//!   @mnetsch%0A<<Here describe the issue or question you have about the
//!   VK_EXT_fragment_density_map extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceFragmentDensityMapFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceFragmentDensityMapPropertiesEXT`]
//! - Extending [`RenderPassCreateInfo`], [`RenderPassCreateInfo2`]:  -
//!   [`RenderPassFragmentDensityMapCreateInfoEXT`]
//!# New constants
//! - [`EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME`]
//! - [`EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`
//! - Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT`
//! - Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`
//! - Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`
//! - Extending [`ImageUsageFlagBits`]:  - `VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT`
//! - Extending [`ImageViewCreateFlagBits`]:  -
//!   `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT`
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
//! - Extending [`SamplerCreateFlagBits`]:  - `VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT`  -
//!   `VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT`
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:  - `VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT`
//!# Version History
//! - Revision 1, 2018-09-25 (Matthew Netsch)  - Initial version
//! - Revision 2, 2021-09-30 (Jon Leech)  - Add interaction with `[`VK_KHR_format_feature_flags2`]`
//!   to `vk.xml`
//!# Other info
//! * 2021-09-30
//! * - This extension requires [`SPV_EXT_fragment_invocation_density`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_fragment_invocation_density.html)
//!   - This extension provides API support for [`GL_EXT_fragment_invocation_density`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_fragment_invocation_density.txt)
//! * - Matthew Netsch, Qualcomm Technologies, Inc.  - Robert VanReenen, Qualcomm Technologies, Inc.
//!   - Jonathan Wicks, Qualcomm Technologies, Inc.  - Tate Hornbeck, Qualcomm Technologies, Inc.  -
//!   Sam Holmes, Qualcomm Technologies, Inc.  - Jeff Leger, Qualcomm Technologies, Inc.  -
//!   Jan-Harald Fredriksen, ARM  - Jeff Bolz, NVIDIA  - Pat Brown, NVIDIA  - Daniel Rakos, AMD  -
//!   Piers Daniell, NVIDIA
//!# Related
//! - [`PhysicalDeviceFragmentDensityMapFeaturesEXT`]
//! - [`PhysicalDeviceFragmentDensityMapPropertiesEXT`]
//! - [`RenderPassFragmentDensityMapCreateInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{AttachmentReference, BaseInStructure, BaseOutStructure, Bool32, Extent2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_fragment_density_map");
///[VkPhysicalDeviceFragmentDensityMapFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapFeaturesEXT.html) - Structure describing fragment density map features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMapFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_fragment_density_map
///typedef struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           fragmentDensityMap;
///    VkBool32           fragmentDensityMapDynamic;
///    VkBool32           fragmentDensityMapNonSubsampledImages;
///} VkPhysicalDeviceFragmentDensityMapFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_map`] specifies whether the implementation supports render passes with a
///   fragment density map attachment. If this feature is not enabled and the [`p_next`] chain of
///   [`RenderPassCreateInfo`] includes a [`RenderPassFragmentDensityMapCreateInfoEXT`] structure,
///   `fragmentDensityMapAttachment` **must**  be [`ATTACHMENT_UNUSED`].
/// - [`fragment_density_map_dynamic`] specifies whether the implementation supports dynamic
///   fragment density map image views. If this feature is not enabled,
///   `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT` **must**  not be included in
///   [`ImageViewCreateInfo::flags`].
/// - [`fragment_density_map_non_subsampled_images`] specifies whether the implementation supports regular non-subsampled image attachments with fragment density map render passes. If this feature is not enabled, render passes with a [fragment density map attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment) **must**  only have [subsampled attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-subsamplesampler) bound.
///If the [`PhysicalDeviceFragmentDensityMapFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceFragmentDensityMapFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT`
///# Related
/// - [`VK_EXT_fragment_density_map`]
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
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapFeaturesEXT")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapFeaturesEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`fragment_density_map`] specifies
    ///whether the implementation supports render passes with a fragment
    ///density map attachment.
    ///If this feature is not enabled and the [`p_next`] chain of
    ///[`RenderPassCreateInfo`] includes a
    ///[`RenderPassFragmentDensityMapCreateInfoEXT`] structure,
    ///`fragmentDensityMapAttachment` **must**  be [`ATTACHMENT_UNUSED`].
    pub fragment_density_map: Bool32,
    ///[`fragment_density_map_dynamic`]
    ///specifies whether the implementation supports dynamic fragment density
    ///map image views.
    ///If this feature is not enabled,
    ///`VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT` **must**
    ///not be included in [`ImageViewCreateInfo`]::`flags`.
    pub fragment_density_map_dynamic: Bool32,
    ///[`fragment_density_map_non_subsampled_images`] specifies whether the
    ///implementation supports regular non-subsampled image attachments with
    ///fragment density map render passes.
    ///If this feature is not enabled, render passes with a
    ///[fragment density map
    ///attachment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-fragmentdensitymapattachment) **must**  only have [subsampled
    ///attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-subsamplesampler) bound.
    pub fragment_density_map_non_subsampled_images: Bool32,
}
impl<'lt> Default for PhysicalDeviceFragmentDensityMapFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            fragment_density_map: 0,
            fragment_density_map_dynamic: 0,
            fragment_density_map_non_subsampled_images: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentDensityMapFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::fragment_density_map`]
    pub fn fragment_density_map_raw(&self) -> Bool32 {
        self.fragment_density_map
    }
    ///Gets the raw value of [`Self::fragment_density_map_dynamic`]
    pub fn fragment_density_map_dynamic_raw(&self) -> Bool32 {
        self.fragment_density_map_dynamic
    }
    ///Gets the raw value of [`Self::fragment_density_map_non_subsampled_images`]
    pub fn fragment_density_map_non_subsampled_images_raw(&self) -> Bool32 {
        self.fragment_density_map_non_subsampled_images
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_map`]
    pub fn set_fragment_density_map_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_density_map = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_map_dynamic`]
    pub fn set_fragment_density_map_dynamic_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_density_map_dynamic = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_map_non_subsampled_images`]
    pub fn set_fragment_density_map_non_subsampled_images_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_density_map_non_subsampled_images = value;
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
    ///Gets the value of [`Self::fragment_density_map`]
    pub fn fragment_density_map(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_density_map as u8) }
    }
    ///Gets the value of [`Self::fragment_density_map_dynamic`]
    pub fn fragment_density_map_dynamic(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_density_map_dynamic as u8) }
    }
    ///Gets the value of [`Self::fragment_density_map_non_subsampled_images`]
    pub fn fragment_density_map_non_subsampled_images(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_density_map_non_subsampled_images as u8) }
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
    ///Gets a mutable reference to the value of [`Self::fragment_density_map`]
    pub fn fragment_density_map_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_density_map as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_density_map as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::fragment_density_map_dynamic`]
    pub fn fragment_density_map_dynamic_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_density_map_dynamic as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_density_map_dynamic as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::fragment_density_map_non_subsampled_images`]
    pub fn fragment_density_map_non_subsampled_images_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_density_map_non_subsampled_images as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_density_map_non_subsampled_images as *mut Bool32)
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
    ///Sets the raw value of [`Self::fragment_density_map`]
    pub fn set_fragment_density_map(&mut self, value: bool) -> &mut Self {
        self.fragment_density_map = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_map_dynamic`]
    pub fn set_fragment_density_map_dynamic(&mut self, value: bool) -> &mut Self {
        self.fragment_density_map_dynamic = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_map_non_subsampled_images`]
    pub fn set_fragment_density_map_non_subsampled_images(&mut self, value: bool) -> &mut Self {
        self.fragment_density_map_non_subsampled_images = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceFragmentDensityMapPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentDensityMapPropertiesEXT.html) - Structure describing fragment density map properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceFragmentDensityMapPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_fragment_density_map
///typedef struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkExtent2D         minFragmentDensityTexelSize;
///    VkExtent2D         maxFragmentDensityTexelSize;
///    VkBool32           fragmentDensityInvocations;
///} VkPhysicalDeviceFragmentDensityMapPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_fragment_density_texel_size`] is the minimum [fragment density texel size](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-fragment-density-texel-size).
/// - [`max_fragment_density_texel_size`] is the maximum fragment density texel size.
/// - [`fragment_density_invocations`] specifies whether the implementation  **may**  invoke
///   additional fragment shader invocations for each covered sample.
///# Description
///If the [`PhysicalDeviceFragmentDensityMapPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_fragment_density_map`]
/// - [`Bool32`]
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
#[doc(alias = "VkPhysicalDeviceFragmentDensityMapPropertiesEXT")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceFragmentDensityMapPropertiesEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`min_fragment_density_texel_size`]
    ///is the minimum [fragment density
    ///texel size](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#glossary-fragment-density-texel-size).
    pub min_fragment_density_texel_size: Extent2D,
    ///[`max_fragment_density_texel_size`]
    ///is the maximum fragment density texel size.
    pub max_fragment_density_texel_size: Extent2D,
    ///[`fragment_density_invocations`]
    ///specifies whether the implementation  **may**  invoke additional fragment
    ///shader invocations for each covered sample.
    pub fragment_density_invocations: Bool32,
}
impl<'lt> Default for PhysicalDeviceFragmentDensityMapPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            min_fragment_density_texel_size: Default::default(),
            max_fragment_density_texel_size: Default::default(),
            fragment_density_invocations: 0,
        }
    }
}
impl<'lt> PhysicalDeviceFragmentDensityMapPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::fragment_density_invocations`]
    pub fn fragment_density_invocations_raw(&self) -> Bool32 {
        self.fragment_density_invocations
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_invocations`]
    pub fn set_fragment_density_invocations_raw(&mut self, value: Bool32) -> &mut Self {
        self.fragment_density_invocations = value;
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
    ///Gets the value of [`Self::min_fragment_density_texel_size`]
    pub fn min_fragment_density_texel_size(&self) -> Extent2D {
        self.min_fragment_density_texel_size
    }
    ///Gets the value of [`Self::max_fragment_density_texel_size`]
    pub fn max_fragment_density_texel_size(&self) -> Extent2D {
        self.max_fragment_density_texel_size
    }
    ///Gets the value of [`Self::fragment_density_invocations`]
    pub fn fragment_density_invocations(&self) -> bool {
        unsafe { std::mem::transmute(self.fragment_density_invocations as u8) }
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
    ///Gets a mutable reference to the value of [`Self::min_fragment_density_texel_size`]
    pub fn min_fragment_density_texel_size_mut(&mut self) -> &mut Extent2D {
        &mut self.min_fragment_density_texel_size
    }
    ///Gets a mutable reference to the value of [`Self::max_fragment_density_texel_size`]
    pub fn max_fragment_density_texel_size_mut(&mut self) -> &mut Extent2D {
        &mut self.max_fragment_density_texel_size
    }
    ///Gets a mutable reference to the value of [`Self::fragment_density_invocations`]
    pub fn fragment_density_invocations_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.fragment_density_invocations as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.fragment_density_invocations as *mut Bool32)
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
    ///Sets the raw value of [`Self::min_fragment_density_texel_size`]
    pub fn set_min_fragment_density_texel_size(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.min_fragment_density_texel_size = value;
        self
    }
    ///Sets the raw value of [`Self::max_fragment_density_texel_size`]
    pub fn set_max_fragment_density_texel_size(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_fragment_density_texel_size = value;
        self
    }
    ///Sets the raw value of [`Self::fragment_density_invocations`]
    pub fn set_fragment_density_invocations(&mut self, value: bool) -> &mut Self {
        self.fragment_density_invocations = value as u8 as u32;
        self
    }
}
///[VkRenderPassFragmentDensityMapCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassFragmentDensityMapCreateInfoEXT.html) - Structure containing fragment density map attachment for render pass
///# C Specifications
///If the [`RenderPassCreateInfo`]::[`p_next`] chain includes a
///[`RenderPassFragmentDensityMapCreateInfoEXT`] structure, then that
///structure includes a fragment density map attachment for the render pass.The
/// [`RenderPassFragmentDensityMapCreateInfoEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_fragment_density_map
///typedef struct VkRenderPassFragmentDensityMapCreateInfoEXT {
///    VkStructureType          sType;
///    const void*              pNext;
///    VkAttachmentReference    fragmentDensityMapAttachment;
///} VkRenderPassFragmentDensityMapCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`fragment_density_map_attachment`] is the fragment density map to use for the render pass.
///# Description
///The fragment density map is read at an implementation-dependent time with
///the following constraints determined by the attachment’s image view
///`flags`:
/// - `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT` specifies that the fragment
///   density map will be read by the device during
///   `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - `VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT` specifies that the fragment
///   density map will be read by the host during [`EndCommandBuffer`] of the primary command buffer
///   that the render pass is recorded into
/// - Otherwise the fragment density map will be read by the host during [`CmdBeginRenderPass`]
///The fragment density map  **may**  additionally be read by the device during
///`VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT` for any mode.If this structure is not
/// present, it is as if
///[`fragment_density_map_attachment`] was given as [`ATTACHMENT_UNUSED`].
///## Valid Usage
/// - If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`],
///   [`fragment_density_map_attachment`] **must**  not be an element of
///   [`SubpassDescription::input_attachments`], [`SubpassDescription::color_attachments`],
///   [`SubpassDescription::resolve_attachments`], [`SubpassDescription::depth_stencil_attachment`],
///   or [`SubpassDescription::preserve_attachments`] for any subpass
/// - If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`], `layout` **must**  be
///   equal to `VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT`, or `VK_IMAGE_LAYOUT_GENERAL`
/// - If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`],
///   [`fragment_density_map_attachment`] **must**  reference an attachment with a `loadOp` equal to
///   `VK_ATTACHMENT_LOAD_OP_LOAD` or `VK_ATTACHMENT_LOAD_OP_DONT_CARE`
/// - If [`fragment_density_map_attachment`] is not [`ATTACHMENT_UNUSED`],
///   [`fragment_density_map_attachment`] **must**  reference an attachment with a `storeOp` equal
///   to `VK_ATTACHMENT_STORE_OP_DONT_CARE`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT`
/// - [`fragment_density_map_attachment`] **must**  be a valid [`AttachmentReference`] structure
///# Related
/// - [`VK_EXT_fragment_density_map`]
/// - [`AttachmentReference`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkRenderPassFragmentDensityMapCreateInfoEXT")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct RenderPassFragmentDensityMapCreateInfoEXT<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`fragment_density_map_attachment`] is the fragment density map to use
    ///for the render pass.
    pub fragment_density_map_attachment: AttachmentReference,
}
impl<'lt> Default for RenderPassFragmentDensityMapCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            fragment_density_map_attachment: Default::default(),
        }
    }
}
impl<'lt> RenderPassFragmentDensityMapCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::fragment_density_map_attachment`]
    pub fn fragment_density_map_attachment(&self) -> AttachmentReference {
        self.fragment_density_map_attachment
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::fragment_density_map_attachment`]
    pub fn fragment_density_map_attachment_mut(&mut self) -> &mut AttachmentReference {
        &mut self.fragment_density_map_attachment
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
    ///Sets the raw value of [`Self::fragment_density_map_attachment`]
    pub fn set_fragment_density_map_attachment(&mut self, value: crate::vulkan1_0::AttachmentReference) -> &mut Self {
        self.fragment_density_map_attachment = value;
        self
    }
}
