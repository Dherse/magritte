[`independent_blend`] specifies whether
the [`PipelineColorBlendAttachmentState`] settings are controlled
independently per-attachment.
If this feature is not enabled, the
[`PipelineColorBlendAttachmentState`] settings for all color
attachments  **must**  be identical.
Otherwise, a different [`PipelineColorBlendAttachmentState`] **can**  be
provided for each bound color attachment.