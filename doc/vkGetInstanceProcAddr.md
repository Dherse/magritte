[vkGetInstanceProcAddr](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html) - Return a function pointer for a command

# C Specifications
Function pointers for all Vulkan commands  **can**  be obtained with the command:
```c
// Provided by VK_VERSION_1_0
PFN_vkVoidFunction vkGetInstanceProcAddr(
    VkInstance                                  instance,
    const char*                                 pName);
```

# Parameters
- [`instance`] is the instance that the function pointer will be compatible with, or `NULL` for commands not dependent on any instance.
- [`p_name`] is the name of the command to obtain.

# Description
[`get_instance_proc_addr`] itself is obtained in a platform- and loader-
specific manner.
Typically, the loader library will export this command as a function symbol,
so applications  **can**  link against the loader library, or load it dynamically
and look up the symbol using platform-specific APIs.The table below defines the various use cases for
[`get_instance_proc_addr`] and expected return value (“fp” is “function
pointer”) for each case.
A valid returned function pointer (“fp”)  **must**  not be `NULL`.The returned function pointer is of type [`PFNVoidFunction`], and  **must** 
be cast to the type of the command being queried before use.
* "*" means any representable value for the parameter (including valid values, invalid values, and `NULL`).
*     The global commands are: [`enumerate_instance_version`],     [`enumerate_instance_extension_properties`],     [`enumerate_instance_layer_properties`], and [`create_instance`].     Dispatchable commands are all other commands which are not global.
* The returned function pointer  **must**  only be called with a dispatchable object (the first parameter) that is [`instance`] or a child of [`instance`], e.g. [`Instance`], [`PhysicalDevice`], [`Device`], [`Queue`], or [`CommandBuffer`].
* An “available device extension” is a device extension supported by any physical device enumerated by [`instance`].
* Starting with Vulkan 1.2, [`get_instance_proc_addr`] can resolve itself with a `NULL` instance pointer.

## Valid Usage (Implicit)
-    If [`instance`] is not `NULL`, [`instance`] **must**  be a valid [`Instance`] handle
-  [`p_name`] **must**  be a null-terminated UTF-8 string

# Related
- [`PFNVoidFunction`]
- [`crate::vulkan1_0`]
- [`Instance`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        