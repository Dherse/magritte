[`rectangles`] is either `NULL` or a pointer to an array of
[`RectLayerKHR`] structures.
The [`RectLayerKHR`] structure is the framebuffer coordinates, plus
layer, of a portion of a presentable image that has changed and  **must**  be
presented.
If non-`NULL`, each entry in [`rectangles`] is a rectangle of the
given image that has changed since the last image was presented to the
given swapchain.
The rectangles  **must**  be specified relative to
[`SurfaceCapabilitiesKHR`]::`currentTransform`, regardless of
the swapchain’s `preTransform`.
The presentation engine will apply the `preTransform` transformation
to the rectangles, along with any further transformation it applies to
the image content.