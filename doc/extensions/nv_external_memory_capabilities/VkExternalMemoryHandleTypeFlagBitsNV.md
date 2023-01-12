[VkExternalMemoryHandleTypeFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html) - Bitmask specifying external memory handle types

# C Specifications
Possible values of [`ImportMemoryWin32HandleInfoNV::handle_type`],
specifying the type of an external memory handle, are:
```c
// Provided by VK_NV_external_memory_capabilities
typedef enum VkExternalMemoryHandleTypeFlagBitsNV {
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = 0x00000001,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = 0x00000002,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = 0x00000004,
    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = 0x00000008,
} VkExternalMemoryHandleTypeFlagBitsNV;
```

# Description
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_NV`] specifies a handle to memory returned by [`get_memory_win32_handle_nv`].
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_NV`] specifies a handle to memory returned by [`get_memory_win32_handle_nv`], or one duplicated from such a handle using `DuplicateHandle()`.
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_NV`] specifies a valid NT handle to memory returned by `IDXGIResource1::CreateSharedHandle`, or a handle duplicated from such a handle using `DuplicateHandle()`.
- [`VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_NV`] specifies a handle to memory returned by `IDXGIResource::GetSharedHandle()`.

# Related
- [`nv_external_memory_capabilities`]
- [VkExternalMemoryHandleTypeFlagsNV]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        