[`fragment_density_map_dynamic`]
specifies whether the implementation supports dynamic fragment density
map image views.
If this feature is not enabled,
`VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT` **must** 
not be included in [`ImageViewCreateInfo`]::`flags`.