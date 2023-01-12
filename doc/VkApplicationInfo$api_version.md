[`api_version`] **must**  be the highest version of Vulkan that the
application is designed to use, encoded as described in
[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers).
The patch version number specified in [`api_version`] is ignored when
creating an instance object.
Only the major and minor versions of the instance  **must**  match those
requested in [`api_version`].