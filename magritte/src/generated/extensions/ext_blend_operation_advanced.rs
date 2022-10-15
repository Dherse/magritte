//![VK_EXT_blend_operation_advanced](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_blend_operation_advanced.html) - device extension
//!# Description
//!This extension adds a number of “advanced” blending operations that  **can**
//!be used to perform new color blending operations, many of which are more
//!complex than the standard blend modes provided by unextended Vulkan.
//!This extension requires different styles of usage, depending on the level of
//!hardware support and the enabled features:
//! - If [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT::advanced_blend_coherent_operations`] is
//!   [`FALSE`], the new blending operations are supported, but a memory dependency  **must**
//!   separate each advanced blend operation on a given sample.
//!   `VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is used to synchronize reads using
//!   advanced blend operations.
//! - If [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT::advanced_blend_coherent_operations`] is
//!   [`TRUE`], advanced blend operations obey primitive order just like basic blend operations.
//!In unextended Vulkan, the set of blending operations is limited, and  **can**  be
//!expressed very simply.
//!The `VK_BLEND_OP_MIN` and `VK_BLEND_OP_MAX` blend operations simply
//!compute component-wise minimums or maximums of source and destination color
//!components.
//!The `VK_BLEND_OP_ADD`, `VK_BLEND_OP_SUBTRACT`, and
//!`VK_BLEND_OP_REVERSE_SUBTRACT` modes multiply the source and destination
//!colors by source and destination factors and either add the two products
//!together or subtract one from the other.
//!This limited set of operations supports many common blending operations but
//!precludes the use of more sophisticated transparency and blending operations
//!commonly available in many dedicated imaging APIs.This extension provides a number of new
//! “advanced” blending operations.
//!Unlike traditional blending operations using `VK_BLEND_OP_ADD`, these
//!blending equations do not use source and destination factors specified by
//![`BlendFactor`].
//!Instead, each blend operation specifies a complete equation based on the
//!source and destination colors.
//!These new blend operations are used for both RGB and alpha components; they
//! **must**  not be used to perform separate RGB and alpha blending (via different
//!values of color and alpha [`BlendOp`]).These blending operations are performed using
//! premultiplied colors, where
//!RGB colors  **can**  be considered premultiplied or non-premultiplied by alpha,
//!according to the `srcPremultiplied` and `dstPremultiplied` members
//!of [`PipelineColorBlendAdvancedStateCreateInfoEXT`].
//!If a color is considered non-premultiplied, the (R,G,B) color components are
//!multiplied by the alpha component prior to blending.
//!For non-premultiplied color components in the range [0,1], the
//!corresponding premultiplied color component would have values in the range
//![0 × A, 1 × A].Many of these advanced blending equations are formulated where the result of
//!blending source and destination colors with partial coverage have three
//!separate contributions: from the portions covered by both the source and the
//!destination, from the portion covered only by the source, and from the
//!portion covered only by the destination.
//!The blend parameter
//![`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`] **can**  be used to specify a
//! correlation between source and destination pixel
//!coverage.
//!If set to `VK_BLEND_OVERLAP_CONJOINT_EXT`, the source and destination
//!are considered to have maximal overlap, as would be the case if drawing two
//!objects on top of each other.
//!If set to `VK_BLEND_OVERLAP_DISJOINT_EXT`, the source and destination
//!are considered to have minimal overlap, as would be the case when rendering
//!a complex polygon tessellated into individual non-intersecting triangles.
//!If set to `VK_BLEND_OVERLAP_UNCORRELATED_EXT`, the source and
//!destination coverage are assumed to have no spatial correlation within the
//!pixel.In addition to the coherency issues on implementations not supporting
//!`advancedBlendCoherentOperations`, this extension has several
//!limitations worth noting.
//!First, the new blend operations have a limit on the number of color
//!attachments they  **can**  be used with, as indicated by
//![`PhysicalDeviceBlendOperationAdvancedPropertiesEXT::advanced_blend_max_color_attachments`].
//!Additionally, blending precision  **may**  be limited to 16-bit floating-point,
//!which  **may**  result in a loss of precision and dynamic range for framebuffer
//!formats with 32-bit floating-point components, and in a loss of precision
//!for formats with 12- and 16-bit signed or unsigned normalized integer
//!components.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_blend_operation_advanced]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_blend_operation_advanced extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  -
//!   [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`]
//! - Extending [`PipelineColorBlendStateCreateInfo`]:  -
//!   [`PipelineColorBlendAdvancedStateCreateInfoEXT`]
//!# New enums
//! - [`BlendOverlapEXT`]
//!# New constants
//! - [`EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME`]
//! - [`EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`
//! - Extending [`BlendOp`]:  - `VK_BLEND_OP_BLUE_EXT`  - `VK_BLEND_OP_COLORBURN_EXT`  -
//!   `VK_BLEND_OP_COLORDODGE_EXT`  - `VK_BLEND_OP_CONTRAST_EXT`  - `VK_BLEND_OP_DARKEN_EXT`  -
//!   `VK_BLEND_OP_DIFFERENCE_EXT`  - `VK_BLEND_OP_DST_ATOP_EXT`  - `VK_BLEND_OP_DST_EXT`  -
//!   `VK_BLEND_OP_DST_IN_EXT`  - `VK_BLEND_OP_DST_OUT_EXT`  - `VK_BLEND_OP_DST_OVER_EXT`  -
//!   `VK_BLEND_OP_EXCLUSION_EXT`  - `VK_BLEND_OP_GREEN_EXT`  - `VK_BLEND_OP_HARDLIGHT_EXT`  -
//!   `VK_BLEND_OP_HARDMIX_EXT`  - `VK_BLEND_OP_HSL_COLOR_EXT`  - `VK_BLEND_OP_HSL_HUE_EXT`  -
//!   `VK_BLEND_OP_HSL_LUMINOSITY_EXT`  - `VK_BLEND_OP_HSL_SATURATION_EXT`  -
//!   `VK_BLEND_OP_INVERT_EXT`  - `VK_BLEND_OP_INVERT_OVG_EXT`  - `VK_BLEND_OP_INVERT_RGB_EXT`  -
//!   `VK_BLEND_OP_LIGHTEN_EXT`  - `VK_BLEND_OP_LINEARBURN_EXT`  - `VK_BLEND_OP_LINEARDODGE_EXT`  -
//!   `VK_BLEND_OP_LINEARLIGHT_EXT`  - `VK_BLEND_OP_MINUS_CLAMPED_EXT`  - `VK_BLEND_OP_MINUS_EXT`  -
//!   `VK_BLEND_OP_MULTIPLY_EXT`  - `VK_BLEND_OP_OVERLAY_EXT`  - `VK_BLEND_OP_PINLIGHT_EXT`  -
//!   `VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT`  - `VK_BLEND_OP_PLUS_CLAMPED_EXT`  -
//!   `VK_BLEND_OP_PLUS_DARKER_EXT`  - `VK_BLEND_OP_PLUS_EXT`  - `VK_BLEND_OP_RED_EXT`  -
//!   `VK_BLEND_OP_SCREEN_EXT`  - `VK_BLEND_OP_SOFTLIGHT_EXT`  - `VK_BLEND_OP_SRC_ATOP_EXT`  -
//!   `VK_BLEND_OP_SRC_EXT`  - `VK_BLEND_OP_SRC_IN_EXT`  - `VK_BLEND_OP_SRC_OUT_EXT`  -
//!   `VK_BLEND_OP_SRC_OVER_EXT`  - `VK_BLEND_OP_VIVIDLIGHT_EXT`  - `VK_BLEND_OP_XOR_EXT`  -
//!   `VK_BLEND_OP_ZERO_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT`
//!# Known issues & F.A.Q.
//!None.
//!# Version history
//! - Revision 1, 2017-06-12 (Jeff Bolz)  - Internal revisions
//! - Revision 2, 2017-06-12 (Jeff Bolz)  - Internal revisions
//!# Other information
//! * 2017-06-12
//! * - Jeff Bolz, NVIDIA
//!# Related
//! - [`BlendOverlapEXT`]
//! - [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`]
//! - [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`]
//! - [`PipelineColorBlendAdvancedStateCreateInfoEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
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
/// # Description
/// - [`UNCORRELATED`] specifies that there is no correlation between the source and destination
///   coverage.
/// - [`CONJOINT`] specifies that the source and destination coverage are considered to have maximal
///   overlap.
/// - [`DISJOINT`] specifies that the source and destination coverage are considered to have minimal
///   overlap.
/// # Related
/// - [`ext_blend_operation_advanced`]
/// - [`PipelineColorBlendAdvancedStateCreateInfoEXT`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBlendOverlapEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct BlendOverlapEXT(i32);
impl const Default for BlendOverlapEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl BlendOverlapEXT {
    ///[`UNCORRELATED`] specifies that there is no
    ///correlation between the source and destination coverage.
    pub const UNCORRELATED: Self = Self(0);
    ///[`DISJOINT`] specifies that the source and
    ///destination coverage are considered to have minimal overlap.
    pub const DISJOINT: Self = Self(1);
    ///[`CONJOINT`] specifies that the source and
    ///destination coverage are considered to have maximal overlap.
    pub const CONJOINT: Self = Self(2);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for BlendOverlapEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(BlendOverlapEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == BlendOverlapEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        BlendOverlapEXT::UNCORRELATED => f.write_str("UNCORRELATED")?,
                        BlendOverlapEXT::DISJOINT => f.write_str("DISJOINT")?,
                        BlendOverlapEXT::CONJOINT => f.write_str("CONJOINT")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(BlendOverlapEXT)).field(&Flags(*self)).finish()
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
/// # Members
/// This structure describes the following feature:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`advanced_blend_coherent_operations`] specifies whether blending using [advanced blend operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced)
///   is guaranteed to execute atomically and in [primitive order](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#drawing-primitive-order).
///   If this is [`TRUE`], `VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT` is treated the same
///   as `VK_ACCESS_COLOR_ATTACHMENT_READ_BIT`, and advanced blending needs no additional
///   synchronization over basic blending. If this is [`FALSE`], then memory dependencies are
///   required to guarantee order between two advanced blending operations that occur on the same
///   sample.
/// If the [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`] structure is included in the
/// [`p_next`] chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceBlendOperationAdvancedFeaturesEXT`] **can**  also be used in the [`p_next`]
/// chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT`
/// # Related
/// - [`ext_blend_operation_advanced`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
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
    pub advanced_blend_coherent_operations: Bool32,
}
impl<'lt> Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            advanced_blend_coherent_operations: 0,
        }
    }
}
impl<'lt> PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
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
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_coherent_operations`]
    pub fn with_advanced_blend_coherent_operations_raw(mut self, value: Bool32) -> Self {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::advanced_blend_coherent_operations`]
    pub fn set_advanced_blend_coherent_operations(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_coherent_operations = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::advanced_blend_coherent_operations`]
    pub fn with_advanced_blend_coherent_operations(mut self, value: bool) -> Self {
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`advanced_blend_max_color_attachments`] is one greater than the highest color attachment index that  **can**  be used in a subpass, for a pipeline that uses an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced).
/// - [`advanced_blend_independent_blend`] specifies whether advanced blend operations  **can**
///   vary per-attachment.
/// - [`advanced_blend_non_premultiplied_src_color`] specifies whether the source color  **can**  be
///   treated as non-premultiplied. If this is [`FALSE`], then
///   [`PipelineColorBlendAdvancedStateCreateInfoEXT::src_premultiplied`] **must**  be [`TRUE`].
/// - [`advanced_blend_non_premultiplied_dst_color`] specifies whether the destination color
///   **can**  be treated as non-premultiplied. If this is [`FALSE`], then
///   [`PipelineColorBlendAdvancedStateCreateInfoEXT::dst_premultiplied`] **must**  be [`TRUE`].
/// - [`advanced_blend_correlated_overlap`] specifies whether the overlap mode  **can**  be treated
///   as correlated. If this is [`FALSE`], then
///   [`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`] **must**  be
///   `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
/// - [`advanced_blend_all_operations`] specifies whether all advanced blend operation enums are
///   supported. See the valid usage of [`PipelineColorBlendAttachmentState`].
/// # Description
/// If the [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT`] structure is included in the
/// [`p_next`] chain of the
/// [`PhysicalDeviceProperties2`] structure passed to
/// [`get_physical_device_properties2`], it is filled in with each
/// corresponding implementation-dependent property.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT`
/// # Related
/// - [`ext_blend_operation_advanced`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`advanced_blend_max_color_attachments`] is one greater than the highest
    ///color attachment index that  **can**  be used in a subpass, for a pipeline
    ///that uses an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced).
    pub advanced_blend_max_color_attachments: u32,
    ///[`advanced_blend_independent_blend`] specifies whether advanced blend
    ///operations  **can**  vary per-attachment.
    pub advanced_blend_independent_blend: Bool32,
    ///[`advanced_blend_non_premultiplied_src_color`] specifies whether the source
    ///color  **can**  be treated as non-premultiplied.
    ///If this is [`FALSE`], then
    ///[`PipelineColorBlendAdvancedStateCreateInfoEXT`]::`srcPremultiplied` **must**  be [`TRUE`].
    pub advanced_blend_non_premultiplied_src_color: Bool32,
    ///[`advanced_blend_non_premultiplied_dst_color`] specifies whether the
    ///destination color  **can**  be treated as non-premultiplied.
    ///If this is [`FALSE`], then
    ///[`PipelineColorBlendAdvancedStateCreateInfoEXT`]::`dstPremultiplied` **must**  be [`TRUE`].
    pub advanced_blend_non_premultiplied_dst_color: Bool32,
    ///[`advanced_blend_correlated_overlap`] specifies whether the overlap mode
    /// **can**  be treated as correlated.
    ///If this is [`FALSE`], then
    ///[`PipelineColorBlendAdvancedStateCreateInfoEXT`]::`blendOverlap` **must**  be `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
    pub advanced_blend_correlated_overlap: Bool32,
    ///[`advanced_blend_all_operations`]
    ///specifies whether all advanced blend operation enums are supported.
    ///See the valid usage of [`PipelineColorBlendAttachmentState`].
    pub advanced_blend_all_operations: Bool32,
}
impl<'lt> Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
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
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
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
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_independent_blend`]
    pub fn with_advanced_blend_independent_blend_raw(mut self, value: Bool32) -> Self {
        self.advanced_blend_independent_blend = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_non_premultiplied_src_color`]
    pub fn with_advanced_blend_non_premultiplied_src_color_raw(mut self, value: Bool32) -> Self {
        self.advanced_blend_non_premultiplied_src_color = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_non_premultiplied_dst_color`]
    pub fn with_advanced_blend_non_premultiplied_dst_color_raw(mut self, value: Bool32) -> Self {
        self.advanced_blend_non_premultiplied_dst_color = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_correlated_overlap`]
    pub fn with_advanced_blend_correlated_overlap_raw(mut self, value: Bool32) -> Self {
        self.advanced_blend_correlated_overlap = value;
        self
    }
    ///Sets the raw value of [`Self::advanced_blend_all_operations`]
    pub fn with_advanced_blend_all_operations_raw(mut self, value: Bool32) -> Self {
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
        &mut self.advanced_blend_max_color_attachments
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::advanced_blend_max_color_attachments`]
    pub fn set_advanced_blend_max_color_attachments(&mut self, value: u32) -> &mut Self {
        self.advanced_blend_max_color_attachments = value;
        self
    }
    ///Sets the value of [`Self::advanced_blend_independent_blend`]
    pub fn set_advanced_blend_independent_blend(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_independent_blend = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::advanced_blend_non_premultiplied_src_color`]
    pub fn set_advanced_blend_non_premultiplied_src_color(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_non_premultiplied_src_color = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::advanced_blend_non_premultiplied_dst_color`]
    pub fn set_advanced_blend_non_premultiplied_dst_color(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_non_premultiplied_dst_color = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::advanced_blend_correlated_overlap`]
    pub fn set_advanced_blend_correlated_overlap(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_correlated_overlap = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::advanced_blend_all_operations`]
    pub fn set_advanced_blend_all_operations(&mut self, value: bool) -> &mut Self {
        self.advanced_blend_all_operations = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::advanced_blend_max_color_attachments`]
    pub fn with_advanced_blend_max_color_attachments(mut self, value: u32) -> Self {
        self.advanced_blend_max_color_attachments = value;
        self
    }
    ///Sets the value of [`Self::advanced_blend_independent_blend`]
    pub fn with_advanced_blend_independent_blend(mut self, value: bool) -> Self {
        self.advanced_blend_independent_blend = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::advanced_blend_non_premultiplied_src_color`]
    pub fn with_advanced_blend_non_premultiplied_src_color(mut self, value: bool) -> Self {
        self.advanced_blend_non_premultiplied_src_color = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::advanced_blend_non_premultiplied_dst_color`]
    pub fn with_advanced_blend_non_premultiplied_dst_color(mut self, value: bool) -> Self {
        self.advanced_blend_non_premultiplied_dst_color = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::advanced_blend_correlated_overlap`]
    pub fn with_advanced_blend_correlated_overlap(mut self, value: bool) -> Self {
        self.advanced_blend_correlated_overlap = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::advanced_blend_all_operations`]
    pub fn with_advanced_blend_all_operations(mut self, value: bool) -> Self {
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
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_premultiplied`] specifies whether the source color of the blend operation is treated as
///   premultiplied.
/// - [`dst_premultiplied`] specifies whether the destination color of the blend operation is
///   treated as premultiplied.
/// - [`blend_overlap`] is a [`BlendOverlapEXT`] value specifying how the source and destination
///   sample’s coverage is correlated.
/// # Description
/// If this structure is not present, [`src_premultiplied`] and
/// [`dst_premultiplied`] are both considered to be [`TRUE`], and
/// [`blend_overlap`] is considered to be
/// `VK_BLEND_OVERLAP_UNCORRELATED_EXT`.
/// ## Valid Usage
/// - If the [non-premultiplied source color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendNonPremultipliedSrcColor)
///   property is not supported, [`src_premultiplied`] **must**  be [`TRUE`]
/// - If the [non-premultiplied destination color](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendNonPremultipliedDstColor)
///   property is not supported, [`dst_premultiplied`] **must**  be [`TRUE`]
/// - If the [correlated overlap](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-advancedBlendCorrelatedOverlap)
///   property is not supported, [`blend_overlap`] **must**  be `VK_BLEND_OVERLAP_UNCORRELATED_EXT`
///
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT`
/// - [`blend_overlap`] **must**  be a valid [`BlendOverlapEXT`] value
/// # Related
/// - [`ext_blend_operation_advanced`]
/// - [`BlendOverlapEXT`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineColorBlendAdvancedStateCreateInfoEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`src_premultiplied`] specifies whether the source color of the blend
    ///operation is treated as premultiplied.
    pub src_premultiplied: Bool32,
    ///[`dst_premultiplied`] specifies whether the destination color of the
    ///blend operation is treated as premultiplied.
    pub dst_premultiplied: Bool32,
    ///[`blend_overlap`] is a [`BlendOverlapEXT`] value specifying how the
    ///source and destination sample’s coverage is correlated.
    pub blend_overlap: BlendOverlapEXT,
}
impl<'lt> Default for PipelineColorBlendAdvancedStateCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
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
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::src_premultiplied`]
    pub fn with_src_premultiplied_raw(mut self, value: Bool32) -> Self {
        self.src_premultiplied = value;
        self
    }
    ///Sets the raw value of [`Self::dst_premultiplied`]
    pub fn with_dst_premultiplied_raw(mut self, value: Bool32) -> Self {
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
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::src_premultiplied`]
    pub fn set_src_premultiplied(&mut self, value: bool) -> &mut Self {
        self.src_premultiplied = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::dst_premultiplied`]
    pub fn set_dst_premultiplied(&mut self, value: bool) -> &mut Self {
        self.dst_premultiplied = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::blend_overlap`]
    pub fn set_blend_overlap(
        &mut self,
        value: crate::extensions::ext_blend_operation_advanced::BlendOverlapEXT,
    ) -> &mut Self {
        self.blend_overlap = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::src_premultiplied`]
    pub fn with_src_premultiplied(mut self, value: bool) -> Self {
        self.src_premultiplied = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::dst_premultiplied`]
    pub fn with_dst_premultiplied(mut self, value: bool) -> Self {
        self.dst_premultiplied = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::blend_overlap`]
    pub fn with_blend_overlap(
        mut self,
        value: crate::extensions::ext_blend_operation_advanced::BlendOverlapEXT,
    ) -> Self {
        self.blend_overlap = value;
        self
    }
}
