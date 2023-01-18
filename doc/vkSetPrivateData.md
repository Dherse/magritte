[vkSetPrivateData](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkSetPrivateData.html) - Associate data with a Vulkan object

# C Specifications
To store user defined data in a slot associated with a Vulkan object, call:
```c
// Provided by VK_VERSION_1_3
VkResult vkSetPrivateData(
    VkDevice                                    device,
    VkObjectType                                objectType,
    uint64_t                                    objectHandle,
    VkPrivateDataSlot                           privateDataSlot,
    uint64_t                                    data);
```
or the equivalent command
```c
// Provided by VK_EXT_private_data
VkResult vkSetPrivateDataEXT(
    VkDevice                                    device,
    VkObjectType                                objectType,
    uint64_t                                    objectHandle,
    VkPrivateDataSlot                           privateDataSlot,
    uint64_t                                    data);
```

# Parameters
- [`device`] is the device that created the object.
- [`object_type`] is a [`ObjectType`] specifying the type of object to associate data with.
- [`object_handle`] is a handle to the object to associate data with.
- [`private_data_slot`] is a handle to a [`PrivateDataSlot`] specifying location of private data storage.
- [`data`] is user defined data to associate the object with. This data will be stored at [`private_data_slot`].

# Description
## Valid Usage
-  [`object_handle`] **must**  be [`device`] or a child of [`device`]
-  [`object_handle`] **must**  be a valid handle to an object of type [`object_type`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`object_type`] **must**  be a valid [`ObjectType`] value
-  [`private_data_slot`] **must**  be a valid [`PrivateDataSlot`] handle
-  [`private_data_slot`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_EXT_private_data`]
- [`crate::vulkan1_3`]
- [`Device`]
- [`ObjectType`]
- [`PrivateDataSlot`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        