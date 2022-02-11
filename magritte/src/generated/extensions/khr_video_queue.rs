#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_VIDEO_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_QUEUE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_video_queue");
///[VkQueryResultStatusKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryResultStatusKHR.html) - Specific status codes for operations
///# C Specifications
///Specific status codes that **can** be returned from a query are:
///```c
///// Provided by VK_KHR_video_queue
///typedef enum VkQueryResultStatusKHR {
///    VK_QUERY_RESULT_STATUS_ERROR_KHR = -1,
///    VK_QUERY_RESULT_STATUS_NOT_READY_KHR = 0,
///    VK_QUERY_RESULT_STATUS_COMPLETE_KHR = 1,
///} VkQueryResultStatusKHR;
///```
///# Description
/// - [`QUERY_RESULT_STATUS_NOT_READY`] specifies that the query
///result is not yet available.
/// - [`QUERY_RESULT_STATUS_ERROR`] specifies that operations did not
///complete successfully.
/// - [`QUERY_RESULT_STATUS_COMPLETE`] specifies that operations
///completed successfully and the query result is available.
///# Related
/// - [`VK_KHR_video_queue`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueryResultStatusKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueryResultStatusKHR(i32);
impl const Default for QueryResultStatusKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for QueryResultStatusKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("QueryResultStatusKHR")
            .field(match *self {
                Self::QUERY_RESULT_STATUS_ERROR => &"QUERY_RESULT_STATUS_ERROR",
                Self::QUERY_RESULT_STATUS_NOT_READY => &"QUERY_RESULT_STATUS_NOT_READY",
                Self::QUERY_RESULT_STATUS_COMPLETE => &"QUERY_RESULT_STATUS_COMPLETE",
                other => unreachable!("invalid value for `QueryResultStatusKHR`: {:?}", other),
            })
            .finish()
    }
}
impl QueryResultStatusKHR {
    ///[`QUERY_RESULT_STATUS_ERROR`] specifies that operations did not
    ///complete successfully.
    pub const QUERY_RESULT_STATUS_ERROR: Self = Self(-1);
    ///[`QUERY_RESULT_STATUS_NOT_READY`] specifies that the query
    ///result is not yet available.
    pub const QUERY_RESULT_STATUS_NOT_READY: Self = Self(0);
    ///[`QUERY_RESULT_STATUS_COMPLETE`] specifies that operations
    ///completed successfully and the query result is available.
    pub const QUERY_RESULT_STATUS_COMPLETE: Self = Self(1);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
