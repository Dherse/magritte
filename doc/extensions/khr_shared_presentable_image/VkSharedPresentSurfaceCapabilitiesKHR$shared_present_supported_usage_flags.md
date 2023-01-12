[`shared_present_supported_usage_flags`] is a bitmask of
[`ImageUsageFlagBits`] representing the ways the application  **can** 
use the shared presentable image from a swapchain created with
[`PresentModeKHR`] set to
`VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
`VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` for the surface on
the specified device.
`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set
but implementations  **may**  support additional usages.