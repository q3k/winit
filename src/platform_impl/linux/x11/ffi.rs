use crate::platform_impl::platform::x11::x11::xmd::CARD32;
pub use crate::platform_impl::platform::x11::x11::{error::OpenError, xcursor::*, xinput2::*, xlib::*, xlib_xcb::*};

// Isn't defined by x11/x11_dl
#[allow(non_upper_case_globals)]
pub const IconicState: CARD32 = 3;
