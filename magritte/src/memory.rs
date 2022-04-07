use crate::vulkan1_0::{PhysicalDeviceMemoryProperties, MemoryPropertyFlags, MemoryRequirements};


/// Tries to find a memory type that has the flags you require and fits the memory requirements.
pub fn find_memory_type_index(
    requirements: &MemoryRequirements,
    properties: &PhysicalDeviceMemoryProperties,
    flags: MemoryPropertyFlags
) -> Option<u32> {
    properties.memory_types()
        .iter()
        .take(properties.memory_type_count() as usize)
        .enumerate()
        .find(|(index, type_)| {
            (1 << index) & requirements.memory_type_bits() != 0 && type_.property_flags().contains(flags)
        }).map(|(index, _)| index as u32)
}