//![VK_KHR_display_swapchain](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_display_swapchain.html) - device extension
//!# Description
//!This extension provides an API to create a swapchain directly on a device’s
//!display without any underlying window system.
//!# Revision
//!10
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_swapchain`]`
//! - Requires `[`VK_KHR_display`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_display_swapchain]
//!   @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_display_swapchain
//!   extension>>)
//!# New functions & commands
//! - [`CreateSharedSwapchainsKHR`]
//!# New structures
//! - Extending [`PresentInfoKHR`]:  - [`DisplayPresentInfoKHR`]
//!# New constants
//! - [`KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME`]
//! - [`KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION`]
//! - Extending [`VulkanResultCodes`]:  - `VK_ERROR_INCOMPATIBLE_DISPLAY_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Should swapchains sharing images each hold a reference to the images, or
//!should it be up to the application to destroy the swapchains and images in
//!an order that avoids the need for reference counting? **RESOLVED** : Take a reference.
//!The lifetime of presentable images is already complex enough.2) Should the `srcRect` and
//! `dstRect` parameters be specified as
//!part of the presentation command, or at swapchain creation time? **RESOLVED** : As part of the
//! presentation command.
//!This allows moving and scaling the image on the screen without the need to
//!respecify the mode or create a new swapchain and presentable images.3) Should `srcRect` and
//! `dstRect` be specified as rects, or separate
//!offset/extent values? **RESOLVED** : As rects.
//!Specifying them separately might make it easier for hardware to expose
//!support for one but not the other, but in such cases applications must just
//!take care to obey the reported capabilities and not use non-zero offsets or
//!extents that require scaling, as appropriate.4) How can applications create multiple swapchains
//! that use the same images? **RESOLVED** : By calling [`CreateSharedSwapchainsKHR`].An earlier
//! resolution used [`CreateSwapchainKHR`], chaining multiple
//![`SwapchainCreateInfoKHR`] structures through `pNext`.
//!In order to allow each swapchain to also allow other extension structs, a
//!level of indirection was used: [`SwapchainCreateInfoKHR::p_next`]
//!pointed to a different structure, which had both `sType` and `pNext`
//!members for additional extensions, and also had a pointer to the next
//![`SwapchainCreateInfoKHR`] structure.
//!The number of swapchains to be created could only be found by walking this
//!linked list of alternating structures, and the `pSwapchains` out
//!parameter was reinterpreted to be an array of [`SwapchainKHR`] handles.Another option considered
//! was a method to specify a “shared” swapchain
//!when creating a new swapchain, such that groups of swapchains using the same
//!images could be built up one at a time.
//!This was deemed unusable because drivers need to know all of the displays an
//!image will be used on when determining which internal formats and layouts to
//!use for that image.
//!# Version History
//! - Revision 1, 2015-07-29 (James Jones)  - Initial draft
//! - Revision 2, 2015-08-21 (Ian Elliott)  - Renamed this extension and all of its enumerations,
//!   types, functions, etc. This makes it compliant with the proposed standard for Vulkan
//!   extensions.  - Switched from “revision” to “version”, including use of the VK_MAKE_VERSION
//!   macro in the header file.
//! - Revision 3, 2015-09-01 (James Jones)  - Restore single-field revision number.
//! - Revision 4, 2015-09-08 (James Jones)  - Allow creating multiple swap chains that share the
//!   same images using a single call to vkCreateSwapchainKHR().
//! - Revision 5, 2015-09-10 (Alon Or-bach)  - Removed underscores from SWAP_CHAIN in two enums.
//! - Revision 6, 2015-10-02 (James Jones)  - Added support for smart panels/buffered displays.
//! - Revision 7, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_display_swapchain to
//!   VK_KHR_display_swapchain.
//! - Revision 8, 2015-11-03 (Daniel Rakos)  - Updated sample code based on the changes to
//!   VK_KHR_swapchain.
//! - Revision 9, 2015-11-10 (Jesse Hall)  - Replaced VkDisplaySwapchainCreateInfoKHR with
//!   vkCreateSharedSwapchainsKHR, changing resolution of issue #4.
//! - Revision 10, 2017-03-13 (James Jones)  - Closed all remaining issues. The specification and
//!   implementations have been shipping with the proposed resolutions for some time now.  - Removed
//!   the sample code and noted it has been integrated into the official Vulkan SDK cube demo.
//!# Other info
//! * 2017-03-13
//! * No known IP claims.
//! * - James Jones, NVIDIA  - Jeff Vigil, Qualcomm  - Jesse Hall, Google
//!# Related
//! - [`DisplayPresentInfoKHR`]
//! - [`CreateSharedSwapchainsKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, Bool32, Rect2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_display_swapchain");
///[VkDisplayPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPresentInfoKHR.html) - Structure describing parameters of a queue presentation to a swapchain
///# C Specifications
///The [`DisplayPresentInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display_swapchain
///typedef struct VkDisplayPresentInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkRect2D           srcRect;
///    VkRect2D           dstRect;
///    VkBool32           persistent;
///} VkDisplayPresentInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_rect`] is a rectangular region of pixels to present. It  **must**  be a subset of the
///   image being presented. If [`DisplayPresentInfoKHR`] is not specified, this region will be
///   assumed to be the entire presentable image.
/// - [`dst_rect`] is a rectangular region within the visible region of the swapchain’s display
///   mode. If [`DisplayPresentInfoKHR`] is not specified, this region will be assumed to be the
///   entire visible region of the swapchain’s mode. If the specified rectangle is a subset of the
///   display mode’s visible region, content from display planes below the swapchain’s plane will be
///   visible outside the rectangle. If there are no planes below the swapchain’s, the area outside
///   the specified rectangle will be black. If portions of the specified rectangle are outside of
///   the display’s visible region, pixels mapping only to those portions of the rectangle will be
///   discarded.
/// - [`persistent`]: If this is [`TRUE`], the display engine will enable buffered mode on displays
///   that support it. This allows the display engine to stop sending content to the display until a
///   new image is presented. The display will instead maintain a copy of the last presented image.
///   This allows less power to be used, but  **may**  increase presentation latency. If
///   [`DisplayPresentInfoKHR`] is not specified, persistent mode will not be used.
///# Description
///If the extent of the [`src_rect`] and [`dst_rect`] are not equal, the
///presented pixels will be scaled accordingly.
///## Valid Usage
/// - [`src_rect`] **must**  specify a rectangular region that is a subset of the image being
///   presented
/// - [`dst_rect`] **must**  specify a rectangular region that is a subset of the `visibleRegion`
///   parameter of the display mode the swapchain being presented uses
/// - If the `persistentContent` member of the [`DisplayPropertiesKHR`] structure returned by
///   [`GetPhysicalDeviceDisplayPropertiesKHR`] for the display the present operation targets is
///   [`FALSE`], then [`persistent`] **must**  be [`FALSE`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR`
///# Related
/// - [`VK_KHR_display_swapchain`]
/// - [`Bool32`]
/// - [`Rect2D`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDisplayPresentInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DisplayPresentInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`src_rect`] is a rectangular region of pixels to present.
    ///It  **must**  be a subset of the image being presented.
    ///If [`DisplayPresentInfoKHR`] is not specified, this region will be
    ///assumed to be the entire presentable image.
    pub src_rect: Rect2D,
    ///[`dst_rect`] is a rectangular region within the visible region of the
    ///swapchain’s display mode.
    ///If [`DisplayPresentInfoKHR`] is not specified, this region will be
    ///assumed to be the entire visible region of the swapchain’s mode.
    ///If the specified rectangle is a subset of the display mode’s visible
    ///region, content from display planes below the swapchain’s plane will be
    ///visible outside the rectangle.
    ///If there are no planes below the swapchain’s, the area outside the
    ///specified rectangle will be black.
    ///If portions of the specified rectangle are outside of the display’s
    ///visible region, pixels mapping only to those portions of the rectangle
    ///will be discarded.
    pub dst_rect: Rect2D,
    ///[`persistent`]: If this is [`TRUE`], the display engine will
    ///enable buffered mode on displays that support it.
    ///This allows the display engine to stop sending content to the display
    ///until a new image is presented.
    ///The display will instead maintain a copy of the last presented image.
    ///This allows less power to be used, but  **may**  increase presentation
    ///latency.
    ///If [`DisplayPresentInfoKHR`] is not specified, persistent mode will
    ///not be used.
    pub persistent: Bool32,
}
impl<'lt> Default for DisplayPresentInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src_rect: Default::default(),
            dst_rect: Default::default(),
            persistent: 0,
        }
    }
}
impl<'lt> DisplayPresentInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::persistent`]
    pub fn persistent_raw(&self) -> Bool32 {
        self.persistent
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::persistent`]
    pub fn set_persistent_raw(&mut self, value: Bool32) -> &mut Self {
        self.persistent = value;
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
    ///Gets the value of [`Self::src_rect`]
    pub fn src_rect(&self) -> Rect2D {
        self.src_rect
    }
    ///Gets the value of [`Self::dst_rect`]
    pub fn dst_rect(&self) -> Rect2D {
        self.dst_rect
    }
    ///Gets the value of [`Self::persistent`]
    pub fn persistent(&self) -> bool {
        unsafe { std::mem::transmute(self.persistent as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src_rect`]
    pub fn src_rect_mut(&mut self) -> &mut Rect2D {
        &mut self.src_rect
    }
    ///Gets a mutable reference to the value of [`Self::dst_rect`]
    pub fn dst_rect_mut(&mut self) -> &mut Rect2D {
        &mut self.dst_rect
    }
    ///Gets a mutable reference to the value of [`Self::persistent`]
    pub fn persistent_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.persistent as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.persistent as *mut Bool32)
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
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::src_rect`]
    pub fn set_src_rect(&mut self, value: crate::vulkan1_0::Rect2D) -> &mut Self {
        self.src_rect = value;
        self
    }
    ///Sets the raw value of [`Self::dst_rect`]
    pub fn set_dst_rect(&mut self, value: crate::vulkan1_0::Rect2D) -> &mut Self {
        self.dst_rect = value;
        self
    }
    ///Sets the raw value of [`Self::persistent`]
    pub fn set_persistent(&mut self, value: bool) -> &mut Self {
        self.persistent = value as u8 as u32;
        self
    }
}
