pub use crate::common::extensions::khr_performance_query::{
    AcquireProfilingLockFlagBitsKHR, AcquireProfilingLockFlagsKHR, PerformanceCounterDescriptionFlagBitsKHR,
    PerformanceCounterDescriptionFlagsKHR, PerformanceCounterScopeKHR, PerformanceCounterStorageKHR,
    PerformanceCounterUnitKHR, KHR_PERFORMANCE_QUERY_EXTENSION_NAME, KHR_PERFORMANCE_QUERY_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{StructureType, MAX_DESCRIPTION_SIZE},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevicePerformanceQueryFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
    #[doc(alias = "performanceCounterQueryPools")]
    pub performance_counter_query_pools: bool,
    #[doc(alias = "performanceCounterMultipleQueryPools")]
    pub performance_counter_multiple_query_pools: bool,
}
impl PhysicalDevicePerformanceQueryFeaturesKHR {
    ///Get a reference to the `performance_counter_query_pools` field.
    pub fn performance_counter_query_pools(&self) -> &bool {
        &self.performance_counter_query_pools
    }
    ///Get a reference to the `performance_counter_multiple_query_pools` field.
    pub fn performance_counter_multiple_query_pools(&self) -> &bool {
        &self.performance_counter_multiple_query_pools
    }
    ///Get a mutable reference to the `performance_counter_query_pools` field.
    pub fn performance_counter_query_pools_mut(&mut self) -> &mut bool {
        &mut self.performance_counter_query_pools
    }
    ///Get a mutable reference to the `performance_counter_multiple_query_pools` field.
    pub fn performance_counter_multiple_query_pools_mut(&mut self) -> &mut bool {
        &mut self.performance_counter_multiple_query_pools
    }
    ///Sets the `performance_counter_query_pools` field.
    pub fn set_performance_counter_query_pools(&mut self, performance_counter_query_pools: bool) -> &mut Self {
        self.performance_counter_query_pools = performance_counter_query_pools;
        self
    }
    ///Sets the `performance_counter_multiple_query_pools` field.
    pub fn set_performance_counter_multiple_query_pools(
        &mut self,
        performance_counter_multiple_query_pools: bool,
    ) -> &mut Self {
        self.performance_counter_multiple_query_pools = performance_counter_multiple_query_pools;
        self
    }
    ///Sets the `performance_counter_query_pools` field in a builder way.
    pub fn with_performance_counter_query_pools(mut self, performance_counter_query_pools: bool) -> Self {
        self.performance_counter_query_pools = performance_counter_query_pools;
        self
    }
    ///Sets the `performance_counter_multiple_query_pools` field in a builder way.
    pub fn with_performance_counter_multiple_query_pools(
        mut self,
        performance_counter_multiple_query_pools: bool,
    ) -> Self {
        self.performance_counter_multiple_query_pools = performance_counter_multiple_query_pools;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePerformanceQueryFeaturesKHR {
    type LowLevel = crate::native::extensions::khr_performance_query::PhysicalDevicePerformanceQueryFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_performance_query::PhysicalDevicePerformanceQueryFeaturesKHR {
            s_type: StructureType::PhysicalDevicePerformanceQueryFeaturesKhr,
            p_next: std::ptr::null_mut(),
            performance_counter_query_pools: self.performance_counter_query_pools.into_low_level(context, bump),
            performance_counter_multiple_query_pools: self
                .performance_counter_multiple_query_pools
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePerformanceQueryFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            performance_counter_query_pools: crate::conv::FromLowLevel::from_low_level(
                context,
                value.performance_counter_query_pools,
            ),
            performance_counter_multiple_query_pools: crate::conv::FromLowLevel::from_low_level(
                context,
                value.performance_counter_multiple_query_pools,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDevicePerformanceQueryPropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
    #[doc(alias = "allowCommandBufferQueryCopies")]
    pub allow_command_buffer_query_copies: bool,
}
impl PhysicalDevicePerformanceQueryPropertiesKHR {
    ///Get a reference to the `allow_command_buffer_query_copies` field.
    pub fn allow_command_buffer_query_copies(&self) -> &bool {
        &self.allow_command_buffer_query_copies
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePerformanceQueryPropertiesKHR {
    type LowLevel = crate::native::extensions::khr_performance_query::PhysicalDevicePerformanceQueryPropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_performance_query::PhysicalDevicePerformanceQueryPropertiesKHR {
            s_type: StructureType::PhysicalDevicePerformanceQueryPropertiesKhr,
            p_next: std::ptr::null_mut(),
            allow_command_buffer_query_copies: self.allow_command_buffer_query_copies.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePerformanceQueryPropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            allow_command_buffer_query_copies: crate::conv::FromLowLevel::from_low_level(
                context,
                value.allow_command_buffer_query_copies,
            ),
        }
    }
}
#[doc(alias = "VkPerformanceCounterKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceCounterKHR {
    pub unit: PerformanceCounterUnitKHR,
    pub scope: PerformanceCounterScopeKHR,
    pub storage: PerformanceCounterStorageKHR,
    pub uuid: uuid::Uuid,
}
impl PerformanceCounterKHR {
    ///Get a reference to the `unit` field.
    pub fn unit(&self) -> PerformanceCounterUnitKHR {
        self.unit
    }
    ///Get a reference to the `scope` field.
    pub fn scope(&self) -> PerformanceCounterScopeKHR {
        self.scope
    }
    ///Get a reference to the `storage` field.
    pub fn storage(&self) -> PerformanceCounterStorageKHR {
        self.storage
    }
    ///Get a reference to the `uuid` field.
    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceCounterKHR {
    type LowLevel = crate::native::extensions::khr_performance_query::PerformanceCounterKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_performance_query::PerformanceCounterKHR {
            s_type: StructureType::PerformanceCounterKhr,
            p_next: std::ptr::null_mut(),
            unit: self.unit.into_low_level(context, bump),
            scope: self.scope.into_low_level(context, bump),
            storage: self.storage.into_low_level(context, bump),
            uuid: self.uuid.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceCounterKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            unit: crate::conv::FromLowLevel::from_low_level(context, value.unit),
            scope: crate::conv::FromLowLevel::from_low_level(context, value.scope),
            storage: crate::conv::FromLowLevel::from_low_level(context, value.storage),
            uuid: crate::conv::FromLowLevel::from_low_level(context, value.uuid),
        }
    }
}
#[doc(alias = "VkPerformanceCounterDescriptionKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceCounterDescriptionKHR {
    pub flags: PerformanceCounterDescriptionFlagsKHR,
    pub name: String,
    pub category: String,
    pub description: String,
}
impl PerformanceCounterDescriptionKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PerformanceCounterDescriptionFlagsKHR {
        self.flags
    }
    ///Get a reference to the `name` field.
    pub fn name(&self) -> &String {
        &self.name
    }
    ///Get a reference to the `category` field.
    pub fn category(&self) -> &String {
        &self.category
    }
    ///Get a reference to the `description` field.
    pub fn description(&self) -> &String {
        &self.description
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceCounterDescriptionKHR {
    type LowLevel = crate::native::extensions::khr_performance_query::PerformanceCounterDescriptionKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let name_bytes = self.name.as_bytes();
        debug_assert!(
            memchr::memchr(0, name_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            name_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        name[0..name_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            name_bytes.as_ptr() as *const std::ffi::c_char,
            name_bytes.len(),
        ));
        let category_bytes = self.category.as_bytes();
        debug_assert!(
            memchr::memchr(0, category_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            category_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut category: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        category[0..category_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            category_bytes.as_ptr() as *const std::ffi::c_char,
            category_bytes.len(),
        ));
        let description_bytes = self.description.as_bytes();
        debug_assert!(
            memchr::memchr(0, description_bytes).is_none(),
            "string array contains null characters"
        );
        debug_assert!(
            description_bytes.len() <= MAX_DESCRIPTION_SIZE as usize,
            "string is too long for the backing array"
        );
        let mut description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize] = [0; MAX_DESCRIPTION_SIZE as usize];
        description[0..description_bytes.len()].copy_from_slice(std::slice::from_raw_parts(
            description_bytes.as_ptr() as *const std::ffi::c_char,
            description_bytes.len(),
        ));
        crate::native::extensions::khr_performance_query::PerformanceCounterDescriptionKHR {
            s_type: StructureType::PerformanceCounterDescriptionKhr,
            p_next: std::ptr::null_mut(),
            flags: self.flags.into_low_level(context, bump),
            name: name,
            category: category,
            description: description,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceCounterDescriptionKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let name_cstr = std::ffi::CStr::from_ptr(value.name.as_ptr());
        let name = name_cstr.to_string_lossy().into_owned();
        let category_cstr = std::ffi::CStr::from_ptr(value.category.as_ptr());
        let category = category_cstr.to_string_lossy().into_owned();
        let description_cstr = std::ffi::CStr::from_ptr(value.description.as_ptr());
        let description = description_cstr.to_string_lossy().into_owned();
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            name: name,
            category: category,
            description: description,
        }
    }
}
#[doc(alias = "VkQueryPoolPerformanceCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPoolPerformanceCreateInfoKHR {
    #[doc(alias = "queueFamilyIndex")]
    pub queue_family_index: u32,
    #[doc(alias = "pCounterIndices")]
    pub counter_indices: SmallVec<[u32; 8]>,
}
impl QueryPoolPerformanceCreateInfoKHR {
    ///Get a reference to the `queue_family_index` field.
    pub fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }
    ///Get a reference to the `counter_indices` field.
    pub fn counter_indices(&self) -> &SmallVec<[u32; 8]> {
        &self.counter_indices
    }
    ///Get a mutable reference to the `queue_family_index` field.
    pub fn queue_family_index_mut(&mut self) -> &mut u32 {
        &mut self.queue_family_index
    }
    ///Get a mutable reference to the `counter_indices` field.
    pub fn counter_indices_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.counter_indices
    }
    ///Sets the `queue_family_index` field.
    pub fn set_queue_family_index(&mut self, queue_family_index: u32) -> &mut Self {
        self.queue_family_index = queue_family_index;
        self
    }
    ///Sets the `counter_indices` field.
    pub fn set_counter_indices(&mut self, counter_indices: SmallVec<[u32; 8]>) -> &mut Self {
        self.counter_indices = counter_indices;
        self
    }
    ///Sets the `queue_family_index` field in a builder way.
    pub fn with_queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.queue_family_index = queue_family_index;
        self
    }
    ///Sets the `counter_indices` field in a builder way.
    pub fn with_counter_indices(mut self, counter_indices: SmallVec<[u32; 8]>) -> Self {
        self.counter_indices = counter_indices;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for QueryPoolPerformanceCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_counter_indices = self.counter_indices.len() as u32;
        let counter_indices = bump
            .alloc_slice_fill_iter(self.counter_indices.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_performance_query::QueryPoolPerformanceCreateInfoKHR {
            s_type: StructureType::QueryPoolPerformanceCreateInfoKhr,
            p_next: std::ptr::null(),
            queue_family_index: self.queue_family_index.into_low_level(context, bump),
            counter_index_count: len_counter_indices,
            counter_indices: counter_indices,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for QueryPoolPerformanceCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let counter_indices_len = value.counter_index_count;
        let mut counter_indices = SmallVec::with_capacity(counter_indices_len as usize);
        for i in 0..counter_indices_len {
            counter_indices.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.counter_indices.add(i as usize).read(),
            ));
        }
        Self {
            queue_family_index: crate::conv::FromLowLevel::from_low_level(context, value.queue_family_index),
            counter_indices: counter_indices,
        }
    }
}
#[doc(alias = "VkAcquireProfilingLockInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AcquireProfilingLockInfoKHR {
    pub flags: AcquireProfilingLockFlagsKHR,
    pub timeout: u64,
}
impl AcquireProfilingLockInfoKHR {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> AcquireProfilingLockFlagsKHR {
        self.flags
    }
    ///Get a reference to the `timeout` field.
    pub fn timeout(&self) -> u64 {
        self.timeout
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut AcquireProfilingLockFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `timeout` field.
    pub fn timeout_mut(&mut self) -> &mut u64 {
        &mut self.timeout
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: AcquireProfilingLockFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `timeout` field.
    pub fn set_timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = timeout;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: AcquireProfilingLockFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `timeout` field in a builder way.
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AcquireProfilingLockInfoKHR {
    type LowLevel = crate::native::extensions::khr_performance_query::AcquireProfilingLockInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_performance_query::AcquireProfilingLockInfoKHR {
            s_type: StructureType::AcquireProfilingLockInfoKhr,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            timeout: self.timeout.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AcquireProfilingLockInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            timeout: crate::conv::FromLowLevel::from_low_level(context, value.timeout),
        }
    }
}
#[doc(alias = "VkPerformanceQuerySubmitInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceQuerySubmitInfoKHR {
    #[doc(alias = "counterPassIndex")]
    pub counter_pass_index: u32,
}
impl PerformanceQuerySubmitInfoKHR {
    ///Get a reference to the `counter_pass_index` field.
    pub fn counter_pass_index(&self) -> u32 {
        self.counter_pass_index
    }
    ///Get a mutable reference to the `counter_pass_index` field.
    pub fn counter_pass_index_mut(&mut self) -> &mut u32 {
        &mut self.counter_pass_index
    }
    ///Sets the `counter_pass_index` field.
    pub fn set_counter_pass_index(&mut self, counter_pass_index: u32) -> &mut Self {
        self.counter_pass_index = counter_pass_index;
        self
    }
    ///Sets the `counter_pass_index` field in a builder way.
    pub fn with_counter_pass_index(mut self, counter_pass_index: u32) -> Self {
        self.counter_pass_index = counter_pass_index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceQuerySubmitInfoKHR {
    type LowLevel = crate::native::extensions::khr_performance_query::PerformanceQuerySubmitInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_performance_query::PerformanceQuerySubmitInfoKHR {
            s_type: StructureType::PerformanceQuerySubmitInfoKhr,
            p_next: std::ptr::null(),
            counter_pass_index: self.counter_pass_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceQuerySubmitInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            counter_pass_index: crate::conv::FromLowLevel::from_low_level(context, value.counter_pass_index),
        }
    }
}
#[doc(alias = "VkPerformanceCounterResultKHR")]
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PerformanceCounterResultKHR {
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Float32(f32),
    Float64(f64),
}
impl From<i32> for PerformanceCounterResultKHR {
    fn from(v: i32) -> Self {
        Self::Int32(v)
    }
}
impl From<i64> for PerformanceCounterResultKHR {
    fn from(v: i64) -> Self {
        Self::Int64(v)
    }
}
impl From<u32> for PerformanceCounterResultKHR {
    fn from(v: u32) -> Self {
        Self::Uint32(v)
    }
}
impl From<u64> for PerformanceCounterResultKHR {
    fn from(v: u64) -> Self {
        Self::Uint64(v)
    }
}
impl From<f32> for PerformanceCounterResultKHR {
    fn from(v: f32) -> Self {
        Self::Float32(v)
    }
}
impl From<f64> for PerformanceCounterResultKHR {
    fn from(v: f64) -> Self {
        Self::Float64(v)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PerformanceCounterResultKHR {
    type LowLevel = crate::native::extensions::khr_performance_query::PerformanceCounterResultKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::Int32(v) => crate::native::extensions::khr_performance_query::PerformanceCounterResultKHR {
                int32: (v.into_low_level(context, bump)),
            },
            Self::Int64(v) => crate::native::extensions::khr_performance_query::PerformanceCounterResultKHR {
                int64: (v.into_low_level(context, bump)),
            },
            Self::Uint32(v) => crate::native::extensions::khr_performance_query::PerformanceCounterResultKHR {
                uint32: (v.into_low_level(context, bump)),
            },
            Self::Uint64(v) => crate::native::extensions::khr_performance_query::PerformanceCounterResultKHR {
                uint64: (v.into_low_level(context, bump)),
            },
            Self::Float32(v) => crate::native::extensions::khr_performance_query::PerformanceCounterResultKHR {
                float32: (v.into_low_level(context, bump)),
            },
            Self::Float64(v) => crate::native::extensions::khr_performance_query::PerformanceCounterResultKHR {
                float64: (v.into_low_level(context, bump)),
            },
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PerformanceCounterResultKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        _value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        unreachable!("cannot convert native union to high level if it does not have a selection");
    }
}
