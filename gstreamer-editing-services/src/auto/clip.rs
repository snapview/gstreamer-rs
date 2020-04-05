// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ges_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::mem::transmute;
use Asset;
use BaseEffect;
use Container;
use Extractable;
use Layer;
use TimelineElement;
use Track;
use TrackElement;
use TrackType;

glib_wrapper! {
    pub struct Clip(Object<ges_sys::GESClip, ges_sys::GESClipClass, ClipClass>) @extends Container, TimelineElement, @implements Extractable;

    match fn {
        get_type => || ges_sys::ges_clip_get_type(),
    }
}

pub const NONE_CLIP: Option<&Clip> = None;

pub trait ClipExt: 'static {
    fn add_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<TrackElement, glib::BoolError>;

    fn find_track_element<P: IsA<Track>>(
        &self,
        track: Option<&P>,
        type_: glib::types::Type,
    ) -> Option<TrackElement>;

    fn find_track_elements<P: IsA<Track>>(
        &self,
        track: Option<&P>,
        track_type: TrackType,
        type_: glib::types::Type,
    ) -> Vec<TrackElement>;

    fn get_layer(&self) -> Option<Layer>;

    fn get_supported_formats(&self) -> TrackType;

    fn get_top_effect_index<P: IsA<BaseEffect>>(&self, effect: &P) -> i32;

    fn get_top_effect_position<P: IsA<BaseEffect>>(&self, effect: &P) -> i32;

    fn get_top_effects(&self) -> Vec<TrackElement>;

    fn move_to_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError>;

    fn set_supported_formats(&self, supportedformats: TrackType);

    fn set_top_effect_index<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newindex: u32,
    ) -> Result<(), glib::error::BoolError>;

    fn set_top_effect_priority<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newpriority: u32,
    ) -> Result<(), glib::error::BoolError>;

    fn split(&self, position: u64) -> Result<Clip, glib::BoolError>;

    fn connect_property_layer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_supported_formats_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Clip>> ClipExt for O {
    fn add_asset<P: IsA<Asset>>(&self, asset: &P) -> Result<TrackElement, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ges_sys::ges_clip_add_asset(
                self.as_ref().to_glib_none().0,
                asset.as_ref().to_glib_none().0,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to add asset"))
        }
    }

    fn find_track_element<P: IsA<Track>>(
        &self,
        track: Option<&P>,
        type_: glib::types::Type,
    ) -> Option<TrackElement> {
        unsafe {
            from_glib_full(ges_sys::ges_clip_find_track_element(
                self.as_ref().to_glib_none().0,
                track.map(|p| p.as_ref()).to_glib_none().0,
                type_.to_glib(),
            ))
        }
    }

    fn find_track_elements<P: IsA<Track>>(
        &self,
        track: Option<&P>,
        track_type: TrackType,
        type_: glib::types::Type,
    ) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ges_sys::ges_clip_find_track_elements(
                self.as_ref().to_glib_none().0,
                track.map(|p| p.as_ref()).to_glib_none().0,
                track_type.to_glib(),
                type_.to_glib(),
            ))
        }
    }

    fn get_layer(&self) -> Option<Layer> {
        unsafe { from_glib_full(ges_sys::ges_clip_get_layer(self.as_ref().to_glib_none().0)) }
    }

    fn get_supported_formats(&self) -> TrackType {
        unsafe {
            from_glib(ges_sys::ges_clip_get_supported_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_top_effect_index<P: IsA<BaseEffect>>(&self, effect: &P) -> i32 {
        unsafe {
            ges_sys::ges_clip_get_top_effect_index(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
            )
        }
    }

    fn get_top_effect_position<P: IsA<BaseEffect>>(&self, effect: &P) -> i32 {
        unsafe {
            ges_sys::ges_clip_get_top_effect_position(
                self.as_ref().to_glib_none().0,
                effect.as_ref().to_glib_none().0,
            )
        }
    }

    fn get_top_effects(&self) -> Vec<TrackElement> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ges_sys::ges_clip_get_top_effects(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn move_to_layer<P: IsA<Layer>>(&self, layer: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_clip_move_to_layer(
                    self.as_ref().to_glib_none().0,
                    layer.as_ref().to_glib_none().0
                ),
                "Failed to move clip to specified layer"
            )
        }
    }

    fn set_supported_formats(&self, supportedformats: TrackType) {
        unsafe {
            ges_sys::ges_clip_set_supported_formats(
                self.as_ref().to_glib_none().0,
                supportedformats.to_glib(),
            );
        }
    }

    fn set_top_effect_index<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newindex: u32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_clip_set_top_effect_index(
                    self.as_ref().to_glib_none().0,
                    effect.as_ref().to_glib_none().0,
                    newindex
                ),
                "Failed to move effect"
            )
        }
    }

    fn set_top_effect_priority<P: IsA<BaseEffect>>(
        &self,
        effect: &P,
        newpriority: u32,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                ges_sys::ges_clip_set_top_effect_priority(
                    self.as_ref().to_glib_none().0,
                    effect.as_ref().to_glib_none().0,
                    newpriority
                ),
                "Failed to the set top effect priority"
            )
        }
    }

    fn split(&self, position: u64) -> Result<Clip, glib::BoolError> {
        unsafe {
            Option::<_>::from_glib_none(ges_sys::ges_clip_split(
                self.as_ref().to_glib_none().0,
                position,
            ))
            .ok_or_else(|| glib_bool_error!("Failed to split clip"))
        }
    }

    fn connect_property_layer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_layer_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESClip,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Clip>,
        {
            let f: &F = &*(f as *const F);
            f(&Clip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::layer\0".as_ptr() as *const _,
                Some(transmute(notify_layer_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_supported_formats_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_supported_formats_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ges_sys::GESClip,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Clip>,
        {
            let f: &F = &*(f as *const F);
            f(&Clip::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::supported-formats\0".as_ptr() as *const _,
                Some(transmute(
                    notify_supported_formats_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }
}
