[`reference_slots`] is `NULL` or a pointer to an array of
[`VideoReferenceSlotKHR`] structures that will be used when this
encoding operation is executing.
Each entry in [`reference_slots`] **must**  be one of the entries provided
in [`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within
the [`cmd_begin_video_coding_khr`] command that established the Vulkan
Video Encode Context for this command.