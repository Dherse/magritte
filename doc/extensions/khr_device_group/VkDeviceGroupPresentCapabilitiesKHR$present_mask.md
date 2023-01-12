[`present_mask`] is an array of `VK_MAX_DEVICE_GROUP_SIZE``uint32_t` masks, where the mask at element i is non-zero if
physical device i has a presentation engine, and where bit j
is set in element i if physical device i **can**  present
swapchain images from physical device j.
If element i is non-zero, then bit i **must**  be set.