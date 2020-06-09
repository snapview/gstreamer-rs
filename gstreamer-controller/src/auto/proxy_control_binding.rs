// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::object::Cast;
#[cfg(any(feature = "v1_12", feature = "dox"))]
use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_controller_sys;

glib_wrapper! {
    pub struct ProxyControlBinding(Object<gst_controller_sys::GstProxyControlBinding, gst_controller_sys::GstProxyControlBindingClass, ProxyControlBindingClass>) @extends gst::ControlBinding, gst::Object;

    match fn {
        get_type => || gst_controller_sys::gst_proxy_control_binding_get_type(),
    }
}

impl ProxyControlBinding {
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    pub fn new<P: IsA<gst::Object>, Q: IsA<gst::Object>>(
        object: &P,
        property_name: &str,
        ref_object: &Q,
        ref_property_name: &str,
    ) -> ProxyControlBinding {
        assert_initialized_main_thread!();
        unsafe {
            gst::ControlBinding::from_glib_none(gst_controller_sys::gst_proxy_control_binding_new(
                object.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                ref_object.as_ref().to_glib_none().0,
                ref_property_name.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

unsafe impl Send for ProxyControlBinding {}
unsafe impl Sync for ProxyControlBinding {}

pub const NONE_PROXY_CONTROL_BINDING: Option<&ProxyControlBinding> = None;
