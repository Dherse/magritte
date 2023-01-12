[`non_coherent_atom_size`] is the size and
alignment in bytes that bounds concurrent access to
[host-mapped device memory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-device-hostaccess).
The value  **must**  be a power of two.
 * For all bitmasks of [`SampleCountFlagBits`], the sample count limits defined above represent the minimum supported sample counts for each image type. Individual images  **may**  support additional sample counts, which are queried using [`get_physical_device_image_format_properties`] as described in [Supported Sample Counts](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-supported-sample-counts).