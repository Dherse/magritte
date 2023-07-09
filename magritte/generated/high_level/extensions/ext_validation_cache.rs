pub use crate::common::extensions::ext_validation_cache::{
    ValidationCacheCreateFlagsEXT, ValidationCacheHeaderVersionEXT, EXT_VALIDATION_CACHE_EXTENSION_NAME,
    EXT_VALIDATION_CACHE_SPEC_VERSION,
};
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::StructureType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkValidationCacheCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationCacheCreateInfoEXT {
    pub flags: ValidationCacheCreateFlagsEXT,
    #[doc(alias = "pInitialData")]
    pub initial_data: Vec<u8>,
}
impl ValidationCacheCreateInfoEXT {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> ValidationCacheCreateFlagsEXT {
        self.flags
    }
    ///Get a reference to the `initial_data` field.
    pub fn initial_data(&self) -> &Vec<u8> {
        &self.initial_data
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut ValidationCacheCreateFlagsEXT {
        &mut self.flags
    }
    ///Get a mutable reference to the `initial_data` field.
    pub fn initial_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.initial_data
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: ValidationCacheCreateFlagsEXT) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `initial_data` field.
    pub fn set_initial_data(&mut self, initial_data: Vec<u8>) -> &mut Self {
        self.initial_data = initial_data;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: ValidationCacheCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `initial_data` field in a builder way.
    pub fn with_initial_data(mut self, initial_data: Vec<u8>) -> Self {
        self.initial_data = initial_data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ValidationCacheCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_initial_data = self.initial_data.len() as usize;
        crate::native::extensions::ext_validation_cache::ValidationCacheCreateInfoEXT {
            s_type: StructureType::ValidationCacheCreateInfoExt,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            initial_data_size: len_initial_data,
            initial_data: self.initial_data.as_ptr().cast(),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ValidationCacheCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let initial_data_len = value.initial_data_size;
        let mut initial_data = Vec::with_capacity(initial_data_len as usize);
        let initial_data_ptr = value.initial_data as *const u8;
        for i in 0..initial_data_len {
            initial_data.push(crate::conv::FromLowLevel::from_low_level(
                context,
                initial_data_ptr.add(i as usize).read(),
            ));
        }
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            initial_data: initial_data,
        }
    }
}
#[doc(alias = "VkShaderModuleValidationCacheCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    #[doc(alias = "validationCache")]
    pub validation_cache: ValidationCacheEXT,
}
impl ShaderModuleValidationCacheCreateInfoEXT {
    ///Get a reference to the `validation_cache` field.
    pub fn validation_cache(&self) -> &ValidationCacheEXT {
        &self.validation_cache
    }
    ///Get a mutable reference to the `validation_cache` field.
    pub fn validation_cache_mut(&mut self) -> &mut ValidationCacheEXT {
        &mut self.validation_cache
    }
    ///Sets the `validation_cache` field.
    pub fn set_validation_cache(&mut self, validation_cache: ValidationCacheEXT) -> &mut Self {
        self.validation_cache = validation_cache;
        self
    }
    ///Sets the `validation_cache` field in a builder way.
    pub fn with_validation_cache(mut self, validation_cache: ValidationCacheEXT) -> Self {
        self.validation_cache = validation_cache;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ShaderModuleValidationCacheCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_validation_cache::ShaderModuleValidationCacheCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_validation_cache::ShaderModuleValidationCacheCreateInfoEXT {
            s_type: StructureType::ShaderModuleValidationCacheCreateInfoExt,
            p_next: std::ptr::null(),
            validation_cache: self.validation_cache.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ShaderModuleValidationCacheCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            validation_cache: crate::conv::FromLowLevel::from_low_level(context, value.validation_cache),
        }
    }
}
#[doc(alias = "VkValidationCacheEXT")]
#[derive(Debug)]
pub struct ValidationCacheEXT {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for ValidationCacheEXT {
    fn clone(&self) -> Self {
        self.context.clone_validation_cache_ext(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for ValidationCacheEXT {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for ValidationCacheEXT {
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
impl Drop for ValidationCacheEXT {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_validation_cache_ext(&self.id);
        }
    }
}
impl PartialEq for ValidationCacheEXT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl ValidationCacheEXT {
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
unsafe impl crate::conv::IntoLowLevel for ValidationCacheEXT {
    type LowLevel = crate::native::extensions::ext_validation_cache::ValidationCacheEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .validation_cache_ext()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ValidationCacheEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context.validation_cache_ext().insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
