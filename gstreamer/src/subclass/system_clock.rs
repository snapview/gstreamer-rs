// Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_sys;

use super::prelude::*;
use glib::subclass::prelude::*;

use SystemClockClass;

pub trait SystemClockImpl: ClockImpl {}

unsafe impl<T: SystemClockImpl> IsSubclassable<T> for SystemClockClass {
    fn override_vfuncs(&mut self) {
        <::ClockClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let _klass = &mut *(self as *mut Self as *mut gst_sys::GstSystemClockClass);
            // Nothing to do here
        }
    }
}
