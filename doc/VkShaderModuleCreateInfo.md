[VkShaderModuleCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModuleCreateInfo.html) - Structure specifying parameters of a newly created shader module

# C Specifications
The [`ShaderModuleCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkShaderModuleCreateInfo {
    VkStructureType              sType;
    const void*                  pNext;
    VkShaderModuleCreateFlags    flags;
    size_t                       codeSize;
    const uint32_t*              pCode;
} VkShaderModuleCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`code_size`] is the size, in bytes, of the code pointed to by [`code`].
- [`code`] is a pointer to code that is used to create the shader module. The type and format of the code is determined from the content of the memory addressed by [`code`].

# Description
## Valid Usage
-  [`code_size`] **must**  be greater than 0
-    If [`code`] is a pointer to SPIR-V code, [`code_size`] **must**  be a multiple of 4
-  [`code`] **must**  point to either valid SPIR-V code, formatted and packed as described by the [Khronos SPIR-V Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirv-spec) or valid GLSL code which  **must**  be written to the `GL_KHR_vulkan_glsl` extension specification
-    If [`code`] is a pointer to SPIR-V code, that code  **must**  adhere to the validation rules described by the [Validation Rules within a Module](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-module-validation) section of the [SPIR-V Environment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-capabilities) appendix
-    If [`code`] is a pointer to GLSL code, it  **must**  be valid GLSL code written to the `GL_KHR_vulkan_glsl` GLSL extension specification
-  [`code`] **must**  declare the `Shader` capability for SPIR-V code
-  [`code`] **must**  not declare any capability that is not supported by the API, as described by the [Capabilities](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-module-validation) section of the [SPIR-V Environment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-capabilities) appendix
-    If [`code`] declares any of the capabilities listed in the [SPIR-V Environment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-capabilities-table) appendix, one of the corresponding requirements  **must**  be satisfied
-  [`code`] **must**  not declare any SPIR-V extension that is not supported by the API, as described by the [Extension](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-extensions) section of the [SPIR-V Environment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-capabilities) appendix
-    If [`code`] declares any of the SPIR-V extensions listed in the [SPIR-V Environment](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#spirvenv-extensions-table) appendix, one of the corresponding requirements  **must**  be satisfied

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`ShaderModuleValidationCacheCreateInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be `0`
-  [`code`] **must**  be a valid pointer to an array of <span class="katex"><span class="katex-html" aria-hidden="true"><span class="base"><span style="height:1.2251079999999999em;vertical-align:-0.345em;" class="strut"></span><span class="mord"><span class="mopen nulldelimiter"></span><span class="mfrac"><span class="vlist-t vlist-t2"><span class="vlist-r"><span class="vlist" style="height:0.8801079999999999em;"><span style="top:-2.6550000000000002em;"><span style="height:3em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord mtight">4</span></span></span></span><span style="top:-3.23em;"><span class="pstrut" style="height:3em;"></span><span style="border-bottom-width:0.04em;" class="frac-line"></span></span><span style="top:-3.394em;"><span style="height:3em;" class="pstrut"></span><span class="sizing reset-size6 size3 mtight"><span class="mord mtight"><span class="mord text mtight"><span class="mord textrm mtight">codeSize</span></span></span></span></span></span><span class="vlist-s">​</span></span><span class="vlist-r"><span class="vlist" style="height:0.345em;"><span></span></span></span></span></span><span class="mclose nulldelimiter"></span></span></span></span></span>`uint32_t` values

# Related
- [`crate::vulkan1_0`]
- [`ShaderModuleCreateFlags`]
- [`StructureType`]
- [`create_shader_module`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        