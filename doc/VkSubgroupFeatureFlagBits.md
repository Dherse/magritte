[VkSubgroupFeatureFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSubgroupFeatureFlagBits.html) - Bitmask describing what group operations are supported with subgroup scope

# C Specifications
Bits which  **can**  be set in
[`PhysicalDeviceSubgroupProperties::supported_operations`]
and
[`PhysicalDeviceVulkan11Properties::subgroup_supported_operations`]
to specify supported [group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) with
[subgroup scope](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup) are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkSubgroupFeatureFlagBits {
    VK_SUBGROUP_FEATURE_BASIC_BIT = 0x00000001,
    VK_SUBGROUP_FEATURE_VOTE_BIT = 0x00000002,
    VK_SUBGROUP_FEATURE_ARITHMETIC_BIT = 0x00000004,
    VK_SUBGROUP_FEATURE_BALLOT_BIT = 0x00000008,
    VK_SUBGROUP_FEATURE_SHUFFLE_BIT = 0x00000010,
    VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT = 0x00000020,
    VK_SUBGROUP_FEATURE_CLUSTERED_BIT = 0x00000040,
    VK_SUBGROUP_FEATURE_QUAD_BIT = 0x00000080,
  // Provided by VK_NV_shader_subgroup_partitioned
    VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV = 0x00000100,
} VkSubgroupFeatureFlagBits;
```

# Description
- [`VK_SUBGROUP_FEATURE_FLAG_BITS`] specifies the device will accept SPIR-V shader modules containing the `GroupNonUniform` capability.
- [`VK_SUBGROUP_FEATURE_FLAG_BITS`] specifies the device will accept SPIR-V shader modules containing the `GroupNonUniformVote` capability.
- [`VK_SUBGROUP_FEATURE_FLAG_BITS`] specifies the device will accept SPIR-V shader modules containing the `GroupNonUniformArithmetic` capability.
- [`VK_SUBGROUP_FEATURE_FLAG_BITS`] specifies the device will accept SPIR-V shader modules containing the `GroupNonUniformBallot` capability.
- [`VK_SUBGROUP_FEATURE_FLAG_BITS`] specifies the device will accept SPIR-V shader modules containing the `GroupNonUniformShuffle` capability.
- [`VK_SUBGROUP_FEATURE_FLAG_BITS`] specifies the device will accept SPIR-V shader modules containing the `GroupNonUniformShuffleRelative` capability.
- [`VK_SUBGROUP_FEATURE_FLAG_BITS`] specifies the device will accept SPIR-V shader modules containing the `GroupNonUniformClustered` capability.
- [`VK_SUBGROUP_FEATURE_FLAG_BITS`] specifies the device will accept SPIR-V shader modules containing the `GroupNonUniformQuad` capability.
- [`PARTITIONED_NV`] specifies the device will accept SPIR-V shader modules containing the `GroupNonUniformPartitionedNV` capability.

# Related
- [`crate::vulkan1_1`]
- [VkSubgroupFeatureFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        