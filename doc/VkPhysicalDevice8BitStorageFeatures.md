[VkPhysicalDevice8BitStorageFeatures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice8BitStorageFeatures.html) - Structure describing features supported by VK_KHR_8bit_storage

# C Specifications
The [`PhysicalDevice8BitStorageFeatures`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkPhysicalDevice8BitStorageFeatures {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           storageBuffer8BitAccess;
    VkBool32           uniformAndStorageBuffer8BitAccess;
    VkBool32           storagePushConstant8;
} VkPhysicalDevice8BitStorageFeatures;
```
or the equivalent
```c
// Provided by VK_KHR_8bit_storage
typedef VkPhysicalDevice8BitStorageFeatures VkPhysicalDevice8BitStorageFeaturesKHR;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.

- [`storage_buffer8_bit_access`] indicates whether objects in the     `StorageBuffer`, `ShaderRecordBufferKHR`,     or `PhysicalStorageBuffer`     storage class with the `Block` decoration  **can**  have 8-bit integer     members.     If this feature is not enabled, 8-bit integer members  **must**  not be used     in such objects.     This also indicates whether shader modules  **can**  declare the     `StorageBuffer8BitAccess` capability.
- [`uniform_and_storage_buffer8_bit_access`] indicates whether objects in the `Uniform` storage class with the `Block` decoration  **can**  have 8-bit integer members. If this feature is not enabled, 8-bit integer members  **must**  not be used in such objects. This also indicates whether shader modules  **can**  declare the `UniformAndStorageBuffer8BitAccess` capability.
- [`storage_push_constant8`] indicates whether objects in the `PushConstant` storage class  **can**  have 8-bit integer members. If this feature is not enabled, 8-bit integer members  **must**  not be used in such objects. This also indicates whether shader modules  **can**  declare the `StoragePushConstant8` capability.
If the [`PhysicalDevice8BitStorageFeatures`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDevice8BitStorageFeatures`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES`

# Related
- [`khr_8bit_storage`]
- [`crate::vulkan1_2`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        