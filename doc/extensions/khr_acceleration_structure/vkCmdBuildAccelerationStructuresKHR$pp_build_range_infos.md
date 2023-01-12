[`pp_build_range_infos`] is a pointer to an array of [`info_count`]
pointers to arrays of [`AccelerationStructureBuildRangeInfoKHR`]
structures.
Each [`pp_build_range_infos`][i] is a pointer to an array of
[`p_infos`][i].`geometryCount`[`AccelerationStructureBuildRangeInfoKHR`] structures defining
dynamic offsets to the addresses where geometry data is stored, as
defined by [`p_infos`][i].