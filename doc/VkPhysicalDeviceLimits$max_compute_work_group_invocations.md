[`max_compute_work_group_invocations`] is the maximum total number of
    compute shader invocations in a single local workgroup.
    The product of the X, Y, and Z sizes, as specified by the `LocalSize`
or `LocalSizeId`
    execution mode in shader modules or by the object decorated by the
    `WorkgroupSize` decoration,  **must**  be less than or equal to this
    limit.