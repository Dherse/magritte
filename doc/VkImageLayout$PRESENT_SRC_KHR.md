[`PRESENT_SRC_KHR`] **must**  only be used for presenting
a presentable image for display.
A swapchain’s image  **must**  be transitioned to this layout before calling
[`queue_present_khr`], and  **must**  be transitioned away from this layout
after calling [`acquire_next_image_khr`].