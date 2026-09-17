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
        $struct_name::DEFAULT
      }
    }

    impl $struct_name {
      $vis const DEFAULT: $struct_name = $struct_name {
        $($extension_name: false,)*
      };

      #[allow(unused)]
      pub(crate) fn to_vec_cstr(&self) -> Vec<&::core::ffi::CStr> {
        let mut list = Vec::new();
        $(
          if self.$extension_name {
            list.push(
              ::core::ffi::CStr::from_bytes_with_nul($name_string).unwrap()
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
              ::core::ffi::CStr::from_bytes_with_nul($name_string).unwrap()
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
