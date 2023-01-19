[VkViewport](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkViewport.html) - Structure specifying a viewport

# C Specifications
The [`Viewport`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkViewport {
    float    x;
    float    y;
    float    width;
    float    height;
    float    minDepth;
    float    maxDepth;
} VkViewport;
```

# Members
- [`x`] and [`y`] are the viewport’s upper left corner (x,y).
- [`width`] and [`height`] are the viewport’s width and height, respectively.
- [`min_depth`] and [`max_depth`] are the depth range for the viewport.

# Description
The framebuffer depth coordinate `z`<sub>f</sub> **may**  be represented using
either a fixed-point or floating-point representation.
However, a floating-point representation  **must**  be used if the depth/stencil
attachment has a floating-point depth component.
If an m-bit fixed-point representation is used, we assume that it
represents each value <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span class="strut" style="height:1.283439em;vertical-align:-0.403331em;"></span><span class="mord"><span class="mopen nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:0.8801079999999999em;"><span style="top:-2.655em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight"><span class="mord mtight">2</span><span class="msupsub"><span class="vlist-t"><span class="vlist-r"><span style="height:0.5935428571428571em;" class="vlist"><span style="top:-2.786em;margin-right:0.07142857142857144em;"><span class="pstrut" style="height:2.5em;"></span><span class="sizing reset-size3 size1 mtight"><span class="mord mathdefault mtight">m</span></span></span></span></span></span></span></span><span class="mbin mtight">−</span><span class="mord mtight">1</span></span></span></span><span style="top:-3.23em;"><span style="height:3em;" class="pstrut"></span><span style="border-bottom-width:0.04em;" class="frac-line"></span></span><span style="top:-3.394em;"><span class="pstrut" style="height:3em;"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span style="margin-right:0.03148em;" class="mord mathdefault mtight">k</span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:0.403331em;" class="vlist"><span></span></span></span></span></span><span class="mclose nulldelimiter"></span></span></span></span></span>, where k ∈ {
0, 1, …​, 2<sup>m</sup>-1 }, as k (e.g. 1.0 is represented in binary as a
string of all ones).The viewport parameters shown in the above equations are found from these
values as
* o<sub>x</sub> = [`x`] +  [`width`] / 2
* o<sub>y</sub> = [`y`] +  [`height`] / 2
* o<sub>z</sub> = [`min_depth`]
* p<sub>x</sub> = [`width`]
* p<sub>y</sub> = [`height`]
* p<sub>z</sub> = [`max_depth`] - [`min_depth`].
If a render pass transform is enabled, the values (p<sub>x</sub>,p<sub>y</sub>) and
(o<sub>x</sub>, o<sub>y</sub>) defining the viewport are transformed as described in
[render pass transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-renderpass-transform) before
participating in the viewport transform.The application  **can**  specify a negative term for [`height`], which has the
effect of negating the y coordinate in clip space before performing the
transform.
When using a negative [`height`], the application  **should**  also adjust the
[`y`] value to point to the lower left corner of the viewport instead of
the upper left corner.
Using the negative [`height`] allows the application to avoid having to
negate the y component of the `Position` output from the last
[pre-rasterization shader
stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).The width and height of the [implementation-dependent maximum viewport dimensions](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxViewportDimensions) **must**  be greater than
or equal to the width and height of the largest image which  **can**  be created
and attached to a framebuffer.The floating-point viewport bounds are represented with an
[implementation-dependent precision](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-viewportSubPixelBits).
## Valid Usage
-  [`width`] **must**  be greater than `0.0`
-  [`width`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_viewport_dimensions`][0]
-    The absolute value of [`height`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_viewport_dimensions`][1]
-  [`x`] **must**  be greater than or equal to `viewportBoundsRange`[0]
-  ([`x`] +  [`width`]) **must**  be less than or equal to `viewportBoundsRange`[1]
-  [`y`] **must**  be greater than or equal to `viewportBoundsRange`[0]
-  [`y`] **must**  be less than or equal to `viewportBoundsRange`[1]
-  ([`y`] +  [`height`]) **must**  be greater than or equal to `viewportBoundsRange`[0]
-  ([`y`] +  [`height`]) **must**  be less than or equal to `viewportBoundsRange`[1]
-    Unless `[`VK_EXT_depth_range_unrestricted`]` extension is enabled [`min_depth`] **must**  be between `0.0` and `1.0`, inclusive
-    Unless `[`VK_EXT_depth_range_unrestricted`]` extension is enabled [`max_depth`] **must**  be between `0.0` and `1.0`, inclusive

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferInheritanceViewportScissorInfoNV`]
- [`PipelineViewportStateCreateInfo`]
- [`cmd_set_viewport`]
- [`cmd_set_viewport_with_count`]
- [`cmd_set_viewport_with_count_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        