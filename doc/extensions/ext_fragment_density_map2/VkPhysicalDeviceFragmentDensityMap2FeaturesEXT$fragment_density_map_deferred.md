[`fragment_density_map_deferred`]
specifies whether the implementation supports deferred reads of fragment
density map image views.
If this feature is not enabled,
`VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT` **must** 
not be included in [`ImageViewCreateInfo`]::`flags`.