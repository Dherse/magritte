[VkInstanceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkInstanceCreateInfo.html) - Structure specifying parameters of a newly created instance

# C Specifications
The [`InstanceCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkInstanceCreateInfo {
    VkStructureType             sType;
    const void*                 pNext;
    VkInstanceCreateFlags       flags;
    const VkApplicationInfo*    pApplicationInfo;
    uint32_t                    enabledLayerCount;
    const char* const*          ppEnabledLayerNames;
    uint32_t                    enabledExtensionCount;
    const char* const*          ppEnabledExtensionNames;
} VkInstanceCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is a bitmask of [`InstanceCreateFlagBits`] indicating the behavior of the instance.
- [`application_info`] is `NULL` or a pointer to a [`ApplicationInfo`] structure. If not `NULL`, this information helps implementations recognize behavior inherent to classes of applications. [`ApplicationInfo`] is defined in detail below.
- [`enabled_layer_count`] is the number of global layers to enable.
- [`pp_enabled_layer_names`] is a pointer to an array of [`enabled_layer_count`] null-terminated UTF-8 strings containing the names of layers to enable for the created instance. The layers are loaded in the order they are listed in this array, with the first array element being the closest to the application, and the last array element being the closest to the driver. See the [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-layers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-layers) section for further details.
- [`enabled_extension_count`] is the number of global extensions to enable.
- [`pp_enabled_extension_names`] is a pointer to an array of [`enabled_extension_count`] null-terminated UTF-8 strings containing the names of extensions to enable.

# Description
To capture events that occur while creating or destroying an instance, an
application can link a
[`DebugReportCallbackCreateInfoEXT`] structure
or a
[`DebugUtilsMessengerCreateInfoEXT`] structure
to the [`p_next`] element of the [`InstanceCreateInfo`] structure given
to [`create_instance`].
This callback is only valid for the duration of the [`create_instance`]
and the [`destroy_instance`] call.
Use
[`create_debug_report_callback_ext`]
or
[`create_debug_utils_messenger_ext`]
to create persistent callback objects.
## Valid Usage
-    If the [`p_next`] chain of [`InstanceCreateInfo`] includes a [`DebugReportCallbackCreateInfoEXT`] structure, the list of enabled extensions in [`pp_enabled_extension_names`] **must**  contain [`ext_debug_report`]
-    If the [`p_next`] chain of [`InstanceCreateInfo`] includes a [`DebugUtilsMessengerCreateInfoEXT`] structure, the list of enabled extensions in [`pp_enabled_extension_names`] **must**  contain [`ext_debug_utils`]
-    If [`flags`] has the `VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR` bit set, the list of enabled extensions in [`pp_enabled_extension_names`] **must**  contain `[`khr_portability_enumeration`]`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`DebugReportCallbackCreateInfoEXT`], [`DebugUtilsMessengerCreateInfoEXT`], [`ValidationFeaturesEXT`], or [`ValidationFlagsEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique, with the exception of structures of type [`DebugUtilsMessengerCreateInfoEXT`]
-  [`flags`] **must**  be a valid combination of [`InstanceCreateFlagBits`] values
-    If [`application_info`] is not `NULL`, [`application_info`] **must**  be a valid pointer to a valid [`ApplicationInfo`] structure
-    If [`enabled_layer_count`] is not `0`, [`pp_enabled_layer_names`] **must**  be a valid pointer to an array of [`enabled_layer_count`] null-terminated UTF-8 strings
-    If [`enabled_extension_count`] is not `0`, [`pp_enabled_extension_names`] **must**  be a valid pointer to an array of [`enabled_extension_count`] null-terminated UTF-8 strings

# Related
- [`crate::vulkan1_0`]
- [`ApplicationInfo`]
- [VkInstanceCreateFlags]()
- [`StructureType`]
- [`create_instance`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        