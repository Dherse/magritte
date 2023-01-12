[`max_task_work_group_size`][3] is the maximum size of a local task
    workgroup.
    These three values represent the maximum local workgroup size in the X,
    Y, and Z dimensions, respectively.
    The `x`, `y`, and `z` sizes, as specified by the
    `LocalSize`
or `LocalSizeId`
    execution mode or by the object decorated by the `WorkgroupSize`
    decoration in shader modules,  **must**  be less than or equal to the
    corresponding limit.