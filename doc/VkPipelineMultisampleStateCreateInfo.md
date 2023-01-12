[VkPipelineMultisampleStateCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineMultisampleStateCreateInfo.html) - Structure specifying parameters of a newly created pipeline multisample state

# C Specifications
The [`PipelineMultisampleStateCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineMultisampleStateCreateInfo {
    VkStructureType                          sType;
    const void*                              pNext;
    VkPipelineMultisampleStateCreateFlags    flags;
    VkSampleCountFlagBits                    rasterizationSamples;
    VkBool32                                 sampleShadingEnable;
    float                                    minSampleShading;
    const VkSampleMask*                      pSampleMask;
    VkBool32                                 alphaToCoverageEnable;
    VkBool32                                 alphaToOneEnable;
} VkPipelineMultisampleStateCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`rasterization_samples`] is a [`SampleCountFlagBits`] value specifying the number of samples used in rasterization.
- [`sample_shading_enable`] **can**  be used to enable [Sample Shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-sampleshading).
- [`min_sample_shading`] specifies a minimum fraction of sample shading if [`sample_shading_enable`] is set to `VK_TRUE`.
- [`sample_mask`] is a pointer to an array of [`SampleMask`] values used in the [sample mask test](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-samplemask).
- [`alpha_to_coverage_enable`] controls whether a temporary coverage value is generated based on the alpha component of the fragment’s first color output as specified in the [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-covg) section.
- [`alpha_to_one_enable`] controls whether the alpha component of the fragment’s first color output is replaced with one as described in [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-covg).

# Description
Each bit in the sample mask is associated with a unique
[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) as defined for the
[coverage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask).
Each bit b for mask word w in the sample mask corresponds to
sample index i, where i = 32 × w +  b.
[`sample_mask`] has a length equal to ⌈
[`rasterization_samples`] / 32 ⌉ words.If [`sample_mask`] is `NULL`, it is treated as if the mask has all bits
set to `1`.
## Valid Usage
-    If the [sample rate shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-sampleRateShading) feature is not enabled, [`sample_shading_enable`] **must**  be `VK_FALSE`
-    If the [alpha to one](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-alphaToOne) feature is not enabled, [`alpha_to_one_enable`] **must**  be `VK_FALSE`
-  [`min_sample_shading`] **must**  be in the range [0,1]
-    If the `[`nv_framebuffer_mixed_samples`]` extension is enabled, and if the subpass has any color attachments and [`rasterization_samples`] is greater than the number of color samples, then [`sample_shading_enable`] **must**  be `VK_FALSE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`PipelineCoverageModulationStateCreateInfoNV`], [`PipelineCoverageReductionStateCreateInfoNV`], [`PipelineCoverageToColorStateCreateInfoNV`], or [`PipelineSampleLocationsStateCreateInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be `0`
-  [`rasterization_samples`] **must**  be a valid [`SampleCountFlagBits`] value
-    If [`sample_mask`] is not `NULL`, [`sample_mask`] **must**  be a valid pointer to an array of <span class="katex"><span aria-hidden="true" class="katex-html"><span class="base"><span style="height:1.277216em;vertical-align:-0.345em;" class="strut"></span><span class="mopen">⌈</span><span class="mord"><span class="mord"><span class="mopen nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span style="height:0.9322159999999999em;" class="vlist"><span style="top:-2.6550000000000002em;"><span style="height:3em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight">3</span><span class="mord mtight">2</span></span></span></span><span style="top:-3.23em;"><span style="height:3em;" class="pstrut"></span><span class="frac-line" style="border-bottom-width:0.04em;"></span></span><span style="top:-3.446108em;"><span style="height:3em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight"><span class="mord mathit mtight">r</span><span class="mord mathit mtight">a</span><span class="mord mathit mtight">s</span><span class="mord mathit mtight">t</span><span class="mord mathit mtight">e</span><span class="mord mathit mtight">r</span><span class="mord mathit mtight">i</span><span class="mord mathit mtight">z</span><span class="mord mathit mtight">a</span><span class="mord mathit mtight">t</span><span class="mord mathit mtight">i</span><span class="mord mathit mtight">o</span><span class="mord mathit mtight">n</span><span class="mord mathit mtight">S</span><span class="mord mathit mtight">a</span><span class="mord mathit mtight">m</span><span class="mord mathit mtight">p</span><span class="mord mathit mtight">l</span><span class="mord mathit mtight">e</span><span class="mord mathit mtight">s</span></span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span style="height:0.345em;" class="vlist"><span></span></span></span></span></span><span class="mclose nulldelimiter"></span></span></span><span class="mclose">⌉</span></span></span></span>[`SampleMask`] values

# Related
- [`crate::vulkan1_0`]
- [`Bool32`]
- [`GraphicsPipelineCreateInfo`]
- [`PipelineMultisampleStateCreateFlags`]
- [`SampleCountFlagBits`]
- [`SampleMask`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        