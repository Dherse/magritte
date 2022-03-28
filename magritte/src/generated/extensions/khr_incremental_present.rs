//![VK_KHR_incremental_present](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_incremental_present.html) - device extension
//!# Description
//!This device extension extends [`QueuePresentKHR`], from the
//!`[`VK_KHR_swapchain`]` extension, allowing an application to specify a
//!list of rectangular, modified regions of each image to present.
//!This should be used in situations where an application is only changing a
//!small portion of the presentable images within a swapchain, since it enables
//!the presentation engine to avoid wasting time presenting parts of the
//!surface that have not changed.This extension is leveraged from the
//! `EGL_KHR_swap_buffers_with_damage`
//!extension.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_swapchain`]`
//!# Contacts
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_incremental_present]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the
//!   VK_KHR_incremental_present extension>>)
//!# New structures
//! - [`PresentRegionKHR`]
//! - [`RectLayerKHR`]
//! - Extending [`PresentInfoKHR`]:  - [`PresentRegionsKHR`]
//!# New constants
//! - [`KHR_INCREMENTAL_PRESENT_EXTENSION_NAME`]
//! - [`KHR_INCREMENTAL_PRESENT_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR`
//!# Known issues & F.A.Q
//!1) How should we handle steroescopic-3D swapchains? We need to add a layer
//!for each rectangle.
//!One approach is to create another struct containing the [`Rect2D`] plus
//!layer, and have [`PresentRegionsKHR`] point to an array of that struct.
//!Another approach is to have two parallel arrays, `pRectangles` and
//!`pLayers`, where `pRectangles`[i] and `pLayers`[i] must be used
//!together.
//!Which approach should we use, and if the array of a new structure, what
//!should that be called? **RESOLVED** : Create a new structure, which is a [`Rect2D`] plus a
//! layer,
//!and will be called [`RectLayerKHR`].2) Where is the origin of the [`RectLayerKHR`]? **RESOLVED**
//! : The upper left corner of the presentable image(s) of the
//!swapchain, per the definition of framebuffer coordinates.3) Does the rectangular region,
//! [`RectLayerKHR`], specify pixels of the
//!swapchain’s image(s), or of the surface? **RESOLVED** : Of the image(s).
//!Some presentation engines may scale the pixels of a swapchain’s image(s) to
//!the size of the surface.
//!The size of the swapchain’s image(s) will be consistent, where the size of
//!the surface may vary over time.4) What if all of the rectangles for a given swapchain contain a
//! width
//!and/or height of zero? **RESOLVED** : The application is indicating that no pixels changed since
//! the
//!last present.
//!The presentation engine may use such a hint and not update any pixels for
//!the swapchain.
//!However, all other semantics of [`QueuePresentKHR`] must still be
//!honored, including waiting for semaphores to signal.5) When the swapchain is created with
//![`SwapchainCreateInfoKHR::pre_transform`] set to a value other than
//!`VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR`, should the rectangular region,
//![`RectLayerKHR`], be transformed to align with the `preTransform`? **RESOLVED** : No.
//!The rectangular region in [`RectLayerKHR`] should not be tranformed.
//!As such, it may not align with the extents of the swapchain’s image(s).
//!It is the responsibility of the presentation engine to transform the
//!rectangular region.
//!This matches the behavior of the Android presentation engine, which set the
//!precedent.
//!# Version History
//! - Revision 1, 2016-11-02 (Ian Elliott)  - Internal revisions
//! - Revision 2, 2021-03-18 (Ian Elliott)  - Clarified alignment of rectangles for presentation
//!   engines that support transformed swapchains.
//!# Other info
//! * 2016-11-02
//! * No known IP claims.
//! * - Ian Elliott, Google  - Jesse Hall, Google  - Alon Or-bach, Samsung  - James Jones, NVIDIA  -
//!   Daniel Rakos, AMD  - Ray Smith, ARM  - Mika Isojarvi, Google  - Jeff Juliano, NVIDIA  - Jeff
//!   Bolz, NVIDIA
//!# Related
//! - [`PresentRegionKHR`]
//! - [`PresentRegionsKHR`]
//! - [`RectLayerKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, Extent2D, Offset2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION")]
pub const KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME")]
pub const KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_incremental_present");
///[VkPresentRegionsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentRegionsKHR.html) - Structure hint of rectangular regions changed by vkQueuePresentKHR
///# C Specifications
///When the [`VK_KHR_incremental_present`] extension is enabled, additional
///fields  **can**  be specified that allow an application to specify that only
///certain rectangular regions of the presentable images of a swapchain are
///changed.
///This is an optimization hint that a presentation engine  **may**  use to only
///update the region of a surface that is actually changing.
///The application still  **must**  ensure that all pixels of a presented image
///contain the desired values, in case the presentation engine ignores this
///hint.
///An application  **can**  provide this hint by adding a [`PresentRegionsKHR`]
///structure to the [`p_next`] chain of the [`PresentInfoKHR`] structure.The [`PresentRegionsKHR`]
/// structure is defined as:
///```c
///// Provided by VK_KHR_incremental_present
///typedef struct VkPresentRegionsKHR {
///    VkStructureType              sType;
///    const void*                  pNext;
///    uint32_t                     swapchainCount;
///    const VkPresentRegionKHR*    pRegions;
///} VkPresentRegionsKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain_count`] is the number of swapchains being presented to by this command.
/// - [`regions`] is `NULL` or a pointer to an array of [`PresentRegionKHR`] elements with
///   [`swapchain_count`] entries. If not `NULL`, each element of [`regions`] contains the region
///   that has changed since the last present to the swapchain in the corresponding entry in the
///   [`PresentInfoKHR::swapchains`] array.
///# Description
///## Valid Usage
/// - [`swapchain_count`] **must**  be the same value as [`PresentInfoKHR`]::[`swapchain_count`],
///   where [`PresentInfoKHR`] is included in the [`p_next`] chain of this [`PresentRegionsKHR`]
///   structure
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR`
/// - If [`regions`] is not `NULL`, [`regions`] **must**  be a valid pointer to an array of
///   [`swapchain_count`] valid [`PresentRegionKHR`] structures
/// - [`swapchain_count`] **must**  be greater than `0`
///# Related
/// - [`VK_KHR_incremental_present`]
/// - [`PresentRegionKHR`]
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
pub struct PresentRegionsKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`swapchain_count`] is the number of swapchains being presented to by
    ///this command.
    swapchain_count: u32,
    ///[`regions`] is `NULL` or a pointer to an array of
    ///[`PresentRegionKHR`] elements with [`swapchain_count`] entries.
    ///If not `NULL`, each element of [`regions`] contains the region that
    ///has changed since the last present to the swapchain in the corresponding
    ///entry in the [`PresentInfoKHR`]::`pSwapchains` array.
    regions: *const PresentRegionKHR<'lt>,
}
impl<'lt> Default for PresentRegionsKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            swapchain_count: 0,
            regions: std::ptr::null(),
        }
    }
}
impl<'lt> PresentRegionsKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::regions`]
    pub fn regions_raw(&self) -> *const PresentRegionKHR<'lt> {
        self.regions
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions_raw(&mut self, value: *const PresentRegionKHR<'lt>) -> &mut Self {
        self.regions = value;
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
    ///Gets the value of [`Self::swapchain_count`]
    pub fn swapchain_count(&self) -> u32 {
        self.swapchain_count
    }
    ///Gets the value of [`Self::regions`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn regions(&self) -> &[PresentRegionKHR<'lt>] {
        std::slice::from_raw_parts(self.regions, self.swapchain_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain_count`]
    pub fn swapchain_count_mut(&mut self) -> &mut u32 {
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
    ///Sets the raw value of [`Self::swapchain_count`]
    pub fn set_swapchain_count(&mut self, value: u32) -> &mut Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the raw value of [`Self::regions`]
    pub fn set_regions(
        &mut self,
        value: &'lt [crate::extensions::khr_incremental_present::PresentRegionKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.regions = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
}
///[VkPresentRegionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentRegionKHR.html) - Structure containing rectangular region changed by vkQueuePresentKHR for a given VkImage
///# C Specifications
///For a given image and swapchain, the region to present is specified by the
///[`PresentRegionKHR`] structure, which is defined as:
///```c
///// Provided by VK_KHR_incremental_present
///typedef struct VkPresentRegionKHR {
///    uint32_t                 rectangleCount;
///    const VkRectLayerKHR*    pRectangles;
///} VkPresentRegionKHR;
///```
///# Members
/// - [`rectangle_count`] is the number of rectangles in [`rectangles`], or zero if the entire image
///   has changed and should be presented.
/// - [`rectangles`] is either `NULL` or a pointer to an array of [`RectLayerKHR`] structures. The
///   [`RectLayerKHR`] structure is the framebuffer coordinates, plus layer, of a portion of a
///   presentable image that has changed and  **must**  be presented. If non-`NULL`, each entry in
///   [`rectangles`] is a rectangle of the given image that has changed since the last image was
///   presented to the given swapchain. The rectangles  **must**  be specified relative to
///   [`SurfaceCapabilitiesKHR::current_transform`], regardless of the swapchain’s `preTransform`.
///   The presentation engine will apply the `preTransform` transformation to the rectangles, along
///   with any further transformation it applies to the image content.
///# Description
///## Valid Usage (Implicit)
/// - If [`rectangle_count`] is not `0`, and [`rectangles`] is not `NULL`, [`rectangles`] **must**
///   be a valid pointer to an array of [`rectangle_count`] valid [`RectLayerKHR`] structures
///# Related
/// - [`VK_KHR_incremental_present`]
/// - [`PresentRegionsKHR`]
/// - [`RectLayerKHR`]
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
pub struct PresentRegionKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`rectangle_count`] is the number of rectangles in [`rectangles`],
    ///or zero if the entire image has changed and should be presented.
    rectangle_count: u32,
    ///[`rectangles`] is either `NULL` or a pointer to an array of
    ///[`RectLayerKHR`] structures.
    ///The [`RectLayerKHR`] structure is the framebuffer coordinates, plus
    ///layer, of a portion of a presentable image that has changed and  **must**  be
    ///presented.
    ///If non-`NULL`, each entry in [`rectangles`] is a rectangle of the
    ///given image that has changed since the last image was presented to the
    ///given swapchain.
    ///The rectangles  **must**  be specified relative to
    ///[`SurfaceCapabilitiesKHR`]::`currentTransform`, regardless of
    ///the swapchain’s `preTransform`.
    ///The presentation engine will apply the `preTransform` transformation
    ///to the rectangles, along with any further transformation it applies to
    ///the image content.
    rectangles: *const RectLayerKHR,
}
impl<'lt> Default for PresentRegionKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            rectangle_count: 0,
            rectangles: std::ptr::null(),
        }
    }
}
impl<'lt> PresentRegionKHR<'lt> {
    ///Gets the raw value of [`Self::rectangles`]
    pub fn rectangles_raw(&self) -> *const RectLayerKHR {
        self.rectangles
    }
    ///Sets the raw value of [`Self::rectangles`]
    pub fn set_rectangles_raw(&mut self, value: *const RectLayerKHR) -> &mut Self {
        self.rectangles = value;
        self
    }
    ///Gets the value of [`Self::rectangle_count`]
    pub fn rectangle_count(&self) -> u32 {
        self.rectangle_count
    }
    ///Gets the value of [`Self::rectangles`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn rectangles(&self) -> &[RectLayerKHR] {
        std::slice::from_raw_parts(self.rectangles, self.rectangle_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::rectangle_count`]
    pub fn rectangle_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::rectangle_count`]
    pub fn set_rectangle_count(&mut self, value: u32) -> &mut Self {
        self.rectangle_count = value;
        self
    }
    ///Sets the raw value of [`Self::rectangles`]
    pub fn set_rectangles(
        &mut self,
        value: &'lt [crate::extensions::khr_incremental_present::RectLayerKHR],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.rectangles = value.as_ptr();
        self.rectangle_count = len_;
        self
    }
}
///[VkRectLayerKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRectLayerKHR.html) - Structure containing a rectangle, including layer, changed by vkQueuePresentKHR for a given VkImage
///# C Specifications
///The [`RectLayerKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_incremental_present
///typedef struct VkRectLayerKHR {
///    VkOffset2D    offset;
///    VkExtent2D    extent;
///    uint32_t      layer;
///} VkRectLayerKHR;
///```
///# Members
/// - [`offset`] is the origin of the rectangle, in pixels.
/// - [`extent`] is the size of the rectangle, in pixels.
/// - [`layer`] is the layer of the image. For images with only one layer, the value of [`layer`]
///   **must**  be 0.
///# Description
///Some platforms allow the size of a surface to change, and then scale the
///pixels of the image to fit the surface.
///[`RectLayerKHR`] specifies pixels of the swapchain’s image(s), which
///will be constant for the life of the swapchain.
///## Valid Usage
/// - The sum of [`offset`] and [`extent`], after being transformed according to the `preTransform`
///   member of the [`SwapchainCreateInfoKHR`] structure,  **must**  be no greater than the
///   `imageExtent` member of the [`SwapchainCreateInfoKHR`] structure passed to
///   [`CreateSwapchainKHR`]
/// - [`layer`] **must**  be less than the `imageArrayLayers` member of the
///   [`SwapchainCreateInfoKHR`] structure passed to [`CreateSwapchainKHR`]
///# Related
/// - [`VK_KHR_incremental_present`]
/// - [`Extent2D`]
/// - [`Offset2D`]
/// - [`PresentRegionKHR`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RectLayerKHR {
    ///[`offset`] is the origin of the rectangle, in pixels.
    offset: Offset2D,
    ///[`extent`] is the size of the rectangle, in pixels.
    extent: Extent2D,
    ///[`layer`] is the layer of the image.
    ///For images with only one layer, the value of [`layer`] **must**  be 0.
    layer: u32,
}
impl Default for RectLayerKHR {
    fn default() -> Self {
        Self {
            offset: Default::default(),
            extent: Default::default(),
            layer: 0,
        }
    }
}
impl RectLayerKHR {
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> Offset2D {
        self.offset
    }
    ///Gets the value of [`Self::extent`]
    pub fn extent(&self) -> Extent2D {
        self.extent
    }
    ///Gets the value of [`Self::layer`]
    pub fn layer(&self) -> u32 {
        self.layer
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut Offset2D {
        &mut self.offset
    }
    ///Gets a mutable reference to the value of [`Self::extent`]
    pub fn extent_mut(&mut self) -> &mut Extent2D {
        &mut self.extent
    }
    ///Gets a mutable reference to the value of [`Self::layer`]
    pub fn layer_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::offset`]
    pub fn set_offset(&mut self, value: crate::vulkan1_0::Offset2D) -> &mut Self {
        self.offset = value;
        self
    }
    ///Sets the raw value of [`Self::extent`]
    pub fn set_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.extent = value;
        self
    }
    ///Sets the raw value of [`Self::layer`]
    pub fn set_layer(&mut self, value: u32) -> &mut Self {
        self.layer = value;
        self
    }
}
