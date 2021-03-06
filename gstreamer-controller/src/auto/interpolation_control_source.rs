// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gst;
use gst_controller_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use InterpolationMode;
use TimedValueControlSource;

glib_wrapper! {
    pub struct InterpolationControlSource(Object<gst_controller_sys::GstInterpolationControlSource, gst_controller_sys::GstInterpolationControlSourceClass, InterpolationControlSourceClass>) @extends TimedValueControlSource, gst::ControlSource, gst::Object;

    match fn {
        get_type => || gst_controller_sys::gst_interpolation_control_source_get_type(),
    }
}

impl InterpolationControlSource {
    pub fn new() -> InterpolationControlSource {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlSource::from_glib_full(
                gst_controller_sys::gst_interpolation_control_source_new(),
            )
            .unsafe_cast()
        }
    }
}

impl Default for InterpolationControlSource {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for InterpolationControlSource {}
unsafe impl Sync for InterpolationControlSource {}

pub const NONE_INTERPOLATION_CONTROL_SOURCE: Option<&InterpolationControlSource> = None;

pub trait InterpolationControlSourceExt: 'static {
    fn get_property_mode(&self) -> InterpolationMode;

    fn set_property_mode(&self, mode: InterpolationMode);

    fn connect_property_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<InterpolationControlSource>> InterpolationControlSourceExt for O {
    fn get_property_mode(&self) -> InterpolationMode {
        unsafe {
            let mut value = Value::from_type(<InterpolationMode as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mode\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `mode` getter")
                .unwrap()
        }
    }

    fn set_property_mode(&self, mode: InterpolationMode) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mode\0".as_ptr() as *const _,
                Value::from(&mode).to_glib_none().0,
            );
        }
    }

    fn connect_property_mode_notify<F: Fn(&Self) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<P, F: Fn(&P) + Send + Sync + 'static>(
            this: *mut gst_controller_sys::GstInterpolationControlSource,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<InterpolationControlSource>,
        {
            let f: &F = &*(f as *const F);
            f(&InterpolationControlSource::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
