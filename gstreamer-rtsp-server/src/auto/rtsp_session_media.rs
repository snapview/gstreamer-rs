// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use gst;
use gst_rtsp_server_sys;
use std::mem;
use RTSPMedia;
use RTSPStreamTransport;

glib_wrapper! {
    pub struct RTSPSessionMedia(Object<gst_rtsp_server_sys::GstRTSPSessionMedia, gst_rtsp_server_sys::GstRTSPSessionMediaClass, RTSPSessionMediaClass>);

    match fn {
        get_type => || gst_rtsp_server_sys::gst_rtsp_session_media_get_type(),
    }
}

impl RTSPSessionMedia {
    pub fn new<P: IsA<RTSPMedia>>(path: &str, media: &P) -> RTSPSessionMedia {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_session_media_new(
                path.to_glib_none().0,
                media.as_ref().to_glib_full(),
            ))
        }
    }
}

unsafe impl Send for RTSPSessionMedia {}
unsafe impl Sync for RTSPSessionMedia {}

pub const NONE_RTSP_SESSION_MEDIA: Option<&RTSPSessionMedia> = None;

pub trait RTSPSessionMediaExt: 'static {
    //fn alloc_channels(&self, range: /*Ignored*/gst_rtsp::RTSPRange) -> bool;

    fn get_base_time(&self) -> gst::ClockTime;

    fn get_media(&self) -> Option<RTSPMedia>;

    fn get_rtpinfo(&self) -> Option<GString>;

    //fn get_rtsp_state(&self) -> /*Ignored*/gst_rtsp::RTSPState;

    fn get_transport(&self, idx: u32) -> Option<RTSPStreamTransport>;

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn get_transports(&self) -> Vec<RTSPStreamTransport>;

    fn matches(&self, path: &str) -> Option<i32>;

    //fn set_rtsp_state(&self, state: /*Ignored*/gst_rtsp::RTSPState);

    fn set_state(&self, state: gst::State) -> Result<(), glib::error::BoolError>;

    //fn set_transport<P: IsA<RTSPStream>>(&self, stream: &P, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> Option<RTSPStreamTransport>;
}

impl<O: IsA<RTSPSessionMedia>> RTSPSessionMediaExt for O {
    //fn alloc_channels(&self, range: /*Ignored*/gst_rtsp::RTSPRange) -> bool {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_session_media_alloc_channels() }
    //}

    fn get_base_time(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_rtsp_server_sys::gst_rtsp_session_media_get_base_time(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_media(&self) -> Option<RTSPMedia> {
        unsafe {
            from_glib_none(gst_rtsp_server_sys::gst_rtsp_session_media_get_media(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_rtpinfo(&self) -> Option<GString> {
        unsafe {
            from_glib_full(gst_rtsp_server_sys::gst_rtsp_session_media_get_rtpinfo(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_rtsp_state(&self) -> /*Ignored*/gst_rtsp::RTSPState {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_session_media_get_rtsp_state() }
    //}

    fn get_transport(&self, idx: u32) -> Option<RTSPStreamTransport> {
        unsafe {
            from_glib_none(gst_rtsp_server_sys::gst_rtsp_session_media_get_transport(
                self.as_ref().to_glib_none().0,
                idx,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    fn get_transports(&self) -> Vec<RTSPStreamTransport> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(
                gst_rtsp_server_sys::gst_rtsp_session_media_get_transports(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }

    fn matches(&self, path: &str) -> Option<i32> {
        unsafe {
            let mut matched = mem::MaybeUninit::uninit();
            let ret = from_glib(gst_rtsp_server_sys::gst_rtsp_session_media_matches(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
                matched.as_mut_ptr(),
            ));
            let matched = matched.assume_init();
            if ret {
                Some(matched)
            } else {
                None
            }
        }
    }

    //fn set_rtsp_state(&self, state: /*Ignored*/gst_rtsp::RTSPState) {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_session_media_set_rtsp_state() }
    //}

    fn set_state(&self, state: gst::State) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gst_rtsp_server_sys::gst_rtsp_session_media_set_state(
                    self.as_ref().to_glib_none().0,
                    state.to_glib()
                ),
                "Failed to set state of session media"
            )
        }
    }

    //fn set_transport<P: IsA<RTSPStream>>(&self, stream: &P, tr: /*Ignored*/&mut gst_rtsp::RTSPTransport) -> Option<RTSPStreamTransport> {
    //    unsafe { TODO: call gst_rtsp_server_sys:gst_rtsp_session_media_set_transport() }
    //}
}
