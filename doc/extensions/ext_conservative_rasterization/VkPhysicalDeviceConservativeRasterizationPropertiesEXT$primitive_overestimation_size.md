[`primitive_overestimation_size`]
is the size in pixels the generating primitive is increased at each of
its edges during conservative rasterization overestimation mode.
Even with a size of 0.0, conservative rasterization overestimation rules
still apply and if any part of the pixel rectangle is covered by the
generating primitive, fragments are generated for the entire pixel.
However implementations  **may**  make the pixel coverage area even more
conservative by increasing the size of the generating primitive.