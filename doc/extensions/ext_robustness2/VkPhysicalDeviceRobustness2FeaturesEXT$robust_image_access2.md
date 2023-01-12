[`robust_image_access2`] indicates
whether image accesses are tightly bounds-checked against the dimensions
of the image view.
Out of bounds image loads will return zero values, with (0,0,1)
values [inserted for missing G, B, or A
components]() based on the format.