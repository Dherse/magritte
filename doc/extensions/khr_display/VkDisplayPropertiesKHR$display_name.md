[`display_name`] is `NULL` or a pointer to a null-terminated UTF-8
string containing the name of the display.
Generally, this will be the name provided by the display’s EDID.
If `NULL`, no suitable name is available.
If not `NULL`, the string pointed to  **must**  remain accessible and
unmodified as long as [`display`] is valid.