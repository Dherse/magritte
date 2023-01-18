[`supports_protected`] specifies whether a protected swapchain created
from [`PhysicalDeviceSurfaceInfo2KHR`]::`surface` for a
particular windowing system  **can**  be displayed on screen or not.
If [`supports_protected`] is [`TRUE`], then creation of swapchains
with the `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR` flag set  **must**  be
supported for `surface`.