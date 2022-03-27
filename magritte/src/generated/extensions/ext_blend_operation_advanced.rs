use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION")]
pub const EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME")]
pub const EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_blend_operation_advanced");
///[VkBlendOverlapEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOverlapEXT.html) - Enumerant specifying the blend overlap parameter
///# C Specifications
///The weighting functions p<sub>0</sub>, p<sub>1</sub>, and p<sub>2</sub> are defined
///in table [Advanced Blend Overlap
///Modes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced-overlap-modes).
///In these functions, the A components of the source and destination colors
///are taken to indicate the portion of the pixel covered by the fragment
///(source) and the fragments previously accumulated in the pixel
///(destination).
///The functions p<sub>0</sub>, p<sub>1</sub>, and p<sub>2</sub> approximate the
///relative portion of the pixel covered by the intersection of the source and
///destination, covered only by the source, and covered only by the
///destination, respectively.Possible values of
///[`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`],
///specifying the blend overlap functions, are:
///```c
///// Provided by VK_EXT_blend_operation_advanced
///typedef enum VkBlendOverlapEXT {
///    VK_BLEND_OVERLAP_UNCORRELATED_EXT = 0,
///    VK_BLEND_OVERLAP_DISJOINT_EXT = 1,
///    VK_BLEND_OVERLAP_CONJOINT_EXT = 2,
///} VkBlendOverlapEXT;
///```
///# Description
/// - [`BlendOverlapUncorrelatedExt`] specifies that there is no correlation between the source and
///   destination coverage.
/// - [`BlendOverlapConjointExt`] specifies that the source and destination coverage are considered
///   to have maximal overlap.
/// - [`BlendOverlapDisjointExt`] specifies that the source and destination coverage are considered
///   to have minimal overlap.
///# Related
/// - [`VK_EXT_blend_operation_advanced`]
/// - [`PipelineColorBlendAdvancedStateCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBlendOverlapEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum BlendOverlapEXT {
    ///[`BlendOverlapUncorrelatedExt`] specifies that there is no
    ///correlation between the source and destination coverage.
    BlendOverlapUncorrelatedExt = 0,
    ///[`BlendOverlapDisjointExt`] specifies that the source and
    ///destination coverage are considered to have minimal overlap.
    BlendOverlapDisjointExt = 1,
    ///[`BlendOverlapConjointExt`] specifies that the source and
    ///destination coverage are considered to have maximal overlap.
    BlendOverlapConjointExt = 2,
}
impl const Default for BlendOverlapEXT {
    fn default() -> Self {
        BlendOverlapUncorrelatedExt
    }
}
impl BlendOverlapEXT {
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
///[VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html) - Structure describing advanced blending features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_blend_operation_advanced
///typedef struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           advancedBlendCoherentOperations;
///} VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT;
///```
///# Members
///This structure describes the following feature:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`advanced_blend_coherent_operations`] specifies whether blending using [advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced)
///   is guaranteed to execute atomically and in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
///   If this is [`TRUE`], `VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is treated the same
///   as `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`, and advanced blending needs no additional
///   synchronization over basic blending. If this is [`FALSE`], then memory dependencies are
///   required to guarantee order between two advanced blending operations that occur on the same
///   sample.
///If the [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`]**can** also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT`
///# Related
/// - [`VK_EXT_blend_operation_advanced`]
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
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`advanced_blend_coherent_operations`] specifies whether blending using
    ///[advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced) is guaranteed
    ///to execute atomically and in [primitive
    ///order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
    ///If this is [`TRUE`],
    ///`VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is treated the
    ///same as `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`, and advanced blending
    ///needs no additional synchronization over basic blending.
    ///If this is [`FALSE`], then memory dependencies are required to
    ///guarantee order between two advanced blending operations that occur on
    ///the same sample.
    advanced_blend_coherent_operations: Bool32,
}
impl<'lt> Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            advanced_blend_coherent_operations: 0,
        }
    }
}
impl<'lt> PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::advanced_blend_coherent_operations`]
    pub fn advanced_blend_coherent_operations_raw(&self) -> Bool32 {
        self.advanced_blend_coherent_operations
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_coherent_operations`]
    pub fn set_advanced_blend_coherent_operations_raw(&mut self, value: Bool32) -> &mut Self {
        self.advanced_blend_coherent_operations = value;
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
    ///Gets the value of [`Self::advanced_blend_coherent_operations`]
    pub fn advanced_blend_coherent_operations(&self) -> bool {
        unsafe { std::mem::transmute(self.advanced_blend_coherent_operations as u8) }
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
    ///Gets a mutable reference to the value of [`Self::advanced_blend_coherent_operations`]
    pub fn advanced_blend_coherent_operations_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.advanced_blend_coherent_operations as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.advanced_blend_coherent_operations as *mut Bool32)
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
    ///Sets the raw value of [`Self::advanced_blend_coherent_operations`]
    pub fn set_advanced_blend_coherent_operations(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_coherent_operations = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html) - Structure describing advanced blending limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_blend_operation_advanced
///typedef struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           advancedBlendMaxColorAttachments;
///    VkBool32           advancedBlendIndependentBlend;
///    VkBool32           advancedBlendNonPremultipliedSrcColor;
///    VkBool32           advancedBlendNonPremultipliedDstColor;
///    VkBool32           advancedBlendCorrelatedOverlap;
///    VkBool32           advancedBlendAllOperations;
///} VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`advanced_blend_max_color_attachments`] is one greater than the highest color attachment index that **can** be used in a subpass, for a pipeline that uses an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced).
/// - [`advanced_blend_independent_blend`] specifies whether advanced blend operations **can** vary
///   per-attachment.
/// - [`advanced_blend_non_premultiplied_src_color`] specifies whether the source color **can** be
///   treated as non-premultiplied. If this is [`FALSE`], then
///   [`PipelineColorBlendAdvancedStateCreateInfoEXT::src_premultiplied`]**must** be [`TRUE`].
/// - [`advanced_blend_non_premultiplied_dst_color`] specifies whether the destination color **can**
///   be treated as non-premultiplied. If this is [`FALSE`], then
///   [`PipelineColorBlendAdvancedStateCreateInfoEXT::dst_premultiplied`]**must** be [`TRUE`].
/// - [`advanced_blend_correlated_overlap`] specifies whether the overlap mode **can** be treated as
///   correlated. If this is [`FALSE`], then
///   [`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`]**must** be
///   `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
/// - [`advanced_blend_all_operations`] specifies whether all advanced blend operation enums are
///   supported. See the valid usage of [`PipelineColorBlendAttachmentState`].
///# Description
///If the [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT`
///# Related
/// - [`VK_EXT_blend_operation_advanced`]
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
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
    ///[`advanced_blend_max_color_attachments`] is one greater than the highest
    ///color attachment index that **can** be used in a subpass, for a pipeline
    ///that uses an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced).
    advanced_blend_max_color_attachments: u32,
    ///[`advanced_blend_independent_blend`] specifies whether advanced blend
    ///operations **can** vary per-attachment.
    advanced_blend_independent_blend: Bool32,
    ///[`advanced_blend_non_premultiplied_src_color`] specifies whether the source
    ///color **can** be treated as non-premultiplied.
    ///If this is [`FALSE`], then
    ///[`PipelineColorBlendAdvancedStateCreateInfoEXT`]::`srcPremultiplied`**must** be [`TRUE`].
    advanced_blend_non_premultiplied_src_color: Bool32,
    ///[`advanced_blend_non_premultiplied_dst_color`] specifies whether the
    ///destination color **can** be treated as non-premultiplied.
    ///If this is [`FALSE`], then
    ///[`PipelineColorBlendAdvancedStateCreateInfoEXT`]::`dstPremultiplied`**must** be [`TRUE`].
    advanced_blend_non_premultiplied_dst_color: Bool32,
    ///[`advanced_blend_correlated_overlap`] specifies whether the overlap mode
    ///**can** be treated as correlated.
    ///If this is [`FALSE`], then
    ///[`PipelineColorBlendAdvancedStateCreateInfoEXT`]::`blendOverlap`**must** be `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
    advanced_blend_correlated_overlap: Bool32,
    ///[`advanced_blend_all_operations`]
    ///specifies whether all advanced blend operation enums are supported.
    ///See the valid usage of [`PipelineColorBlendAttachmentState`].
    advanced_blend_all_operations: Bool32,
}
impl<'lt> Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            advanced_blend_max_color_attachments: 0,
            advanced_blend_independent_blend: 0,
            advanced_blend_non_premultiplied_src_color: 0,
            advanced_blend_non_premultiplied_dst_color: 0,
            advanced_blend_correlated_overlap: 0,
            advanced_blend_all_operations: 0,
        }
    }
}
impl<'lt> PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::advanced_blend_max_color_attachments`]
    pub fn advanced_blend_max_color_attachments_raw(&self) -> u32 {
        self.advanced_blend_max_color_attachments
    }
    ///Gets the raw value of [`Self::advanced_blend_independent_blend`]
    pub fn advanced_blend_independent_blend_raw(&self) -> Bool32 {
        self.advanced_blend_independent_blend
    }
    ///Gets the raw value of [`Self::advanced_blend_non_premultiplied_src_color`]
    pub fn advanced_blend_non_premultiplied_src_color_raw(&self) -> Bool32 {
        self.advanced_blend_non_premultiplied_src_color
    }
    ///Gets the raw value of [`Self::advanced_blend_non_premultiplied_dst_color`]
    pub fn advanced_blend_non_premultiplied_dst_color_raw(&self) -> Bool32 {
        self.advanced_blend_non_premultiplied_dst_color
    }
    ///Gets the raw value of [`Self::advanced_blend_correlated_overlap`]
    pub fn advanced_blend_correlated_overlap_raw(&self) -> Bool32 {
        self.advanced_blend_correlated_overlap
    }
    ///Gets the raw value of [`Self::advanced_blend_all_operations`]
    pub fn advanced_blend_all_operations_raw(&self) -> Bool32 {
        self.advanced_blend_all_operations
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_max_color_attachments`]
    pub fn set_advanced_blend_max_color_attachments_raw(&mut self, value: u32) -> &mut Self {
        self.advanced_blend_max_color_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_independent_blend`]
    pub fn set_advanced_blend_independent_blend_raw(&mut self, value: Bool32) -> &mut Self {
        self.advanced_blend_independent_blend = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_non_premultiplied_src_color`]
    pub fn set_advanced_blend_non_premultiplied_src_color_raw(&mut self, value: Bool32) -> &mut Self {
        self.advanced_blend_non_premultiplied_src_color = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_non_premultiplied_dst_color`]
    pub fn set_advanced_blend_non_premultiplied_dst_color_raw(&mut self, value: Bool32) -> &mut Self {
        self.advanced_blend_non_premultiplied_dst_color = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_correlated_overlap`]
    pub fn set_advanced_blend_correlated_overlap_raw(&mut self, value: Bool32) -> &mut Self {
        self.advanced_blend_correlated_overlap = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_all_operations`]
    pub fn set_advanced_blend_all_operations_raw(&mut self, value: Bool32) -> &mut Self {
        self.advanced_blend_all_operations = value;
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
    ///Gets the value of [`Self::advanced_blend_max_color_attachments`]
    pub fn advanced_blend_max_color_attachments(&self) -> u32 {
        self.advanced_blend_max_color_attachments
    }
    ///Gets the value of [`Self::advanced_blend_independent_blend`]
    pub fn advanced_blend_independent_blend(&self) -> bool {
        unsafe { std::mem::transmute(self.advanced_blend_independent_blend as u8) }
    }
    ///Gets the value of [`Self::advanced_blend_non_premultiplied_src_color`]
    pub fn advanced_blend_non_premultiplied_src_color(&self) -> bool {
        unsafe { std::mem::transmute(self.advanced_blend_non_premultiplied_src_color as u8) }
    }
    ///Gets the value of [`Self::advanced_blend_non_premultiplied_dst_color`]
    pub fn advanced_blend_non_premultiplied_dst_color(&self) -> bool {
        unsafe { std::mem::transmute(self.advanced_blend_non_premultiplied_dst_color as u8) }
    }
    ///Gets the value of [`Self::advanced_blend_correlated_overlap`]
    pub fn advanced_blend_correlated_overlap(&self) -> bool {
        unsafe { std::mem::transmute(self.advanced_blend_correlated_overlap as u8) }
    }
    ///Gets the value of [`Self::advanced_blend_all_operations`]
    pub fn advanced_blend_all_operations(&self) -> bool {
        unsafe { std::mem::transmute(self.advanced_blend_all_operations as u8) }
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
    ///Gets a mutable reference to the value of [`Self::advanced_blend_max_color_attachments`]
    pub fn advanced_blend_max_color_attachments_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::advanced_blend_independent_blend`]
    pub fn advanced_blend_independent_blend_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.advanced_blend_independent_blend as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.advanced_blend_independent_blend as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::advanced_blend_non_premultiplied_src_color`]
    pub fn advanced_blend_non_premultiplied_src_color_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.advanced_blend_non_premultiplied_src_color as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.advanced_blend_non_premultiplied_src_color as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::advanced_blend_non_premultiplied_dst_color`]
    pub fn advanced_blend_non_premultiplied_dst_color_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.advanced_blend_non_premultiplied_dst_color as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.advanced_blend_non_premultiplied_dst_color as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::advanced_blend_correlated_overlap`]
    pub fn advanced_blend_correlated_overlap_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.advanced_blend_correlated_overlap as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.advanced_blend_correlated_overlap as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::advanced_blend_all_operations`]
    pub fn advanced_blend_all_operations_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.advanced_blend_all_operations as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.advanced_blend_all_operations as *mut Bool32)
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
    ///Sets the raw value of [`Self::advanced_blend_max_color_attachments`]
    pub fn set_advanced_blend_max_color_attachments(&mut self, value: u32) -> &mut Self {
        self.advanced_blend_max_color_attachments = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_independent_blend`]
    pub fn set_advanced_blend_independent_blend(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_independent_blend = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_non_premultiplied_src_color`]
    pub fn set_advanced_blend_non_premultiplied_src_color(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_non_premultiplied_src_color = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_non_premultiplied_dst_color`]
    pub fn set_advanced_blend_non_premultiplied_dst_color(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_non_premultiplied_dst_color = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_correlated_overlap`]
    pub fn set_advanced_blend_correlated_overlap(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_correlated_overlap = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_all_operations`]
    pub fn set_advanced_blend_all_operations(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_all_operations = value as u8 as u32;
        self
    }
}
///[VkPipelineColorBlendAdvancedStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html) - Structure specifying parameters that affect advanced blend operations
///# C Specifications
///If the [`p_next`] chain of [`PipelineColorBlendStateCreateInfo`]
///includes a [`PipelineColorBlendAdvancedStateCreateInfoEXT`] structure,
///then that structure includes parameters that affect advanced blend
///operations.The [`PipelineColorBlendAdvancedStateCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_blend_operation_advanced
///typedef struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
///    VkStructureType      sType;
///    const void*          pNext;
///    VkBool32             srcPremultiplied;
///    VkBool32             dstPremultiplied;
///    VkBlendOverlapEXT    blendOverlap;
///} VkPipelineColorBlendAdvancedStateCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_premultiplied`] specifies whether the source color of the blend operation is treated as
///   premultiplied.
/// - [`dst_premultiplied`] specifies whether the destination color of the blend operation is
///   treated as premultiplied.
/// - [`blend_overlap`] is a [`BlendOverlapEXT`] value specifying how the source and destination
///   sample’s coverage is correlated.
///# Description
///If this structure is not present, [`src_premultiplied`] and
///[`dst_premultiplied`] are both considered to be [`TRUE`], and
///[`blend_overlap`] is considered to be
///`VK_BLEND_OVERLAP_UNCORRELATED_EXT`.Valid Usage
/// - If the [non-premultiplied source color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendNonPremultipliedSrcColor)
///   property is not supported, [`src_premultiplied`]**must** be [`TRUE`]
/// - If the [non-premultiplied destination color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendNonPremultipliedDstColor)
///   property is not supported, [`dst_premultiplied`]**must** be [`TRUE`]
/// - If the [correlated overlap](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendCorrelatedOverlap)
///   property is not supported, [`blend_overlap`]**must** be `VK_BLEND_OVERLAP_UNCORRELATED_EXT`
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT`
/// - [`blend_overlap`]**must** be a valid [`BlendOverlapEXT`] value
///# Related
/// - [`VK_EXT_blend_operation_advanced`]
/// - [`BlendOverlapEXT`]
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`src_premultiplied`] specifies whether the source color of the blend
    ///operation is treated as premultiplied.
    src_premultiplied: Bool32,
    ///[`dst_premultiplied`] specifies whether the destination color of the
    ///blend operation is treated as premultiplied.
    dst_premultiplied: Bool32,
    ///[`blend_overlap`] is a [`BlendOverlapEXT`] value specifying how the
    ///source and destination sample’s coverage is correlated.
    blend_overlap: BlendOverlapEXT,
}
impl<'lt> Default for PipelineColorBlendAdvancedStateCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_premultiplied: 0,
            dst_premultiplied: 0,
            blend_overlap: Default::default(),
        }
    }
}
impl<'lt> PipelineColorBlendAdvancedStateCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::src_premultiplied`]
    pub fn src_premultiplied_raw(&self) -> Bool32 {
        self.src_premultiplied
    }
    ///Gets the raw value of [`Self::dst_premultiplied`]
    pub fn dst_premultiplied_raw(&self) -> Bool32 {
        self.dst_premultiplied
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::src_premultiplied`]
    pub fn set_src_premultiplied_raw(&mut self, value: Bool32) -> &mut Self {
        self.src_premultiplied = value;
        self
    }
    ///Sets the raw value of [`Self::dst_premultiplied`]
    pub fn set_dst_premultiplied_raw(&mut self, value: Bool32) -> &mut Self {
        self.dst_premultiplied = value;
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
    ///Gets the value of [`Self::src_premultiplied`]
    pub fn src_premultiplied(&self) -> bool {
        unsafe { std::mem::transmute(self.src_premultiplied as u8) }
    }
    ///Gets the value of [`Self::dst_premultiplied`]
    pub fn dst_premultiplied(&self) -> bool {
        unsafe { std::mem::transmute(self.dst_premultiplied as u8) }
    }
    ///Gets the value of [`Self::blend_overlap`]
    pub fn blend_overlap(&self) -> BlendOverlapEXT {
        self.blend_overlap
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_premultiplied`]
    pub fn src_premultiplied_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.src_premultiplied as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.src_premultiplied as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::dst_premultiplied`]
    pub fn dst_premultiplied_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.dst_premultiplied as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.dst_premultiplied as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::blend_overlap`]
    pub fn blend_overlap_mut(&mut self) -> &mut BlendOverlapEXT {
        &mut self.blend_overlap
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
    ///Sets the raw value of [`Self::src_premultiplied`]
    pub fn set_src_premultiplied(&mut self, value: bool) -> &mut Self {
        self.src_premultiplied = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::dst_premultiplied`]
    pub fn set_dst_premultiplied(&mut self, value: bool) -> &mut Self {
        self.dst_premultiplied = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::blend_overlap`]
    pub fn set_blend_overlap(
        &mut self,
        value: crate::extensions::ext_blend_operation_advanced::BlendOverlapEXT,
    ) -> &mut Self {
        self.blend_overlap = value;
        self
    }
}
