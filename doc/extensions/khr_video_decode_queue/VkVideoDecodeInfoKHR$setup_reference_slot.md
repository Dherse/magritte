[`setup_reference_slot`] is `NULL` or a pointer to a
[`VideoReferenceSlotKHR`] structure used for generating a DPB
reference slot and Picture Resource.
`pSetupReferenceSlot->slotIndex` specifies the slot index number to
use as a target for producing the DPB data.
`slotIndex` **must**  reference a valid entry as specified in
[`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within the
[`cmd_begin_video_coding_khr`] command that established the Vulkan Video
Decode Context for this command.