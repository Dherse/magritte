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
/// - [`PRESENT_MODE_IMMEDIATE`] specifies that the presentation
///engine does not wait for a vertical blanking period to update the
///current image, meaning this mode **may** result in visible tearing.
///No internal queuing of presentation requests is needed, as the requests
///are applied immediately.
/// - [`PRESENT_MODE_MAILBOX`] specifies that the presentation engine
///waits for the next vertical blanking period to update the current image.
///Tearing **cannot** be observed.
///An internal single-entry queue is used to hold pending presentation
///requests.
///If the queue is full when a new presentation request is received, the
///new request replaces the existing entry, and any images associated with
///the prior entry become available for re-use by the application.
///One request is removed from the queue and processed during each vertical
///blanking period in which the queue is non-empty.
/// - [`PRESENT_MODE_FIFO`] specifies that the presentation engine
///waits for the next vertical blanking period to update the current image.
///Tearing **cannot** be observed.
///An internal queue is used to hold pending presentation requests.
///New requests are appended to the end of the queue, and one request is
///removed from the beginning of the queue and processed during each
///vertical blanking period in which the queue is non-empty.
///This is the only value of `presentMode` that is **required** to be
///supported.
/// - [`PRESENT_MODE_FIFO_RELAXED`] specifies that the presentation
///engine generally waits for the next vertical blanking period to update
///the current image.
///If a vertical blanking period has already passed since the last update
///of the current image then the presentation engine does not wait for
///another vertical blanking period for the update, meaning this mode **may**
///result in visible tearing in this case.
///This mode is useful for reducing visual stutter with an application that
///will mostly present a new image before the next vertical blanking
///period, but may occasionally be late, and present a new image just after
///the next vertical blanking period.
///An internal queue is used to hold pending presentation requests.
///New requests are appended to the end of the queue, and one request is
///removed from the beginning of the queue and processed during or after
///each vertical blanking period in which the queue is non-empty.
/// - [`PRESENT_MODE_SHARED_DEMAND_REFRESH`] specifies that the
///presentation engine and application have concurrent access to a single
///image, which is referred to as a *shared presentable image*.
///The presentation engine is only required to update the current image
///after a new presentation request is received.
///Therefore the application **must** make a presentation request whenever an
///update is required.
///However, the presentation engine **may** update the current image at any
///point, meaning this mode **may** result in visible tearing.
/// - [`PRESENT_MODE_SHARED_CONTINUOUS_REFRESH`] specifies that the
///presentation engine and application have concurrent access to a single
///image, which is referred to as a *shared presentable image*.
///The presentation engine periodically updates the current image on its
///regular refresh cycle.
///The application is only required to make one initial presentation
///request, after which the presentation engine **must** update the current
///image without any need for further presentation requests.
///The application **can** indicate the image contents have been updated by
///making a presentation request, but this does not guarantee the timing of
///when it will be updated.
///This mode **may** result in visible tearing if rendering to the image is
///not timed correctly.The supported [`ImageUsageFlagBits`] of the presentable images of a
///swapchain created for a surface **may** differ depending on the presentation
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PresentModeKHR(i32);
impl const Default for PresentModeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PresentModeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PresentModeKHR")
            .field(match *self {
                Self::PRESENT_MODE_IMMEDIATE => &"PRESENT_MODE_IMMEDIATE",
                Self::PRESENT_MODE_MAILBOX => &"PRESENT_MODE_MAILBOX",
                Self::PRESENT_MODE_FIFO => &"PRESENT_MODE_FIFO",
                Self::PRESENT_MODE_FIFO_RELAXED => &"PRESENT_MODE_FIFO_RELAXED",
                Self::PRESENT_MODE_SHARED_DEMAND_REFRESH => &"PRESENT_MODE_SHARED_DEMAND_REFRESH",
                Self::PRESENT_MODE_SHARED_CONTINUOUS_REFRESH => &"PRESENT_MODE_SHARED_CONTINUOUS_REFRESH",
                other => unreachable!("invalid value for `PresentModeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl PresentModeKHR {
    ///[`PRESENT_MODE_IMMEDIATE`] specifies that the presentation
    ///engine does not wait for a vertical blanking period to update the
    ///current image, meaning this mode **may** result in visible tearing.
    ///No internal queuing of presentation requests is needed, as the requests
    ///are applied immediately.
    pub const PRESENT_MODE_IMMEDIATE: Self = Self(0);
    ///[`PRESENT_MODE_MAILBOX`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing **cannot** be observed.
    ///An internal single-entry queue is used to hold pending presentation
    ///requests.
    ///If the queue is full when a new presentation request is received, the
    ///new request replaces the existing entry, and any images associated with
    ///the prior entry become available for re-use by the application.
    ///One request is removed from the queue and processed during each vertical
    ///blanking period in which the queue is non-empty.
    pub const PRESENT_MODE_MAILBOX: Self = Self(1);
    ///[`PRESENT_MODE_FIFO`] specifies that the presentation engine
    ///waits for the next vertical blanking period to update the current image.
    ///Tearing **cannot** be observed.
    ///An internal queue is used to hold pending presentation requests.
    ///New requests are appended to the end of the queue, and one request is
    ///removed from the beginning of the queue and processed during each
    ///vertical blanking period in which the queue is non-empty.
    ///This is the only value of `presentMode` that is **required** to be
    ///supported.
    pub const PRESENT_MODE_FIFO: Self = Self(2);
    ///[`PRESENT_MODE_FIFO_RELAXED`] specifies that the presentation
    ///engine generally waits for the next vertical blanking period to update
    ///the current image.
    ///If a vertical blanking period has already passed since the last update
    ///of the current image then the presentation engine does not wait for
    ///another vertical blanking period for the update, meaning this mode **may**
    ///result in visible tearing in this case.
    ///This mode is useful for reducing visual stutter with an application that
    ///will mostly present a new image before the next vertical blanking
    ///period, but may occasionally be late, and present a new image just after
    ///the next vertical blanking period.
    ///An internal queue is used to hold pending presentation requests.
    ///New requests are appended to the end of the queue, and one request is
    ///removed from the beginning of the queue and processed during or after
    ///each vertical blanking period in which the queue is non-empty.
    pub const PRESENT_MODE_FIFO_RELAXED: Self = Self(3);
    ///[`PRESENT_MODE_SHARED_DEMAND_REFRESH`] specifies that the
    ///presentation engine and application have concurrent access to a single
    ///image, which is referred to as a *shared presentable image*.
    ///The presentation engine is only required to update the current image
    ///after a new presentation request is received.
    ///Therefore the application **must** make a presentation request whenever an
    ///update is required.
    ///However, the presentation engine **may** update the current image at any
    ///point, meaning this mode **may** result in visible tearing.
    ///
    ///Provided by [`crate::extensions::khr_shared_presentable_image`]
    pub const PRESENT_MODE_SHARED_DEMAND_REFRESH: Self = Self(1000111000);
    ///[`PRESENT_MODE_SHARED_CONTINUOUS_REFRESH`] specifies that the
    ///presentation engine and application have concurrent access to a single
    ///image, which is referred to as a *shared presentable image*.
    ///The presentation engine periodically updates the current image on its
    ///regular refresh cycle.
    ///The application is only required to make one initial presentation
    ///request, after which the presentation engine **must** update the current
    ///image without any need for further presentation requests.
    ///The application **can** indicate the image contents have been updated by
    ///making a presentation request, but this does not guarantee the timing of
    ///when it will be updated.
    ///This mode **may** result in visible tearing if rendering to the image is
    ///not timed correctly.
    ///
    ///Provided by [`crate::extensions::khr_shared_presentable_image`]
    pub const PRESENT_MODE_SHARED_CONTINUOUS_REFRESH: Self = Self(1000111001);
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
/// - [`COLOR_SPACE_SRGB_NONLINEAR`] specifies support for the sRGB
///color space.
/// - [`COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT`] specifies support for the
///Display-P3 color space to be displayed using an sRGB-like EOTF (defined
///below).
/// - [`COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT`] specifies support for the
///extended sRGB color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT`] specifies support for
///the extended sRGB color space to be displayed using an sRGB EOTF.
/// - [`COLOR_SPACE_DISPLAY_P3_LINEAR_EXT`] specifies support for the
///Display-P3 color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_DCI_P3_NONLINEAR_EXT`] specifies support for the
///DCI-P3 color space to be displayed using the DCI-P3 EOTF.
///Note that values in such an image are interpreted as XYZ encoded color
///data by the presentation engine.
/// - [`COLOR_SPACE_BT709_LINEAR_EXT`] specifies support for the BT709
///color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_BT709_NONLINEAR_EXT`] specifies support for the BT709
///color space to be displayed using the SMPTE 170M EOTF.
/// - [`COLOR_SPACE_BT2020_LINEAR_EXT`] specifies support for the BT2020
///color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_HDR10_ST2084_EXT`] specifies support for the HDR10
///(BT2020 color) space to be displayed using the SMPTE ST2084 Perceptual
///Quantizer (PQ) EOTF.
/// - [`COLOR_SPACE_DOLBYVISION_EXT`] specifies support for the Dolby
///Vision (BT2020 color space), proprietary encoding, to be displayed using
///the SMPTE ST2084 EOTF.
/// - [`COLOR_SPACE_HDR10_HLG_EXT`] specifies support for the HDR10
///(BT2020 color space) to be displayed using the Hybrid Log Gamma (HLG)
///EOTF.
/// - [`COLOR_SPACE_ADOBERGB_LINEAR_EXT`] specifies support for the
///AdobeRGB color space to be displayed using a linear EOTF.
/// - [`COLOR_SPACE_ADOBERGB_NONLINEAR_EXT`] specifies support for the
///AdobeRGB color space to be displayed using the Gamma 2.2 EOTF.
/// - [`COLOR_SPACE_PASS_THROUGH_EXT`] specifies that color components
///are used “as is”.
///This is intended to allow applications to supply data for color spaces
///not described here.
/// - [`COLOR_SPACE_DISPLAY_NATIVE_AMD`] specifies support for the
///display’s native color space.
///This matches the color space expectations of AMD’s FreeSync2 standard,
///for displays supporting it.The color components of non-linear color space swap chain images
/// **must** have
///had the appropriate transfer function applied.
///The color space selected for the swap chain image will not affect the
///processing of data written into the image by the implementation.
///Vulkan requires that all implementations support the sRGB transfer function
///by use of an SRGB pixel format.
///Other transfer functions, such as SMPTE 170M or SMPTE2084, **can** be performed
///by the application shader.
///This extension defines enums for [`ColorSpaceKHR`] that correspond to
///the following color spaces:The transfer functions are described in the “Transfer Functions”
/// chapter
///of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).Except Display-P3 OETF, which is:<span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span style="height:3.30003em;vertical-align:-1.400015em;" class="strut"></span><span class="mord"><span class="mtable"><span class="col-align-r"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.900015em;" class="vlist"><span style="top:-3.9000150000000002em;"><span class="pstrut" style="height:3.75em;"></span><span class="mord"><span style="margin-right:0.05764em;" class="mord mathdefault">E</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.400015em;"><span></span></span></span></span></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.900015em;"><span style="top:-3.9000150000000002em;"><span class="pstrut" style="height:3.75em;"></span><span class="mord"><span class="mord"></span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="minner"><span style="top:0em;" class="mopen delimcenter"><span class="delimsizing size4">{</span></span><span class="mord"><span class="mtable"><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:1.69em;"><span style="top:-3.69em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord">1</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mord"><span class="mord mathdefault">L</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span style="height:0.9540200000000001em;" class="vlist"><span style="top:-3.363em;margin-right:0.05em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight"><span class="mopen nulldelimiter sizing reset-size3 size6"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:0.8443142857142858em;"><span style="top:-2.656em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">2</span><span class="mord mtight">.</span><span class="mord mtight">4</span></span></span></span><span style="top:-3.2255000000000003em;"><span class="pstrut" style="height:3em;"></span><span class="frac-line mtight" style="border-bottom-width:0.049em;"></span></span><span style="top:-3.384em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">1</span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.344em;"><span></span></span></span></span></span><span class="mclose nulldelimiter sizing reset-size3 size6"></span></span></span></span></span></span></span></span></span></span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">−</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span></span></span><span style="top:-2.25em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord">1</span><span class="mord">2</span><span class="mord">.</span><span class="mord">9</span><span class="mord">2</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mord mathdefault">L</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.19em;"><span></span></span></span></span></span><span class="arraycolsep" style="width:1em;"></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.69em;" class="vlist"><span style="top:-3.69em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">≤</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord mathdefault">L</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">≤</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord">1</span></span></span><span style="top:-2.25em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">≤</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord mathdefault">L</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">&lt;</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.19em;"><span></span></span></span></span></span></span></span><span class="mclose nulldelimiter"></span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.400015em;"><span></span></span></span></span></span></span></span></span></span></span>where L is the linear value of a color component and E is the
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ColorSpaceKHR(i32);
impl const Default for ColorSpaceKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ColorSpaceKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ColorSpaceKHR")
            .field(match *self {
                Self::COLOR_SPACE_SRGB_NONLINEAR => &"COLOR_SPACE_SRGB_NONLINEAR",
                Self::COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT => &"COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT",
                Self::COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT => &"COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT",
                Self::COLOR_SPACE_DISPLAY_P3_LINEAR_EXT => &"COLOR_SPACE_DISPLAY_P3_LINEAR_EXT",
                Self::COLOR_SPACE_DCI_P3_NONLINEAR_EXT => &"COLOR_SPACE_DCI_P3_NONLINEAR_EXT",
                Self::COLOR_SPACE_BT709_LINEAR_EXT => &"COLOR_SPACE_BT709_LINEAR_EXT",
                Self::COLOR_SPACE_BT709_NONLINEAR_EXT => &"COLOR_SPACE_BT709_NONLINEAR_EXT",
                Self::COLOR_SPACE_BT2020_LINEAR_EXT => &"COLOR_SPACE_BT2020_LINEAR_EXT",
                Self::COLOR_SPACE_HDR10_ST2084_EXT => &"COLOR_SPACE_HDR10_ST2084_EXT",
                Self::COLOR_SPACE_DOLBYVISION_EXT => &"COLOR_SPACE_DOLBYVISION_EXT",
                Self::COLOR_SPACE_HDR10_HLG_EXT => &"COLOR_SPACE_HDR10_HLG_EXT",
                Self::COLOR_SPACE_ADOBERGB_LINEAR_EXT => &"COLOR_SPACE_ADOBERGB_LINEAR_EXT",
                Self::COLOR_SPACE_ADOBERGB_NONLINEAR_EXT => &"COLOR_SPACE_ADOBERGB_NONLINEAR_EXT",
                Self::COLOR_SPACE_PASS_THROUGH_EXT => &"COLOR_SPACE_PASS_THROUGH_EXT",
                Self::COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT => &"COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT",
                Self::COLOR_SPACE_DISPLAY_NATIVE_AMD => &"COLOR_SPACE_DISPLAY_NATIVE_AMD",
                other => unreachable!("invalid value for `ColorSpaceKHR`: {:?}", other),
            })
            .finish()
    }
}
impl ColorSpaceKHR {
    ///[`COLOR_SPACE_SRGB_NONLINEAR`] specifies support for the sRGB
    ///color space.
    pub const COLOR_SPACE_SRGB_NONLINEAR: Self = Self(0);
    ///[`COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT`] specifies support for the
    ///Display-P3 color space to be displayed using an sRGB-like EOTF (defined
    ///below).
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: Self = Self(1000104001);
    ///[`COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT`] specifies support for the
    ///extended sRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: Self = Self(1000104002);
    ///[`COLOR_SPACE_DISPLAY_P3_LINEAR_EXT`] specifies support for the
    ///Display-P3 color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DISPLAY_P3_LINEAR_EXT: Self = Self(1000104003);
    ///[`COLOR_SPACE_DCI_P3_NONLINEAR_EXT`] specifies support for the
    ///DCI-P3 color space to be displayed using the DCI-P3 EOTF.
    ///Note that values in such an image are interpreted as XYZ encoded color
    ///data by the presentation engine.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DCI_P3_NONLINEAR_EXT: Self = Self(1000104004);
    ///[`COLOR_SPACE_BT709_LINEAR_EXT`] specifies support for the BT709
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_BT709_LINEAR_EXT: Self = Self(1000104005);
    ///[`COLOR_SPACE_BT709_NONLINEAR_EXT`] specifies support for the BT709
    ///color space to be displayed using the SMPTE 170M EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_BT709_NONLINEAR_EXT: Self = Self(1000104006);
    ///[`COLOR_SPACE_BT2020_LINEAR_EXT`] specifies support for the BT2020
    ///color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_BT2020_LINEAR_EXT: Self = Self(1000104007);
    ///[`COLOR_SPACE_HDR10_ST2084_EXT`] specifies support for the HDR10
    ///(BT2020 color) space to be displayed using the SMPTE ST2084 Perceptual
    ///Quantizer (PQ) EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_HDR10_ST2084_EXT: Self = Self(1000104008);
    ///[`COLOR_SPACE_DOLBYVISION_EXT`] specifies support for the Dolby
    ///Vision (BT2020 color space), proprietary encoding, to be displayed using
    ///the SMPTE ST2084 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DOLBYVISION_EXT: Self = Self(1000104009);
    ///[`COLOR_SPACE_HDR10_HLG_EXT`] specifies support for the HDR10
    ///(BT2020 color space) to be displayed using the Hybrid Log Gamma (HLG)
    ///EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_HDR10_HLG_EXT: Self = Self(1000104010);
    ///[`COLOR_SPACE_ADOBERGB_LINEAR_EXT`] specifies support for the
    ///AdobeRGB color space to be displayed using a linear EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_ADOBERGB_LINEAR_EXT: Self = Self(1000104011);
    ///[`COLOR_SPACE_ADOBERGB_NONLINEAR_EXT`] specifies support for the
    ///AdobeRGB color space to be displayed using the Gamma 2.2 EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: Self = Self(1000104012);
    ///[`COLOR_SPACE_PASS_THROUGH_EXT`] specifies that color components
    ///are used “as is”.
    ///This is intended to allow applications to supply data for color spaces
    ///not described here.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_PASS_THROUGH_EXT: Self = Self(1000104013);
    ///[`COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT`] specifies support for
    ///the extended sRGB color space to be displayed using an sRGB EOTF.
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1000104014);
    ///[`COLOR_SPACE_DISPLAY_NATIVE_AMD`] specifies support for the
    ///display’s native color space.
    ///This matches the color space expectations of AMD’s FreeSync2 standard,
    ///for displays supporting it.
    ///
    ///Provided by [`crate::extensions::amd_display_native_hdr`]
    pub const COLOR_SPACE_DISPLAY_NATIVE_AMD: Self = Self(1000213000);
    ///No documentation found
    pub const COLORSPACE_SRGB_NONLINEAR: Self = Self::COLOR_SPACE_SRGB_NONLINEAR;
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::ext_swapchain_colorspace`]
    pub const COLOR_SPACE_DCI_P3_LINEAR_EXT: Self = Self::COLOR_SPACE_DISPLAY_P3_LINEAR_EXT;
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
}
