[`width`], [`height`] and [`layers`] define the dimensions of the
framebuffer.
If the render pass uses multiview, then [`layers`] **must**  be one and
each attachment requires a number of layers that is greater than the
maximum bit index set in the view mask in the subpasses in which it is
used.