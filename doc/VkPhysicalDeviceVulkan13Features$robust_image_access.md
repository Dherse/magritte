[`robust_image_access`]
indicates whether image accesses are tightly bounds-checked against the
dimensions of the image view.
[Invalid texels]() resulting from out of
bounds image loads will be replaced as described in
[Texel Replacement](), with either
(0,0,1) or (0,0,0) values inserted for missing G, B, or A
components based on the format.