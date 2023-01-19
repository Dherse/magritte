[VkColorSpaceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorSpaceKHR.html) - Supported color space of the presentation engine

# C Specifications
Possible values of [`SurfaceFormatKHR::color_space`], specifying
supported color spaces of a presentation engine, are:
```c
// Provided by VK_KHR_surface
typedef enum VkColorSpaceKHR {
    VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT = 1000104001,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT = 1000104002,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT = 1000104003,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT = 1000104004,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_BT709_LINEAR_EXT = 1000104005,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_BT709_NONLINEAR_EXT = 1000104006,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_BT2020_LINEAR_EXT = 1000104007,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_HDR10_ST2084_EXT = 1000104008,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_DOLBYVISION_EXT = 1000104009,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_HDR10_HLG_EXT = 1000104010,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT = 1000104011,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT = 1000104012,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_PASS_THROUGH_EXT = 1000104013,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT = 1000104014,
  // Provided by VK_AMD_display_native_hdr
    VK_COLOR_SPACE_DISPLAY_NATIVE_AMD = 1000213000,
    VK_COLORSPACE_SRGB_NONLINEAR_KHR = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
  // Provided by VK_EXT_swapchain_colorspace
    VK_COLOR_SPACE_DCI_P3_LINEAR_EXT = VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT,
} VkColorSpaceKHR;
```

# Description
- [`SRGB_NONLINEAR`] specifies support for the sRGB color space.
- [`DISPLAY_P3_NONLINEAR_EXT`] specifies support for the Display-P3 color space to be displayed using an sRGB-like EOTF (defined below).
- [`EXTENDED_SRGB_LINEAR_EXT`] specifies support for the extended sRGB color space to be displayed using a linear EOTF.
- [`EXTENDED_SRGB_NONLINEAR_EXT`] specifies support for the extended sRGB color space to be displayed using an sRGB EOTF.
- [`DISPLAY_P3_LINEAR_EXT`] specifies support for the Display-P3 color space to be displayed using a linear EOTF.
- [`DCI_P3_NONLINEAR_EXT`] specifies support for the DCI-P3 color space to be displayed using the DCI-P3 EOTF. Note that values in such an image are interpreted as XYZ encoded color data by the presentation engine.
- [`BT709_LINEAR_EXT`] specifies support for the BT709 color space to be displayed using a linear EOTF.
- [`BT709_NONLINEAR_EXT`] specifies support for the BT709 color space to be displayed using the SMPTE 170M EOTF.
- [`BT2020_LINEAR_EXT`] specifies support for the BT2020 color space to be displayed using a linear EOTF.
- [`HDR10_ST2084_EXT`] specifies support for the HDR10 (BT2020 color) space to be displayed using the SMPTE ST2084 Perceptual Quantizer (PQ) EOTF.
- [`DOLBYVISION_EXT`] specifies support for the Dolby Vision (BT2020 color space), proprietary encoding, to be displayed using the SMPTE ST2084 EOTF.
- [`HDR10_HLG_EXT`] specifies support for the HDR10 (BT2020 color space) to be displayed using the Hybrid Log Gamma (HLG) EOTF.
- [`ADOBERGB_LINEAR_EXT`] specifies support for the AdobeRGB color space to be displayed using a linear EOTF.
- [`ADOBERGB_NONLINEAR_EXT`] specifies support for the AdobeRGB color space to be displayed using the Gamma 2.2 EOTF.
- [`PASS_THROUGH_EXT`] specifies that color components are used “as is”. This is intended to allow applications to supply data for color spaces not described here.
- [`DISPLAY_NATIVE_AMD`] specifies support for the display’s native color space. This matches the color space expectations of AMD’s FreeSync2 standard, for displays supporting it.
The color components of non-linear color space swap chain images  **must**  have
had the appropriate transfer function applied.
The color space selected for the swap chain image will not affect the
processing of data written into the image by the implementation.
Vulkan requires that all implementations support the sRGB transfer function
by use of an SRGB pixel format.
Other transfer functions, such as SMPTE 170M or SMPTE2084,  **can**  be performed
by the application shader.
This extension defines enums for [`ColorSpaceKHR`] that correspond to
the following color spaces:The transfer functions are described in the “Transfer Functions” chapter
of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).Except Display-P3 OETF, which is:<span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span style="height:3.30003em;vertical-align:-1.400015em;" class="strut"></span><span class="mord"><span class="mtable"><span class="col-align-r"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.900015em;" class="vlist"><span style="top:-3.9000150000000002em;"><span class="pstrut" style="height:3.75em;"></span><span class="mord"><span class="mord mathdefault" style="margin-right:0.05764em;">E</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.400015em;" class="vlist"><span></span></span></span></span></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.900015em;" class="vlist"><span style="top:-3.9000150000000002em;"><span style="height:3.75em;" class="pstrut"></span><span class="mord"><span class="mord"></span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">=</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="minner"><span class="mopen delimcenter" style="top:0em;"><span class="delimsizing size4">{</span></span><span class="mord"><span class="mtable"><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.69em;" class="vlist"><span style="top:-3.69em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord">1</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mbin">×</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord"><span class="mord mathdefault">L</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span style="height:0.9540200000000001em;" class="vlist"><span style="top:-3.363em;margin-right:0.05em;"><span style="height:3em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight"><span class="mopen nulldelimiter sizing reset-size3 size6"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.8443142857142858em;" class="vlist"><span style="top:-2.656em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">2</span><span class="mord mtight">.</span><span class="mord mtight">4</span></span></span></span><span style="top:-3.2255000000000003em;"><span class="pstrut" style="height:3em;"></span><span class="frac-line mtight" style="border-bottom-width:0.049em;"></span></span><span style="top:-3.384em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mtight"><span class="mord mtight">1</span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:0.344em;" class="vlist"><span></span></span></span></span></span><span class="mclose nulldelimiter sizing reset-size3 size6"></span></span></span></span></span></span></span></span></span></span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mbin">−</span><span class="mspace" style="margin-right:0.2222222222222222em;"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">5</span><span class="mord">5</span></span></span><span style="top:-2.25em;"><span class="pstrut" style="height:3.008em;"></span><span class="mord"><span class="mord">1</span><span class="mord">2</span><span class="mord">.</span><span class="mord">9</span><span class="mord">2</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mbin">×</span><span style="margin-right:0.2222222222222222em;" class="mspace"></span><span class="mord mathdefault">L</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.19em;"><span></span></span></span></span></span><span style="width:1em;" class="arraycolsep"></span><span class="col-align-l"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:1.69em;" class="vlist"><span style="top:-3.69em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">≤</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord mathdefault">L</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord">1</span></span></span><span style="top:-2.25em;"><span style="height:3.008em;" class="pstrut"></span><span class="mord"><span class="mord text"><span class="mord">for</span></span><span class="mspace">&nbsp;</span><span class="mord">0</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mrel">≤</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mord mathdefault">L</span><span class="mspace" style="margin-right:0.2777777777777778em;"></span><span class="mrel">&lt;</span><span style="margin-right:0.2777777777777778em;" class="mspace"></span><span class="mord">0</span><span class="mord">.</span><span class="mord">0</span><span class="mord">0</span><span class="mord">3</span><span class="mord">0</span><span class="mord">1</span><span class="mord">8</span><span class="mord">6</span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:1.19em;"><span></span></span></span></span></span></span></span><span class="mclose nulldelimiter"></span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:1.400015em;" class="vlist"><span></span></span></span></span></span></span></span></span></span></span>where L is the linear value of a color component and E is the
encoded value (as stored in the image in memory).

# Related
- [`VK_KHR_surface`]
- [`SurfaceFormatKHR`]
- [`SwapchainCreateInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        