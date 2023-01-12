[`FILL_RECTANGLE_NV`] specifies that polygons are
rendered using polygon rasterization rules, modified to consider a
sample within the primitive if the sample location is inside the
axis-aligned bounding box of the triangle after projection.
Note that the barycentric weights used in attribute interpolation  **can** 
extend outside the range [0,1] when these primitives are shaded.
Special treatment is given to a sample position on the boundary edge of
the bounding box.
In such a case, if two rectangles lie on either side of a common edge
(with identical endpoints) on which a sample position lies, then exactly
one of the triangles  **must**  produce a fragment that covers that sample
during rasterization.Polygons rendered in [`FILL_RECTANGLE_NV`] mode  **may**  be
clipped by the frustum or by user clip planes.
If clipping is applied, the triangle is culled rather than clipped.Area calculation and facingness are determined for
[`FILL_RECTANGLE_NV`] mode using the triangle’s vertices.