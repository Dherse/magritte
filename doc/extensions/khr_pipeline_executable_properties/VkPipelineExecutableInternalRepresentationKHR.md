[VkPipelineExecutableInternalRepresentationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineExecutableInternalRepresentationKHR.html) - Structure describing the textual form of a pipeline executable internal representation

# C Specifications
The [`PipelineExecutableInternalRepresentationKHR`] structure is defined
as:
```c
// Provided by VK_KHR_pipeline_executable_properties
typedef struct VkPipelineExecutableInternalRepresentationKHR {
    VkStructureType    sType;
    void*              pNext;
    char               name[VK_MAX_DESCRIPTION_SIZE];
    char               description[VK_MAX_DESCRIPTION_SIZE];
    VkBool32           isText;
    size_t             dataSize;
    void*              pData;
} VkPipelineExecutableInternalRepresentationKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`name`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8 string which is a short human readable name for this internal representation.
- [`description`] is an array of [`MAX_DESCRIPTION_SIZE`]`char` containing a null-terminated UTF-8 string which is a human readable description for this internal representation.
- [`is_text`] specifies whether the returned data is text or opaque data. If [`is_text`] is [`TRUE`] then the data returned in [`data`] is text and is guaranteed to be a null-terminated UTF-8 string.
- [`data_size`] is an integer related to the size, in bytes, of the internal representation’s data, as described below.
- [`data`] is either `NULL` or a pointer to a block of data into which the implementation will write the internal representation.

# Description
If [`data`] is `NULL`, then the size, in bytes, of the internal
representation data is returned in [`data_size`].
Otherwise, [`data_size`] must be the size of the buffer, in bytes, pointed
to by [`data`] and on return [`data_size`] is overwritten with the
number of bytes of data actually written to [`data`] including any
trailing null character.
If [`data_size`] is less than the size, in bytes, of the internal
representation’s data, at most [`data_size`] bytes of data will be written
to [`data`], and `VK_INCOMPLETE` will be returned instead of
`VK_SUCCESS`, to indicate that not all the available representation was
returned.If [`is_text`] is [`TRUE`] and [`data`] is not `NULL` and
[`data_size`] is not zero, the last byte written to [`data`] will be a
null character.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_KHR_pipeline_executable_properties`]
- [`Bool32`]
- [`StructureType`]
- [`get_pipeline_executable_internal_representations_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        