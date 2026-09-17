macro_rules! delegate_builder_setters {
  // variant with 0 level of nesting
  (
    $(to self {
      $($vis:vis fn $name:ident($val_ty:tt$(.$method:ident())?);)*
    })*
  )
  => {
    $($(
      #[inline]
      $vis fn $name(mut self, value: $val_ty) -> Self {
        self.$name = value$(.$method())?;
        self
      }
    )*)*
  };
  // variant with 1 level of nesting
  (
    $(to self.$base:tt {
      $($vis:vis fn $name:ident($val_ty:tt$(.$method:ident())?);)*
    })*
  )
  => {
    $($(
      #[inline]
      $vis fn $name(mut self, value: $val_ty) -> Self {
        self.$base.$name = value$(.$method())?;
        self
      }
    )*)*
  };
  // variant with 2 level of nesting
  (
    $(to self.$base:tt.$base2:tt {
      $($vis:vis fn $name:ident($val_ty:tt$(.$method:ident())?);)*
    })*
  )
  => {
    $($(
      #[inline]
      $vis fn $name(mut self, value: $val_ty) -> Self {
        self.$base.$base2.$name = value$(.$method())?;
        self
      }
    )*)*
  };
  // variant with 3 levels of nesting
  (
    $(to self.$base:tt.$base2:tt.$base3:tt {
      $($vis:vis fn $name:ident($val_ty:tt$(.$method:ident())?);)*
    })*
  )
  => {
    $($(
      #[inline]
      $vis fn $name(mut self, value: $val_ty) -> Self {
        self.$base.$base2.$base3.$name = value$(.$method())?;
        self
      }
    )*)*
  };
}
pub(crate) use delegate_builder_setters;

// we implement a basic version of the delegate macro with declarative macros
// so that rust-analyzer plays nice.
//
// one limitation is that all methods must either be marked unsafe or none must
// be.
macro_rules! delegate {
  // safe version
  (
    $(
      to self.$field_name:ident {
        $(
          $vis:vis fn $method_name:ident(
            &self$(,)?
            $($param:ident: $param_ty:ty,)*
          ) $(-> $ret:ty)?;
        )*
      }
    )*
  )
  => {
    $(
      $(
        #[inline]
        $vis fn $method_name(
          &self,
          $($param: $param_ty,)*
        ) $(-> $ret)? {
          self.$field_name.$method_name($($param,)*)
        }
      )*
    )*
  };
  // unsafe version
  (
    $(
      to self.$field_name:ident {
        $(
          $vis:vis unsafe fn $method_name:ident(
            &self$(,)?
            $($param:ident: $param_ty:ty,)*
          ) $(-> $ret:ty)?;
        )*
      }
    )*
  )
  => {
    $(
      $(
        #[allow(clippy::missing_safety_doc)]
        #[inline]
        $vis unsafe fn $method_name(
          &self,
          $($param: $param_ty,)*
        ) $(-> $ret)? {
          unsafe { self.$field_name.$method_name($($param,)*) }
        }
      )*
    )*
  };
}
pub(crate) use delegate;

/// SAFETY: requires all the string literals to be null-terminated
macro_rules! make_extensions {
  (
    $vis:vis $struct_name:ident {
      $($extension_name:ident: $name_string:literal,)*
    }
  ) => {
    #[derive(Clone, Debug, PartialEq, Eq)]
    $vis struct $struct_name {
      $($vis $extension_name: bool,)*
    }

    impl Default for $struct_name {
      fn default() -> Self {
        $struct_name::NONE
      }
    }

    impl $struct_name {
      $vis const NONE: $struct_name = $struct_name {
        $($extension_name: false,)*
      };

      #[allow(unused)]
      pub(crate) fn to_vec_cstr(&self) -> Vec<&::core::ffi::CStr> {
        let mut list = Vec::new();
        $(
          if self.$extension_name {
            list.push(
              $name_string
            );
          }
        )*
        list
      }

      pub(crate) fn to_vec_cstr_ptr(&self) -> Vec<*const ::core::ffi::c_char> {
        let mut list = Vec::new();
        $(
          if self.$extension_name {
            list.push(
              $name_string
              .as_ptr()
            );
          }
        )*
        list
      }

      $(
        #[inline]
        $vis fn $extension_name(mut self, enable: bool) -> Self {
          self.$extension_name = enable;
          self
        }
      )*
    }
  };
}
pub(crate) use make_extensions;

macro_rules! make_device_features {
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
    }

    impl Default for Features {
      #[inline]
      fn default() -> Self {
        Features::NONE
      }
    }

    impl Features {
      pub(crate) fn features(&self) -> ::ash::vk::PhysicalDeviceFeatures {
        ::ash::vk::PhysicalDeviceFeatures {
          $($field_10: self.$field_10 as u32,)*
        }
      }

      pub(crate) fn features11(&self) -> ::ash::vk::PhysicalDeviceVulkan11Features<'_> {
        ::ash::vk::PhysicalDeviceVulkan11Features {
          $($field_11: self.$field_11 as u32,)*
          ..::ash::vk::PhysicalDeviceVulkan11Features::default()
        }
      }

      pub(crate) fn features12(&self) -> ::ash::vk::PhysicalDeviceVulkan12Features<'_> {
        ::ash::vk::PhysicalDeviceVulkan12Features {
          $($field_12: self.$field_12 as u32,)*
          ..::ash::vk::PhysicalDeviceVulkan12Features::default()
        }
      }

      pub(crate) fn features13(&self) -> ::ash::vk::PhysicalDeviceVulkan13Features<'_> {
        ::ash::vk::PhysicalDeviceVulkan13Features {
          $($field_13: self.$field_13 as u32,)*
          ..::ash::vk::PhysicalDeviceVulkan13Features::default()
        }
      }

      pub(crate) fn khr_fragment_shader_barycentric(&self) -> ::ash::vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_> {
        ::ash::vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
          $($field_khr_fragment_shader_barycentric:
            self.$field_khr_fragment_shader_barycentric as u32,)*
          ..::ash::vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR::default()
        }
      }

      pub(crate) fn from_vk_physical_device_features2(
        features: ::ash::vk::PhysicalDeviceFeatures,
        features11: ::ash::vk::PhysicalDeviceVulkan11Features,
        features12: ::ash::vk::PhysicalDeviceVulkan12Features,
        features13: ::ash::vk::PhysicalDeviceVulkan13Features,
        khr_fragment_shader_barycentric:
          ::ash::vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR,
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
        (self & rhs) == *self
      }
    }

    impl ::core::ops::BitAnd for &Features {
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

    impl ::core::ops::BitOr for &Features {
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

    impl ::core::ops::BitXor for &Features {
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
pub(crate) use make_device_features;
