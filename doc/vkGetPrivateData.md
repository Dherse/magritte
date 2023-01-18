[vkGetPrivateData](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPrivateData.html) - Retrieve data associated with a Vulkan object

# C Specifications
To retrieve user defined data from a slot associated with a Vulkan object,
call:
```c
// Provided by VK_VERSION_1_3
void vkGetPrivateData(
    VkDevice                                    device,
    VkObjectType                                objectType,
    uint64_t                                    objectHandle,
    VkPrivateDataSlot                           privateDataSlot,
    uint64_t*                                   pData);
```
or the equivalent command
```c
// Provided by VK_EXT_private_data
void vkGetPrivateDataEXT(
    VkDevice                                    device,
    VkObjectType                                objectType,
    uint64_t                                    objectHandle,
    VkPrivateDataSlot                           privateDataSlot,
    uint64_t*                                   pData);
```

# Parameters
- [`device`] is the device that created the object
- [`object_type`] is a [`ObjectType`] specifying the type of object data is associated with.
- [`object_handle`] is a handle to the object data is associated with.
- [`private_data_slot`] is a handle to a [`PrivateDataSlot`] specifying location of private data pointer storage.
- [`p_data`] is a pointer to specify where user data is returned. `0` will be written in the absence of a previous call to [`set_private_data`] using the object specified by [`object_handle`].

# Description
## Valid Usage
-  [`object_type`] **must**  be `VK_OBJECT_TYPE_DEVICE`, or an object type whose parent is [`Device`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`object_type`] **must**  be a valid [`ObjectType`] value
-  [`private_data_slot`] **must**  be a valid [`PrivateDataSlot`] handle
-  [`p_data`] **must**  be a valid pointer to a `uint64_t` value
-  [`private_data_slot`] **must**  have been created, allocated, or retrieved from [`device`]

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
        