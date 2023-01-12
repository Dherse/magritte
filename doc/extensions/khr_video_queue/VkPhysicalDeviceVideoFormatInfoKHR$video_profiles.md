[`video_profiles`] is a pointer to a [`VideoProfilesKHR`]
structure providing the video profile(s) of video session(s) that will
use the image.
For most use cases, the image is used by a single video session and a
single video profile is provided.
For a use case such as transcode, where a decode session output image
 **may**  be used as encode input for one or more encode sessions, multiple
video profiles representing the video sessions that will share the image
 **may**  be provided.