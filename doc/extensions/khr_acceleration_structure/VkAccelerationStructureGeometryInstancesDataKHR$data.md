[`data`] is either the address of an array of device or host addresses
referencing individual [`AccelerationStructureInstanceKHR`]
structures
or packed motion instance information as described in
[motion instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-motion-instances)
if [`array_of_pointers`] is [`TRUE`], or the address of an array of
[`AccelerationStructureInstanceKHR`]
or [`AccelerationStructureMotionInstanceNV`]
structures.
Addresses and [`AccelerationStructureInstanceKHR`] structures are
tightly packed.
[`AccelerationStructureMotionInstanceNV`] structures have a stride
of 160 bytes.