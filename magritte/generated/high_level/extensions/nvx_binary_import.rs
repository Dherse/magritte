pub use crate::common::extensions::nvx_binary_import::{
    NVX_BINARY_IMPORT_EXTENSION_NAME, NVX_BINARY_IMPORT_SPEC_VERSION,
};
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::StructureType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkCuModuleCreateInfoNVX")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CuModuleCreateInfoNVX {
    #[doc(alias = "pData")]
    pub data: Vec<u8>,
}
impl CuModuleCreateInfoNVX {
    ///Get a reference to the `data` field.
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
    ///Get a mutable reference to the `data` field.
    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    ///Sets the `data` field.
    pub fn set_data(&mut self, data: Vec<u8>) -> &mut Self {
        self.data = data;
        self
    }
    ///Sets the `data` field in a builder way.
    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CuModuleCreateInfoNVX {
    type LowLevel = crate::native::extensions::nvx_binary_import::CuModuleCreateInfoNVX;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_data = self.data.len() as usize;
        crate::native::extensions::nvx_binary_import::CuModuleCreateInfoNVX {
            s_type: StructureType::CuModuleCreateInfoNvx,
            p_next: std::ptr::null(),
            data_size: len_data,
            data: self.data.as_ptr().cast(),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CuModuleCreateInfoNVX {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let data_len = value.data_size;
        let mut data = Vec::with_capacity(data_len as usize);
        let data_ptr = value.data as *const u8;
        for i in 0..data_len {
            data.push(crate::conv::FromLowLevel::from_low_level(
                context,
                data_ptr.add(i as usize).read(),
            ));
        }
        Self { data: data }
    }
}
#[doc(alias = "VkCuFunctionCreateInfoNVX")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CuFunctionCreateInfoNVX {
    pub module: CuModuleNVX,
    #[doc(alias = "pName")]
    pub name: String,
}
impl CuFunctionCreateInfoNVX {
    ///Get a reference to the `module` field.
    pub fn module(&self) -> &CuModuleNVX {
        &self.module
    }
    ///Get a reference to the `name` field.
    pub fn name(&self) -> &String {
        &self.name
    }
    ///Get a mutable reference to the `module` field.
    pub fn module_mut(&mut self) -> &mut CuModuleNVX {
        &mut self.module
    }
    ///Get a mutable reference to the `name` field.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    ///Sets the `module` field.
    pub fn set_module(&mut self, module: CuModuleNVX) -> &mut Self {
        self.module = module;
        self
    }
    ///Sets the `name` field.
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    ///Sets the `module` field in a builder way.
    pub fn with_module(mut self, module: CuModuleNVX) -> Self {
        self.module = module;
        self
    }
    ///Sets the `name` field in a builder way.
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CuFunctionCreateInfoNVX {
    type LowLevel = crate::native::extensions::nvx_binary_import::CuFunctionCreateInfoNVX;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nvx_binary_import::CuFunctionCreateInfoNVX {
            s_type: StructureType::CuFunctionCreateInfoNvx,
            p_next: std::ptr::null(),
            module: self.module.into_low_level(context, bump),
            name: self.name.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CuFunctionCreateInfoNVX {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            module: crate::conv::FromLowLevel::from_low_level(context, value.module),
            name: crate::conv::FromLowLevel::from_low_level(context, value.name),
        }
    }
}
#[doc(alias = "VkCuModuleNVX")]
#[derive(Debug)]
pub struct CuModuleNVX {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for CuModuleNVX {
    fn clone(&self) -> Self {
        self.context.clone_cu_module_nvx(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CuModuleNVX {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for CuModuleNVX {
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
impl Drop for CuModuleNVX {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_cu_module_nvx(&self.id);
        }
    }
}
impl PartialEq for CuModuleNVX {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl CuModuleNVX {
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
unsafe impl crate::conv::IntoLowLevel for CuModuleNVX {
    type LowLevel = crate::native::extensions::nvx_binary_import::CuModuleNVX;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context.cu_module_nvx().get(&self.id).expect("unknwon handle").handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CuModuleNVX {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context.cu_module_nvx().insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
#[doc(alias = "VkCuFunctionNVX")]
#[derive(Debug)]
pub struct CuFunctionNVX {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for CuFunctionNVX {
    fn clone(&self) -> Self {
        self.context.clone_cu_function_nvx(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for CuFunctionNVX {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for CuFunctionNVX {
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
impl Drop for CuFunctionNVX {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_cu_function_nvx(&self.id);
        }
    }
}
impl PartialEq for CuFunctionNVX {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl CuFunctionNVX {
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
unsafe impl crate::conv::IntoLowLevel for CuFunctionNVX {
    type LowLevel = crate::native::extensions::nvx_binary_import::CuFunctionNVX;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .cu_function_nvx()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CuFunctionNVX {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context.cu_function_nvx().insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
