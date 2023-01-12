[`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`]
representing the ways the application  **can**  use the presentable images of
a swapchain created
with [`PresentModeKHR`] set to `VK_PRESENT_MODE_IMMEDIATE_KHR`,
`VK_PRESENT_MODE_MAILBOX_KHR`, `VK_PRESENT_MODE_FIFO_KHR` or
`VK_PRESENT_MODE_FIFO_RELAXED_KHR`
for the surface on the specified device.
`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set.
Implementations  **may**  support additional usages.