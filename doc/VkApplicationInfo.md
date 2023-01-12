[VkApplicationInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkApplicationInfo.html) - Structure specifying application information

# C Specifications
The [`ApplicationInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkApplicationInfo {
    VkStructureType    sType;
    const void*        pNext;
    const char*        pApplicationName;
    uint32_t           applicationVersion;
    const char*        pEngineName;
    uint32_t           engineVersion;
    uint32_t           apiVersion;
} VkApplicationInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`application_name`] is `NULL` or is a pointer to a null-terminated UTF-8 string containing the name of the application.
- [`application_version`] is an unsigned integer variable containing the developer-supplied version number of the application.
- [`engine_name`] is `NULL` or is a pointer to a null-terminated UTF-8 string containing the name of the engine (if any) used to create the application.
- [`engine_version`] is an unsigned integer variable containing the developer-supplied version number of the engine used to create the application.
- [`api_version`] **must**  be the highest version of Vulkan that the application is designed to use, encoded as described in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#extendingvulkan-coreversions-versionnumbers). The patch version number specified in [`api_version`] is ignored when creating an instance object. Only the major and minor versions of the instance  **must**  match those requested in [`api_version`].

# Description
Vulkan 1.0 implementations were required to return
`VK_ERROR_INCOMPATIBLE_DRIVER` if [`api_version`] was larger than 1.0.
Implementations that support Vulkan 1.1 or later  **must**  not return
`VK_ERROR_INCOMPATIBLE_DRIVER` for any value of [`api_version`].As long as the instance supports at least Vulkan 1.1, an application  **can** 
use different versions of Vulkan with an instance than it does with a device
or physical device.Implicit layers  **must**  be disabled if they do not support a version at least
as high as [`api_version`].
See the [“Architecture of the Vulkan Loader
Interfaces”](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#LoaderInterfaceArchitecture) document for additional information.
## Valid Usage
-    If [`api_version`] is not `0`, then it  **must**  be greater than or equal to [`crate::Version::VULKAN1_0`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_APPLICATION_INFO`
-  [`p_next`] **must**  be `NULL`
-    If [`application_name`] is not `NULL`, [`application_name`] **must**  be a null-terminated UTF-8 string
-    If [`engine_name`] is not `NULL`, [`engine_name`] **must**  be a null-terminated UTF-8 string

# Related
- [`crate::vulkan1_0`]
- [`InstanceCreateInfo`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        