[VkSampleLocationEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleLocationEXT.html) - Structure specifying the coordinates of a sample location

# C Specifications
The [`SampleLocationEXT`] structure is defined as:
```c
// Provided by VK_EXT_sample_locations
typedef struct VkSampleLocationEXT {
    float    x;
    float    y;
} VkSampleLocationEXT;
```

# Members
- [`x`] is the horizontal coordinate of the sample’s location.
- [`y`] is the vertical coordinate of the sample’s location.

# Description
The domain space of the sample location coordinates has an upper-left origin
within the pixel in framebuffer space.The values specified in a [`SampleLocationEXT`] structure are always
clamped to the implementation-dependent sample location coordinate range
[`sampleLocationCoordinateRange`[0],`sampleLocationCoordinateRange`[1]]
that  **can**  be queried using
[`PhysicalDeviceSampleLocationsPropertiesEXT`].

# Related
- [`ext_sample_locations`]
- [`SampleLocationsInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        