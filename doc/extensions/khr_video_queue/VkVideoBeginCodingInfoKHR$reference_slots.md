[`reference_slots`] is a pointer to an array of
[`VideoReferenceSlotKHR`] structures specifying reference slots,
used within the video command context between this
[`cmd_begin_video_coding_khr`] command and the
[`cmd_end_video_coding_khr`] commmand that follows.
Each reference slot provides a slot index and the
[`VideoPictureResourceKHR`] specifying the reference picture
resource bound to this slot index.
A slot index  **must**  not appear more than once in [`reference_slots`] in
a given command.