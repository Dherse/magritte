[`dst_rect`] is a rectangular region within the visible region of the
swapchain’s display mode.
If [`DisplayPresentInfoKHR`] is not specified, this region will be
assumed to be the entire visible region of the swapchain’s mode.
If the specified rectangle is a subset of the display mode’s visible
region, content from display planes below the swapchain’s plane will be
visible outside the rectangle.
If there are no planes below the swapchain’s, the area outside the
specified rectangle will be black.
If portions of the specified rectangle are outside of the display’s
visible region, pixels mapping only to those portions of the rectangle
will be discarded.