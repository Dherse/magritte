pub use crate::common::extensions::ext_debug_report::{
    DebugReportFlagBitsEXT, DebugReportFlagsEXT, EXT_DEBUG_REPORT_EXTENSION_NAME, EXT_DEBUG_REPORT_SPEC_VERSION,
};
use crate::context::{Container, Context, Error, ObjectId};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkDebugReportCallbackEXT")]
#[derive(Debug)]
pub struct DebugReportCallbackEXT {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for DebugReportCallbackEXT {
    fn clone(&self) -> Self {
        self.context.clone_debug_report_callback_ext(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DebugReportCallbackEXT {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for DebugReportCallbackEXT {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = ObjectId::deserialize(deserializer)?;
        crate::context::CONTEXT.with(|context| {
            let borrow = context.borrow();
            let context = borrow.as_ref().expect("Context not set.");
            Ok(Self {
                context: Arc::clone(context),
                id,
            })
        })
    }
}
impl Drop for DebugReportCallbackEXT {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_debug_report_callback_ext(&self.id);
        }
    }
}
impl PartialEq for DebugReportCallbackEXT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl DebugReportCallbackEXT {
    ///Creates a new instance of this handle from its core components.
    pub(crate) const unsafe fn new(context: Arc<Context>, id: ObjectId) -> Self {
        Self { context, id }
    }
    ///Gets the object id
    pub fn id(&self) -> &ObjectId {
        &self.id
    }
    ///Gets a reference to the context
    pub fn context(&self) -> &Context {
        &self.context
    }
    ///Gets a reference to the context wrapped in an [`Arc`]
    pub fn arc_context(&self) -> &Arc<Context> {
        &self.context
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DebugReportCallbackEXT {
    type LowLevel = crate::native::extensions::ext_debug_report::DebugReportCallbackEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .debug_report_callback_ext()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugReportCallbackEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .debug_report_callback_ext()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
