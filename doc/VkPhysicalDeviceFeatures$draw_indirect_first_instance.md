[`draw_indirect_first_instance`]
specifies whether indirect drawing calls support the `firstInstance`
parameter.
If this feature is not enabled, the `firstInstance` member of all
[`DrawIndirectCommand`] and [`DrawIndexedIndirectCommand`]
structures that are provided to the [`cmd_draw_indirect`] and
[`cmd_draw_indexed_indirect`] commands  **must**  be 0.