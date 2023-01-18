[VkCooperativeMatrixPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCooperativeMatrixPropertiesNV.html) - Structure specifying cooperative matrix properties

# C Specifications
Each [`CooperativeMatrixPropertiesNV`] structure describes a single
supported combination of types for a matrix multiply/add operation
(`OpCooperativeMatrixMulAddNV`).
The multiply  **can**  be described in terms of the following variables and types
(in SPIR-V pseudocode):
```c
    %A is of type OpTypeCooperativeMatrixNV %AType %scope %MSize %KSize
    %B is of type OpTypeCooperativeMatrixNV %BType %scope %KSize %NSize
    %C is of type OpTypeCooperativeMatrixNV %CType %scope %MSize %NSize
    %D is of type OpTypeCooperativeMatrixNV %DType %scope %MSize %NSize

    %D = %A * %B + %C // using OpCooperativeMatrixMulAddNV
```
A matrix multiply with these dimensions is known as an *MxNxK* matrix
multiply.The [`CooperativeMatrixPropertiesNV`] structure is defined as:
```c
// Provided by VK_NV_cooperative_matrix
typedef struct VkCooperativeMatrixPropertiesNV {
    VkStructureType      sType;
    void*                pNext;
    uint32_t             MSize;
    uint32_t             NSize;
    uint32_t             KSize;
    VkComponentTypeNV    AType;
    VkComponentTypeNV    BType;
    VkComponentTypeNV    CType;
    VkComponentTypeNV    DType;
    VkScopeNV            scope;
} VkCooperativeMatrixPropertiesNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`m_size`] is the number of rows in matrices A, C, and D.
- [`k_size`] is the number of columns in matrix A and rows in matrix B.
- [`n_size`] is the number of columns in matrices B, C, D.
- [`a_type`] is the component type of matrix A, of type [`ComponentTypeNV`].
- [`b_type`] is the component type of matrix B, of type [`ComponentTypeNV`].
- [`c_type`] is the component type of matrix C, of type [`ComponentTypeNV`].
- [`d_type`] is the component type of matrix D, of type [`ComponentTypeNV`].
- [`scope`] is the scope of all the matrix types, of type [`ScopeNV`].

# Description
If some types are preferred over other types (e.g. for performance), they
 **should**  appear earlier in the list enumerated by
[`get_physical_device_cooperative_matrix_properties_nv`].At least one entry in the list  **must**  have power of two values for all of
[`m_size`], [`k_size`], and [`n_size`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV`
-  [`p_next`] **must**  be `NULL`
-  [`a_type`] **must**  be a valid [`ComponentTypeNV`] value
-  [`b_type`] **must**  be a valid [`ComponentTypeNV`] value
-  [`c_type`] **must**  be a valid [`ComponentTypeNV`] value
-  [`d_type`] **must**  be a valid [`ComponentTypeNV`] value
-  [`scope`] **must**  be a valid [`ScopeNV`] value

# Related
- [`VK_NV_cooperative_matrix`]
- [`ComponentTypeNV`]
- [`ScopeNV`]
- [`StructureType`]
- [`get_physical_device_cooperative_matrix_properties_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        