[`alpha_to_one`] specifies whether the
implementation is able to replace the alpha value of the fragment shader
color output in the [Multisample Coverage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fragops-covg) fragment
operation.
If this feature is not enabled, then the `alphaToOneEnable` member
of the [`PipelineMultisampleStateCreateInfo`] structure  **must**  be set
to `VK_FALSE`.
Otherwise setting `alphaToOneEnable` to `VK_TRUE` will enable
alpha-to-one behavior.