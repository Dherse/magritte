[`setup_reference_slot`] is a pointer to a
[`VideoReferenceSlotKHR`] structure used for generating a
reconstructed reference slot and Picture Resource.
`pSetupReferenceSlot->slotIndex` specifies the slot index number to
use as a target for producing the Reconstructed (DPB) data.
[`setup_reference_slot`] **must**  be one of the entries provided in
[`VideoBeginCodingInfoKHR`] via the [`reference_slots`] within the
[`cmd_begin_video_coding_khr`] command that established the Vulkan Video
Encode Context for this command.