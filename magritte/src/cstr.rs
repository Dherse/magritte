#[doc(hidden)]
pub const fn validate_cstr_contents(bytes: &[u8]) {
    let mut i = 0;
    while i < bytes.len() {
        assert!(bytes[i] != b'\0', "Nul byte in literal [`CStr`]");
        i += 1;
    }
}

/// Converts a static [`&str`] into a static [`std::ffi::CStr`].
#[macro_export]
macro_rules! cstr {
    ($s:literal) => {{
        #[allow(clippy::string_lit_as_bytes)]
        $crate::cstr::validate_cstr_contents($s.as_bytes());

        #[allow(unused_unsafe)]
        unsafe {
            &*(concat!($s, "\0") as *const str as *const std::ffi::CStr)
        }
    }};
}

/// Converts a static [`&str`] into a static [`std::ffi::CStr`].
#[macro_export]
macro_rules! cstr_ptr {
    ($s:literal) => {{
        #[allow(clippy::string_lit_as_bytes)]
        $crate::cstr::validate_cstr_contents($s.as_bytes());

        concat!($s, "\0") as *const str as *const std::os::raw::c_char
    }};
}
