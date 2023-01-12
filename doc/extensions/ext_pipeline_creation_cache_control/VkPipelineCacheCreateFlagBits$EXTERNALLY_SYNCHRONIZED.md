[`EXTERNALLY_SYNCHRONIZED`] specifies
that all commands that modify the created [`PipelineCache`] will be
[externally synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-threadingbehavior).
When set, the implementation  **may**  skip any unnecessary processing needed
to support simultaneous modification from multiple threads where
allowed.