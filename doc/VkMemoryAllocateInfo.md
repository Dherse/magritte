[VkMemoryAllocateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryAllocateInfo.html) - Structure containing parameters of a memory allocation

# C Specifications
The [`MemoryAllocateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkMemoryAllocateInfo {
    VkStructureType    sType;
    const void*        pNext;
    VkDeviceSize       allocationSize;
    uint32_t           memoryTypeIndex;
} VkMemoryAllocateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`allocation_size`] is the size of the allocation in bytes.
- [`memory_type_index`] is an index identifying a memory type from the `memoryTypes` array of the [`PhysicalDeviceMemoryProperties`] structure.

# Description
The internal data of an allocated device memory object  **must**  include a
reference to implementation-specific resources, referred to as the memory
object’s *payload*.
Applications  **can**  also import and export that internal data to and from
device memory objects to share data between Vulkan instances and other
compatible APIs.
A [`MemoryAllocateInfo`] structure defines a memory import operation if
its [`p_next`] chain includes one of the following structures:
- [`ImportMemoryWin32HandleInfoKHR`] with a non-zero `handleType` value
- [`ImportMemoryFdInfoKHR`] with a non-zero `handleType` value
- [`ImportMemoryHostPointerInfoEXT`] with a non-zero `handleType` value
- [`ImportAndroidHardwareBufferInfoANDROID`] with a non-`NULL``buffer` value
- [`ImportMemoryZirconHandleInfoFUCHSIA`] with a non-zero `handleType` value
- [`ImportMemoryBufferCollectionFUCHSIA`]
If the parameters define an import operation and the external handle type is
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT`,
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT`, or
`VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT`,
[`allocation_size`] is ignored.
The implementation  **must**  query the size of these allocations from the OS.Whether device memory objects constructed via a memory import operation hold
a reference to their payload depends on the properties of the handle type
used to perform the import, as defined below for each valid handle type.
Importing memory  **must**  not modify the content of the memory.
Implementations  **must**  ensure that importing memory does not enable the
importing Vulkan instance to access any memory or resources in other Vulkan
instances other than that corresponding to the memory object imported.
Implementations  **must**  also ensure accessing imported memory which has not
been initialized does not allow the importing Vulkan instance to obtain data
from the exporting Vulkan instance or vice-versa.Importing memory  **must**  not increase overall heap usage within a system.
However, it  **must**  affect the following per-process values:
- [`PhysicalDeviceMaintenance3Properties`]`::maxMemoryAllocationCount`
- [`PhysicalDeviceMemoryBudgetPropertiesEXT::heap_usage`]
When performing a memory import operation, it is the responsibility of the
application to ensure the external handles and their associated payloads
meet all valid usage requirements.
However, implementations  **must**  perform sufficient validation of external
handles and payloads to ensure that the operation results in a valid memory
object which will not cause program termination, device loss, queue stalls,
or corruption of other resources when used as allowed according to its
allocation parameters.
If the external handle provided does not meet these requirements, the
implementation  **must**  fail the memory import operation with the error code
`VK_ERROR_INVALID_EXTERNAL_HANDLE`.
## Valid Usage
-    The parameters  **must**  not define more than one [import operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-import-operation)
-    If the parameters define an import operation from an [`BufferCollectionFUCHSIA`], and [`MemoryDedicatedAllocateInfo::buffer`] is present and non-NULL, [`ImportMemoryBufferCollectionFUCHSIA::collection`] and [`ImportMemoryBufferCollectionFUCHSIA::index`] must match [`BufferCollectionBufferCreateInfoFUCHSIA::collection`] and [`BufferCollectionBufferCreateInfoFUCHSIA::index`], respectively, of the [`BufferCollectionBufferCreateInfoFUCHSIA`] structure used to create the [`MemoryDedicatedAllocateInfo::buffer`]
-    If the parameters define an import operation from an [`BufferCollectionFUCHSIA`], and [`MemoryDedicatedAllocateInfo::image`] is present and non-NULL, [`ImportMemoryBufferCollectionFUCHSIA::collection`] and [`ImportMemoryBufferCollectionFUCHSIA::index`] must match [`BufferCollectionImageCreateInfoFUCHSIA::collection`] and [`BufferCollectionImageCreateInfoFUCHSIA::index`], respectively, of the [`BufferCollectionImageCreateInfoFUCHSIA`] structure used to create the [`MemoryDedicatedAllocateInfo::image`]
-    If the parameters define an import operation from an [`BufferCollectionFUCHSIA`], [`allocation_size`] **must**  match [`MemoryRequirements::size`] value retrieved by [`get_image_memory_requirements`] or [`get_buffer_memory_requirements`] for image-based or buffer-based collections respectively
-    If the parameters define an import operation from an [`BufferCollectionFUCHSIA`], the [`p_next`] chain  **must**  include a [`MemoryDedicatedAllocateInfo`] structure with either its `image` or `buffer` field set to a value other than [`crate::Handle::null`].
-    If the parameters define an import operation from an [`BufferCollectionFUCHSIA`] and [`MemoryDedicatedAllocateInfo::image`] is not [`crate::Handle::null`], the `image` **must**  be created with a [`BufferCollectionImageCreateInfoFUCHSIA`] structure chained to its [`ImageCreateInfo`]::[`p_next`] pointer
-    If the parameters define an import operation from an [`BufferCollectionFUCHSIA`] and [`MemoryDedicatedAllocateInfo::buffer`] is not [`crate::Handle::null`], the `buffer` **must**  be created with a [`BufferCollectionBufferCreateInfoFUCHSIA`] structure chained to its [`BufferCreateInfo`]::[`p_next`] pointer
-    If the parameters define an import operation from an [`BufferCollectionFUCHSIA`], [`memory_type_index`] **must**  be from [`BufferCollectionPropertiesFUCHSIA`] as retrieved by [`get_buffer_collection_properties_fuchsia`].
-    If the [`p_next`] chain includes a [`ExportMemoryAllocateInfo`] structure, and any of the handle types specified in [`ExportMemoryAllocateInfo::handle_types`] require a dedicated allocation, as reported by [`get_physical_device_image_format_properties2`] in [`ExternalImageFormatProperties`]::`externalMemoryProperties.externalMemoryFeatures` or [`ExternalBufferProperties`]::`externalMemoryProperties.externalMemoryFeatures`, the [`p_next`] chain  **must**  include a [`MemoryDedicatedAllocateInfo`] or [`DedicatedAllocationMemoryAllocateInfoNV`] structure with either its `image` or `buffer` member set to a value other than [`crate::Handle::null`]
-    If the [`p_next`] chain includes a [`ExportMemoryAllocateInfo`] structure, it  **must**  not include a [`ExportMemoryAllocateInfoNV`] or [`ExportMemoryWin32HandleInfoNV`] structure
-    If the [`p_next`] chain includes a [`ImportMemoryWin32HandleInfoKHR`] structure, it  **must**  not include a [`ImportMemoryWin32HandleInfoNV`] structure
-    If the parameters define an import operation, the external handle specified was created by the Vulkan API, and the external handle type is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT`, then the values of [`allocation_size`] and [`memory_type_index`] **must**  match those specified when the payload being imported was created
-    If the parameters define an import operation and the external handle specified was created by the Vulkan API, the device mask specified by [`MemoryAllocateFlagsInfo`] **must**  match the mask specified when the payload being imported was allocated
-    If the parameters define an import operation and the external handle specified was created by the Vulkan API, the list of physical devices that comprise the logical device passed to [`allocate_memory`] **must**  match the list of physical devices that comprise the logical device on which the payload was originally allocated
-    If the parameters define an import operation and the external handle is an NT handle or a global share handle created outside of the Vulkan API, the value of [`memory_type_index`] **must**  be one of those returned by [`get_memory_win32_handle_properties_khr`]
-    If the parameters define an import operation, the external handle was created by the Vulkan API, and the external handle type is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT` or `VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT`, then the values of [`allocation_size`] and [`memory_type_index`] **must**  match those specified when the payload being imported was created
-    If the parameters define an import operation and the external handle type is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT`, [`allocation_size`] **must**  match the size specified when creating the Direct3D 12 heap from which the payload was extracted
-    If the parameters define an import operation and the external handle is a POSIX file descriptor created outside of the Vulkan API, the value of [`memory_type_index`] **must**  be one of those returned by [`get_memory_fd_properties_khr`]
-    If the protected memory feature is not enabled, the [`MemoryAllocateInfo`]::[`memory_type_index`] **must**  not indicate a memory type that reports `VK_MEMORY_PROPERTY_PROTECTED_BIT`
-    If the parameters define an import operation and the external handle is a host pointer, the value of [`memory_type_index`] **must**  be one of those returned by [`get_memory_host_pointer_properties_ext`]
-    If the parameters define an import operation and the external handle is a host pointer, [`allocation_size`] **must**  be an integer multiple of [`PhysicalDeviceExternalMemoryHostPropertiesEXT::min_imported_host_pointer_alignment`]
-    If the parameters define an import operation and the external handle is a host pointer, the [`p_next`] chain  **must**  not include a [`DedicatedAllocationMemoryAllocateInfoNV`] structure with either its `image` or `buffer` field set to a value other than [`crate::Handle::null`]
-    If the parameters define an import operation and the external handle is a host pointer, the [`p_next`] chain  **must**  not include a [`MemoryDedicatedAllocateInfo`] structure with either its `image` or `buffer` field set to a value other than [`crate::Handle::null`]
-    If the parameters define an import operation and the external handle type is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`, [`allocation_size`] **must**  be the size returned by [`get_android_hardware_buffer_properties_android`] for the Android hardware buffer
-    If the parameters define an import operation and the external handle type is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`, and the [`p_next`] chain does not include a [`MemoryDedicatedAllocateInfo`] structure or [`MemoryDedicatedAllocateInfo::image`] is [`crate::Handle::null`], the Android hardware buffer  **must**  have a `AHardwareBuffer_Desc`::`format` of `AHARDWAREBUFFER_FORMAT_BLOB` and a `AHardwareBuffer_Desc`::`usage` that includes `AHARDWAREBUFFER_USAGE_GPU_DATA_BUFFER`
-    If the parameters define an import operation and the external handle type is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID`, [`memory_type_index`] **must**  be one of those returned by [`get_android_hardware_buffer_properties_android`] for the Android hardware buffer
-    If the parameters do not define an import operation, and the [`p_next`] chain includes a [`ExportMemoryAllocateInfo`] structure with `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID` included in its `handleTypes` member, and the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`] structure with `image` not equal to [`crate::Handle::null`], then [`allocation_size`] **must**  be `0`, otherwise [`allocation_size`] **must**  be greater than `0`
-    If the parameters define an import operation, the external handle is an Android hardware buffer, and the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`] with `image` that is not [`crate::Handle::null`], the Android hardware buffer’s [`AHardwareBuffer`]`::usage` **must**  include at least one of `AHARDWAREBUFFER_USAGE_GPU_FRAMEBUFFER`, `AHARDWAREBUFFER_USAGE_GPU_SAMPLED_IMAGE` or `AHARDWAREBUFFER_USAGE_GPU_DATA_BUFFER`
-    If the parameters define an import operation, the external handle is an Android hardware buffer, and the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`] with `image` that is not [`crate::Handle::null`], the format of `image` **must**  be `VK_FORMAT_UNDEFINED` or the format returned by [`get_android_hardware_buffer_properties_android`] in [`AndroidHardwareBufferFormatPropertiesANDROID::format`] for the Android hardware buffer
-    If the parameters define an import operation, the external handle is an Android hardware buffer, and the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`] structure with `image` that is not [`crate::Handle::null`], the width, height, and array layer dimensions of `image` and the Android hardware buffer’s `AHardwareBuffer_Desc` **must**  be identical
-    If the parameters define an import operation, the external handle is an Android hardware buffer, and the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`] structure with `image` that is not [`crate::Handle::null`], and the Android hardware buffer’s [`AHardwareBuffer`]`::usage` includes `AHARDWAREBUFFER_USAGE_GPU_MIPMAP_COMPLETE`, the `image` **must**  have a complete mipmap chain
-    If the parameters define an import operation, the external handle is an Android hardware buffer, and the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`] structure with `image` that is not [`crate::Handle::null`], and the Android hardware buffer’s [`AHardwareBuffer`]`::usage` does not include `AHARDWAREBUFFER_USAGE_GPU_MIPMAP_COMPLETE`, the `image` **must**  have exactly one mipmap level
-    If the parameters define an import operation, the external handle is an Android hardware buffer, and the [`p_next`] chain includes a [`MemoryDedicatedAllocateInfo`] structure with `image` that is not [`crate::Handle::null`], each bit set in the usage of `image` **must**  be listed in [AHardwareBuffer Usage Equivalence](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-usage), and if there is a corresponding `AHARDWAREBUFFER_USAGE` bit listed that bit  **must**  be included in the Android hardware buffer’s `AHardwareBuffer_Desc`::`usage`
-    If [`MemoryOpaqueCaptureAddressAllocateInfo::opaque_capture_address`] is not zero, [`MemoryAllocateFlagsInfo::flags`] **must**  include `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`
-    If [`MemoryAllocateFlagsInfo::flags`] includes `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT`, the [bufferDeviceAddressCaptureReplay](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressCaptureReplay) feature  **must**  be enabled
-    If [`MemoryAllocateFlagsInfo::flags`] includes `VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT`, the [bufferDeviceAddress](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddress) feature  **must**  be enabled
-    If the [`p_next`] chain includes a [`ImportMemoryHostPointerInfoEXT`] structure, [`MemoryOpaqueCaptureAddressAllocateInfo::opaque_capture_address`] **must**  be zero
-    If the parameters define an import operation, [`MemoryOpaqueCaptureAddressAllocateInfo::opaque_capture_address`] **must**  be zero
-    If the parameters define an import operation and the external handle type is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`, the value of [`memory_type_index`] **must**  be an index identifying a memory type from the `memoryTypeBits` field of the [`MemoryZirconHandlePropertiesFUCHSIA`] structure populated by a call to [`get_memory_zircon_handle_properties_fuchsia`]
-    If the parameters define an import operation and the external handle type is `VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA`, the value of [`allocation_size`] **must**  be greater than `0` and  **must**  be less than or equal to the size of the VMO as determined by `zx_vmo_get_size`(`handle`) where `handle` is the VMO handle to the imported external memory

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`DedicatedAllocationMemoryAllocateInfoNV`], [`ExportMemoryAllocateInfo`], [`ExportMemoryAllocateInfoNV`], [`ExportMemoryWin32HandleInfoKHR`], [`ExportMemoryWin32HandleInfoNV`], [`ImportAndroidHardwareBufferInfoANDROID`], [`ImportMemoryBufferCollectionFUCHSIA`], [`ImportMemoryFdInfoKHR`], [`ImportMemoryHostPointerInfoEXT`], [`ImportMemoryWin32HandleInfoKHR`], [`ImportMemoryWin32HandleInfoNV`], [`ImportMemoryZirconHandleInfoFUCHSIA`], [`MemoryAllocateFlagsInfo`], [`MemoryDedicatedAllocateInfo`], [`MemoryOpaqueCaptureAddressAllocateInfo`], or [`MemoryPriorityAllocateInfoEXT`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique

# Related
- [`crate::vulkan1_0`]
- [`DeviceSize`]
- [`StructureType`]
- [`allocate_memory`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        