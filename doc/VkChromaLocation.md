[VkChromaLocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkChromaLocation.html) - Position of downsampled chroma samples

# C Specifications
The [`ChromaLocation`] enum defines the location of downsampled chroma
component samples relative to the luma samples, and is defined as:
```c
// Provided by VK_VERSION_1_1
typedef enum VkChromaLocation {
    VK_CHROMA_LOCATION_COSITED_EVEN = 0,
    VK_CHROMA_LOCATION_MIDPOINT = 1,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_CHROMA_LOCATION_COSITED_EVEN_KHR = VK_CHROMA_LOCATION_COSITED_EVEN,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_CHROMA_LOCATION_MIDPOINT_KHR = VK_CHROMA_LOCATION_MIDPOINT,
} VkChromaLocation;
```
or the equivalent
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
typedef VkChromaLocation VkChromaLocationKHR;
```

# Description
- [`VK_CHROMA_LOCATION`] specifies that downsampled chroma samples are aligned with luma samples with even coordinates.
- [`VK_CHROMA_LOCATION`] specifies that downsampled chroma samples are located half way between each even luma sample and the nearest higher odd luma sample.

# Related
- [`crate::vulkan1_1`]
- [`AndroidHardwareBufferFormatProperties2ANDROID`]
- [`AndroidHardwareBufferFormatPropertiesANDROID`]
- [`BufferCollectionPropertiesFUCHSIA`]
- [`SamplerYcbcrConversionCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        