[vkGetMemoryZirconHandlePropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryZirconHandlePropertiesFUCHSIA.html) - Get a Zircon handle properties for an external memory object

# C Specifications
To obtain the memoryTypeIndex for the [`MemoryAllocateInfo`] structure,
call [`get_memory_zircon_handle_properties_fuchsia`]:
```c
// Provided by VK_FUCHSIA_external_memory
VkResult vkGetMemoryZirconHandlePropertiesFUCHSIA(
    VkDevice                                    device,
    VkExternalMemoryHandleTypeFlagBits          handleType,
    zx_handle_t                                 zirconHandle,
    VkMemoryZirconHandlePropertiesFUCHSIA*      pMemoryZirconHandleProperties);
```

# Parameters
- [`device`] is the [`Device`].
- [`handle_type`] is a [`ExternalMemoryHandleTypeFlagBits`] value specifying the type of [`zircon_handle`]
- [`zircon_handle`] is a [`zx_handle_t`] (Zircon) handle to the external resource.
- [`p_memory_zircon_handle_properties`] is a pointer to a [`MemoryZirconHandlePropertiesFUCHSIA`] structure in which the result will be stored.

# Description
## Valid Usage
-  [`handle_type`] **must**  be `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`
-  [`zircon_handle`] must reference a valid VMO

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`handle_type`] **must**  be a valid [`ExternalMemoryHandleTypeFlagBits`] value
-  [`p_memory_zircon_handle_properties`] **must**  be a valid pointer to a [`MemoryZirconHandlePropertiesFUCHSIA`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_INVALID_EXTERNAL_HANDLE`

# Related
- [`VK_FUCHSIA_external_memory`]
- [`Device`]
- [`ExternalMemoryHandleTypeFlagBits`]
- [`MemoryZirconHandlePropertiesFUCHSIA`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        