use std::{
    error::Error,
    ops::Deref,
    path::{Path, PathBuf},
};

use log::{error, info};
use magritte::{
    size::Size,
    vulkan1_0::{PipelineCache as VkPipelineCache, PipelineCacheCreateInfo},
    AsRaw, Unique,
};

use crate::vulkan::Vulkan;

pub struct PipelineCache {
    path: PathBuf,
    cache: Unique<VkPipelineCache>,
}

impl Drop for PipelineCache {
    fn drop(&mut self) {
        let mut len = 0_usize;

        // first we get the data length
        if let Err(e) = unsafe {
            self.cache()
                .device()
                .get_pipeline_cache_data(self.cache().as_raw(), &mut len, None)
                .result()
        } {
            error!("Failed to get pipeline cache data length: {}", e);
            return;
        }

        // then we get the data
        let mut data = vec![0_u8; len];
        if let Err(e) = unsafe {
            self.cache()
                .device()
                .get_pipeline_cache_data(self.cache().as_raw(), &mut len, Some(data.as_mut_ptr().cast()))
                .result()
        } {
            error!("Failed to get pipeline cache data: {}", e);
            return;
        }

        if let Err(e) = std::fs::write(self.path(), &data[0..len]) {
            error!("Failed to write pipeline cache data: {}", e);
        } else {
            info!("Wrote pipeline cache data: {}", Size::from(len));
        }
    }
}

impl PipelineCache {
    pub fn new<P: AsRef<Path>>(vulkan: &Vulkan, path: P) -> Result<Self, Box<dyn Error>> {
        let path = path.as_ref();

        let mut info = PipelineCacheCreateInfo::default();
        if path.exists() {
            let data = std::fs::read(path)?;

            info = info.with_initial_data(unsafe { std::slice::from_raw_parts(data.as_ptr().cast(), data.len()) });
        }

        let (cache, _) = unsafe { vulkan.device().create_pipeline_cache(&info, None)? };

        Ok(Self {
            path: path.to_owned(),
            cache,
        })
    }

    /// Get a reference to the pipeline cache's path.
    #[must_use]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Set the pipeline cache's path.
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    /// Get a reference to the pipeline cache's cache.
    #[must_use]
    pub fn cache(&self) -> &Unique<VkPipelineCache> {
        &self.cache
    }
}

impl Deref for PipelineCache {
    type Target = Unique<VkPipelineCache>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}
