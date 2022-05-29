use std::error::Error;

use log::info;
use magritte::{
    cstr_ptr,
    spv::read_spv,
    vulkan1_0::{PipelineShaderStageCreateInfo, ShaderModule, ShaderModuleCreateInfo, ShaderStageFlagBits},
    AsRaw, Unique,
};

use crate::vulkan::Vulkan;

/// Simple wrapper around a shader module
pub struct Shader {
    /// The shader module
    pub module: Unique<ShaderModule>,

    /// The shader stage this shader is for
    pub stage: ShaderStageFlagBits,
}

impl Shader {
    pub fn new(vulkan: &Vulkan, code: &[u8], stage: ShaderStageFlagBits) -> Result<Self, Box<dyn Error>> {
        // First, we use bytemuck to cast the code into a slice of [`u32`].
        // We use a helper from Magritte for this
        let code = read_spv(code)?;

        // We add the code into a `ShaderModuleCreateInfo`.
        // This structure can be extended using the pointer chain.
        let module_create_info = ShaderModuleCreateInfo::default().with_code(&code);

        let (module, _) = unsafe { vulkan.device().create_shader_module(&module_create_info, None)? };

        info!("Created shader module: {:?}", module.as_raw());

        Ok(Self { module, stage })
    }

    /// Get a reference to the shader's module.
    pub fn module(&self) -> &Unique<ShaderModule> {
        &self.module
    }

    /// Get a reference to the shader's stage.
    pub fn stage(&self) -> ShaderStageFlagBits {
        self.stage
    }

    pub fn as_shader_stage_create_info(&self) -> PipelineShaderStageCreateInfo<'_> {
        PipelineShaderStageCreateInfo::default()
            .with_module(self.module().as_raw())
            .with_name(cstr_ptr!("main"))
            .with_stage(self.stage())
    }
}
