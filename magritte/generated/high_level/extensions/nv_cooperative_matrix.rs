pub use crate::common::extensions::nv_cooperative_matrix::{
    ComponentTypeNV, ScopeNV, NV_COOPERATIVE_MATRIX_EXTENSION_NAME, NV_COOPERATIVE_MATRIX_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{ShaderStageFlags, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
    #[doc(alias = "cooperativeMatrix")]
    pub cooperative_matrix: bool,
    #[doc(alias = "cooperativeMatrixRobustBufferAccess")]
    pub cooperative_matrix_robust_buffer_access: bool,
}
impl PhysicalDeviceCooperativeMatrixFeaturesNV {
    ///Get a reference to the `cooperative_matrix` field.
    pub fn cooperative_matrix(&self) -> &bool {
        &self.cooperative_matrix
    }
    ///Get a reference to the `cooperative_matrix_robust_buffer_access` field.
    pub fn cooperative_matrix_robust_buffer_access(&self) -> &bool {
        &self.cooperative_matrix_robust_buffer_access
    }
    ///Get a mutable reference to the `cooperative_matrix` field.
    pub fn cooperative_matrix_mut(&mut self) -> &mut bool {
        &mut self.cooperative_matrix
    }
    ///Get a mutable reference to the `cooperative_matrix_robust_buffer_access` field.
    pub fn cooperative_matrix_robust_buffer_access_mut(&mut self) -> &mut bool {
        &mut self.cooperative_matrix_robust_buffer_access
    }
    ///Sets the `cooperative_matrix` field.
    pub fn set_cooperative_matrix(&mut self, cooperative_matrix: bool) -> &mut Self {
        self.cooperative_matrix = cooperative_matrix;
        self
    }
    ///Sets the `cooperative_matrix_robust_buffer_access` field.
    pub fn set_cooperative_matrix_robust_buffer_access(
        &mut self,
        cooperative_matrix_robust_buffer_access: bool,
    ) -> &mut Self {
        self.cooperative_matrix_robust_buffer_access = cooperative_matrix_robust_buffer_access;
        self
    }
    ///Sets the `cooperative_matrix` field in a builder way.
    pub fn with_cooperative_matrix(mut self, cooperative_matrix: bool) -> Self {
        self.cooperative_matrix = cooperative_matrix;
        self
    }
    ///Sets the `cooperative_matrix_robust_buffer_access` field in a builder way.
    pub fn with_cooperative_matrix_robust_buffer_access(
        mut self,
        cooperative_matrix_robust_buffer_access: bool,
    ) -> Self {
        self.cooperative_matrix_robust_buffer_access = cooperative_matrix_robust_buffer_access;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceCooperativeMatrixFeaturesNV {
    type LowLevel = crate::native::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixFeaturesNV {
            s_type: StructureType::PhysicalDeviceCooperativeMatrixFeaturesNv,
            p_next: std::ptr::null_mut(),
            cooperative_matrix: self.cooperative_matrix.into_low_level(context, bump),
            cooperative_matrix_robust_buffer_access: self
                .cooperative_matrix_robust_buffer_access
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceCooperativeMatrixFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            cooperative_matrix: crate::conv::FromLowLevel::from_low_level(context, value.cooperative_matrix),
            cooperative_matrix_robust_buffer_access: crate::conv::FromLowLevel::from_low_level(
                context,
                value.cooperative_matrix_robust_buffer_access,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixPropertiesNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
    #[doc(alias = "cooperativeMatrixSupportedStages")]
    pub cooperative_matrix_supported_stages: ShaderStageFlags,
}
impl PhysicalDeviceCooperativeMatrixPropertiesNV {
    ///Get a reference to the `cooperative_matrix_supported_stages` field.
    pub fn cooperative_matrix_supported_stages(&self) -> ShaderStageFlags {
        self.cooperative_matrix_supported_stages
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceCooperativeMatrixPropertiesNV {
    type LowLevel = crate::native::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixPropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_cooperative_matrix::PhysicalDeviceCooperativeMatrixPropertiesNV {
            s_type: StructureType::PhysicalDeviceCooperativeMatrixPropertiesNv,
            p_next: std::ptr::null_mut(),
            cooperative_matrix_supported_stages: self.cooperative_matrix_supported_stages.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceCooperativeMatrixPropertiesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            cooperative_matrix_supported_stages: crate::conv::FromLowLevel::from_low_level(
                context,
                value.cooperative_matrix_supported_stages,
            ),
        }
    }
}
#[doc(alias = "VkCooperativeMatrixPropertiesNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CooperativeMatrixPropertiesNV {
    #[doc(alias = "MSize")]
    pub m_size: u32,
    #[doc(alias = "NSize")]
    pub n_size: u32,
    #[doc(alias = "KSize")]
    pub k_size: u32,
    #[doc(alias = "AType")]
    pub a_type: ComponentTypeNV,
    #[doc(alias = "BType")]
    pub b_type: ComponentTypeNV,
    #[doc(alias = "CType")]
    pub c_type: ComponentTypeNV,
    #[doc(alias = "DType")]
    pub d_type: ComponentTypeNV,
    pub scope: ScopeNV,
}
impl CooperativeMatrixPropertiesNV {
    ///Get a reference to the `m_size` field.
    pub fn m_size(&self) -> u32 {
        self.m_size
    }
    ///Get a reference to the `n_size` field.
    pub fn n_size(&self) -> u32 {
        self.n_size
    }
    ///Get a reference to the `k_size` field.
    pub fn k_size(&self) -> u32 {
        self.k_size
    }
    ///Get a reference to the `a_type` field.
    pub fn a_type(&self) -> ComponentTypeNV {
        self.a_type
    }
    ///Get a reference to the `b_type` field.
    pub fn b_type(&self) -> ComponentTypeNV {
        self.b_type
    }
    ///Get a reference to the `c_type` field.
    pub fn c_type(&self) -> ComponentTypeNV {
        self.c_type
    }
    ///Get a reference to the `d_type` field.
    pub fn d_type(&self) -> ComponentTypeNV {
        self.d_type
    }
    ///Get a reference to the `scope` field.
    pub fn scope(&self) -> ScopeNV {
        self.scope
    }
    ///Get a mutable reference to the `m_size` field.
    pub fn m_size_mut(&mut self) -> &mut u32 {
        &mut self.m_size
    }
    ///Get a mutable reference to the `n_size` field.
    pub fn n_size_mut(&mut self) -> &mut u32 {
        &mut self.n_size
    }
    ///Get a mutable reference to the `k_size` field.
    pub fn k_size_mut(&mut self) -> &mut u32 {
        &mut self.k_size
    }
    ///Get a mutable reference to the `a_type` field.
    pub fn a_type_mut(&mut self) -> &mut ComponentTypeNV {
        &mut self.a_type
    }
    ///Get a mutable reference to the `b_type` field.
    pub fn b_type_mut(&mut self) -> &mut ComponentTypeNV {
        &mut self.b_type
    }
    ///Get a mutable reference to the `c_type` field.
    pub fn c_type_mut(&mut self) -> &mut ComponentTypeNV {
        &mut self.c_type
    }
    ///Get a mutable reference to the `d_type` field.
    pub fn d_type_mut(&mut self) -> &mut ComponentTypeNV {
        &mut self.d_type
    }
    ///Get a mutable reference to the `scope` field.
    pub fn scope_mut(&mut self) -> &mut ScopeNV {
        &mut self.scope
    }
    ///Sets the `m_size` field.
    pub fn set_m_size(&mut self, m_size: u32) -> &mut Self {
        self.m_size = m_size;
        self
    }
    ///Sets the `n_size` field.
    pub fn set_n_size(&mut self, n_size: u32) -> &mut Self {
        self.n_size = n_size;
        self
    }
    ///Sets the `k_size` field.
    pub fn set_k_size(&mut self, k_size: u32) -> &mut Self {
        self.k_size = k_size;
        self
    }
    ///Sets the `a_type` field.
    pub fn set_a_type(&mut self, a_type: ComponentTypeNV) -> &mut Self {
        self.a_type = a_type;
        self
    }
    ///Sets the `b_type` field.
    pub fn set_b_type(&mut self, b_type: ComponentTypeNV) -> &mut Self {
        self.b_type = b_type;
        self
    }
    ///Sets the `c_type` field.
    pub fn set_c_type(&mut self, c_type: ComponentTypeNV) -> &mut Self {
        self.c_type = c_type;
        self
    }
    ///Sets the `d_type` field.
    pub fn set_d_type(&mut self, d_type: ComponentTypeNV) -> &mut Self {
        self.d_type = d_type;
        self
    }
    ///Sets the `scope` field.
    pub fn set_scope(&mut self, scope: ScopeNV) -> &mut Self {
        self.scope = scope;
        self
    }
    ///Sets the `m_size` field in a builder way.
    pub fn with_m_size(mut self, m_size: u32) -> Self {
        self.m_size = m_size;
        self
    }
    ///Sets the `n_size` field in a builder way.
    pub fn with_n_size(mut self, n_size: u32) -> Self {
        self.n_size = n_size;
        self
    }
    ///Sets the `k_size` field in a builder way.
    pub fn with_k_size(mut self, k_size: u32) -> Self {
        self.k_size = k_size;
        self
    }
    ///Sets the `a_type` field in a builder way.
    pub fn with_a_type(mut self, a_type: ComponentTypeNV) -> Self {
        self.a_type = a_type;
        self
    }
    ///Sets the `b_type` field in a builder way.
    pub fn with_b_type(mut self, b_type: ComponentTypeNV) -> Self {
        self.b_type = b_type;
        self
    }
    ///Sets the `c_type` field in a builder way.
    pub fn with_c_type(mut self, c_type: ComponentTypeNV) -> Self {
        self.c_type = c_type;
        self
    }
    ///Sets the `d_type` field in a builder way.
    pub fn with_d_type(mut self, d_type: ComponentTypeNV) -> Self {
        self.d_type = d_type;
        self
    }
    ///Sets the `scope` field in a builder way.
    pub fn with_scope(mut self, scope: ScopeNV) -> Self {
        self.scope = scope;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CooperativeMatrixPropertiesNV {
    type LowLevel = crate::native::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_cooperative_matrix::CooperativeMatrixPropertiesNV {
            s_type: StructureType::CooperativeMatrixPropertiesNv,
            p_next: std::ptr::null_mut(),
            m_size: self.m_size.into_low_level(context, bump),
            n_size: self.n_size.into_low_level(context, bump),
            k_size: self.k_size.into_low_level(context, bump),
            a_type: self.a_type.into_low_level(context, bump),
            b_type: self.b_type.into_low_level(context, bump),
            c_type: self.c_type.into_low_level(context, bump),
            d_type: self.d_type.into_low_level(context, bump),
            scope: self.scope.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CooperativeMatrixPropertiesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            m_size: crate::conv::FromLowLevel::from_low_level(context, value.m_size),
            n_size: crate::conv::FromLowLevel::from_low_level(context, value.n_size),
            k_size: crate::conv::FromLowLevel::from_low_level(context, value.k_size),
            a_type: crate::conv::FromLowLevel::from_low_level(context, value.a_type),
            b_type: crate::conv::FromLowLevel::from_low_level(context, value.b_type),
            c_type: crate::conv::FromLowLevel::from_low_level(context, value.c_type),
            d_type: crate::conv::FromLowLevel::from_low_level(context, value.d_type),
            scope: crate::conv::FromLowLevel::from_low_level(context, value.scope),
        }
    }
}
