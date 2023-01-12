[`max_image_count`] is the maximum number of images the specified device
supports for a swapchain created for the surface, and will be either 0,
or greater than or equal to [`min_image_count`].
A value of 0 means that there is no limit on the number of images,
though there  **may**  be limits related to the total amount of memory used
by presentable images.