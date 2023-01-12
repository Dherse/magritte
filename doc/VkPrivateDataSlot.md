[VkPrivateDataSlot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPrivateDataSlot.html) - Opaque handle to a private data slot object

# C Specifications
Private data slots are represented by [`PrivateDataSlot`] handles:
```c
// Provided by VK_VERSION_1_3
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPrivateDataSlot)
```
or the equivalent
```c
// Provided by VK_EXT_private_data
typedef VkPrivateDataSlot VkPrivateDataSlotEXT;
```

# Related
- [`ext_private_data`]
- [`crate::vulkan1_3`]
- [`create_private_data_slot`]
- [`create_private_data_slot_ext`]
- [`destroy_private_data_slot`]
- [`destroy_private_data_slot_ext`]
- [`get_private_data`]
- [`get_private_data_ext`]
- [`set_private_data`]
- [`set_private_data_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        