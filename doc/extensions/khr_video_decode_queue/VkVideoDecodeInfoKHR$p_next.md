[`p_next`] is `NULL` or a pointer to a structure extending this
structure.
All the codec specific structures related to each frame(picture
parameters, quantization matrix, etc.)  **must**  be chained here and pass to
decode session with the function call [`cmd_decode_video_khr`].