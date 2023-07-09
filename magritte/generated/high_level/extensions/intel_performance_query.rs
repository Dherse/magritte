pub use crate::common::extensions::intel_performance_query::{
    PerformanceConfigurationTypeINTEL, PerformanceOverrideTypeINTEL, PerformanceParameterTypeINTEL,
    PerformanceValueTypeINTEL, QueryPoolSamplingModeINTEL, INTEL_PERFORMANCE_QUERY_EXTENSION_NAME,
    INTEL_PERFORMANCE_QUERY_SPEC_VERSION,
};
use crate::{
    context::{Container, Context, Error, ObjectId},
    vulkan1_0::StructureType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{any::Any, sync::Arc};
#[doc(alias = "VkQueryPoolCreateInfoINTEL")]
pub type QueryPoolCreateInfoINTEL = QueryPoolPerformanceQueryCreateInfoINTEL;
#[doc(alias = "VkPerformanceValueINTEL")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceValueINTEL {
    #[doc(alias = "type")]
    pub type_: PerformanceValueTypeINTEL,
    pub data: PerformanceValueDataINTEL,
}
impl PerformanceValueINTEL {
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> PerformanceValueTypeINTEL {
        self.type_
    }
    ///Get a reference to the `data` field.
    pub fn data(&self) -> &PerformanceValueDataINTEL {
        &self.data
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut PerformanceValueTypeINTEL {
        &mut self.type_
    }
    ///Get a mutable reference to the `data` field.
    pub fn data_mut(&mut self) -> &mut PerformanceValueDataINTEL {
        &mut self.data
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: PerformanceValueTypeINTEL) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `data` field.
    pub fn set_data(&mut self, data: PerformanceValueDataINTEL) -> &mut Self {
        self.data = data;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: PerformanceValueTypeINTEL) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `data` field in a builder way.
    pub fn with_data(mut self, data: PerformanceValueDataINTEL) -> Self {
        self.data = data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceValueINTEL {
    type LowLevel = crate::native::extensions::intel_performance_query::PerformanceValueINTEL;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::intel_performance_query::PerformanceValueINTEL {
            type_: self.type_.into_low_level(context, bump),
            data: self.data.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceValueINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            data: crate::conv::FromLowLevel::from_low_level(context, value.data),
        }
    }
}
#[doc(alias = "VkInitializePerformanceApiInfoINTEL")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InitializePerformanceApiInfoINTEL {
    #[doc(alias = "pUserData")]
    pub user_data: Box<dyn Any + Send + Sync + 'static>,
}
impl InitializePerformanceApiInfoINTEL {
    ///Get a reference to the `user_data` field.
    pub fn user_data(&self) -> &Box<dyn Any + Send + Sync + 'static> {
        &self.user_data
    }
    ///Get a mutable reference to the `user_data` field.
    pub fn user_data_mut(&mut self) -> &mut Box<dyn Any + Send + Sync + 'static> {
        &mut self.user_data
    }
    ///Sets the `user_data` field.
    pub fn set_user_data(&mut self, user_data: Box<dyn Any + Send + Sync + 'static>) -> &mut Self {
        self.user_data = user_data;
        self
    }
    ///Sets the `user_data` field in a builder way.
    pub fn with_user_data(mut self, user_data: Box<dyn Any + Send + Sync + 'static>) -> Self {
        self.user_data = user_data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for InitializePerformanceApiInfoINTEL {
    type LowLevel = crate::native::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::intel_performance_query::InitializePerformanceApiInfoINTEL {
            s_type: StructureType::InitializePerformanceApiInfoIntel,
            p_next: std::ptr::null(),
            user_data: self
                .user_data
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *mut _)
                .unwrap_or_else(std::ptr::null_mut),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for InitializePerformanceApiInfoINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            user_data: crate::conv::FromLowLevel::from_low_level(context, *value.user_data),
        }
    }
}
#[doc(alias = "VkQueryPoolPerformanceQueryCreateInfoINTEL")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
    #[doc(alias = "performanceCountersSampling")]
    pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
}
impl QueryPoolPerformanceQueryCreateInfoINTEL {
    ///Get a reference to the `performance_counters_sampling` field.
    pub fn performance_counters_sampling(&self) -> QueryPoolSamplingModeINTEL {
        self.performance_counters_sampling
    }
    ///Get a mutable reference to the `performance_counters_sampling` field.
    pub fn performance_counters_sampling_mut(&mut self) -> &mut QueryPoolSamplingModeINTEL {
        &mut self.performance_counters_sampling
    }
    ///Sets the `performance_counters_sampling` field.
    pub fn set_performance_counters_sampling(
        &mut self,
        performance_counters_sampling: QueryPoolSamplingModeINTEL,
    ) -> &mut Self {
        self.performance_counters_sampling = performance_counters_sampling;
        self
    }
    ///Sets the `performance_counters_sampling` field in a builder way.
    pub fn with_performance_counters_sampling(
        mut self,
        performance_counters_sampling: QueryPoolSamplingModeINTEL,
    ) -> Self {
        self.performance_counters_sampling = performance_counters_sampling;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for QueryPoolPerformanceQueryCreateInfoINTEL {
    type LowLevel = crate::native::extensions::intel_performance_query::QueryPoolPerformanceQueryCreateInfoINTEL;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::intel_performance_query::QueryPoolPerformanceQueryCreateInfoINTEL {
            s_type: StructureType::QueryPoolPerformanceQueryCreateInfoIntel,
            p_next: std::ptr::null(),
            performance_counters_sampling: self.performance_counters_sampling.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for QueryPoolPerformanceQueryCreateInfoINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            performance_counters_sampling: crate::conv::FromLowLevel::from_low_level(
                context,
                value.performance_counters_sampling,
            ),
        }
    }
}
#[doc(alias = "VkPerformanceMarkerInfoINTEL")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceMarkerInfoINTEL {
    pub marker: u64,
}
impl PerformanceMarkerInfoINTEL {
    ///Get a reference to the `marker` field.
    pub fn marker(&self) -> u64 {
        self.marker
    }
    ///Get a mutable reference to the `marker` field.
    pub fn marker_mut(&mut self) -> &mut u64 {
        &mut self.marker
    }
    ///Sets the `marker` field.
    pub fn set_marker(&mut self, marker: u64) -> &mut Self {
        self.marker = marker;
        self
    }
    ///Sets the `marker` field in a builder way.
    pub fn with_marker(mut self, marker: u64) -> Self {
        self.marker = marker;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceMarkerInfoINTEL {
    type LowLevel = crate::native::extensions::intel_performance_query::PerformanceMarkerInfoINTEL;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::intel_performance_query::PerformanceMarkerInfoINTEL {
            s_type: StructureType::PerformanceMarkerInfoIntel,
            p_next: std::ptr::null(),
            marker: self.marker.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceMarkerInfoINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            marker: crate::conv::FromLowLevel::from_low_level(context, value.marker),
        }
    }
}
#[doc(alias = "VkPerformanceStreamMarkerInfoINTEL")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceStreamMarkerInfoINTEL {
    pub marker: u32,
}
impl PerformanceStreamMarkerInfoINTEL {
    ///Get a reference to the `marker` field.
    pub fn marker(&self) -> u32 {
        self.marker
    }
    ///Get a mutable reference to the `marker` field.
    pub fn marker_mut(&mut self) -> &mut u32 {
        &mut self.marker
    }
    ///Sets the `marker` field.
    pub fn set_marker(&mut self, marker: u32) -> &mut Self {
        self.marker = marker;
        self
    }
    ///Sets the `marker` field in a builder way.
    pub fn with_marker(mut self, marker: u32) -> Self {
        self.marker = marker;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceStreamMarkerInfoINTEL {
    type LowLevel = crate::native::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::intel_performance_query::PerformanceStreamMarkerInfoINTEL {
            s_type: StructureType::PerformanceStreamMarkerInfoIntel,
            p_next: std::ptr::null(),
            marker: self.marker.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceStreamMarkerInfoINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            marker: crate::conv::FromLowLevel::from_low_level(context, value.marker),
        }
    }
}
#[doc(alias = "VkPerformanceOverrideInfoINTEL")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceOverrideInfoINTEL {
    #[doc(alias = "type")]
    pub type_: PerformanceOverrideTypeINTEL,
    pub enable: bool,
    pub parameter: u64,
}
impl PerformanceOverrideInfoINTEL {
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> PerformanceOverrideTypeINTEL {
        self.type_
    }
    ///Get a reference to the `enable` field.
    pub fn enable(&self) -> &bool {
        &self.enable
    }
    ///Get a reference to the `parameter` field.
    pub fn parameter(&self) -> u64 {
        self.parameter
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut PerformanceOverrideTypeINTEL {
        &mut self.type_
    }
    ///Get a mutable reference to the `enable` field.
    pub fn enable_mut(&mut self) -> &mut bool {
        &mut self.enable
    }
    ///Get a mutable reference to the `parameter` field.
    pub fn parameter_mut(&mut self) -> &mut u64 {
        &mut self.parameter
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: PerformanceOverrideTypeINTEL) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `enable` field.
    pub fn set_enable(&mut self, enable: bool) -> &mut Self {
        self.enable = enable;
        self
    }
    ///Sets the `parameter` field.
    pub fn set_parameter(&mut self, parameter: u64) -> &mut Self {
        self.parameter = parameter;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: PerformanceOverrideTypeINTEL) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `enable` field in a builder way.
    pub fn with_enable(mut self, enable: bool) -> Self {
        self.enable = enable;
        self
    }
    ///Sets the `parameter` field in a builder way.
    pub fn with_parameter(mut self, parameter: u64) -> Self {
        self.parameter = parameter;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceOverrideInfoINTEL {
    type LowLevel = crate::native::extensions::intel_performance_query::PerformanceOverrideInfoINTEL;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::intel_performance_query::PerformanceOverrideInfoINTEL {
            s_type: StructureType::PerformanceOverrideInfoIntel,
            p_next: std::ptr::null(),
            type_: self.type_.into_low_level(context, bump),
            enable: self.enable.into_low_level(context, bump),
            parameter: self.parameter.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceOverrideInfoINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            enable: crate::conv::FromLowLevel::from_low_level(context, value.enable),
            parameter: crate::conv::FromLowLevel::from_low_level(context, value.parameter),
        }
    }
}
#[doc(alias = "VkPerformanceConfigurationAcquireInfoINTEL")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceConfigurationAcquireInfoINTEL {
    #[doc(alias = "type")]
    pub type_: PerformanceConfigurationTypeINTEL,
}
impl PerformanceConfigurationAcquireInfoINTEL {
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> PerformanceConfigurationTypeINTEL {
        self.type_
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut PerformanceConfigurationTypeINTEL {
        &mut self.type_
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: PerformanceConfigurationTypeINTEL) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: PerformanceConfigurationTypeINTEL) -> Self {
        self.type_ = type_;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceConfigurationAcquireInfoINTEL {
    type LowLevel = crate::native::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::intel_performance_query::PerformanceConfigurationAcquireInfoINTEL {
            s_type: StructureType::PerformanceConfigurationAcquireInfoIntel,
            p_next: std::ptr::null(),
            type_: self.type_.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceConfigurationAcquireInfoINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
        }
    }
}
#[doc(alias = "VkPerformanceValueDataINTEL")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerformanceValueDataINTEL {
    Value32(u32),
    Value64(u64),
    ValueFloat(f32),
    ValueBool(bool),
    ValueString(String),
}
impl From<u32> for PerformanceValueDataINTEL {
    fn from(v: u32) -> Self {
        Self::Value32(v)
    }
}
impl From<u64> for PerformanceValueDataINTEL {
    fn from(v: u64) -> Self {
        Self::Value64(v)
    }
}
impl From<f32> for PerformanceValueDataINTEL {
    fn from(v: f32) -> Self {
        Self::ValueFloat(v)
    }
}
impl From<bool> for PerformanceValueDataINTEL {
    fn from(v: bool) -> Self {
        Self::ValueBool(v)
    }
}
impl From<String> for PerformanceValueDataINTEL {
    fn from(v: String) -> Self {
        Self::ValueString(v)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceValueDataINTEL {
    type LowLevel = (
        PerformanceValueTypeINTEL,
        crate::native::extensions::intel_performance_query::PerformanceValueDataINTEL,
    );
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::Value32(v) => crate::native::extensions::intel_performance_query::PerformanceValueDataINTEL {
                value32: (v.into_low_level(context, bump)),
            },
            Self::Value64(v) => crate::native::extensions::intel_performance_query::PerformanceValueDataINTEL {
                value64: (v.into_low_level(context, bump)),
            },
            Self::ValueFloat(v) => crate::native::extensions::intel_performance_query::PerformanceValueDataINTEL {
                value_float: (v.into_low_level(context, bump)),
            },
            Self::ValueBool(v) => crate::native::extensions::intel_performance_query::PerformanceValueDataINTEL {
                value_bool: (v.into_low_level(context, bump)),
            },
            Self::ValueString(v) => crate::native::extensions::intel_performance_query::PerformanceValueDataINTEL {
                value_string: (v.into_low_level(context, bump)),
            },
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceValueDataINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        (variant, value): <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        match variant {
            UINT32 => Self::Value32(value.value32.into_high_level(context)),
            UINT64 => Self::Value64(value.value64.into_high_level(context)),
            FLOAT => Self::ValueFloat(value.value_float.into_high_level(context)),
            BOOL => Self::ValueBool(value.value_bool.into_high_level(context)),
            STRING => Self::ValueString(value.value_string.into_high_level(context)),
        }
    }
}
#[doc(alias = "VkPerformanceConfigurationINTEL")]
#[derive(Debug)]
pub struct PerformanceConfigurationINTEL {
    context: Arc<Context>,
    id: ObjectId,
}
impl Clone for PerformanceConfigurationINTEL {
    fn clone(&self) -> Self {
        self.context.clone_performance_configuration_intel(self.id);
        Self {
            context: Arc::clone(&self.context),
            id: self.id,
        }
    }
}
#[cfg(feature = "serde")]
impl serde::Serialize for PerformanceConfigurationINTEL {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for PerformanceConfigurationINTEL {
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
impl Drop for PerformanceConfigurationINTEL {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.context.drop_performance_configuration_intel(&self.id);
        }
    }
}
impl PartialEq for PerformanceConfigurationINTEL {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl PerformanceConfigurationINTEL {
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
unsafe impl crate::conv::IntoLowLevel for PerformanceConfigurationINTEL {
    type LowLevel = crate::native::extensions::intel_performance_query::PerformanceConfigurationINTEL;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *context
            .performance_configuration_intel()
            .get(&self.id)
            .expect("unknwon handle")
            .handle()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceConfigurationINTEL {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let object_id = ObjectId::random();
        context
            .performance_configuration_intel()
            .insert(object_id, Container::new(value));
        Self {
            context: context.clone(),
            id: object_id,
        }
    }
}
