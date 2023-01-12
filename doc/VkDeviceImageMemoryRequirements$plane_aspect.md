[`plane_aspect`] is a [`ImageAspectFlagBits`] value specifying the
aspect corresponding to the image plane to query.
This parameter is ignored unless
[`create_info`]::`tiling` is
`VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT`, or
[`create_info`]::`flags` has `VK_IMAGE_CREATE_DISJOINT_BIT`
set.