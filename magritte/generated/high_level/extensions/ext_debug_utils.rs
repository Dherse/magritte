pub use crate::common::extensions::ext_debug_utils::{
    DebugUtilsMessageSeverityFlagBitsEXT, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagBitsEXT,
    DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessengerCallbackDataFlagsEXT, DebugUtilsMessengerCreateFlagsEXT,
    EXT_DEBUG_UTILS_EXTENSION_NAME, EXT_DEBUG_UTILS_SPEC_VERSION,
};
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::{ObjectType, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkDebugUtilsObjectNameInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugUtilsObjectNameInfoEXT {
    #[doc(alias = "objectType")]
    pub object_type: ObjectType,
    #[doc(alias = "objectHandle")]
    pub object_handle: u64,
    #[doc(alias = "pObjectName")]
    pub object_name: Option<String>,
}
impl DebugUtilsObjectNameInfoEXT {
    ///Get a reference to the `object_type` field.
    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }
    ///Get a reference to the `object_handle` field.
    pub fn object_handle(&self) -> u64 {
        self.object_handle
    }
    ///Get a reference to the `object_name` field.
    pub fn object_name(&self) -> &Option<String> {
        &self.object_name
    }
    ///Get a mutable reference to the `object_type` field.
    pub fn object_type_mut(&mut self) -> &mut ObjectType {
        &mut self.object_type
    }
    ///Get a mutable reference to the `object_handle` field.
    pub fn object_handle_mut(&mut self) -> &mut u64 {
        &mut self.object_handle
    }
    ///Get a mutable reference to the `object_name` field.
    pub fn object_name_mut(&mut self) -> &mut Option<String> {
        &mut self.object_name
    }
    ///Sets the `object_type` field.
    pub fn set_object_type(&mut self, object_type: ObjectType) -> &mut Self {
        self.object_type = object_type;
        self
    }
    ///Sets the `object_handle` field.
    pub fn set_object_handle(&mut self, object_handle: u64) -> &mut Self {
        self.object_handle = object_handle;
        self
    }
    ///Sets the `object_name` field.
    pub fn set_object_name(&mut self, object_name: Option<String>) -> &mut Self {
        self.object_name = object_name;
        self
    }
    ///Sets the `object_type` field in a builder way.
    pub fn with_object_type(mut self, object_type: ObjectType) -> Self {
        self.object_type = object_type;
        self
    }
    ///Sets the `object_handle` field in a builder way.
    pub fn with_object_handle(mut self, object_handle: u64) -> Self {
        self.object_handle = object_handle;
        self
    }
    ///Sets the `object_name` field in a builder way.
    pub fn with_object_name(mut self, object_name: Option<String>) -> Self {
        self.object_name = object_name;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DebugUtilsObjectNameInfoEXT {
    type LowLevel = crate::native::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT {
            s_type: StructureType::DebugUtilsObjectNameInfoExt,
            p_next: std::ptr::null(),
            object_type: self.object_type.into_low_level(context, bump),
            object_handle: self.object_handle.into_low_level(context, bump),
            object_name: self
                .object_name
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(std::ptr::null),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugUtilsObjectNameInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            object_type: crate::conv::FromLowLevel::from_low_level(context, value.object_type),
            object_handle: crate::conv::FromLowLevel::from_low_level(context, value.object_handle),
            object_name: crate::conv::FromLowLevel::from_low_level(context, value.object_name),
        }
    }
}
#[doc(alias = "VkDebugUtilsObjectTagInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugUtilsObjectTagInfoEXT {
    #[doc(alias = "objectType")]
    pub object_type: ObjectType,
    #[doc(alias = "objectHandle")]
    pub object_handle: u64,
    #[doc(alias = "tagName")]
    pub tag_name: u64,
    #[doc(alias = "pTag")]
    pub tag: Vec<u8>,
}
impl DebugUtilsObjectTagInfoEXT {
    ///Get a reference to the `object_type` field.
    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }
    ///Get a reference to the `object_handle` field.
    pub fn object_handle(&self) -> u64 {
        self.object_handle
    }
    ///Get a reference to the `tag_name` field.
    pub fn tag_name(&self) -> u64 {
        self.tag_name
    }
    ///Get a reference to the `tag` field.
    pub fn tag(&self) -> &Vec<u8> {
        &self.tag
    }
    ///Get a mutable reference to the `object_type` field.
    pub fn object_type_mut(&mut self) -> &mut ObjectType {
        &mut self.object_type
    }
    ///Get a mutable reference to the `object_handle` field.
    pub fn object_handle_mut(&mut self) -> &mut u64 {
        &mut self.object_handle
    }
    ///Get a mutable reference to the `tag_name` field.
    pub fn tag_name_mut(&mut self) -> &mut u64 {
        &mut self.tag_name
    }
    ///Get a mutable reference to the `tag` field.
    pub fn tag_mut(&mut self) -> &mut Vec<u8> {
        &mut self.tag
    }
    ///Sets the `object_type` field.
    pub fn set_object_type(&mut self, object_type: ObjectType) -> &mut Self {
        self.object_type = object_type;
        self
    }
    ///Sets the `object_handle` field.
    pub fn set_object_handle(&mut self, object_handle: u64) -> &mut Self {
        self.object_handle = object_handle;
        self
    }
    ///Sets the `tag_name` field.
    pub fn set_tag_name(&mut self, tag_name: u64) -> &mut Self {
        self.tag_name = tag_name;
        self
    }
    ///Sets the `tag` field.
    pub fn set_tag(&mut self, tag: Vec<u8>) -> &mut Self {
        self.tag = tag;
        self
    }
    ///Sets the `object_type` field in a builder way.
    pub fn with_object_type(mut self, object_type: ObjectType) -> Self {
        self.object_type = object_type;
        self
    }
    ///Sets the `object_handle` field in a builder way.
    pub fn with_object_handle(mut self, object_handle: u64) -> Self {
        self.object_handle = object_handle;
        self
    }
    ///Sets the `tag_name` field in a builder way.
    pub fn with_tag_name(mut self, tag_name: u64) -> Self {
        self.tag_name = tag_name;
        self
    }
    ///Sets the `tag` field in a builder way.
    pub fn with_tag(mut self, tag: Vec<u8>) -> Self {
        self.tag = tag;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DebugUtilsObjectTagInfoEXT {
    type LowLevel = crate::native::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_tag = self.tag.len() as usize;
        crate::native::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT {
            s_type: StructureType::DebugUtilsObjectTagInfoExt,
            p_next: std::ptr::null(),
            object_type: self.object_type.into_low_level(context, bump),
            object_handle: self.object_handle.into_low_level(context, bump),
            tag_name: self.tag_name.into_low_level(context, bump),
            tag_size: len_tag,
            tag: self.tag.as_ptr().cast(),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugUtilsObjectTagInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let tag_len = value.tag_size;
        let mut tag = Vec::with_capacity(tag_len as usize);
        let tag_ptr = value.tag as *const u8;
        for i in 0..tag_len {
            tag.push(crate::conv::FromLowLevel::from_low_level(
                context,
                tag_ptr.add(i as usize).read(),
            ));
        }
        Self {
            object_type: crate::conv::FromLowLevel::from_low_level(context, value.object_type),
            object_handle: crate::conv::FromLowLevel::from_low_level(context, value.object_handle),
            tag_name: crate::conv::FromLowLevel::from_low_level(context, value.tag_name),
            tag: tag,
        }
    }
}
#[doc(alias = "VkDebugUtilsLabelEXT")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugUtilsLabelEXT {
    #[doc(alias = "pLabelName")]
    pub label_name: String,
    pub color: [f32; 4 as usize],
}
impl DebugUtilsLabelEXT {
    ///Get a reference to the `label_name` field.
    pub fn label_name(&self) -> &String {
        &self.label_name
    }
    ///Get a reference to the `color` field.
    pub fn color(&self) -> [f32; 4 as usize] {
        self.color
    }
    ///Get a mutable reference to the `label_name` field.
    pub fn label_name_mut(&mut self) -> &mut String {
        &mut self.label_name
    }
    ///Get a mutable reference to the `color` field.
    pub fn color_mut(&mut self) -> &mut [f32; 4 as usize] {
        &mut self.color
    }
    ///Sets the `label_name` field.
    pub fn set_label_name(&mut self, label_name: String) -> &mut Self {
        self.label_name = label_name;
        self
    }
    ///Sets the `color` field.
    pub fn set_color(&mut self, color: [f32; 4 as usize]) -> &mut Self {
        self.color = color;
        self
    }
    ///Sets the `label_name` field in a builder way.
    pub fn with_label_name(mut self, label_name: String) -> Self {
        self.label_name = label_name;
        self
    }
    ///Sets the `color` field in a builder way.
    pub fn with_color(mut self, color: [f32; 4 as usize]) -> Self {
        self.color = color;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DebugUtilsLabelEXT {
    type LowLevel = crate::native::extensions::ext_debug_utils::DebugUtilsLabelEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_debug_utils::DebugUtilsLabelEXT {
            s_type: StructureType::DebugUtilsLabelExt,
            p_next: std::ptr::null(),
            label_name: self.label_name.into_low_level(context, bump),
            color: self.color.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugUtilsLabelEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            label_name: crate::conv::FromLowLevel::from_low_level(context, value.label_name),
            color: crate::conv::FromLowLevel::from_low_level(context, value.color),
        }
    }
}
#[doc(alias = "VkDebugUtilsMessengerCallbackDataEXT")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    #[doc(alias = "pMessageIdName")]
    pub message_id_name: Option<String>,
    #[doc(alias = "messageIdNumber")]
    pub message_id_number: i32,
    #[doc(alias = "pMessage")]
    pub message: String,
    #[doc(alias = "pQueueLabels")]
    pub queue_labels: SmallVec<[DebugUtilsLabelEXT; 8]>,
    #[doc(alias = "pCmdBufLabels")]
    pub cmd_buf_labels: SmallVec<[DebugUtilsLabelEXT; 8]>,
    #[doc(alias = "pObjects")]
    pub objects: SmallVec<[DebugUtilsObjectNameInfoEXT; 8]>,
}
impl DebugUtilsMessengerCallbackDataEXT {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> DebugUtilsMessengerCallbackDataFlagsEXT {
        self.flags
    }
    ///Get a reference to the `message_id_name` field.
    pub fn message_id_name(&self) -> &Option<String> {
        &self.message_id_name
    }
    ///Get a reference to the `message_id_number` field.
    pub fn message_id_number(&self) -> i32 {
        self.message_id_number
    }
    ///Get a reference to the `message` field.
    pub fn message(&self) -> &String {
        &self.message
    }
    ///Get a reference to the `queue_labels` field.
    pub fn queue_labels(&self) -> &SmallVec<[DebugUtilsLabelEXT; 8]> {
        &self.queue_labels
    }
    ///Get a reference to the `cmd_buf_labels` field.
    pub fn cmd_buf_labels(&self) -> &SmallVec<[DebugUtilsLabelEXT; 8]> {
        &self.cmd_buf_labels
    }
    ///Get a reference to the `objects` field.
    pub fn objects(&self) -> &SmallVec<[DebugUtilsObjectNameInfoEXT; 8]> {
        &self.objects
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut DebugUtilsMessengerCallbackDataFlagsEXT {
        &mut self.flags
    }
    ///Get a mutable reference to the `message_id_name` field.
    pub fn message_id_name_mut(&mut self) -> &mut Option<String> {
        &mut self.message_id_name
    }
    ///Get a mutable reference to the `message_id_number` field.
    pub fn message_id_number_mut(&mut self) -> &mut i32 {
        &mut self.message_id_number
    }
    ///Get a mutable reference to the `message` field.
    pub fn message_mut(&mut self) -> &mut String {
        &mut self.message
    }
    ///Get a mutable reference to the `queue_labels` field.
    pub fn queue_labels_mut(&mut self) -> &mut SmallVec<[DebugUtilsLabelEXT; 8]> {
        &mut self.queue_labels
    }
    ///Get a mutable reference to the `cmd_buf_labels` field.
    pub fn cmd_buf_labels_mut(&mut self) -> &mut SmallVec<[DebugUtilsLabelEXT; 8]> {
        &mut self.cmd_buf_labels
    }
    ///Get a mutable reference to the `objects` field.
    pub fn objects_mut(&mut self) -> &mut SmallVec<[DebugUtilsObjectNameInfoEXT; 8]> {
        &mut self.objects
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: DebugUtilsMessengerCallbackDataFlagsEXT) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `message_id_name` field.
    pub fn set_message_id_name(&mut self, message_id_name: Option<String>) -> &mut Self {
        self.message_id_name = message_id_name;
        self
    }
    ///Sets the `message_id_number` field.
    pub fn set_message_id_number(&mut self, message_id_number: i32) -> &mut Self {
        self.message_id_number = message_id_number;
        self
    }
    ///Sets the `message` field.
    pub fn set_message(&mut self, message: String) -> &mut Self {
        self.message = message;
        self
    }
    ///Sets the `queue_labels` field.
    pub fn set_queue_labels(&mut self, queue_labels: SmallVec<[DebugUtilsLabelEXT; 8]>) -> &mut Self {
        self.queue_labels = queue_labels;
        self
    }
    ///Sets the `cmd_buf_labels` field.
    pub fn set_cmd_buf_labels(&mut self, cmd_buf_labels: SmallVec<[DebugUtilsLabelEXT; 8]>) -> &mut Self {
        self.cmd_buf_labels = cmd_buf_labels;
        self
    }
    ///Sets the `objects` field.
    pub fn set_objects(&mut self, objects: SmallVec<[DebugUtilsObjectNameInfoEXT; 8]>) -> &mut Self {
        self.objects = objects;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: DebugUtilsMessengerCallbackDataFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `message_id_name` field in a builder way.
    pub fn with_message_id_name(mut self, message_id_name: Option<String>) -> Self {
        self.message_id_name = message_id_name;
        self
    }
    ///Sets the `message_id_number` field in a builder way.
    pub fn with_message_id_number(mut self, message_id_number: i32) -> Self {
        self.message_id_number = message_id_number;
        self
    }
    ///Sets the `message` field in a builder way.
    pub fn with_message(mut self, message: String) -> Self {
        self.message = message;
        self
    }
    ///Sets the `queue_labels` field in a builder way.
    pub fn with_queue_labels(mut self, queue_labels: SmallVec<[DebugUtilsLabelEXT; 8]>) -> Self {
        self.queue_labels = queue_labels;
        self
    }
    ///Sets the `cmd_buf_labels` field in a builder way.
    pub fn with_cmd_buf_labels(mut self, cmd_buf_labels: SmallVec<[DebugUtilsLabelEXT; 8]>) -> Self {
        self.cmd_buf_labels = cmd_buf_labels;
        self
    }
    ///Sets the `objects` field in a builder way.
    pub fn with_objects(mut self, objects: SmallVec<[DebugUtilsObjectNameInfoEXT; 8]>) -> Self {
        self.objects = objects;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DebugUtilsMessengerCallbackDataEXT {
    type LowLevel = crate::native::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_queue_labels = self.queue_labels.len() as u32;
        let queue_labels = bump
            .alloc_slice_fill_iter(self.queue_labels.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_cmd_buf_labels = self.cmd_buf_labels.len() as u32;
        let cmd_buf_labels = bump
            .alloc_slice_fill_iter(self.cmd_buf_labels.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        let len_objects = self.objects.len() as u32;
        let objects = bump
            .alloc_slice_fill_iter(self.objects.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::ext_debug_utils::DebugUtilsMessengerCallbackDataEXT {
            s_type: StructureType::DebugUtilsMessengerCallbackDataExt,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            message_id_name: self
                .message_id_name
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(std::ptr::null),
            message_id_number: self.message_id_number.into_low_level(context, bump),
            message: self.message.into_low_level(context, bump),
            queue_label_count: len_queue_labels,
            queue_labels: queue_labels,
            cmd_buf_label_count: len_cmd_buf_labels,
            cmd_buf_labels: cmd_buf_labels,
            object_count: len_objects,
            objects: objects,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugUtilsMessengerCallbackDataEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let queue_labels_len = value.queue_label_count;
        let mut queue_labels = SmallVec::with_capacity(queue_labels_len as usize);
        for i in 0..queue_labels_len {
            queue_labels.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.queue_labels.add(i as usize).read(),
            ));
        }
        let cmd_buf_labels_len = value.cmd_buf_label_count;
        let mut cmd_buf_labels = SmallVec::with_capacity(cmd_buf_labels_len as usize);
        for i in 0..cmd_buf_labels_len {
            cmd_buf_labels.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.cmd_buf_labels.add(i as usize).read(),
            ));
        }
        let objects_len = value.object_count;
        let mut objects = SmallVec::with_capacity(objects_len as usize);
        for i in 0..objects_len {
            objects.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.objects.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            message_id_name: crate::conv::FromLowLevel::from_low_level(context, value.message_id_name),
            message_id_number: crate::conv::FromLowLevel::from_low_level(context, value.message_id_number),
            message: crate::conv::FromLowLevel::from_low_level(context, value.message),
            queue_labels: queue_labels,
            cmd_buf_labels: cmd_buf_labels,
            objects: objects,
        }
    }
}
#[doc(alias = "VkDebugUtilsMessengerEXT")]
#[derive(Debug)]
pub struct DebugUtilsMessengerEXT {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for DebugUtilsMessengerEXT {
    fn clone(&self) -> Self {
        self.context.clone_debug_utils_messenger_ext(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for DebugUtilsMessengerEXT {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for DebugUtilsMessengerEXT {
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
impl Drop for DebugUtilsMessengerEXT {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_debug_utils_messenger_ext(&self.id);
        }
    }
}
impl PartialEq for DebugUtilsMessengerEXT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl DebugUtilsMessengerEXT {
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
unsafe impl crate::conv::IntoLowLevel for DebugUtilsMessengerEXT {
    type LowLevel = crate::native::extensions::ext_debug_utils::DebugUtilsMessengerEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .debug_utils_messenger_ext()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DebugUtilsMessengerEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .debug_utils_messenger_ext()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
