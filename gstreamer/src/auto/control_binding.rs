// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use gst_sys;
use ClockTime;
use Object;

glib_wrapper! {
    pub struct ControlBinding(Object<gst_sys::GstControlBinding, gst_sys::GstControlBindingClass, ControlBindingClass>) @extends Object;

    match fn {
        get_type => || gst_sys::gst_control_binding_get_type(),
    }
}

unsafe impl Send for ControlBinding {}
unsafe impl Sync for ControlBinding {}

pub const NONE_CONTROL_BINDING: Option<&ControlBinding> = None;

pub trait ControlBindingExt: 'static {
    //fn get_g_value_array(&self, timestamp: ClockTime, interval: ClockTime, values: /*Ignored*/&[&glib::Value]) -> bool;

    //fn get_value(&self, timestamp: ClockTime) -> /*Ignored*/Option<glib::Value>;

    //fn get_value_array(&self, timestamp: ClockTime, interval: ClockTime, values: /*Unimplemented*/&[&Fundamental: Pointer]) -> bool;

    fn is_disabled(&self) -> bool;

    fn set_disabled(&self, disabled: bool);

    fn sync_values<P: IsA<Object>>(
        &self,
        object: &P,
        timestamp: ClockTime,
        last_sync: ClockTime,
    ) -> bool;

    fn get_property_object(&self) -> Option<Object>;
}

impl<O: IsA<ControlBinding>> ControlBindingExt for O {
    //fn get_g_value_array(&self, timestamp: ClockTime, interval: ClockTime, values: /*Ignored*/&[&glib::Value]) -> bool {
    //    unsafe { TODO: call gst_sys:gst_control_binding_get_g_value_array() }
    //}

    //fn get_value(&self, timestamp: ClockTime) -> /*Ignored*/Option<glib::Value> {
    //    unsafe { TODO: call gst_sys:gst_control_binding_get_value() }
    //}

    //fn get_value_array(&self, timestamp: ClockTime, interval: ClockTime, values: /*Unimplemented*/&[&Fundamental: Pointer]) -> bool {
    //    unsafe { TODO: call gst_sys:gst_control_binding_get_value_array() }
    //}

    fn is_disabled(&self) -> bool {
        unsafe {
            from_glib(gst_sys::gst_control_binding_is_disabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_disabled(&self, disabled: bool) {
        unsafe {
            gst_sys::gst_control_binding_set_disabled(
                self.as_ref().to_glib_none().0,
                disabled.to_glib(),
            );
        }
    }

    fn sync_values<P: IsA<Object>>(
        &self,
        object: &P,
        timestamp: ClockTime,
        last_sync: ClockTime,
    ) -> bool {
        unsafe {
            from_glib(gst_sys::gst_control_binding_sync_values(
                self.as_ref().to_glib_none().0,
                object.as_ref().to_glib_none().0,
                timestamp.to_glib(),
                last_sync.to_glib(),
            ))
        }
    }

    fn get_property_object(&self) -> Option<Object> {
        unsafe {
            let mut value = Value::from_type(<Object as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"object\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `object` getter")
        }
    }
}