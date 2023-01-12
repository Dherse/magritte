[`pp_enabled_layer_names`] is a pointer to an array of
[`enabled_layer_count`] null-terminated UTF-8 strings containing the
names of layers to enable for the created instance.
The layers are loaded in the order they are listed in this array, with
the first array element being the closest to the application, and the
last array element being the closest to the driver.
See the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-layers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-layers) section for further details.