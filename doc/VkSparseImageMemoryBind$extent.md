[`extent`] is the size in texels of the region within the image
subresource to bind.
The extent  **must**  be a multiple of the sparse image block dimensions,
except when binding sparse image blocks along the edge of an image
subresource it  **can**  instead be such that any coordinate of
[`offset`] +  [`extent`] equals the corresponding
dimensions of the image subresource.