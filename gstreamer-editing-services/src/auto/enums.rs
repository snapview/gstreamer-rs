// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum Edge {
    EdgeStart,
    EdgeEnd,
    EdgeNone,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for Edge {
    type GlibType = ffi::GESEdge;

    fn to_glib(&self) -> ffi::GESEdge {
        match *self {
            Edge::EdgeStart => ffi::GES_EDGE_START,
            Edge::EdgeEnd => ffi::GES_EDGE_END,
            Edge::EdgeNone => ffi::GES_EDGE_NONE,
            Edge::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESEdge> for Edge {
    fn from_glib(value: ffi::GESEdge) -> Self {
        skip_assert_initialized!();
        match value {
            0 => Edge::EdgeStart,
            1 => Edge::EdgeEnd,
            2 => Edge::EdgeNone,
            value => Edge::__Unknown(value),
        }
    }
}

impl StaticType for Edge {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ges_edge_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for Edge {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for Edge {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for Edge {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
pub enum EditMode {
    EditNormal,
    EditRipple,
    EditRoll,
    EditTrim,
    EditSlide,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for EditMode {
    type GlibType = ffi::GESEditMode;

    fn to_glib(&self) -> ffi::GESEditMode {
        match *self {
            EditMode::EditNormal => ffi::GES_EDIT_MODE_NORMAL,
            EditMode::EditRipple => ffi::GES_EDIT_MODE_RIPPLE,
            EditMode::EditRoll => ffi::GES_EDIT_MODE_ROLL,
            EditMode::EditTrim => ffi::GES_EDIT_MODE_TRIM,
            EditMode::EditSlide => ffi::GES_EDIT_MODE_SLIDE,
            EditMode::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GESEditMode> for EditMode {
    fn from_glib(value: ffi::GESEditMode) -> Self {
        skip_assert_initialized!();
        match value {
            0 => EditMode::EditNormal,
            1 => EditMode::EditRipple,
            2 => EditMode::EditRoll,
            3 => EditMode::EditTrim,
            4 => EditMode::EditSlide,
            value => EditMode::__Unknown(value),
        }
    }
}

impl StaticType for EditMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ges_edit_mode_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for EditMode {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for EditMode {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for EditMode {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
