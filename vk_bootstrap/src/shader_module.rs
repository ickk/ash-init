use {
  crate::{DeviceContext, Result, VkContext},
  ::ash::vk,
};

pub struct ShaderModule {
  _spirv: Vec<u32>, // TODO can remove this
  vk_shader_module: vk::ShaderModule,
}

impl ShaderModule {
  pub fn module(&self) -> vk::ShaderModule {
    self.vk_shader_module
  }
}

impl<VkC> DeviceContext<VkC>
where
  VkC: AsRef<VkContext>,
{
  pub fn create_shader_module(&self, filename: &str) -> Result<ShaderModule> {
    let file = &mut std::fs::File::open(filename)?;
    let spirv = ash::util::read_spv(file)?;
    let create_info = vk::ShaderModuleCreateInfo::default().code(&spirv);

    let vk_shader_module =
      unsafe { self.device.create_shader_module(&create_info, None)? };

    Ok(ShaderModule {
      _spirv: spirv,
      vk_shader_module,
    })
  }

  #[allow(clippy::missing_safety_doc)]
  pub unsafe fn destroy_shader_module(&self, shader_module: ShaderModule) {
    unsafe {
      self
        .device
        .destroy_shader_module(shader_module.vk_shader_module, None);
    }
  }
}

// TODO implement drop on ShaderModule, store reference to device context
