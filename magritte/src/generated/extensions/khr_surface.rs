use crate::{
    extensions::khr_display::SurfaceTransformFlagsKHR,
    vulkan1_0::{Extent2D, Format, ImageUsageFlags},
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SURFACE_SPEC_VERSION")]
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SURFACE_EXTENSION_NAME")]
pub const KHR_SURFACE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_surface");
///[VkPresentModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentModeKHR.html) - Presentation mode supported for a surface
///# C Specifications
///Possible values of elements of the
///[`GetPhysicalDeviceSurfacePresentModesKHR`]`::pPresentModes` array,
///indicating the supported presentation modes for a surface, are:
///```c
///// Provided by VK_KHR_surface
///typedef enum VkPresentModeKHR {
///    VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
///    VK_PRESENT_MODE_MAILBOX_KHR = 1,
///    VK_PRESENT_MODE_FIFO_KHR = 2,
///    VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,
///  // Provided by VK_KHR_shared_presentable_image
///    VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR = 1000111000,
///  // Provided by VK_KHR_shared_presentable_image
///    VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR = 1000111001,
///} VkPresentModeKHR;
///```
///# Description
/// - [`PresentModeImmediateKhr`] specifies that the presentation engine does not wait for a
///   vertical blanking period to update the current image, meaning this mode  **may**  result in
///   visible tearing. No internal queuing of presentation requests is needed, as the requests are
///   applied immediately.
/// - [`PresentModeMailboxKhr`] specifies that the presentation engine waits for the next vertical
///   blanking period to update the current image. Tearing  **cannot**  be observed. An internal
///   single-entry queue is used to hold pending presentation requests. If the queue is full when a
///   new presentation request is received, the new request replaces the existing entry, and any
///   images associated with the prior entry become available for re-use by the application. One
///   request is removed from the queue and processed during each vertical blanking period in which
///   the queue is non-empty.
/// - [`PresentModeFifoKhr`] specifies that the presentation engine waits for the next vertical
///   blanking period to update the current image. Tearing  **cannot**  be observed. An internal
///   queue is used to hold pending presentation requests. New requests are appended to the end of
///   the queue, and one request is removed from the beginning of the queue and processed during
///   each vertical blanking period in which the queue is non-empty. This is the only value of
///   `presentMode` that is  **required**  to be supported.
/// - [`PresentModeFifoRelaxedKhr`] specifies that the presentation engine generally waits for the
///   next vertical blanking period to update the current image. If a vertical blanking period has
///   already passed since the last update of the current image then the presentation engine does
///   not wait for another vertical blanking period for the update, meaning this mode  **may**
///   result in visible tearing in this case. This mode is useful for reducing visual stutter with
///   an application that will mostly present a new image before the next vertical blanking period,
///   but may occasionally be late, and present a new image just after the next vertical blanking
///   period. An internal queue is used to hold pending presentation requests. New requests are
///   appended to the end of the queue, and one request is removed from the beginning of the queue
///   and processed during or after each vertical blanking period in which the queue is non-empty.
/// - [`PresentModeSharedDemandRefreshKhr`] specifies that the presentation engine and application
///   have concurrent access to a single image, which is referred to as a *shared presentable
///   image*. The presentation engine is only required to update the current image after a new
///   presentation request is received. Therefore the application  **must**  make a presentation
///   request whenever an update is required. However, the presentation engine  **may**  update the
///   current image at any point, meaning this mode  **may**  result in visible tearing.
/// - [`PresentModeSharedContinuousRefreshKhr`] specifies that the presentation engine and
///   application have concurrent access to a single image, which is referred to as a *shared
///   presentable image*. The presentation engine periodically updates the current image on its
///   regular refresh cycle. The application is only required to make one initial presentation
///   request, after which the presentation engine  **must**  update the current image without any
///   need for further presentation requests. The application  **can**  indicate the image contents
///   have been updated by making a presentation request, but this does not guarantee the timing of
///   when it will be updated. This mode  **may**  result in visible tearing if rendering to the
///   image is not timed correctly.
///The supported [`ImageUsageFlagBits`] of the presentable images of a
///swapchain created for a surface  **may**  differ depending on the presentation
///mode, and can be determined as per the table below:
///# Related
/// - [`VK_KHR_surface`]
/// - [`SwapchainCreateInfoKHR`]
/// - [`GetPhysicalDeviceSurfacePresentModes2EXT`]
/// - [`GetPhysicalDeviceSurfacePresentModesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPresentModeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum PresentModeKHR {
    ///[`PresentModeImmediateKhr`] specifies that the presentation
    ///engine does not wait for a vertical blanking period to update the
    ///current image, meaning this mode  **may**  result in visible tearing.
    ///No internal queuing of presentation requests is needed, as the requests
    ///are applied immediately.
    PresentModeImmediateKhr = 0,
    ///[`PresentModeMailboxKhr`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing  **cannot**  be observed.
    ///An internal single-entry queue is used to hold pending presentation
    ///requests.
    ///If the queue is full when a new presentation request is received, the
    ///new request replaces the existing entry, and any images associated with
    ///the prior entry become available for re-use by the application.
    ///One request is removed from the queue and processed during each vertical
    ///blanking period in which the queue is non-empty.
    PresentModeMailboxKhr = 1,
    ///[`PresentModeFifoKhr`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing  **cannot**  be observed.
    ///An internal queue is used to hold pending presentation requests.
    ///New requests are appended to the end of the queue, and one request is
    ///removed from the beginning of the queue and processed during each
    ///vertical blanking period in which the queue is non-empty.
    ///This is the only value of `presentMode` that is  **required**  to be
    ///supported.
    PresentModeFifoKhr = 2,
    ///[`PresentModeFifoRelaxedKhr`] specifies that the presentation
    ///engine generally waits for the next vertical blanking period to update
    ///the current image.
    ///If a vertical blanking period has already passed since the last update
    ///of the current image then the presentation engine does not wait for
    ///another vertical blanking period for the update, meaning this mode  **may**
    ///result in visible tearing in this case.
    ///This mode is useful for reducing visual stutter with an application that
    ///will mostly present a new image before the next vertical blanking
    ///period, but may occasionally be late, and present a new image just after
    ///the next vertical blanking period.
    ///An internal queue is used to hold pending presentation requests.
    ///New requests are appended to the end of the queue, and one request is
    ///removed from the beginning of the queue and processed during or after
    ///each vertical blanking period in which the queue is non-empty.
    PresentModeFifoRelaxedKhr = 3,
    ///[`PresentModeSharedDemandRefreshKhr`] specifies that the
    ///presentation engine and application have concurrent access to a single
    ///image, which is referred to as a *shared presentable image*.
    ///The presentation engine is only required to update the current image
    ///after a new presentation request is received.
    ///Therefore the application  **must**  make a presentation request whenever an
    ///update is required.
    ///However, the presentation engine  **may**  update the current image at any
    ///point, meaning this mode  **may**  result in visible tearing.
    ///
    ///Provided by [`crate::extensions::khr_shared_presentable_image`]
    PresentModeSharedDemandRefreshKhr = 1000111000,
    ///[`PresentModeSharedContinuousRefreshKhr`] specifies that the
    ///presentation engine and application have concurrent access to a single
    ///image, which is referred to as a *shared presentable image*.
    ///The presentation engine periodically updates the current image on its
    ///regular refresh cycle.
    ///The application is only required to make one initial presentation
    ///request, after which the presentation engine  **must**  update the current
    ///image without any need for further presentation requests.
    ///The application  **can**  indicate the image contents have been updated by
    ///making a presentation request, but this does not guarantee the timing of
    ///when it will be updated.
    ///This mode  **may**  result in visible tearing if rendering to the image is
    ///not timed correctly.
    ///
    ///Provided by [`crate::extensions::khr_shared_presentable_image`]
    PresentModeSharedContinuousRefreshKhr = 1000111001,
}
impl const Default for PresentModeKHR {
    fn default() -> Self {
        PresentModeImmediateKhr
    }
}
impl PresentModeKHR {
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
///[VkColorSpaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorSpaceKHR.html) - Supported color space of the presentation engine
///# C Specifications
///Possible values of [`SurfaceFormatKHR::color_space`], specifying
///supported color spaces of a presentation engine, are:
///```c
///// Provided by VK_KHR_surface
///typedef enum VkColorSpaceKHR {
///    VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT = 1000104001,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT = 1000104002,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT = 1000104003,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT = 1000104004,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_BT709_LINEAR_EXT = 1000104005,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_BT709_NONLINEAR_EXT = 1000104006,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_BT2020_LINEAR_EXT = 1000104007,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_HDR10_ST2084_EXT = 1000104008,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DOLBYVISION_EXT = 1000104009,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_HDR10_HLG_EXT = 1000104010,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT = 1000104011,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT = 1000104012,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_PASS_THROUGH_EXT = 1000104013,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT = 1000104014,
///  // Provided by VK_AMD_display_native_hdr
///    VK_COLOR_SPACE_DISPLAY_NATIVE_AMD = 1000213000,
///    VK_COLORSPACE_SRGB_NONLINEAR_KHR = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
///  // Provided by VK_EXT_swapchain_colorspace
///    VK_COLOR_SPACE_DCI_P3_LINEAR_EXT = VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT,
///} VkColorSpaceKHR;
///```
///# Description
/// - [`ColorSpaceSrgbNonlinearKhr`] specifies support for the sRGB color space.
/// - [`ColorSpaceDisplayP3NonlinearExt`] specifies support for the Display-P3 color space to be
///   displayed using an sRGB-like EOTF (defined below).
/// - [`ColorSpaceExtendedSrgbLinearExt`] specifies support for the extended sRGB color space to be
///   displayed using a linear EOTF.
/// - [`ColorSpaceExtendedSrgbNonlinearExt`] specifies support for the extended sRGB color space to
///   be displayed using an sRGB EOTF.
/// - [`ColorSpaceDisplayP3LinearExt`] specifies support for the Display-P3 color space to be
///   displayed using a linear EOTF.
/// - [`ColorSpaceDciP3NonlinearExt`] specifies support for the DCI-P3 color space to be displayed
///   using the DCI-P3 EOTF. Note that values in such an image are interpreted as XYZ encoded color
///   data by the presentation engine.
/// - [`ColorSpaceBt709LinearExt`] specifies support for the BT709 color space to be displayed using
///   a linear EOTF.
/// - [`ColorSpaceBt709NonlinearExt`] specifies support for the BT709 color space to be displayed
///   using the SMPTE 170M EOTF.
/// - [`ColorSpaceBt2020LinearExt`] specifies support for the BT2020 color space to be displayed
///   using a linear EOTF.
/// - [`ColorSpaceHdr10St2084Ext`] specifies support for the HDR10 (BT2020 color) space to be
///   displayed using the SMPTE ST2084 Perceptual Quantizer (PQ) EOTF.
/// - [`ColorSpaceDolbyvisionExt`] specifies support for the Dolby Vision (BT2020 color space),
///   proprietary encoding, to be displayed using the SMPTE ST2084 EOTF.
/// - [`ColorSpaceHdr10HlgExt`] specifies support for the HDR10 (BT2020 color space) to be displayed
///   using the Hybrid Log Gamma (HLG) EOTF.
/// - [`ColorSpaceAdobergbLinearExt`] specifies support for the AdobeRGB color space to be displayed
///   using a linear EOTF.
/// - [`ColorSpaceAdobergbNonlinearExt`] specifies support for the AdobeRGB color space to be
///   displayed using the Gamma 2.2 EOTF.
/// - [`ColorSpacePassThroughExt`] specifies that color components are used “as is”. This is
///   intended to allow applications to supply data for color spaces not described here.
/// - [`ColorSpaceDisplayNativeAmd`] specifies support for the display’s native color space. This
///   matches the color space expectations of AMD’s FreeSync2 standard, for displays supporting it.
///The color components of non-linear color space swap chain images  **must**  have
///had the appropriate transfer function applied.
///The color space selected for the swap chain image will not affect the
///processing of data written into the image by the implementation.
///Vulkan requires that all implementations support the sRGB transfer function
///by use of an SRGB pixel format.
///Other transfer functions, such as SMPTE 170M or SMPTE2084,  **can**  be performed
///by the application shader.
///This extension defines enums for [`ColorSpaceKHR`] that correspond to
///the following color spaces:The transfer functions are described in the “Transfer Functions”
/// chapter
///of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).Except Display-P3 OETF, which is:<span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span style="height:3.30003em;vertical-align:-1.400015em;" class="strut"></span><span class="mord"><span class="mtable"><span class="col-align-r"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.900015em;"><span style="top:-3.9000150000000002em;"><span class="pstrut" style="height:3.75em;"></span><span class="mord"><span class="mord mathdefault" style="margin-right:0.05764em;">E</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.400015em;" class="vlist"><span></span></span></span></span></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.900015em;"><span style="top:-3.9000150000000002em;"><span style="height:3.75em;" class="pstrut"></span><span class="mord"><span class="mord"></span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="minner"><span style="top:0em;" class="mopen delimcenter"><span class="delimsizing size4">{</span></span><span class="mord"><span class="mtable"><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.69em;" class="vlist"><span style="top:-3.69em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord">1</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord"><span class="mord mathdefault">L</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span style="height:0.9540200000000001em;" class="vlist"><span style="top:-3.363em;margin-right:0.05em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight"><span class="mopen nulldelimiter sizing reset-size3 size6"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.8443142857142858em;" class="vlist"><span style="top:-2.656em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">2</span><span class="mord mtight">.</span><span class="mord mtight">4</span></span></span></span><span style="top:-3.2255000000000003em;"><span class="pstrut" style="height:3em;"></span><span style="border-bottom-width:0.049em;" class="frac-line mtight"></span></span><span style="top:-3.384em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">1</span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:0.344em;" class="vlist"><span></span></span></span></span></span><span class="mclose nulldelimiter sizing reset-size3 size6"></span></span></span></span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">−</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span></span></span><span style="top:-2.25em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord">1</span><span class="mord">2</span><span class="mord">.</span><span class="mord">9</span><span class="mord">2</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord mathdefault">L</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.19em;" class="vlist"><span></span></span></span></span></span><span class="arraycolsep" style="width:1em;"></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.69em;" class="vlist"><span style="top:-3.69em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord mathdefault">L</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">≤</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord">1</span></span></span><span style="top:-2.25em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord mathdefault">L</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">&lt;</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.19em;"><span></span></span></span></span></span></span></span><span class="mclose nulldelimiter"></span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.400015em;" class="vlist"><span></span></span></span></span></span></span></span></span></span></span>where L is the linear value of a color component and E is the
///encoded value (as stored in the image in memory).
///# Related
/// - [`VK_KHR_surface`]
/// - [`SurfaceFormatKHR`]
/// - [`SwapchainCreateInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkColorSpaceKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ColorSpaceKHR {
    ///[`ColorSpaceSrgbNonlinearKhr`] specifies support for the sRGB
    ///color space.
    ColorSpaceSrgbNonlinearKhr = 0,
    ///[`ColorSpaceDisplayP3NonlinearExt`] specifies support for the
    ///Display-P3 color space to be displayed using an sRGB-like EOTF (defined
    ///below).
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceDisplayP3NonlinearExt = 1000104001,
    ///[`ColorSpaceExtendedSrgbLinearExt`] specifies support for the
    ///extended sRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceExtendedSrgbLinearExt = 1000104002,
    ///[`ColorSpaceDisplayP3LinearExt`] specifies support for the
    ///Display-P3 color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceDisplayP3LinearExt = 1000104003,
    ///[`ColorSpaceDciP3NonlinearExt`] specifies support for the
    ///DCI-P3 color space to be displayed using the DCI-P3 EOTF.
    ///Note that values in such an image are interpreted as XYZ encoded color
    ///data by the presentation engine.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceDciP3NonlinearExt = 1000104004,
    ///[`ColorSpaceBt709LinearExt`] specifies support for the BT709
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceBt709LinearExt = 1000104005,
    ///[`ColorSpaceBt709NonlinearExt`] specifies support for the BT709
    ///color space to be displayed using the SMPTE 170M EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceBt709NonlinearExt = 1000104006,
    ///[`ColorSpaceBt2020LinearExt`] specifies support for the BT2020
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceBt2020LinearExt = 1000104007,
    ///[`ColorSpaceHdr10St2084Ext`] specifies support for the HDR10
    ///(BT2020 color) space to be displayed using the SMPTE ST2084 Perceptual
    ///Quantizer (PQ) EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceHdr10St2084Ext = 1000104008,
    ///[`ColorSpaceDolbyvisionExt`] specifies support for the Dolby
    ///Vision (BT2020 color space), proprietary encoding, to be displayed using
    ///the SMPTE ST2084 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceDolbyvisionExt = 1000104009,
    ///[`ColorSpaceHdr10HlgExt`] specifies support for the HDR10
    ///(BT2020 color space) to be displayed using the Hybrid Log Gamma (HLG)
    ///EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceHdr10HlgExt = 1000104010,
    ///[`ColorSpaceAdobergbLinearExt`] specifies support for the
    ///AdobeRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceAdobergbLinearExt = 1000104011,
    ///[`ColorSpaceAdobergbNonlinearExt`] specifies support for the
    ///AdobeRGB color space to be displayed using the Gamma 2.2 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceAdobergbNonlinearExt = 1000104012,
    ///[`ColorSpacePassThroughExt`] specifies that color components
    ///are used “as is”.
    ///This is intended to allow applications to supply data for color spaces
    ///not described here.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpacePassThroughExt = 1000104013,
    ///[`ColorSpaceExtendedSrgbNonlinearExt`] specifies support for
    ///the extended sRGB color space to be displayed using an sRGB EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    ColorSpaceExtendedSrgbNonlinearExt = 1000104014,
    ///[`ColorSpaceDisplayNativeAmd`] specifies support for the
    ///display’s native color space.
    ///This matches the color space expectations of AMD’s FreeSync2 standard,
    ///for displays supporting it.
    ///
    ///Provided by [`crate::extensions::amd_display_native_hdr`]
    ColorSpaceDisplayNativeAmd = 1000213000,
}
impl const Default for ColorSpaceKHR {
    fn default() -> Self {
        ColorSpaceSrgbNonlinearKhr
    }
}
impl ColorSpaceKHR {
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
///[VkSurfaceCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesKHR.html) - Structure describing capabilities of a surface
///# C Specifications
///The [`SurfaceCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_surface
///typedef struct VkSurfaceCapabilitiesKHR {
///    uint32_t                         minImageCount;
///    uint32_t                         maxImageCount;
///    VkExtent2D                       currentExtent;
///    VkExtent2D                       minImageExtent;
///    VkExtent2D                       maxImageExtent;
///    uint32_t                         maxImageArrayLayers;
///    VkSurfaceTransformFlagsKHR       supportedTransforms;
///    VkSurfaceTransformFlagBitsKHR    currentTransform;
///    VkCompositeAlphaFlagsKHR         supportedCompositeAlpha;
///    VkImageUsageFlags                supportedUsageFlags;
///} VkSurfaceCapabilitiesKHR;
///```
///# Description
/// - [`min_image_count`] is the minimum number of images the specified device supports for a
///   swapchain created for the surface, and will be at least one.
/// - [`max_image_count`] is the maximum number of images the specified device supports for a
///   swapchain created for the surface, and will be either 0, or greater than or equal to
///   [`min_image_count`]. A value of 0 means that there is no limit on the number of images, though
///   there  **may**  be limits related to the total amount of memory used by presentable images.
/// - [`current_extent`] is the current width and height of the surface, or the special value
///   (0xFFFFFFFF, 0xFFFFFFFF) indicating that the surface size will be determined by the extent of
///   a swapchain targeting the surface.
/// - [`min_image_extent`] contains the smallest valid swapchain extent for the surface on the
///   specified device. The `width` and `height` of the extent will each be less than or equal to
///   the corresponding `width` and `height` of [`current_extent`], unless [`current_extent`] has
///   the special value described above.
/// - [`max_image_extent`] contains the largest valid swapchain extent for the surface on the
///   specified device. The `width` and `height` of the extent will each be greater than or equal to
///   the corresponding `width` and `height` of [`min_image_extent`]. The `width` and `height` of
///   the extent will each be greater than or equal to the corresponding `width` and `height` of
///   [`current_extent`], unless [`current_extent`] has the special value described above.
/// - [`max_image_array_layers`] is the maximum number of layers presentable images  **can**  have
///   for a swapchain created for this device and surface, and will be at least one.
/// - [`supported_transforms`] is a bitmask of [`SurfaceTransformFlagBitsKHR`] indicating the
///   presentation transforms supported for the surface on the specified device. At least one bit
///   will be set.
/// - [`current_transform`] is [`SurfaceTransformFlagBitsKHR`] value indicating the surface’s
///   current transform relative to the presentation engine’s natural orientation.
/// - [`supported_composite_alpha`] is a bitmask of [`CompositeAlphaFlagBitsKHR`], representing the
///   alpha compositing modes supported by the presentation engine for the surface on the specified
///   device, and at least one bit will be set. Opaque composition  **can**  be achieved in any
///   alpha compositing mode by either using an image format that has no alpha component, or by
///   ensuring that all pixels in the presentable images have an alpha value of 1.0.
/// - [`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`] representing the ways the
///   application  **can**  use the presentable images of a swapchain created with
///   [`PresentModeKHR`] set to `VK_PRESENT_MODE_IMMEDIATE_KHR`, `VK_PRESENT_MODE_MAILBOX_KHR`,
///   `VK_PRESENT_MODE_FIFO_KHR` or `VK_PRESENT_MODE_FIFO_RELAXED_KHR` for the surface on the
///   specified device. `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set.
///   Implementations  **may**  support additional usages.
///# Related
/// - [`VK_KHR_surface`]
/// - [`CompositeAlphaFlagsKHR`]
/// - [`Extent2D`]
/// - [`ImageUsageFlags`]
/// - [`SurfaceCapabilities2KHR`]
/// - [`SurfaceTransformFlagBitsKHR`]
/// - [`SurfaceTransformFlagsKHR`]
/// - [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]
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
pub struct SurfaceCapabilitiesKHR {
    ///[`min_image_count`] is the minimum number of images the specified device
    ///supports for a swapchain created for the surface, and will be at least
    ///one.
    min_image_count: u32,
    ///[`max_image_count`] is the maximum number of images the specified device
    ///supports for a swapchain created for the surface, and will be either 0,
    ///or greater than or equal to [`min_image_count`].
    ///A value of 0 means that there is no limit on the number of images,
    ///though there  **may**  be limits related to the total amount of memory used
    ///by presentable images.
    max_image_count: u32,
    ///[`current_extent`] is the current width and height of the surface, or
    ///the special value (0xFFFFFFFF, 0xFFFFFFFF) indicating that the
    ///surface size will be determined by the extent of a swapchain targeting
    ///the surface.
    current_extent: Extent2D,
    ///[`min_image_extent`] contains the smallest valid swapchain extent for
    ///the surface on the specified device.
    ///The `width` and `height` of the extent will each be less than or
    ///equal to the corresponding `width` and `height` of
    ///[`current_extent`], unless [`current_extent`] has the special value
    ///described above.
    min_image_extent: Extent2D,
    ///[`max_image_extent`] contains the largest valid swapchain extent for the
    ///surface on the specified device.
    ///The `width` and `height` of the extent will each be greater than
    ///or equal to the corresponding `width` and `height` of
    ///[`min_image_extent`].
    ///The `width` and `height` of the extent will each be greater than
    ///or equal to the corresponding `width` and `height` of
    ///[`current_extent`], unless [`current_extent`] has the special value
    ///described above.
    max_image_extent: Extent2D,
    ///[`max_image_array_layers`] is the maximum number of layers presentable
    ///images  **can**  have for a swapchain created for this device and surface,
    ///and will be at least one.
    max_image_array_layers: u32,
    ///[`supported_transforms`] is a bitmask of
    ///[`SurfaceTransformFlagBitsKHR`] indicating the presentation
    ///transforms supported for the surface on the specified device.
    ///At least one bit will be set.
    supported_transforms: SurfaceTransformFlagsKHR,
    ///[`current_transform`] is [`SurfaceTransformFlagBitsKHR`] value
    ///indicating the surface’s current transform relative to the presentation
    ///engine’s natural orientation.
    current_transform: SurfaceTransformFlagBitsKHR,
    ///[`supported_composite_alpha`] is a bitmask of
    ///[`CompositeAlphaFlagBitsKHR`], representing the alpha compositing
    ///modes supported by the presentation engine for the surface on the
    ///specified device, and at least one bit will be set.
    ///Opaque composition  **can**  be achieved in any alpha compositing mode by
    ///either using an image format that has no alpha component, or by ensuring
    ///that all pixels in the presentable images have an alpha value of 1.0.
    supported_composite_alpha: CompositeAlphaFlagsKHR,
    ///[`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`]
    ///representing the ways the application  **can**  use the presentable images of
    ///a swapchain created
    ///with [`PresentModeKHR`] set to `VK_PRESENT_MODE_IMMEDIATE_KHR`,
    ///`VK_PRESENT_MODE_MAILBOX_KHR`, `VK_PRESENT_MODE_FIFO_KHR` or
    ///`VK_PRESENT_MODE_FIFO_RELAXED_KHR`
    ///for the surface on the specified device.
    ///`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set.
    ///Implementations  **may**  support additional usages.
    supported_usage_flags: ImageUsageFlags,
}
impl Default for SurfaceCapabilitiesKHR {
    fn default() -> Self {
        Self {
            min_image_count: 0,
            max_image_count: 0,
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: 0,
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
        }
    }
}
impl SurfaceCapabilitiesKHR {
    ///Gets the value of [`Self::min_image_count`]
    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }
    ///Gets the value of [`Self::max_image_count`]
    pub fn max_image_count(&self) -> u32 {
        self.max_image_count
    }
    ///Gets the value of [`Self::current_extent`]
    pub fn current_extent(&self) -> Extent2D {
        self.current_extent
    }
    ///Gets the value of [`Self::min_image_extent`]
    pub fn min_image_extent(&self) -> Extent2D {
        self.min_image_extent
    }
    ///Gets the value of [`Self::max_image_extent`]
    pub fn max_image_extent(&self) -> Extent2D {
        self.max_image_extent
    }
    ///Gets the value of [`Self::max_image_array_layers`]
    pub fn max_image_array_layers(&self) -> u32 {
        self.max_image_array_layers
    }
    ///Gets the value of [`Self::supported_transforms`]
    pub fn supported_transforms(&self) -> SurfaceTransformFlagsKHR {
        self.supported_transforms
    }
    ///Gets the value of [`Self::current_transform`]
    pub fn current_transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.current_transform
    }
    ///Gets the value of [`Self::supported_composite_alpha`]
    pub fn supported_composite_alpha(&self) -> CompositeAlphaFlagsKHR {
        self.supported_composite_alpha
    }
    ///Gets the value of [`Self::supported_usage_flags`]
    pub fn supported_usage_flags(&self) -> ImageUsageFlags {
        self.supported_usage_flags
    }
    ///Gets a mutable reference to the value of [`Self::min_image_count`]
    pub fn min_image_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_image_count`]
    pub fn max_image_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::current_extent`]
    pub fn current_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.current_extent
    }
    ///Gets a mutable reference to the value of [`Self::min_image_extent`]
    pub fn min_image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_image_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_image_extent`]
    pub fn max_image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_image_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_image_array_layers`]
    pub fn max_image_array_layers_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::supported_transforms`]
    pub fn supported_transforms_mut(&mut self) -> &mut SurfaceTransformFlagsKHR {
        &mut self.supported_transforms
    }
    ///Gets a mutable reference to the value of [`Self::current_transform`]
    pub fn current_transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.current_transform
    }
    ///Gets a mutable reference to the value of [`Self::supported_composite_alpha`]
    pub fn supported_composite_alpha_mut(&mut self) -> &mut CompositeAlphaFlagsKHR {
        &mut self.supported_composite_alpha
    }
    ///Gets a mutable reference to the value of [`Self::supported_usage_flags`]
    pub fn supported_usage_flags_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.supported_usage_flags
    }
    ///Sets the raw value of [`Self::min_image_count`]
    pub fn set_min_image_count(&mut self, value: u32) -> &mut Self {
        self.min_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_count`]
    pub fn set_max_image_count(&mut self, value: u32) -> &mut Self {
        self.max_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::current_extent`]
    pub fn set_current_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.current_extent = value;
        self
    }
    ///Sets the raw value of [`Self::min_image_extent`]
    pub fn set_min_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.min_image_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_extent`]
    pub fn set_max_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_image_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_array_layers`]
    pub fn set_max_image_array_layers(&mut self, value: u32) -> &mut Self {
        self.max_image_array_layers = value;
        self
    }
    ///Sets the raw value of [`Self::supported_transforms`]
    pub fn set_supported_transforms(
        &mut self,
        value: crate::extensions::khr_display::SurfaceTransformFlagsKHR,
    ) -> &mut Self {
        self.supported_transforms = value;
        self
    }
    ///Sets the raw value of [`Self::current_transform`]
    pub fn set_current_transform(
        &mut self,
        value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> &mut Self {
        self.current_transform = value;
        self
    }
    ///Sets the raw value of [`Self::supported_composite_alpha`]
    pub fn set_supported_composite_alpha(
        &mut self,
        value: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    ) -> &mut Self {
        self.supported_composite_alpha = value;
        self
    }
    ///Sets the raw value of [`Self::supported_usage_flags`]
    pub fn set_supported_usage_flags(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.supported_usage_flags = value;
        self
    }
}
///[VkSurfaceFormatKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceFormatKHR.html) - Structure describing a supported swapchain format-color space pair
///# C Specifications
///The [`SurfaceFormatKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_surface
///typedef struct VkSurfaceFormatKHR {
///    VkFormat           format;
///    VkColorSpaceKHR    colorSpace;
///} VkSurfaceFormatKHR;
///```
///# Members
/// - [`format`] is a [`Format`] that is compatible with the specified surface.
/// - [`color_space`] is a presentation [`ColorSpaceKHR`] that is compatible with the surface.
///# Related
/// - [`VK_KHR_surface`]
/// - [`ColorSpaceKHR`]
/// - [`Format`]
/// - [`SurfaceFormat2KHR`]
/// - [`GetPhysicalDeviceSurfaceFormatsKHR`]
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
pub struct SurfaceFormatKHR {
    ///[`format`] is a [`Format`] that is compatible with the specified
    ///surface.
    format: Format,
    ///[`color_space`] is a presentation [`ColorSpaceKHR`] that is
    ///compatible with the surface.
    color_space: ColorSpaceKHR,
}
impl Default for SurfaceFormatKHR {
    fn default() -> Self {
        Self {
            format: Default::default(),
            color_space: Default::default(),
        }
    }
}
impl SurfaceFormatKHR {
    ///Gets the value of [`Self::format`]
    pub fn format(&self) -> Format {
        self.format
    }
    ///Gets the value of [`Self::color_space`]
    pub fn color_space(&self) -> ColorSpaceKHR {
        self.color_space
    }
    ///Gets a mutable reference to the value of [`Self::format`]
    pub fn format_mut(&mut self) -> &mut Format {
        &mut self.format
    }
    ///Gets a mutable reference to the value of [`Self::color_space`]
    pub fn color_space_mut(&mut self) -> &mut ColorSpaceKHR {
        &mut self.color_space
    }
    ///Sets the raw value of [`Self::format`]
    pub fn set_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.format = value;
        self
    }
    ///Sets the raw value of [`Self::color_space`]
    pub fn set_color_space(&mut self, value: crate::extensions::khr_surface::ColorSpaceKHR) -> &mut Self {
        self.color_space = value;
        self
    }
}
///[VkSurfaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceKHR.html) - Opaque handle to a surface object
///# C Specifications
///Native platform surface or window objects are abstracted by surface objects,
///which are represented by [`SurfaceKHR`] handles:
///```c
///// Provided by VK_KHR_surface
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSurfaceKHR)
///```
///# Related
/// - [`VK_KHR_surface`]
/// - [`PhysicalDeviceSurfaceInfo2KHR`]
/// - [`SwapchainCreateInfoKHR`]
/// - [`CreateAndroidSurfaceKHR`]
/// - [`CreateDirectFBSurfaceEXT`]
/// - [`CreateDisplayPlaneSurfaceKHR`]
/// - [`CreateHeadlessSurfaceEXT`]
/// - [`CreateIosSurfaceMVK`]
/// - [`CreateImagePipeSurfaceFUCHSIA`]
/// - [`CreateMacOsSurfaceMVK`]
/// - [`CreateMetalSurfaceEXT`]
/// - [`CreateScreenSurfaceQNX`]
/// - [`CreateStreamDescriptorSurfaceGGP`]
/// - [`CreateViSurfaceNN`]
/// - [`CreateWaylandSurfaceKHR`]
/// - [`CreateWin32SurfaceKHR`]
/// - [`CreateXcbSurfaceKHR`]
/// - [`CreateXlibSurfaceKHR`]
/// - [`DestroySurfaceKHR`]
/// - [`GetDeviceGroupSurfacePresentModesKHR`]
/// - [`GetPhysicalDevicePresentRectanglesKHR`]
/// - [`GetPhysicalDeviceSurfaceCapabilities2EXT`]
/// - [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]
/// - [`GetPhysicalDeviceSurfaceFormatsKHR`]
/// - [`GetPhysicalDeviceSurfacePresentModesKHR`]
/// - [`GetPhysicalDeviceSurfaceSupportKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct SurfaceKHR(pub u64);
impl SurfaceKHR {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub const fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for SurfaceKHR {}
impl Default for SurfaceKHR {
    fn default() -> Self {
        Self::default()
    }
}
impl std::fmt::Pointer for SurfaceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
impl std::fmt::Debug for SurfaceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
