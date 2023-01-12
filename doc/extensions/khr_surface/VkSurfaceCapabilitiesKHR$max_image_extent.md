[`max_image_extent`] contains the largest valid swapchain extent for the
surface on the specified device.
The `width` and `height` of the extent will each be greater than
or equal to the corresponding `width` and `height` of
[`min_image_extent`].
The `width` and `height` of the extent will each be greater than
or equal to the corresponding `width` and `height` of
[`current_extent`], unless [`current_extent`] has the special value
described above.