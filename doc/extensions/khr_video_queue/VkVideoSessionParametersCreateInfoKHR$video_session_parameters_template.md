[`video_session_parameters_template`] is [`crate::Handle::null`] or a valid
handle to a [`VideoSessionParametersKHR`] object.
If this parameter represents a valid handle, then the underlying Video
Session Parameters object will be used as a template for constructing
the new video session parameters object.
All of the template object’s current parameters will be inherited by the
new object in such a case.
Optionally, some of the template’s parameters can be updated or new
parameters added to the newly constructed object via the
extension-specific parameters.