use {
  ::ash::vk,
  ::core::ops::{BitAnd, BitOr, BitXor},
};

macro_rules! make_features {
  (
    features10: {
      $($field_10:ident,)*
    }
    features11: {
      $($field_11:ident,)*
    }
    features12: {
      $($field_12:ident,)*
    }
    features13: {
      $($field_13:ident,)*
    }
    khr_fragment_shader_barycentric: {
      $($field_khr_fragment_shader_barycentric:ident,)*
    }
  ) => {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Features {
      $(pub $field_10: bool,)*
      $(pub $field_11: bool,)*
      $(pub $field_12: bool,)*
      $(pub $field_13: bool,)*
      $(pub $field_khr_fragment_shader_barycentric: bool,)*
    }

    impl Features {
      pub const NONE: Self = Features {
        $($field_10: false,)*
        $($field_11: false,)*
        $($field_12: false,)*
        $($field_13: false,)*
        $($field_khr_fragment_shader_barycentric: false,)*
      };
      pub const DEFAULT: Self = Features::NONE;
    }

    impl Default for Features {
      #[inline]
      fn default() -> Self {
        Features::DEFAULT
      }
    }

    impl Features {
      pub(crate) fn features(&self) -> vk::PhysicalDeviceFeatures {
        vk::PhysicalDeviceFeatures {
          $($field_10: self.$field_10 as u32,)*
        }
      }

      pub(crate) fn features11(&self) -> vk::PhysicalDeviceVulkan11Features<'_> {
        vk::PhysicalDeviceVulkan11Features {
          $($field_11: self.$field_11 as u32,)*
          ..vk::PhysicalDeviceVulkan11Features::default()
        }
      }

      pub(crate) fn features12(&self) -> vk::PhysicalDeviceVulkan12Features<'_> {
        vk::PhysicalDeviceVulkan12Features {
          $($field_12: self.$field_12 as u32,)*
          ..vk::PhysicalDeviceVulkan12Features::default()
        }
      }

      pub(crate) fn features13(&self) -> vk::PhysicalDeviceVulkan13Features<'_> {
        vk::PhysicalDeviceVulkan13Features {
          $($field_13: self.$field_13 as u32,)*
          ..vk::PhysicalDeviceVulkan13Features::default()
        }
      }

      pub(crate) fn khr_fragment_shader_barycentric(&self) -> vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_> {
        vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
          $($field_khr_fragment_shader_barycentric:
            self.$field_khr_fragment_shader_barycentric as u32,)*
          ..vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR::default()
        }
      }

      pub(crate) fn from_vk_physical_device_features2(
        features: vk::PhysicalDeviceFeatures,
        features11: vk::PhysicalDeviceVulkan11Features,
        features12: vk::PhysicalDeviceVulkan12Features,
        features13: vk::PhysicalDeviceVulkan13Features,
        khr_fragment_shader_barycentric:
          vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR,
      ) -> Self {
        Features {
          $($field_10: features.$field_10 == 1,)*
          $($field_11: features11.$field_11 == 1,)*
          $($field_12: features12.$field_12 == 1,)*
          $($field_13: features13.$field_13 == 1,)*
          $($field_khr_fragment_shader_barycentric:
            khr_fragment_shader_barycentric.$field_khr_fragment_shader_barycentric
            == 1,)*
        }
      }

      /// returns whether the features are a subset of the rhs features
      #[inline]
      pub(crate) fn is_subset(&self, rhs: &Self) -> bool {
        &self.bitand(rhs) == self
      }
    }

    impl BitAnd for &Features {
      type Output = Features;

      fn bitand(self, rhs: Self) -> Self::Output {
        Features {
          $($field_10: self.$field_10 & rhs.$field_10,)*
          $($field_11: self.$field_11 & rhs.$field_11,)*
          $($field_12: self.$field_12 & rhs.$field_12,)*
          $($field_13: self.$field_13 & rhs.$field_13,)*
          $($field_khr_fragment_shader_barycentric:
            self.$field_khr_fragment_shader_barycentric
            & rhs.$field_khr_fragment_shader_barycentric,)*
        }
      }
    }

    impl BitOr for &Features {
      type Output = Features;

      fn bitor(self, rhs: Self) -> Self::Output {
        Features {
          $($field_10: self.$field_10 | rhs.$field_10,)*
          $($field_11: self.$field_11 | rhs.$field_11,)*
          $($field_12: self.$field_12 | rhs.$field_12,)*
          $($field_13: self.$field_13 | rhs.$field_13,)*
          $($field_khr_fragment_shader_barycentric:
            self.$field_khr_fragment_shader_barycentric
            | rhs.$field_khr_fragment_shader_barycentric,)*
        }
      }
    }

    impl BitXor for &Features {
      type Output = Features;

      fn bitxor(self, rhs: Self) -> Self::Output {
        Features {
          $($field_10: self.$field_10 ^ rhs.$field_10,)*
          $($field_11: self.$field_11 ^ rhs.$field_11,)*
          $($field_12: self.$field_12 ^ rhs.$field_12,)*
          $($field_13: self.$field_13 ^ rhs.$field_13,)*
          $($field_khr_fragment_shader_barycentric:
            self.$field_khr_fragment_shader_barycentric
            ^ rhs.$field_khr_fragment_shader_barycentric,)*
        }
      }
    }
  }
}

make_features! {
  features10: {
    robust_buffer_access,
    full_draw_index_uint32,
    image_cube_array,
    independent_blend,
    geometry_shader,
    tessellation_shader,
    sample_rate_shading,
    dual_src_blend,
    logic_op,
    multi_draw_indirect,
    draw_indirect_first_instance,
    depth_clamp,
    depth_bias_clamp,
    fill_mode_non_solid,
    depth_bounds,
    wide_lines,
    large_points,
    alpha_to_one,
    multi_viewport,
    sampler_anisotropy,
    texture_compression_etc2,
    texture_compression_astc_ldr,
    texture_compression_bc,
    occlusion_query_precise,
    pipeline_statistics_query,
    vertex_pipeline_stores_and_atomics,
    fragment_stores_and_atomics,
    shader_tessellation_and_geometry_point_size,
    shader_image_gather_extended,
    shader_storage_image_extended_formats,
    shader_storage_image_multisample,
    shader_storage_image_read_without_format,
    shader_storage_image_write_without_format,
    shader_uniform_buffer_array_dynamic_indexing,
    shader_sampled_image_array_dynamic_indexing,
    shader_storage_buffer_array_dynamic_indexing,
    shader_storage_image_array_dynamic_indexing,
    shader_clip_distance,
    shader_cull_distance,
    shader_float64,
    shader_int64,
    shader_int16,
    shader_resource_residency,
    shader_resource_min_lod,
    sparse_binding,
    sparse_residency_buffer,
    sparse_residency_image2_d,
    sparse_residency_image3_d,
    sparse_residency2_samples,
    sparse_residency4_samples,
    sparse_residency8_samples,
    sparse_residency16_samples,
    sparse_residency_aliased,
    variable_multisample_rate,
    inherited_queries,
  }
  features11: {
    storage_buffer16_bit_access,
    uniform_and_storage_buffer16_bit_access,
    storage_push_constant16,
    storage_input_output16,
    multiview,
    multiview_geometry_shader,
    multiview_tessellation_shader,
    variable_pointers_storage_buffer,
    variable_pointers,
    protected_memory,
    sampler_ycbcr_conversion,
    shader_draw_parameters,
  }
  features12: {
    sampler_mirror_clamp_to_edge,
    draw_indirect_count,
    storage_buffer8_bit_access,
    uniform_and_storage_buffer8_bit_access,
    storage_push_constant8,
    shader_buffer_int64_atomics,
    shader_shared_int64_atomics,
    shader_float16,
    shader_int8,
    descriptor_indexing,
    shader_input_attachment_array_dynamic_indexing,
    shader_uniform_texel_buffer_array_dynamic_indexing,
    shader_storage_texel_buffer_array_dynamic_indexing,
    shader_uniform_buffer_array_non_uniform_indexing,
    shader_sampled_image_array_non_uniform_indexing,
    shader_storage_buffer_array_non_uniform_indexing,
    shader_storage_image_array_non_uniform_indexing,
    shader_input_attachment_array_non_uniform_indexing,
    shader_uniform_texel_buffer_array_non_uniform_indexing,
    shader_storage_texel_buffer_array_non_uniform_indexing,
    descriptor_binding_uniform_buffer_update_after_bind,
    descriptor_binding_sampled_image_update_after_bind,
    descriptor_binding_storage_image_update_after_bind,
    descriptor_binding_storage_buffer_update_after_bind,
    descriptor_binding_uniform_texel_buffer_update_after_bind,
    descriptor_binding_storage_texel_buffer_update_after_bind,
    descriptor_binding_update_unused_while_pending,
    descriptor_binding_partially_bound,
    descriptor_binding_variable_descriptor_count,
    runtime_descriptor_array,
    sampler_filter_minmax,
    scalar_block_layout,
    imageless_framebuffer,
    uniform_buffer_standard_layout,
    shader_subgroup_extended_types,
    separate_depth_stencil_layouts,
    host_query_reset,
    timeline_semaphore,
    buffer_device_address,
    buffer_device_address_capture_replay,
    buffer_device_address_multi_device,
    vulkan_memory_model,
    vulkan_memory_model_device_scope,
    vulkan_memory_model_availability_visibility_chains,
    shader_output_viewport_index,
    shader_output_layer,
    subgroup_broadcast_dynamic_id,
  }
  features13: {
    robust_image_access,
    inline_uniform_block,
    descriptor_binding_inline_uniform_block_update_after_bind,
    pipeline_creation_cache_control,
    private_data,
    shader_demote_to_helper_invocation,
    shader_terminate_invocation,
    subgroup_size_control,
    compute_full_subgroups,
    synchronization2,
    texture_compression_astc_hdr,
    shader_zero_initialize_workgroup_memory,
    dynamic_rendering,
    shader_integer_dot_product,
    maintenance4,
  }
  khr_fragment_shader_barycentric: {
    fragment_shader_barycentric,
  }
}
