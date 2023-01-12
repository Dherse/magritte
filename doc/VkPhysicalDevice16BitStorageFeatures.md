[VkPhysicalDevice16BitStorageFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice16BitStorageFeatures.html) - Structure describing features supported by VK_KHR_16bit_storage

# C Specifications
The [`PhysicalDevice16BitStorageFeatures`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkPhysicalDevice16BitStorageFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           storageBuffer16BitAccess;
    VkBool32           uniformAndStorageBuffer16BitAccess;
    VkBool32           storagePushConstant16;
    VkBool32           storageInputOutput16;
} VkPhysicalDevice16BitStorageFeatures;
```
or the equivalent
```c
// Provided by VK_KHR_16bit_storage
typedef VkPhysicalDevice16BitStorageFeatures VkPhysicalDevice16BitStorageFeaturesKHR;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`storage_buffer16_bit_access`] specifies whether objects in the     `StorageBuffer`, `ShaderRecordBufferKHR`,     or `PhysicalStorageBuffer`     storage class with the `Block` decoration  **can**  have 16-bit integer     and 16-bit floating-point members.     If this feature is not enabled, 16-bit integer or 16-bit floating-point     members  **must**  not be used in such objects.     This also specifies whether shader modules  **can**  declare the     `StorageBuffer16BitAccess` capability.
- [`uniform_and_storage_buffer16_bit_access`] specifies whether objects in the `Uniform` storage class with the `Block` decoration  **can**  have 16-bit integer and 16-bit floating-point members. If this feature is not enabled, 16-bit integer or 16-bit floating-point members  **must**  not be used in such objects. This also specifies whether shader modules  **can**  declare the `UniformAndStorageBuffer16BitAccess` capability.
- [`storage_push_constant16`] specifies whether objects in the `PushConstant` storage class  **can**  have 16-bit integer and 16-bit floating-point members. If this feature is not enabled, 16-bit integer or floating-point members  **must**  not be used in such objects. This also specifies whether shader modules  **can**  declare the `StoragePushConstant16` capability.
- [`storage_input_output16`] specifies whether objects in the `Input` and `Output` storage classes  **can**  have 16-bit integer and 16-bit floating-point members. If this feature is not enabled, 16-bit integer or 16-bit floating-point members  **must**  not be used in such objects. This also specifies whether shader modules  **can**  declare the `StorageInputOutput16` capability.
If the [`PhysicalDevice16BitStorageFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDevice16BitStorageFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES`

# Related
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        