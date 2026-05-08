// X11 screen capture
// Ported from boomer's screenshot.nim

use std::ffi::c_uint;
use x11::xlib::{
    Display, Window, XDestroyImage, XGetImage, XGetWindowAttributes, XImage,
    ZPixmap,
};

const ALL_PLANES: u64 = !0u64;

/// A captured screenshot backed by an XImage.
pub struct Screenshot {
    pub image: *mut XImage,
    pub width: u32,
    pub height: u32,
}
