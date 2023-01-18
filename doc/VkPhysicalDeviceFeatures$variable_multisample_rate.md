[`variable_multisample_rate`]
specifies whether all pipelines that will be bound to a command buffer
during a [subpass which uses no attachments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-noattachments) **must**  have the same value for
[`PipelineMultisampleStateCreateInfo`]::`rasterizationSamples`.
If set to [`TRUE`], the implementation supports variable
multisample rates in a subpass which uses no attachments.
If set to [`FALSE`], then all pipelines bound in such a subpass
 **must**  have the same multisample rate.
This has no effect in situations where a subpass uses any attachments.