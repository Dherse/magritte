[`degenerate_triangles_rasterized`] is [`FALSE`] if the
implementation culls primitives generated from triangles that become
zero area after they are quantized to the fixed-point rasterization
pixel grid.
[`degenerate_triangles_rasterized`] is [`TRUE`] if these primitives
are not culled and the provoking vertex attributes and depth value are
used for the fragments.
The primitive area calculation is done on the primitive generated from
the clipped triangle if applicable.
Zero area primitives are backfacing and the application  **can**  enable
backface culling if desired.