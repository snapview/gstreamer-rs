// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_14", feature = "dox"))]
use glib::translate::*;
#[cfg(any(feature = "v1_14", feature = "dox"))]
use gst;
use gst_audio_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AudioStreamAlign(Boxed<gst_audio_sys::GstAudioStreamAlign>);

    match fn {
        copy => |ptr| gst_audio_sys::gst_audio_stream_align_copy(mut_override(ptr)),
        free => |ptr| gst_audio_sys::gst_audio_stream_align_free(ptr),
        get_type => || gst_audio_sys::gst_audio_stream_align_get_type(),
    }
}

impl AudioStreamAlign {
    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn new(
        rate: i32,
        alignment_threshold: gst::ClockTime,
        discont_wait: gst::ClockTime,
    ) -> AudioStreamAlign {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gst_audio_sys::gst_audio_stream_align_new(
                rate,
                alignment_threshold.to_glib(),
                discont_wait.to_glib(),
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_alignment_threshold(&self) -> gst::ClockTime {
        unsafe {
            from_glib(
                gst_audio_sys::gst_audio_stream_align_get_alignment_threshold(mut_override(
                    self.to_glib_none().0,
                )),
            )
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_discont_wait(&self) -> gst::ClockTime {
        unsafe {
            from_glib(gst_audio_sys::gst_audio_stream_align_get_discont_wait(
                mut_override(self.to_glib_none().0),
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_rate(&self) -> i32 {
        unsafe {
            gst_audio_sys::gst_audio_stream_align_get_rate(mut_override(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_samples_since_discont(&self) -> u64 {
        unsafe {
            gst_audio_sys::gst_audio_stream_align_get_samples_since_discont(mut_override(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn get_timestamp_at_discont(&self) -> gst::ClockTime {
        unsafe {
            from_glib(
                gst_audio_sys::gst_audio_stream_align_get_timestamp_at_discont(mut_override(
                    self.to_glib_none().0,
                )),
            )
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn mark_discont(&mut self) {
        unsafe {
            gst_audio_sys::gst_audio_stream_align_mark_discont(self.to_glib_none_mut().0);
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn set_alignment_threshold(&mut self, alignment_threshold: gst::ClockTime) {
        unsafe {
            gst_audio_sys::gst_audio_stream_align_set_alignment_threshold(
                self.to_glib_none_mut().0,
                alignment_threshold.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn set_discont_wait(&mut self, discont_wait: gst::ClockTime) {
        unsafe {
            gst_audio_sys::gst_audio_stream_align_set_discont_wait(
                self.to_glib_none_mut().0,
                discont_wait.to_glib(),
            );
        }
    }

    #[cfg(any(feature = "v1_14", feature = "dox"))]
    pub fn set_rate(&mut self, rate: i32) {
        unsafe {
            gst_audio_sys::gst_audio_stream_align_set_rate(self.to_glib_none_mut().0, rate);
        }
    }
}

unsafe impl Send for AudioStreamAlign {}
unsafe impl Sync for AudioStreamAlign {}
